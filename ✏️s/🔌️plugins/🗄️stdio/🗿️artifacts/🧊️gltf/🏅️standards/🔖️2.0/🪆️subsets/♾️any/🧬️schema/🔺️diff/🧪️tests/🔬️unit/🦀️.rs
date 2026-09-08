
use super::*;
use crate::STDIO_GLTF_DOCUMENT_SCHEMA;

//#region 🔖️Fixtures
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn scene(seed: usize) -> GltfScene {
    GltfScene { nodes: vec![seed, seed + 1], name: Some(format!("scene{seed}")), extensions: None, extras: None }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn node(seed: usize) -> GltfNode {
    GltfNode { children: vec![seed], mesh: Some(seed), name: Some(format!("node{seed}")), ..GltfNode::default() }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn mesh(seed: usize) -> GltfMesh {
    GltfMesh { primitives: vec![GltfPrimitive { attributes: vec![("POSITION".into(), seed)], material: Some(seed), mode: Some(4), ..GltfPrimitive::default() }], name: Some(format!("mesh{seed}")), ..GltfMesh::default() }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn accessor(seed: usize) -> GltfAccessor {
    GltfAccessor {
        buffer_view: Some(seed),
        byte_offset: 0,
        component_type: GltfComponentType::Float,
        normalized: false,
        count: seed + 1,
        kind: GltfAccessorType::Vec3,
        max: Some(vec![1.0]),
        min: Some(vec![0.0]),
        sparse: None,
        name: Some(format!("acc{seed}")),
        extensions: None,
        extras: None,
    }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn material(seed: usize) -> GltfMaterial {
    GltfMaterial { name: Some(format!("mat{seed}")), double_sided: seed % 2 == 0, ..GltfMaterial::default() }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn buffer_meta(seed: usize) -> GltfBuffer {
    GltfBuffer { byte_length: seed * 4, uri: None, name: Some(format!("buf{seed}")), extensions: None, extras: None }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn animation(seed: usize) -> GltfAnimation {
    GltfAnimation { name: Some(format!("anim{seed}")), ..GltfAnimation::default() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn base_snapshot() -> GltfSnapshot {
    GltfSnapshot {
        schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
        document: GltfDocument {
            asset: GltfAsset { version: "2.0".into(), generator: Some("semio".into()), ..GltfAsset::default() },
            scene: Some(0),
            scenes: vec![scene(0), scene(1)],
            nodes: vec![node(0), node(1), node(2)],
            meshes: vec![mesh(0), mesh(1)],
            accessors: vec![accessor(0), accessor(1)],
            buffer_views: vec![],
            buffers: vec![buffer_meta(0)],
            materials: vec![material(0), material(1)],
            animations: vec![animation(0)],
            extensions_used: vec!["KHR_materials_unlit".into()],
            ..GltfDocument::default()
        },
        buffers: vec![vec![1, 2, 3, 4]],
        source_form: GltfSourceForm::Json,
    }
}
//#endregion 🔖️Fixtures

//#region 🔖️AbsorbCanonicalCases
/// 🧪️ Canonical absorb case 1: `Insert(2,x)` then `Remove(0)` → `{removed:[0], added:[(1,x)]}`.
#[test]
fn absorb_law_insert_then_remove_before_shifts_index() {
    let n = node(9);
    let mut d1 = GltfNodesDiff { added: vec![GltfAdded { index: 2, item: n.clone() }], ..Default::default() };
    let d2 = GltfNodesDiff { removed: vec![0], ..Default::default() };
    d1.absorb(d2);
    assert_eq!(d1.removed, vec![0]);
    assert_eq!(d1.added, vec![GltfAdded { index: 1, item: n }]);
    assert!(d1.modified.is_empty());
}

/// 🧪️ Canonical absorb case 2: `Insert(2,f)` then `Insert(2,g)` → BOTH survive.
#[test]
fn absorb_law_insert_insert_same_index_both_survive() {
    let f = node(1);
    let g = node(2);
    let mut d1 = GltfNodesDiff { added: vec![GltfAdded { index: 2, item: f.clone() }], ..Default::default() };
    let d2 = GltfNodesDiff { added: vec![GltfAdded { index: 2, item: g.clone() }], ..Default::default() };
    d1.absorb(d2);
    assert_eq!(d1.added, vec![GltfAdded { index: 2, item: g }, GltfAdded { index: 3, item: f }]);
}

/// 🧪️ Canonical absorb case 3: `Insert(1,f)` then `SetField(1,name)` patches INTO the added
/// payload -- merged has only `added`, no separate `modified` entry.
#[test]
fn absorb_law_insert_then_set_field_patches_into_added() {
    let f = node(1);
    let mut d1 = GltfNodesDiff { added: vec![GltfAdded { index: 1, item: f.clone() }], ..Default::default() };
    let d2 = GltfNodesDiff { modified: vec![GltfModified { index: 1, diff: GltfNodeDiff { name: Some(Some("renamed".into())), ..Default::default() } }], ..Default::default() };
    d1.absorb(d2);
    assert!(d1.modified.is_empty());
    assert_eq!(d1.added.len(), 1);
    assert_eq!(d1.added[0].item.name, Some("renamed".to_string()));
    assert_eq!(d1.added[0].index, 1);
}

/// 🧪️ Canonical absorb case 4 (id-keyed-collection-analog / modify-of-removed): `Remove(0)`
/// then `Modify(0)` (post-remove index 0 refers to a DIFFERENT surviving base item) must NOT
/// corrupt the removed item and must attach the d2 patch to the correct transported base index.
#[test]
fn absorb_law_remove_then_modify_transports_to_correct_surviving_item() {
    let mut d1 = GltfNodesDiff { removed: vec![0], ..Default::default() };
    // after d1, base[1] is now at position 0 -- d2 modifies position 0 (== base[1]).
    let d2 = GltfNodesDiff { modified: vec![GltfModified { index: 0, diff: GltfNodeDiff { name: Some(Some("x".into())), ..Default::default() } }], ..Default::default() };
    d1.absorb(d2);
    assert_eq!(d1.removed, vec![0]);
    assert_eq!(d1.modified.len(), 1);
    assert_eq!(d1.modified[0].index, 1, "d2's patch at post-remove position 0 must transport to BASE index 1");
}

#[test]
fn absorb_law_holds_over_curated_ops() {
    let base = base_snapshot();
    let mid = {
        let mut s = base.clone();
        s.document.nodes.insert(1, node(9));
        s.document.nodes.remove(0);
        s.document.materials.push(material(5));
        s
    };
    let after = {
        let mut s = mid.clone();
        s.document.nodes[0].name = Some("renamed-in-after".into());
        s.document.scenes.push(scene(7));
        s.document.materials.remove(0);
        s.buffers.push(vec![9, 9]);
        s.document.buffers.push(buffer_meta(9));
        s
    };
    let mut d1 = <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&base, &mid);
    let d2 = <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&mid, &after);
    d1.absorb(d2);
    assert_eq!(MutationDiff::apply(&d1, &base).expect("apply must succeed for a well-formed fixture"), after);
}
//#endregion 🔖️AbsorbCanonicalCases

//#region 🔖️BetweenRoundtripLaw
#[test]
fn between_roundtrip_law_holds_on_synthetic_fixture() {
    let a = base_snapshot();
    let mut b = a.clone();
    b.document.nodes.push(node(5));
    b.document.asset.generator = Some("other-tool".into());
    b.source_form = GltfSourceForm::Glb;
    let ab = <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&a, &b);
    assert_eq!(MutationDiff::apply(&ab, &a).expect("apply must succeed for a well-formed fixture"), b);
    let ba = <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&b, &a);
    assert_eq!(MutationDiff::apply(&ba, &b).expect("apply must succeed for a well-formed fixture"), a);
    assert!(<GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&a, &a).is_empty());
}
//#endregion 🔖️BetweenRoundtripLaw

//#region 🔖️InverseLaw
#[test]
fn inverse_law_diff_level_round_trips() {
    let base = base_snapshot();
    let next = {
        let mut s = base.clone();
        s.document.nodes[0].mesh = None;
        s.document.nodes.remove(1);
        s.document.nodes.push(node(8));
        s.document.extensions_used.clear();
        s.document.materials[0].alpha_mode = GltfAlphaMode::Blend;
        s
    };
    let d = <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&base, &next);
    let mutated = MutationDiff::apply(&d, &base).expect("apply must succeed for a well-formed fixture");
    let inv = <GltfDiff as DiffAlgebra<GltfSnapshot>>::inverse(&d, &base);
    assert_eq!(MutationDiff::apply(&inv, &mutated).expect("apply must succeed for a well-formed fixture"), base);
}
//#endregion 🔖️InverseLaw

//#region 🔖️FieldSweep
/// 🎯️ Shared field-sweep fixture: `sweep_a`/`sweep_b` differ in EVERY mutable field, incl.
/// every tri-state exercising `Some(None)`, with asymmetric collection lengths split across
/// both `between()` directions (F1's structural trap). Factored out of
/// `field_sweep_covers_every_mutable_field` (its original owner) so `diff_codec_text_binary_
/// roundtrip_law` (`HandcraftedDiffCodec` tests, further down) can reuse the exact same
/// comprehensive diff rather than re-deriving a second copy. `pub(super)` (not private) so the
/// sibling `handcrafted_diff_codec_tests` module can reach it via `super::tests::sweep_a()`.
pub(super) fn sweep_a() -> GltfSnapshot {
    GltfSnapshot {
        schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
        document: GltfDocument {
            asset: GltfAsset { version: "2.0".into(), generator: Some("a-tool".into()), copyright: Some("(c) a".into()), min_version: Some("2.0".into()), extensions: Some(GltfJson::Bool(true)), extras: Some(GltfJson::String("a".into())) },
            scene: Some(0),
            scenes: vec![scene(0), scene(1)],
            nodes: vec![node(0), node(1)],
            meshes: vec![mesh(0), mesh(1)],
            accessors: vec![accessor(0), accessor(1)],
            buffer_views: vec![GltfBufferView { buffer: 0, byte_offset: 0, byte_length: 4, byte_stride: None, target: None, name: None, extensions: None, extras: None }],
            buffers: vec![buffer_meta(0), buffer_meta(1)],
            materials: vec![material(0), material(1)],
            textures: vec![GltfTexture { sampler: Some(0), source: Some(0), name: None, extensions: None, extras: None }],
            images: vec![GltfImage { uri: Some("a.png".into()), ..Default::default() }],
            samplers: vec![GltfSampler::default()],
            skins: vec![GltfSkin { joints: vec![0, 1], ..Default::default() }],
            animations: vec![animation(0), animation(1)],
            cameras: vec![],
            extensions_used: vec!["KHR_a".into()],
            extensions_required: vec!["KHR_a".into()],
            extensions: Some(GltfJson::Object(vec![("KHR_a".into(), GltfJson::Object(vec![]))])),
            extras: Some(GltfJson::String("sweep-a-extras".into())),
        },
        buffers: vec![vec![1, 2], vec![3, 4]],
        source_form: GltfSourceForm::Json,
    }
}
pub(super) fn sweep_b() -> GltfSnapshot {
    GltfSnapshot {
        schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
        document: GltfDocument {
            asset: GltfAsset { version: "2.1".into(), generator: None, copyright: None, min_version: None, extensions: None, extras: None },
            scene: None,
            scenes: vec![scene(9)],
            nodes: vec![node(9), node(10), node(11)],
            meshes: vec![mesh(9)],
            accessors: vec![accessor(9)],
            buffer_views: vec![],
            buffers: vec![buffer_meta(9)],
            materials: vec![material(9), material(10), material(11)],
            textures: vec![],
            images: vec![],
            samplers: vec![],
            skins: vec![],
            animations: vec![],
            cameras: vec![GltfCamera {
                projection: GltfCameraProjection::Perspective(GltfPerspective { aspect_ratio: Some(1.5), yfov: 0.8, zfar: Some(100.0), znear: 0.1, extensions: None, extras: None }),
                name: Some("cam".into()),
                extensions: None,
                extras: None,
            }],
            extensions_used: vec![],
            extensions_required: vec![],
            extensions: None,
            extras: None,
        },
        buffers: vec![vec![9]],
        source_form: GltfSourceForm::Glb,
    }
}

/// 🧪️ Field sweep — the acceptance criterion: `sweep_a`/`sweep_b` differ in EVERY mutable
/// field, incl. every tri-state exercising `Some(None)`, with asymmetric collection lengths
/// split across both `between()` directions (F1's structural trap).
#[test]
fn field_sweep_covers_every_mutable_field() {
    let sweep_a = sweep_a();
    let sweep_b = sweep_b();

    let ab = <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&sweep_a, &sweep_b);
    assert_eq!(MutationDiff::apply(&ab, &sweep_a).expect("apply must succeed for a well-formed fixture"), sweep_b);
    assert!(ab.asset.is_some());
    assert_eq!(ab.scene, Some(None), "scene going Some->None must be tri-state Some(None)");
    assert!(ab.scenes.is_some());
    assert!(ab.nodes.is_some());
    assert!(ab.meshes.is_some());
    assert!(ab.accessors.is_some());
    assert!(ab.buffer_views.is_some());
    assert!(ab.buffers.is_some());
    assert!(ab.buffer_bytes.is_some());
    assert!(ab.materials.is_some());
    assert!(ab.textures.is_some());
    assert!(ab.images.is_some());
    assert!(ab.samplers.is_some());
    assert!(ab.skins.is_some());
    assert!(ab.animations.is_some());
    assert!(ab.cameras.is_some());
    assert!(ab.extensions_used.is_some());
    assert!(ab.extensions_required.is_some());
    assert!(ab.source_form.is_some());
    assert_eq!(ab.extensions, Some(None), "document.extensions going Some->None must be tri-state Some(None)");
    assert_eq!(ab.extras, Some(None), "document.extras going Some->None must be tri-state Some(None)");
    let nodes_ab = ab.nodes.as_ref().unwrap();
    assert!(!nodes_ab.modified.is_empty() || !nodes_ab.added.is_empty());
    assert!(!nodes_ab.added.is_empty(), "sweep must exercise an added node (b is longer)");

    let ba = <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&sweep_b, &sweep_a);
    assert_eq!(MutationDiff::apply(&ba, &sweep_b).expect("apply must succeed for a well-formed fixture"), sweep_a);
    let nodes_ba = ba.nodes.as_ref().unwrap();
    assert!(!nodes_ba.removed.is_empty(), "reverse direction must exercise a removed node (a is shorter, b is longer)");
    let cameras_ba = ba.cameras.as_ref().unwrap();
    assert!(!cameras_ba.removed.is_empty(), "reverse direction must exercise a removed camera");

    assert!(<GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&sweep_a, &sweep_a).is_empty());
}

#[test]
fn touched_regions_are_stable_precise_for_modification_and_conservative_for_transport() {
    use protocol::DiffRegions as _;
    let modified = GltfDiff {
        nodes: Some(GltfNodesDiff { modified: vec![GltfModified { index: 3, diff: GltfNodeDiff { translation: Some(Some([1.0, 2.0, 3.0])), ..Default::default() } }], ..Default::default() }),
        buffer_bytes: Some(GltfBufferBytesDiff { modified: vec![GltfModified { index: 2, diff: vec![1, 2] }], ..Default::default() }),
        ..Default::default()
    };
    assert_eq!(modified.touches().paths, vec!["buffers/2", "document/nodes/3/transform"]);
    let structural = GltfDiff { nodes: Some(GltfNodesDiff { removed: vec![1], ..Default::default() }), ..Default::default() };
    assert_eq!(structural.touches().paths, vec!["document/nodes"]);
}
//#endregion 🔖️FieldSweep
