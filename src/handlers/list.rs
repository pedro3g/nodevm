use core::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ResponseData {
    version: String,
}

#[derive(Debug)]
pub enum CustomError {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::Reqwest(e) => write!(f, "Reqwest Error: {}", e),
            CustomError::Serde(e) => write!(f, "Serde Error: {}", e),
        }
    }
}

impl std::error::Error for CustomError {}

impl From<reqwest::Error> for CustomError {
    fn from(err: reqwest::Error) -> CustomError {
        CustomError::Reqwest(err)
    }
}

impl From<serde_json::Error> for CustomError {
    fn from(err: serde_json::Error) -> CustomError {
        CustomError::Serde(err)
    }
}

pub async fn list() -> Result<(), CustomError> {
    let url = "https://nodejs.org/download/release/index.json";

    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let response_text = response.text().await?;

        let versions: Vec<ResponseData> = serde_json::from_str(&response_text)?;

        for version in versions {
            println!("{}", version.version);
        }

        return Ok(());
    } else {
        println!("Erro: Código de status {}", response.status());
    }

    Ok(())
}
