//! 🧪️ Borrowed validation preserves retained Flow owners through ordered rejection.
use super::*;
use crate::os_spr::Identified;
use crate::vcs::apply_flow_collection_delta;
use std::cell::Cell;
use std::rc::Rc;

//#region 🧪️RetainedPayloads
fn retire_diff(diff: FlowDiff) { MutationDiff::retire_cold(diff); }

/// 🧊️ Every delta variant must survive the generic cold-retirement seam the store's replay and
/// history folds use (`os_store::replay_mutations`), including the `Fixture` variant whose
/// `OrderedMap<WidgetLayout>` root aborts the process on a bare drop.
#[test]
fn every_delta_variant_retires_cold_without_a_bare_drop() {
    let vectors = crate::os_pack::json::parse(include_str!("../../🧫️fixtures/🧾️ownership/🔣️.json")).unwrap();
    let base: FlowFixture = crate::os_dsl::FromValue::from_value(crate::os_pack::json::to_dsl_value(vectors.get("base").unwrap())).unwrap();
    let widget = base.widgets.first().cloned().unwrap();
    let synapse = base.synapses.first().cloned().unwrap();
    let deltas = vec![
        FlowDelta::Widgets(FlowCollectionDelta { removed: vec!["gone".into()], inserted: vec![(0, widget.clone())], replaced: vec![(widget.id().clone(), widget)] }),
        FlowDelta::Synapses(FlowCollectionDelta { removed: vec!["gone".into()], inserted: vec![(0, synapse.clone())], replaced: vec![(synapse.id.clone(), synapse)] }),
        FlowDelta::Layout(base.layout.iter().map(|(id, layout)| FlowLayoutEntry { id: id.clone(), layout: Some(layout.clone()) }).collect()),
        FlowDelta::Fixture(base.clone()),
    ];
    assert_eq!(deltas.len(), 4);
    MutationDiff::retire_cold(FlowDiff { deltas });
    <FlowDiff as MutationDiff<FlowFixture>>::retire_projection(base);
}

#[test]
fn retained_payload_projection_matches_neutral_vectors() {
    let vectors = crate::os_pack::json::parse(include_str!("../../🧫️fixtures/🧾️ownership/🔣️.json")).unwrap();
    let base: FlowFixture = crate::os_dsl::FromValue::from_value(crate::os_pack::json::to_dsl_value(vectors.get("base").unwrap())).unwrap();
    let base_json = crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&base));
    for row in vectors.get("cases").and_then(crate::os_pack::json::Value::as_array).unwrap() {
        let diff: FlowDiff = crate::os_dsl::FromValue::from_value(crate::os_pack::json::to_dsl_value(row.get("diff").unwrap())).unwrap();
        let diff_json = crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&diff));
        let name = row.get("name").and_then(crate::os_pack::json::Value::as_str).unwrap_or_default();
        match diff.apply(&base) {
            Ok(result) => {
                assert!(row.get("errorCode").is_none(), "{name}");
                let widget_ids: Vec<String> = result.widgets.iter().map(|widget| widget.id().clone()).collect();
                assert_eq!(crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&widget_ids)), row.get("expectedWidgetIds").cloned().unwrap(), "{name}");
                let synapse_ids: Vec<String> = result.synapses.iter().map(|synapse| synapse.id.clone()).collect();
                assert_eq!(crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&synapse_ids)), row.get("expectedSynapseIds").cloned().unwrap(), "{name}");
                let layout_ids: Vec<String> = result.layout.keys().cloned().collect();
                assert_eq!(crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&layout_ids)), row.get("expectedLayoutIds").cloned().unwrap(), "{name}");
                result.retire_cold();
            }
            Err(error) => assert_eq!(Some(error.code.as_str()), row.get("errorCode").and_then(crate::os_pack::json::Value::as_str), "{name}"),
        }
        assert_eq!(crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&base)), base_json, "{name}");
        assert_eq!(crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&diff)), diff_json, "{name}");
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
