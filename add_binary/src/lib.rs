pub fn add_binary(a: String, b: String) -> String {
    let mut deposit = 0;
    let mut a = a;
    let mut b = b;

    if a.len() > b.len() {
        b = "0".repeat(a.len() - b.len()).to_string() + &b;
    } else if b.len() > a.len() {
        a = "0".repeat(b.len() - a.len()).to_string() + &a;
    }

    let mut result = Vec::new();
    for (a1, b1) in a.chars().rev().zip(b.chars().rev()) {
        let a1 = a1.to_digit(10).unwrap_or(0);
        let b1 = b1.to_digit(10).unwrap_or(0);
        let tmp = a1 + b1 + deposit;

        if tmp >= 2 {
            deposit = 1;
            result.push(tmp - 2);
        } else {
            deposit = 0;
            result.push(tmp);
        }
    }
    if deposit == 1 {
        result.push(1);
    }

    result.into_iter().map(|c| c.to_string()).rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            add_binary("11".to_string(), "1".to_string()),
            "100".to_string()
        );
        assert_eq!(
            add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
    }
}
