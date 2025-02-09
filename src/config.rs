use alloy::primitives::B256;
use clap::Parser;
use crate::utils::SearchType;

#[derive(Parser)]
pub struct Config {
    #[arg(long, env = "RPC_URL")]
    pub rpc_url: String,
    
    #[arg(long)]
    pub hash: B256,

    #[arg(long)]
    pub search: Option<SearchType>,
} 