//! 🗣️ Imperative artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! The parent text carries only the two composed child handles (design §20.15): the program lives in the `flow` child and
//! the seed in the `text` child, so no step or seed literal is ever printed here.
//!
//! `Value`/`Atom`/`Dictionary` are defined in `neural_engine` (a foreign kernel crate), so none of them can carry a
//! `#[derive(dsl::Dsl...)]` themselves — Rust's orphan rule requires the impl target type to live in the
//! crate that also owns the trait or the type. `ValueDsl` below is the local structural twin command payloads
//! (`🎮️commands/🎚️set-step-params`) convert to/from.
//!
//! `ValueDsl` deliberately does NOT route through `dsl_schema`'s built-in `Shape::Value`/`DslValue`
//! dynamic-literal primitive: even now that `DslValue::Number` wraps a typed `UInt`/`Int`/`Float`
//! enum rather than a bare `f64`, its printer still can't reproduce the OLD hand-rolled printer's
//! own distinction (always giving a `Decimal` a trailing `.`) — an existing test parses a bare `1`
//! and expects `Atom::Integer(1)` back. So `ValueDsl` is its own typed record instead: exactly one of its mutually-exclusive `Option`
//! fields is ever `Some`, each keyed so the ACTUAL Rust variant (not a text heuristic) decides which one,
//! which is exactly as precise as the old hand-rolled `Atom` match. `ValueDsl` also derives
//! `dsl::ToValue`/`dsl::FromValue` (on top of `dsl::DslRecord`) so it can nest inside
//! `🎮️commands/🎚️set-step-params`'s `BTreeMap<String, ValueDsl>` payload field — that command struct's
//! own `ToValue`/`FromValue` derive needs it, via the framework's blanket `BTreeMap<String, T: ToValue>`
//! impl (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/02: `app_commands!` itself
//! forces only `ToValue`/`FromValue`/`dsl::DslOps` onto the generated command enum, never serde).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::{Dictionary, ProcedureSnapshot};
use neural_engine::{Atom, Value};
use std::collections::BTreeMap;

//#region 🔖️Value
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
pub struct ValueDsl {
    /// 🕳️ Presence-only flag (the payload is never inspected) — `Atom::Null`'s tag.
    null: Option<bool>,
    #[dsl(key = "bool")]
    boolean: Option<bool>,
    #[dsl(key = "int")]
    integer: Option<i64>,
    decimal: Option<f64>,
    text: Option<String>,
    #[dsl(key = "dict")]
    dictionary: Option<BTreeMap<String, ValueDsl>>,
}

pub fn value_to_value_dsl(value: &Value) -> ValueDsl {
    let mut dsl_value = ValueDsl { null: None, boolean: None, integer: None, decimal: None, text: None, dictionary: None };
    match value {
        Value::Atom(Atom::Null) => dsl_value.null = Some(true),
        Value::Atom(Atom::Boolean(b)) => dsl_value.boolean = Some(*b),
        Value::Atom(Atom::Integer(i)) => dsl_value.integer = Some(*i),
        Value::Atom(Atom::Decimal(d)) => dsl_value.decimal = Some(*d),
        Value::Atom(Atom::String(s)) => dsl_value.text = Some(s.clone()),
        Value::Dictionary(dict) => dsl_value.dictionary = Some(dictionary_to_value_dsl_map(dict)),
    }
    dsl_value
}

pub fn value_dsl_to_value(dsl_value: &ValueDsl) -> Value {
    if dsl_value.null.is_some() {
        return Value::Atom(Atom::Null);
    }
    if let Some(b) = dsl_value.boolean {
        return Value::Atom(Atom::Boolean(b));
    }
    if let Some(i) = dsl_value.integer {
        return Value::Atom(Atom::Integer(i));
    }
    if let Some(d) = dsl_value.decimal {
        return Value::Atom(Atom::Decimal(d));
    }
    if let Some(s) = &dsl_value.text {
        return Value::Atom(Atom::String(s.clone()));
    }
    match &dsl_value.dictionary {
        Some(entries) => Value::Dictionary(value_dsl_map_to_dictionary(entries)),
        None => Value::Atom(Atom::Null),
    }
}

pub fn dictionary_to_value_dsl_map(dict: &Dictionary) -> BTreeMap<String, ValueDsl> {
    dict.keys().map(|key| (key.clone(), value_to_value_dsl(dict.get(key).expect("key came from dict.keys()")))).collect()
}

pub fn value_dsl_map_to_dictionary(entries: &BTreeMap<String, ValueDsl>) -> Dictionary {
    entries.iter().fold(Dictionary::new(), |dict, (key, value)| dict.insert(key.clone(), value_dsl_to_value(value)))
}

//#endregion 🔖️Value


//#region 🔖️Api
/// 📄️ The default `imperative` document, handcrafted in the `.imperative` DSL.
pub const PROCEDURE_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.imperative` DSL text into an `ProcedureSnapshot`.
pub fn parse_dsl(text: &str) -> Result<ProcedureSnapshot, semio_framework_diagnostic::TextError> {
    <ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints an `ProcedureSnapshot` back to `.imperative` DSL text.
pub fn print_dsl(document: &ProcedureSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}
//#endregion 🔖️Api

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type ProcedureSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{ProcedureFlowChild, ProcedureTextChild};
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::any::io::binary::snapshot::{ProcedurePackRecord};
/// 🖨️ The derived text body: the same `ProcedurePackRecord` the pack encodes, printed by the spec-driven engine.
pub(crate) fn print_pack_record_text(snapshot: &ProcedureSnapshot) -> String {
    semio_framework_dsl_record::print(&ProcedurePackRecord::from_snapshot(snapshot).__dsl_to_record(), &ProcedurePackRecord::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document)
}

/// 📖️ Parses a derived text body back through `ProcedurePackRecord`, with the same decode steps as the pack.
pub(crate) fn parse_pack_record_text(body: &str) -> Result<ProcedureSnapshot, semio_framework_diagnostic::TextError> {
    let record = semio_framework_dsl_record::parse(body, &ProcedurePackRecord::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
    Ok(ProcedurePackRecord::__dsl_from_record(&record)?.into_snapshot())
}

/// 🎁 `ArtifactDsl` and `ArtifactPack` are the derived text and pack of `ProcedurePackRecord`.
impl store::ArtifactDsl for ProcedureSnapshot {
    const EXTENSION: &'static str = "imperative";
    fn envelope_id() -> &'static str {
        "imperative.imperative"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_pack_record_text(body)
    }
    fn print_dsl(&self) -> String {
        let body = print_pack_record_text(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📤️ Renders a [`ProcedureSnapshot`] as this facet's own camelCase JSON projection: the schema and the two
/// content-addressed child HANDLES, never content (first-party JSON codec behind this interface).
pub fn encode_procedure_snapshot_json(snapshot: &ProcedureSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ The inverse of [`encode_procedure_snapshot_json`].
pub fn decode_procedure_snapshot_json(text: &str) -> Result<ProcedureSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📝️ Parses `.imperative.dsl.semio` text into a [`ProcedureSnapshot`] — a named pass-through of this type's own
/// `store::ArtifactDsl` impl, whose trait and error type are both unnameable outside this crate.
pub fn parse_procedure_dsl(text: &str) -> Result<ProcedureSnapshot, String> {
    <ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 📝️ Renders a [`ProcedureSnapshot`] back as `.imperative.dsl.semio` text — the inverse of [`parse_procedure_dsl`].
pub fn print_procedure_dsl(snapshot: &ProcedureSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;
