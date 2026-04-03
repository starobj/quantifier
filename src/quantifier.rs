use std::{ops::{Range},  str::FromStr};

/**
Errors which can occur when attempting to interpret a string as a quantifier..

As such, the from_utf8 family of functions and methods for both Strings and &strs make use of this error, for example.
 */
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuantifierParseError;

/**
Quantifiers specify how many times a pattern must be repeated to achieve a match.
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
    /**
    Invalid quantifier.
     */
    Invalid,

    /**
    Match the pattern exactly.
     */
    One,

    /**
    Match the pattern zero or one time(s).
     */
    ZeroOrOne,

    /**
    Match the pattern zero or more times.
     */
    ZeroOrMore,

    /**
    Match the pattern one or more times.
     */
    OneOrMore,

    /**
    Match the pattern an exact number of times.
     */
    ExactCount(usize),

    /**
    Match the pattern any number of times within a range.
     */
    Range(Range<usize>),

    Not,
}

impl Quantifier {
    /**
    Convert the `Quantifier` into a `String`.
     */
    pub fn to_string(&self) -> String {
        match self {
            Quantifier::Invalid => String::from("\0"),
            Quantifier::One => String::from(""),
            Quantifier::ZeroOrOne => String::from("?"),
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
            Quantifier::Not => return String::from("^"),
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
            "^" => Ok(Quantifier::Not),
            "?" => Ok(Quantifier::ZeroOrOne),
            "*" => Ok(Quantifier::ZeroOrMore),
            "+" => Ok(Quantifier::OneOrMore),
            ""
            | " " => Ok(Quantifier::One),
            // Ok(Quantifier::Range(n..m)), Ok(Quantifier::ExactCount(n)), or Err(QuantifierParseError)
            _ => {
                if s.starts_with("{") && s.ends_with("}") {
                    let range_str = &s[1..(s.len() - 1)];

                    match range_str.split(',').collect::<Vec<&str>>().as_slice() {
                        [range_min_str, range_max_str] => {
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
        Quantifier::from_str(value).unwrap_or(Quantifier::Invalid)
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
