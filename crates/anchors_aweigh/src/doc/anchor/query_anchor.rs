//! Query Anchor

use nom::branch::alt;
use nom::bytes::complete::{is_a, tag, take_until};
use nom::character::complete::{alphanumeric1, space1};
use nom::combinator::{eof, map, rest, value};
use nom::error::Error;
use nom::multi::{fold_many0, many1};
use nom::sequence::{preceded, separated_pair, terminated};
use nom::{Finish, IResult, Parser};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, thiserror::Error)]
#[error("{0:?}")]
pub struct ParseQueryAnchorError(#[from] Error<String>);

type Bindings = BTreeMap<String, String>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QueryAnchor {
    pub name: String,
    pub bindings: Bindings,
}

impl QueryAnchor {
    pub fn parse(input: &str) -> Result<Self, ParseQueryAnchorError> {
        let (_, anchor) = query_anchor
            .parse(input)
            .finish()
            .map_err(|err| ParseQueryAnchorError(err.into()))?;

        Ok(anchor)
    }
}

pub(super) fn query_anchor(input: &str) -> IResult<&str, QueryAnchor> {
    (anchor_name, anchor_bindings)
        .map(|(name, bindings)| QueryAnchor { name, bindings })
        .parse(input)
}

fn anchor_name(input: &str) -> IResult<&str, String> {
    alt((take_until("?"), rest)).map(String::from).parse(input)
}

fn target_chars(input: &str) -> IResult<&str, String> {
    map(
        many1(alt((
            alphanumeric1,
            space1,
            is_a("@!\"'$%^*_-+()<>[]{}/|;`"),
            preceded(
                tag("\\"),
                alt((tag("\\"), tag("="), tag("&"), tag("?"), tag("#"), tag(":"))),
            ),
        ))),
        |v| v.join(""),
    )
    .parse(input)
}

fn anchor_bindings(input: &str) -> IResult<&str, Bindings> {
    let target = alt((terminated(target_chars, tag("&")), target_chars));
    alt((
        value(Bindings::default(), eof),
        preceded(
            tag("?"),
            fold_many0(
                separated_pair(alphanumeric1, tag("="), target),
                Bindings::default,
                |mut hashmap, (left, right)| {
                    hashmap.insert(String::from(left), String::from(right));
                    hashmap
                },
            ),
        ),
    ))
    .parse(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing_works() {
        let anchor = QueryAnchor::parse("class?name=Foo").unwrap();
        assert_eq!(anchor.name, "class");
        assert_eq!(anchor.bindings.get("name").unwrap(), "Foo");

        let anchor = QueryAnchor::parse("mod").unwrap();
        assert_eq!(anchor.name, "mod");
        assert!(anchor.bindings.is_empty());

        let anchor = QueryAnchor::parse("class?name=Foo bar&name2=\\?buzz\\&_\"").unwrap();
        assert_eq!(anchor.name, "class");
        assert_eq!(anchor.bindings.get("name").unwrap(), "Foo bar");
        assert_eq!(anchor.bindings.get("name2").unwrap(), "?buzz&_\"");
    }
}
