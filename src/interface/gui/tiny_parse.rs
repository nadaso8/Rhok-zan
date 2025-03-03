use std::{ffi::OsStr, path::Path};

use nom::{
    branch::*,
    bytes::complete::{tag, take_while},
    character::complete::{
        alpha1, alphanumeric1, anychar, multispace0, multispace1, one_of, usize,
    },
    combinator::{rest, verify},
    error::FromExternalError,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated},
    IResult, Parser,
};
pub enum CMD {
    // Graph manipulaiton
    Allocate { name: String },
    DefineInput { name: String, pattern: usize },
    DefineOutput { name: String, val: usize },
    DefineNot { val: usize },
    DefineAnd { lhs: usize, rhs: usize },
    DefineOr { lhs: usize, rhs: usize },
    DefineNand { lhs: usize, rhs: usize },
    DefineNor { lhs: usize, rhs: usize },
    DefineXor { lhs: usize, rhs: usize },
    DefineXnor { lhs: usize, rhs: usize },

    // utility
    Save { file_name: Box<std::path::Path> },
    Load { file_name: Box<std::path::Path> },
    Compile,

    // navigation
    Graph,
    Src,
    Test,
}

pub fn parse_cmd(cmd_txt: &str) -> IResult<&str, CMD> {
    alt((
        parse_alloc,
        parse_not,
        parse_and,
        parse_or,
        parse_nand,
        parse_nor,
        parse_xor,
        parse_xnor,
        parse_compile,
        parse_save,
        parse_load,
        parse_src,
        parse_graph,
        parse_test,
    ))
    .parse(cmd_txt)
}

/// produces a valid allocate cmd paired with the remaining unmatched text on match
fn parse_alloc(i: &str) -> IResult<&str, CMD> {
    match preceded(pair(multispace0, tag("alloc")), name).parse(i) {
        Ok((remainder, name)) => Ok((
            remainder,
            CMD::Allocate {
                name: name.to_string(),
            },
        )),
        Err(err) => IResult::Err(err),
    }
}

fn parse_input(i: &str) -> IResult<&str, CMD> {
    todo!()
}

fn parse_output(i: &str) -> IResult<&str, CMD> {
    todo!()
}

fn parse_not(i: &str) -> IResult<&str, CMD> {
    match preceded(pair(multispace0, tag("not")), preceded(multispace1, usize)).parse(i) {
        Ok((remainder, val)) => Ok((remainder, CMD::DefineNot { val })),
        Err(err) => IResult::Err(err),
    }
}

fn parse_and(i: &str) -> IResult<&str, CMD> {
    match preceded(
        pair(multispace0, tag("and")),
        pair(preceded(multispace1, usize), preceded(multispace1, usize)),
    )
    .parse(i)
    {
        Ok((remainder, (lhs, rhs))) => Ok((remainder, CMD::DefineAnd { lhs, rhs })),
        Err(err) => IResult::Err(err),
    }
}

fn parse_or(i: &str) -> IResult<&str, CMD> {
    match preceded(
        pair(multispace0, tag("or")),
        pair(preceded(multispace1, usize), preceded(multispace1, usize)),
    )
    .parse(i)
    {
        Ok((remainder, (lhs, rhs))) => Ok((remainder, CMD::DefineOr { lhs, rhs })),
        Err(err) => IResult::Err(err),
    }
}

fn parse_nand(i: &str) -> IResult<&str, CMD> {
    match preceded(
        pair(multispace0, tag("nand")),
        pair(preceded(multispace1, usize), preceded(multispace1, usize)),
    )
    .parse(i)
    {
        Ok((remainder, (lhs, rhs))) => Ok((remainder, CMD::DefineNand { lhs, rhs })),
        Err(err) => IResult::Err(err),
    }
}

fn parse_nor(i: &str) -> IResult<&str, CMD> {
    match preceded(
        pair(multispace0, tag("nor")),
        pair(preceded(multispace1, usize), preceded(multispace1, usize)),
    )
    .parse(i)
    {
        Ok((remainder, (lhs, rhs))) => Ok((remainder, CMD::DefineNor { lhs, rhs })),
        Err(err) => IResult::Err(err),
    }
}

fn parse_xor(i: &str) -> IResult<&str, CMD> {
    match preceded(
        pair(multispace0, tag("xor")),
        pair(preceded(multispace1, usize), preceded(multispace1, usize)),
    )
    .parse(i)
    {
        Ok((remainder, (lhs, rhs))) => Ok((remainder, CMD::DefineXor { lhs, rhs })),
        Err(err) => IResult::Err(err),
    }
}

fn parse_xnor(i: &str) -> IResult<&str, CMD> {
    match preceded(
        pair(multispace0, tag("xnor")),
        pair(preceded(multispace1, usize), preceded(multispace1, usize)),
    )
    .parse(i)
    {
        Ok((remainder, (lhs, rhs))) => Ok((remainder, CMD::DefineXnor { lhs, rhs })),
        Err(err) => IResult::Err(err),
    }
}

fn parse_save(i: &str) -> IResult<&str, CMD> {
    match preceded(
        pair(multispace0, tag("save")),
        preceded(
            multispace1,
            verify(rest, |f: &str| {
                std::path::Path::new(f).extension() == Some(OsStr::new("rz"))
                    && std::path::Path::new(f).is_absolute()
            }),
        ),
    )
    .parse(i)
    {
        Ok((remainder, file)) => Ok((
            remainder,
            CMD::Save {
                file_name: std::path::Path::new(file).into(),
            },
        )),
        Err(err) => IResult::Err(err),
    }
}

fn parse_load(i: &str) -> IResult<&str, CMD> {
    match preceded(
        pair(multispace0, tag("load")),
        preceded(
            multispace1,
            verify(rest, |f: &str| {
                std::path::Path::new(f).extension() == Some(OsStr::new("rz"))
                    && std::path::Path::new(f).is_absolute()
            }),
        ),
    )
    .parse(i)
    {
        Ok((remainder, file)) => Ok((
            remainder,
            CMD::Load {
                file_name: std::path::Path::new(file).into(),
            },
        )),
        Err(err) => IResult::Err(err),
    }
}

fn parse_compile(i: &str) -> IResult<&str, CMD> {
    match preceded(multispace0, tag("compile")).parse(i) {
        Ok((remainder, _)) => IResult::Ok((remainder, CMD::Compile)),
        Err(err) => IResult::Err(err),
    }
}

fn parse_graph(i: &str) -> IResult<&str, CMD> {
    match preceded(multispace0, tag("graph")).parse(i) {
        Ok((remainder, _)) => IResult::Ok((remainder, CMD::Graph)),
        Err(err) => IResult::Err(err),
    }
}

fn parse_src(i: &str) -> IResult<&str, CMD> {
    match preceded(multispace0, tag("src")).parse(i) {
        Ok((remainder, _)) => IResult::Ok((remainder, CMD::Src)),
        Err(err) => IResult::Err(err),
    }
}

fn parse_test(i: &str) -> IResult<&str, CMD> {
    match preceded(multispace0, tag("test")).parse(i) {
        Ok((remainder, _)) => IResult::Ok((remainder, CMD::Test)),
        Err(err) => IResult::Err(err),
    }
}

/// a series of alpha characters delimited by whitespace
/// may be an empty string
fn name(i: &str) -> IResult<&str, &str> {
    alt((delimited(multispace1, alpha1, multispace0), multispace0)).parse(i)
}
