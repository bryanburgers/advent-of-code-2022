use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{i64, space0},
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{preceded, terminated, tuple},
    IResult,
};
use std::{collections::HashMap, fmt::Debug};

fn main() {
    let input = include_str!("input.txt");

    let mut troop = parse(input).unwrap();
    let root = troop.get("root").unwrap();
    let result = root.expr.eval(&troop).unwrap();
    println!("{result}");

    let mut humn = troop.get_mut("humn").unwrap();
    humn.expr = Expr::Unknown;
    let root = troop.get("root").unwrap();
    let Expr::Op(_, ref expr1, ref expr2) = root.expr else {
            panic!("Not an op")
        };
    let expr1 = expr1.no_variable_expression(&troop).unwrap();
    let expr2 = expr2.no_variable_expression(&troop).unwrap();
    let expr = NoVariableExpression::Op(Op::Sub, Box::new(expr1), Box::new(expr2));
    let solved = expr.solve(0).unwrap();
    println!("{solved}");
}

struct Troop<'a>(HashMap<&'a str, Monkey<'a>>);

impl<'a> Troop<'a> {
    fn get(&self, id: &'a str) -> Option<&Monkey<'a>> {
        self.0.get(id)
    }

    fn get_mut(&mut self, id: &'a str) -> Option<&mut Monkey<'a>> {
        self.0.get_mut(id)
    }
}

#[derive(Debug, Clone)]
enum Expr<'a> {
    Const(i64),
    Var(&'a str),
    Op(Op, Box<Expr<'a>>, Box<Expr<'a>>),
    Unknown,
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
            Expr::Unknown => Err(ExprError::CantEvaluateUnknown),
        }
    }

    fn no_variable_expression(
        &self,
        troop: &Troop<'a>,
    ) -> Result<NoVariableExpression, ExprError<'a>> {
        match self {
            Expr::Const(val) => Ok(NoVariableExpression::Const(*val)),
            Expr::Var(var) => {
                let monkey = troop.get(var).ok_or(ExprError::VarNotFound(var))?;
                monkey.expr.no_variable_expression(troop)
            }
            Expr::Op(op, expr1, expr2) => {
                let val1 = expr1.no_variable_expression(troop)?;
                let val2 = expr2.no_variable_expression(troop)?;

                match (val1, val2) {
                    (NoVariableExpression::Const(v1), NoVariableExpression::Const(v2)) => {
                        Ok(NoVariableExpression::Const(op.eval(v1, v2)))
                    }
                    (val1, val2) => Ok(NoVariableExpression::Op(
                        *op,
                        Box::new(val1),
                        Box::new(val2),
                    )),
                }
            }
            Expr::Unknown => Ok(NoVariableExpression::Unknown),
        }
    }
}

#[derive(Debug)]
enum ExprError<'a> {
    VarNotFound(&'a str),
    CantEvaluateUnknown,
}

enum NoVariableExpression {
    Const(i64),
    Op(Op, Box<NoVariableExpression>, Box<NoVariableExpression>),
    Unknown,
}

impl Debug for NoVariableExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Const(arg0) => arg0.fmt(f),
            Self::Op(arg0, arg1, arg2) => {
                f.write_str("(")?;
                arg1.fmt(f)?;
                std::fmt::Display::fmt(arg0, f)?;
                arg2.fmt(f)?;
                f.write_str(")")?;
                Ok(())
            }
            Self::Unknown => f.write_str("x"),
        }
    }
}

impl NoVariableExpression {
    fn solve(&self, val: i64) -> Result<i64, SolveError> {
        match self {
            NoVariableExpression::Const(_) => Err(SolveError::SolvingAConstant),
            NoVariableExpression::Op(op, expr1, expr2) => match (expr1.as_ref(), expr2.as_ref()) {
                (NoVariableExpression::Const(c), expr2) => {
                    // 8 = 2 % x
                    match op {
                        Op::Add => expr2.solve(val - c),
                        Op::Sub => {
                            // 8 = 2 - x
                            // 8 - 2 = -x
                            // -(8 - 2) = x
                            expr2.solve(-(val - c))
                        }
                        Op::Mul => expr2.solve(val / c),
                        Op::Div => expr2.solve(todo!("8 = 2 / x")),
                    }
                }
                (expr1, NoVariableExpression::Const(c)) => {
                    // 8 = x % 2
                    match op {
                        Op::Add => expr1.solve(val - c),
                        Op::Sub => expr1.solve(val + c),
                        Op::Mul => expr1.solve(val / c),
                        Op::Div => expr1.solve(val * c),
                    }
                }
                _ => Err(SolveError::UnknownOnBothSides),
            },
            NoVariableExpression::Unknown => Ok(val),
        }
    }
}

#[derive(Debug)]
enum SolveError {
    UnknownOnBothSides,
    SolvingAConstant,
}

#[derive(Debug, Copy, Clone)]
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

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => f.write_str("+"),
            Op::Sub => f.write_str("-"),
            Op::Mul => f.write_str("*"),
            Op::Div => f.write_str("/"),
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

    #[test]
    fn test_part_2() {
        let input = include_str!("example.txt");

        let mut troop = parse(input).unwrap();
        let mut humn = troop.get_mut("humn").unwrap();
        humn.expr = Expr::Unknown;
        let root = troop.get("root").unwrap();
        let Expr::Op(_, ref expr1, ref expr2) = root.expr else {
            panic!("Not an op")
        };
        let expr1 = expr1.no_variable_expression(&troop).unwrap();
        let expr2 = expr2.no_variable_expression(&troop).unwrap();
        let expr = NoVariableExpression::Op(Op::Sub, Box::new(expr1), Box::new(expr2));
        let solved = expr.solve(0).unwrap();
        assert_eq!(solved, 301);
    }

    #[test]
    fn test_part_2_makes_sense() {
        let input = include_str!("input.txt");

        let mut troop = parse(input).unwrap();
        let mut humn = troop.get_mut("humn").unwrap();
        humn.expr = Expr::Unknown;
        let root = troop.get("root").unwrap();
        let Expr::Op(_, ref expr1, ref expr2) = root.expr else {
            panic!("Not an op")
        };
        let expr1 = expr1.no_variable_expression(&troop).unwrap();
        let expr2 = expr2.no_variable_expression(&troop).unwrap();
        let expr = NoVariableExpression::Op(Op::Sub, Box::new(expr1), Box::new(expr2));
        let solved = expr.solve(0).unwrap();

        let mut humn = troop.get_mut("humn").unwrap();
        humn.expr = Expr::Const(solved);
        let mut root = troop.get_mut("root").unwrap();
        let prev_expr = std::mem::replace(&mut root.expr, Expr::Unknown);
        if let Expr::Op(_, expr1, expr2) = prev_expr {
            root.expr = Expr::Op(Op::Sub, expr1, expr2);
        } else {
            root.expr = prev_expr;
        }

        let root = troop.get("root").unwrap();
        let result = root.expr.eval(&troop).unwrap();
        assert_eq!(result, 0);
    }
}
