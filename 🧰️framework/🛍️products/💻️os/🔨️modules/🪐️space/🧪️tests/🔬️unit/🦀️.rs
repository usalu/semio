
use super::*;
use protocol::DiffCodec;
use protocol::Mutation as _;
use protocol::MutationDiff as _;
use store::{ArtifactDsl, BlobStore};

//#region 🧸️Fixtures
fn demo_user(id: &str, role: SpaceRole) -> SpaceUser {
    SpaceUser { id: id.into(), name: format!("User {id}"), avatar: None, role }
}

fn demo_extension(extension_id: &str, enabled: bool) -> InstalledExtension {
    InstalledExtension { extension_id: extension_id.into(), version: "1.0.0".into(), source_uri: format!("https://example.test/{extension_id}.sxt"), package_hash: format!("hash-{extension_id}"), enabled }
}

fn demo_space() -> SpaceSnapshot {
    let mut space = empty_space_snapshot("Atelier Demo", SpaceKind::Atelier, SpaceVisibility::Private);
    space.users.push(demo_user("u1", SpaceRole::Author));
    space.collections.push(CollectionRef { id: "c1".into(), name: "Main".into(), document_id: "doc-c1".into() });
    space
}

fn demo_collection() -> CollectionSnapshot {
    let mut collection = empty_collection_snapshot("Main");
    collection.folders.push(CollectionFolder { id: "f1".into(), parent_id: None, name: "Renders".into() });
    collection.entries.push(CollectionEntry {
        id: "e1".into(),
        folder_id: Some("f1".into()),
        name: "sketch".into(),
        kind_id: "puzzle.2d".into(),
        body: Box::new(ArtifactBody::Document { schema: "test.puzzle2d".into(), document_id: "doc-e1".into() }),
    });
    collection.entries.push(CollectionEntry {
        id: "e2".into(),
        folder_id: None,
        name: "reference.png".into(),
        kind_id: "file.blob".into(),
        body: Box::new(ArtifactBody::Blob { blob: store::BlobRef { hash: "blake3-deadbeef".into(), size: 42, media_type: "image/png".into() } }),
    });
    collection
}
//#endregion 🧸️Fixtures

//#region 🧪️SpaceDocumentLaws
#[test]
fn empty_space_snapshot_matches_schema() {
    let space = empty_space_snapshot("Demo", SpaceKind::Studio, SpaceVisibility::Public);
    assert_eq!(space.schema, S_SPACE_SCHEMA);
    assert!(space.users.is_empty());
    assert!(space.collections.is_empty());
}

#[test]
fn space_snapshot_dsl_pack_round_trips() {
    store::test_support::assert_dsl_pack_equivalence(&demo_space());
}

#[test]
fn space_default_example_dsl_round_trips() {
    let text = include_str!("../../📚️examples/🪐️demo.space");
    let parsed = <SpaceSnapshot as ArtifactDsl>::parse_dsl(text).expect("parse default .space example");
    store::test_support::assert_dsl_round_trip(&parsed);
}
//#endregion 🧪️SpaceDocumentLaws

//#region 🧪️CollectionDocumentLaws
#[test]
fn empty_collection_snapshot_matches_schema() {
    let collection = empty_collection_snapshot("Demo");
    assert_eq!(collection.schema, S_COLLECTION_SCHEMA);
    assert!(collection.folders.is_empty());
    assert!(collection.entries.is_empty());
}

#[test]
fn collection_projection_dsl_pack_round_trips() {
    store::test_support::assert_dsl_pack_equivalence(&demo_collection());
}

#[test]
fn collection_envelope_id_is_two_dot_segments() {
    let id = <CollectionSnapshot as ArtifactDsl>::envelope_id();
    assert_eq!(id, "os.collection");
    assert_eq!(id.split('.').count(), 2, "SemioEnvelope::from_envelope_id requires plugin.artifact");
    let pack = store::ArtifactPack::encode_pack(&empty_collection_snapshot("test"));
    let decoded = <CollectionSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode");
    assert_eq!(decoded.name, "test");
}

#[test]
fn space_envelope_id_is_two_dot_segments() {
    let id = <SpaceSnapshot as ArtifactDsl>::envelope_id();
    assert_eq!(id, "os.space");
    assert_eq!(id.split('.').count(), 2, "SemioEnvelope::from_envelope_id requires plugin.artifact");
    let pack = store::ArtifactPack::encode_pack(&empty_space_snapshot("test", SpaceKind::Atelier, SpaceVisibility::Private));
    let decoded = <SpaceSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode");
    assert_eq!(decoded.name, "test");
}

#[test]
fn collection_default_example_dsl_round_trips() {
    let text = include_str!("../../📚️examples/🎬️demo.collection");
    let parsed = <CollectionSnapshot as ArtifactDsl>::parse_dsl(text).expect("parse default .collection example");
    store::test_support::assert_dsl_round_trip(&parsed);
}
//#endregion 🧪️CollectionDocumentLaws

//#region 🧪️SpaceMutationLaws
#[test]
fn space_operation_op_text_round_trips_every_variant() {
    store::test_support::assert_op_line_round_trip(&SpaceMutation::SetName { name: "Renamed".into() });
    store::test_support::assert_op_line_round_trip(&SpaceMutation::SetKind { kind: SpaceKind::Studio });
    store::test_support::assert_op_line_round_trip(&SpaceMutation::SetVisibility { visibility: SpaceVisibility::Public });
    store::test_support::assert_op_line_round_trip(&SpaceMutation::UpsertUser { user: demo_user("u2", SpaceRole::Spectator) });
    store::test_support::assert_op_line_round_trip(&SpaceMutation::RemoveUser { user_id: "u2".into() });
    store::test_support::assert_op_line_round_trip(&SpaceMutation::AddCollection { collection: CollectionRef { id: "c2".into(), name: "Extra".into(), document_id: "doc-c2".into() } });
    store::test_support::assert_op_line_round_trip(&SpaceMutation::RemoveCollection { collection_id: "c2".into() });
    store::test_support::assert_op_line_round_trip(&SpaceMutation::RenameCollection { collection_id: "c1".into(), name: "Renamed Collection".into() });
    store::test_support::assert_op_line_round_trip(&SpaceMutation::InstallProgram { plugin_id: "cad".into() });
    store::test_support::assert_op_line_round_trip(&SpaceMutation::UninstallProgram { plugin_id: "cad".into() });
    store::test_support::assert_op_line_round_trip(&SpaceMutation::InstallExtension {
        extension_id: "flow-math".into(),
        version: "1.0.0".into(),
        source_uri: "https://example.test/flow-math.sxt".into(),
        package_hash: "hash-flow-math".into(),
        enabled: true,
    });
    store::test_support::assert_op_line_round_trip(&SpaceMutation::UninstallExtension { extension_id: "flow-math".into() });
    store::test_support::assert_op_line_round_trip(&SpaceMutation::SetExtensionEnabled { extension_id: "flow-math".into(), enabled: false });
}

#[test]
fn space_operation_backwards_restores_pre_state() {
    let base = demo_space();
    store::test_support::assert_operation_round_trip(&base, SpaceMutation::SetName { name: "New Name".into() });
    store::test_support::assert_operation_round_trip(&base, SpaceMutation::UpsertUser { user: demo_user("u2", SpaceRole::Author) });
    store::test_support::assert_operation_round_trip(&base, SpaceMutation::UpsertUser { user: demo_user("u1", SpaceRole::Spectator) });
    store::test_support::assert_operation_round_trip(&base, SpaceMutation::RemoveUser { user_id: "u1".into() });
    store::test_support::assert_operation_round_trip(&base, SpaceMutation::AddCollection { collection: CollectionRef { id: "c2".into(), name: "Extra".into(), document_id: "doc-c2".into() } });
    store::test_support::assert_operation_round_trip(&base, SpaceMutation::RemoveCollection { collection_id: "c1".into() });
    store::test_support::assert_operation_round_trip(&base, SpaceMutation::RenameCollection { collection_id: "c1".into(), name: "Renamed".into() });
    store::test_support::assert_operation_round_trip(&base, SpaceMutation::InstallProgram { plugin_id: "cad".into() });
    let mut with_program = base.clone();
    with_program.programs.push("cad".into());
    store::test_support::assert_operation_round_trip(&with_program, SpaceMutation::UninstallProgram { plugin_id: "cad".into() });
    store::test_support::assert_operation_round_trip(
        &base,
        SpaceMutation::InstallExtension { extension_id: "flow-math".into(), version: "1.0.0".into(), source_uri: "https://example.test/flow-math.sxt".into(), package_hash: "hash-flow-math".into(), enabled: true },
    );
    let mut with_extension = base.clone();
    with_extension.extensions.push(demo_extension("flow-math", true));
    store::test_support::assert_operation_round_trip(&with_extension, SpaceMutation::UninstallExtension { extension_id: "flow-math".into() });
    store::test_support::assert_operation_round_trip(&with_extension, SpaceMutation::SetExtensionEnabled { extension_id: "flow-math".into(), enabled: false });
    store::test_support::assert_operation_round_trip(
        &with_extension,
        SpaceMutation::InstallExtension { extension_id: "flow-math".into(), version: "2.0.0".into(), source_uri: "https://example.test/flow-math-v2.sxt".into(), package_hash: "hash-flow-math-v2".into(), enabled: false },
    );
}

#[test]
fn space_diff_print_parse_and_encode_decode_round_trip() {
    let diffs = vec![
        SpaceDiff { name: Some("Renamed".into()), ..Default::default() },
        SpaceDiff { upsert_user: Some(demo_user("u2", SpaceRole::Author)), ..Default::default() },
        SpaceDiff { install_program: Some("cad".into()), ..Default::default() },
        SpaceDiff { uninstall_program: Some("cad".into()), ..Default::default() },
        SpaceDiff { install_extension: Some(demo_extension("flow-math", true)), ..Default::default() },
        SpaceDiff { uninstall_extension_id: Some("flow-math".into()), ..Default::default() },
        SpaceDiff { set_extension_enabled_id: Some("flow-math".into()), set_extension_enabled: Some(false), ..Default::default() },
        SpaceDiff::default(),
    ];
    for diff in diffs {
        let printed = diff.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line: {printed:?}");
        let parsed = SpaceDiff::parse_diff(&printed).unwrap_or_else(|error| panic!("parse_diff failed for {printed:?}: {error}"));
        assert_eq!(parsed, diff);
        let encoded = diff.encode_diff().expect("encode_diff");
        let decoded = SpaceDiff::decode_diff(&encoded).expect("decode_diff");
        assert_eq!(decoded, diff);
    }
}
//#endregion 🧪️SpaceMutationLaws

//#region 🧪️CollectionMutationLaws
#[test]
fn collection_operation_op_text_round_trips_every_variant() {
    let folder = CollectionFolder { id: "f2".into(), parent_id: None, name: "Extra".into() };
    let entry = CollectionEntry { id: "e3".into(), folder_id: None, name: "extra".into(), kind_id: "puzzle.2d".into(), body: Box::new(ArtifactBody::Document { schema: "test.puzzle2d".into(), document_id: "doc-e3".into() }) };
    store::test_support::assert_op_line_round_trip(&CollectionMutation::RenameCollection { new_name: "Renamed".into() });
    store::test_support::assert_op_line_round_trip(&CollectionMutation::CreateFolder { folder: folder.clone(), index: 0 });
    store::test_support::assert_op_line_round_trip(&CollectionMutation::DeleteFolder { folder_id: "f1".into() });
    store::test_support::assert_op_line_round_trip(&CollectionMutation::MoveToCollection { folder_id: "f1".into(), new_parent: Some("f2".into()) });
    store::test_support::assert_op_line_round_trip(&CollectionMutation::MoveToCollection { folder_id: "f1".into(), new_parent: None });
    store::test_support::assert_op_line_round_trip(&CollectionMutation::RenameFolder { folder_id: "f1".into(), new_name: "Renders 2".into() });
    store::test_support::assert_op_line_round_trip(&CollectionMutation::CreateEntry { entry: entry.clone(), index: 0 });
    store::test_support::assert_op_line_round_trip(&CollectionMutation::DeleteEntry { entry_id: "e1".into() });
    store::test_support::assert_op_line_round_trip(&CollectionMutation::MoveToFolder { entry_id: "e1".into(), new_folder: None });
    store::test_support::assert_op_line_round_trip(&CollectionMutation::RenameEntry { entry_id: "e1".into(), new_name: "sketch 2".into() });
    store::test_support::assert_op_line_round_trip(&CollectionMutation::ReplaceEntryBody { entry_id: "e2".into(), new_body: Box::new(ArtifactBody::Blob { blob: store::BlobRef { hash: "h2".into(), size: 1, media_type: "image/png".into() } }) });
}

#[test]
fn collection_operation_binary_matches_text() {
    store::test_support::assert_op_text_binary_equivalence(&CollectionMutation::RenameCollection { new_name: "Renamed".into() });
    let entry = CollectionEntry { id: "e3".into(), folder_id: None, name: "extra".into(), kind_id: "puzzle.2d".into(), body: Box::new(ArtifactBody::Document { schema: "test.puzzle2d".into(), document_id: "doc-e3".into() }) };
    store::test_support::assert_op_text_binary_equivalence(&CollectionMutation::CreateEntry { entry, index: 0 });
}

#[test]
fn collection_operation_backwards_restores_pre_state() {
    let base = demo_collection();
    store::test_support::assert_operation_round_trip(&base, CollectionMutation::RenameCollection { new_name: "Renamed".into() });
    store::test_support::assert_operation_round_trip(&base, CollectionMutation::CreateFolder { folder: CollectionFolder { id: "f2".into(), parent_id: None, name: "Extra".into() }, index: 0 });
    store::test_support::assert_operation_round_trip(&base, CollectionMutation::DeleteFolder { folder_id: "f1".into() });
    store::test_support::assert_operation_round_trip(&base, CollectionMutation::MoveToCollection { folder_id: "f1".into(), new_parent: None });
    store::test_support::assert_operation_round_trip(&base, CollectionMutation::RenameFolder { folder_id: "f1".into(), new_name: "Renders 2".into() });
    store::test_support::assert_operation_round_trip(&base, CollectionMutation::DeleteEntry { entry_id: "e1".into() });
    store::test_support::assert_operation_round_trip(&base, CollectionMutation::MoveToFolder { entry_id: "e1".into(), new_folder: None });
    store::test_support::assert_operation_round_trip(&base, CollectionMutation::RenameEntry { entry_id: "e1".into(), new_name: "sketch 2".into() });
    store::test_support::assert_operation_round_trip(
        &base,
        CollectionMutation::ReplaceEntryBody { entry_id: "e2".into(), new_body: Box::new(ArtifactBody::Blob { blob: store::BlobRef { hash: "h2".into(), size: 1, media_type: "image/png".into() } }) },
    );
}

/// 🧪️ `DeleteFolder`'s cascade: deleting a folder that contains a nested subfolder plus entries in
/// both must remove the whole subtree, and `inverse` must restore it leaves-first (proves the
/// `folder_subtree_ids`/`folder_depth` helpers rather than trusting the shallow single-folder cases
/// above to exercise them).
#[test]
fn delete_folder_cascade_removes_and_restores_whole_subtree() {
    let mut collection = empty_collection_snapshot("Demo");
    collection.folders.push(CollectionFolder { id: "root".into(), parent_id: None, name: "Root".into() });
    collection.folders.push(CollectionFolder { id: "child".into(), parent_id: Some("root".into()), name: "Child".into() });
    collection.entries.push(CollectionEntry {
        id: "e-root".into(),
        folder_id: Some("root".into()),
        name: "in-root".into(),
        kind_id: "puzzle.2d".into(),
        body: Box::new(ArtifactBody::Document { schema: "test.puzzle2d".into(), document_id: "doc-e-root".into() }),
    });
    collection.entries.push(CollectionEntry {
        id: "e-child".into(),
        folder_id: Some("child".into()),
        name: "in-child".into(),
        kind_id: "puzzle.2d".into(),
        body: Box::new(ArtifactBody::Document { schema: "test.puzzle2d".into(), document_id: "doc-e-child".into() }),
    });

    store::test_support::assert_operation_round_trip(&collection, CollectionMutation::DeleteFolder { folder_id: "root".into() });

    let diff = CollectionMutation::DeleteFolder { folder_id: "root".into() }.diff(&collection).into_parts().0;
    let mut deleted_folders = diff.deleted_folder_ids.clone().unwrap_or_default();
    deleted_folders.sort();
    assert_eq!(deleted_folders, vec!["child".to_string(), "root".to_string()]);
    let mut deleted_entries = diff.deleted_entry_ids.clone().unwrap_or_default();
    deleted_entries.sort();
    assert_eq!(deleted_entries, vec!["e-child".to_string(), "e-root".to_string()]);

    let after = diff.apply(&collection).expect("valid collection diff");
    assert!(after.folders.is_empty());
    assert!(after.entries.is_empty());
}

#[test]
fn collection_diff_print_parse_and_encode_decode_round_trip() {
    let diffs = vec![CollectionDiff { renamed_collection: Some("Renamed".into()), ..Default::default() }, CollectionDiff { deleted_entry_ids: Some(vec!["e1".into()]), ..Default::default() }, CollectionDiff::default()];
    for diff in diffs {
        let printed = diff.print_diff();
        assert!(!printed.contains('\n'));
        let parsed = CollectionDiff::parse_diff(&printed).unwrap_or_else(|error| panic!("parse_diff failed for {printed:?}: {error}"));
        assert_eq!(parsed, diff);
        let encoded = diff.encode_diff().expect("encode_diff");
        let decoded = CollectionDiff::decode_diff(&encoded).expect("decode_diff");
        assert_eq!(decoded, diff);
    }
}
//#endregion 🧪️CollectionMutationLaws

//#region 🧪️RoleLaws
#[test]
fn can_write_follows_kind_and_role() {
    let mut archive = empty_space_snapshot("Frozen", SpaceKind::Archive, SpaceVisibility::Public);
    archive.users.push(demo_user("u1", SpaceRole::Author));
    assert!(!can_write(&archive, "u1"), "archive never accepts writes, even from an author");

    let mut studio = empty_space_snapshot("Studio", SpaceKind::Studio, SpaceVisibility::Private);
    studio.users.push(demo_user("u1", SpaceRole::Author));
    studio.users.push(demo_user("u2", SpaceRole::Spectator));
    assert!(can_write(&studio, "u1"));
    assert!(!can_write(&studio, "u2"));
    assert!(!can_write(&studio, "unknown"));
}

#[test]
fn atelier_reconcile_keeps_a_single_author_by_smallest_id() {
    let mut atelier = empty_space_snapshot("Atelier", SpaceKind::Atelier, SpaceVisibility::Private);
    atelier.users.push(demo_user("u2", SpaceRole::Author));
    atelier.users.push(demo_user("u1", SpaceRole::Author));
    let (reconciled, reports) = reconcile_space_atelier_invariant(atelier);
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].target.first().map(String::as_str), Some("space/atelier-multi-author"));
    assert_eq!(space_role_of(&reconciled, "u1"), Some(SpaceRole::Author));
    assert_eq!(space_role_of(&reconciled, "u2"), Some(SpaceRole::Spectator));
}

#[test]
fn atelier_reconcile_is_a_noop_with_a_single_author() {
    let (_, reports) = reconcile_space_atelier_invariant(demo_space());
    assert!(reports.is_empty());
}
//#endregion 🧪️RoleLaws

//#region 🧪️CollectionReconcileLaws
#[test]
fn reconcile_reparents_orphan_folder_to_root() {
    let mut collection = empty_collection_snapshot("Demo");
    collection.folders.push(CollectionFolder { id: "f1".into(), parent_id: Some("missing".into()), name: "Orphan".into() });
    let (reconciled, reports) = reconcile_collection_integrity(collection);
    assert!(reports.iter().any(|r| r.target.first().map(String::as_str) == Some("collection/folder-orphaned")));
    assert_eq!(reconciled.folders[0].parent_id, None);
}

#[test]
fn reconcile_cuts_folder_cycle() {
    let mut collection = empty_collection_snapshot("Demo");
    collection.folders.push(CollectionFolder { id: "a".into(), parent_id: Some("b".into()), name: "A".into() });
    collection.folders.push(CollectionFolder { id: "b".into(), parent_id: Some("a".into()), name: "B".into() });
    let (reconciled, reports) = reconcile_collection_integrity(collection);
    assert!(reports.iter().any(|r| r.target.first().map(String::as_str) == Some("collection/folder-cycle")));
    assert!(reconciled.folders.iter().all(|f| f.parent_id.is_none()), "both cyclic folders must be cut to root");
}

#[test]
fn reconcile_suffixes_duplicate_sibling_folder_names() {
    let mut collection = empty_collection_snapshot("Demo");
    collection.folders.push(CollectionFolder { id: "f1".into(), parent_id: None, name: "Renders".into() });
    collection.folders.push(CollectionFolder { id: "f2".into(), parent_id: None, name: "Renders".into() });
    let (reconciled, reports) = reconcile_collection_integrity(collection);
    assert!(reports.iter().any(|r| r.target.first().map(String::as_str) == Some("collection/folder-name-collision")));
    let names: Vec<&str> = reconciled.folders.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(names, vec!["Renders", "Renders (2)"]);
}

#[test]
fn reconcile_reparents_entry_pointing_at_missing_folder() {
    let mut collection = empty_collection_snapshot("Demo");
    collection.entries.push(CollectionEntry {
        id: "e1".into(),
        folder_id: Some("missing".into()),
        name: "sketch".into(),
        kind_id: "puzzle.2d".into(),
        body: Box::new(ArtifactBody::Document { schema: "test.puzzle2d".into(), document_id: "doc-e1".into() }),
    });
    let (reconciled, reports) = reconcile_collection_integrity(collection);
    assert!(reports.iter().any(|r| r.target.first().map(String::as_str) == Some("collection/entry-folder-missing")));
    assert_eq!(reconciled.entries[0].folder_id, None);
}
//#endregion 🧪️CollectionReconcileLaws

//#region 🧪️PathResolverLaws
#[test]
fn folder_and_entry_path_round_trip() {
    let collection = demo_collection();
    assert_eq!(folder_path(&collection, "f1"), Some("Renders".into()));
    assert_eq!(entry_path(&collection, "e1"), Some("Renders/sketch".into()));
    assert_eq!(entry_path(&collection, "e2"), Some("reference.png".into()));
    assert_eq!(resolve_entry_by_path(&collection, "Renders/sketch").map(|entry| entry.id.as_str()), Some("e1"));
    assert_eq!(resolve_entry_by_path(&collection, "reference.png").map(|entry| entry.id.as_str()), Some("e2"));
    assert_eq!(resolve_entry_by_path(&collection, "nowhere"), None);
}

#[test]
fn moves_and_renames_never_break_id_based_refs() {
    let mut collection = demo_collection();
    let before_id = collection.entries[0].id.clone();
    // Rename the folder — the path changes, the id-based ref doesn't.
    collection.folders[0].name = "Outputs".into();
    assert_eq!(entry_path(&collection, &before_id), Some("Outputs/sketch".into()));
    assert!(collection.entries.iter().any(|entry| entry.id == before_id));
}

#[test]
fn backbone_uris_are_stable() {
    assert_eq!(space_backbone_uri("space-1"), "space://space-1");
    assert_eq!(collection_backbone_uri("space-1", "col-1"), "space://space-1/collection/col-1");
    assert_eq!(artifact_backbone_uri("space-1", "art-1"), "space://space-1/artifact/art-1");
}
//#endregion 🧪️PathResolverLaws

//#region 🧪️DraftLaws
fn memory_draft_port() -> Arc<store::BackbonePorts> {
    Arc::new(store::BackbonePorts::Memory(crate::host::resolve_kernel_future(store::MemoryBackbonePort::new())))
}

#[test]
fn draft_create_list_expire_lifecycle() {
    let catalog = DraftCatalog::new();
    let port = memory_draft_port();
    let draft_a = catalog.create_draft("puzzle.2d", "test.puzzle2d", "sketch-a", 1_000, Some(500));
    let draft_b = catalog.create_draft("puzzle.2d", "test.puzzle2d", "sketch-b", 1_000, None);
    assert_eq!(draft_a.expires_at_ms, Some(1_500));
    assert_eq!(draft_b.expires_at_ms, None, "None ttl means pinned");

    let listed = catalog.list_drafts();
    assert_eq!(listed.len(), 2);

    let expired = catalog.expire_drafts(1_400, &port);
    assert!(expired.is_empty(), "not yet expired");
    let expired = catalog.expire_drafts(1_500, &port);
    assert_eq!(expired, vec![draft_a.artifact_id.clone()]);
    assert_eq!(catalog.list_drafts().len(), 1, "the pinned draft survives");
}

#[test]
fn list_drafts_sweeping_expired_removes_stale_entries_first() {
    let catalog = DraftCatalog::new();
    let port = memory_draft_port();
    let draft = catalog.create_draft("puzzle.2d", "test.puzzle2d", "stale", 0, Some(100));
    assert_eq!(catalog.list_drafts_sweeping_expired(50, &port).len(), 1, "not yet expired");
    assert!(catalog.list_drafts_sweeping_expired(200, &port).is_empty(), "swept before listing");
    assert!(catalog.list_drafts().iter().all(|entry| entry.artifact_id != draft.artifact_id));
}

#[test]
fn discard_draft_removes_bookkeeping_and_tombstones_bytes() {
    let catalog = DraftCatalog::new();
    let port = memory_draft_port();
    let draft = catalog.create_draft("puzzle.2d", "test.puzzle2d", "scratch", 0, None);
    port.write(&draft_uri(&draft.artifact_id), b"draft-bytes").expect("seed draft bytes");

    let removed = catalog.discard_draft(&port, &draft.artifact_id).expect("discard");
    assert_eq!(removed.artifact_id, draft.artifact_id);
    assert!(catalog.list_drafts().is_empty());
    assert_eq!(port.read(&draft_uri(&draft.artifact_id)).expect("read tombstone"), Vec::<u8>::new());
    assert!(catalog.discard_draft(&port, &draft.artifact_id).is_none(), "already discarded");
}

/// 🧪️ The plan's core promotion invariant, proven with REAL bytes (not a fixture string): the
/// artifact's envelope bytes at `artifact_backbone_uri` after promotion are byte-for-byte IDENTICAL
/// to what was at `draft_uri` before promotion — just relocated under a different backbone uri, no
/// decode/re-encode anywhere in the path.
#[test]
fn draft_promote_moves_envelope_bytes_byte_identical() {
    let catalog = DraftCatalog::new();
    let port = memory_draft_port();
    let draft = catalog.create_draft("puzzle.2d", "test.puzzle2d", "sketch", 0, None);
    let original_bytes = b"pretend-pack-plus-spr-envelope-bytes-with-full-vcs-history".to_vec();
    port.write(&draft_uri(&draft.artifact_id), &original_bytes).expect("seed draft bytes");

    let (removed_draft, operation) = catalog.promote_draft(&port, "space-1", &draft.artifact_id, Some("f1".into())).expect("promote");
    assert_eq!(removed_draft.artifact_id, draft.artifact_id);
    let CollectionMutation::CreateEntry { entry, .. } = &operation else { panic!("promote_draft must return CreateEntry") };
    assert_eq!(entry.id, draft.artifact_id, "promotion preserves the document id");
    assert_eq!(entry.folder_id, Some("f1".into()));
    assert!(catalog.list_drafts().is_empty(), "promoted draft is no longer a draft");

    let moved_bytes = port.read(&artifact_backbone_uri("space-1", &draft.artifact_id)).expect("read promoted bytes");
    assert_eq!(moved_bytes, original_bytes, "promoted envelope bytes are byte-identical, just at a different backbone uri");
    assert_eq!(port.read(&draft_uri(&draft.artifact_id)).expect("read tombstoned draft uri"), Vec::<u8>::new(), "draft uri is tombstoned after promotion");

    let demote = DraftCatalog::demote_operation(&entry.id);
    assert_eq!(demote, CollectionMutation::DeleteEntry { entry_id: entry.id.clone() });

    // Mutation-sourced round trip: CreateEntry then its inverse restores the empty collection.
    let empty = empty_collection_snapshot("Demo");
    store::test_support::assert_operation_round_trip(&empty, operation);
}

/// 🧪️ `demote_asset` is `promote_draft`'s real byte-moving inverse: the SAME bytes travel back
/// from the asset uri to the draft uri, byte-identical, and fresh draft bookkeeping reappears.
#[test]
fn demote_asset_moves_bytes_back_and_reregisters_draft_bookkeeping() {
    let catalog = DraftCatalog::new();
    let port = memory_draft_port();
    let draft = catalog.create_draft("puzzle.2d", "test.puzzle2d", "sketch", 0, None);
    let original_bytes = b"envelope-bytes-round-tripping-through-promote-then-demote".to_vec();
    port.write(&draft_uri(&draft.artifact_id), &original_bytes).expect("seed draft bytes");

    let (_, operation) = catalog.promote_draft(&port, "space-1", &draft.artifact_id, None).expect("promote");
    let CollectionMutation::CreateEntry { entry, .. } = operation else { panic!("expected CreateEntry") };

    let demote_operation = catalog.demote_asset(&port, "space-1", &entry, "test.puzzle2d", 2_000, Some(1_000)).expect("demote");
    assert_eq!(demote_operation, CollectionMutation::DeleteEntry { entry_id: entry.id.clone() });

    let restored_bytes = port.read(&draft_uri(&entry.id)).expect("read demoted draft bytes");
    assert_eq!(restored_bytes, original_bytes, "demoted envelope bytes are byte-identical to the originally-promoted ones");
    assert_eq!(port.read(&artifact_backbone_uri("space-1", &entry.id)).expect("read tombstoned asset uri"), Vec::<u8>::new());

    let redrafted = catalog.list_drafts();
    assert_eq!(redrafted.len(), 1);
    assert_eq!(redrafted[0].artifact_id, entry.id);
    assert_eq!(redrafted[0].expires_at_ms, Some(3_000), "demotion re-registers a fresh TTL window");
}

#[test]
fn promote_unknown_draft_errors() {
    let catalog = DraftCatalog::new();
    let port = memory_draft_port();
    assert_eq!(catalog.promote_draft(&port, "space-1", "nope", None), Err(SpaceError::UnknownDraft("nope".into())));
}

/// 🧪️ `draft_catalog_for` is the port-keyed global registry: the SAME `Arc<store::BackbonePorts>`
/// identity always resolves to the SAME `DraftCatalog` instance (so callers sharing a port share
/// draft bookkeeping), while two DISTINCT port identities never share one.
#[test]
fn draft_catalog_for_is_keyed_by_port_identity() {
    let port_a = memory_draft_port();
    let port_b = memory_draft_port();

    let catalog_a1 = draft_catalog_for(&port_a);
    let catalog_a1_created = catalog_a1.create_draft("puzzle.2d", "test.puzzle2d", "shared", 0, None);
    let catalog_a2 = draft_catalog_for(&port_a);
    assert_eq!(catalog_a2.list_drafts().iter().map(|entry| entry.artifact_id.clone()).collect::<Vec<_>>(), vec![catalog_a1_created.artifact_id.clone()], "same port identity shares one catalog");

    let catalog_b = draft_catalog_for(&port_b);
    assert!(catalog_b.list_drafts().is_empty(), "a distinct port identity gets its own catalog");
}
//#endregion 🧪️DraftLaws

//#region 🧪️ZipLaws
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn zip_fixture_bytes() -> Vec<u8> {
    let collection = demo_collection();
    let read_artifact = |entry_id: &str| -> Result<(Vec<u8>, Vec<u8>), SpaceZipError> { Ok((format!("pack-bytes-for-{entry_id}").into_bytes(), format!("spr-bytes-for-{entry_id}").into_bytes())) };
    let read_blob = |hash: &str| -> Result<Vec<u8>, SpaceZipError> { Ok(format!("blob-bytes-for-{hash}").into_bytes()) };
    export_collection_zip(&collection, b"collection-spr-bytes", &read_artifact, &read_blob).expect("export")
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[test]
fn zip_export_import_round_trips_structure_and_bytes() {
    let bytes = zip_fixture_bytes();
    let imported = import_collection_zip(&bytes).expect("import");
    assert_eq!(imported.collection, demo_collection());
    assert_eq!(imported.collection_spr, b"collection-spr-bytes");
    assert_eq!(imported.artifacts.len(), 1, "one document entry");
    let (entry, pack_bytes, spr_bytes) = &imported.artifacts[0];
    assert_eq!(entry.id, "e1");
    assert_eq!(pack_bytes, b"pack-bytes-for-e1");
    assert_eq!(spr_bytes, b"spr-bytes-for-e1");
    assert_eq!(imported.blobs.len(), 1, "one blob entry");
    assert_eq!(imported.blobs[0].0.hash, "blake3-deadbeef");
    assert_eq!(imported.blobs[0].1, b"blob-bytes-for-blake3-deadbeef");
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[test]
fn zip_export_import_export_is_byte_stable() {
    let once = zip_fixture_bytes();
    let imported = import_collection_zip(&once).expect("import");
    let read_artifact = |entry_id: &str| -> Result<(Vec<u8>, Vec<u8>), SpaceZipError> {
        let (_, pack_bytes, spr_bytes) = imported.artifacts.iter().find(|(entry, _, _)| entry.id == entry_id).expect("artifact bytes");
        Ok((pack_bytes.clone(), spr_bytes.clone()))
    };
    let read_blob = |hash: &str| -> Result<Vec<u8>, SpaceZipError> {
        let (_, bytes) = imported.blobs.iter().find(|(blob, _)| blob.hash == hash).expect("blob bytes");
        Ok(bytes.clone())
    };
    let twice = export_collection_zip(&imported.collection, &imported.collection_spr, &read_artifact, &read_blob).expect("re-export");
    assert_eq!(once, twice, "export -> import -> export must be byte-stable");
}
//#endregion 🧪️ZipLaws

//#region 🧪️ZipStoreBridgeLaws
/// 🧪️ Minimal in-memory `store::BlobStore` test double — content-addressed via a fast
/// non-cryptographic hash (no need for `framework_hash`'s real Blake3, this crate has no such
/// dependency and a test double only needs internal consistency, not a production hash).
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[derive(Default)]
struct TestBlobStore {
    entries: Mutex<HashMap<String, (Vec<u8>, String)>>,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn test_blob_hash(bytes: &[u8]) -> String {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    bytes.hash(&mut hasher);
    format!("test-{:016x}", hasher.finish())
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl BlobStore for TestBlobStore {
    async fn put(&self, bytes: &[u8], media_type: &str) -> Result<store::BlobRef, store::VcsError> {
        let hash = test_blob_hash(bytes);
        self.entries.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(hash.clone(), (bytes.to_vec(), media_type.to_string()));
        Ok(store::BlobRef { hash, size: bytes.len() as u64, media_type: media_type.to_string() })
    }

    async fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, store::VcsError> {
        Ok(self.entries.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(hash).map(|(bytes, _)| bytes.clone()))
    }

    async fn has(&self, hash: &str) -> Result<bool, store::VcsError> {
        Ok(self.entries.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains_key(hash))
    }

    async fn delete(&self, hash: &str) -> Result<(), store::VcsError> {
        self.entries.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(hash);
        Ok(())
    }
}

/// 🧪️ End-to-end law with REAL `store` types (not the fixture-string readers `zip_fixture_bytes`
/// injects above): a real `store::ArtifactStore<SpaceSnapshot, SpaceMutation>` document
/// artifact (itself an ordinary `#[derive(dsl::DslArtifact)]`/`#[derive(dsl::DslOps)]` document —
/// exercising exactly the same `ArtifactPack`/`OpBinary`/`OpText` machinery any real app document
/// would) plus a real blob round-trip through `real_artifact_reader`/`real_blob_reader`/
/// `import_document_artifact`/`import_blob`, asserting the round trip preserves collection
/// structure AND artifact envelope bytes byte-for-byte (the plan's "lossless" requirement), and
/// that export->import->export stays byte-stable with real data too.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[test]
fn zip_export_import_round_trips_real_store_documents_and_blob() {
    // 🌉️ `ArtifactStore::new`/`dispatch`/`snapshot_pack`/`TestBlobStore::put`/`get` all turned
    // `async fn` under the runtime-dependency sweep; this test stays a plain sync `#[test]` (this
    // crate has no `#[async_test]` harness dependency) and bridges via
    // `crate::host::resolve_kernel_future`, same as this file's other async fallout fixes — every
    // op here is an in-memory fixture operation, never real I/O.
    let mut nested_space_store =
        crate::host::resolve_kernel_future(store::ArtifactStore::new(store::create_document_envelope::<SpaceSnapshot, SpaceMutation>(S_SPACE_SCHEMA, "art-nested-space", demo_space(), None))).expect("valid artifact store fixture");
    crate::host::resolve_kernel_future(nested_space_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![SpaceMutation::SetName { name: "Nested Space".into() }], description: None })).expect("apply");
    crate::host::resolve_kernel_future(nested_space_store.dispatch(store::ArtifactCommand::CommitCheckpoint { message: Some("checkpoint".into()), authors: Vec::new() })).expect("commit checkpoint");
    let original_pack_files = crate::host::resolve_kernel_future(nested_space_store.snapshot_pack()).expect("snapshot pack");

    let blob_store = TestBlobStore::default();
    let blob_ref = crate::host::resolve_kernel_future(blob_store.put(b"hello blob bytes", "text/plain")).expect("put blob");

    let mut collection = empty_collection_snapshot("RealDemo");
    collection.entries.push(CollectionEntry {
        id: "art-nested-space".into(),
        folder_id: None,
        name: "nested-space".into(),
        kind_id: "os.space".into(),
        body: Box::new(ArtifactBody::Document { schema: S_SPACE_SCHEMA.into(), document_id: "art-nested-space".into() }),
    });
    collection.entries.push(CollectionEntry { id: "blob-1".into(), folder_id: None, name: "note.txt".into(), kind_id: "file.blob".into(), body: Box::new(ArtifactBody::Blob { blob: blob_ref.clone() }) });

    let mut pack_files = HashMap::new();
    pack_files.insert("art-nested-space".to_string(), original_pack_files.clone());
    let read_artifact = real_artifact_reader(&pack_files);
    let read_blob = real_blob_reader(&blob_store);
    let collection_spr = b"collection-history-bytes".to_vec();
    let zip_bytes = export_collection_zip(&collection, &collection_spr, &read_artifact, &read_blob).expect("export");

    let imported = import_collection_zip(&zip_bytes).expect("import");
    assert_eq!(imported.collection, collection, "collection structure survives the round trip");
    assert_eq!(imported.collection_spr, collection_spr);

    let (_, imported_pack, imported_spr) = imported.artifacts.iter().find(|(entry, _, _)| entry.id == "art-nested-space").expect("artifact present");
    assert_eq!(imported_pack, &original_pack_files.pack, "artifact pack bytes are byte-identical after the round trip");
    assert_eq!(imported_spr, &original_pack_files.spr, "artifact spr bytes are byte-identical after the round trip");

    let restored_store = crate::host::resolve_kernel_future(import_document_artifact::<SpaceSnapshot, SpaceMutation>(imported_pack, imported_spr)).expect("reconstruct store");
    assert_eq!(restored_store.snapshot().expect("projection"), nested_space_store.snapshot().expect("projection"), "reconstructed document projection matches the original exactly");

    let (imported_blob, imported_blob_bytes) = imported.blobs.iter().find(|(blob, _)| blob.hash == blob_ref.hash).expect("blob present");
    assert_eq!(imported_blob_bytes, b"hello blob bytes");
    let fresh_blob_store = TestBlobStore::default();
    import_blob(&fresh_blob_store, imported_blob, imported_blob_bytes.clone()).expect("import blob");
    assert_eq!(crate::host::resolve_kernel_future(fresh_blob_store.get(&blob_ref.hash)).expect("get"), Some(b"hello blob bytes".to_vec()));

    // export -> import -> export must stay byte-stable with REAL data too (not just injected
    // fixture strings) — the law `zip_export_import_export_is_byte_stable` proved with mock bytes
    // holds end-to-end.
    let mut reexport_pack_files = HashMap::new();
    reexport_pack_files.insert("art-nested-space".to_string(), store::ArtifactPackFiles { pack: imported_pack.clone(), spr: imported_spr.clone(), ops: String::new() });
    let re_read_artifact = real_artifact_reader(&reexport_pack_files);
    let re_read_blob = real_blob_reader(&fresh_blob_store);
    let twice = export_collection_zip(&imported.collection, &imported.collection_spr, &re_read_artifact, &re_read_blob).expect("re-export");
    assert_eq!(zip_bytes, twice, "export -> import -> export is byte-stable with real store-backed data");
}
//#endregion 🧪️ZipStoreBridgeLaws
