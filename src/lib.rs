// Needed by pest
#![recursion_limit = "300"]


extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::result::Result;
use std::collections::HashMap;

use pest::Parser;
use pest::iterators::Pair;
use pest::inputs::Input;


#[cfg(test)]
mod tests;

// This include forces recompiling this source file if the grammar file changes.
const _GRAMMAR: &'static str = include_str!("graphql.pest");


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

/// What can be found in a SelectionSet
#[derive(Clone, Debug, PartialEq)]
pub enum Selection {
    Field(Box<Field>),
    FragmentSpread(Box<FragmentSpread>),
    FragmentInline(Box<FragmentInline>),
}

pub type SelectionSet = Vec<Selection>;

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

#[derive(Clone, Debug, PartialEq)]
pub enum OperationType {
    Query,
    Mutation,
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
pub struct FragmentInline {
    pub type_condition: String,
    pub directives: Vec<Directive>,
    pub selection_set: SelectionSet,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub alias: Option<String>,
    pub name: String,
    pub arguments: HashMap<String, InputValue>,
    pub directives: Vec<Directive>,
    pub selection_set: SelectionSet,
}

impl Default for Field {
    fn default() -> Self {
        Field {
            alias: None,
            name: String::new(),
            arguments: HashMap::new(),
            directives: vec![],
            selection_set: vec![],
        }
    }
}

/// All nodes in GraphQL AST
#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Name(String),
    Document,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FragmentDefinition {
    name: String,
    on: String,
    directive: Option<Directive>,
    selection_set: SelectionSet,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    SelectionSet(SelectionSet),
}

pub type Document = Vec<Operation>;

#[derive(Parser)]
#[grammar = "graphql.pest"]
pub struct GraphQLParser;


fn parse_field<I: Input>(pair: Pair<Rule, I>) -> Field {
    let mut field = Field::default();

    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::alias => field.alias = Some(p.into_span().as_str().to_string()),
            Rule::name => field.name = p.into_span().as_str().to_string(),
            Rule::args => unreachable!(),
            Rule::directive => unreachable!(),
            Rule::selection_set => field.selection_set = parse_selection_set(p),
            _ => unreachable!()
        };
    }

    field
}

fn parse_value<I: Input>(pair: Pair<Rule, I>) -> InputValue {
    match pair.as_rule() {
        Rule::variable => InputValue::Variable(pair.into_span().as_str().to_string()),
        Rule::string => InputValue::Variable(pair.into_span().as_str().to_string()),
        Rule::enum_val => InputValue::Enum(pair.into_span().as_str().to_string()),
        Rule::boolean => {
            match pair.into_span().as_str() {
                "false" => InputValue::Boolean(false),
                "true" => InputValue::Boolean(true),
                _ => unreachable!()
            }
        },
        Rule::float => InputValue::Float(pair.into_span().as_str().parse().unwrap()),
        Rule::int => InputValue::Int(pair.into_span().as_str().parse().unwrap()),
        Rule::null => InputValue::Null,

        _ => unreachable!("woops"),
    }
}

fn parse_selection<I: Input>(pair: Pair<Rule, I>) -> Selection {
    match pair.as_rule() {
        Rule::field => Selection::Field(Box::new(parse_field(pair))),
        Rule::fragment_spread => unreachable!(),
        Rule::fragment_inline => unreachable!(),
        _ => unreachable!("woops"),
    }
}

fn parse_selection_set<I: Input>(pair: Pair<Rule, I>) -> SelectionSet {
    let selections = pair.into_inner().map(|pos| {
        let mut pair = pos.into_inner();
        parse_selection(pair.next().unwrap())
    });

    selections.collect()
}

pub fn parse(input: &str) -> Document {
    let pairs = GraphQLParser::parse_str(Rule::document, input).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        match pair.as_rule() {
            Rule::document => {
                let content = pair.into_inner().map(|pos| {
                    let mut pair = pos.into_inner();
                    let next_pair = pair.next().unwrap();
                    match next_pair.as_rule() {
                        Rule::selection_set => Operation::SelectionSet(parse_selection_set(next_pair)),
                        Rule::operation => unreachable!("operation rule"),
                        _ => unreachable!("unknown doc rule"),
                    }
                }).collect::<Vec<_>>();
                return content;
            },
            _ => println!("Not handled yet"),
        }
    }
    unreachable!("End of parse")
}
