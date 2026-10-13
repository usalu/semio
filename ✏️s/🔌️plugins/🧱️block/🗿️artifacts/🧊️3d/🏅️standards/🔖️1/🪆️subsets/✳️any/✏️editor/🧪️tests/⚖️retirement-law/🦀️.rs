//! ⚖️ Block3d owners retire under quoted grants: the document and its catalog lease retire exactly, every mutation passes the typed-command input lane, and a document edit authors, applies, undoes and redoes through the editor's real catalog.

#[global_allocator]
static HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[path = "../../../../../../../../../../../../🧰️framework/🔨️modules/🧪️test/⚖️typed-input-law/🦀️.rs"]
mod typed_input_law;

use semio_framework_plugin::{authoring_law::author_through_document_store, ArtifactEditor, EditorApp};
use semio_s_artifact_block_3d::editor::block3d::Block3dPlayApp;
use semio_s_artifact_block_3d::standards::v1::subsets::any::schema::mutations::{
    change_object_kind_label::change_object_kind_label, create_representation::create_representation, create_vortex::create_vortex, move_camera3d::move_camera3d, rename_object_kind::rename_object_kind, scale_camera3d::scale_camera3d,
};
use semio_s_artifact_block_3d::{Block3dMutation, Block3dSnapshot, Block3dVortexTemplate, BlockAttribute, BlockRepresentation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use std::sync::Arc;
use typed_input_law::{typed_command_retirement_law, typed_input_retirement_law};

fn boot() -> Block3dSnapshot {
    <Block3dPlayApp as ArtifactEditor>::initial_snapshot()
}

fn representation() -> BlockRepresentation {
    BlockRepresentation {
        id: "law-representation".into(),
        name: "Law representation".into(),
        mesh_url: Some("https://example.invalid/law.glb".into()),
        tags: vec!["law".into(), "exact".into()],
        lod: Some("lod0".into()),
        description: "A representation that retires under quoted grants".into(),
        attributes: vec![BlockAttribute { key: "material".into(), value: "concrete".into(), definition: None }],
    }
}

fn vortex() -> Block3dVortexTemplate {
    Block3dVortexTemplate { id: "law-vortex".into(), vortex_kind: "plug".into(), position: [1.0, 2.0, 3.0], direction: [0.0, 1.0, 0.0], radius: 0.5, label: Some("Law vortex".into()) }
}

fn mutations() -> Vec<(&'static str, Block3dMutation)> {
    vec![
        ("rename-object-kind", rename_object_kind("Renamed".into())),
        ("change-object-kind-label", change_object_kind_label("Label".into())),
        ("create-representation", create_representation(representation())),
        ("create-vortex", create_vortex(vortex())),
        ("move-camera3d", move_camera3d([1.0, 2.0, 3.0], [4.0, 5.0, 6.0])),
        ("scale-camera3d", scale_camera3d(2.0)),
    ]
}

/// 🎟️ Every mutation of the vocabulary passes the typed-command input lane and retires to its terminal-empty witness.
#[test]
fn block3d_mutations_retire_through_the_typed_command_lane() {
    for (label, mutation) in mutations() {
        let law = typed_command_retirement_law(label, mutation);
        assert!(law.turns > 0 && law.declared_births > 0, "{label}: retiring costs paid turns and births");
    }
}

/// 📚️ A document retires exactly, and its catalog child retires as a lease whether the document is the last alias of the catalog or not.
#[test]
fn block3d_snapshot_retires_with_its_catalog_lease() {
    let catalog = Arc::new(SemioKitSnapshot::default());
    let mut sole = boot();
    sole.catalog = sole.catalog.with_local_owner(catalog.clone());
    drop(catalog);
    let sole_law = typed_input_retirement_law("block3d snapshot with the last catalog alias", sole);
    let shared = Arc::new(SemioKitSnapshot::default());
    let mut aliased = boot();
    aliased.catalog = aliased.catalog.with_local_owner(shared.clone());
    let aliased_law = typed_input_retirement_law("block3d snapshot with a live catalog alias", aliased);
    assert!(Arc::strong_count(&shared) == 1, "retiring a document releases exactly its catalog alias");
    assert!(sole_law.declared_releases > aliased_law.declared_releases, "only the last alias pays the catalog release");
    let bare = typed_input_retirement_law("block3d snapshot without a catalog owner", boot());
    assert!(bare.declared_births < aliased_law.declared_births, "a catalog alias lease costs its own frame");
}

/// 📄️ A rename and a created representation author one forward row and one inverse row, apply, undo and redo through the editor's own document catalog.
#[semio_framework_async_macros::async_test]
async fn a_block3d_document_edit_authors_applies_undoes_and_redoes_through_its_real_store() {
    let base = boot();
    let law = author_through_document_store::<EditorApp<Block3dPlayApp>>(rename_object_kind("Renamed".into())).await;
    assert_eq!(law.applied.object_kind.name, "Renamed");
    assert_eq!(law.undone.object_kind.name, base.object_kind.name, "undo restores the object kind name");
    assert_eq!(law.redone.object_kind.name, "Renamed", "redo renames again");
    assert_eq!(law.forward_rows.len(), 1);
    assert_eq!(law.inverse_rows.len(), 1);
    let representation = representation();
    let law = author_through_document_store::<EditorApp<Block3dPlayApp>>(create_representation(representation.clone())).await;
    assert_eq!(law.applied.representations.last(), Some(&representation));
    assert_eq!(law.undone.representations, base.representations, "undo removes the created representation");
    assert_eq!(law.redone.representations, law.applied.representations, "redo restores the created representation");
}
