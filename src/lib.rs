// Copyright (c) 2018 FaultyRAM
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Safe, high-level bindings to OpenAL.

#![no_std]
#![forbid(
    warnings,
    future_incompatible,
    rust_2018_idioms,
    unused,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_qualifications,
    unused_results
)]
#![deny(unused_qualifications)]
#![cfg_attr(
    feature = "cargo-clippy",
    forbid(
        clippy,
        clippy_pedantic,
        clippy_complexity,
        clippy_correctness,
        clippy_perf,
        clippy_style,
    )
)]

extern crate bindgen_openal_sys;

mod device;

pub use device::Device;
