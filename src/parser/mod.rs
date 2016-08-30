pub mod ast;

use std::iter::Peekable;
use std::marker::Sized;
use std::fmt;

use super::lexer::Token::{self, IdentT, LeftParenthesis, RightParenthesis, Comma, Semicolon, NumberT, StringT};
use self::ast::Node::{self, Delete, From, Where, Id, NumberC, StringC, Insert, TableN, Values, Column, TableColumn, Create, Select};
use self::ast::Type;
use self::ast::Condition::{Eq};

pub trait Parser {
    fn parse(self) -> Result<Node, String>;
}

impl<T: IntoIterator<Item = Token>> Parser for T {
    fn parse(self) -> Result<Node, String> {
        let mut iter = self.into_iter().peekable();
        match iter.next() {
            Some(IdentT(statement)) => {
                match statement.as_str() {
                    "create" => Ok(Create(Box::new(try!(parse_create(&mut iter.by_ref()))))),
                    "delete" => Ok(Delete(Box::new(try!(parse_from(&mut iter.by_ref()))), Box::new(try!(parse_where(&mut iter.by_ref()))))),
                    "insert" => Ok(Insert(Box::new(try!(parse_table(&mut iter.by_ref()))), Box::new(Values(parse_values(&mut iter.by_ref()))))),
                    "select" => Ok(try!(parse_select(&mut iter.by_ref()))),
                    _ => Err("undefined query type".to_owned()),
                }
            }
            _ => Err("".to_owned()),
        }
    }
}

fn parse_create<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Result<Node, String> {
    tokens.next(); //skip 'TABLE' keyword
    let table_name = match tokens.next() {
        Some(IdentT(name)) => name,
        Some(token) => return Err(format_unexpected_token("table name", Some(&token))),
        _ => return Err("".to_owned()),
    };
    Ok(TableN(table_name, try!(parse_table_columns(&mut tokens.by_ref()))))
}

fn parse_table_columns<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Result<Vec<Node>, String> {
    match tokens.next() {
        Some(LeftParenthesis) => {} //skip '('
        token => return Err(format_unexpected_token(LeftParenthesis, token.as_ref())),
    }

    let mut columns = vec![];

    loop {
        let col_name = match tokens.next() {
            Some(IdentT(name)) => name,
            _ => return Err("".to_owned()),
        };
        let col_type = match tokens.next() {
            Some(IdentT(_)) => Type::Int,
            _ => return Err("".to_owned()),
        };

        columns.push(TableColumn(col_name, col_type, None));

        match tokens.next() {
            Some(Comma) => {}, //skip ','
            Some(RightParenthesis) => break,
            Some(IdentT(id)) => return Err(format_unexpected_token(Comma, Some(&IdentT(id)))),
            Some(token) => return Err(format_unexpected_token(RightParenthesis, Some(&token))),
            None => return Err("parsing error missing ','".to_owned()),
        }
    }

    //    tokens.next(); //skip ')'
    match tokens.peek() {
        Some(&Semicolon) => { tokens.next(); } // skip ';'
        _ => return Err(format_unexpected_token(Semicolon, None)),
    }

    Ok(columns)
}

fn format_unexpected_token<D: fmt::Display + Sized>(expected: D, found: Option<&Token>) -> String {
    match found {
        Some(token) => format!("error: expected <{}> found <{}>", expected, token),
        None => format!("error: expected <{}>", expected)
    }
}

fn parse_from<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Result<Node, String> {
    tokens.next(); //skip 'FROM' keyword
    match tokens.next() {
        Some(IdentT(table_name)) => Ok(From(table_name)),
        _ => Err("".to_owned()),
    }
}

fn parse_where<I: Iterator<Item = Token>>(tokens: &mut I) -> Result<Node, String> {
    tokens.next(); //skip 'WHERE' keyword
    match tokens.next() {
        Some(_) => Ok(Where(Some(Eq(Box::new(Id("col".to_owned())), Box::new(NumberC("5".to_owned())))))),
        _ => Ok(Where(None)),
    }
}

fn parse_table<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Result<Node, String> {
    tokens.next(); //skip 'INTO' keyword
    tokens.next(); //skip table name
    Ok(TableN("table_name".to_owned(), parse_columns(&mut tokens.by_ref())))
}

fn parse_columns<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Vec<Node> {
    match tokens.peek() {
        Some(&LeftParenthesis) => { tokens.next(); }, //skip '('
        _ => return vec![],
    }
    let mut columns = vec![];
    loop {
        match tokens.next() {
            Some(Comma) => {},
            Some(IdentT(col)) => { columns.push(Column(col)); },
            _ => break,
        }
    }
    columns
}

fn parse_values<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Vec<Node> {
    tokens.next(); //skip 'VALUES' keyword
    tokens.next(); //skip '('
    let mut values = vec![];
    loop {
        match tokens.next() {
            Some(NumberT(val)) => values.push(NumberC(val)),
            Some(StringT(val)) => values.push(StringC(val)),
            Some(Comma) => {},
            _ => break,
        }
    }

    tokens.next(); // skip ';'
    values
}

fn parse_select<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Result<Node, String> {
    match tokens.peek() {
        Some(&IdentT(ref v)) => if v == "from" {
            return Err("parsing error".to_owned());
        },
        _ => {},
    }

    let mut columns = vec![];

    loop {
        match tokens.next() {
            Some(IdentT(v)) => if v == "from" {
                break; // skip 'FROM' keyword
            } else {
                columns.push(Column(v))
            },
            Some(Comma) => {},
            _ => return Err("parsing error".to_owned()),
        }
    }

    let table = match tokens.next() {
        Some(IdentT(table_name)) => TableN(table_name, vec![]),
        _ => return Err("parsing error".to_owned()),
    };

    Ok(Select(Box::new(table), columns))
}
