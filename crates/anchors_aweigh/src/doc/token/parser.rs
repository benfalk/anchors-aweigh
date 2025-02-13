use super::Token;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::combinator::{eof, rest};
use nom::error::Error;
use nom::multi::many_till;
use nom::sequence::delimited;
use nom::{Finish, IResult, Parser};

#[derive(Debug, thiserror::Error)]
#[error("{0:?}")]
pub struct ParseError(#[from] Error<String>);

pub(super) fn parse(source: &str) -> Result<Vec<Token>, ParseError> {
    let (_, parts) = all_parts(source)
        .finish()
        .map_err(|err| ParseError(err.into()))?;
    Ok(parts)
}

fn raw_anchor(input: &str) -> IResult<&str, Token> {
    delimited(tag("{{#aa "), is_not("}\n"), tag("}}"))
        .map(|data| Token::RawAnchor(String::from(data)))
        .parse(input)
}

fn content(input: &str) -> IResult<&str, Token> {
    alt((take_until("{{#aa "), rest))
        .map(|data| Token::Content(String::from(data)))
        .parse(input)
}

fn token(input: &str) -> IResult<&str, Token> {
    alt((raw_anchor, content)).parse(input)
}

fn all_parts(input: &str) -> IResult<&str, Vec<Token>> {
    many_till(token, eof)
        .map(|(parts, _)| parts) // Throw away eof empty slice
        .parse(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::support::fixtures;

    #[test]
    fn raw_anchor_parse_simple() {
        let (remainder, link) = raw_anchor("{{#aa file.rs}}").unwrap();
        dbg!(remainder);
        assert!(remainder.is_empty());
        assert_eq!(link, Token::RawAnchor("file.rs".to_owned()));
    }

    #[test]
    fn content_parse_simple() {
        let (rem, content) = content("...").unwrap();
        assert!(rem.is_empty());
        assert_eq!(content, Token::Content("...".to_owned()));
    }

    #[test]
    fn part_parse_simple() {
        let (rem, item) = token("aaa{{#aa file.rs}}bbb").unwrap();
        assert_eq!(rem, "{{#aa file.rs}}bbb");
        assert_eq!(item, Token::Content("aaa".to_owned()));

        let (rem, item) = token(rem).unwrap();
        assert_eq!(rem, "bbb");
        assert_eq!(item, Token::RawAnchor("file.rs".to_owned()));

        let (rem, item) = token(rem).unwrap();
        assert_eq!("", rem);
        assert_eq!(item, Token::Content("bbb".to_owned()));
    }

    #[test]
    fn parse_parts_simple() {
        let (rem, parts) = all_parts("aaa{{#aa file.rs}}bbb").unwrap();
        assert_eq!(rem, "");
        assert_eq!(
            parts,
            vec![
                Token::Content("aaa".to_owned()),
                Token::RawAnchor("file.rs".to_owned()),
                Token::Content("bbb".to_owned()),
            ]
        );
    }

    #[test]
    fn parse_sample_doc() {
        let doc = fixtures::sample_doc_contents();
        let (rem, parts) = all_parts(&doc).unwrap();
        assert_eq!(rem, "");
        assert_eq!(
            parts,
            vec![
                Token::Content("# Sample Doc\n\n```ruby\n".to_owned()),
                Token::RawAnchor("sample_ruby_file.rb".to_owned()),
                Token::Content("\n```\n".to_owned()),
            ]
        );
    }

    #[test]
    fn parse_parts_sample_doc() {
        let doc = fixtures::sample_doc_contents();
        let parts = parse(&doc).unwrap();
        assert_eq!(
            parts,
            vec![
                Token::Content("# Sample Doc\n\n```ruby\n".to_owned()),
                Token::RawAnchor("sample_ruby_file.rb".to_owned()),
                Token::Content("\n```\n".to_owned()),
            ]
        );
    }
}
