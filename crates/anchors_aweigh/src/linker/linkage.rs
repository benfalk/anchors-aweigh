use crate::doc::{Decoration, Strategy};
use crate::source::SharedFile;

#[derive(Debug)]
pub struct Linkage {
    pub source: SharedFile,
    pub strategy: Strategy,
    pub contents: Option<String>,
    pub decoration: Decoration,
}

impl Linkage {
    pub fn compile(&self, buf: &mut String) {
        let data = self.contents.as_deref().unwrap_or("");

        match self.decoration {
            Decoration::None => buf.push_str(data),
            Decoration::LeftShift => left_shift_lines(data, buf),
        }
    }
}

fn left_shift_lines(input: &str, buf: &mut String) {
    match largest_common_left_padding(input) {
        "" => buf.push_str(input),
        padding => {
            let lines = input.lines().map(|line| strip_padding(padding, line));
            for line in lines {
                buf.push_str(line);
                buf.push('\n');
            }
            buf.pop();
        }
    }
}

fn strip_padding<'a>(padding: &str, line: &'a str) -> &'a str {
    use nom::bytes::complete::tag;
    use nom::combinator::rest;
    use nom::sequence::preceded;
    use nom::{IResult, Parser};

    fn parse<'a>(padding: &str, line: &'a str) -> IResult<&'a str, &'a str> {
        preceded(tag(padding), rest).parse(line)
    }

    parse(padding, line).unwrap_or((line, line)).1
}

fn largest_common_left_padding(input: &str) -> &str {
    fn left_padding(input: &str) -> &str {
        use nom::character::complete::multispace0;
        use nom::combinator::recognize;
        use nom::{IResult, Parser};

        fn parse(input: &str) -> IResult<&str, &str> {
            recognize(multispace0).parse(input)
        }

        parse(input).unwrap().1
    }

    input
        .lines()
        .map(left_padding)
        .min_by_key(|padding| padding.len())
        .unwrap_or("")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn left_shift_works() {
        let data = "  one\n  two";
        let buf = &mut String::with_capacity(7);
        left_shift_lines(data, buf);
        assert_eq!(buf, "one\ntwo");

        let data = "one\n  two";
        let buf = &mut String::with_capacity(7);
        left_shift_lines(data, buf);
        assert_eq!(data, buf);

        let data = " one\n  two";
        let buf = &mut String::with_capacity(7);
        left_shift_lines(data, buf);
        assert_eq!(buf, "one\n two");
    }
}
