use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub rpcs: Vec<String>,
    pub contract: String,
    pub mail_api_url: String,
    pub mail_api_key: String,
    pub default_sender: String,
    pub decryption_key: String,
    pub mail_api_template: String,
}
lazy_static! {
    pub static ref CONFIG: Config = {
        let config = Config::from_cli().expect("Failed to initialize");
        config
    };
}
impl Config {
    pub fn from_cli() -> anyhow::Result<Config> {
        dotenv().ok();
        let eth_rpc = env::var("RPC_ENDPOINT_ETH").expect("RPC_ENDPOINT_ETH is not set");
        let op_rpc = env::var("RPC_ENDPOINT_OP").expect("RPC_ENDPOINT_OP is not set");
        let contract = env::var("CONTRACT").expect("CONTRACT is not set");
        let default_sender = env::var("DEFAULT_SENDER").expect("DEFAULT_SENDER is not set");
        let mail_api_url = env::var("MAIL_API_URL").expect("MAIL_API_URL is not set");
        let mail_api_key = env::var("MAIL_API_KEY").expect("MAIL_API_KEY is not set");
        let mail_api_template = env::var("TEMPLATE_ID").expect("TEMPLATE_ID is not set");
        let decryption_key = env::var("PK").expect("PK is not set");
        Ok(Config {
            rpcs: vec![eth_rpc, op_rpc],
            contract,
            mail_api_url,
            mail_api_key,
            default_sender,
            decryption_key,
            mail_api_template,
        })
    }
}
