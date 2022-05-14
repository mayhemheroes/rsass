//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1654/basic.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%foo {\
             \n  &bar {\
             \n    display: block;\
             \n  }\
             \n  &.bar {\
             \n    display: block;\
             \n  }\
             \n}\
             \nzoo {\
             \n  @extend %foo;\
             \n}"),
        "zoo.bar {\
         \n  display: block;\
         \n}\n"
    );
}
