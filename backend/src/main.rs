#[macro_use]
extern crate lazy_static;
mod api;
mod checkpoint;
mod config;
use crate::checkpoint::{get_checkpoint, write_checkpoint};
use crate::config::CONFIG;
use api::{create_contact, get_default_sender, send_letter, Contact, Letter};
use ecies::decrypt;
use ethers::contract::Contract;
use ethers::prelude::LogMeta;
use ethers::providers::{Middleware, Ws};
use ethers::utils::hex::decode;
use ethers::{prelude::abigen, providers::Provider, types::Address};
use std::str::FromStr;
use std::sync::Arc;

abigen!(
    MailContract,
    "../contracts/out/EthereumPostalService.sol/EthereumPostalService.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // INSTANCE.set(config).expect("couldn't globalize config");
    for rpc in &CONFIG.rpcs {
        let provider = Provider::<Ws>::connect(rpc).await.unwrap();
        let provider = Arc::new(provider);
        let chain_id = provider.get_chainid().await?;
        let contract_address = Address::from_str(&CONFIG.contract)?;
        let last_block = get_checkpoint(chain_id).unwrap_or(0);

        let event = Contract::event_of_type::<MailReceivedFilter>(&provider)
            .from_block(last_block)
            .address(contract_address.into());

        let events = event.query_with_meta().await?;
        for (log, meta) in events {
            let res = handle_log(log, &meta).await;
            match res {
                Ok(_) => {
                    write_checkpoint(chain_id, meta.block_number.as_u64() + 1)?;
                    println!("Log handle success.");
                }
                Err(e) => {
                    println!("Failed to handle log: {:?}", e);
                }
            }
        }
    }
    println!("Completed Scan");
    Ok(())
}

async fn handle_log(event: MailReceivedFilter, meta: &LogMeta) -> anyhow::Result<()> {
    // this may be encrypted
    let secret_key = &CONFIG.decryption_key;
    let target_contact = if !event.address_encrypted {
        Contact::new(
            event.postal_address.address_line_1,
            event.postal_address.address_line_2,
            event.postal_address.city,
            event.postal_address.country_code,
            event.postal_address.postal_or_zip,
            event.postal_address.name,
        )
    } else {
        Contact::new(
            decrypt_string(secret_key, event.postal_address.address_line_1)?,
            decrypt_string(secret_key, event.postal_address.address_line_2)?,
            decrypt_string(secret_key, event.postal_address.city)?,
            decrypt_string(secret_key, event.postal_address.country_code)?,
            decrypt_string(secret_key, event.postal_address.postal_or_zip)?,
            decrypt_string(secret_key, event.postal_address.name)?,
        )
    };
    let target = create_contact(target_contact).await?;
    let from = get_default_sender().await?;
    let msg = if !event.msg_encrypted {
        event.msg_html
    } else {
        decrypt_string(secret_key, event.msg_html)?
    };
    let idem_key: String = format!("{:?}", meta.transaction_hash);
    let letter = Letter::new(
        from,
        target,
        msg,
        idem_key,
        meta.transaction_index.as_u64(),
    );
    let letter_id = send_letter(letter).await?;
    println!("Letter sent! {:?}", letter_id);
    Ok(())
}

fn decrypt_string(secret_key: &str, message: String) -> anyhow::Result<String> {
    let sk = &decode(secret_key)?;
    let msg = &decode(message)?;
    let decrypted = decrypt(sk, msg)?;
    Ok(String::from_utf8(decrypted)?)
}

#[tokio::test]
async fn test_decrypt() {
    use ecies::encrypt;
    use ecies::utils::generate_keypair;

    let (sk, pk) = generate_keypair();
    let (sk, pk) = (&sk.serialize(), &pk.serialize());
    let msg_o: String = "test message".to_string();
    let msg = msg_o.as_bytes();
    let msg_e = encrypt(pk, &msg).unwrap();
    let msg_e = ethers::utils::hex::encode(msg_e);
    let sk = ethers::utils::hex::encode(sk);
    let message = decrypt_string(&sk, msg_e).unwrap();
    println!("{:?}", message);
    assert_eq!(msg_o, message);
}
