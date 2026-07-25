#![no_main]

use garde::Validate;
use libfuzzer_sys::fuzz_target;
use use_case::login::LoginInput;

fuzz_target!(|input: (String, String)| {
    let (email, password) = input;
    let input = LoginInput { email, password };
    let _ = input.validate();
});
