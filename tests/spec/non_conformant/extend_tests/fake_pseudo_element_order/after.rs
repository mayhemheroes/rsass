//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/fake-pseudo-element-order/after.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("after")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%a:after {x: y}\
             \nb:c {@extend %a}\n"),
        "b:c:after {\
         \n  x: y;\
         \n}\n"
    );
}
