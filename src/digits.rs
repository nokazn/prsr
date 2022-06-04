pub fn digits(s: &str) -> Option<(i64, &str)> {
    let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    match s[..end].parse() {
        Ok(value) => Some((value, &s[end..])),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestCase<I, E> {
        input: I,
        expected: E,
    }

    #[test]
    fn test_digits() {
        let cases = [
            TestCase {
                input: "123*456",
                expected: Some((123, "*456")),
            },
            TestCase {
                input: "123 456",
                expected: Some((123, " 456")),
            },
            TestCase {
                input: "123456",
                expected: Some((123456, "")),
            },
        ];

        cases.iter().for_each(|case| {
            assert_eq!(
                digits(case.input),
                case.expected,
                "Failed in the {:?}.",
                case
            );
        })
    }
}

// Fn(&str) -> Option<(T, $str)>
