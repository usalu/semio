//! 🧪️ `#[derive(Mutations)]`'s generic payload accessors on the counter fixture aggregate: every row of `INPUT_SCHEMAS`
//! is its leaf's `MutationLeaf::PAYLOAD_SCHEMA`, `payload_value` is the leaf payload without the aggregate's
//! `operation` tag, and `with_payload_value` rebuilds the same kind or refuses — never another kind.

use super::mutation_laws_fixture::{AddCounter, AddCounterTwice, Counter, CounterMutation};
use crate::os_spr::{Mutation, MutationLeaf};
use crate::{DslValue, ToValue};
use super::super::{mutation_payload_round_trip_failures, mutation_fixture_ops};

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
