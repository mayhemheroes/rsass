//! Types describing how to format output.
mod cssbuf;
mod format;
mod style;
mod transform;

pub use format::{Format, Formatted};
pub use style::Style;

pub(crate) use cssbuf::{CssBuf, CssHead};
pub(crate) use transform::handle_parsed;
