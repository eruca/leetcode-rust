pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let mut deposit = 1;

    for i in (0..digits.len()).rev() {
        if deposit == 1 {
            if digits[i] == 9 {
                digits[i] = 0;
            } else {
                digits[i] += 1;
                deposit = 0;
            }
        } else {
            break;
        }
    }

    if deposit == 1 {
        let mut result = Vec::with_capacity(digits.len() + 1);
        result.push(1);
        result.extend(digits);
        result
    } else {
        digits
    }

    // let result: Vec<i32> = digits
    //     .iter()
    //     .rev()
    //     .map(|&d| {
    //         if deposit == 1 {
    //             if d == 9 {
    //                 0
    //             } else {
    //                 deposit = 0;
    //                 d + 1
    //             }
    //         } else {
    //             d
    //         }
    //     })
    //     .collect();

    //     result.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(plus_one(vec![9, 9, 9, 9]), vec![1, 0, 0, 0, 0]);
    }
}
