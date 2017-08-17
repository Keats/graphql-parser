use ::{parse};


#[test]
fn can_parse_simple_doc() {
    let res = parse(r#"
  node(1) {
    id
    name
    birthdate {
      day,
      month,
      year
    },
    friends.first(1) {
      id
      name
    }
  }
"#);
}
