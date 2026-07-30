#![forbid(unsafe_code)]

//! Unified command-line interaction and presentation for rsomics products.
//! Products define one Clap command tree and parse it through [`parse`].

mod cli;

pub use cli::{command, parse, try_parse, try_parse_from};
