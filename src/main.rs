use clap::Parser;
use rsass::output::{Format, Style};
use rsass::{input, Error};
use std::io::{stdout, Write};
use std::path::PathBuf;

fn main() -> Result<(), Error> {
    Args::parse().run()
}

#[derive(Parser)]
#[clap(
    about,
    author,
    version,
    mut_arg("version", |v| v.short('v')),
    after_help = "For information about rsass and its current state of \
                  development, please refer to https://github.com/kaj/rsass/ .\
                  \n\n\
                  The sass / scss languate itself is documented at \
                  https://sass-lang.com/ ."
)]
struct Args {
    /// How many digits of precision to use when outputting decimal numbers.
    #[clap(long, default_value = "5")]
    precision: usize,

    /// How to format output.
    #[clap(long, short = 't', ignore_case = true,
                default_value = "expanded",
                possible_values = Style::variants())]
    style: Style,

    /// Some kind of forced ascii output
    /// (Not implemented, but set by the sass-spec test runner)
    #[cfg(feature = "unimplemented_args")]
    #[clap(long)]
    #[allow(unused)]
    no_unicode: bool,

    /// No color in error messages
    /// (not that there is any support for color in error messages
    /// anyway yet, but the test runner uses this flag)
    #[cfg(feature = "unimplemented_args")]
    #[clap(long)]
    #[allow(unused)]
    no_color: bool,

    /// Verbose diagnostics
    /// (Always on, but set by the sass-spec test runner)
    #[cfg(feature = "unimplemented_args")]
    #[clap(long)]
    #[allow(unused)]
    verbose: bool,

    /// Where to search for included resources.
    #[clap(long, short = 'I')]
    load_path: Option<PathBuf>,

    /// Sass file(s) to translate
    #[clap(required = true)]
    input: Vec<PathBuf>,
}

impl Args {
    fn run(self) -> Result<(), Error> {
        let format = Format {
            style: self.style,
            precision: self.precision,
        };
        for name in &self.input {
            let (mut context, source) = input::FsContext::for_path(name)?;
            if let Some(include_path) = &self.load_path {
                context.push_path(include_path.as_ref());
            }
            let result = context.with_format(format).transform(source)?;
            stdout().write_all(&result)?;
        }
        Ok(())
    }
}
