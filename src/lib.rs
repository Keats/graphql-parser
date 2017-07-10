// Needed by pest
#![recursion_limit = "300"]


#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::result::Result;
use std::collections::HashMap;

// use pest::prelude::*;


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
struct GraphQLParser;

//impl_rdp! {
//    grammar! {
//        whitespace = _{ ([" "] | ["\t"] | ["\r"] | ["\n"]) + }
//
//        comment = _{ ["#"] ~ (!(["\n"]) ~ any)* ~ ["\n"] }
//        letters = _{ ['A'..'Z'] | ['a'..'z'] }
//        exp     = _{ (["e"] | ["E"]) ~ (["+"] | ["-"])? ~ ['1'..'9']+ }
//        hex     = _{ ['0'..'9'] | ['a'..'f'] | ['A'..'F'] }
//        unicode = _{ ["u"] ~ hex ~ hex ~ hex ~ hex }
//        escape  = _{ ["\\"] ~ (["\""] | ["\\"] | ["/"] | ["b"] | ["f"] | ["n"] | ["r"] | ["t"] | unicode) }
//
//        op_true  = { ["true"] }
//        op_false = { ["false"] }
//        boolean  = _{ op_true | op_false }
//        null     = { ["null"] }
//        int      = @{ ["-"]? ~ (["0"] | ['1'..'9'] ~ ['0'..'9']*) }
//        float    = @{
//            ["-"]? ~
//                (
//                    ['1'..'9']+ ~ exp |
//                    ["0"] ~ ["."] ~ ['0'..'9']+ ~ exp? |
//                    ['1'..'9'] ~ ['0'..'9']* ~ ["."] ~ ['0'..'9']+ ~ exp?
//                )
//        }
//        string   = @{ ["\""] ~ (escape | !(["\""] | ["\\"]) ~ any)* ~ ["\""] }
//        variable = @{ ["$"] ~ name }
//        enum_val = @{ !(boolean | null) ~ name }
//        list     = @{ ["["] ~ value ~ ["]"] }
//        arg      = { name ~ [":"] ~ value }
//        object   = { ["{"] ~ (arg ~ ([","] ~ arg)*)? ~ ["}"] }
//
//        name  = @{ (["_"] | letters) ~ (["_"] | letters | ['0'..'9'])* }
//        value = @{ variable | float | int | string | boolean | null | enum_val | list | object }
//
//        // More variables stuff
//        named_type = { name }
//        list_type = {["["] ~ types ~ ["]"]}
//        non_null_type = { (named_type | list_type) ~ ["!"]}
//        types = { named_type | list_type | non_null_type }
//        default_value = { ["="] ~ value }
//        variable_def = { variable ~ [":"] ~ types ~ default_value? }
//        variable_defs = { ["("] ~ variable_def? ~ ([","] ~ variable_def)* ~ [")"] }
//
//        // Directive
//        directive = { ["@"] ~ name ~ args? }
//
//        // Selections
//        selection = { field | fragment_spread | fragment_inline }
//        selection_set = { ["{"] ~ selection+ ~ ["}"] }
//
//        // Field
//        alias = { name ~ [":"]}
//        args  = { ["("] ~ arg ~ ([","] ~ arg)* ~ [","]? ~ [")"]}
//        field = { alias? ~ name ~ args? ~ directive? ~selection_set? }
//
//        // Fragments
//        fragment_name = { !["on"] ~ name }
//        fragment_def = { ["fragment"] ~ fragment_name ~ ["on"] ~ name ~ directive? ~ selection_set }
//        fragment_spread = @{ ["..."] ~ fragment_name ~ directive? }
//        fragment_inline = { ["..."] ~ (["on"] ~ name)? ~ directive? ~ selection_set }
//
//        query = { ["query"] ~ name? ~ variable_defs? ~ selection_set }
//        mutation = { ["mutation"] ~ name? ~ variable_defs? ~ selection_set }
//        operation = { query | mutation | selection_set }
//
//        document = @{ soi ~ (operation | fragment_def)+ ~ eoi }
//    }
//}

//pub fn parse(input: &str) -> Result<(), String> {
//    let mut parser = Rdp::new(StringInput::new(input));
//
//    if !parser.document() {
//        let (_, pos) = parser.expected();
//        let (line_no, col_no) = parser.input().line_col(pos);
//        return Err(format!("Invalid GraphQL syntax at line {}, column {}", line_no, col_no));
//    }
//
////    parser.main()
//    Ok(())
//}


#[cfg(test)]
mod tests {
    use pest::prelude::*;
    use super::Rdp;

    #[test]
    fn test_lex_int() {
        let input = vec!["123", "-10", "0"];
        for i in input {
            let mut parser = Rdp::new(StringInput::new(i));
            assert!(parser.int());
            assert!(parser.end());
        }
    }

    #[test]
    fn test_lex_float() {
        let input = vec![
            "123.1", "-10.1", "0.123", "12.43",
            "123e4", "123E4", "123e-4", "123e+4",
            "-1.123e4", "-1.123E4", "-1.123e-4", "-1.123e+4", "-1.123e4567",
        ];
        for i in input {
            let mut parser = Rdp::new(StringInput::new(i));
            assert!(parser.float());
            assert!(parser.end());
        }
    }

    #[test]
    fn test_lex_null() {
        let mut parser = Rdp::new(StringInput::new("null"));
        assert!(parser.null());
        assert!(parser.end());
    }

    #[test]
    fn test_lex_boolean() {
        let input = vec!["true", "false"];
        for i in input {
            let mut parser = Rdp::new(StringInput::new(i));
            assert!(parser.boolean());
            assert!(parser.end());
        }
    }

    #[test]
    fn test_lex_escape() {
        let input = vec![r#"\n"#, r#"\""#];
        for i in input {
            let mut parser = Rdp::new(StringInput::new(i));
            assert!(parser.escape());
            assert!(parser.end());
        }
    }

    #[test]
    fn test_lex_string() {
        let input = vec![
            r#""simple""#, r#"" white space ""#, r#""quote \"""#,
            r#""escaped \\n\\r\\b\\t\\f""#, r#""slashes \\\\ \\/""#,
            r#""unicode \\u1234\\u5678\\u90AB\\uCDEF""#,
        ];
        for i in input {
            let mut parser = Rdp::new(StringInput::new(i));
            assert!(parser.string());
            assert!(parser.end());
        }
    }

    #[test]
    fn test_lex_comment() {
        let input = vec!["# a comment \n", "#\n"];
        for i in input {
            let mut parser = Rdp::new(StringInput::new(i));
            assert!(parser.comment());
            assert!(parser.end());
        }
    }

    #[test]
    fn test_lex_name() {
        let input = vec![
            "name", "Name", "NAME", "other_name", "othername",
            "name12", "__type"
        ];
        for i in input {
            let mut parser = Rdp::new(StringInput::new(i));
            assert!(parser.name());
            assert!(parser.end());
        }
    }

    #[test]
    fn test_lex_object() {
        let input = vec![
            "{}", "{ lon: 12.43 }", "{ lon: 12.43, lat: -53.211 }",
        ];
        for i in input {
            let mut parser = Rdp::new(StringInput::new(i));
            println!("{:?}", i);
            assert!(parser.object());
            assert!(parser.end());
        }
    }

    #[test]
    fn test_lex_args() {
        let input = vec![
            "(size: small)", r#"(size: "small")"#, "(size: SMALL)",
            "(size: $size)", "(first: 10, after: 20)",
        ];
        for i in input {
            let mut parser = Rdp::new(StringInput::new(i));
            println!("{:?}", i);
            assert!(parser.args());
            assert!(parser.end());
        }
    }

    #[test]
    fn test_lex_field() {
        let input = vec![
            "name", "pic: profilePic(size: small)", "newName: name",
            "pic: profilePic(size: 140)", "field1(first: 10, after: 20)",
            "field1(first: 10, after: 20,)",
            "alias: field1(first:10, after:$foo,) @include(if: $foo)"
        ];
        for i in input {
            let mut parser = Rdp::new(StringInput::new(i));
            println!("{:?}", i);
            assert!(parser.field());
            assert!(parser.end());
        }
    }

    #[test]
    fn test_lex_directive() {
        let input = vec![
            "@defer", "@stream", "@live", "@include(if: $foo)",
        ];
        for i in input {
            let mut parser = Rdp::new(StringInput::new(i));
            println!("{:?}", i);
            assert!(parser.directive());
            assert!(parser.end());
        }

    }

    #[test]
    fn test_lex_selection_set() {
        let input = vec![
r#"{
  id
  firstName
  lastName
}"#,
r#"{
  me {
    id
    firstName
    lastName
    birthday {
      month
      day
    }
    friends {
      name
    }
  }
}"#,
r#"{
  user(id: 4) {
    name
  }
}"#,
r#"{
  user(id: 4) {
    id
    name
    smallPic: profilePic(size: 64)
    bigPic: profilePic(size: 1024)
  }
}"#,
r#"{
  me {
    ...userData
  }
}"#,
r#"{
  me {
   ... on User {
      friends {
        count
      }
    }
  }
}"#,
r#"{
    hero(episode: $episode) {
        name
        friends {
          name
        }
    }
}"#,
        ];
        for i in input {
            let mut parser = Rdp::new(StringInput::new(i));
            println!("{:?}", i);
            assert!(parser.selection_set());
            assert!(parser.end());
        }
    }

    #[test]
    fn test_lex_fragment_def() {
        let input = vec![
r#"fragment friendFields on User {
  id
  name
  profilePic(size: 50)
}"#,
        ];
        for i in input {
            let mut parser = Rdp::new(StringInput::new(i));
            println!("{:?}", i);
            assert!(parser.fragment_def());
            assert!(parser.end());
        }
    }

    #[test]
    fn test_lex_inline() {
        let input = vec![
r#"... on User {
  friends {
    count
  }
}"#,
r#"... @include(if: $expandedInfo) {
  firstName
  lastName
  birthday
}"#,
        ];
        for i in input {
            let mut parser = Rdp::new(StringInput::new(i));
            println!("{:?}", i);
            assert!(parser.fragment_inline());
            assert!(parser.end());
        }
    }

    #[test]
    fn test_lex_fragment_spread() {
        let mut parser = Rdp::new(StringInput::new("...userData"));
        assert!(parser.fragment_spread());
        assert!(parser.end());
    }

    #[test]
    fn test_variable_defs() {
        let input = vec![
            "()", "($episode: Episode)", "($episode: Episode, $user: Int)",
            "($episode: Episode = 1, $user: Int)",
        ];
        for i in input {
            let mut parser = Rdp::new(StringInput::new(i));
            println!("{:?}", i);
            assert!(parser.variable_defs());
            assert!(parser.end());
        }
    }

    #[test]
    fn test_lex_query() {
        let input = vec![
r#"query HeroNameAndFriends($episode: Episode) {
  hero(episode: $episode) {
    name
    friends {
      name
    }
  }
}"#,
r#"query inlineFragmentNoType($expandedInfo: Boolean) {
  user(handle: "zuck") {
    id
    name
    ... @include(if: $expandedInfo) {
      firstName
      lastName
      birthday
    }
  }
}"#,
//r#"{
//  nearestThing(location: { lon: 12.43, lat: -53.211 })
//}"#,
        ];
        for i in input {
            let mut parser = Rdp::new(StringInput::new(i));
            println!("{:?}", i);
            assert!(parser.query());
            assert!(parser.end());
        }
    }

    #[test]
    fn test_lex_mutation() {
        let input = vec![
r#"mutation likeStory {
  like(story: 123) {
    story {
      id
    }
  }
}"#,
        ];
        for i in input {
            let mut parser = Rdp::new(StringInput::new(i));
            println!("{:?}", i);
            assert!(parser.mutation());
            assert!(parser.end());
        }
    }
}
