use pest::Parser;

use ::{GraphQLParser, Rule};

#[test]
fn lex_int() {
    let input = vec!["123", "-10", "0"];
    for i in input {
        GraphQLParser::parse_str(Rule::int, i).unwrap();
    }
}

#[test]
fn lex_float() {
    let input = vec![
        "123.1", "-10.1", "0.123", "12.43",
        "123e4", "123E4", "123e-4", "123e+4",
        "-1.123e4", "-1.123E4", "-1.123e-4", "-1.123e+4", "-1.123e4567",
    ];
    for i in input {
        GraphQLParser::parse_str(Rule::float, i).unwrap();
    }
}

#[test]
fn lex_null() {
    GraphQLParser::parse_str(Rule::null, "null").unwrap();
}

#[test]
fn lex_boolean() {
    let input = vec!["true", "false"];
    for i in input {
        GraphQLParser::parse_str(Rule::boolean, i).unwrap();
    }
}

#[test]
fn lex_escape() {
    let input = vec![r#"\n"#, r#"\""#];
    for i in input {
        GraphQLParser::parse_str(Rule::escape, i).unwrap();
    }
}

#[test]
fn lex_string() {
    let input = vec![
        r#""simple""#, r#"" white space ""#, r#""quote \"""#,
        r#""escaped \\n\\r\\b\\t\\f""#, r#""slashes \\\\ \\/""#,
        r#""unicode \\u1234\\u5678\\u90AB\\uCDEF""#,
    ];
    for i in input {
        GraphQLParser::parse_str(Rule::string, i).unwrap();
    }
}

#[test]
fn lex_comment() {
    let input = vec!["# a comment \n", "#\n"];
    for i in input {
        GraphQLParser::parse_str(Rule::comment, i).unwrap();
    }
}

#[test]
fn lex_name() {
    let input = vec![
        "name", "Name", "NAME", "other_name", "othername",
        "name12", "__type"
    ];
    for i in input {
        GraphQLParser::parse_str(Rule::name, i).unwrap();
    }
}

#[test]
fn lex_object() {
    let input = vec![
        "{}", "{ lon: 12.43 }", "{ lon: 12.43, lat: -53.211 }",
    ];
    for i in input {
        GraphQLParser::parse_str(Rule::object, i).unwrap();
    }
}

#[test]
fn lex_args() {
    let input = vec![
        "(size: small)", r#"(size: "small")"#, "(size: SMALL)",
        "(size: $size)", "(first: 10, after: 20)",
    ];
    for i in input {
        GraphQLParser::parse_str(Rule::args, i).unwrap();
    }
}

#[test]
fn lex_field() {
    let input = vec![
        "name", "pic: profilePic(size: small)", "newName: name",
        "pic: profilePic(size: 140)", "field1(first: 10, after: 20)",
        "field1(first: 10, after: 20,)",
        "alias: field1(first:10, after:$foo,) @include(if: $foo)"
    ];
    for i in input {
        GraphQLParser::parse_str(Rule::field, i).unwrap();
    }
}

#[test]
fn lex_directive() {
    let input = vec![
        "@defer", "@stream", "@live", "@include(if: $foo)",
    ];
    for i in input {
        GraphQLParser::parse_str(Rule::directive, i).unwrap();
    }
}

#[test]
fn lex_selection_set() {
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
        GraphQLParser::parse_str(Rule::selection_set, i).unwrap();
    }
}

#[test]
fn lex_fragment_def() {
    let input = vec![
        r#"fragment friendFields on User {
  id
  name
  profilePic(size: 50)
}"#,
    ];
    for i in input {
        GraphQLParser::parse_str(Rule::fragment_def, i).unwrap();
    }
}

#[test]
fn lex_fragment_inline() {
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
        GraphQLParser::parse_str(Rule::fragment_inline, i).unwrap();
    }
}

#[test]
fn lex_fragment_spread() {
    GraphQLParser::parse_str(Rule::fragment_spread, "...userData").unwrap();
}

#[test]
fn variable_defs() {
    let input = vec![
        "()", "($episode: Episode)", "($episode: Episode, $user: Int)",
        "($episode: Episode = 1, $user: Int)",
    ];
    for i in input {
        GraphQLParser::parse_str(Rule::variable_defs, i).unwrap();
    }
}

#[test]
fn lex_query() {
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
        GraphQLParser::parse_str(Rule::query, i).unwrap();
    }
}

#[test]
fn lex_mutation() {
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
        GraphQLParser::parse_str(Rule::mutation, i).unwrap();
    }
}
