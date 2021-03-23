//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/20_escapes_literal_uppercase/05_variable_quoted_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z;\
            \n.result {\
            \n  dquoted: \"#{#{$input}}\";\
            \n  dquoted: \"#{\"[#{$input}]\"}\";\
            \n  dquoted: \"#{\"#{$input}\"}\";\
            \n  dquoted: \"#{\'#{$input}\'}\";\
            \n  dquoted: \"#{\"[\'#{$input}\']\"}\";\
            \n  squoted: \'#{#{$input}}\';\
            \n  squoted: \'#{\"[#{$input}]\"}\';\
            \n  squoted: \'#{\"#{$input}\"}\';\
            \n  squoted: \'#{\'#{$input}\'}\';\
            \n  squoted: \'#{\"[\'#{$input}\']\"}\';\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  dquoted: \"\\\\b \\\\c \\\\d \\\\e \\\\f GHIJKLMNOPQRSTUVWXYZ\";\
        \n  dquoted: \"[\\\\b \\\\c \\\\d \\\\e \\\\f GHIJKLMNOPQRSTUVWXYZ]\";\
        \n  dquoted: \"\\\\b \\\\c \\\\d \\\\e \\\\f GHIJKLMNOPQRSTUVWXYZ\";\
        \n  dquoted: \"\\\\b \\\\c \\\\d \\\\e \\\\f GHIJKLMNOPQRSTUVWXYZ\";\
        \n  dquoted: \"[\'\\\\b \\\\c \\\\d \\\\e \\\\f GHIJKLMNOPQRSTUVWXYZ\']\";\
        \n  squoted: \"\\\\b \\\\c \\\\d \\\\e \\\\f GHIJKLMNOPQRSTUVWXYZ\";\
        \n  squoted: \"[\\\\b \\\\c \\\\d \\\\e \\\\f GHIJKLMNOPQRSTUVWXYZ]\";\
        \n  squoted: \"\\\\b \\\\c \\\\d \\\\e \\\\f GHIJKLMNOPQRSTUVWXYZ\";\
        \n  squoted: \"\\\\b \\\\c \\\\d \\\\e \\\\f GHIJKLMNOPQRSTUVWXYZ\";\
        \n  squoted: \"[\'\\\\b \\\\c \\\\d \\\\e \\\\f GHIJKLMNOPQRSTUVWXYZ\']\";\
        \n}\
        \n"
    );
}