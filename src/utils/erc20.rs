use alloy::{
    primitives::{Address, Bytes, keccak256, TxKind, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::eth::transaction::{TransactionInput, TransactionRequest},
};
use eyre::Result;

#[derive(Debug)]
pub struct Erc20Details {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: Option<u8>,
    pub total_supply: Option<U256>,
}

type DecoderFn = Box<dyn Fn(Bytes) -> Option<Box<dyn std::any::Any + Send>>>;

pub async fn get_erc20_details(rpc_url: &str, address: Address) -> Result<Option<Erc20Details>> {
    let provider = ProviderBuilder::new().on_http(rpc_url.parse()?);
    let mut details = Erc20Details {
        name: None,
        symbol: None,
        decimals: None,
        total_supply: None,
    };
    let mut valid_calls = 0;

    let functions: [(_, DecoderFn); 4] = [
        ("totalSupply()", Box::new(|bytes: Bytes| {
            if bytes.len() >= 32 {
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&bytes[..32]);
                Some(Box::new(U256::from_be_bytes(arr)))
            } else {
                None
            }
        })),
        ("name()", Box::new(|bytes: Bytes| {
            String::from_utf8(bytes.to_vec()).ok().map(|s| Box::new(s) as Box<dyn std::any::Any + Send>)
        })),
        ("symbol()", Box::new(|bytes: Bytes| {
            String::from_utf8(bytes.to_vec()).ok().map(|s| Box::new(s) as Box<dyn std::any::Any + Send>)
        })),
        ("decimals()", Box::new(|bytes: Bytes| {
            if bytes.len() >= 32 {
                Some(Box::new(bytes[31] as u8))
            } else {
                None
            }
        })),
    ];

    for (sig, decoder) in functions {
        let data = Bytes::copy_from_slice(&keccak256(sig.as_bytes())[..4]);
        let mut tx = TransactionRequest::default();
        tx.to = Some(TxKind::Call(address));
        tx.input = TransactionInput::new(data);

        if let Ok(result) = provider.call(&tx).await {
            valid_calls += 1;
            let decoded = decoder(result);
            match sig {
                "totalSupply()" => if let Some(val) = decoded {
                    details.total_supply = val.downcast_ref::<U256>().copied()
                },
                "name()" => if let Some(val) = decoded {
                    details.name = val.downcast_ref::<String>().cloned()
                },
                "symbol()" => if let Some(val) = decoded {
                    details.symbol = val.downcast_ref::<String>().cloned()
                },
                "decimals()" => if let Some(val) = decoded {
                    details.decimals = val.downcast_ref::<u8>().copied()
                },
                _ => {}
            }
        }
    }

    if valid_calls >= 3 {
        Ok(Some(details))
    } else {
        Ok(None)
    }
} 