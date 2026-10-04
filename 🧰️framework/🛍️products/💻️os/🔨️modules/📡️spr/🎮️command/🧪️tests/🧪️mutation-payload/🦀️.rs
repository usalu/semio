//! 🧪️ `#[derive(Mutations)]`'s generic payload accessors on the counter fixture aggregate: every row of `INPUT_SCHEMAS`
//! is its leaf's `MutationLeaf::PAYLOAD_SCHEMA`, `payload_value` is the leaf payload without the aggregate's
//! `operation` tag, and `with_payload_value` rebuilds the same kind or refuses — never another kind.

use super::mutation_laws_fixture::{AddCounter, AddCounterTwice, Counter, CounterMutation};
use crate::os_spr::{Mutation, MutationLeaf, SemanticMutation};
use crate::{DslValue, ToValue};
use super::super::{mutation_fixture_ops, mutation_input_schema_failures, mutation_label_failures, mutation_payload_round_trip_failures};

#[test]
fn input_schemas_follow_the_descriptor_rows() {
    let schemas = <CounterMutation as Mutation<Counter>>::INPUT_SCHEMAS;
    assert_eq!(schemas.len(), <CounterMutation as Mutation<Counter>>::DESCRIPTORS.len());
    assert_eq!(schemas[0], <AddCounter as MutationLeaf>::PAYLOAD_SCHEMA);
    assert!(schemas[0].contains("/mutation/add-counter/schema.json"), "{}", schemas[0]);
    assert!(schemas[1].contains("/mutation/add-counter-twice/schema.json"), "{}", schemas[1]);
    let op = CounterMutation::AddCounterTwice(AddCounterTwice { delta: 2 });
    assert_eq!(Mutation::<Counter>::input_schema(&op), Some(schemas[1]));
}

#[test]
fn payload_value_strips_the_aggregate_tag_and_rebuilds_the_same_kind() {
    let op = CounterMutation::AddCounter(AddCounter { delta: 3 });
    assert_eq!(op.to_value().get("operation").and_then(DslValue::as_str), Some("addCounter"));
    let payload = Mutation::<Counter>::payload_value(&op);
    assert_eq!(payload, AddCounter { delta: 3 }.to_value());
    assert!(payload.get("operation").is_none());
    let edited = DslValue::object([("delta".to_string(), DslValue::int(-7))]);
    assert_eq!(Mutation::<Counter>::with_payload_value(&op, edited.clone()).expect("an add-counter payload"), CounterMutation::AddCounter(AddCounter { delta: -7 }));
    let twice = CounterMutation::AddCounterTwice(AddCounterTwice { delta: 1 });
    assert_eq!(Mutation::<Counter>::with_payload_value(&twice, edited).expect("an add-counter-twice payload"), CounterMutation::AddCounterTwice(AddCounterTwice { delta: -7 }));
    assert!(Mutation::<Counter>::with_payload_value(&op, DslValue::object([("deltas".to_string(), DslValue::Array(Vec::new()))])).is_err());
}

#[test]
fn from_payload_value_builds_each_kind_from_its_payload_alone() {
    let payload = DslValue::object([("delta".to_string(), DslValue::int(4))]);
    assert_eq!(<CounterMutation as Mutation<Counter>>::from_payload_value("add-counter", payload.clone()).expect("add-counter"), CounterMutation::AddCounter(AddCounter { delta: 4 }));
    assert_eq!(<CounterMutation as Mutation<Counter>>::from_payload_value("add-counter-twice", payload.clone()).expect("add-counter-twice"), CounterMutation::AddCounterTwice(AddCounterTwice { delta: 4 }));
    assert!(<CounterMutation as Mutation<Counter>>::from_payload_value("remove-counter", payload).is_err());
}

#[test]
fn the_payload_law_holds_for_every_editable_op_and_reads_only_decodable_fixtures() {
    let ops = [CounterMutation::AddCounter(AddCounter { delta: 3 }), CounterMutation::AddCounterTwice(AddCounterTwice { delta: -1 })];
    assert_eq!(mutation_payload_round_trip_failures::<Counter, _>(ops.to_vec()), Vec::<String>::new());
    let root = std::env::temp_dir().join(format!("semio-payload-law-{}", std::process::id()));
    let fixture = root.join("🧫️fixtures").join("➕️add").join("🦠️mutation");
    std::fs::create_dir_all(&fixture).expect("fixture dir");
    std::fs::write(fixture.join("🔣️.json"), r#"{"operation":"addCounter","delta":9}"#).expect("fixture");
    std::fs::create_dir_all(root.join("other").join("🦠️mutation")).expect("sibling dir");
    std::fs::write(root.join("other").join("🦠️mutation").join("🔣️.json"), r#"{"operation":"somethingElse"}"#).expect("sibling");
    let (decoded, files) = mutation_fixture_ops::<CounterMutation>(&root);
    std::fs::remove_dir_all(&root).expect("cleanup");
    assert_eq!((decoded, files), (vec![CounterMutation::AddCounter(AddCounter { delta: 9 })], 2));
}

/// 🙈️ A hand-written aggregate over the counter that forwards its behavior but publishes no payload schema and labels itself
/// without German — the shape the editability and label laws must refuse.
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
struct Unpublished {
    op: CounterMutation,
}

impl Mutation<Counter> for Unpublished {
    type Diff = <CounterMutation as Mutation<Counter>>::Diff;
    const DESCRIPTORS: &'static [crate::os_spr::MutationLeafDescriptor] = <CounterMutation as Mutation<Counter>>::DESCRIPTORS;
    fn descriptor(&self) -> &'static crate::os_spr::MutationLeafDescriptor {
        self.op.descriptor()
    }
    fn diff(&self, base: &Counter) -> crate::os_spr::MutationOutcome<Self::Diff> {
        self.op.diff(base)
    }
    fn inverse(&self, base: &Counter) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok({
        self.op.inverse(base)?.into_iter().map(|op| Self { op }).collect()
    
    })
}
}

impl SemanticMutation<Counter> for Unpublished {
    fn kinds() -> &'static [crate::os_spr::SemanticDescriptor] {
        <CounterMutation as SemanticMutation<Counter>>::kinds()
    }
    fn semantics(&self) -> &'static crate::os_spr::SemanticDescriptor {
        self.op.semantics()
    }
    fn label(&self) -> crate::LocalizedLabel {
        crate::LocalizedLabel::native("Add to the counter", "")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

fn witnesses() -> Vec<CounterMutation> {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️mutation-editability/🔣️.json")).expect("editability fixture");
    fixture["cases"].as_array().expect("cases").iter().map(|case| semio_framework_pack_json::from_json_str(&case["wire"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("a counter witness")).collect()
}

/// ⚖️ LAW: every runtime operation reaches the language-agnostic fixture's editability verdict — editable exactly when it has an
/// input schema and cannot emit foreign steps — which the static `schema mutation-editability` gate reaches from source alone.
#[test]
fn every_witness_reaches_its_editability_verdict() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️mutation-editability/🔣️.json")).expect("editability fixture");
    for (op, case) in witnesses().iter().zip(fixture["cases"].as_array().expect("cases")) {
        let verdict = match (Mutation::<Counter>::input_schema(op).is_some(), Mutation::<Counter>::may_emit_foreign_steps(op)) {
            (_, true) => "foreign",
            (true, false) => "editable",
            (false, false) => "inert",
        };
        assert_eq!((op.descriptor().semantic_kind, verdict), (case["kind"].as_str().expect("kind"), case["verdict"].as_str().expect("verdict")));
    }
}

/// ⚖️ LAW: a derived aggregate publishes one payload schema per leaf; a hand-written one that forwards none is refused.
#[test]
fn every_leaf_publishes_one_payload_schema() {
    assert_eq!(mutation_input_schema_failures::<Counter, CounterMutation>(), Vec::<String>::new());
    assert_eq!(mutation_input_schema_failures::<Counter, Unpublished>(), vec!["0 payload schema(s) for 5 leaf descriptor(s)".to_string()]);
}

/// ⚖️ LAW: every derived operation is labelled in every locale; a label with an empty German cell names that cell.
#[test]
fn the_label_law_reads_every_locale_of_every_operation() {
    assert_eq!(mutation_label_failures::<Counter, _>(&witnesses()), Vec::<String>::new());
    let unpublished: Vec<Unpublished> = witnesses().into_iter().map(|op| Unpublished { op }).collect();
    let failures = mutation_label_failures::<Counter, _>(&unpublished);
    assert_eq!(failures.len(), unpublished.len());
    assert!(failures.iter().all(|failure| failure.contains("native/de") && !failure.contains("/en")), "{failures:#?}");
}

/// ⚖️ LAW: an operation without an input schema is editable by no one, so it must refuse to rebuild from its payload — a
/// hand-written aggregate whose defaults decode the whole value breaks the law once per operation.
#[test]
fn an_operation_without_a_schema_refuses_its_payload() {
    let unpublished: Vec<Unpublished> = witnesses().into_iter().map(|op| Unpublished { op }).collect();
    let count = unpublished.len();
    let failures = mutation_payload_round_trip_failures::<Counter, _>(unpublished);
    assert_eq!(failures.len(), count, "{failures:#?}");
    assert!(failures.iter().all(|failure| failure.contains("declares no input schema yet rebuilds from its payload")), "{failures:#?}");
}
