
use super::*;

fn strict_import_source(source: &str) -> bool {
    [
        "Puzzle5dImportStage::CensusParts",
        "Puzzle5dImportStage::ReserveCatalogParts",
        "Puzzle5dImportStage::ReserveMutations",
        "Puzzle5dImportStage::LoadCatalogParts",
        "Puzzle5dImportStage::PartReserve",
        "Puzzle5dImportStage::PartVortices",
        "Puzzle5dImportStage::PartPublish",
        "Puzzle5dImportStage::CatalogMutation",
        "PUZZLE5D_IMPORT_SEMANTIC_ITEMS",
        "part_index: Vec<(String, usize)>",
        "compatibility_index: Vec<((String, String), usize)>",
        "puzzle5d_import_checkpoint(self.stage as u8, self.cursor, self.nested_cursor, self.decoded_items, self.progress, cx)",
        "puzzle5d_retire_part_kind_step",
        "puzzle5d_retire_catalogs_step",
        "puzzle5d_retire_import_mutation_step",
        "puzzle5d_decode_import_fragment",
    ]
    .into_iter()
    .all(|marker| source.contains(marker))
        && source.matches("Puzzle5dImportStage::PartVortices").count() == 2
        && source.matches("Puzzle5dImportStage::CatalogMutation").count() == 2
        && source.matches("puzzle5d_retire_part_kind_step").count() == 4
        && !source.contains("self.initial_catalogs")
        && !source.contains("self.compatibility_mutations")
        && !source.contains("part_index: HashMap")
        && !source.contains("compatibility_index: HashMap")
        && !source.contains("self.rows(\"objectKinds\").get(self.cursor).cloned()")
        && !source.contains("pop_owner!(self.catalogs.parts)")
}

#[test]
fn recursive_json_zero_and_key_max_plus_one_preserve_exact_owner_before_incremental_close() {
    let oversized_key = "k".repeat(PUZZLE5D_JSON_RETIREMENT_KEY_BYTES + 1);
    let mut value = serde_json::json!({ oversized_key.clone(): { "nested": ["payload"] } });
    let before = value.clone();
    let mut key = [0; PUZZLE5D_JSON_RETIREMENT_KEY_BYTES];
    assert!(puzzle5d_retire_json_step(&mut value, &mut key, 0).is_err());
    assert_eq!(value, before);
    let nested =
        value.as_object_mut().and_then(|object| object.get_mut(&oversized_key)).and_then(serde_json::Value::as_object_mut).and_then(|object| object.get_mut("nested")).and_then(serde_json::Value::as_array_mut).and_then(|array| array.last_mut());
    if let Some(nested) = nested {
        *nested = serde_json::Value::Null;
    }
    assert!(puzzle5d_retire_json_step(&mut value, &mut key, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).is_err());
    assert!(value.get(&oversized_key).is_some());
}

#[test]
fn empty_vector_backing_requires_exact_byte_credit_and_retires_once() {
    let mut owners = Vec::<u64>::with_capacity(17);
    let admitted = owners.capacity();
    let bytes = admitted * size_of::<u64>();
    assert!(puzzle5d_retire_vec_backing(&mut owners, bytes - 1).is_err());
    assert_eq!(owners.capacity(), admitted);
    assert!(matches!(puzzle5d_retire_vec_backing(&mut owners, bytes), Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes })) if released_bytes == bytes));
    assert_eq!(owners.capacity(), 0);
    assert!(matches!(puzzle5d_retire_vec_backing(&mut owners, bytes), Ok(None)));
}

#[test]
fn reserved_wire_exact_max_and_plus_one_preflight_return_the_original_owner() {
    let maximum = vec![7; PUZZLE5D_RESERVED_RAW_BYTES];
    let maximum_identity = maximum.as_ptr();
    let admitted = puzzle5d_preflight_reserved_wire(maximum, PUZZLE5D_RESERVED_RAW_BYTES).expect("exact maximum is admitted");
    assert_eq!(admitted.as_ptr(), maximum_identity, "exact maximum preserves the original fixed-page source owner");

    let plus_one = vec![9; PUZZLE5D_RESERVED_RAW_BYTES + 1];
    let plus_one_identity = plus_one.as_ptr();
    let (_, rejected) = puzzle5d_preflight_reserved_wire(plus_one, PUZZLE5D_RESERVED_RAW_BYTES).expect_err("maximum plus one is rejected before copy");
    assert_eq!(rejected.as_ptr(), plus_one_identity, "maximum plus one returns the exact rejected wire owner");
}

#[test]
fn import_media_exact_parse_cap_and_plus_one_are_preflighted_under_one_turn_budget() {
    let prefix = r#"{"objectKinds":[],"vortexKinds":[{"id":"grip","name":"Grip","label":""#;
    let suffix = r##"","color":"#fff","defaultCableKind":""}],"kindCompatibility":[]}"##;
    let label = "x".repeat(PUZZLE5D_IMPORT_MEDIA_BYTES.checked_sub(prefix.len() + suffix.len()).expect("fixture shell fits import cap"));
    let maximum = format!("{prefix}{label}{suffix}");
    assert_eq!(maximum.len(), PUZZLE5D_IMPORT_MEDIA_BYTES);
    let started = std::time::Instant::now();
    let parsed = puzzle5d_decode_import_fragment(&maximum).expect("exact import-media cap parses");
    let elapsed = started.elapsed();
    assert!(elapsed < std::time::Duration::from_micros(7_500), "bounded import-media serde turn exceeded 7.5 ms: {elapsed:?}");
    assert!(puzzle5d_import_keys_are(&parsed, &["objectKinds", "vortexKinds", "kindCompatibility"]));
    let maximum_plus_one = format!("{maximum} ");
    assert_eq!(maximum_plus_one.len(), PUZZLE5D_IMPORT_MEDIA_BYTES + 1);
    assert!(puzzle5d_decode_import_fragment(&maximum_plus_one).is_err());

    let canonical = dsl::json!({
        "schema": "manifest",
        "objectKinds": [],
        "vortexKinds": [],
        "cableKinds": [],
        "attractionKinds": [],
        "kindCompatibility": [],
    });
    assert!(puzzle5d_import_keys_are(&canonical, &["schema", "objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"]));
    let hostile = dsl::json!({ "objectKinds": [], "legacyRows": [] });
    assert!(!puzzle5d_import_keys_are(&hostile, &["schema", "objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"]));
}

#[test]
fn import_media_typed_close_retires_nested_owners_one_bounded_unit_per_turn() {
    let mut owner = crate::Puzzle5dCatalogPartKind {
        id: "part-ä".repeat(64),
        name: "Part".into(),
        label: "Teil".into(),
        representations: vec![crate::Puzzle5dRepresentation { id: "mesh".into(), name: "Mesh".into(), url: "mesh.glb".into(), mime: "model/gltf-binary".into(), tags: vec!["tag-ß".repeat(64)], ..Default::default() }],
        grips: vec![crate::Puzzle5dGripTemplate { id: "g0".into(), name: "socket".into(), label: "Socket".into(), grip_kind: Some("socket".into()), ..Default::default() }],
        ..Default::default()
    };
    let mut turns = 0usize;
    loop {
        match puzzle5d_retire_part_kind_step(&mut owner, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).expect("bounded typed close") {
            Some(PluginCloseStep::Pending { released_items, released_bytes }) => {
                assert!(released_items <= 1);
                assert!(released_bytes <= semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                turns += 1;
            }
            Some(PluginCloseStep::AwaitingInput { .. } | PluginCloseStep::Blocked { .. }) => panic!("closed typed fixture unexpectedly requires input"),
            Some(PluginCloseStep::Complete) => panic!("nested owner helper cannot publish outer completion"),
            None => break,
        }
        assert!(turns < 100_000, "typed close did not converge");
    }
    assert!(turns > 8, "nested close was collapsed into an unbounded row drop");
    assert!(owner.id.is_empty() && owner.id.capacity() == 0);
    assert!(owner.representations.is_empty() && owner.representations.capacity() == 0);
    assert!(owner.grips.is_empty() && owner.grips.capacity() == 0);

    let mut mutation = crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs(Some(crate::Puzzle5dKindCatalogs { parts: vec![crate::Puzzle5dCatalogPartKind { id: "cancelled-part".into(), ..Default::default() }], ..Default::default() }));
    let mut mutation_turns = 0usize;
    loop {
        match puzzle5d_retire_import_mutation_step(&mut mutation, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).expect("bounded preassembled mutation close") {
            Some(PluginCloseStep::Pending { released_items, released_bytes }) => {
                assert!(released_items <= 1);
                assert!(released_bytes <= semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                mutation_turns += 1;
            }
            Some(PluginCloseStep::AwaitingInput { .. } | PluginCloseStep::Blocked { .. }) => panic!("closed typed fixture unexpectedly requires input"),
            Some(PluginCloseStep::Complete) => panic!("mutation helper cannot publish outer completion"),
            None => break,
        }
        assert!(mutation_turns < 100_000, "preassembled mutation close did not converge");
    }
    assert!(matches!(mutation, Puzzle5dMutation::ReplaceKindCatalogs(value) if value.new_catalogs.is_none()));
}

#[test]
fn import_media_exact_semantic_backings_fit_one_native_or_wasm_close_page() {
    fn exact_backing_bytes<T>(items: usize) -> usize {
        let mut owner = Vec::<T>::new();
        owner.try_reserve_exact(items).expect("fixed-page descriptor reserve");
        owner.capacity().checked_mul(size_of::<T>()).expect("fixed-page descriptor extent")
    }

    let page = semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    assert!(exact_backing_bytes::<crate::Puzzle5dCatalogPartKind>(PUZZLE5D_IMPORT_SEMANTIC_ITEMS) <= page);
    assert!(exact_backing_bytes::<crate::Puzzle5dCatalogGripKind>(PUZZLE5D_IMPORT_SEMANTIC_ITEMS) <= page);
    assert!(exact_backing_bytes::<crate::Puzzle5dCatalogFastenerKind>(PUZZLE5D_IMPORT_SEMANTIC_ITEMS) <= page);
    assert!(exact_backing_bytes::<crate::Puzzle5dCatalogRopeKind>(PUZZLE5D_IMPORT_SEMANTIC_ITEMS) <= page);
    assert!(exact_backing_bytes::<crate::Puzzle5dKindCompatibility>(PUZZLE5D_IMPORT_SEMANTIC_ITEMS) <= page);
    assert!(exact_backing_bytes::<crate::Puzzle5dGripTemplate>(PUZZLE5D_IMPORT_SEMANTIC_ITEMS) <= page);
    assert!(exact_backing_bytes::<(String, usize)>(PUZZLE5D_IMPORT_SEMANTIC_ITEMS) <= page);
    assert!(exact_backing_bytes::<((String, String), usize)>(PUZZLE5D_IMPORT_SEMANTIC_ITEMS) <= page);
    for page_index in 0..PUZZLE5D_IMPORT_MUTATION_PAGES {
        let remaining = PUZZLE5D_IMPORT_MUTATION_ITEMS.saturating_sub(page_index * PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE);
        let items = remaining.min(PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE);
        assert!(exact_backing_bytes::<Puzzle5dMutation>(items) <= page);
    }
    assert_eq!(PUZZLE5D_IMPORT_DECODED_ITEMS, 1_184);
    assert_eq!(PUZZLE5D_IMPORT_MUTATION_ITEMS, 65);
    assert_eq!(PUZZLE5D_IMPORT_MUTATION_PAGES, 2);
}

#[test]
fn import_media_checkpoint_preserves_outer_nested_census_and_progress_cursors() {
    let state = puzzle5d_import_checkpoint_bytes(23, usize::MAX, usize::MAX - 1, usize::MAX - 2, u64::MAX - 3);
    assert_eq!(state.len(), 33);
    assert_eq!(state[0], 23);
    assert_eq!(u64::from_le_bytes(state[1..9].try_into().expect("outer cursor slice")), usize::MAX as u64);
    assert_eq!(u64::from_le_bytes(state[9..17].try_into().expect("nested cursor slice")), (usize::MAX - 1) as u64);
    assert_eq!(u64::from_le_bytes(state[17..25].try_into().expect("census slice")), (usize::MAX - 2) as u64);
    assert_eq!(u64::from_le_bytes(state[25..33].try_into().expect("progress slice")), u64::MAX - 3);
    assert_ne!(state, puzzle5d_import_checkpoint_bytes(23, usize::MAX, usize::MAX - 2, usize::MAX - 2, u64::MAX - 3));
}

#[test]
fn import_media_source_requires_schema_census_reserve_nested_cursor_and_recursive_close() {
    let source = include_str!("../../🦀️.rs");
    let import = source.rsplit_once("enum Puzzle5dImportStage").and_then(|(_, suffix)| suffix.split_once("//#endregion 🧵️ReservedJobs").map(|(import, _)| import)).expect("import source region");
    assert!(strict_import_source(import));
    assert!(!strict_import_source(&import.replacen("Puzzle5dImportStage::PartVortices", "Puzzle5dImportStage::Parts", 1)));
    assert!(!strict_import_source(&import.replacen("puzzle5d_retire_part_kind_step", "unbounded_drop", 1)));
    assert!(!strict_import_source(&import.replacen("Puzzle5dImportStage::CatalogMutation", "Puzzle5dImportStage::Complete", 1)));
    assert!(!strict_import_source(&import.replacen("self.nested_cursor, self.decoded_items", "0, 0", 1)));
}

fn completion_rejection(kind: Puzzle5dCompletionOwnerKind, emit: Emit<Puzzle5dMutation, Puzzle5dConfigMutation, NoDraftMutation>) -> Puzzle5dPendingCompletionRejection {
    let mut fault = Fault::from("injected completion rejection");
    fault.scope.plugin_id = Some("s.puzzle".repeat(32));
    Puzzle5dPendingCompletionRejection::new(
        kind,
        ArtifactToolCompletionRejection {
            emit: Ok(emit),
            ephemeral: EphemeralEmit { presence: vec![Puzzle5dPresenceMutation::Snapshot { presence: Puzzle5dPresence { active_utility_id: "select".repeat(32), ..Default::default() } }], transient: Vec::new() },
            fault,
        },
    )
}

fn close_completion_rejection(owner: &mut Puzzle5dPendingCompletionRejection) -> usize {
    assert!(matches!(owner.close_step(0, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 })));
    assert!(owner.owner.is_some(), "zero-item close must preserve the exact rejection owner");
    let mut turns = 0usize;
    loop {
        match owner.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).expect("bounded completion rejection close") {
            PluginCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                turns += 1;
            }
            PluginCloseStep::Complete => break,
            PluginCloseStep::Blocked { reason } => panic!("completion rejection close blocked: {reason}"),
            PluginCloseStep::AwaitingInput { .. } => panic!("completion rejection close cannot await input"),
        }
        assert!(turns < 100_000, "completion rejection close did not converge");
    }
    assert!(owner.owner.is_none());
    turns
}

#[test]
fn copy_completion_rejection_retains_and_incrementally_closes_clipboard_ephemeral_and_fault_owners() {
    let fragment = ClipboardFragment {
        schema: PUZZLE5D_SCHEMA.repeat(32),
        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Design },
        dsl_text: "part".repeat(256),
        pack_bytes: Some(vec![7; 31]),
        source_app: PUZZLE5D_PLAY_APP_ID.repeat(32),
        label: "copy".repeat(32),
    };
    let mut owner = completion_rejection(Puzzle5dCompletionOwnerKind::Copy, Emit { effects: vec![Effect::ClipboardWrite { fragment }], ..Default::default() });
    assert!(close_completion_rejection(&mut owner) > 100, "clipboard strings must not collapse into one owner drop");
}

#[test]
fn cut_completion_rejection_retains_and_incrementally_closes_exact_cut_mutations() {
    let emit = Emit { artifact_mutations: vec![crate::standards::v1::subsets::any::schema::mutations::disconnect_grips("fastener".repeat(64)), crate::standards::v1::subsets::any::schema::mutations::delete_part("part".repeat(64))], ..Default::default() };
    let mut owner = completion_rejection(Puzzle5dCompletionOwnerKind::Cut, emit);
    assert!(close_completion_rejection(&mut owner) > 100);
}

#[test]
fn paste_completion_rejection_retains_original_flattened_mutation_vector_until_bounded_close() {
    let part = crate::Puzzle5dPart {
        id: "part".repeat(64),
        part_kind: Some("kind".repeat(64)),
        part_2d: crate::Puzzle5dPart2d { text: Some("text".repeat(64)), ..Default::default() },
        grips: vec![crate::Puzzle5dGrip { id: "grip".repeat(64), grip_kind: Some("socket".repeat(64)), grip_2d: Default::default(), grip_3d: Default::default() }],
        ..Default::default()
    };
    let emit =
        Emit::mutations(vec![crate::standards::v1::subsets::any::schema::mutations::create_part(part, None), crate::standards::v1::subsets::any::schema::mutations::connect_grips("fastener".repeat(64), "part:source".repeat(32), "part:target".repeat(32), Some("fixed".repeat(32)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)]);
    let original = emit.artifact_mutations.as_ptr();
    let mut owner = completion_rejection(Puzzle5dCompletionOwnerKind::Paste, emit);
    let retained = owner.owner.as_ref().and_then(|rejected| rejected.emit.as_ref().ok()).expect("retained paste emit");
    assert_eq!(retained.artifact_mutations.as_ptr(), original, "paste must retain the completion-returned mutation vector without reconstruction");
    assert!(close_completion_rejection(&mut owner) > 100);
}

#[test]
fn import_completion_rejection_never_repages_and_closes_catalog_mutations_incrementally() {
    let emit = Emit::mutations(vec![
        crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility("source".repeat(64), "target".repeat(64), true, true, crate::Puzzle5dCompatSpecificity::General),
        crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs(Some(crate::Puzzle5dKindCatalogs { parts: vec![crate::Puzzle5dCatalogPartKind { id: "catalog-part".repeat(64), ..Default::default() }], ..Default::default() })),
    ]);
    let original = emit.artifact_mutations.as_ptr();
    let mut owner = completion_rejection(Puzzle5dCompletionOwnerKind::Import, emit);
    let retained = owner.owner.as_ref().and_then(|rejected| rejected.emit.as_ref().ok()).expect("retained import emit");
    assert_eq!(retained.artifact_mutations.as_ptr(), original, "import must retain the completion-returned flattened vector rather than re-page it");
    assert!(close_completion_rejection(&mut owner) > 100);
}

#[test]
fn completion_rejection_guards_precede_all_four_prepare_and_publish_paths() {
    let source = include_str!("../../🦀️.rs");
    for (job, next, guard) in [
        ("\nstruct Puzzle5dCopyJob {", "\nstruct Puzzle5dCutJob {", "puzzle5d copy completion remains rejected"),
        ("\nstruct Puzzle5dCutJob {", "\nenum Puzzle5dPasteStage {", "puzzle5d cut completion remains rejected"),
        ("\nstruct Puzzle5dPasteJob {", "\nenum Puzzle5dImportStage {", "puzzle5d paste completion remains rejected"),
        ("\nstruct Puzzle5dImportJob {", "\n//#endregion 🧵️ReservedJobs", "puzzle5d import completion remains rejected"),
    ] {
        let region = source.split_once(job).and_then(|(_, suffix)| suffix.split_once(next).map(|(region, _)| region)).expect("reserved job region");
        let guard = region.find(guard).expect("pending rejection replay guard");
        let prepare = region.find("commit.prepare").expect("commit prepare");
        let retain = region.find("pending_completion_rejection = Some").expect("exact rejection handoff");
        assert!(guard < prepare && prepare < retain, "{job} must stop replay before preparing or publishing");
        assert!(!region.contains("if let Err(error) = completion.complete"));
    }
}
