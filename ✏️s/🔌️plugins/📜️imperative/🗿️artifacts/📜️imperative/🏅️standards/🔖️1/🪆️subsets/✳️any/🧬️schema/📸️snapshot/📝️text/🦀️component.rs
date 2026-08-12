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
//! dynamic-literal primitive: `DslValue::Number(f64)` merges `Atom::Integer`/`Atom::Decimal` into one case
//! (the OLD hand-rolled printer distinguished them by always giving a `Decimal` a trailing `.`), which is
//! a real, observable loss of fidelity — an existing test parses a bare `1` and expects `Atom::Integer(1)`
//! back. So `ValueDsl` is its own typed record instead: exactly one of its mutually-exclusive `Option`
//! fields is ever `Some`, each keyed so the ACTUAL Rust variant (not a text heuristic) decides which one,
//! which is exactly as precise as the old hand-rolled `Atom` match. `ValueDsl` also derives
//! `serde::Serialize`/`Deserialize` (on top of `dsl::DslRecord`) so it can nest inside `🎮️commands/🔧️step`
//! payload structs — `app_commands!` forces those derives onto the generated `ImperativeCommand` enum.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::imperative::{Dictionary, ImperativeSnapshot, Path, Step};
use neural_engine::{Atom, Value};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
pub const IMPERATIVE_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.imperative` DSL text into an `ImperativeSnapshot`.
pub fn parse_dsl(text: &str) -> Result<ImperativeSnapshot, store::TextError> {
    <ImperativeSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints an `ImperativeSnapshot` back to `.imperative` DSL text.
pub fn print_dsl(document: &ImperativeSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}
//#endregion 🔖️Api

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::imperative::{Dictionary as DocDictionary, Step as DocStep};
    use std::collections::BTreeMap as StdBTreeMap;

    fn step(id: &str, kind: &str) -> DocStep {
        DocStep { id: id.into(), kind: kind.into(), params: DocDictionary::new(), bodies: StdBTreeMap::new() }
    }

    #[test]
    fn default_snapshot_dsl_round_trips() {
        let document = parse_dsl(IMPERATIVE_EXAMPLE_TEXT).expect("parse 📜️default.imperative");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }

    //#region DSL text round trips and error paths
    #[test]
    fn dsl_parses_seed_and_nested_control_bodies() {
        let mut document = parse_dsl(IMPERATIVE_EXAMPLE_TEXT).expect("parse 📜️default.imperative");
        document.seed = StdBTreeMap::from([
            ("counter".into(), Value::Atom(Atom::Integer(1))),
            ("label".into(), Value::Atom(Atom::String("x".into()))),
        ]);
        let inner = step("step-inner", "log.print");
        let mut owner = step("step-if", "control.if");
        owner.bodies.insert("then".to_string(), Path { steps: vec![inner] });
        document.path.steps = vec![owner];

        assert_eq!(document.seed.get("counter"), Some(&Value::Atom(Atom::Integer(1))));
        let owner = &document.path.steps[0];
        assert_eq!(owner.bodies.get("then").map(|body| body.steps.len()), Some(1));
        store::os_store::test_support::assert_dsl_round_trip(&document);
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    }

    #[test]
    fn dsl_parses_dictionary_and_atom_variants() {
        let mut document = parse_dsl(IMPERATIVE_EXAMPLE_TEXT).expect("parse 📜️default.imperative");
        document.seed = StdBTreeMap::from([
            ("a".into(), Value::Atom(Atom::Null)),
            ("b".into(), Value::Atom(Atom::Boolean(true))),
            ("c".into(), Value::Atom(Atom::Boolean(false))),
            ("d".into(), Value::Atom(Atom::Decimal(1.5))),
            ("e".into(), Value::Atom(Atom::Decimal(-1.0))),
            ("f".into(), Value::Dictionary(DocDictionary::new())),
        ]);

        assert_eq!(document.seed.get("a"), Some(&Value::Atom(Atom::Null)));
        assert_eq!(document.seed.get("b"), Some(&Value::Atom(Atom::Boolean(true))));
        assert_eq!(document.seed.get("c"), Some(&Value::Atom(Atom::Boolean(false))));
        assert_eq!(document.seed.get("d"), Some(&Value::Atom(Atom::Decimal(1.5))));
        assert_eq!(document.seed.get("e"), Some(&Value::Atom(Atom::Decimal(-1.0))));
        assert_eq!(document.seed.get("f"), Some(&Value::Dictionary(DocDictionary::new())));
        store::os_store::test_support::assert_dsl_round_trip(&document);
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    }

    #[test]
    fn dsl_rejects_unterminated_string() {
        let text = r#"imperative schema="unterminated"#;
        assert!(<ImperativeSnapshot as store::ArtifactDsl>::parse_dsl(text).is_err());
    }

    #[test]
    fn dsl_rejects_wrong_leading_keyword() {
        let text = r#"notimperative schema="x""#;
        assert!(<ImperativeSnapshot as store::ArtifactDsl>::parse_dsl(text).is_err());
    }

    #[test]
    fn dsl_rejects_invalid_number_literal() {
        let text = r#"imperative schema="imperative.document" seed={ n=1.2.3 }"#;
        assert!(<ImperativeSnapshot as store::ArtifactDsl>::parse_dsl(text).is_err());
    }
    //#endregion DSL text round trips and error paths
}
//#endregion 🧪️Tests
