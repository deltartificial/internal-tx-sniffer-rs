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
use crate::utils::{SearchType, analyze_call_frame};

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
    
    let mut call_file = File::create("call_trace.txt")?;
    write!(call_file, "{:#?}", result)?;
    println!("Call trace saved to call_trace.txt");

    if let Some(search_type) = search {
        if let alloy::rpc::types::trace::geth::GethTrace::CallTracer(result_frame) = result {
            let findings = analyze_call_frame(&result_frame, Some(search_type));
            if findings.is_empty() {
                println!("No {:?} deployments found in this transaction", search_type);
            } else {
                println!("\nFound {:?} deployments:", search_type);
                for finding in findings {
                    println!("{}\n", finding);
                }
            }
        }
    }
    
    Ok(())
}