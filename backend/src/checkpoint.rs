use std::fs;

use ethers::types::U256;

pub fn write_checkpoint(chain_id: U256, block_number: u64) -> anyhow::Result<()> {
    // write to disk cache
    let key = get_key(chain_id);
    fs::write(key, block_number.to_string())?;
    Ok(())
}

pub fn get_checkpoint(chain_id: U256) -> anyhow::Result<u64> {
    let key = get_key(chain_id);
    let block_number: u64 = fs::read_to_string(key)?.parse()?;
    Ok(block_number)
}

fn get_key(chain_id: U256) -> String {
    format!(".checkpoint:{chain_id:?}").to_string()
}