//! 🃏️ Shared Jack query language for graph frameworks.
//!
//! 🚚 Relocated verbatim from `🧰️framework/🔨️modules/🧮️math/🕸️graph/🗣️dsl` in ticket 26/08/12/
//! DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave MATHEND — see that wave's report
//! for why the framework/plugin split hypothesis was measured and rejected (real coupling between
//! `DslIdiom` and the language-service surface). `dsl_core` (crate-root alias for
//! `semio_framework_os_kernel`, renamed from `dsl` by this same wave to free that name for this
//! module) is this file's own `os_dsl`/`dsl`-derive dependency.

#![allow(async_fn_in_trait)]

// #region ⚠️ Errors
/// 🚧️ Unified failure mode for jack parsing/execution, wire-literal parsing, and fixture ingestion.
#[derive(Debug)]
pub enum GraphDslError {
    /// 🧾️ Fixture or query-result JSON failed to parse or serialize.
    Json(semio_framework_pack_json::JsonError),
    /// 🔤️ A string literal was never closed (Jack's own dual-quote pre-scan, `dsl_core` only
    /// natively lexes `"..."`).
    UnterminatedString,
    /// ❓️ A byte outside the token grammar was found (Jack's own pre-scan for `'`/`"`/`!=`, ahead
    /// of delegating the rest of the alphabet to `os_dsl::lex`).
    UnexpectedChar(char),
    /// 🔢️ A numeric literal did not parse as a float (defensive — `os_dsl::lex` only ever
    /// accumulates well-formed digit runs, so this should be unreachable in practice).
    NumberFormat(std::num::ParseFloatError),
    /// ➡️ Parser expected one token shape and found another.
    UnexpectedToken { expected: String, found: String },
    /// 🪝️ A wire-literal edge was missing a mandatory `@port` on one of its endpoints (this
    /// module's own DAG domain rule, enforced on top of the unified wire grammar).
    EdgeTargetMissingPort,
    /// 🕸️ A jack pattern had no nodes.
    EmptyPattern,
    /// 🚫️ CREATE/DELETE/SET/MERGE are not supported on read-only queryable graphs.
    UnsupportedMutation,
    /// 📞️ A `CALL` named a procedure outside this module's small owned registry (see
    /// [`call_procedure`]) — full arbitrary-procedure dispatch is deferred to unifying
    /// semio_compose_rs's Architect query language onto Jack (Wave 2 / P9).
    UnknownProcedure(String),
    /// 🔢️ A `CALL` supplied the wrong number of positional arguments for the named procedure.
    ProcedureArity { name: String, expected: usize, found: usize },
    /// 🪪️ A supplied manifest must match the snapshot's declared identity.
    ManifestIdentity { expected: String, actual: String },
    /// 🔡️ A lexical/grammar error surfaced verbatim by the unified `dsl_core`/`dsl_schema` engine —
    /// used by both the wire-literal delegate (`dsl_core::parse_wire_text`) and Jack's
    /// `dsl_core`-backed lexer.
    Lex(semio_framework_diagnostic::TextError),
}

impl std::fmt::Display for GraphDslError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "invalid json: {error}"),
            Self::UnterminatedString => formatter.write_str("unterminated string literal"),
            Self::UnexpectedChar(character) => write!(formatter, "unexpected character '{character}'"),
            Self::NumberFormat(error) => write!(formatter, "invalid number literal: {error}"),
            Self::UnexpectedToken { expected, found } => write!(formatter, "expected {expected}, got {found}"),
            Self::EdgeTargetMissingPort => formatter.write_str("edge target requires @port"),
            Self::EmptyPattern => formatter.write_str("empty pattern"),
            Self::UnsupportedMutation => formatter.write_str("mutating jack clauses are not supported on this graph domain"),
            Self::UnknownProcedure(name) => write!(formatter, "unknown CALL procedure '{name}'"),
            Self::ProcedureArity { name, expected, found } => write!(formatter, "procedure '{name}' expects {expected} argument(s), got {found}"),
            Self::ManifestIdentity { expected, actual } => write!(formatter, "snapshot manifest '{actual}' does not match supplied manifest '{expected}'"),
            Self::Lex(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for GraphDslError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            Self::NumberFormat(error) => Some(error),
            Self::Lex(error) => Some(error),
            _ => None,
        }
    }
}

impl From<semio_framework_pack_json::JsonError> for GraphDslError {
    fn from(error: semio_framework_pack_json::JsonError) -> Self {
        Self::Json(error)
    }
}

impl From<std::num::ParseFloatError> for GraphDslError {
    fn from(error: std::num::ParseFloatError) -> Self {
        Self::NumberFormat(error)
    }
}

impl From<semio_framework_diagnostic::TextError> for GraphDslError {
    fn from(error: semio_framework_diagnostic::TextError) -> Self {
        Self::Lex(error)
    }
}
// #endregion ⚠️ Errors

#[path = "🔍️queryable/🦀️.rs"]
pub mod queryable;

#[path = "🔌️wire/🦀️.rs"]
pub mod wire;

pub use queryable::{manifest_edge_kinds, manifest_node_kinds, manifest_port_kinds, manifest_property_names, BoardQueryableGraph, QueryableEdge, QueryableGraph};
pub use wire::{dag_from_wire_literal, wire_literal_from_dag, WireEdge, WireNode};

use semio_framework_graph_manifest::PropertyValue;
use std::collections::{BTreeMap, BTreeSet};

// #region jack_impl

#[path = "🌳️syntax/🦀️.rs"]
mod jack_syntax;
pub use jack_syntax::*;

#[path = "🧭️language-service/🦀️.rs"]
mod jack_language_service;
pub use jack_language_service::*;

#[path = "🧮️parser/🦀️.rs"]
mod jack_parser;
pub use jack_parser::*;

#[path = "⚡️executor/🦀️.rs"]
mod jack_executor;
pub use jack_executor::*;

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
// #endregion jack_impl
