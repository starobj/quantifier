pub mod quantify;
pub mod quantify_vec;
pub mod quantifier;

pub use quantify::*;
pub use quantifier::*;

#[cfg(test)]
mod tests {
    use std::{str::FromStr};

    use super::*;

    #[test]
    fn from_str_invalid() {
        assert_eq!(Quantifier::from_str("\0"), Err(QuantifierParseError));
        assert_eq!(Quantifier::from_str("42"), Err(QuantifierParseError));
        assert_eq!(Quantifier::from_str("42"), Err(QuantifierParseError));
        assert_eq!(Quantifier::from_str("{}"), Err(QuantifierParseError));
        assert_eq!(Quantifier::from_str("{,}"), Err(QuantifierParseError));
        assert_eq!(Quantifier::from_str("{,,}"), Err(QuantifierParseError));
        assert_eq!(Quantifier::from_str("{2,,}"), Err(QuantifierParseError));
        assert_eq!(Quantifier::from_str("{,4,}"), Err(QuantifierParseError));
        assert_eq!(Quantifier::from_str("{,,6}"), Err(QuantifierParseError));
    }

    #[test]
    fn from_str_valid() {
        assert_eq!(Quantifier::from_str("").unwrap(), Quantifier::One);
        assert_eq!(Quantifier::from_str(" ").unwrap(), Quantifier::One);
        assert_eq!(Quantifier::from_str("?").unwrap(), Quantifier::ZeroOrOne);
        assert_eq!(Quantifier::from_str("*").unwrap(), Quantifier::ZeroOrMore);
        assert_eq!(Quantifier::from_str("+").unwrap(), Quantifier::OneOrMore);
        assert_eq!(Quantifier::from_str("{2,4}").unwrap(), Quantifier::Range(2..4));
    }

    #[test]
    fn from_invalid() {
        assert_eq!(Quantifier::from("\0"), Quantifier::One);
        assert_eq!(Quantifier::from("42"), Quantifier::One);
        assert_eq!(Quantifier::from("42"), Quantifier::One);
        assert_eq!(Quantifier::from("{}"), Quantifier::One);
        assert_eq!(Quantifier::from("{,}"), Quantifier::One);
        assert_eq!(Quantifier::from("{,,}"), Quantifier::One);
        assert_eq!(Quantifier::from("{2,,}"), Quantifier::One);
        assert_eq!(Quantifier::from("{,4,}"), Quantifier::One);
        assert_eq!(Quantifier::from("{,,6}"), Quantifier::One);
    }

    #[test]
    fn from_valid() {
        assert_eq!(Quantifier::from(""), Quantifier::One);
        assert_eq!(Quantifier::from(" "), Quantifier::One);
        assert_eq!(Quantifier::from("?"), Quantifier::ZeroOrOne);
        assert_eq!(Quantifier::from("*"), Quantifier::ZeroOrMore);
        assert_eq!(Quantifier::from("+"), Quantifier::OneOrMore);
    }

    #[test]
    fn from_exact_count_valid() {
        assert_eq!(Quantifier::from("{42}"), Quantifier::ExactCount(42));
    }

    #[test]
    fn from_range_full_valid() {
        assert_eq!(Quantifier::from("{2,4}"), Quantifier::Range(2..4));
    }

    #[test]
    fn from_range_max_valid() {
        assert_eq!(Quantifier::from("{2,}"), Quantifier::Range(2..usize::MAX));
    }

    #[test]
    fn from_range_min_valid() {
        assert_eq!(Quantifier::from("{,4}"), Quantifier::Range(usize::MIN..4));
    }

    #[test]
    fn from_string_invalid() {
        assert_eq!(Quantifier::from(String::from("\0")), Quantifier::One);
        assert_eq!(Quantifier::from(String::from("42")), Quantifier::One);
        assert_eq!(Quantifier::from(String::from("{}")), Quantifier::One);
        assert_eq!(Quantifier::from(String::from("{,}")), Quantifier::One);
        assert_eq!(Quantifier::from(String::from("{,,}")), Quantifier::One);
        assert_eq!(Quantifier::from(String::from("{2,,}")), Quantifier::One);
        assert_eq!(Quantifier::from(String::from("{,4,}")), Quantifier::One);
        assert_eq!(Quantifier::from(String::from("{,,6}")), Quantifier::One);
    }

    #[test]
    fn from_string_valid() {
        assert_eq!(Quantifier::from(String::from("")), Quantifier::One);
        assert_eq!(Quantifier::from(String::from(" ")), Quantifier::One);
        assert_eq!(Quantifier::from(String::from("?")), Quantifier::ZeroOrOne);
        assert_eq!(Quantifier::from(String::from("*")), Quantifier::ZeroOrMore);
        assert_eq!(Quantifier::from(String::from("+")), Quantifier::OneOrMore);
    }

    #[test]
    fn from_string_exact_count_valid() {
        assert_eq!(Quantifier::from(String::from("{42}")), Quantifier::ExactCount(42));
    }

    #[test]
    fn from_string_range_full_valid() {
        assert_eq!(Quantifier::from(String::from("{2,4}")), Quantifier::Range(2..4));
    }

    #[test]
    fn from_string_range_max_valid() {
        assert_eq!(Quantifier::from(String::from("{2,}")), Quantifier::Range(2..usize::MAX));
    }

    #[test]
    fn from_string_range_min_valid() {
        assert_eq!(Quantifier::from(String::from("{,4}")), Quantifier::Range(usize::MIN..4));
    }

    #[test]
    fn quantify_vec_matches_all_one() {
        let v: Vec<i32> = vec![1, 2, 1, 2];
        let patterns_source = vec![vec![1, 2, 1, 2]];
        let patterns = build_patterns(patterns_source.as_slice());

        let actual = v.matches_all(&patterns, &Quantifier::One);

        let expected: Vec<&[i32]> = vec![&v[..]];

        assert_eq!(actual, expected);
    }

    #[test]
    fn quantify_vec_matches_all_exact_count() {
        let v: Vec<i32> = vec![1, 2, 1, 2];
        let patterns_source = vec![vec![1, 2]];
        let patterns = build_patterns(patterns_source.as_slice());

        let actual = v.matches_all(&patterns, &Quantifier::ExactCount(2));

        let expected: Vec<&[i32]> = vec![&v[..]];

        assert_eq!(actual, expected);
    }

    #[test]
    fn quantify_vec_matches_pattern_one() {
        let v: Vec<usize> = vec![1, 2, 1, 2];
        let pattern: Vec<usize> = vec![1, 2, 1, 2];

        let actual = v.matches_pattern(&pattern.iter(), &Quantifier::One);

        let expected: Vec<&[usize]> = vec![&v[..]];

        assert_eq!(actual, expected);
    }

    #[test]
    fn quantify_vec_matches_pattern_exact_count() {
        let v: Vec<usize> = vec![1, 2, 1, 2];
        let pattern: Vec<usize> = vec![1, 2];

        let actual = v.matches_pattern(&pattern.iter(), &Quantifier::ExactCount(2));

        let expected: Vec<&[usize]> = vec![&v[..]];

        assert_eq!(actual, expected);
    }

    #[test]
    fn quantify_vec_matches_pattern_zero_or_one() {
        let v: Vec<usize> = vec![1, 2, 1, 2];
        let empty = &v[..0];
        let one_two = &v[0..=1];
        let pattern_valid_a: Vec<usize> = vec![1, 2];
        let pattern_valid_b: Vec<usize> = vec![1];

        let mut actual = v.matches_pattern(&pattern_valid_a.iter(), &Quantifier::ZeroOrOne);
        let mut expected: Vec<&[usize]> = vec![empty, one_two, empty, empty, one_two, empty, empty];

        assert_eq!(actual, expected);

        actual = v.matches_pattern(&pattern_valid_b.iter(), &Quantifier::ZeroOrOne);
        expected = vec![empty, one_two, empty, empty];

        assert_eq!(actual, expected);
    }

    #[test]
    fn to_string() {
        assert_eq!(Quantifier::One.to_string(), String::from(""));
        assert_eq!(Quantifier::ZeroOrOne.to_string(), String::from("?"));
        assert_eq!(Quantifier::ZeroOrMore.to_string(), String::from("*"));
        assert_eq!(Quantifier::OneOrMore.to_string(), String::from("+"));
        assert_eq!(Quantifier::ExactCount(42).to_string(), String::from("{42}"));
        assert_eq!(Quantifier::Range(2..4).to_string(), String::from("{2,4}"));
        assert_eq!(Quantifier::Range(2..usize::MAX).to_string(), String::from("{2,}"));
        assert_eq!(Quantifier::Range(usize::MIN..4).to_string(), String::from("{,4}"));
    }
}
