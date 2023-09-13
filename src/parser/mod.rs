use  crate::problem::Problem;
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_until};
use nom::sequence::{terminated, delimited, pair, preceded};
use nom::multi::{separated_list0, many1, many0};
use parse_hyperlinks::take_until_unbalanced;

pub fn parse( input: &str ) -> IResult<&str, &str> {

  let (mut input, mut res) = delimited(tag("("), take_until_unbalanced('(', ')'), tag(")"))(input)?;

  let (mut input, vec_res) = fish_parenteses(res)?;

  println!("Result: {:?}", vec_res);


  // let prob = Problem { 

  // };

  IResult::Ok((input, "test"))

}

pub fn fish_parenteses( input: &str ) -> IResult<&str, Vec<&str>> {
  println!("to fp: {}", input);

  let (remain, res) = many0( delimited(tag("("), take_until(")"), tag(")")) )("( tester )( tester1 )")?;


  IResult::Ok((remain, res))
}