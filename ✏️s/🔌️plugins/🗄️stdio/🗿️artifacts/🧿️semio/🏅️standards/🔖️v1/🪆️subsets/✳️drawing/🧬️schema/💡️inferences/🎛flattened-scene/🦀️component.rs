//! 🎛 `flattened-scene` — one named inference: world transform (composed down through nested
//! `Group`s) + resolved style (name lookup into `styles` materialized into the real value) per
//! scene-graph entity. Direct schema-level replacement for the framework's own (deleted-by-this-
//! ticket) `◻2d/🗄️store/🦀️component.rs` `DrawingStore::flatten_handle`/`flatten_scene_sync` — same
//! "compose parent transform into each descendant, snapshot the resolved presentation" shape, now
//! expressed as a real `InferredField<P>` dependency chain instead of a process-local
//! content-addressed cache over a parallel non-artifact store.
//!
//! Every scene-graph entity (a `Group`, or one of its descendants) is one `InferredField` key,
//! addressed the same structural way every mutation triad in this facet already addresses nodes
//! (`NodePath`, since `DrawNode` carries no stable id) — encoded here as `"<layer>:<p0>.<p1>..."`
//! so it satisfies `InferredField::Key`'s `Ord`/`Hash`/(De)Serialize bounds. A `Group` entity's
//! parent is its enclosing `Group` (or none, for a layer root); its `Value.world_transform`
//! composes the parent's already-computed world transform with its own local `transform` (TRS
//! order: scale then rotate then translate, matching `engine::geometry::SemioTransform`'s own
//! field order). `Path`/`Text` inherit their parent's world transform unchanged (neither carries a
//! transform of its own) and additionally resolve their `style: Option<String>` reference against
//! `styles` — included in `dep_input` so a `change-stroke-color`/`replace-fill`/… on the
//! REFERENCED style (which never touches this entity's own node fields) still correctly
//! invalidates its cached value. `Image` inherits the world transform and has no style field.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioQuaternion, SemioTransform};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{node_at, NodePath};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, DrawStyle, SemioDrawingSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Value
/// 🖼️ One entity's resolved presentation — world transform + (for `Path`/`Text`) the fully
/// resolved style value (not merely its name).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlattenedNode {
    pub world_transform: SemioTransform,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_style: Option<DrawStyle>,
}
//#endregion 🔖️Value

//#region 🔖️KeyCodec
/// 🔑️ `"<layer>:<p0>.<p1>..."` — the same structural substitute for a stable node id every
/// mutation triad in this facet already uses (`NodePath`), reformatted as one `Ord`/`Hash`-able
/// `String` (`InferredField::Key`'s bound).
pub(crate) fn key_for(layer: usize, path: &[usize]) -> String {
    format!("{layer}:{}", path.iter().map(|i| i.to_string()).collect::<Vec<_>>().join("."))
}

pub(crate) fn node_path_from_key(key: &str) -> NodePath {
    let (layer_str, path_str) = key.split_once(':').expect("flattened-scene key always contains ':'");
    let layer = layer_str.parse().expect("flattened-scene key layer segment always parses as usize");
    let path = if path_str.is_empty() { Vec::new() } else { path_str.split('.').map(|s| s.parse().expect("flattened-scene key path segment always parses as usize")).collect() };
    NodePath { layer, path }
}
//#endregion 🔖️KeyCodec

//#region 🔖️Transform
/// 🧮️ TRS composition of a child's LOCAL transform into its parent's already-composed WORLD
/// transform — standard scene-graph rule: `world.translation = parent.translation +
/// rotate(parent.rotation, child.translation * parent.scale)`, `world.rotation =
/// normalize(parent.rotation * child.rotation)`, `world.scale = parent.scale * child.scale`
/// (component-wise).
pub(crate) fn compose_transform(parent: SemioTransform, child: SemioTransform) -> SemioTransform {
    let scaled_child_translation = SemioPoint3 { x: child.translation.x * parent.scale.x, y: child.translation.y * parent.scale.y, z: child.translation.z * parent.scale.z };
    let rotated = rotate_point(parent.rotation, scaled_child_translation);
    SemioTransform {
        translation: SemioPoint3 { x: parent.translation.x + rotated.x, y: parent.translation.y + rotated.y, z: parent.translation.z + rotated.z },
        rotation: normalize_quaternion(multiply_quaternion(parent.rotation, child.rotation)),
        scale: SemioPoint3 { x: parent.scale.x * child.scale.x, y: parent.scale.y * child.scale.y, z: parent.scale.z * child.scale.z },
    }
}

fn multiply_quaternion(a: SemioQuaternion, b: SemioQuaternion) -> SemioQuaternion {
    SemioQuaternion {
        w: a.w * b.w - a.x * b.x - a.y * b.y - a.z * b.z,
        x: a.w * b.x + a.x * b.w + a.y * b.z - a.z * b.y,
        y: a.w * b.y - a.x * b.z + a.y * b.w + a.z * b.x,
        z: a.w * b.z + a.x * b.y - a.y * b.x + a.z * b.w,
    }
}

fn normalize_quaternion(q: SemioQuaternion) -> SemioQuaternion {
    let len = (q.x * q.x + q.y * q.y + q.z * q.z + q.w * q.w).sqrt();
    if len == 0.0 {
        return SemioQuaternion::default();
    }
    SemioQuaternion { x: q.x / len, y: q.y / len, z: q.z / len, w: q.w / len }
}

/// ↻️ Rotates `v` by unit quaternion `q` — `v + 2*q.xyz × (q.xyz × v + q.w*v)` (the standard
/// avoid-quaternion-inverse rotation formula).
fn rotate_point(q: SemioQuaternion, v: SemioPoint3) -> SemioPoint3 {
    let qv = (q.x, q.y, q.z);
    let cross1 = cross(qv, (v.x, v.y, v.z));
    let t = (cross1.0 + q.w * v.x, cross1.1 + q.w * v.y, cross1.2 + q.w * v.z);
    let cross2 = cross(qv, t);
    SemioPoint3 { x: v.x + 2.0 * cross2.0, y: v.y + 2.0 * cross2.1, z: v.z + 2.0 * cross2.2 }
}

fn cross(a: (f64, f64, f64), b: (f64, f64, f64)) -> (f64, f64, f64) {
    (a.1 * b.2 - a.2 * b.1, a.2 * b.0 - a.0 * b.2, a.0 * b.1 - a.1 * b.0)
}
//#endregion 🔖️Transform

//#region 🔖️StyleResolution
fn resolve_style(snapshot: &SemioDrawingSnapshot, style_ref: &Option<String>) -> Option<DrawStyle> {
    let name = style_ref.as_ref()?;
    snapshot.styles.iter().find(|s| &s.name == name).cloned()
}

fn push_number(bytes: &mut Vec<u8>, v: f64) {
    bytes.extend_from_slice(&v.to_le_bytes());
}

fn push_style_dep(bytes: &mut Vec<u8>, snapshot: &SemioDrawingSnapshot, style_ref: &Option<String>) {
    match style_ref {
        None => bytes.push(0),
        Some(name) => {
            bytes.push(1);
            bytes.extend_from_slice(name.as_bytes());
            bytes.push(0x1f);
            if let Some(style) = snapshot.styles.iter().find(|s| &s.name == name) {
                bytes.push(1);
                if let Some(fill) = style.fill {
                    bytes.push(1);
                    push_number(bytes, fill.r as f64);
                    push_number(bytes, fill.g as f64);
                    push_number(bytes, fill.b as f64);
                    push_number(bytes, fill.a as f64);
                } else {
                    bytes.push(0);
                }
                if let Some(stroke) = style.stroke {
                    bytes.push(1);
                    push_number(bytes, stroke.r as f64);
                    push_number(bytes, stroke.g as f64);
                    push_number(bytes, stroke.b as f64);
                    push_number(bytes, stroke.a as f64);
                } else {
                    bytes.push(0);
                }
                if let Some(w) = style.stroke_width {
                    bytes.push(1);
                    push_number(bytes, w);
                } else {
                    bytes.push(0);
                }
                if let Some(o) = style.opacity {
                    bytes.push(1);
                    push_number(bytes, o as f64);
                } else {
                    bytes.push(0);
                }
            } else {
                bytes.push(0);
            }
        }
    }
}
//#endregion 🔖️StyleResolution

//#region 🔖️InferredField
pub struct DrawFlattenedScene;

impl store::InferredField<SemioDrawingSnapshot> for DrawFlattenedScene {
    type Key = String;
    type Value = FlattenedNode;
    const FIELD_ID: &'static str = "s.stdio.semio.drawing.inference.flattenedScene";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["layers", "styles"]
    }

    fn plan(snapshot: &SemioDrawingSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        let mut steps = Vec::new();
        for (layer_idx, layer) in snapshot.layers.iter().enumerate() {
            walk(&layer.root, layer_idx, &mut Vec::new(), None, &mut steps);
        }
        steps
    }

    fn dep_input(snapshot: &SemioDrawingSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        let np = node_path_from_key(key);
        let mut bytes = Vec::new();
        match node_at(snapshot, &np) {
            Some(DrawNode::Group { transform, .. }) => {
                push_number(&mut bytes, transform.translation.x);
                push_number(&mut bytes, transform.translation.y);
                push_number(&mut bytes, transform.translation.z);
                push_number(&mut bytes, transform.rotation.x);
                push_number(&mut bytes, transform.rotation.y);
                push_number(&mut bytes, transform.rotation.z);
                push_number(&mut bytes, transform.rotation.w);
                push_number(&mut bytes, transform.scale.x);
                push_number(&mut bytes, transform.scale.y);
                push_number(&mut bytes, transform.scale.z);
            }
            Some(DrawNode::Path { style, .. }) | Some(DrawNode::Text { style, .. }) => push_style_dep(&mut bytes, snapshot, style),
            Some(DrawNode::Image { .. }) | None => {}
        }
        bytes
    }

    fn compute(snapshot: &SemioDrawingSnapshot, key: &Self::Key, parents: &[Self::Value]) -> Self::Value {
        let np = node_path_from_key(key);
        let parent_transform = parents.first().map(|p| p.world_transform).unwrap_or_else(SemioTransform::identity);
        match node_at(snapshot, &np) {
            Some(DrawNode::Group { transform, .. }) => FlattenedNode { world_transform: compose_transform(parent_transform, *transform), resolved_style: None },
            Some(DrawNode::Path { style, .. }) | Some(DrawNode::Text { style, .. }) => FlattenedNode { world_transform: parent_transform, resolved_style: resolve_style(snapshot, style) },
            Some(DrawNode::Image { .. }) | None => FlattenedNode { world_transform: parent_transform, resolved_style: None },
        }
    }
}

fn walk(node: &DrawNode, layer: usize, path: &mut Vec<usize>, parent_key: Option<String>, out: &mut Vec<store::InferenceStep<String>>) {
    let key = key_for(layer, path);
    out.push(store::InferenceStep { key: key.clone(), parents: parent_key.into_iter().collect() });
    if let DrawNode::Group { children, .. } = node {
        for (i, child) in children.iter().enumerate() {
            path.push(i);
            walk(child, layer, path, Some(key.clone()), out);
            path.pop();
        }
    }
}
//#endregion 🔖️InferredField

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
    use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, PathSegment, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
    use store::{InferenceCache, InferenceCacheConfig, InferredField};

    fn fixture() -> SemioDrawingSnapshot {
        SemioDrawingSnapshot {
            schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
            canvas: DrawCanvas { width: 10.0, height: 10.0, background: None },
            styles: vec![DrawStyle { name: "s1".into(), fill: Some(crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }), stroke: None, stroke_width: Some(2.0), opacity: None }],
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

    #[test]
    fn world_transform_composes_down_through_nested_groups() {
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

    #[test]
    fn style_reference_resolves_to_the_real_value() {
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
    #[test]
    fn changing_a_leaf_own_style_does_not_recompute_ancestors_or_siblings() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let base = fixture();
        let _ = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(&base, Some(&mut cache));

        let mut changed = base.clone();
        changed.styles[0].stroke_width = Some(99.0);
        let before = cache.stats();
        let values = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(&changed, Some(&mut cache));
        let after = cache.stats();

        assert_eq!(after.misses - before.misses, 1, "only the leaf referencing the changed style may miss");
        assert_eq!(after.hits - before.hits, 3, "root + the unrelated nested Group + the unrelated nested Text (its sibling subtree) must all remain cache hits");
        assert_eq!(values[&key_for(0, &[0])].resolved_style.as_ref().unwrap().stroke_width, Some(99.0));
    }

    /// 🌳️ Ancestor law: changing the ROOT's own transform must miss for every entity in the plan
    /// (root + every descendant transitively folds root's `DepHash` into its own chain).
    #[test]
    fn changing_the_root_transform_recomputes_the_whole_subtree() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let base = fixture();
        let _ = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(&base, Some(&mut cache));

        let mut changed = base.clone();
        let DrawNode::Group { transform, .. } = &mut changed.layers[0].root else { panic!() };
        transform.translation.x = 999.0;
        let before = cache.stats();
        let _ = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(&changed, Some(&mut cache));
        let after = cache.stats();

        assert_eq!(after.misses - before.misses, 4, "root + its 3 descendants all depend on the root's world transform");
        assert_eq!(after.hits - before.hits, 0, "a root-wide change leaves nothing warm");
    }

    /// 🤝️ Sibling law: two INDEPENDENT leaves (each referencing its own style, under different
    /// parent Groups so neither is an ancestor of the other) — editing one's referenced style must
    /// leave the other's entire subtree, and everything between it and the shared root, warm.
    fn two_independent_styled_siblings() -> SemioDrawingSnapshot {
        SemioDrawingSnapshot {
            schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
            canvas: DrawCanvas { width: 10.0, height: 10.0, background: None },
            styles: vec![
                DrawStyle { name: "s1".into(), fill: Some(crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }), stroke: None, stroke_width: Some(2.0), opacity: None },
                DrawStyle { name: "s2".into(), fill: Some(crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioRgba { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }), stroke: None, stroke_width: Some(3.0), opacity: None },
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

    #[test]
    fn an_unrelated_sibling_edit_leaves_the_other_siblings_chain_warm() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let base = two_independent_styled_siblings();
        let baseline = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(&base, Some(&mut cache));
        let sibling_before = baseline[&key_for(0, &[1])].clone();

        let mut changed = base.clone();
        changed.styles[0].stroke_width = Some(42.0); // only sibling 0 (style "s1") is affected
        let before = cache.stats();
        let values = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(&changed, Some(&mut cache));
        let after = cache.stats();

        assert_eq!(after.misses - before.misses, 1, "only sibling 0 (which references the changed style) may miss");
        assert_eq!(after.hits - before.hits, 2, "root + sibling 1 (which references a different, untouched style) must remain cache hits");
        assert_eq!(values[&key_for(0, &[1])], sibling_before, "sibling 1's flattened value must be byte-identical — it never depended on sibling 0's style");
    }
    //#endregion 🧪️IncrementalityLaw

    #[test]
    fn disabled_cache_matches_pure_recompute() {
        let snapshot = fixture();
        let pure = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(&snapshot, None);
        let mut disabled = InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() });
        let via_disabled = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(&snapshot, Some(&mut disabled));
        assert_eq!(pure, via_disabled);
    }

    #[test]
    fn quaternion_rotation_of_identity_is_a_no_op() {
        let p = SemioPoint3 { x: 3.0, y: 4.0, z: 5.0 };
        assert_eq!(rotate_point(SemioQuaternion::default(), p), p);
    }
}
//#endregion 🧪️Tests
