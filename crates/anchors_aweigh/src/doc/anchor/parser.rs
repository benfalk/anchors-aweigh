use super::{query_anchor::query_anchor, Anchor, Decoration, Link, NamedAnchor, Strategy};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{alphanumeric1, digit1, multispace0};
use nom::combinator::{eof, map_res, opt, recognize, value};
use nom::error::Error;
use nom::multi::many1_count;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated};
use nom::{Finish, IResult, Parser};
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
#[error("{0:?}")]
pub struct ParseError(#[from] Error<String>);

pub(super) fn parse(source: &str) -> Result<Anchor, ParseError> {
    let (rem, decoration) = preceded(multispace0, decoration_part)
        .parse(source)
        .finish()
        .map_err(|err| ParseError(err.into()))?;

    let (_, link) = delimited(multispace0, link_part, multispace0)
        .parse(rem)
        .finish()
        .map_err(|err| ParseError(err.into()))?;

    Ok(Anchor { decoration, link })
}

// == Link Parsing

fn link_part(source: &str) -> IResult<&str, Link> {
    (path_part, strategy_part)
        .map(|(path, strategy)| Link { path, strategy })
        .parse(source)
}

// == Decoration Parsing

fn decoration_part(input: &str) -> IResult<&str, Decoration> {
    opt(delimited(tag("["), leftshift_decoration, tag("]")))
        .map(Option::unwrap_or_default)
        .parse(input)
}

fn leftshift_decoration(input: &str) -> IResult<&str, Decoration> {
    value(Decoration::LeftShift, tag("<")).parse(input)
}

// == Path Parsing

fn path_part(input: &str) -> IResult<&str, PathBuf> {
    alt((path_with_strategy, path_with_no_strategy)).parse(input)
}

fn path_with_strategy(input: &str) -> IResult<&str, PathBuf> {
    is_not(":#").map(PathBuf::from).parse(input)
}

fn path_with_no_strategy(input: &str) -> IResult<&str, PathBuf> {
    terminated(is_not(" "), multispace0)
        .map(PathBuf::from)
        .parse(input)
}

// == Strategy Parsing

fn strategy_part(input: &str) -> IResult<&str, Strategy> {
    alt((
        full_range_strategy,
        query_strategy,
        single_line_strategy,
        down_to_strategy,
        here_down_strategy,
        between_lines_strategy,
        named_strategy,
    ))
    .parse(input)
}

fn full_range_strategy(input: &str) -> IResult<&str, Strategy> {
    eof.map(|_| Strategy::Full).parse(input)
}

fn query_strategy(input: &str) -> IResult<&str, Strategy> {
    preceded(tag("#"), query_anchor)
        .map(Strategy::Query)
        .parse(input)
}

fn single_line_strategy(input: &str) -> IResult<&str, Strategy> {
    delimited(tag(":"), usize_number, eof)
        .map(Strategy::ThisLine)
        .parse(input)
}

fn down_to_strategy(input: &str) -> IResult<&str, Strategy> {
    delimited(tag("::"), usize_number, eof)
        .map(Strategy::DownTo)
        .parse(input)
}

fn here_down_strategy(input: &str) -> IResult<&str, Strategy> {
    delimited(tag(":"), usize_number, pair(tag(":"), eof))
        .map(Strategy::HereDown)
        .parse(input)
}

fn between_lines_strategy(input: &str) -> IResult<&str, Strategy> {
    preceded(
        tag(":"),
        separated_pair(usize_number, tag(":"), usize_number).map(|(left, right)| {
            let mut lines = [left, right];
            lines.sort();
            Strategy::Between {
                start: lines[0],
                end: lines[1],
            }
        }),
    )
    .parse(input)
}

fn named_strategy(input: &str) -> IResult<&str, Strategy> {
    delimited(tag(":"), named_token, eof)
        .map(|token| Strategy::Named(NamedAnchor::new(token)))
        .parse(input)
}

// Helpers

fn usize_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse).parse(input)
}

fn named_token(input: &str) -> IResult<&str, &str> {
    recognize(
        // at least one of
        many1_count(alt((
            // these variants
            alphanumeric1,
            tag("_"),
            tag("-"),
        ))),
    )
    .parse(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::doc::QueryAnchor;

    #[test]
    fn decoration_part_works() {
        let (rem, decoration) = decoration_part("[<]").unwrap();
        assert_eq!(rem, "");
        assert_eq!(decoration, Decoration::LeftShift);

        let (rem, decoration) = decoration_part("file.rb#class?name=Foo").unwrap();
        assert_eq!(rem, "file.rb#class?name=Foo");
        assert_eq!(decoration, Decoration::None);
    }

    #[test]
    fn path_part_works() {
        let (rem, string) = path_part("path.rb:example").unwrap();
        assert_eq!(rem, ":example");
        assert_eq!(string, PathBuf::from("path.rb"));

        let (rem, string) = path_part("path.rb#class?name=Foo").unwrap();
        assert_eq!(rem, "#class?name=Foo");
        assert_eq!(string, PathBuf::from("path.rb"));

        let (rem, string) = path_part("path.rb").unwrap();
        assert_eq!(rem, "");
        assert_eq!(string, PathBuf::from("path.rb"));
    }

    #[test]
    fn path_with_no_strategy_works() {
        let (rem, path) = path_with_no_strategy("file.rb ").unwrap();
        assert_eq!(rem, "");
        assert_eq!(path, PathBuf::from("file.rb"));
    }

    #[test]
    fn strategy_part_works() {
        let (rem, strat) = strategy_part("").unwrap();
        assert_eq!(rem, "");
        assert_eq!(strat, Strategy::Full);

        let (rem, strat) = strategy_part("#class?name=Foo").unwrap();
        assert_eq!(rem, "");
        assert_eq!(
            strat,
            Strategy::Query(QueryAnchor::parse("class?name=Foo").unwrap())
        );

        let (rem, strat) = strategy_part(":42").unwrap();
        assert_eq!(rem, "");
        assert_eq!(strat, Strategy::ThisLine(42));

        let (rem, strat) = strategy_part("::42").unwrap();
        assert_eq!(rem, "");
        assert_eq!(strat, Strategy::DownTo(42));

        let (rem, strat) = strategy_part(":42:").unwrap();
        assert_eq!(rem, "");
        assert_eq!(strat, Strategy::HereDown(42));

        let (rem, strat) = strategy_part(":42:69").unwrap();
        assert_eq!(rem, "");
        assert_eq!(strat, Strategy::Between { start: 42, end: 69 });

        let (rem, strat) = strategy_part(":_pink-floyd").unwrap();
        assert_eq!(rem, "");
        assert_eq!(strat, Strategy::Named(NamedAnchor::new("_pink-floyd")));
    }

    #[test]
    fn parser_full_strategy() {
        let anchor = parse("file.rb").unwrap();
        assert_eq!(anchor.link.path, PathBuf::from("file.rb"));
        assert_eq!(anchor.link.strategy, Strategy::Full);
    }

    #[test]
    fn parser_named_strategy() {
        let anchor = parse("../../file.rb:rofl").unwrap();
        assert_eq!(anchor.link.path, PathBuf::from("../../file.rb"));
        assert_eq!(
            anchor.link.strategy,
            Strategy::Named(NamedAnchor::new("rofl"))
        );
    }

    #[test]
    fn parse_with_leftshift() {
        let anchor = parse(" [<] ../file.rb:block1").unwrap();
        assert_eq!(anchor.link.path, PathBuf::from("../file.rb"));
        assert_eq!(
            anchor.link.strategy,
            Strategy::Named(NamedAnchor::new("block1"))
        );
        assert_eq!(anchor.decoration, Decoration::LeftShift);
    }

    #[test]
    fn parser_between_strategy() {
        let anchor = parse("file.rb:42:69").unwrap();
        assert_eq!(anchor.link.path, PathBuf::from("file.rb"));
        assert_eq!(
            anchor.link.strategy,
            Strategy::Between { start: 42, end: 69 }
        );
    }

    #[test]
    fn parser_here_down_strategy() {
        let anchor = parse("file.rb:2:").unwrap();
        assert_eq!(anchor.link.path, PathBuf::from("file.rb"));
        assert_eq!(anchor.link.strategy, Strategy::HereDown(2));
    }

    #[test]
    fn parser_down_to_strategy() {
        let anchor = parse("file.rb::10").unwrap();
        assert_eq!(anchor.link.path, PathBuf::from("file.rb"));
        assert_eq!(anchor.link.strategy, Strategy::DownTo(10));
    }

    #[test]
    fn parser_this_line_strategy() {
        let anchor = parse("file.rb:42").unwrap();
        assert_eq!(anchor.link.path, PathBuf::from("file.rb"));
        assert_eq!(anchor.link.strategy, Strategy::ThisLine(42));
    }

    #[test]
    fn parser_query_strategy() {
        let anchor = parse("file.rb#class?name=Foo").unwrap();
        assert_eq!(anchor.link.path, PathBuf::from("file.rb"));
        assert_eq!(
            anchor.link.strategy,
            Strategy::Query(QueryAnchor::parse("class?name=Foo").unwrap())
        );
    }
}
