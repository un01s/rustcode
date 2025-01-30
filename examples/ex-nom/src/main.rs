use nom::{
  bytes::complete::{tag, take_while_m_n},
  combinator::map_res,
  sequence::separated_pair,
  IResult,
  Parser,
};

use nom::character::complete::alpha0;
use nom::character::complete::char;

#[derive(Debug, PartialEq)]
pub struct Color {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
  u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
  c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
  map_res(
    take_while_m_n(2, 2, is_hex_digit),
    from_hex
  ).parse(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
  let (input, _) = tag("#")(input)?;
  let (input, (red, green, blue)) = (hex_primary, hex_primary, hex_primary).parse(input)?;
  Ok((input, Color { red, green, blue }))
}

pub fn do_nothing_parser(input: &str) -> IResult<&str, &str> {
  Ok((input, ""))
}

pub fn parser_alphabets(input: &str) -> IResult<&str, &str> {
  alpha0(input)
}

pub fn foo(s: &str) -> IResult<&str, &str> {
  tag("foo")(s)
}

pub fn bar(s: &str) -> IResult<&str, &str> {
  tag("bar")(s)
}

pub fn foo_bar(s: &str) -> IResult<&str, (&str, &str)> {
  separated_pair(foo, char(' '), bar).parse(s)
}

fn main() {
  println!("{:?}", hex_color("#2F14DF"));
  println!("{:?}", do_nothing_parser(""));

  println!("{:?}", parser_alphabets("abc123"));

  println!("{:?}", foo("foo bar")); // try to match "foo", if found, return Ok((remainder, parsed_value))
}

#[test]
fn parse_color() {
  assert_eq!(
    hex_color("#2F14DF"),
    Ok((
      "",
      Color {
        red: 47,
        green: 20,
        blue: 223,
      }
    ))
  );
}

#[test]
fn parse_foo() {
  assert_eq!(
    foo("foo bar"),
    Ok((
      " bar",
      "foo"
    ))
  );
}

#[test]
fn parse_foo_err() {
  assert!(foo("1234567").is_err());
}

#[test]
fn parse_foo_bar() {
  assert_eq!(foo_bar("foo bar"), Ok(("", ("foo", "bar"))));
  assert!(foo_bar("1234567").is_err());
}

