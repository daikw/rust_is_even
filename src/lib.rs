use thiserror::Error;

extern crate anyhow;
extern crate reqwest;
extern crate serde_json;

const EP: &str = "https://api.isevenapi.xyz/api/iseven/";

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct IsEvenResult {
    ad: Option<String>,
    is_even: Option<bool>,
    error: Option<String>,
}


#[derive(Error, Debug)]
pub enum IsEvenError {
    #[error("request failed")]
    Reqwest(#[from] reqwest::Error),
    #[error("API returned error")]
    APIError {message: String}
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
    async fn it_works() {
        assert!(!is_even(1).await.unwrap());
        assert!(is_even(2).await.unwrap());
        assert!(is_even(42).await.unwrap());
        assert!(!is_even(43).await.unwrap());
    }
}
