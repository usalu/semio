//! 🗣️ Imperative artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! `Value`/`Atom`/`Dictionary`/`Step`/`Path` are all defined in `neural_engine`/`imperative_engine`
//! (foreign kernel crates out of scope for this conversion), so none of them can carry a
//! `#[derive(dsl::Dsl...)]` themselves — Rust's orphan rule requires the impl target type to live in the
//! crate that also owns the trait or the type, and neither is true for a foreign type receiving a foreign
//! derive. `ValueDsl`/`StepNodeDsl`/`PathDsl` below are local structural twins that the real types convert
//! to/from right at the `parse_dsl`/`print_dsl`/`parse_op`/`print_op` boundary.
//!
//! `ValueDsl` deliberately does NOT route through `dsl_schema`'s built-in `Shape::Value`/`DslValue`
//! dynamic-literal primitive: even now that `DslValue::Number` wraps a typed `UInt`/`Int`/`Float`
//! enum rather than a bare `f64`, its printer still can't reproduce the OLD hand-rolled printer's
//! own distinction (always giving a `Decimal` a trailing `.`) — an existing test parses a bare `1`
//! and expects `Atom::Integer(1)` back. So `ValueDsl` is its own typed record instead: exactly one of its mutually-exclusive `Option`
//! fields is ever `Some`, each keyed so the ACTUAL Rust variant (not a text heuristic) decides which one,
//! which is exactly as precise as the old hand-rolled `Atom` match. `ValueDsl` also derives
//! `dsl::ToValue`/`dsl::FromValue` (on top of `dsl::DslRecord`) so it can nest inside
//! `🎮️commands/🔧️set-step-params`'s `BTreeMap<String, ValueDsl>` payload field — that command struct's
//! own `ToValue`/`FromValue` derive needs it, via the framework's blanket `BTreeMap<String, T: ToValue>`
//! impl (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/02: `app_commands!` itself
//! forces only `ToValue`/`FromValue`/`dsl::DslOps` onto the generated command enum, never serde).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::procedure::{Dictionary, ProcedureSnapshot, Path, Step};
use neural_engine::{Atom, Value};
use std::collections::BTreeMap;

//#region 🔖️Value
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
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

/// 📦️ `None` when `dict` is empty, mirroring the old printer's "omit an empty dictionary section".
pub fn dictionary_to_option_dsl_map(dict: &Dictionary) -> Option<BTreeMap<String, ValueDsl>> {
    (!dict.is_empty()).then(|| dictionary_to_value_dsl_map(dict))
}

pub fn option_dsl_map_to_dictionary(entries: Option<BTreeMap<String, ValueDsl>>) -> Dictionary {
    entries.map(|entries| value_dsl_map_to_dictionary(&entries)).unwrap_or_default()
}
//#endregion 🔖️Value

//#region 🔖️Step
/// 👣️ `Step`'s recursive `bodies: BTreeMap<String, Path>` mirrors through `Path`'s own single field —
/// `StepNodeDsl` is a one-variant `DslEnum` (not a plain `DslRecord`) purely so the mutual recursion with
/// `PathDsl` goes through `dsl::DslVariants`'s LAZY `fn() -> RecordSpec` pointers: building `PathDsl`'s
/// `RecordSpec` needs `StepNodeDsl`'s variant table, and vice versa, and only the lazy pointer indirection
/// keeps that finite instead of recursing forever just to construct the schema.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
pub enum StepNodeDsl {
    Step {
        #[dsl(positional)]
        id: String,
        kind: String,
        params: Option<BTreeMap<String, ValueDsl>>,
        bodies: BTreeMap<String, PathDsl>,
    },
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct PathDsl {
    #[dsl(statements, block)]
    steps: Vec<StepNodeDsl>,
}

pub fn step_to_step_node_dsl(step: &Step) -> StepNodeDsl {
    StepNodeDsl::Step { id: step.id.clone(), kind: step.kind.clone(), params: dictionary_to_option_dsl_map(&step.params), bodies: step.bodies.iter().map(|(slot, path)| (slot.clone(), path_to_path_dsl(path))).collect() }
}

pub fn step_node_dsl_to_step(node: StepNodeDsl) -> Step {
    let StepNodeDsl::Step { id, kind, params, bodies } = node;
    Step { id, kind, params: option_dsl_map_to_dictionary(params), bodies: bodies.into_iter().map(|(slot, path)| (slot, path_dsl_to_path(path))).collect() }
}

pub fn path_to_path_dsl(path: &Path) -> PathDsl {
    PathDsl { steps: path.steps.iter().map(step_to_step_node_dsl).collect() }
}

pub fn path_dsl_to_path(path_dsl: PathDsl) -> Path {
    Path { steps: path_dsl.steps.into_iter().map(step_node_dsl_to_step).collect() }
}
//#endregion 🔖️Step

//#region 🔖️Api
/// 📄️ The default `imperative` document, handcrafted in the `.imperative` DSL.
pub const PROCEDURE_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");

/// 📖️ Parses `.imperative` DSL text into an `ProcedureSnapshot`.
pub fn parse_dsl(text: &str) -> Result<ProcedureSnapshot, store::TextError> {
    <ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints an `ProcedureSnapshot` back to `.imperative` DSL text.
pub fn print_dsl(document: &ProcedureSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}
//#endregion 🔖️Api

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::procedure::{Dictionary as DocDictionary, Step as DocStep};
    use std::collections::BTreeMap as StdBTreeMap;

    fn step(id: &str, kind: &str) -> DocStep {
        DocStep { id: id.into(), kind: kind.into(), params: DocDictionary::new(), bodies: StdBTreeMap::new() }
    }

    #[semio_framework_async_macros::async_test]
    async fn default_snapshot_dsl_round_trips() {
        let document = parse_dsl(PROCEDURE_EXAMPLE_TEXT).expect("parse 📜️default.imperative");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }

    //#region DSL text round trips and error paths
    /// 🔁 Replaces the retired `dsl_parses_seed_and_nested_control_bodies` — the artifact's own DSL
    /// text no longer carries `path`/`seed` content directly (only the opaque `flow`/`text` composed
    /// child HANDLES do, ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`), so the equivalent
    /// real-behavior law now lives at the CONVERTER: `Path`s with nested `control.*` bodies round-trip
    /// losslessly through `flow_content_snapshot_from_path`/`path_from_flow_content_snapshot`, and the
    /// full snapshot (built from that `Path`) still satisfies the DSL/pack round-trip laws.
    #[semio_framework_async_macros::async_test]
    async fn flow_content_round_trips_nested_control_bodies() {
        let inner = step("step-inner", "log.print");
        let mut owner = step("step-if", "control.if");
        owner.bodies.insert("then".to_string(), Path { steps: vec![inner] });
        let path = Path { steps: vec![owner] };

        let flow_snapshot = crate::artifacts::procedure::flow_content_snapshot_from_path(&path);
        let restored = crate::artifacts::procedure::path_from_flow_content_snapshot(&flow_snapshot);
        assert_eq!(restored, path);
        assert_eq!(restored.steps[0].bodies.get("then").map(|body| body.steps.len()), Some(1));

        let seed = StdBTreeMap::from([("counter".into(), Value::Atom(Atom::Integer(1))), ("label".into(), Value::Atom(Atom::String("x".into())))]);
        let document = crate::artifacts::procedure::procedure_snapshot_with_content("procedure.document", &path, &seed);
        store::os_store::test_support::assert_dsl_round_trip(&document);
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    }

    /// 🔁 Replaces the retired `dsl_parses_dictionary_and_atom_variants` — same rationale as
    /// [`flow_content_round_trips_nested_control_bodies`], for `seed`'s `text_content_snapshot_from_seed`/
    /// `seed_from_text_content_snapshot` converter and every `Value`/`Atom` variant it carries.
    #[semio_framework_async_macros::async_test]
    async fn text_content_round_trips_dictionary_and_atom_variants() {
        let seed = StdBTreeMap::from([
            ("a".into(), Value::Atom(Atom::Null)),
            ("b".into(), Value::Atom(Atom::Boolean(true))),
            ("c".into(), Value::Atom(Atom::Boolean(false))),
            ("d".into(), Value::Atom(Atom::Decimal(1.5))),
            ("e".into(), Value::Atom(Atom::Decimal(-1.0))),
            ("f".into(), Value::Dictionary(DocDictionary::new())),
        ]);

        let text_snapshot = crate::artifacts::procedure::text_content_snapshot_from_seed(&seed);
        let restored = crate::artifacts::procedure::seed_from_text_content_snapshot(&text_snapshot);
        assert_eq!(restored, seed);

        let document = crate::artifacts::procedure::procedure_snapshot_with_content("procedure.document", &Path::new(), &seed);
        store::os_store::test_support::assert_dsl_round_trip(&document);
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    }

    /// 🔁 Retired-format twin was `dsl_rejects_unterminated_string`; the new hand-rolled body grammar
    /// has no quoted-string literals, so the equivalent rejection is a malformed hex value (odd length).
    #[semio_framework_async_macros::async_test]
    async fn dsl_rejects_malformed_hex_value() {
        let text = "schema=zzz";
        assert!(<ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(text).is_err());
    }

    /// 🔁 Retired-format twin was `dsl_rejects_wrong_leading_keyword`; the new hand-rolled body is
    /// line-based (`schema=`/`flow=`/`text=`), not keyword-based, so the equivalent rejection is an
    /// unrecognized line.
    #[semio_framework_async_macros::async_test]
    async fn dsl_rejects_unrecognized_body_line() {
        let text = r#"notimperative schema="x""#;
        assert!(<ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(text).is_err());
    }

    /// 🔁 Retired-format twin was `dsl_rejects_invalid_number_literal`; the new hand-rolled body
    /// requires all three lines (`schema=`/`flow=`/`text=`), so the equivalent rejection is a body
    /// missing a required line.
    #[semio_framework_async_macros::async_test]
    async fn dsl_rejects_incomplete_body_missing_required_line() {
        let text = "schema=696d70657261746976652e646f63756d656e74";
        assert!(<ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(text).is_err());
    }
    //#endregion DSL text round trips and error paths
}
//#endregion 🧪️Tests
