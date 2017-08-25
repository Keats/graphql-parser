use ::{parse};


#[test]
fn can_parse_simple_doc() {
    let res = parse(r#"{
  id
  firstName
  lastName
}"#);
    println!("{:#?}", res);
    assert!(false);
}
