//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/023_test_universal_unification_with_simple_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("023_test_universal_unification_with_simple_target")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a .foo.bar {a: b}\
             \n*|* {@extend .foo} -a {@extend %-a}\n"),
        "-a .bar {\
         \n  a: b;\
         \n}\n"
    );
}
