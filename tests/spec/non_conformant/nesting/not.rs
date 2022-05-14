//! Tests auto-converted from "sass-spec/spec/non_conformant/nesting/not.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("not")
}

#[test]
#[ignore] // wrong result
fn multiple_parent_selectors_with_trailing_ident() {
    assert_eq!(
        runner().ok("// Regression test for sass/libsass#2630\
             \n.a, .b {\
             \n  :not(&-c) {d: e}\
             \n}\n"),
        ":not(.a-c, .b-c) {\
         \n  d: e;\
         \n}\n"
    );
}
