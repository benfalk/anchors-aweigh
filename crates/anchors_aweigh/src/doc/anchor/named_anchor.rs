use nom::bytes::complete::{tag, take_until};
use nom::character::complete::multispace1;
use nom::combinator::{recognize, rest};
use nom::sequence::delimited;
use nom::{IResult, Parser};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct NamedAnchor(String);

impl NamedAnchor {
    pub fn new<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self(value.into())
    }

    pub fn missing_start(&self, line: &str) -> bool {
        find("ANCHOR:", self.0.as_ref(), line).is_err()
    }

    pub fn missing_end(&self, line: &str) -> bool {
        find("ANCHOR_END:", self.0.as_ref(), line).is_err()
    }
}

fn find<'a>(anchor: &str, name: &str, input: &'a str) -> IResult<&'a str, &'a str> {
    let a_tag = tag(anchor);
    let token = recognize(delimited(a_tag, multispace1, tag(name)));
    delimited(take_until(anchor), token, rest).parse(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_start() {
        let line = "  # ANCHOR: test";
        let (rem, token) = find("ANCHOR:", "test", line).unwrap();
        assert_eq!(rem, "");
        assert_eq!(token, "ANCHOR: test");
    }
}
