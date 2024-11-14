use std::{fmt, str::FromStr};

use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

const KEY: &[u8] = b"01234567890123456789012345678901";

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    #[serde(rename = "lastName")]
    name: String,
    email: Option<String>,
    age: u32,
    skills: Vec<String>,
    date_of_birth: DateTime<Utc>,
    state: WorkState,
    #[serde(serialize_with = "b64_encode", deserialize_with = "b64_decode")]
    data: Vec<u8>,
    // #[serde(
    //     serialize_with = "process_encrypt",
    //     deserialize_with = "process_decrypt"
    // )]
    #[serde_as(as = "DisplayFromStr")]
    encrypted_data: EncryptedData,
    #[serde_as(as = "DisplayFromStr")]
    bar: u8,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    url: Vec<http::Uri>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "details")]
enum WorkState {
    Working(String),
    OnLeave(DateTime<Utc>),
    Terminated,
}

#[derive(Debug)]
struct EncryptedData(String);

fn main() -> Result<()> {
    let user = User {
        name: "Alice".to_string(),
        email: Some("alice@example.com".to_string()),
        age: 30,
        skills: vec!["Rust".to_string(), "C++".to_string()],
        date_of_birth: Utc::now(),
        state: WorkState::OnLeave(Utc::now()),
        data: b"Hello, World!".to_vec(),
        encrypted_data: EncryptedData::new("yu tian"),
        bar: 42,
        url: vec!["http://example.com".parse()?],
    };

    let json = serde_json::to_string(&user)?;
    println!("{}", json);

    let user: User = serde_json::from_str(&json)?;
    println!("{:?}", user);
    println!("{:?}", user.url[0].host());

    Ok(())
}

fn b64_encode<S>(data: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let encoded = URL_SAFE_NO_PAD.encode(data);
    serializer.serialize_str(&encoded)
}

fn b64_decode<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let encoded = String::deserialize(deserializer)?;
    let decoded = URL_SAFE_NO_PAD
        .decode(encoded.as_bytes())
        .map_err(serde::de::Error::custom)?;
    Ok(decoded)
}

#[allow(dead_code)]
fn process_encrypt<S>(data: &str, serilaizer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let encrypted = encypt(data.as_bytes()).map_err(serde::ser::Error::custom)?;
    serilaizer.serialize_str(&encrypted)
}

#[allow(dead_code)]
fn process_decrypt<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let encrypted = String::deserialize(deserializer)?;
    let decrypted = decrypt(&encrypted).map_err(serde::de::Error::custom)?;
    let decrypted = String::from_utf8(decrypted).map_err(serde::de::Error::custom)?;
    Ok(decrypted)
}

fn encypt(data: &[u8]) -> Result<String> {
    let cipher = ChaCha20Poly1305::new(KEY.into());
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, data).unwrap();
    let nonce_cypertext: Vec<_> = nonce.iter().copied().chain(ciphertext).collect();
    let encoded = URL_SAFE_NO_PAD.encode(nonce_cypertext);
    Ok(encoded)
}

fn decrypt(encoded: &str) -> Result<Vec<u8>> {
    let decoded = URL_SAFE_NO_PAD.decode(encoded.as_bytes())?;
    let cipher = ChaCha20Poly1305::new(KEY.into());
    let nonce = decoded[..12].into();
    let decrypted = cipher.decrypt(nonce, &decoded[12..]).unwrap();
    Ok(decrypted)
}

impl fmt::Display for EncryptedData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let encrypted = encypt(self.0.as_bytes()).unwrap();
        write!(f, "{}", encrypted)
    }
}

impl FromStr for EncryptedData {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decrypted = decrypt(s)?;
        let decrypted = String::from_utf8(decrypted)?;
        Ok(Self(decrypted))
    }
}

impl EncryptedData {
    fn new(data: impl Into<String>) -> Self {
        Self(data.into())
    }
}
