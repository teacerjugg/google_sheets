use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub fn read_credential_from_string<T>(credential: T) -> GoogleCredential
where
    T: AsRef<str>,
{
    let credential: GoogleCredential =
        serde_json::from_str(credential.as_ref()).expect("credential string is not valid JSON");
    credential
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct GoogleCredential {
    r#type: String,
    project_id: String,
    private_key_id: String,
    private_key: String,
    client_email: String,
    client_id: String,
    auth_uri: String,
    token_uri: String,
    auth_provider_x509_cert_url: String,
    client_x509_cert_url: String,
    universe_domain: String,
}

#[derive(Debug, Serialize)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: i64,
    iat: i64,
}

impl GoogleCredential {
    pub async fn get_access_token(&self, client: &Client) -> Result<String> {
        let jwt = self.create_jwt();
        let response = client
            .post(&self.token_uri)
            .json(&json!({
                "grant_type": "urn:ietf:params:oauth:grant-type:jwt-bearer",
                "assertion": jwt,
            }))
            .send()
            .await?;

        let response_body: Value = response
            .json()
            .await
            .expect("failed to parse response body");
        let access_token = response_body
            .get("access_token")
            .expect("access_token not found")
            .as_str()
            .expect("access_token is not a string")
            .to_owned();

        Ok(access_token)
    }

    fn create_jwt(&self) -> String {
        let mut header = Header::default();
        header.typ = Some("JWT".to_owned());
        header.alg = Algorithm::RS256;

        let now = Utc::now();
        let iat = now.timestamp();
        let exp = now
            .checked_add_signed(Duration::try_seconds(60).unwrap())
            .expect("invalid timestamp")
            .timestamp();

        let claims = Claims {
            iss: self.client_email.clone(),
            scope: "https://www.googleapis.com/auth/spreadsheets".to_owned(),
            aud: self.token_uri.clone(),
            exp: exp,
            iat: iat,
        };

        let jwt = encode(
            &header,
            &claims,
            &EncodingKey::from_rsa_pem(self.private_key.as_bytes()).expect("invalid private key"),
        )
        .expect("failed to encode JWT");

        jwt
    }
}
