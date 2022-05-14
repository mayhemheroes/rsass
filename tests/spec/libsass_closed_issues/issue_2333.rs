//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2333.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2333")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("test { test: inspect((a:1,b:(foo,bar),c:3)); }"),
        "test {\
         \n  test: (a: 1, b: (foo, bar), c: 3);\
         \n}\n"
    );
}
