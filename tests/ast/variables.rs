use compass::utils::ast_from_code_str;

#[test]
fn should_assign_i32() {
  insta::assert_snapshot!(ast_from_code_str(
    r#"a: i32 = 13"#,
    "variables/should_assign_i32/default"
  ));

  insta::assert_snapshot!(ast_from_code_str(
    r#"a: i32 = 13i32"#,
    "variables/should_assign_i32/with_type"
  ));
}

#[test]
fn should_assign_f32() {
  insta::assert_snapshot!(ast_from_code_str(
    r#"a: f32 = 12.0"#,
    "variables/should_assign_f32/default"
  ));

  insta::assert_snapshot!(ast_from_code_str(
    r#"a: f32 = 2."#,
    "variables/should_assign_f32/suffix_missing"
  ));

  insta::assert_snapshot!(ast_from_code_str(
    r#"a: f32 = .2"#,
    "variables/should_assign_f32/prefix_missing"
  ));

  insta::assert_snapshot!(ast_from_code_str(
    r#"a: f32 = 14.0f32"#,
    "variables/should_assign_f32/default_with_type"
  ));

  insta::assert_snapshot!(ast_from_code_str(
    r#"a: f32 = .3f32"#,
    "variables/should_assign_f32/prefix_missing_with_type"
  ));

  insta::assert_snapshot!(ast_from_code_str(
    r#"a: f32 = 2.f32"#,
    "variables/should_assign_f32/suffix_missing_with_type"
  ));

  insta::assert_snapshot!(ast_from_code_str(
    r#"a: f32 = 1f32"#,
    "variables/should_assign_f32/decimal_with_type"
  ));
}

#[test]
fn should_assign_f64() {
  insta::assert_snapshot!(ast_from_code_str(
    r#"a: f64 = 14.0f64"#,
    "variables/should_assign_f64/default_with_type"
  ));

  insta::assert_snapshot!(ast_from_code_str(
    r#"a: f64 = .3f64"#,
    "variables/should_assign_f64/prefix_missing_with_type"
  ));

  insta::assert_snapshot!(ast_from_code_str(
    r#"a: f64 = 2.f64"#,
    "variables/should_assign_f64/suffix_missing_with_type"
  ));

  insta::assert_snapshot!(ast_from_code_str(
    r#"a: f64 = 1f64"#,
    "variables/should_assign_f64/decimal_with_type"
  ));
}

#[test]
fn should_assign_sum_of_i32() {
  insta::assert_snapshot!(ast_from_code_str(
    r#"a: i32 = 13 + 14"#,
    "variables/should_assign_sum_of_i32/default"
  ));

  insta::assert_snapshot!(ast_from_code_str(
    r#"a: i32 = 13i32 + 14i32"#,
    "variables/should_assign_sum_of_i32/with_type"
  ));
}

#[test]
fn should_mismatch_type_i32() {
  insta::assert_snapshot!(ast_from_code_str(
    r#"a: i32 = 13.0"#,
    "variables/should_mismatch_type_i32/default"
  ));

  insta::assert_snapshot!(ast_from_code_str(
    r#"
    b: f32 = 13.0
    a: i32 = b
    "#,
    "variables/should_mismatch_type_i32/from_variable"
  ));
}

#[test]
fn should_mismatch_type_f32() {
  insta::assert_snapshot!(ast_from_code_str(
    r#"a: f32 = 13"#,
    "variables/should_mismatch_type_f32/default"
  ));

  insta::assert_snapshot!(ast_from_code_str(
    r#"
    b: i32 = 13
    a: f32 = b
    "#,
    "variables/should_mismatch_type_f32/from_variable"
  ));
}
