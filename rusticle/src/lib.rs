#![warn(clippy::pedantic)]
#![allow(non_camel_case_types, clippy::module_name_repetitions)]
#![cfg_attr(feature = "nightly", feature(concat_bytes, const_format_args))]

pub use rusticle_macros::*;

pub use std::{
    assert_eq as assert_equals, assert_ne as assert_not_equals, cfg as configuration,
    concat as concatenate, dbg as debug, debug_assert_eq as debug_assert_equals,
    debug_assert_ne as debug_assert_not_equals, env as environment,
    format_args as format_arguments, option_env as option_environment, println as printline,
    writeln as writeline,
};

pub type string = str;

#[cfg(feature = "nightly")]
pub use std::{concat_bytes as concatenate_bytes, const_format_args as constant_format_arguments};
