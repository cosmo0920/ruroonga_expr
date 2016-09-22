//! # Ruroonga Expr
//!
//! `ruroonga_expr` provides query syntax for Groonga expression.
//!
//! This crate is used for specify searching condition and concatinate conditions.
extern crate regex_syntax;

mod exprable;
pub mod expr;
pub mod fulltext_expr;
pub mod phrase_expr;
pub mod prefix_expr;
pub mod suffix_expr;
pub mod equal_expr;
pub mod notequal_expr;
pub mod less_expr;
pub mod greater_expr;
pub mod less_equal_expr;
pub mod greater_equal_expr;
pub mod match_expr;
pub mod groupable;
pub mod dsl;
