//! `thread_local_compat` is a stub crate that re-exports the `thread_local` macro from std.
//! Customize it in your workspace to have it use different backing implimentations

pub use std::thread_local;
