use alloy::rpc::types::trace::geth::CallFrame;
use alloy::primitives::U256;

#[derive(Debug, Clone, Copy)]
pub enum SearchType {
    Create2,
    Create3,
}

impl std::str::FromStr for SearchType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "create2" => Ok(SearchType::Create2),
            "create3" => Ok(SearchType::Create3),
            _ => Err("Invalid search type. Must be 'create2' or 'create3'".to_string()),
        }
    }
}

pub fn search_deployment_type(frame: &CallFrame, search_type: SearchType) -> bool {
    match search_type {
        SearchType::Create2 => frame.typ == "CREATE2",
        SearchType::Create3 => frame.typ == "CREATE3",
    }
}

pub fn analyze_call_frame(frame: &CallFrame, search_type: Option<SearchType>) -> Vec<String> {
    let mut findings = Vec::new();
    
    if let Some(search_type) = search_type {
        if search_deployment_type(frame, search_type) {
            findings.push(format!(
                "Found {} deployment:\nFrom: {:#x}\nTo: {:#x?}\nValue: {:#x?}",
                format!("{:?}", search_type),
                frame.from,
                frame.to,
                frame.value.unwrap_or(U256::ZERO)
            ));
        }
    }

    for call in &frame.calls {
        findings.extend(analyze_call_frame(call, search_type));
    }

    findings
} 