pub mod quantify;
pub mod quantify_vec;
pub mod quantifier;

pub use quantify::*;
pub use quantify_vec::*;
pub use quantifier::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_invalid() {
        assert_eq!(Quantifier::from("\0"), Quantifier::Invalid);
        assert_eq!(Quantifier::from("42"), Quantifier::Invalid);
        assert_eq!(Quantifier::from("42"), Quantifier::Invalid);
        assert_eq!(Quantifier::from("{}"), Quantifier::Invalid);
        assert_eq!(Quantifier::from("{,}"), Quantifier::Invalid);
        assert_eq!(Quantifier::from("{,,}"), Quantifier::Invalid);
        assert_eq!(Quantifier::from("{2,,}"), Quantifier::Invalid);
        assert_eq!(Quantifier::from("{,4,}"), Quantifier::Invalid);
        assert_eq!(Quantifier::from("{,,6}"), Quantifier::Invalid);
    }

    #[test]
    fn from_str_valid() {
        assert_eq!(Quantifier::from(""), Quantifier::One);
        assert_eq!(Quantifier::from(" "), Quantifier::One);
        assert_eq!(Quantifier::from("?"), Quantifier::ZeroOrOne);
        assert_eq!(Quantifier::from("*"), Quantifier::ZeroOrMore);
        assert_eq!(Quantifier::from("+"), Quantifier::OneOrMore);
    }

    #[test]
    fn from_str_exact_count_valid() {
        assert_eq!(Quantifier::from("{42}"), Quantifier::ExactCount(42));
    }

    #[test]
    fn from_str_range_full_valid() {
        assert_eq!(Quantifier::from("{2,4}"), Quantifier::Range(2..4));
    }

    #[test]
    fn from_str_range_max_valid() {
        assert_eq!(Quantifier::from("{2,}"), Quantifier::Range(2..usize::MAX));
    }

    #[test]
    fn from_str_range_min_valid() {
        assert_eq!(Quantifier::from("{,4}"), Quantifier::Range(usize::MIN..4));
    }

    #[test]
    fn from_string_invalid() {
        assert_eq!(Quantifier::from(String::from("\0")), Quantifier::Invalid);
        assert_eq!(Quantifier::from(String::from("42")), Quantifier::Invalid);
        assert_eq!(Quantifier::from(String::from("{}")), Quantifier::Invalid);
        assert_eq!(Quantifier::from(String::from("{,}")), Quantifier::Invalid);
        assert_eq!(Quantifier::from(String::from("{,,}")), Quantifier::Invalid);
        assert_eq!(Quantifier::from(String::from("{2,,}")), Quantifier::Invalid);
        assert_eq!(Quantifier::from(String::from("{,4,}")), Quantifier::Invalid);
        assert_eq!(Quantifier::from(String::from("{,,6}")), Quantifier::Invalid);
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
    fn quantify_vec_match_one() {
        let v: Vec<usize> = vec![1, 2, 1, 2];
        let pattern: Vec<usize> = vec![1, 2, 1, 2];

        let actual = v.matches(&pattern.iter(), &Quantifier::One);

        let expected: Vec<&[usize]> = vec![&v[..]];

        assert_eq!(actual, expected);
    }

    #[test]
    fn quantify_vec_match_exact_count() {
        let v: Vec<usize> = vec![1, 2, 1, 2];
        let pattern: Vec<usize> = vec![1, 2];

        let actual = v.matches(&pattern.iter(), &Quantifier::ExactCount(2));

        let expected: Vec<&[usize]> = vec![&v[..]];

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
