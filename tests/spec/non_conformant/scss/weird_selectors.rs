//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/weird-selectors.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("weird-selectors")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("> > E {\
             \n  color: red;\
             \n}\n\
             \nE > > {\
             \n  color: red;\
             \n}\n\
             \n> > E > > {\
             \n  > > F > > {\
             \n    color: red;\
             \n  }\
             \n}"),
        ""
    );
}
