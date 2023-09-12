use  crate::problem::Problem;
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_until};
use nom::sequence::{terminated, delimited, pair};
use nom::multi::{separated_list0, many1};
use parse_hyperlinks::take_until_unbalanced;

pub fn parse( input: &str ) -> IResult<&str, &str> {

  let (mut input, mut res) = delimited(tag("("), take_until_unbalanced('(', ')'), tag(")"))(input)?;

  while res.len() != 0 {
    (input, res) = fish_parenteses(res)?;
    println!("{}", res);
  }

  // let prob = Problem { 

  // };

  IResult::Ok((input, "test"))

}

pub fn fish_parenteses( input: &str ) -> IResult<&str, &str> {
  println!("Input: {}", input);

  let (input, res) = delimited(tag("("), take_until_unbalanced('(', ')'), tag(")"))(input)?;

  println!("res: {}", res);

  IResult::Ok((input, res))
}