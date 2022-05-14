//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/021_test_universal_unification_with_simple_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("021_test_universal_unification_with_simple_target")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a .foo {a: b}\
             \n*|* {@extend .foo} -a {@extend %-a}\n"),
        "-a .foo, -a *|* {\
         \n  a: b;\
         \n}\n"
    );
}
