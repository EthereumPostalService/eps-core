use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::config::CONFIG;

pub async fn send_letter(letter: Letter) -> Result<String> {
    let client = reqwest::Client::new();
    // HACK: this is jank, we are dropping any other fields that were actualy customized in the
    // letter object. but we dont use anything else atm and the reason i cant pass the whole obj
    // is there is something wrong w the api that it doesnt let me send the full nested obj
    let req = LetterRequest {
        from: letter.from.id.unwrap(),
        to: letter.to.id.unwrap(),
        color: true,
        template: CONFIG.mail_api_template.clone(),
        merge_variables: MergeVariables {
            body: letter.html.unwrap(),
        },
        description: letter.idem_key.clone().unwrap(),
    };
    let uri = &[&CONFIG.mail_api_url, "/print-mail/v1/letters"].concat();
    let response = client
        .post(uri)
        .header("x-api-key", &CONFIG.mail_api_key)
        .header("Content-Type", "application/json")
        .header("Idempotency-key", letter.idem_key.unwrap())
        .json(&req)
        .send()
        .await?;

    let body: Letter = response.json().await?;
    // TODO
    Ok(body.id.unwrap())
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct LetterRequest {
    pub from: String,
    pub to: String,
    pub template: String,
    pub color: bool,
    pub merge_variables: MergeVariables,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct MergeVariables {
    pub body: String,
}

pub async fn create_contact(contact: Contact) -> Result<Contact> {
    let client = reqwest::Client::new();
    let uri = &[&CONFIG.mail_api_url, "/print-mail/v1/contacts"].concat();
    let response = client
        .post(uri)
        .json(&contact)
        .header("x-api-key", &CONFIG.mail_api_key)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    let contact: Contact = response.json().await?;
    Ok(contact)
}

pub async fn get_default_sender() -> Result<Contact> {
    let sender = &CONFIG.default_sender;
    let client = reqwest::Client::new();
    let uri = &[&CONFIG.mail_api_url, "/print-mail/v1/contacts/", sender].concat();
    let response = client
        .get(uri)
        .header("x-api-key", &CONFIG.mail_api_key)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    let contact: Contact = response.json().await?;
    Ok(contact)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Letter {
    pub from: Contact,
    pub to: Contact,
    pub html: Option<String>,
    pub template: Option<String>,
    pub idem_key: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub address_placement: Option<String>,
    pub double_sided: Option<bool>,
    pub color: Option<bool>,
    pub perforated_page: Option<String>,
    pub extra_service: Option<String>,
    pub envelope_type: Option<String>,
    pub return_envelope: Option<String>,
    pub send_date: Option<String>,
    pub description: Option<String>,
    pub express: Option<String>,
    pub mailing_class: Option<String>,
    pub size: Option<String>,
}

impl Letter {
    pub fn new(from: Contact, to: Contact, html: String, tx_hash: String, tx_index: u64) -> Self {
        let idem_key = format!("{}-{}", tx_hash, tx_index.to_string());
        Letter {
            idem_key: Some(idem_key),
            from,
            to,
            html: Some(html),
            id: None,
            address_placement: None,
            double_sided: None,
            color: None,
            perforated_page: None,
            extra_service: None,
            envelope_type: None,
            return_envelope: None,
            send_date: None,
            description: None,
            express: None,
            mailing_class: None,
            size: None,
            template: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub address_line_1: String,
    pub country_code: String,
    pub id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub address_line_2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub postal_or_zip: Option<String>,
    pub province_state: Option<String>,
}
impl Contact {
    pub fn new(
        address_line_1: String,
        address_line_2: String,
        city: String,
        country_code: String,
        postal_or_zip: String,
        name: String,
    ) -> Self {
        Contact {
            address_line_1: address_line_1,
            country_code: country_code,
            first_name: Some(name),
            postal_or_zip: Some(postal_or_zip),
            address_line_2: Some(address_line_2),
            city: Some(city),
            country: None,
            province_state: None,
            last_name: None,
            id: None,
        }
    }
}

#[tokio::test]
async fn test_send_mail() {
    let from_contact = Contact::new(
        "the street".to_string(),
        "unit 1".to_string(),
        "City".to_string(),
        "US".to_string(),
        "12345".to_string(),
        "name".to_string(),
    );
    let from = create_contact(from_contact).await.unwrap();
    let to_contact = Contact::new(
        "the street".to_string(),
        "unit 1".to_string(),
        "City".to_string(),
        "US".to_string(),
        "12345".to_string(),
        "name".to_string(),
    );
    let to = create_contact(to_contact).await.unwrap();
    let letter = Letter::new(to, from, "Hello world".to_string(), "txid".to_string(), 1);
    let letter = send_letter(letter).await.unwrap();
    println!("{:?}", letter);
}
