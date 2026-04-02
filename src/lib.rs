use std::{ops::Range, str::FromStr};

#[derive(Clone, Copy, Debug)]
pub struct QuantifierParseError;

/**
A quantifier is a representation of how many times a pattern should be matched.
For example, for the pattern `a`:

None:
- `"a"`

ZeroOrOne:
- `""`
- `"a"`

ZeroOrMore:
- `""`
- `"a"`
- `"aa"`
- `"aaa"`
- `...`

OneOrMore:
- `"a"`
- `"aa"`
- `"aaa"`
- `...`
 */
#[derive(Clone, Debug, PartialEq)]
pub enum Quantifier {
    None,
    Optional,
    ZeroOrMore,
    OneOrMore,
    ExactCount(usize),
    Range(Range<usize>)
}

impl Quantifier {
    /**
    Convert the `Quantifier` into a `String`.
     */
    pub fn to_string(&self) -> String {
        match self {
            Quantifier::None => String::from(""),
            Quantifier::Optional => String::from("?"),
            Quantifier::ZeroOrMore => String::from("*"),
            Quantifier::OneOrMore => String::from("+"),
            Quantifier::ExactCount(count) => format!("{{{}}}", count),
            Quantifier::Range(range) => {
                if range.start == usize::MIN && range.end == usize::MAX {
                    return String::from("*");
                }
                if range.start == usize::MIN {
                    return format!("{{,{}}}", range.end);
                }
                if range.end == usize::MAX {
                    return format!("{{{},}}", range.start);
                }

                format!("{{{},{}}}", range.start, range.end)
            },
        }
    }
}

impl FromStr for Quantifier {
    type Err = QuantifierParseError;

    /**
    Parse a `Quantifier`.
     */
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = Err(QuantifierParseError);

        match s {
            "?" => Ok(Quantifier::Optional),
            "*" => Ok(Quantifier::ZeroOrMore),
            "+" => Ok(Quantifier::OneOrMore),
            ""
            | " "
            | "\0" => Ok(Quantifier::None),
            // Ok(Quantifier::Range(n..m)), Ok(Quantifier::ExactCount(n)), or Err(QuantifierParseError)
            _ => {
                if s.starts_with("{") && s.ends_with("}") {
                    let range_str = &s[1..(s.len() - 1)];

                    match range_str.split(',').collect::<Vec<&str>>().as_slice() {
                        [range_min_str, range_max_str] => {
                            println!("Range({}..{})", range_min_str, range_max_str);
                            let range_min: usize;
                            let range_max: usize;

                            if range_min_str.len() > 0 {
                                let range_min_result = range_min_str.parse::<usize>();

                                if range_min_result.is_err() {
                                    return err;
                                }

                                range_min = range_min_result.unwrap();
                            }
                            else {
                                range_min = usize::MIN;
                            }

                            if range_max_str.len() > 0 {
                                let range_max_result = range_max_str.parse::<usize>();

                                if range_max_result.is_err() {
                                    return err;
                                }

                                range_max = range_max_result.unwrap();
                            }
                            else {
                                range_max = usize::MAX;
                            }

                            if range_min == usize::MIN && range_max == usize::MAX {
                                return Err(QuantifierParseError);
                            }

                            return Ok(Quantifier::Range(range_min..range_max));
                        },
                        [exact_count_str] => {
                            let exact_count_result = exact_count_str.parse::<usize>();

                            if exact_count_result.is_err() {
                                return err;
                            }

                            let exact_count = exact_count_result.unwrap();

                            return Ok(Quantifier::ExactCount(exact_count))
                        },
                        _ => return err,
                    }
                }

                err
            },
        }
    }
}

impl From<&str> for Quantifier {
    /**
    Parse a `Quantifier`.
     */
    fn from(value: &str) -> Self {
        Quantifier::from_str(value).unwrap_or(Quantifier::None)
    }
}

impl From<String> for Quantifier {
    /**
    Parse a `Quantifier`.
     */
    fn from(value: String) -> Self {
        Quantifier::from(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_invalid() {
        assert_eq!(Quantifier::from("42"), Quantifier::None);
        assert_eq!(Quantifier::from("{}"), Quantifier::None);
        assert_eq!(Quantifier::from("{,}"), Quantifier::None);
        assert_eq!(Quantifier::from("{,,}"), Quantifier::None);
        assert_eq!(Quantifier::from("{2,,}"), Quantifier::None);
        assert_eq!(Quantifier::from("{,4,}"), Quantifier::None);
        assert_eq!(Quantifier::from("{,,6}"), Quantifier::None);
    }

    #[test]
    fn from_str_valid() {
        assert_eq!(Quantifier::from(""), Quantifier::None);
        assert_eq!(Quantifier::from(" "), Quantifier::None);
        assert_eq!(Quantifier::from("\0"), Quantifier::None);
        assert_eq!(Quantifier::from("?"), Quantifier::Optional);
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
        assert_eq!(Quantifier::from(String::from("42")), Quantifier::None);
        assert_eq!(Quantifier::from(String::from("{}")), Quantifier::None);
        assert_eq!(Quantifier::from(String::from("{,}")), Quantifier::None);
        assert_eq!(Quantifier::from(String::from("{,,}")), Quantifier::None);
        assert_eq!(Quantifier::from(String::from("{2,,}")), Quantifier::None);
        assert_eq!(Quantifier::from(String::from("{,4,}")), Quantifier::None);
        assert_eq!(Quantifier::from(String::from("{,,6}")), Quantifier::None);
    }

    #[test]
    fn from_string_valid() {
        assert_eq!(Quantifier::from(String::from("")), Quantifier::None);
        assert_eq!(Quantifier::from(String::from(" ")), Quantifier::None);
        assert_eq!(Quantifier::from(String::from("\0")), Quantifier::None);
        assert_eq!(Quantifier::from(String::from("?")), Quantifier::Optional);
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
    fn to_string() {
        assert_eq!(Quantifier::None.to_string(), String::from(""));
        assert_eq!(Quantifier::Optional.to_string(), String::from("?"));
        assert_eq!(Quantifier::ZeroOrMore.to_string(), String::from("*"));
        assert_eq!(Quantifier::OneOrMore.to_string(), String::from("+"));
        assert_eq!(Quantifier::ExactCount(42).to_string(), String::from("{42}"));
        assert_eq!(Quantifier::Range(2..4).to_string(), String::from("{2,4}"));
        assert_eq!(Quantifier::Range(2..usize::MAX).to_string(), String::from("{2,}"));
        assert_eq!(Quantifier::Range(usize::MIN..4).to_string(), String::from("{,4}"));
    }
}
