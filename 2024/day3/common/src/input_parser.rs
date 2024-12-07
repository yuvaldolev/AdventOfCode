use nom::{branch, bytes, character, combinator, multi, sequence, IResult};

use crate::{error, Instruction};

pub struct InputParser;

impl InputParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, input: &str) -> error::Result<Vec<Instruction>> {
        multi::many1(Self::instruction_parser)(input)
            .map(|(_, instructions)| instructions)
            .map_err(|e| error::Error::ParseInput(e.map_input(str::to_owned), input.to_owned()))
    }

    fn instruction_parser(input: &str) -> IResult<&str, Instruction> {
        combinator::map(
            multi::many_till(
                character::complete::anychar,
                branch::alt((
                    Self::multiply_parser,
                    Self::enable_parser,
                    Self::disable_parser,
                )),
            ),
            |(_, result)| result,
        )(input)
    }

    fn multiply_parser(input: &str) -> IResult<&str, Instruction> {
        combinator::map(
            sequence::tuple((
                bytes::complete::tag("mul"),
                sequence::delimited(
                    character::complete::char('('),
                    Self::multiply_operands_parser,
                    character::complete::char(')'),
                ),
            )),
            |(_, operands)| Instruction::Multiply(operands.0, operands.1),
        )(input)
    }

    fn enable_parser(input: &str) -> IResult<&str, Instruction> {
        combinator::map(
            sequence::tuple((
                bytes::complete::tag("do"),
                character::complete::char('('),
                character::complete::char(')'),
            )),
            |_| Instruction::Enable,
        )(input)
    }

    fn disable_parser(input: &str) -> IResult<&str, Instruction> {
        combinator::map(
            sequence::tuple((
                bytes::complete::tag("don't"),
                character::complete::char('('),
                character::complete::char(')'),
            )),
            |_| Instruction::Disable,
        )(input)
    }

    fn multiply_operands_parser(input: &str) -> IResult<&str, (u32, u32)> {
        sequence::separated_pair(
            Self::multiply_operand_parser,
            character::complete::char(','),
            Self::multiply_operand_parser,
        )(input)
    }

    fn multiply_operand_parser(input: &str) -> IResult<&str, u32> {
        combinator::map_res(
            bytes::complete::take_while_m_n(1, 3, |c: char| c.is_digit(10)),
            str::parse,
        )(input)
    }
}
