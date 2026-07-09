#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let format = rsass::output::Format::default();
    let _ = rsass::compile_scss(data, format);
});
