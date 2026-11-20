//! 🧪️ Original Drawing lookup preserves identity, first-owner order and paid cancellation.

use super::*;
use semio_framework_value::{RetainedAllocationEffect, ValueError, ValueRefusalKind, paged::PagedUtf8, retained_clone::{RetainedCloneGrant, RetainedCloneStep}};
use semio_framework_value_clone_source_testing::{FixtureSource, drive::drive_reply};
use semio_s_artifact_draw_drawing_model::{layer_base, snapshot::{DrawingSnapshot, lookup::original::DRAWING_ORIGINAL_MAXIMUM_DEPTH}};
use serde_json::{Value, json};

const BODY: usize = 8192;
const GRANT: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1,
    maximum_copy_bytes: BODY,
    maximum_capacity_bytes: 1 << 24,
    maximum_release_bytes: 1 << 24,
    maximum_depth: 128,
};

fn turn(grant: RetainedCloneGrant, mut operation: impl FnMut(&mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError>) -> Result<RetainedCloneStep, ValueError> {
    drive_reply::<RetainedCloneStep, ValueError>(grant, 256, |result| result.as_ref().map_or_else(|error| error.retained_progress(), |step| step.progress()), |effect, reply, recovering| {
        assert!(!recovering);
        Ok(reply.complete(operation(effect)))
    }).result
}

#[test]
fn drawing_original_lookup_preserves_both_sources_and_refuses_every_zero_axis() {
    let law: Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let mut template: Value = serde_json::from_str(include_str!("../../../../../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let mut later = template["layers"][1]["children"][0].clone();
    later["name"] = "Later Duplicate".into();
    template["layers"].as_array_mut().unwrap().push(later);
    let long = format!("{}\0", law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize));
    let mut deep = template["layers"][0].clone();
    deep["id"] = long.clone().into();
    deep["name"] = "Long UTF8".into();
    for depth in 0..DRAWING_ORIGINAL_MAXIMUM_DEPTH - 1 {
        let mut group = template["layers"][1].clone();
        group["id"] = format!("nested-{depth}").into();
        group["children"] = json!([deep]);
        deep = group;
    }
    let mut cases = law["lookup"]["cases"].as_array().unwrap().clone();
    let ordinal = template["layers"].as_array().unwrap().len();
    let path = std::iter::once(ordinal).chain(std::iter::repeat_n(0usize, DRAWING_ORIGINAL_MAXIMUM_DEPTH - 1)).collect::<Vec<_>>();
    cases.push(json!({ "target": &long, "name": "Long UTF8", "path": path }));
    cases.push(json!({ "target": "missing-after-deep-tree", "name": null, "path": null }));
    let mut overflow = template.clone();
    let mut parent = template["layers"][1].clone();
    parent["id"] = "outer-overflow".into();
    parent["children"] = json!([deep.clone()]);
    overflow["layers"] = json!([parent]);
    template["layers"].as_array_mut().unwrap().push(deep);

    let source = FixtureSource::from_owner(serde_json::from_value::<DrawingSnapshot>(template).unwrap());
    let original = serde_json::to_value(source.borrow().get()).unwrap();
    for case in &cases {
        let value = case["target"].as_str().unwrap();
        let target = FixtureSource::from_owner(PagedUtf8::<{usize::MAX}>::try_from_str(value).unwrap());
        let unrelated = FixtureSource::from_owner(PagedUtf8::<{usize::MAX}>::try_from_str(value).unwrap());
        let pauses = law["cancelAt"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize);
        for limit in pauses.chain([usize::MAX]) {
            let mut lookup = DrawingOriginalLookup::default();
            for step in 0..25_000 {
                if step == limit { break; }
                let demand = lookup.demand(source.borrow(), target.borrow(), BODY).unwrap();
                let mut denied = [GRANT; 5];
                denied[0].maximum_items = 0;
                denied[1].maximum_copy_bytes = 0;
                denied[2].maximum_capacity_bytes = 0;
                denied[3].maximum_release_bytes = 0;
                denied[4].maximum_depth = 0;
                let unavailable = [true, demand.copy_bytes > 0, demand.capacity_bytes > 0, demand.release_bytes > 0, demand.depth > 0];
                if step < 3 {
                    for (grant, unavailable) in denied.into_iter().zip(unavailable) {
                        if !unavailable { continue; }
                        let stopped = turn(grant, |effect| lookup.advance(source.borrow(), target.borrow(), BODY, effect)).unwrap();
                        assert_eq!(stopped.progress(), Default::default());
                        assert_eq!(lookup.demand(source.borrow(), target.borrow(), BODY).unwrap(), demand);
                    }
                }
                turn(GRANT, |effect| lookup.advance(source.borrow(), target.borrow(), BODY, effect)).unwrap();
                if step == 2 {
                    assert!(lookup.demand(source.borrow(), unrelated.borrow(), BODY).is_err());
                }
                if lookup.ready() { break; }
            }
            if limit == usize::MAX {
                let address = lookup.address().unwrap();
                assert_eq!(json!(address.map(|address| address.indices[..address.length].to_vec())), case["path"]);
                let name = address.map(|address| {
                    let projected = address.project(source.borrow());
                    assert!(std::ptr::eq(projected.get(), address.node(source.borrow().get()).unwrap()));
                    layer_base(projected.get()).name.to_string_owner()
                });
                assert_eq!(json!(name), case["name"]);
            }
            lookup.begin_close();
            assert_eq!(turn(Default::default(), |effect| lookup.close_step(BODY, effect)).unwrap().progress(), Default::default());
            for _ in 0..1000 {
                if lookup.terminal_is_empty() { break; }
                turn(GRANT, |effect| lookup.close_step(BODY, effect)).unwrap();
            }
            assert!(lookup.terminal_is_empty());
        }
    }
    assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(), original);

    let overflow = FixtureSource::from_owner(serde_json::from_value::<DrawingSnapshot>(overflow).unwrap());
    let target = FixtureSource::from_owner(PagedUtf8::<{usize::MAX}>::try_from_str(&long).unwrap());
    let mut lookup = DrawingOriginalLookup::default();
    let refusal = (0..1000).find_map(|_| turn(GRANT, |effect| lookup.advance(overflow.borrow(), target.borrow(), BODY, effect)).err()).unwrap();
    assert_eq!(refusal.primary().unwrap().kind, ValueRefusalKind::DepthLimit);
    lookup.begin_close();
    for _ in 0..1000 {
        if lookup.terminal_is_empty() { break; }
        turn(GRANT, |effect| lookup.close_step(BODY, effect)).unwrap();
    }
    assert!(lookup.terminal_is_empty());
}
