extern crate anyhow;
extern crate reqwest;
extern crate serde_json;

const EP: &str = "https://api.isevenapi.xyz/api/iseven/";

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct IsEvenResult {
    ad: Option<String>,
    iseven: Option<bool>,
    error: Option<String>,
}

pub async fn is_even<T: std::fmt::Display>(n: T) -> anyhow::Result<bool> {
    let url = format!("{}{}", EP, n);
    let res = reqwest::get(url).await?.json::<IsEvenResult>().await?;

    if let Some(iseven) = res.iseven {
        Ok(iseven)
    } else if let Some(error) = res.error {
        Err(anyhow::anyhow!(error))
    } else {
        panic!("Unknown response from isEven api. Is it under maintenance?");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_even(1), false);
        assert_eq!(is_even(2), true);
        assert_eq!(is_even(42), true);
        assert_eq!(is_even(43), false);
    }
}
