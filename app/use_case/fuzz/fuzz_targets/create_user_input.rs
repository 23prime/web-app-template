#![no_main]

use garde::Validate;
use libfuzzer_sys::fuzz_target;
use use_case::create_user::CreateUserInput;

fuzz_target!(|input: (String, String, String)| {
    let (name, email, password) = input;
    let input = CreateUserInput {
        name,
        email,
        password,
    };
    let _ = input.validate();
});
