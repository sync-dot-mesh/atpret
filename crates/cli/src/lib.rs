//! `atpret-cli` — atpret's entire argv/env surface, declared via
//! `argenv` rather than hand-rolled flag parsing.
//!
//! Skeleton only: the real flag/env declarations (`--strategy`,
//! `--test-run`, include/exclude IP and domain lists, config
//! import/export path) are not yet written.

use argenv::{Arg, Env, Input, Type};

/// Which backend to target. First real `argenv`-declared input in
/// this crate — a template for the rest.
pub const BACKEND: Input<&str> = Input {
    key: "backend",
    ty: Type::String,
    default: Some("auto"),
    env: Some(Env::new("ATPRET_BACKEND")),
    arg: Some(Arg { value_name: "BACKEND", ..Arg::pair("backend", 'b') }),
    summary: "Which platform backend to target (nfqws, winws, or auto-detect)",
    ..Input::EMPTY
};
