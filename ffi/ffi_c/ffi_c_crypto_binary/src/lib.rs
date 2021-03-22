// Copyright 2020 WeDPR Lab Project Authors. Licensed under Apache-2.0.

//! Library of FFI of wedpr_crypto wrapper functions, targeting C/C++
//! compatible architectures (including iOS).

#[macro_use]
extern crate wedpr_ffi_macros;

// #[macro_use]
// extern crate wedpr_l_macros;

#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

mod config;
pub mod ecies;
pub mod hash;
pub mod signature;
pub mod vrf;

// C/C++ FFI: C-style interfaces will be generated.
