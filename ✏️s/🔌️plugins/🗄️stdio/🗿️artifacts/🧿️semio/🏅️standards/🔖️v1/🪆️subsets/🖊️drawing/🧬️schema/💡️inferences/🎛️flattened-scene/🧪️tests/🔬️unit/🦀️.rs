
use super::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, PathSegment, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
use store::{InferenceCache, InferenceCacheConfig};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fixture() -> SemioDrawingSnapshot {
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width: 10.0, height: 10.0, background: None },
        styles: vec![DrawStyle { name: "s1".into(), fill: Some(crate::standards::v1::subsets::base::schema::geometry::SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }), stroke: None, stroke_width: Some(2.0), opacity: None }],
        layers: vec![DrawLayer {
            id: "l0".into(),
            name: "base".into(),
            visible: true,
            root: DrawNode::Group {
                transform: SemioTransform { translation: SemioPoint3 { x: 10.0, y: 0.0, z: 0.0 }, rotation: SemioQuaternion::default(), scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } },
                children: vec![
                    DrawNode::Path { segments: vec![PathSegment::MoveTo { to: SemioPoint2 { x: 0.0, y: 0.0 } }], style: Some("s1".into()) },
                    DrawNode::Group {
                        transform: SemioTransform { translation: SemioPoint3 { x: 5.0, y: 0.0, z: 0.0 }, rotation: SemioQuaternion::default(), scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } },
                        children: vec![DrawNode::Text { value: "hi".into(), at: SemioPoint2 { x: 0.0, y: 0.0 }, style: None }],
                    },
                ],
            },
        }],
    }
}

#[semio_framework_async_macros::async_test]
async fn world_transform_composes_down_through_nested_groups() {
    let snapshot = fixture();
    let values = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(&snapshot, None);
    let root = &values[&key_for(0, &[])];
    assert_eq!(root.world_transform.translation.x, 10.0);
    let path_child = &values[&key_for(0, &[0])];
    assert_eq!(path_child.world_transform.translation.x, 10.0, "Path inherits the parent Group's world transform unchanged");
    let nested_group = &values[&key_for(0, &[1])];
    assert_eq!(nested_group.world_transform.translation.x, 15.0, "10 (parent) + 5 (own local translation)");
    let nested_text = &values[&key_for(0, &[1, 0])];
    assert_eq!(nested_text.world_transform.translation.x, 15.0, "Text inherits its parent Group's composed world transform");
}

#[semio_framework_async_macros::async_test]
async fn style_reference_resolves_to_the_real_value() {
    let snapshot = fixture();
    let values = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(&snapshot, None);
    let path_child = &values[&key_for(0, &[0])];
    assert_eq!(path_child.resolved_style, Some(snapshot.styles[0].clone()));
    let nested_text = &values[&key_for(0, &[1, 0])];
    assert_eq!(nested_text.resolved_style, None, "Text with no style ref resolves to None");
}

//#region 🧪️IncrementalityLaw
/// 🍃️ Puzzle3d-pilot-shaped law: a LEAF's own field (its `style` reference — the only field of
/// `Path`/`Text` this inference actually reads) changes → only that leaf misses; every ancestor
/// (and every unrelated sibling subtree) stays a cache hit. The `hits` assertion is the load-
/// bearing half: a miss-count-only check can't distinguish "only the leaf missed" from "the leaf
/// missed AND everything else was never looked up"; asserting `hits == plan.len() - 1` proves
/// every other entity was actually consulted and found warm.
#[semio_framework_async_macros::async_test]
async fn changing_a_leaf_own_style_does_not_recompute_ancestors_or_siblings() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = fixture();
    let _ = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(&base, Some(&mut cache));

    let mut changed = base.clone();
    changed.styles[0].stroke_width = Some(99.0);
    let before = cache.stats().await;
    let values = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(&changed, Some(&mut cache));
    let after = cache.stats().await;

    assert_eq!(after.misses - before.misses, 1, "only the leaf referencing the changed style may miss");
    assert_eq!(after.hits - before.hits, 3, "root + the unrelated nested Group + the unrelated nested Text (its sibling subtree) must all remain cache hits");
    assert_eq!(values[&key_for(0, &[0])].resolved_style.as_ref().unwrap().stroke_width, Some(99.0));
}

/// 🌳️ Ancestor law: changing the ROOT's own transform must miss for every entity in the plan
/// (root + every descendant transitively folds root's `DepHash` into its own chain).
#[semio_framework_async_macros::async_test]
async fn changing_the_root_transform_recomputes_the_whole_subtree() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = fixture();
    let _ = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(&base, Some(&mut cache));

    let mut changed = base.clone();
    let DrawNode::Group { transform, .. } = &mut changed.layers[0].root else { panic!() };
    transform.translation.x = 999.0;
    let before = cache.stats().await;
    let _ = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(&changed, Some(&mut cache));
    let after = cache.stats().await;

    assert_eq!(after.misses - before.misses, 4, "root + its 3 descendants all depend on the root's world transform");
    assert_eq!(after.hits - before.hits, 0, "a root-wide change leaves nothing warm");
}

/// 🤝️ Sibling law: two INDEPENDENT leaves (each referencing its own style, under different
/// parent Groups so neither is an ancestor of the other) — editing one's referenced style must
/// leave the other's entire subtree, and everything between it and the shared root, warm.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn two_independent_styled_siblings() -> SemioDrawingSnapshot {
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width: 10.0, height: 10.0, background: None },
        styles: vec![
            DrawStyle { name: "s1".into(), fill: Some(crate::standards::v1::subsets::base::schema::geometry::SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }), stroke: None, stroke_width: Some(2.0), opacity: None },
            DrawStyle { name: "s2".into(), fill: Some(crate::standards::v1::subsets::base::schema::geometry::SemioRgba { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }), stroke: None, stroke_width: Some(3.0), opacity: None },
        ],
        layers: vec![DrawLayer {
            id: "l0".into(),
            name: "base".into(),
            visible: true,
            root: DrawNode::Group {
                transform: SemioTransform::identity(),
                children: vec![
                    DrawNode::Path { segments: vec![PathSegment::MoveTo { to: SemioPoint2 { x: 0.0, y: 0.0 } }], style: Some("s1".into()) },
                    DrawNode::Path { segments: vec![PathSegment::MoveTo { to: SemioPoint2 { x: 1.0, y: 1.0 } }], style: Some("s2".into()) },
                ],
            },
        }],
    }
}

#[semio_framework_async_macros::async_test]
async fn an_unrelated_sibling_edit_leaves_the_other_siblings_chain_warm() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = two_independent_styled_siblings();
    let baseline = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(&base, Some(&mut cache));
    let sibling_before = baseline[&key_for(0, &[1])].clone();

    let mut changed = base.clone();
    changed.styles[0].stroke_width = Some(42.0); // only sibling 0 (style "s1") is affected
    let before = cache.stats().await;
    let values = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(&changed, Some(&mut cache));
    let after = cache.stats().await;

    assert_eq!(after.misses - before.misses, 1, "only sibling 0 (which references the changed style) may miss");
    assert_eq!(after.hits - before.hits, 2, "root + sibling 1 (which references a different, untouched style) must remain cache hits");
    assert_eq!(values[&key_for(0, &[1])], sibling_before, "sibling 1's flattened value must be byte-identical — it never depended on sibling 0's style");
}
//#endregion 🧪️IncrementalityLaw

#[semio_framework_async_macros::async_test]
async fn disabled_cache_matches_pure_recompute() {
    let snapshot = fixture();
    let pure = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(&snapshot, None);
    let mut disabled = InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() }).await;
    let via_disabled = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(&snapshot, Some(&mut disabled));
    assert_eq!(pure, via_disabled);
}

#[semio_framework_async_macros::async_test]
async fn quaternion_rotation_of_identity_is_a_no_op() {
    let p = SemioPoint3 { x: 3.0, y: 4.0, z: 5.0 };
    assert_eq!(rotate_point(SemioQuaternion::default(), p), p);
}
