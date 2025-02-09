use alloy::{
    primitives::B256,
    providers::{ext::DebugApi, ProviderBuilder},
    rpc::types::trace::geth::{
        GethDebugBuiltInTracerType, GethDebugTracerType, GethDebugTracingOptions,
        GethDefaultTracingOptions,
    },
};
use eyre::Result;
use std::fs::File;
use std::io::Write;
use crate::utils::{SearchType, analyze_call_frame, get_erc20_details};

pub async fn trace_transaction(rpc_url: &str, hash: B256, search: Option<SearchType>) -> Result<()> {
    let provider = ProviderBuilder::new().on_http(rpc_url.parse()?);

    let call_options = GethDebugTracingOptions {
        config: GethDefaultTracingOptions {
            disable_storage: Some(true),
            enable_memory: Some(false),
            ..Default::default()
        },
        tracer: Some(GethDebugTracerType::BuiltInTracer(
            GethDebugBuiltInTracerType::CallTracer,
        )),
        ..Default::default()
    };
    
    let result = provider.debug_trace_transaction(hash, call_options).await?;
    
    std::fs::create_dir_all("out")?;
    let mut call_file = File::create("out/call_trace.txt")?;
    write!(call_file, "{:#?}", result)?;
    println!("Call trace saved to out/call_trace.txt");

    if let Some(search_type) = search {
        if let alloy::rpc::types::trace::geth::GethTrace::CallTracer(result_frame) = result {
            let findings = analyze_call_frame(&result_frame, Some(search_type));
            if findings.is_empty() {
                println!("No {:?} deployments found in this transaction", search_type);
            } else {
                println!("\nFound {:?} deployments:", search_type);
                for finding in findings {
                    println!("{}", finding);
                    
                    if let Some(addr_str) = finding.lines().find(|line| line.starts_with("To: ")) {
                        if let Ok(addr) = addr_str[4..].trim().parse() {
                            match get_erc20_details(rpc_url, addr).await {
                                Ok(Some(details)) => {
                                    println!("Contract is an ERC20 token");
                                    if let Some(name) = details.name {
                                        println!("Name: {}", name);
                                    }
                                    if let Some(symbol) = details.symbol {
                                        println!("Symbol: {}", symbol);
                                    }
                                    if let Some(decimals) = details.decimals {
                                        println!("Decimals: {}", decimals);
                                    }
                                    if let Some(total_supply) = details.total_supply {
                                        println!("Total Supply: {}", total_supply);
                                    }
                                },
                                Ok(None) => println!("Contract is not an ERC20 token"),
                                Err(_) => println!("Could not determine if contract is an ERC20 token"),
                            }
                        }
                    }
                    println!();
                }
            }
        }
    }
    
    Ok(())
}