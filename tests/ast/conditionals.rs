use celestial_hub_compass::utils::ast_from_code_str;

#[test]
fn should_declare_if_statement() {
  insta::assert_snapshot!(ast_from_code_str(
    r#"
    a: i32 = 1
    b: i32 = 2
    c: i32 = 3

    "#,
    "conditionals/should_declare_if_statement/default"
  ));
}
