#[macro_use]
extern crate lazy_static;
mod api;
mod checkpoint;
mod config;
mod listener;
use crate::checkpoint::{get_checkpoint, write_checkpoint};
use crate::config::CONFIG;
use api::{create_contact, get_default_sender, send_letter, Contact, Letter};
use ecies::decrypt;
use ethers::contract::Contract;
use ethers::prelude::LogMeta;
use ethers::providers::{Middleware, StreamExt, Ws};
use ethers::utils::hex::decode;
use ethers::{prelude::abigen, providers::Provider, types::Address};
use std::str::FromStr;
use std::sync::Arc;

abigen!(
    MailContract,
    "../contracts/out/EthMail.sol/EthMail.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // INSTANCE.set(config).expect("couldn't globalize config");
    let provider = Provider::<Ws>::connect(&CONFIG.rpc).await.unwrap();
    let provider = Arc::new(provider);
    let chain_id = provider.get_chainid().await?;
    let contract_address = Address::from_str(&CONFIG.contract)?;
    let last_block = get_checkpoint(chain_id).unwrap_or(0);

    let event = Contract::event_of_type::<MailReceivedFilter>(&provider)
        .from_block(last_block)
        .address(contract_address.into());

    let mut stream = event.subscribe_with_meta().await?.take(2);
    while let Some(Ok((log, meta))) = stream.next().await {
        handle_log(log, &meta).await?;
        // TODO: is this fully wired up now?
        write_checkpoint(chain_id, meta.block_number.as_u64())?;
    }
    Ok(())
}

async fn handle_log(event: MailReceivedFilter, meta: &LogMeta) -> anyhow::Result<()> {
    // this may be encrypted
    let secret_key = &CONFIG.decryption_key;
    let target_contact = if !event.address_encrypted {
        Contact::new(
            event.address_line_1,
            event.address_line_2,
            event.city,
            event.country_code,
            event.postal_or_zip,
            event.name,
        )
    } else {
        Contact::new(
            decrypt_string(secret_key, event.address_line_1)?,
            decrypt_string(secret_key, event.address_line_2)?,
            decrypt_string(secret_key, event.city)?,
            decrypt_string(secret_key, event.country_code)?,
            decrypt_string(secret_key, event.postal_or_zip)?,
            decrypt_string(secret_key, event.name)?,
        )
    };
    let target = create_contact(target_contact).await?;
    let from = get_default_sender().await?;
    let msg = if !event.msg_encrypted {
        event.msg_html
    } else {
        decrypt_string(secret_key, event.msg_html)?
    };
    let letter = Letter::new(
        from,
        target,
        msg,
        meta.transaction_hash.to_string(),
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
