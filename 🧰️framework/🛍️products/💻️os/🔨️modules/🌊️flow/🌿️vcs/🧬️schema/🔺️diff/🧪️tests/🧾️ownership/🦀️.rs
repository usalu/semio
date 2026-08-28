//! 🧪️ Borrowed validation preserves retained Flow owners through ordered rejection.
use super::*;
use crate::os_spr::Identified;
use std::cell::Cell;
use std::rc::Rc;

//#region 🧪️RetainedPayloads
fn retire_diff(diff: FlowDiff) {
    for delta in diff.deltas {
        match delta {
            FlowDelta::Widgets(delta) => {
                for (_, widget) in delta.inserted { widget.retire_cold(); }
                for (_, widget) in delta.replaced { widget.retire_cold(); }
            }
            FlowDelta::Fixture(fixture) => fixture.retire_cold(),
            _ => {}
        }
    }
}

#[test]
fn retained_payload_projection_matches_neutral_vectors() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("🔣️vectors.json")).unwrap();
    let base: FlowFixture = serde_json::from_value(vectors["base"].clone()).unwrap();
    let base_json = serde_json::to_value(&base).unwrap();
    for row in vectors["cases"].as_array().unwrap() {
        let diff: FlowDiff = serde_json::from_value(row["diff"].clone()).unwrap();
        let diff_json = serde_json::to_value(&diff).unwrap();
        match diff.apply(&base) {
            Ok(result) => {
                assert!(row.get("errorCode").is_none(), "{}", row["name"]);
                assert_eq!(serde_json::to_value(result.widgets.iter().map(|widget| widget.id()).collect::<Vec<_>>()).unwrap(), row["expectedWidgetIds"], "{}", row["name"]);
                assert_eq!(serde_json::to_value(result.synapses.iter().map(|synapse| &synapse.id).collect::<Vec<_>>()).unwrap(), row["expectedSynapseIds"], "{}", row["name"]);
                assert_eq!(serde_json::to_value(result.layout.keys().collect::<Vec<_>>()).unwrap(), row["expectedLayoutIds"], "{}", row["name"]);
                result.retire_cold();
            }
            Err(error) => assert_eq!(Some(error.code.as_str()), row["errorCode"].as_str(), "{}", row["name"]),
        }
        assert_eq!(serde_json::to_value(&base).unwrap(), base_json, "{}", row["name"]);
        assert_eq!(serde_json::to_value(&diff).unwrap(), diff_json, "{}", row["name"]);
        retire_diff(diff);
    }
    base.retire_cold();
}
//#endregion 🧪️RetainedPayloads

//#region 🧪️BorrowedCollection
struct Payload {
    id: String,
    clones: Rc<Cell<usize>>,
    drops: Rc<Cell<usize>>,
}

impl Identified<String> for Payload {
    fn id(&self) -> &String { &self.id }
}

impl Clone for Payload {
    fn clone(&self) -> Self {
        self.clones.set(self.clones.get() + 1);
        Self { id: self.id.clone(), clones: self.clones.clone(), drops: self.drops.clone() }
    }
}

impl Drop for Payload {
    fn drop(&mut self) { self.drops.set(self.drops.get() + 1); }
}

#[test]
fn collection_validation_never_clones_or_drops_payloads() {
    let clones = Rc::new(Cell::new(0));
    let drops = Rc::new(Cell::new(0));
    let make = |id: &str| Payload { id: id.into(), clones: clones.clone(), drops: drops.clone() };
    let a = make("a");
    let b = make("b");
    let delta = FlowCollectionDelta { removed: vec!["b".into()], replaced: vec![("a".into(), make("renamed"))], inserted: Vec::new() };
    let mut items = vec![&a, &b];
    apply_flow_collection_delta(&mut items, &delta).unwrap();
    assert!(std::ptr::eq(items[0], &delta.replaced[0].1));
    assert_eq!(items.len(), 1);
    assert_eq!(clones.get(), 0);
    assert_eq!(drops.get(), 0);
    let invalid = FlowCollectionDelta { removed: vec!["renamed".into(), "missing".into()], replaced: Vec::new(), inserted: Vec::new() };
    assert_eq!(apply_flow_collection_delta(&mut items, &invalid).unwrap_err().code, "mutation.apply.missing-target");
    assert_eq!(clones.get(), 0);
    assert_eq!(drops.get(), 0);
    drop(items);
    drop(delta);
    drop(a);
    drop(b);
    assert_eq!(drops.get(), 3);
}
//#endregion 🧪️BorrowedCollection
