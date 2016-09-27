//! # Ruroonga Expr
//!
//! `ruroonga_expr` provides query syntax for Groonga expression.
//!
//! This crate is used for specify searching condition and concatinate conditions.
extern crate regex_syntax;

mod exprable;
#[macro_use]
pub mod macros;
pub mod expr;
/// Create Fulltext grn_expr.
pub mod fulltext_expr;
/// Create Phrase grn_expr.
pub mod phrase_expr;
/// Create Prefix grn_expr.
pub mod prefix_expr;
/// Create Suffix grn_expr.
pub mod suffix_expr;
/// Create Equal grn_expr.
pub mod equal_expr;
/// Create Not Equal grn_expr.
pub mod notequal_expr;
/// Create Less grn_expr.
pub mod less_expr;
/// Create Greater grn_expr.
pub mod greater_expr;
/// Create Less Equal grn_expr.
pub mod less_equal_expr;
/// Create Greater Equal grn_expr.
pub mod greater_equal_expr;
/// Create Match grn_expr.
pub mod match_expr;
pub mod groupable;
pub mod dsl;
