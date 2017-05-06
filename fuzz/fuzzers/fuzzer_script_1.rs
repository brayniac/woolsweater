#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate woolsweater;

use std::str;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if let Ok(s) = str::from_utf8(data)  {
	let _ = woolsweater::sometimes_crash(&s);
    }
});
