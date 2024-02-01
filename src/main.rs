//! Builds fancy Markdown truth tables for logical expressions.

mod error;

use cas_compute::numerical::{ctxt::Ctxt, eval::eval_stmts, value::Value};
use cas_parser::parser::{ast::{stmt::Stmt, Expr, Literal}, Parser};
use error::Error;
use std::io::{self, IsTerminal, Read, Write};

/// Generates the truth table for the given AST.
fn make_table(ast: &[Stmt]) -> Result<String, Error> {
    // scan for all input variables, remove duplicates, and sort them
    let mut vars = ast.iter()
        .map(|stmt| {
            stmt.expr.post_order_iter()
                .filter_map(|node| match node {
                    Expr::Literal(Literal::Symbol(s)) => Some(s.name.as_ref()),
                    _ => None,
                })
        })
        .flatten()
        .collect::<Vec<_>>();
    vars.sort_unstable();
    vars.dedup();

    fn create_row<T: std::fmt::Display>(vars: &[T], res: T) -> Vec<String> {
        vars.iter()
            .chain(Some(&res))
            .map(|item| item.to_string())
            .collect()
    }

    let mut ctxt = Ctxt::new();
    let mut table = Vec::new();
    table.push(create_row(&vars, "F"));

    // evaluate the expression for each possible input
    for i in 0..(1 << vars.len()) {
        let var_values = vars.iter()
            .enumerate()
            .map(|(j, _)| {
                let value = i & (1 << vars.len() - j - 1) != 0;
                value.into()
            })
            .collect::<Vec<Value>>();

        vars.iter()
            .zip(&var_values)
            .for_each(|(var, value)| ctxt.add_var(var, value.clone()));

        let res = match eval_stmts(&ast, &mut ctxt)? {
            Value::Boolean(a) => Value::Boolean(a),
            _ => todo!(),
        };
        table.push(create_row(&var_values, res));
    }

    // build the table
    let column_widths = table.iter()
        .fold(vec![0usize; vars.len() + 1], |mut acc, row| {
            acc.iter_mut()
                .zip(row.iter())
                .for_each(|(a, b)| *a = *a.max(&mut b.len()));
            acc
        });

    let mut out = String::new();

    fn make_seperator(column_widths: &[usize]) -> String {
        let middle = column_widths.iter()
            .map(|w| "-".repeat(*w + 2))
            .collect::<Vec<_>>()
            .join("+");
        format!("|{}|", middle)
    }

    fn make_row(row: &[String], column_widths: &[usize]) -> String {
        let middle = row.iter()
            .zip(column_widths)
            .map(|(cell, width)| format!(" {} ", cell) + &" ".repeat(width - cell.len()))
            .collect::<Vec<_>>()
            .join("|");
        format!("|{}|", middle)
    }

    for (i, row) in table.iter().enumerate() {
        if i == 0 {
            out.push_str(&make_seperator(&column_widths));
            out.push('\n');
        }
        out.push_str(&make_row(row, &column_widths));
        out.push_str("\n");
        if i == 0 {
            out.push_str(&make_seperator(&column_widths));
            out.push('\n');
        }
    }

    out.push_str(&make_seperator(&column_widths));
    Ok(out)
}

/// Parses and evaluates the given input string, returning the results of both operations.
fn parse_eval(input: &str) -> Result<String, Error> {
    let ast = Parser::new(input).try_parse_full_many::<Stmt>()?;
    let table = make_table(&ast)?;
    Ok(table)
}

/// Reads from the provided file or stdin and parses / evaluates the input, printing the success or
/// failure.
fn read_eval(input: &str) {
    match parse_eval(input) {
        Ok(res) => println!("{}", res),
        Err(err) => err.report_to_stderr(input),
    }
}

fn main() {
    let mut args = std::env::args();
    args.next();

    if !io::stdin().is_terminal() {
        // read source from stdin
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();

        read_eval(&input);
    } else {
        // run the repl / interactive mode
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            read_eval(&input);
        }
    }
}
