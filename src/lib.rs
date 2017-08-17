// Needed by pest
#![recursion_limit = "300"]


extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::result::Result;
use std::collections::HashMap;

use pest::Parser;


#[cfg(test)]
mod tests;


/// A type literal in a query
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    /// A type like Int
    Named(String),
    /// A non-nullable type like Int!
    NonNullNamed(String),
    /// A nullable type like [Int].
    /// The nullable part is the list, not Int in that example
    List(Vec<String>),
    /// A non-nullable list like [Int]!, the types inside can be null
    NonNullList(Vec<String>),
}

/// Input to fields and directives
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum InputValue {
    Variable(String),
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Enum(String),
    List(Vec<Node>),
    Object,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OperationType {
    Query,
    Mutation,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Selection {
    Field(Box<Field>),
    FragmentSpread(Box<FragmentSpread>),
    InlineFragment(Box<InlineFragment>),
}


#[derive(Clone, Debug, PartialEq)]
pub struct Directive {
    pub name: String,
    pub arguments: HashMap<String, InputValue>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FragmentSpread {
    pub name: String,
    pub directives: Vec<Directive>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InlineFragment {
    pub type_condition: String,
    pub directives: Vec<Directive>,
    pub selection_set: Selection,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub alias: Option<String>,
    pub name: String,
    pub arguments: HashMap<String, InputValue>,
    pub directives: Vec<Directive>,
    pub selection_set: Selection,
}

/// All nodes in GraphQL AST
#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Name(String),
    Document,
}

#[derive(Parser)]
#[grammar = "graphql.pest"]
pub struct GraphQLParser;

pub fn parse(input: &str) {
    let pairs = GraphQLParser::parse_str(Rule::document, input).unwrap_or_else(|e| panic!("{}", e));
}
