# rust_is_even
Check if interger is even using [isevenapi](https://isevenapi.xyz/).

## Usage
```
// Cargo.toml: 'tokio = { version = "1.4.0", features = ["full"] }'

extern crate rust_is_even as ie;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {    
    let n: u32 = 42;
    if ie::is_even(n).await? {
        println!("{} is even", n);
    } else {
        println!("{} is odd", n);
    }    

    Ok(())
}
```
