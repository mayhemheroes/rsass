//! Tests auto-converted from "sass-spec/spec/css/media/logic/and_not.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("and_not")
}

#[test]
fn after_interpolation() {
    assert_eq!(
        runner().ok("@media #{a} and not (b) {x {y: z}}\n"),
        "@media a and not (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
    );
}
#[test]
fn after_type() {
    assert_eq!(
        runner().ok("@media a and not (b) {x {y: z}}\n"),
        "@media a and not (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
    );
}
#[test]
fn after_type_and_modifier() {
    assert_eq!(
        runner().ok("@media only a and not (b) {x {y: z}}\n"),
        "@media only a and not (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
    );
}
mod comment_after {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn after_type() {
        assert_eq!(
            runner().ok("@media a and not/**/(b) {x {y: z}}\n"),
            "@media a and not (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn after_type_and_modifier() {
        assert_eq!(
            runner().ok("@media only a and not/**/(b) {x {y: z}}\n"),
            "@media only a and not (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
        );
    }
}
#[test]
fn interpolation() {
    assert_eq!(
        runner().ok("@media a and not #{\"(b)\"} {x {y: z}}\n"),
        "@media a and not (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn mixed_case() {
    assert_eq!(
        runner().ok("@media a AnD nOt (b) {x {y: z}}\n"),
        "@media a and not (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
    );
}
