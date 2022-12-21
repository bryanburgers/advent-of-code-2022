use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{i64, space0},
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{preceded, terminated, tuple},
    IResult,
};
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let troop = parse(input).unwrap();
    let root = troop.get("root").unwrap();
    let result = root.expr.eval(&troop).unwrap();
    println!("{result}");
}

struct Troop<'a>(HashMap<&'a str, Monkey<'a>>);

impl<'a> Troop<'a> {
    fn get(&self, id: &'a str) -> Option<&Monkey<'a>> {
        self.0.get(id)
    }
}

#[derive(Debug)]
enum Expr<'a> {
    Const(i64),
    Var(&'a str),
    Op(Op, Box<Expr<'a>>, Box<Expr<'a>>),
}

impl<'a> Expr<'a> {
    fn eval(&self, troop: &Troop<'a>) -> Result<i64, ExprError<'a>> {
        match self {
            Expr::Const(val) => Ok(*val),
            Expr::Var(var) => {
                let monkey = troop.get(var).ok_or(ExprError::VarNotFound(var))?;
                monkey.expr.eval(troop)
            }
            Expr::Op(op, expr1, expr2) => {
                let val1 = expr1.eval(troop)?;
                let val2 = expr2.eval(troop)?;
                Ok(op.eval(val1, val2))
            }
        }
    }
}

#[derive(Debug)]
enum ExprError<'a> {
    VarNotFound(&'a str),
}

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn eval(&self, val1: i64, val2: i64) -> i64 {
        match self {
            Op::Add => val1 + val2,
            Op::Sub => val1 - val2,
            Op::Mul => val1 * val2,
            Op::Div => val1 / val2,
        }
    }
}

#[derive(Debug)]
struct Monkey<'a> {
    id: &'a str,
    expr: Expr<'a>,
}

fn parse(input: &str) -> Result<Troop<'_>, Box<dyn std::error::Error + Send + Sync + '_>> {
    let (rest, troop) = parse_troop(input)?;
    if !rest.is_empty() {
        return Err("Not empty".into());
    }

    Ok(troop)
}

fn parse_troop(input: &str) -> IResult<&str, Troop<'_>> {
    let (rest, monkeys) =
        terminated(separated_list0(tag("\n"), parse_monkey), opt(tag("\n")))(input)?;
    let hashmap = monkeys
        .into_iter()
        .map(|monkey| (monkey.id, monkey))
        .collect();
    Ok((rest, Troop(hashmap)))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey<'_>> {
    let (rest, id) = parse_id(input)?;
    let (rest, _) = tag(":")(rest)?;
    let (rest, expr) = preceded(space0, parse_expr)(rest)?;
    let monkey = Monkey { id, expr };

    Ok((rest, monkey))
}

fn parse_expr(input: &str) -> IResult<&str, Expr<'_>> {
    alt((parse_const_expr, parse_operation))(input)
}

fn parse_const_expr(input: &str) -> IResult<&str, Expr<'_>> {
    map(parse_const, Expr::Const)(input)
}

fn parse_const(input: &str) -> IResult<&str, i64> {
    i64(input)
}

fn parse_operation(input: &str) -> IResult<&str, Expr<'_>> {
    let (rest, (id1, op, id2)) = tuple((
        parse_var_expr,
        preceded(space0, parse_op),
        preceded(space0, parse_var_expr),
    ))(input)?;
    let expr = Expr::Op(op, Box::new(id1), Box::new(id2));
    Ok((rest, expr))
}

fn parse_var_expr(input: &str) -> IResult<&str, Expr<'_>> {
    map(parse_id, Expr::Var)(input)
}

fn parse_id(input: &str) -> IResult<&str, &str> {
    take_while1(char::is_alphabetic)(input)
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    alt((
        map(tag("+"), |_| Op::Add),
        map(tag("-"), |_| Op::Sub),
        map(tag("*"), |_| Op::Mul),
        map(tag("/"), |_| Op::Div),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("example.txt");

        let troop = parse(input).unwrap();
        let root = troop.get("root").unwrap();
        let result = root.expr.eval(&troop).unwrap();
        assert_eq!(result, 152);
    }
}
