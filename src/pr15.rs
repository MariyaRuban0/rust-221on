use std::collections::HashMap;

fn gray(n: u8) -> Vec<String> {
    fn gray_recursive(n: u8, memo: &mut HashMap<u8, Vec<String>>) -> Vec<String> {
        if let Some(cached) = memo.get(&n) {
            return cached.clone();
        }

        if n == 0 {
            return vec![String::from("")];
        }
        if n == 1 {
            return vec![String::from("0"), String::from("1")];
        }

        let prev_gray = gray_recursive(n - 1, memo);

        let mut result = Vec::new();

        for code in &prev_gray {
            result.push(format!("0{}", code));
        }
        for code in prev_gray.iter().rev() {
            result.push(format!("1{}", code));
        }

        memo.insert(n, result.clone());
        result
    }

    let mut memo = HashMap::new();
    gray_recursive(n, &mut memo)
}

#[test]
fn test() {
    let test_data = [
        (0, vec![String::from("")]),
        (1, vec![String::from("0"), String::from("1")]),
        (2, vec![
            String::from("00"), String::from("01"),
            String::from("11"), String::from("10")
        ]),
        (3, vec![
            String::from("000"), String::from("001"),
            String::from("011"), String::from("010"),
            String::from("110"), String::from("111"),
            String::from("101"), String::from("100")
        ]),
        (4, vec![
            String::from("0000"), String::from("0001"),
            String::from("0011"), String::from("0010"),
            String::from("0110"), String::from("0111"),
            String::from("0101"), String::from("0100"),
            String::from("1100"), String::from("1101"),
            String::from("1111"), String::from("1110"),
            String::from("1010"), String::from("1011"),
            String::from("1001"), String::from("1000")
        ]),
    ];

    test_data.iter().for_each(|(n, out)| {
        assert_eq!(gray(*n), *out);
    });
}
