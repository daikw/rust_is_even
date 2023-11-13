use thiserror::Error;

extern crate anyhow;
extern crate reqwest;
extern crate serde_json;

const EP: &str = "https://api.isevenapi.xyz/api/iseven/";

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct IsEvenResult {
    ad: Option<String>,
    #[serde(rename = "iseven")]
    is_even: Option<bool>,
    error: Option<String>,
}

#[derive(Error, Debug)]
pub enum IsEvenError {
    #[error("request failed")]
    Reqwest(#[from] reqwest::Error),
    #[error("API returned error: {message:?}")]
    APIError { message: String },
}

pub async fn is_even<T: std::fmt::Display>(n: T) -> Result<bool, IsEvenError> {
    let url = format!("{}{}", EP, n);
    let res = reqwest::get(url).await?.json::<IsEvenResult>().await?;

    if let Some(is_even) = res.is_even {
        Ok(is_even)
    } else if let Some(error) = res.error {
        Err(IsEvenError::APIError { message: error })
    } else {
        panic!("Unknown response from isEven api. Is it under maintenance?");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn even() {
        let even = [2, 42, 100, 1000, 1002, 99998];

        for number in even {
            assert!(is_even(number).await.unwrap());
        }
    }

    #[tokio::test]
    async fn odd() {
        let odd = [1, 3, 7, 11, 9999, 888887];

        for number in odd {
            assert!(!is_even(number).await.unwrap());
        }
    }

    #[tokio::test]
    async fn with_error() {
        assert!(is_even("n").await.is_err());
    }
}
