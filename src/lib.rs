use nom::{
    branch::alt,
    combinator::map,
    multi::{separated_list1, many1},
    IResult,
    InputLength,
};

pub fn separated_lines_ignore<I: Clone + InputLength, O, O2, O3>(
    sep: impl FnMut(I) -> IResult<I, O2>,
    f: impl FnMut(I) -> IResult<I, O>,
    ignore: impl FnMut(I) -> IResult<I, O3>,
) -> impl FnMut(I) -> IResult<I, Vec<O>>
{
    map(
        separated_list1(
            many1(sep),
            alt((
                    map(f, |l| Some(l)),
                    map(ignore, |_| None)
                ))
            ),
        |ls| ls.into_iter().flatten().collect(),
    )
}

#[cfg(test)]
mod tests {
    use nom::{bytes::complete::tag, character::complete::line_ending};
    use nom::{
      IResult,
      error::ParseError,
      combinator::value,
      sequence::pair,
      bytes::complete::is_not,
      character::complete::char,
    };

    use super::*;

    #[test]
    fn should_parse_multiple_lines() {
        let mut parser = separated_lines_ignore(line_ending, tag("a"), line_ending);
        assert_eq!(parser("a\na"), Ok(("", vec!["a", "a"])));
        assert_eq!(parser("a\n\na"), Ok(("", vec!["a", "a"])));
        assert_eq!(parser("a\n\n\na"), Ok(("", vec!["a", "a"])));
        assert_eq!(parser("a\n\n\n\na"), Ok(("", vec!["a", "a"])));
    }


    fn peol_comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E>
    {
      value(
        (),
        pair(char('%'), is_not("\n\r"))
      )(i)
    }

    #[test]
    fn should_parse_multiple_comments_lines() {
        let mut parser = separated_lines_ignore(line_ending, tag("a"), peol_comment);
        assert_eq!(parser("a\na"), Ok(("", vec!["a", "a"])));
        assert_eq!(parser("a\n%hello\na"), Ok(("", vec!["a", "a"])));
        assert_eq!(parser("a\n%hello\n%world\na"), Ok(("", vec!["a", "a"])));
        assert_eq!(parser("a\n%hi\n\n%world\na"), Ok(("", vec!["a", "a"])));
    }

}
