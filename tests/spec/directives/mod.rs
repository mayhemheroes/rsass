//! Tests auto-converted from "sass-spec/spec/directives"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("directives")
}

mod at_root;

// Ignoring "extend", not expected to work yet.

mod test_for;

mod forward;

mod function;

mod test_if;

mod import;

mod test_use;

mod warn;
