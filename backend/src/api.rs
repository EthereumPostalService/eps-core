use std::fs;

use anyhow::Result;
use reqwest::multipart;
use serde::{Deserialize, Serialize};

use crate::{config::CONFIG, pdf::gen_pdf};

pub async fn send_letter(letter: Letter) -> Result<String> {
    let client = reqwest::Client::new();
    let uri = &[&CONFIG.mail_api_url, "/print-mail/v1/letters"].concat();
    let letter_pdf_path = gen_pdf(letter.idem_key.clone().unwrap(), letter.html.unwrap())?;
    let pdf_file = fs::read(letter_pdf_path).unwrap();
    let pdf_file_part = reqwest::multipart::Part::bytes(pdf_file)
        .file_name(letter.idem_key.clone().unwrap())
        .mime_str("application/pdf")
        .unwrap();
    let form = multipart::Form::new()
        .text("from", letter.from.id.expect("missing `from` in letter"))
        .text("to", letter.to.id.expect("missing `to` in letter"))
        .text("addressPlacement", "insert_blank_page")
        .text("color", "true")
        .part("pdf", pdf_file_part);
    let response = client
        .post(uri)
        .header("x-api-key", &CONFIG.mail_api_key)
        .header("Idempotency-key", letter.idem_key.unwrap())
        .multipart(form)
        .send()
        .await?;
    let body: Letter = response.json().await?;
    Ok(body.id.unwrap())
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct LetterRequest {
    pub from: String,
    pub to: String,
    pub template: String,
    pub address_placement: String,
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
    let contact: Contact = get_contact_from_id(sender).await?;
    Ok(contact)
}

pub async fn get_contact_from_id(id: &str) -> Result<Contact> {
    let client = reqwest::Client::new();
    let uri = &[&CONFIG.mail_api_url, "/print-mail/v1/contacts/", id].concat();
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
    pub fn new(
        from: Contact,
        to: Contact,
        html: String,
        tx_hash: String,
        tx_index: String,
    ) -> Self {
        let idem_key = format!("{}-{}", tx_hash, tx_index);
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
            address_line_1,
            country_code,
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
#[cfg(test)]
mod test {
    use crate::api::{create_contact, get_contact_from_id, send_letter, Contact, Letter};
    use ethers::prelude::rand::{self, distributions::Alphanumeric, thread_rng};
    use lipsum::lipsum;
    use rand::Rng;

    #[tokio::test]
    async fn test_send_mail() {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

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
        let letter = Letter::new(
            to,
            from,
            "Hello world".to_string(),
            rand_string,
            "1".to_string(),
        );
        let letter = send_letter(letter).await.unwrap();
        println!("{:?}", letter);
    }

    #[tokio::test]
    async fn test_send_long_mail() {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        let to = get_contact_from_id("contact_4TpcSvfXbgccXpQNkUjUVR")
            .await
            .unwrap();
        let from = get_contact_from_id("contact_1v7o5ECeH8B6D5ogriMwga")
            .await
            .unwrap();
        let letter = Letter::new(from, to, lipsum(4000), rand_string, "1".to_string());
        let letter = send_letter(letter).await.unwrap();
        println!("{:?}", letter);
    }
}
