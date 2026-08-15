//! 🧬️ Closed GLTF mutation command union and command-leaf dispatch.

use crate::artifacts::gltf::schema::diff::GltfDiff;
#[cfg(test)]
use crate::artifacts::gltf::schema::snapshot::GltfScene;
use crate::artifacts::gltf::schema::snapshot::{GltfAccessor, GltfAnimation, GltfAsset, GltfBuffer, GltfMaterial, GltfMesh, GltfNode};
use crate::artifacts::gltf::GltfSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

pub use super::bind_node_mesh::mutation::BindNodeMesh;
pub use super::bind_primitive_material::mutation::BindPrimitiveMaterial;
pub use super::insert_accessor::mutation::InsertAccessor;
pub use super::insert_animation::mutation::InsertAnimation;
pub use super::insert_buffer::mutation::InsertBuffer;
pub use super::insert_material::mutation::InsertMaterial;
pub use super::insert_mesh::mutation::InsertMesh;
pub use super::insert_node::mutation::InsertNode;
pub use super::insert_scene::mutation::InsertScene;
pub use super::no_mutation::mutation::NoMutation;
pub use super::remove_accessor::mutation::RemoveAccessor;
pub use super::remove_animation::mutation::RemoveAnimation;
pub use super::remove_buffer::mutation::RemoveBuffer;
pub use super::remove_material::mutation::RemoveMaterial;
pub use super::remove_mesh::mutation::RemoveMesh;
pub use super::remove_node::mutation::RemoveNode;
pub use super::remove_scene::mutation::RemoveScene;
pub use super::reparent_node::mutation::ReparentNode;
pub use super::set_accessor::mutation::SetAccessor;
pub use super::set_animation::mutation::SetAnimation;
pub use super::set_asset::mutation::SetAsset;
pub use super::set_buffer::mutation::SetBuffer;
pub use super::set_material::mutation::SetMaterial;
pub use super::set_mesh::mutation::SetMesh;
pub use super::set_node::mutation::SetNode;
pub use super::set_scene::mutation::SetScene;
pub use super::set_snapshot::mutation::SetSnapshot;
pub use super::transform_node::mutation::TransformNode;

pub(crate) use super::planning::locate_node_owner;
#[cfg(test)]
pub(crate) use super::planning::semantic_snapshot;
pub use super::planning::{apply_gltf_mutation, plan_gltf_mutation, validate_gltf_references, GltfMutationRejection};

//#region 🔖️Mutations
/// 📐️ Closed semantic command union for `stdio.gltf`; declaration order follows frozen tags.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum GltfMutation {
    NoMutation(NoMutation),
    SetSnapshot(SetSnapshot),
    SetAsset(SetAsset),
    InsertScene(InsertScene),
    RemoveScene(RemoveScene),
    SetScene(SetScene),
    InsertNode(InsertNode),
    RemoveNode(RemoveNode),
    SetNode(SetNode),
    InsertMesh(InsertMesh),
    RemoveMesh(RemoveMesh),
    SetMesh(SetMesh),
    InsertAccessor(InsertAccessor),
    RemoveAccessor(RemoveAccessor),
    SetAccessor(SetAccessor),
    InsertMaterial(InsertMaterial),
    RemoveMaterial(RemoveMaterial),
    SetMaterial(SetMaterial),
    InsertBuffer(InsertBuffer),
    RemoveBuffer(RemoveBuffer),
    SetBuffer(SetBuffer),
    InsertAnimation(InsertAnimation),
    RemoveAnimation(RemoveAnimation),
    SetAnimation(SetAnimation),
    TransformNode(TransformNode),
    ReparentNode(ReparentNode),
    BindNodeMesh(BindNodeMesh),
    BindPrimitiveMaterial(BindPrimitiveMaterial),
}

impl Default for GltfMutation {
    fn default() -> Self {
        Self::NoMutation(NoMutation {})
    }
}
//#endregion 🔖️Mutations

//#region 🔖️MutationTrait
impl Mutation<GltfSnapshot> for GltfMutation {
    type Diff = GltfDiff;

    fn diff(&self, base: &GltfSnapshot) -> Self::Diff {
        match self {
            GltfMutation::NoMutation(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::SetSnapshot(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::SetAsset(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::InsertScene(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::RemoveScene(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::SetScene(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::InsertNode(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::RemoveNode(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::SetNode(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::TransformNode(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::ReparentNode(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::BindNodeMesh(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::InsertMesh(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::RemoveMesh(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::SetMesh(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::InsertAccessor(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::RemoveAccessor(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::SetAccessor(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::InsertMaterial(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::RemoveMaterial(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::SetMaterial(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::BindPrimitiveMaterial(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::InsertBuffer(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::RemoveBuffer(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::SetBuffer(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::InsertAnimation(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::RemoveAnimation(payload) => protocol::MutationKind::diff(payload, base),
            GltfMutation::SetAnimation(payload) => protocol::MutationKind::diff(payload, base),
        }
    }

    fn inverse(&self, base: &GltfSnapshot) -> Vec<Self> {
        if plan_gltf_mutation(base, self).is_err() {
            return Vec::new();
        }
        match self {
            GltfMutation::NoMutation(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::SetSnapshot(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::SetAsset(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::InsertScene(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::RemoveScene(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::SetScene(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::InsertNode(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::RemoveNode(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::SetNode(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::TransformNode(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::ReparentNode(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::BindNodeMesh(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::InsertMesh(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::RemoveMesh(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::SetMesh(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::InsertAccessor(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::RemoveAccessor(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::SetAccessor(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::InsertMaterial(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::RemoveMaterial(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::SetMaterial(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::BindPrimitiveMaterial(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::InsertBuffer(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::RemoveBuffer(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::SetBuffer(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::InsertAnimation(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::RemoveAnimation(payload) => protocol::MutationKind::inverse(payload, base),
            GltfMutation::SetAnimation(payload) => protocol::MutationKind::inverse(payload, base),
        }
    }

    fn validate(&self, snapshot: &GltfSnapshot) -> Result<(), String> {
        plan_gltf_mutation(snapshot, self).map(|_| ()).map_err(|error| error.to_string())
    }
}
//#endregion 🔖️MutationTrait

//#region 🧪️DemoCases
/// 🧪️ P2-FG3: representative `GltfMutation` cases — one per variant (28 total, `NoMutation`
/// through `SetAnimation`, `GltfMutation`'s own declaration order) — used by this artifact's own
/// `ops_grammar_conformance_law`/`protocol_walk_law` conformance tests (⚙️engine/component.rs),
/// mirroring json's own `demo_mutation_cases()` role in its pilot report.
pub(crate) fn demo_mutation_cases() -> Vec<GltfMutation> {
    vec![
        GltfMutation::NoMutation(NoMutation {}),
        GltfMutation::SetSnapshot(SetSnapshot { snapshot: crate::artifacts::gltf::engine::demo_gltf_snapshot() }),
        GltfMutation::SetAsset(SetAsset { asset: GltfAsset { version: "2.1".into(), generator: None, copyright: Some("(c)".into()), min_version: None, extensions: None, extras: None } }),
        GltfMutation::InsertScene(InsertScene { index: 1, scene: crate::artifacts::gltf::schema::snapshot::GltfScene { nodes: vec![1], name: Some("s".into()), ..Default::default() } }),
        GltfMutation::RemoveScene(RemoveScene { index: 0 }),
        GltfMutation::SetScene(SetScene { index: 0, scene: crate::artifacts::gltf::schema::snapshot::GltfScene { nodes: vec![9], name: None, ..Default::default() } }),
        GltfMutation::InsertNode(InsertNode { index: 1, node: GltfNode { mesh: Some(1), matrix: Some([0.0; 16]), ..GltfNode::default() } }),
        GltfMutation::RemoveNode(RemoveNode { index: 0 }),
        GltfMutation::SetNode(SetNode { index: 0, node: GltfNode { mesh: None, camera: Some(2), name: Some("n".into()), ..GltfNode::default() } }),
        GltfMutation::TransformNode(TransformNode { index: 0, matrix: None, translation: Some([1.0, 2.0, 3.0]), rotation: Some([0.0, 0.0, 0.0, 1.0]), scale: Some([2.0, 2.0, 2.0]) }),
        GltfMutation::ReparentNode(ReparentNode { index: 1, parent: Some(0), scene: None, position: 0 }),
        GltfMutation::BindNodeMesh(BindNodeMesh { index: 0, mesh: Some(1) }),
        GltfMutation::InsertMesh(InsertMesh { index: 0, mesh: GltfMesh { name: Some("m".into()), ..GltfMesh::default() } }),
        GltfMutation::RemoveMesh(RemoveMesh { index: 0 }),
        GltfMutation::SetMesh(SetMesh { index: 0, mesh: GltfMesh { name: Some("renamed-mesh".into()), ..GltfMesh::default() } }),
        GltfMutation::InsertAccessor(InsertAccessor {
            index: 0,
            accessor: GltfAccessor {
                buffer_view: None,
                byte_offset: 0,
                component_type: crate::artifacts::gltf::engine::GltfComponentType::UnsignedByte,
                normalized: false,
                count: 1,
                kind: crate::artifacts::gltf::engine::GltfAccessorType::Scalar,
                max: None,
                min: None,
                sparse: None,
                name: None,
                extensions: None,
                extras: None,
            },
        }),
        GltfMutation::RemoveAccessor(RemoveAccessor { index: 0 }),
        GltfMutation::SetAccessor(SetAccessor {
            index: 0,
            accessor: GltfAccessor {
                buffer_view: Some(0),
                byte_offset: 4,
                component_type: crate::artifacts::gltf::engine::GltfComponentType::Float,
                normalized: true,
                count: 9,
                kind: crate::artifacts::gltf::engine::GltfAccessorType::Vec3,
                max: Some(vec![1.0]),
                min: Some(vec![-1.0]),
                sparse: None,
                name: None,
                extensions: None,
                extras: None,
            },
        }),
        GltfMutation::InsertMaterial(InsertMaterial { index: 0, material: GltfMaterial { name: Some("mat".into()), double_sided: true, ..GltfMaterial::default() } }),
        GltfMutation::RemoveMaterial(RemoveMaterial { index: 0 }),
        GltfMutation::SetMaterial(SetMaterial { index: 0, material: GltfMaterial { double_sided: true, ..GltfMaterial::default() } }),
        GltfMutation::BindPrimitiveMaterial(BindPrimitiveMaterial { mesh: 0, primitive: 0, material: Some(0) }),
        GltfMutation::InsertBuffer(InsertBuffer { index: 0, buffer: GltfBuffer { byte_length: 2, uri: Some("data:...".into()), name: None, extensions: None, extras: None }, bytes: vec![7, 8] }),
        GltfMutation::RemoveBuffer(RemoveBuffer { index: 0 }),
        GltfMutation::SetBuffer(SetBuffer { index: 0, buffer: GltfBuffer { byte_length: 8, uri: None, name: None, extensions: None, extras: None }, bytes: vec![1, 2, 3, 4, 5, 6, 7, 8] }),
        GltfMutation::InsertAnimation(InsertAnimation { index: 0, animation: GltfAnimation { name: Some("a".into()), ..GltfAnimation::default() } }),
        GltfMutation::RemoveAnimation(RemoveAnimation { index: 0 }),
        GltfMutation::SetAnimation(SetAnimation { index: 0, animation: GltfAnimation { name: Some("renamed-anim".into()), ..GltfAnimation::default() } }),
    ]
}

//#endregion 🧪️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gltf::schema::snapshot::GltfDocument;
    use crate::artifacts::gltf::STDIO_GLTF_DOCUMENT_SCHEMA;
    use protocol::MutationDiff;
    use protocol::{OpBinary, OpText};

    fn base_snapshot() -> GltfSnapshot {
        GltfSnapshot {
            schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
            document: GltfDocument {
                asset: GltfAsset { version: "2.0".into(), ..GltfAsset::default() },
                scenes: vec![GltfScene { nodes: vec![0], name: Some("s0".into()), ..GltfScene::default() }],
                nodes: vec![GltfNode { mesh: Some(0), ..GltfNode::default() }, GltfNode::default()],
                meshes: vec![GltfMesh { primitives: vec![Default::default()], ..Default::default() }, GltfMesh::default()],
                accessors: vec![GltfAccessor {
                    buffer_view: None,
                    byte_offset: 0,
                    component_type: crate::artifacts::gltf::engine::GltfComponentType::Float,
                    normalized: false,
                    count: 3,
                    kind: crate::artifacts::gltf::engine::GltfAccessorType::Vec3,
                    max: None,
                    min: None,
                    sparse: None,
                    name: None,
                    extensions: None,
                    extras: None,
                }],
                materials: vec![GltfMaterial::default()],
                buffers: vec![GltfBuffer { byte_length: 4, uri: None, name: None, extensions: None, extras: None }],
                animations: vec![GltfAnimation::default()],
                ..GltfDocument::default()
            },
            buffers: vec![vec![1, 2, 3, 4]],
            source_form: crate::artifacts::gltf::schema::snapshot::GltfSourceForm::Json,
        }
    }

    /// 🧪️ `mutation_diff_law`: ∀ variant, `m.diff(base).apply(base) == { apply_gltf_mutation(&mut
    /// s, m); s }`, and the returned diff equals `m.diff(base)`.
    #[test]
    fn mutation_diff_law_holds_for_every_variant() {
        let base = base_snapshot();
        let variants = vec![
            GltfMutation::NoMutation(NoMutation {}),
            GltfMutation::SetAsset(SetAsset { asset: GltfAsset { version: "2.1".into(), ..GltfAsset::default() } }),
            GltfMutation::InsertScene(InsertScene { index: 1, scene: GltfScene { nodes: vec![1], ..GltfScene::default() } }),
            GltfMutation::RemoveScene(RemoveScene { index: 0 }),
            GltfMutation::SetScene(SetScene { index: 0, scene: GltfScene { nodes: vec![1], name: Some("renamed".into()), ..GltfScene::default() } }),
            GltfMutation::InsertNode(InsertNode { index: 1, node: GltfNode { mesh: Some(1), ..GltfNode::default() } }),
            GltfMutation::RemoveNode(RemoveNode { index: 1 }),
            GltfMutation::SetNode(SetNode { index: 0, node: GltfNode { mesh: None, name: Some("n".into()), ..GltfNode::default() } }),
            GltfMutation::TransformNode(TransformNode { index: 1, matrix: None, translation: Some([1.0, 2.0, 3.0]), rotation: None, scale: None }),
            GltfMutation::ReparentNode(ReparentNode { index: 1, parent: Some(0), scene: None, position: 0 }),
            GltfMutation::BindNodeMesh(BindNodeMesh { index: 1, mesh: Some(0) }),
            GltfMutation::InsertMesh(InsertMesh { index: 0, mesh: GltfMesh { name: Some("m".into()), ..GltfMesh::default() } }),
            GltfMutation::RemoveMesh(RemoveMesh { index: 1 }),
            GltfMutation::SetMesh(SetMesh { index: 0, mesh: GltfMesh { name: Some("renamed-mesh".into()), ..GltfMesh::default() } }),
            GltfMutation::InsertAccessor(InsertAccessor {
                index: 0,
                accessor: GltfAccessor {
                    buffer_view: None,
                    byte_offset: 0,
                    component_type: crate::artifacts::gltf::engine::GltfComponentType::UnsignedByte,
                    normalized: false,
                    count: 1,
                    kind: crate::artifacts::gltf::engine::GltfAccessorType::Scalar,
                    max: None,
                    min: None,
                    sparse: None,
                    name: None,
                    extensions: None,
                    extras: None,
                },
            }),
            GltfMutation::RemoveAccessor(RemoveAccessor { index: 0 }),
            GltfMutation::SetAccessor(SetAccessor {
                index: 0,
                accessor: GltfAccessor {
                    buffer_view: None,
                    byte_offset: 0,
                    component_type: crate::artifacts::gltf::engine::GltfComponentType::Float,
                    normalized: false,
                    count: 9,
                    kind: crate::artifacts::gltf::engine::GltfAccessorType::Vec3,
                    max: None,
                    min: None,
                    sparse: None,
                    name: None,
                    extensions: None,
                    extras: None,
                },
            }),
            GltfMutation::InsertMaterial(InsertMaterial { index: 0, material: GltfMaterial { name: Some("mat".into()), ..GltfMaterial::default() } }),
            GltfMutation::RemoveMaterial(RemoveMaterial { index: 0 }),
            GltfMutation::SetMaterial(SetMaterial { index: 0, material: GltfMaterial { double_sided: true, ..GltfMaterial::default() } }),
            GltfMutation::BindPrimitiveMaterial(BindPrimitiveMaterial { mesh: 0, primitive: 0, material: Some(0) }),
            GltfMutation::InsertBuffer(InsertBuffer { index: 0, buffer: GltfBuffer { byte_length: 2, uri: None, name: None, extensions: None, extras: None }, bytes: vec![7, 8] }),
            GltfMutation::RemoveBuffer(RemoveBuffer { index: 0 }),
            GltfMutation::SetBuffer(SetBuffer { index: 0, buffer: GltfBuffer { byte_length: 8, uri: None, name: None, extensions: None, extras: None }, bytes: vec![1, 2, 3, 4, 5, 6, 7, 8] }),
            GltfMutation::InsertAnimation(InsertAnimation { index: 0, animation: GltfAnimation { name: Some("a".into()), ..GltfAnimation::default() } }),
            GltfMutation::RemoveAnimation(RemoveAnimation { index: 0 }),
            GltfMutation::SetAnimation(SetAnimation { index: 0, animation: GltfAnimation { name: Some("renamed-anim".into()), ..GltfAnimation::default() } }),
        ];
        for m in variants {
            let expected_diff = m.diff(&base);
            let mut s = base.clone();
            let actual_diff = apply_gltf_mutation(&mut s, &m).expect("valid mutation");
            assert_eq!(actual_diff, expected_diff, "diff mismatch for mutation {m:?}");
            assert_eq!(s, MutationDiff::apply(&expected_diff, &base), "apply(base) mismatch for mutation {m:?}");
        }
    }

    /// 🧪️ `inverse_law` (mutation level): every variant's `inverse(base)` round-trips.
    #[test]
    fn inverse_law_mutation_level_round_trips_for_every_variant() {
        let base = base_snapshot();
        let variants = vec![
            GltfMutation::SetAsset(SetAsset { asset: GltfAsset { version: "9.9".into(), ..GltfAsset::default() } }),
            GltfMutation::InsertScene(InsertScene { index: 0, scene: GltfScene { nodes: vec![1], ..GltfScene::default() } }),
            GltfMutation::RemoveScene(RemoveScene { index: 0 }),
            GltfMutation::SetScene(SetScene { index: 0, scene: GltfScene { nodes: vec![1], name: Some("z".into()), ..GltfScene::default() } }),
            GltfMutation::InsertNode(InsertNode { index: 0, node: GltfNode { mesh: Some(0), ..GltfNode::default() } }),
            GltfMutation::RemoveNode(RemoveNode { index: 1 }),
            GltfMutation::SetNode(SetNode { index: 1, node: GltfNode { mesh: None, ..GltfNode::default() } }),
            GltfMutation::TransformNode(TransformNode { index: 1, matrix: None, translation: Some([4.0, 5.0, 6.0]), rotation: None, scale: None }),
            GltfMutation::ReparentNode(ReparentNode { index: 1, parent: Some(0), scene: None, position: 0 }),
            GltfMutation::BindNodeMesh(BindNodeMesh { index: 1, mesh: Some(0) }),
            GltfMutation::InsertMesh(InsertMesh { index: 0, mesh: GltfMesh::default() }),
            GltfMutation::RemoveMesh(RemoveMesh { index: 1 }),
            GltfMutation::InsertMaterial(InsertMaterial { index: 0, material: GltfMaterial::default() }),
            GltfMutation::RemoveMaterial(RemoveMaterial { index: 0 }),
            GltfMutation::BindPrimitiveMaterial(BindPrimitiveMaterial { mesh: 0, primitive: 0, material: Some(0) }),
            GltfMutation::InsertBuffer(InsertBuffer { index: 0, buffer: GltfBuffer { byte_length: 1, uri: None, name: None, extensions: None, extras: None }, bytes: vec![1] }),
            GltfMutation::RemoveBuffer(RemoveBuffer { index: 0 }),
            GltfMutation::InsertAnimation(InsertAnimation { index: 0, animation: GltfAnimation::default() }),
            GltfMutation::RemoveAnimation(RemoveAnimation { index: 0 }),
        ];
        for m in variants {
            let (_, forward_diff) = {
                let mut s = base.clone();
                let d = apply_gltf_mutation(&mut s, &m).expect("valid mutation");
                (s, d)
            };
            let mutated = MutationDiff::apply(&forward_diff, &base);
            let inverses = <GltfMutation as Mutation<GltfSnapshot>>::inverse(&m, &base);
            let mut back = mutated.clone();
            for inv in &inverses {
                let d = apply_gltf_mutation(&mut back, inv).expect("valid inverse");
                let _ = d;
            }
            assert_eq!(back, base, "inverse of {m:?} did not restore base");
        }
    }

    #[test]
    fn structural_insert_transports_references_and_inverse_restores_exactly() {
        let base = base_snapshot();
        let mutation = GltfMutation::InsertNode(InsertNode { index: 0, node: GltfNode::default() });
        let diff = plan_gltf_mutation(&base, &mutation).expect("valid insertion");
        let next = diff.apply(&base);
        assert_eq!(next.document.scenes[0].nodes, vec![1]);
        assert_eq!(next.document.nodes[1].mesh, Some(0));
        let inverse = mutation.inverse(&base);
        let mut restored = next;
        for operation in inverse {
            apply_gltf_mutation(&mut restored, &operation).expect("valid inverse");
        }
        assert_eq!(restored, base);
    }

    #[test]
    fn inserted_node_payload_uses_the_pre_insertion_index_namespace() {
        let base = base_snapshot();
        let inserted = GltfNode { children: vec![1], ..GltfNode::default() };
        let next = semantic_snapshot(&base, &GltfMutation::InsertNode(InsertNode { index: 0, node: inserted })).expect("valid node insertion");
        assert_eq!(next.document.nodes[0].children, vec![2]);
        assert_eq!(next.document.scenes[0].nodes, vec![1]);
    }

    #[test]
    fn referenced_remove_and_out_of_range_insert_are_rejected_without_effect() {
        let base = base_snapshot();
        let referenced = plan_gltf_mutation(&base, &GltfMutation::RemoveNode(RemoveNode { index: 0 })).expect_err("scene root is referenced");
        assert_eq!(referenced.code, "gltf.reference.in-use");
        assert!(referenced.detail.contains("document/scenes/0/nodes/0"));
        let out_of_range = plan_gltf_mutation(&base, &GltfMutation::InsertMesh(InsertMesh { index: 99, mesh: GltfMesh::default() })).expect_err("index must not clamp");
        assert_eq!(out_of_range.code, "gltf.mutation.insert-out-of-range");
        let mut unchanged = base.clone();
        assert!(apply_gltf_mutation(&mut unchanged, &GltfMutation::RemoveNode(RemoveNode { index: 0 })).is_err());
        assert_eq!(unchanged, base);
    }

    #[test]
    fn buffer_metadata_payload_misalignment_is_rejected() {
        let base = base_snapshot();
        let mutation = GltfMutation::SetBuffer(SetBuffer { index: 0, buffer: GltfBuffer { byte_length: 8, uri: None, name: None, extensions: None, extras: None }, bytes: vec![1, 2, 3] });
        let rejection = plan_gltf_mutation(&base, &mutation).expect_err("short payload must be rejected");
        assert_eq!(rejection.code, "gltf.buffer.byte-length");
    }

    #[test]
    fn accessor_transport_includes_morph_target_dependencies() {
        use crate::artifacts::gltf::schema::snapshot::GltfMorphTarget;
        let mut base = base_snapshot();
        let primitive = &mut base.document.meshes[0].primitives[0];
        primitive.attributes = vec![("POSITION".into(), 0)];
        primitive.indices = Some(0);
        primitive.targets = vec![GltfMorphTarget(vec![("POSITION".into(), 0)])];
        let accessor = base.document.accessors[0].clone();
        let next = semantic_snapshot(&base, &GltfMutation::InsertAccessor(InsertAccessor { index: 0, accessor })).expect("valid accessor insertion");
        let primitive = &next.document.meshes[0].primitives[0];
        assert_eq!(primitive.attributes[0].1, 1);
        assert_eq!(primitive.indices, Some(1));
        assert_eq!(primitive.targets[0].0[0].1, 1);
    }

    #[test]
    fn semantic_operations_report_stable_regions_and_round_trip() {
        use protocol::DiffRegions as _;
        let base = base_snapshot();
        let operations = [
            GltfMutation::TransformNode(TransformNode { index: 1, matrix: None, translation: Some([1.0, 2.0, 3.0]), rotation: None, scale: None }),
            GltfMutation::ReparentNode(ReparentNode { index: 1, parent: Some(0), scene: None, position: 0 }),
            GltfMutation::BindNodeMesh(BindNodeMesh { index: 1, mesh: Some(0) }),
            GltfMutation::BindPrimitiveMaterial(BindPrimitiveMaterial { mesh: 0, primitive: 0, material: Some(0) }),
        ];
        for operation in operations {
            let diff = plan_gltf_mutation(&base, &operation).expect("semantic operation");
            assert!(!diff.touches().paths.is_empty(), "missing touched paths for {operation:?}");
            let next = diff.apply(&base);
            let mut restored = next;
            for inverse in operation.inverse(&base) {
                apply_gltf_mutation(&mut restored, &inverse).expect("semantic inverse");
            }
            assert_eq!(restored, base, "inverse mismatch for {operation:?}");
        }
    }

    //#region 🔖️HandcraftedOpCodecTests
    /// 🎯️ A snapshot with `bufferViews`/`textures`/`images`/`samplers`/`skins`/`cameras` populated
    /// (`base_snapshot()` above has none of these -- they're WEAK collections only reachable via
    /// `SetSnapshot` per F4's variant vocabulary, so `SetSnapshot`'s `OpText`/`OpBinary` needs a
    /// dedicated fixture to actually exercise `enc_buffer_view`/`enc_texture`/`enc_image`/
    /// `enc_sampler`/`enc_skin`/`enc_camera` — including `GltfCameraProjection::Orthographic`, the
    /// variant `field_sweep`'s `sweep_b` (🔺️diff/component.rs) does not use).
    fn full_snapshot() -> GltfSnapshot {
        let mut s = base_snapshot();
        s.document.buffer_views = vec![crate::artifacts::gltf::schema::snapshot::GltfBufferView { buffer: 0, byte_offset: 0, byte_length: 4, byte_stride: None, target: Some(34962), name: None, extensions: None, extras: None }];
        s.document.textures = vec![crate::artifacts::gltf::schema::snapshot::GltfTexture { sampler: Some(0), source: Some(0), name: None, extensions: None, extras: None }];
        s.document.images = vec![crate::artifacts::gltf::schema::snapshot::GltfImage { uri: Some("tex.png".into()), ..Default::default() }];
        s.document.samplers = vec![crate::artifacts::gltf::schema::snapshot::GltfSampler::default()];
        s.document.skins = vec![crate::artifacts::gltf::schema::snapshot::GltfSkin { joints: vec![0, 1], ..Default::default() }];
        s.document.cameras = vec![crate::artifacts::gltf::schema::snapshot::GltfCamera {
            projection: crate::artifacts::gltf::schema::snapshot::GltfCameraProjection::Orthographic(crate::artifacts::gltf::schema::snapshot::GltfOrthographic { xmag: 1.0, ymag: 1.0, zfar: 10.0, znear: 0.1, extensions: None, extras: None }),
            name: Some("cam0".into()),
            extensions: None,
            extras: Some(crate::artifacts::gltf::schema::snapshot::GltfJson::Object(vec![("k".into(), crate::artifacts::gltf::schema::snapshot::GltfJson::Number(1.0))])),
        }];
        s.document.extensions = Some(crate::artifacts::gltf::schema::snapshot::GltfJson::Array(vec![crate::artifacts::gltf::schema::snapshot::GltfJson::Null, crate::artifacts::gltf::schema::snapshot::GltfJson::Bool(false)]));
        s
    }

    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `GltfMutation` grammar --
    /// every variant, incl. `SetSnapshot` against `full_snapshot()` (exercises every WEAK
    /// collection's item codec plus `GltfCameraProjection::Orthographic` and 4 of the 6 `GltfJson`
    /// variants at once) and a representative Insert/Remove/Set per STRONG-entity array (the same
    /// entities `diff_codec_text_binary_roundtrip_law`'s `sweep_a`/`sweep_b`/`tristate_snapshot_*`
    /// fixtures cover on the diff side, per `🔺️diff/component.rs`).
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        let mutations = vec![
            GltfMutation::NoMutation(NoMutation {}),
            GltfMutation::SetSnapshot(SetSnapshot { snapshot: full_snapshot() }),
            GltfMutation::SetAsset(SetAsset { asset: GltfAsset { version: "2.1".into(), generator: None, copyright: Some("(c)".into()), min_version: None, extensions: None, extras: None } }),
            GltfMutation::InsertScene(InsertScene { index: 1, scene: GltfScene { nodes: vec![1], name: Some("s".into()), ..GltfScene::default() } }),
            GltfMutation::RemoveScene(RemoveScene { index: 0 }),
            GltfMutation::SetScene(SetScene { index: 0, scene: GltfScene { nodes: vec![9], name: None, ..GltfScene::default() } }),
            GltfMutation::InsertNode(InsertNode { index: 1, node: GltfNode { mesh: Some(1), matrix: Some([0.0; 16]), ..GltfNode::default() } }),
            GltfMutation::RemoveNode(RemoveNode { index: 0 }),
            GltfMutation::SetNode(SetNode { index: 0, node: GltfNode { mesh: None, camera: Some(2), name: Some("n".into()), ..GltfNode::default() } }),
            GltfMutation::TransformNode(TransformNode { index: 1, matrix: None, translation: Some([1.25, -2.5, 3.75]), rotation: Some([0.0, 0.0, 0.0, 1.0]), scale: Some([1.0, 2.0, 3.0]) }),
            GltfMutation::ReparentNode(ReparentNode { index: 1, parent: Some(0), scene: None, position: 0 }),
            GltfMutation::BindNodeMesh(BindNodeMesh { index: 1, mesh: None }),
            GltfMutation::InsertMesh(InsertMesh { index: 0, mesh: GltfMesh { name: Some("m".into()), ..GltfMesh::default() } }),
            GltfMutation::RemoveMesh(RemoveMesh { index: 0 }),
            GltfMutation::SetMesh(SetMesh { index: 0, mesh: GltfMesh { name: Some("renamed-mesh".into()), ..GltfMesh::default() } }),
            GltfMutation::InsertAccessor(InsertAccessor {
                index: 0,
                accessor: GltfAccessor {
                    buffer_view: None,
                    byte_offset: 0,
                    component_type: crate::artifacts::gltf::engine::GltfComponentType::UnsignedByte,
                    normalized: false,
                    count: 1,
                    kind: crate::artifacts::gltf::engine::GltfAccessorType::Scalar,
                    max: None,
                    min: None,
                    sparse: None,
                    name: None,
                    extensions: None,
                    extras: None,
                },
            }),
            GltfMutation::RemoveAccessor(RemoveAccessor { index: 0 }),
            GltfMutation::SetAccessor(SetAccessor {
                index: 0,
                accessor: GltfAccessor {
                    buffer_view: Some(0),
                    byte_offset: 4,
                    component_type: crate::artifacts::gltf::engine::GltfComponentType::Float,
                    normalized: true,
                    count: 9,
                    kind: crate::artifacts::gltf::engine::GltfAccessorType::Vec3,
                    max: Some(vec![1.0]),
                    min: Some(vec![-1.0]),
                    sparse: None,
                    name: None,
                    extensions: None,
                    extras: None,
                },
            }),
            GltfMutation::InsertMaterial(InsertMaterial { index: 0, material: GltfMaterial { name: Some("mat".into()), double_sided: true, ..GltfMaterial::default() } }),
            GltfMutation::RemoveMaterial(RemoveMaterial { index: 0 }),
            GltfMutation::SetMaterial(SetMaterial { index: 0, material: GltfMaterial { double_sided: true, ..GltfMaterial::default() } }),
            GltfMutation::BindPrimitiveMaterial(BindPrimitiveMaterial { mesh: 0, primitive: 0, material: None }),
            GltfMutation::InsertBuffer(InsertBuffer { index: 0, buffer: GltfBuffer { byte_length: 2, uri: Some("data:...".into()), name: None, extensions: None, extras: None }, bytes: vec![7, 8] }),
            GltfMutation::RemoveBuffer(RemoveBuffer { index: 0 }),
            GltfMutation::SetBuffer(SetBuffer { index: 0, buffer: GltfBuffer { byte_length: 8, uri: None, name: None, extensions: None, extras: None }, bytes: vec![1, 2, 3, 4, 5, 6, 7, 8] }),
            GltfMutation::InsertAnimation(InsertAnimation { index: 0, animation: GltfAnimation { name: Some("a".into()), ..GltfAnimation::default() } }),
            GltfMutation::RemoveAnimation(RemoveAnimation { index: 0 }),
            GltfMutation::SetAnimation(SetAnimation { index: 0, animation: GltfAnimation { name: Some("renamed-anim".into()), ..GltfAnimation::default() } }),
        ];
        let _ = &base;
        for mutation in mutations {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = GltfMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = GltfMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }

    #[test]
    fn op_codecs_reject_unknown_text_and_trailing_binary() {
        assert!(GltfMutation::parse_op("invent-node index=0").is_err());
        let mut bytes = GltfMutation::BindNodeMesh(BindNodeMesh { index: 1, mesh: None }).encode_op().expect("encode");
        bytes.push(0xff);
        assert!(GltfMutation::decode_op(&bytes).is_err());
    }
    //#endregion 🔖️HandcraftedOpCodecTests
}
//#endregion 🧪️Tests
