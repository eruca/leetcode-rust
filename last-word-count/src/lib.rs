pub fn length_of_last_word(s: String) -> i32 {
    match s.split_ascii_whitespace().last() {
        Some(word) => word.len() as i32,
        None => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(length_of_last_word(" ".to_string()), 0);
    }
}