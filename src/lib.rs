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

pub fn is_even<T: std::fmt::Debug>(n: T) -> bool {
    println!("{:?}", n);

    let n = format!("{:?}", n).parse::<i32>().unwrap();
    n % 2 == 0
}
