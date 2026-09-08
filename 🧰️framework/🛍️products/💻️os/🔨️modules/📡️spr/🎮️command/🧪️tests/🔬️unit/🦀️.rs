use super::mutation_laws_fixture::{foreign_step_fixture, AddCounter, AddCounterFourTimes, AddCounterThenNotifyForeign, AddCounterTwice, CounterDiff, CounterMutation};
use super::*;

fn json_oracle<T: protocol::value::ToValue>(value: &T) -> serde_json::Value {
    serde_json::from_str(&crate::os_pack::json::to_json_string(value)).expect("independent JSON parser accepts first-party value encoding")
}

//#region 🧪️ApplyErrorContract
#[test]
fn mutation_apply_error_json_round_trip_matches_typescript_parity_vector() {
    let error = MutationApplyError::new("mutation.apply.invalid-index", "index 4 exceeds length 2").at(["slides", "4"]);
    let json = crate::os_pack::json::to_json_string(&error);
    assert_eq!(json, r#"{"code":"mutation.apply.invalid-index","message":"index 4 exceeds length 2","target":["slides","4"]}"#);
    assert_eq!(json_oracle(&error), serde_json::json!({"code":"mutation.apply.invalid-index","message":"index 4 exceeds length 2","target":["slides","4"]}));
    assert_eq!(crate::os_pack::json::from_json_str::<MutationApplyError>(&json).expect("decode apply error value"), error);
}

#[test]
fn mutation_apply_error_under_prefixes_without_losing_inner_target() {
    let error = MutationApplyError::new("mutation.apply.missing-target", "vertex missing").at(["vertices", "v1"]).under(["objects", "o1"]);
    assert_eq!(error.target, vec!["objects", "o1", "vertices", "v1"]);
}
//#endregion 🧪️ApplyErrorContract

//#region 🧸️Fixtures
#[derive(Clone, Debug, PartialEq)]
struct Item {
    id: String,
    value: i64,
}
impl Identified<String> for Item {
    // 🚫️async: E1 pure accessor — Identified::id must stay sync, see the trait's own tag.
    fn id(&self) -> &String {
        &self.id
    }
}
impl Patchable<i64> for Item {
    fn apply_patch(&mut self, patch: &i64) {
        self.value += patch;
    }
    fn diff_patch(&self, other: &Self) -> Option<i64> {
        let delta = other.value - self.value;
        if delta == 0 {
            None
        } else {
            Some(delta)
        }
    }
}
//#endregion 🧸️Fixtures

//#region 🧪️MutationLaws
#[test]
fn operation_diff_apply_matches_backwards_inverse() {
    let base: i64 = 10;
    let op = CounterMutation::AddCounter(AddCounter { delta: 5 });
    let forward = op.diff(&base).diff().apply(&base).expect("valid forward diff");
    assert_eq!(forward, 15);
    let [undo] = <[CounterMutation; 1]>::try_from(op.inverse(&base)).unwrap();
    let restored = undo.diff(&forward).diff().apply(&forward).expect("valid inverse diff");
    assert_eq!(restored, base);
}

#[test]
fn operation_diff_absorb_accumulates() {
    let mut a = CounterDiff { deltas: vec![3] };
    a.absorb(CounterDiff { deltas: vec![4] });
    assert_eq!(a.deltas, vec![3, 4]);
    assert_eq!(a.apply(&0), Ok(7));
}

#[test]
fn operation_defaults_are_stable() {
    let op = CounterMutation::AddCounter(AddCounter { delta: 1 });
    assert_eq!(op.mutation_id(), None);
    assert!(op.dependencies().is_empty());
    assert_eq!(op.base_version(), None);
    assert_eq!(op.author_id(), None);
    assert_eq!(op.timestamp(), None);
    assert_eq!(op.undo_policy(), crate::os_spr::UndoPolicy::ExactBaseOnly);
    assert_eq!(op.state_class(), crate::os_spr::StateClass::Artifact);
    assert!(op.foreign_steps(&0).is_empty());
}
//#endregion 🧪️MutationLaws

//#region 🧪️OpTextLaws
#[test]
fn op_text_round_trip() {
    let op = CounterMutation::AddCounter(AddCounter { delta: -7 });
    let line = op.print_op();
    assert!(!line.contains('\n'));
    let parsed = CounterMutation::parse_op(&line).expect("round trip parse");
    assert_eq!(parsed, op);
}

#[test]
fn op_text_parse_error_carries_message() {
    let error = CounterMutation::parse_op("nope").unwrap_err();
    assert!(!error.message.is_empty());
}
//#endregion 🧪️OpTextLaws

//#region 🧪️MetaSerde
#[test]
fn operation_meta_value_round_trip_matches_serde_oracle() {
    let meta = MutationMeta {
        mutation_id: Some(crate::os_spr::ids::MutationId("op-1".into())),
        dependencies: vec![crate::os_spr::ids::MutationId("op-0".into())],
        base_version: 3,
        author_id: Some(crate::os_spr::ids::ActorId("actor-1".into())),
        timestamp: crate::os_spr::ids::HybridLogicalTimestamp::new(1, 1000),
        undo_policy: crate::os_spr::UndoPolicy::TransformAgainstConcurrent,
        payload_hash: Some(crate::os_spr::ids::PayloadHash([7u8; 32])),
        semantic_kind: None,
        label: None,
        group_id: Some("invocation-1".to_string()),
        origin: MutationOrigin::Owner,
    };
    let json = crate::os_pack::json::to_json_string(&meta);
    assert!(json.contains("\"group_id\":\"invocation-1\""), "group_id must serialize under its own field name (MutationMeta has no rename_all), got {json}");
    assert_eq!(json_oracle(&meta)["group_id"], serde_json::json!("invocation-1"));
    let round_tripped: MutationMeta = crate::os_pack::json::from_json_str(&json).expect("decode mutation metadata value");
    assert_eq!(round_tripped, meta, "group_id must round-trip through the value contract exactly like semantic_kind/label");

    let solitary = MutationMeta { group_id: None, ..meta };
    let solitary_json = crate::os_pack::json::to_json_string(&solitary);
    assert!(!solitary_json.contains("group_id"), "a solitary edit's None group_id must be omitted, matching skip_serializing_if on the sibling optional fields");
    let solitary_round_tripped: MutationMeta = crate::os_pack::json::from_json_str(&solitary_json).expect("decode solitary mutation metadata value");
    assert_eq!(solitary_round_tripped, solitary);
}

#[test]
fn edit_value_round_trip_matches_serde_oracle() {
    let edit = Edit::<CounterMutation> {
        id: "edit-1".into(),
        actor: Some("actor-1".into()),
        forwards: vec![CounterMutation::AddCounter(AddCounter { delta: 1 }), CounterMutation::AddCounter(AddCounter { delta: 2 })],
        inverse: vec![CounterMutation::AddCounter(AddCounter { delta: -1 }), CounterMutation::AddCounter(AddCounter { delta: -2 })],
        mutation_meta: vec![MutationMeta {
            mutation_id: None,
            dependencies: Vec::new(),
            base_version: 0,
            author_id: None,
            timestamp: crate::os_spr::ids::HybridLogicalTimestamp::new(1, 0),
            undo_policy: crate::os_spr::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: MutationOrigin::Owner,
        }],
        description: Some("two adds".into()),
        coalesce_key: None,
        sequence_number: 1,
        started_at: "2026-07-27T00:00:00Z".into(),
        finished_at: None,
    };
    let json = crate::os_pack::json::to_json_string(&edit);
    assert_eq!(json_oracle(&edit)["id"], serde_json::json!("edit-1"));
    let round_tripped: Edit<CounterMutation> = crate::os_pack::json::from_json_str(&json).expect("decode edit value");
    assert_eq!(round_tripped, edit);
}
//#endregion 🧪️MetaSerde

//#region 🧪️CollectionLaws
#[test]
fn apply_add_remove_move_patch() {
    let mut items = vec![Item { id: "a".into(), value: 1 }, Item { id: "b".into(), value: 2 }];

    apply_collection_mutation(&mut items, &CollectionMutation::Add { index: 1, item: Item { id: "c".into(), value: 3 } });
    assert_eq!(items.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["a", "c", "b"]);

    apply_collection_mutation(&mut items, &CollectionMutation::Move { id: "c".into(), to_index: 2 });
    assert_eq!(items.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["a", "b", "c"]);

    apply_collection_mutation::<String, Item, i64>(&mut items, &CollectionMutation::Patch { id: "b".into(), patch: 10 });
    assert_eq!(items.iter().find(|i| i.id == "b").unwrap().value, 12);

    apply_collection_mutation(&mut items, &CollectionMutation::Remove { id: "a".into() });
    assert_eq!(items.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["b", "c"]);
}

#[test]
fn invert_collection_operation_round_trips_every_kind() {
    let original = vec![Item { id: "a".into(), value: 1 }, Item { id: "b".into(), value: 2 }];

    let add = CollectionMutation::Add { index: 2, item: Item { id: "c".into(), value: 3 } };
    let mut items = original.clone();
    apply_collection_mutation(&mut items, &add);
    let inverse = inverse_collection_mutation(&original, &add);
    apply_collection_mutation(&mut items, &inverse);
    assert_eq!(items, original);

    let mov = CollectionMutation::<String, Item, i64>::Move { id: "b".into(), to_index: 0 };
    let mut items = original.clone();
    apply_collection_mutation(&mut items, &mov);
    let inverse = inverse_collection_mutation(&original, &mov);
    apply_collection_mutation(&mut items, &inverse);
    assert_eq!(items, original);

    let patch = CollectionMutation::Patch { id: "a".into(), patch: 9i64 };
    let mut items = original.clone();
    apply_collection_mutation(&mut items, &patch);
    let inverse = inverse_collection_mutation(&original, &patch);
    apply_collection_mutation(&mut items, &inverse);
    assert_eq!(items, original);

    let remove = CollectionMutation::<String, Item, i64>::Remove { id: "a".into() };
    let mut items = original.clone();
    apply_collection_mutation(&mut items, &remove);
    let inverse = inverse_collection_mutation(&original, &remove);
    apply_collection_mutation(&mut items, &inverse);
    assert_eq!(items, original);
}

#[test]
fn collection_diff_from_operation_projects_each_kind() {
    let items = vec![Item { id: "a".into(), value: 1 }, Item { id: "b".into(), value: 2 }];

    let add = CollectionMutation::<String, Item, i64>::Add { index: 0, item: Item { id: "c".into(), value: 3 } };
    let diff = collection_diff_from_mutation(&items, &add);
    assert_eq!(diff.added, vec![Item { id: "c".into(), value: 3 }]);
    assert!(diff.removed.is_empty() && diff.modified.is_empty());

    let remove = CollectionMutation::<String, Item, i64>::Remove { id: "a".into() };
    let diff = collection_diff_from_mutation(&items, &remove);
    assert_eq!(diff.removed, vec!["a".to_string()]);

    let patch = CollectionMutation::Patch { id: "b".into(), patch: 5i64 };
    let diff = collection_diff_from_mutation(&items, &patch);
    assert_eq!(diff.modified, vec![ItemPatch { id: "b".into(), patch: 5i64 }]);

    let mov = CollectionMutation::<String, Item, i64>::Move { id: "a".into(), to_index: 1 };
    let diff = collection_diff_from_mutation(&items, &mov);
    assert_eq!(diff.removed, vec!["a".to_string()]);
    assert_eq!(diff.added, vec![Item { id: "a".into(), value: 1 }]);
}
//#endregion 🧪️CollectionLaws

//#region 🧪️DiffKitLaws
#[test]
fn named_apply_removes_patches_then_adds() {
    let mut items = vec![Item { id: "a".into(), value: 1 }, Item { id: "b".into(), value: 2 }, Item { id: "c".into(), value: 3 }];
    let diff = NamedTripleDiff::<String, Item, i64> { removed: vec!["a".into()], modified: vec![ItemPatch { id: "b".into(), patch: 10 }], added: vec![Item { id: "d".into(), value: 4 }] };
    named_apply(&mut items, &diff).expect("valid named diff");
    assert_eq!(items.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["b", "c", "d"]);
    assert_eq!(items.iter().find(|i| i.id == "b").unwrap().value, 12);
}

#[test]
fn named_triple_diff_is_empty_holds() {
    let empty: NamedTripleDiff<String, Item, i64> = NamedTripleDiff::default();
    assert!(empty.is_empty());
    let nonempty = NamedTripleDiff::<String, Item, i64> { added: vec![Item { id: "a".into(), value: 1 }], ..Default::default() };
    assert!(!nonempty.is_empty());
}

#[test]
fn indexed_apply_modifies_removes_descending_then_inserts_ascending() {
    let mut items = vec![Item { id: "a".into(), value: 1 }, Item { id: "b".into(), value: 2 }, Item { id: "c".into(), value: 3 }];
    // BASE state: [a, b, c]. modified targets base index 0 (a). removed targets base index 2 (c).
    // added targets FINAL indices 0 and 2 in the post-remove/pre-add state [a', b].
    let diff = IndexedTripleDiff::<Item, i64> { removed: vec![2], modified: vec![(0, 100)], added: vec![(0, Item { id: "z".into(), value: 9 }), (2, Item { id: "y".into(), value: 8 })] };
    indexed_apply(&mut items, &diff).expect("valid indexed diff");
    assert_eq!(items.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["z", "a", "y", "b"]);
    assert_eq!(items[1].value, 101, "modified applies to BASE-state index 0 (item a) before removal/insertion shift it");
}

#[test]
fn indexed_apply_rejects_duplicate_added_indices_without_mutating() {
    let original = vec![Item { id: "a".into(), value: 1 }];
    let mut items = original.clone();
    let diff = IndexedTripleDiff::<Item, i64> { added: vec![(1, Item { id: "b".into(), value: 2 }), (1, Item { id: "c".into(), value: 3 })], ..Default::default() };
    let error = indexed_apply(&mut items, &diff).expect_err("duplicate final indices must reject");
    assert_eq!(error.code, "mutation.apply.duplicate-target");
    assert_eq!(items, original, "rejected indexed diff must be atomic");
}

#[test]
fn indexed_triple_diff_is_empty_holds() {
    let empty: IndexedTripleDiff<Item, i64> = IndexedTripleDiff::default();
    assert!(empty.is_empty());
    let nonempty = IndexedTripleDiff::<Item, i64> { removed: vec![0], ..Default::default() };
    assert!(!nonempty.is_empty());
}
//#endregion 🧪️DiffKitLaws

//#region 🧪️SemanticsLaws
#[test]
fn str_eq_matches_std_partial_eq() {
    const _: () = {
        let mut index = 0;
        while index < 10000 {
            assert!(!str_eq(
                "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️program/🧩️requirements/🦠️operation/create-a",
                "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️program/🧩️requirements/🦠️operation/create-b"
            ));
            index += 1;
        }
    };
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🔤️string-equality/🔣️.json")).expect("language-neutral equality vectors");
    for row in fixture["cases"].as_array().expect("equality cases") {
        let left = row["left"].as_str().expect("left string");
        let right = row["right"].as_str().expect("right string");
        let actual = str_eq(left, right);
        assert_eq!(actual, row["equal"].as_bool().expect("expected equality"));
        assert_eq!(actual, left == right);
        assert_eq!(actual, row["left"] == row["right"], "independent Serde value equality");
    }
}

#[test]
fn is_approved_verb_matches_the_table() {
    assert!(is_approved_verb("rename"));
    assert!(is_approved_verb("flatten"));
    assert!(!is_approved_verb("set-snapshot"));
    assert!(!is_approved_verb("modify"));
}

#[test]
fn approved_verbs_are_unique_and_lowercase() {
    let mut seen = std::collections::HashSet::new();
    for (verb, record) in APPROVED_VERBS {
        assert_eq!(*verb, verb.to_lowercase(), "verb {verb:?} must be lowercase");
        assert!(seen.insert(*verb), "duplicate verb {verb:?} in APPROVED_VERBS");
        assert!(!record.is_empty(), "verb {verb:?} must have a non-empty past-tense record form");
    }
}

#[test]
fn space_history_verbs_match_the_language_neutral_contract() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🗣️verb-vocabulary/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let verb = case["verb"].as_str().unwrap();
        let expected = case["record"].as_str();
        assert_eq!(APPROVED_VERBS.iter().find(|entry| entry.0 == verb).map(|entry| entry.1), expected);
        assert_eq!(is_approved_verb(verb), expected.is_some());
    }
}

#[test]
fn mutation_descriptor_semantics_participate_in_immutable_identity() {
    use super::registry_fixture::{MiniDoc, MiniMutation, RenameMini};
    let semantics = <RenameMini as MutationKind<MiniDoc, MiniMutation>>::SEMANTICS;
    let construct = |semantics| MutationDescriptor::new(crate::os_spr::SchemaId("mini.doc#rename-mini".into()), crate::os_spr::SchemaVersion(1), crate::os_spr::StateClass::Artifact, RenameMini::DESCRIPTOR, semantics).unwrap();
    let base = construct(semantics);
    let changed = construct(SemanticDescriptor { record: "RenamedMiniLabel", ..semantics });
    assert_ne!(base.fingerprint(), changed.fingerprint());
    assert_eq!(base.semantics(), &semantics);
    assert_eq!(base.leaf(), &RenameMini::DESCRIPTOR);
}
//#endregion 🧪️SemanticsLaws

//#region 🧪️MutationsDeriveLaws
#[test]
fn derive_mutations_wires_complete_leaf_and_atomic_registration() {
    use super::registry_fixture::*;
    let base = MiniDoc { name: "a".into() };
    let mutation: MiniMutation = RenameMini { new_name: "b".into() }.into();
    let after = mutation.diff(&base).diff().apply(&base).expect("valid forward diff");
    assert_eq!(after.name, "b");
    let inverse = mutation.inverse(&base);
    assert_eq!(inverse.len(), 1);
    assert_eq!(inverse[0].diff(&after).diff().apply(&after), Ok(base));
    assert_eq!(MiniMutation::DESCRIPTORS, &[RenameMini::DESCRIPTOR]);
    assert_eq!(mutation.descriptor(), &RenameMini::DESCRIPTOR);
    assert_eq!(MiniMutation::kinds(), &[<RenameMini as MutationKind<MiniDoc, MiniMutation>>::SEMANTICS]);
    assert_eq!(mutation.semantics().record, "RenamedMini");
    assert_eq!(mutation.label(), "Rename mini to \"b\"");
    assert!(mutation.target().is_empty());
    register_mini_mutation_descriptors(crate::os_spr::StateClass::Artifact).unwrap();
    register_mini_mutation_descriptors(crate::os_spr::StateClass::Artifact).unwrap();
    let descriptor = mutation_descriptor("mini.doc#rename-mini").unwrap();
    assert_eq!(descriptor.semantics(), mutation.semantics());
    assert_eq!(descriptor.leaf(), mutation.descriptor());
    let declared: serde_json::Value = serde_json::from_str(include_str!("../📔️registry/🧬️mutations/📛️rename-mini/🔣️.json")).unwrap();
    assert_eq!(json_oracle(descriptor.leaf()), declared);
    assert!(register_mini_mutation_descriptors(crate::os_spr::StateClass::Config).is_err());
    assert_eq!(mutation_descriptor("mini.doc#rename-mini"), Some(descriptor));
}
//#endregion 🧪️MutationsDeriveLaws

//#region 🧪️DescriptorLaws
#[test]
fn descriptor_registry_rejects_conflicts_without_partial_publication() {
    use super::registry_fixture::{MiniDoc, MiniMutation, RenameMini};
    let build = |id: &str, state| MutationDescriptor::new(crate::os_spr::SchemaId(id.into()), crate::os_spr::SchemaVersion(1), state, RenameMini::DESCRIPTOR, <RenameMini as MutationKind<MiniDoc, MiniMutation>>::SEMANTICS).unwrap();
    let first = build("mini.first", crate::os_spr::StateClass::Artifact);
    let conflict = build("mini.first", crate::os_spr::StateClass::Config);
    let second = build("mini.second", crate::os_spr::StateClass::Artifact);
    assert_ne!(first.fingerprint(), conflict.fingerprint());
    let mut registry = MutationDescriptorRegistry::new();
    assert!(registry.register_all([first.clone(), conflict.clone()]).is_err());
    assert!(registry.is_empty());
    registry.register(first.clone()).unwrap();
    assert!(registry.register_all([second.clone(), conflict]).is_err());
    assert_eq!(registry.len(), 1);
    assert_eq!(registry.get("mini.first"), Some(&first));
    assert!(registry.get("mini.second").is_none());
    registry.register_all([first.clone(), second.clone(), second]).unwrap();
    assert_eq!(registry.len(), 2);
    assert_eq!(registry.get("mini.first"), Some(&first));
}
//#endregion 🧪️DescriptorLaws

//#region 🧪️MutationLeafDescriptorLaws
fn mutation_leaf_descriptor_fixture() -> MutationLeafDescriptor {
    MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page",
        semantic_kind: "insert-page",
        display_name: "Insert Page",
        emoji: "➕️",
        aggregate_variant: "InsertPage",
        payload_schema: "🦀️.rs#InsertPage",
        text_opcode: None,
        binary_tag: None,
        invertibility: MutationInvertibility::ExplicitMutation,
        diff_participation: MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[MutationOutcomeClass::Applied],
        composition: MutationComposition::Atomic,
        required_language_surfaces: &[MutationLanguageSurface::Rust],
    }
}

fn mutation_leaf_descriptor_fixture_json() -> serde_json::Value {
    serde_json::from_str(include_str!("../🪪️mutation-leaf-descriptor/🧫️fixtures/🔣️.json")).expect("valid neutral descriptor fixture")
}

static MUTATION_LEAF_DESCRIPTOR_DUPLICATE_OUTCOMES: [MutationOutcomeClass; 2] = [MutationOutcomeClass::Applied, MutationOutcomeClass::Applied];
static MUTATION_LEAF_DESCRIPTOR_NON_RUST_SURFACES: [MutationLanguageSurface; 1] = [MutationLanguageSurface::Text];
static MUTATION_LEAF_DESCRIPTOR_DUPLICATE_SURFACES: [MutationLanguageSurface; 2] = [MutationLanguageSurface::Rust, MutationLanguageSurface::Rust];
static MUTATION_LEAF_DESCRIPTOR_OWNER_BOUNDARIES: [(&str, &str, bool); 5] = [
    ("unicode-line-separator", "prefix\u{2028}/🧬️mutations/➕️insert-page", false),
    ("unicode-paragraph-separator", "prefix\u{2029}/🧬️mutations/➕️insert-page", false),
    ("multiple-markers-later-valid", "/🧬️mutations/first/🧬️mutations/second", true),
    ("multiple-markers-prefixed", "prefix/🧬️mutations/second/🧬️mutations/third", true),
    ("marker-without-suffix", "prefix/🧬️mutations/", false),
];
static MUTATION_LEAF_DESCRIPTOR_OUTCOMES: [MutationOutcomeClass; 1] = [MutationOutcomeClass::Applied];
static MUTATION_LEAF_DESCRIPTOR_SURFACES: [MutationLanguageSurface; 1] = [MutationLanguageSurface::Rust];
static MUTATION_LEAF_DESCRIPTOR_ROSTER_ROOT: &str = "✏️s/🔌️plugins/🧪️probe/🧬️mutations";
static MUTATION_LEAF_DESCRIPTOR_ROSTER_INSERT: MutationLeafDescriptor = MutationLeafDescriptor {
    schema_version: 1,
    owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page",
    semantic_kind: "insert-page",
    display_name: "Insert Page",
    emoji: "➕️",
    aggregate_variant: "InsertPage",
    payload_schema: "🦀️.rs#InsertPage",
    text_opcode: Some("insert-page"),
    binary_tag: Some(1),
    invertibility: MutationInvertibility::ExplicitMutation,
    diff_participation: MutationDiffParticipation::ApplyOnly,
    outcome_classes: &MUTATION_LEAF_DESCRIPTOR_OUTCOMES,
    composition: MutationComposition::Atomic,
    required_language_surfaces: &MUTATION_LEAF_DESCRIPTOR_SURFACES,
};
static MUTATION_LEAF_DESCRIPTOR_ROSTER_REMOVE: MutationLeafDescriptor = MutationLeafDescriptor {
    schema_version: 1,
    owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➖️remove-page",
    semantic_kind: "remove-page",
    display_name: "Remove Page",
    emoji: "➖️",
    aggregate_variant: "RemovePage",
    payload_schema: "🦀️.rs#RemovePage",
    text_opcode: Some("remove-page"),
    binary_tag: Some(2),
    invertibility: MutationInvertibility::ExplicitMutation,
    diff_participation: MutationDiffParticipation::ApplyOnly,
    outcome_classes: &MUTATION_LEAF_DESCRIPTOR_OUTCOMES,
    composition: MutationComposition::Atomic,
    required_language_surfaces: &MUTATION_LEAF_DESCRIPTOR_SURFACES,
};
static MUTATION_LEAF_DESCRIPTOR_ROSTER_NULLABLE: MutationLeafDescriptor = MutationLeafDescriptor {
    schema_version: 1,
    owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/♻️replace-page",
    semantic_kind: "replace-page",
    display_name: "Replace Page",
    emoji: "♻️",
    aggregate_variant: "ReplacePage",
    payload_schema: "🦀️.rs#ReplacePage",
    text_opcode: None,
    binary_tag: None,
    invertibility: MutationInvertibility::ExplicitMutation,
    diff_participation: MutationDiffParticipation::ApplyOnly,
    outcome_classes: &MUTATION_LEAF_DESCRIPTOR_OUTCOMES,
    composition: MutationComposition::Atomic,
    required_language_surfaces: &MUTATION_LEAF_DESCRIPTOR_SURFACES,
};
static MUTATION_LEAF_DESCRIPTOR_ROSTER_OTHER_OWNER: MutationLeafDescriptor = MutationLeafDescriptor {
    schema_version: 1,
    owner: "✏️s/🔌️plugins/🧪️other/🧬️mutations/➕️insert-page",
    semantic_kind: "insert-page",
    display_name: "Insert Page",
    emoji: "➕️",
    aggregate_variant: "InsertPage",
    payload_schema: "🦀️.rs#InsertPage",
    text_opcode: Some("insert-page"),
    binary_tag: Some(1),
    invertibility: MutationInvertibility::ExplicitMutation,
    diff_participation: MutationDiffParticipation::ApplyOnly,
    outcome_classes: &MUTATION_LEAF_DESCRIPTOR_OUTCOMES,
    composition: MutationComposition::Atomic,
    required_language_surfaces: &MUTATION_LEAF_DESCRIPTOR_SURFACES,
};
static MUTATION_LEAF_DESCRIPTOR_ROSTER_UNIQUE: [MutationLeafDescriptor; 2] = [MUTATION_LEAF_DESCRIPTOR_ROSTER_INSERT, MUTATION_LEAF_DESCRIPTOR_ROSTER_REMOVE];
static MUTATION_LEAF_DESCRIPTOR_ROSTER_DUPLICATE_SEMANTIC: [MutationLeafDescriptor; 2] = [MUTATION_LEAF_DESCRIPTOR_ROSTER_INSERT, MutationLeafDescriptor { semantic_kind: "insert-page", ..MUTATION_LEAF_DESCRIPTOR_ROSTER_REMOVE }];
static MUTATION_LEAF_DESCRIPTOR_ROSTER_DUPLICATE_OPCODE: [MutationLeafDescriptor; 2] = [MUTATION_LEAF_DESCRIPTOR_ROSTER_INSERT, MutationLeafDescriptor { text_opcode: Some("insert-page"), ..MUTATION_LEAF_DESCRIPTOR_ROSTER_REMOVE }];
static MUTATION_LEAF_DESCRIPTOR_ROSTER_DUPLICATE_TAG: [MutationLeafDescriptor; 2] = [MUTATION_LEAF_DESCRIPTOR_ROSTER_INSERT, MutationLeafDescriptor { binary_tag: Some(1), ..MUTATION_LEAF_DESCRIPTOR_ROSTER_REMOVE }];
static MUTATION_LEAF_DESCRIPTOR_ROSTER_NULLABLE_REPEAT: [MutationLeafDescriptor; 2] = [
    MUTATION_LEAF_DESCRIPTOR_ROSTER_NULLABLE,
    MutationLeafDescriptor {
        owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/🗄️archive-page",
        semantic_kind: "archive-page",
        display_name: "Archive Page",
        emoji: "🗄️",
        aggregate_variant: "ArchivePage",
        payload_schema: "🦀️.rs#ArchivePage",
        ..MUTATION_LEAF_DESCRIPTOR_ROSTER_NULLABLE
    },
];
static MUTATION_LEAF_DESCRIPTOR_ROSTER_OWNER_MISMATCH: [MutationLeafDescriptor; 2] = [MUTATION_LEAF_DESCRIPTOR_ROSTER_INSERT, MUTATION_LEAF_DESCRIPTOR_ROSTER_OTHER_OWNER];
static MUTATION_LEAF_DESCRIPTOR_ROSTER_DUPLICATE_OWNER: [MutationLeafDescriptor; 2] = [
    MUTATION_LEAF_DESCRIPTOR_ROSTER_INSERT,
    MutationLeafDescriptor {
        semantic_kind: "restore-page",
        display_name: "Restore Page",
        emoji: "↩️",
        aggregate_variant: "RestorePage",
        payload_schema: "🦀️.rs#RestorePage",
        text_opcode: Some("restore-page"),
        binary_tag: Some(3),
        ..MUTATION_LEAF_DESCRIPTOR_ROSTER_INSERT
    },
];
static MUTATION_LEAF_DESCRIPTOR_ROSTER_NESTED_OWNER: [MutationLeafDescriptor; 1] = [MutationLeafDescriptor { owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➖️remove-page/🧪️tests", ..MUTATION_LEAF_DESCRIPTOR_ROSTER_REMOVE }];
static MUTATION_LEAF_DESCRIPTOR_ROSTER_PARENT_CHILD: [MutationLeafDescriptor; 1] = [MutationLeafDescriptor { owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/..", ..MUTATION_LEAF_DESCRIPTOR_ROSTER_REMOVE }];
static MUTATION_LEAF_DESCRIPTOR_ROSTER_BACKSLASH_CHILD: [MutationLeafDescriptor; 1] =
    [MutationLeafDescriptor { owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➖️remove-page\\🧪️tests", ..MUTATION_LEAF_DESCRIPTOR_ROSTER_REMOVE }];
static MUTATION_LEAF_DESCRIPTOR_ROSTER_NUL_CHILD: [MutationLeafDescriptor; 1] = [MutationLeafDescriptor { owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➖️remove-page\0", ..MUTATION_LEAF_DESCRIPTOR_ROSTER_REMOVE }];
static MUTATION_LEAF_DESCRIPTOR_ROSTER_OTHER_OWNER_SINGLE: [MutationLeafDescriptor; 1] = [MUTATION_LEAF_DESCRIPTOR_ROSTER_OTHER_OWNER];
const MUTATION_LEAF_DESCRIPTOR_CONST_VALID: Result<(), MutationLeafDescriptorValidationError> = validate_mutation_leaf_descriptor(&MUTATION_LEAF_DESCRIPTOR_ROSTER_INSERT);
const MUTATION_LEAF_DESCRIPTOR_CONST_ROSTER_VALID: Result<(), MutationLeafDescriptorRosterValidationError> =
    validate_mutation_leaf_descriptor_roster(MUTATION_LEAF_DESCRIPTOR_ROSTER_ROOT, &MUTATION_LEAF_DESCRIPTOR_ROSTER_UNIQUE, MutationOwnerLayout::Flat);

#[test]
fn mutation_leaf_descriptor_serializes_all_schema_fields() {
    let descriptor = mutation_leaf_descriptor_fixture();
    let fixture = mutation_leaf_descriptor_fixture_json();
    assert_eq!(validate_mutation_leaf_descriptor(&descriptor), Ok(()));
    let serialized = json_oracle(&descriptor);
    assert_eq!(serialized, fixture["descriptor"]);
    assert_eq!(serialized.as_object().expect("descriptor object").len(), 14);
    assert!(serialized.get("textOpcode").expect("required nullable text opcode").is_null());
    assert!(serialized.get("binaryTag").expect("required nullable binary tag").is_null());
}

#[test]
fn mutation_leaf_descriptor_enum_wires_match_neutral_fixture() {
    let fixture = mutation_leaf_descriptor_fixture_json();
    let wires = serde_json::json!({
        "invertibility": json_oracle(&vec![MutationInvertibility::SelfInvertible, MutationInvertibility::ExplicitMutation, MutationInvertibility::Plan, MutationInvertibility::NonInvertible]),
        "diffParticipation": json_oracle(&vec![MutationDiffParticipation::Detect, MutationDiffParticipation::ApplyOnly, MutationDiffParticipation::Plan, MutationDiffParticipation::None]),
        "outcomeClasses": json_oracle(&vec![MutationOutcomeClass::Applied, MutationOutcomeClass::Info, MutationOutcomeClass::Warning, MutationOutcomeClass::Error, MutationOutcomeClass::Fatal]),
        "composition": json_oracle(&vec![MutationComposition::Atomic, MutationComposition::Composite]),
        "requiredLanguageSurfaces": json_oracle(&vec![MutationLanguageSurface::Rust, MutationLanguageSurface::Typescript, MutationLanguageSurface::Graphql, MutationLanguageSurface::Protobuf, MutationLanguageSurface::JsonSchema, MutationLanguageSurface::Text, MutationLanguageSurface::Binary]),
    });
    assert_eq!(wires, fixture["enumWireValues"]);
}

#[test]
fn mutation_leaf_descriptor_validates_static_schema_boundaries() {
    let fixture = mutation_leaf_descriptor_fixture_json();
    for vector in fixture["binaryTagVectors"].as_array().expect("vector array") {
        let expected = vector["expected"].as_bool().expect("expected boolean");
        let actual = match vector.get("value") {
            Some(serde_json::Value::Null) => Some(None),
            Some(serde_json::Value::Number(value)) => value.as_u64().and_then(|value| u32::try_from(value).ok()).map(Some),
            _ => None,
        };
        assert_eq!(actual.is_some(), expected, "{}", vector["name"]);
        if let Some(binary_tag) = actual {
            let mut descriptor = mutation_leaf_descriptor_fixture();
            descriptor.binary_tag = binary_tag;
            assert_eq!(descriptor.validate(), Ok(()), "{}", vector["name"]);
        }
    }
    let descriptor = mutation_leaf_descriptor_fixture();
    assert!(json_oracle(&descriptor).get("binaryTag").is_some(), "binaryTag cannot be omitted from the static descriptor shape");

    let mut invalid = descriptor;
    invalid.schema_version = 2;
    assert_eq!(invalid.validate().expect_err("schema version must be one").field, "schemaVersion");
    invalid = descriptor;
    invalid.owner = "compose/🧬️mutations/➕️insert-page";
    assert_eq!(invalid.validate().expect_err("compose owner is excluded").field, "owner");
    invalid = descriptor;
    invalid.owner = "owner-without-mutation-root";
    assert_eq!(invalid.validate().expect_err("owner must name a direct mutation leaf").field, "owner");
    invalid = descriptor;
    invalid.semantic_kind = "insert";
    assert_eq!(invalid.validate().expect_err("semantic kind needs two kebab segments").field, "semanticKind");
    invalid = descriptor;
    invalid.display_name = "";
    assert_eq!(invalid.validate().expect_err("display name is required").field, "displayName");
    invalid = descriptor;
    invalid.emoji = "";
    assert_eq!(invalid.validate().expect_err("emoji is required").field, "emoji");
    invalid = descriptor;
    invalid.aggregate_variant = "insert_page";
    assert_eq!(invalid.validate().expect_err("aggregate variant is Pascal identifier").field, "aggregateVariant");
    invalid = descriptor;
    invalid.payload_schema = "";
    assert_eq!(invalid.validate().expect_err("payload schema is required").field, "payloadSchema");
    invalid = descriptor;
    invalid.text_opcode = Some("insert");
    assert_eq!(invalid.validate().expect_err("text opcode is kebab semantic kind").field, "textOpcode");
    invalid = descriptor;
    invalid.outcome_classes = &[];
    assert_eq!(invalid.validate().expect_err("outcome classes are required").field, "outcomeClasses");
    invalid.outcome_classes = &MUTATION_LEAF_DESCRIPTOR_DUPLICATE_OUTCOMES;
    assert_eq!(invalid.validate().expect_err("outcome classes are unique").field, "outcomeClasses");
    invalid = descriptor;
    invalid.required_language_surfaces = &[];
    assert_eq!(invalid.validate().expect_err("language surfaces are required").field, "requiredLanguageSurfaces");
    invalid.required_language_surfaces = &MUTATION_LEAF_DESCRIPTOR_NON_RUST_SURFACES;
    assert_eq!(invalid.validate().expect_err("rust surface is required").field, "requiredLanguageSurfaces");
    invalid.required_language_surfaces = &MUTATION_LEAF_DESCRIPTOR_DUPLICATE_SURFACES;
    assert_eq!(invalid.validate().expect_err("language surfaces are unique").field, "requiredLanguageSurfaces");
}

#[test]
fn mutation_leaf_descriptor_owner_boundaries_match_neutral_vectors() {
    let fixture = mutation_leaf_descriptor_fixture_json();
    let neutral: Vec<(&str, &str, bool)> =
        fixture["ownerBoundaryVectors"].as_array().expect("owner boundary vectors").iter().map(|vector| (vector["name"].as_str().expect("name"), vector["owner"].as_str().expect("owner"), vector["expected"].as_bool().expect("expected"))).collect();
    assert_eq!(neutral, MUTATION_LEAF_DESCRIPTOR_OWNER_BOUNDARIES);
    for (name, owner, expected) in MUTATION_LEAF_DESCRIPTOR_OWNER_BOUNDARIES {
        let mut descriptor = mutation_leaf_descriptor_fixture();
        descriptor.owner = owner;
        assert_eq!(descriptor.validate().is_ok(), expected, "{name}");
    }
}

#[test]
fn mutation_leaf_descriptor_const_roster_boundaries_match_neutral_vectors() {
    assert_eq!(MUTATION_LEAF_DESCRIPTOR_CONST_VALID, Ok(()));
    assert_eq!(MUTATION_LEAF_DESCRIPTOR_CONST_ROSTER_VALID, Ok(()));
    let fixture = mutation_leaf_descriptor_fixture_json();
    let names: Vec<(&str, bool)> = fixture["rosterVectors"].as_array().expect("roster vectors").iter().map(|vector| (vector["name"].as_str().expect("name"), vector["expected"].as_bool().expect("expected"))).collect();
    assert_eq!(
        names,
        [
            ("same-owner-unique", true),
            ("duplicate-semantic-kind", false),
            ("duplicate-text-opcode", false),
            ("duplicate-binary-tag", false),
            ("nullable-identities-repeat", true),
            ("unrelated-owner", false),
            ("duplicate-owner", false),
            ("nested-child", false),
            ("parent-child", false),
            ("backslash-child", false),
            ("absolute-root", false),
            ("windows-root", false),
            ("windows-slash-drive-root", false),
            ("windows-relative-drive-root", false),
            ("empty-segment-root", false),
            ("dot-root", false),
            ("parent-root", false),
            ("nul-root", false),
            ("nul-child", false),
            ("distinct-owner-same-identities", true)
        ]
    );
    let root = MUTATION_LEAF_DESCRIPTOR_ROSTER_ROOT;
    assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_UNIQUE, MutationOwnerLayout::Flat), Ok(()));
    assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_DUPLICATE_SEMANTIC, MutationOwnerLayout::Flat).expect_err("duplicate semantic kind").field, "semanticKind");
    assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_DUPLICATE_OPCODE, MutationOwnerLayout::Flat).expect_err("duplicate text opcode").field, "textOpcode");
    assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_DUPLICATE_TAG, MutationOwnerLayout::Flat).expect_err("duplicate binary tag").field, "binaryTag");
    assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_NULLABLE_REPEAT, MutationOwnerLayout::Flat), Ok(()));
    assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_OWNER_MISMATCH, MutationOwnerLayout::Flat).expect_err("unrelated owner").field, "owner");
    assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_DUPLICATE_OWNER, MutationOwnerLayout::Flat).expect_err("duplicate owner").field, "owner");
    assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_NESTED_OWNER, MutationOwnerLayout::Flat).expect_err("nested child").field, "owner");
    assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_PARENT_CHILD, MutationOwnerLayout::Flat).expect_err("parent child").field, "owner");
    assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_BACKSLASH_CHILD, MutationOwnerLayout::Flat).expect_err("backslash child").field, "owner");
    assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_NUL_CHILD, MutationOwnerLayout::Flat).expect_err("nul child").field, "owner");
    for unsafe_root in [
        "/✏️s/🔌️plugins/🧪️probe/🧬️mutations",
        "C:\\✏️s\\🔌️plugins\\🧪️probe\\🧬️mutations",
        "C:/✏️s/🔌️plugins/🧪️probe/🧬️mutations",
        "C:✏️s/🔌️plugins/🧪️probe/🧬️mutations",
        "✏️s//🔌️plugins/🧪️probe/🧬️mutations",
        "./✏️s/🔌️plugins/🧪️probe/🧬️mutations",
        "../✏️s/🔌️plugins/🧪️probe/🧬️mutations",
        "✏️s/\0🔌️plugins/🧪️probe/🧬️mutations",
    ] {
        assert_eq!(validate_mutation_leaf_descriptor_roster(unsafe_root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_UNIQUE, MutationOwnerLayout::Flat).expect_err("unsafe root").field, "owner");
    }
    assert_eq!(validate_mutation_leaf_descriptor_roster("✏️s/🔌️plugins/🧪️other/🧬️mutations", &MUTATION_LEAF_DESCRIPTOR_ROSTER_OTHER_OWNER_SINGLE, MutationOwnerLayout::Flat), Ok(()));
}
//#endregion 🧪️MutationLeafDescriptorLaws

//#region 🧪️UpcastLaws
// Clamp-to-floor is the simplest genuinely idempotent upcaster: `upcast(upcast(x)) ==
// upcast(x)` holds because `max(max(x, 10), 10) == max(x, 10)` for every `x`.
struct ClampToFloor;
impl MutationUpcaster<i64> for ClampToFloor {
    fn upcast(&self, _from_version: crate::os_spr::ids::SchemaVersion, op: i64) -> i64 {
        op.max(10)
    }
}

#[test]
fn upcaster_is_idempotent_at_target_version() {
    let upcaster = ClampToFloor;
    let version = crate::os_spr::ids::SchemaVersion(1);
    for start in [0i64, 3, 7, 10, 40] {
        let once = upcaster.upcast(version, start);
        let twice = upcaster.upcast(version, once);
        assert_eq!(once, twice);
    }
}
//#endregion 🧪️UpcastLaws

//#region 🧪️InferenceLaws
// Smallest possible (P=i64) Inference/InferenceSpec/DiffRegions fixture: infers "is_even" and
// "abs_value" from an i64 snapshot, reusing the same CounterDiff/CounterMutation pair as the Mutation laws
// above so this proves the inference traits interoperate with the existing diff/mutation shape.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
struct AddInference {
    is_even: bool,
    abs_value: i64,
}
// 🎯️ Hand-written (not derived) `Default`: it must equal `infer(&i64::default())` for the
// default law to hold, and `0.is_even() == true` disagrees with `bool::default() == false`.
impl Default for AddInference {
    fn default() -> Self {
        AddInference { is_even: true, abs_value: 0 }
    }
}
impl Inference<i64> for AddInference {
    fn infer(snapshot: &i64) -> Self {
        AddInference { is_even: snapshot % 2 == 0, abs_value: snapshot.abs() }
    }
}
impl InferenceSpec<i64> for AddInference {
    fn inference_schema_id() -> &'static str {
        "s.wave3.synthetic.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [InferenceFieldSpec] {
        &[InferenceFieldSpec { id: "isEven", reads: &["value"] }, InferenceFieldSpec { id: "absValue", reads: &["value"] }]
    }
}

#[test]
fn inference_determinism_law() {
    let base: i64 = 42;
    assert_eq!(AddInference::infer(&base), AddInference::infer(&base));
    let json_a = serde_json::to_string(&AddInference::infer(&base)).unwrap();
    let json_b = serde_json::to_string(&AddInference::infer(&42)).unwrap();
    assert_eq!(json_a, json_b, "equal snapshots must infer byte-equal canonical serializations");
    assert_eq!(json_oracle(&AddInference::infer(&base)), serde_json::from_str::<serde_json::Value>(&json_a).unwrap());
}

#[test]
fn inference_default_law() {
    assert_eq!(AddInference::infer(&i64::default()), AddInference::default());
}

#[test]
fn inference_diff_consistency_law() {
    let base: i64 = 10;
    let noop = CounterDiff { deltas: vec![0] };
    assert!(!noop.touches().intersects_any(AddInference::fields()[0].reads));
    assert_eq!(AddInference::infer(&noop.apply(&base).expect("valid no-op diff")), AddInference::infer(&base));

    let real = CounterDiff { deltas: vec![1] };
    assert!(real.touches().intersects_any(AddInference::fields()[0].reads));
    assert_ne!(AddInference::infer(&real.apply(&base).expect("valid real diff")), AddInference::infer(&base));
}

#[test]
fn inference_spec_carries_schema_identity() {
    assert_eq!(AddInference::inference_schema_id(), "s.wave3.synthetic.inference");
    assert_eq!(AddInference::schema_version(), 1);
    assert_eq!(AddInference::fields().len(), 2);
}

#[test]
fn touched_paths_intersects_ancestor_and_descendant_prefixes() {
    let coarse = TouchedPaths::new(["objects"]);
    assert!(coarse.intersects_prefix("objects/o1/vortices"), "a coarse write region must cover a finer read region beneath it");

    let fine = TouchedPaths::new(["objects/o1/vortices"]);
    assert!(fine.intersects_prefix("objects"), "a finer write region must still be caught by a coarser read region above it");

    let unrelated = TouchedPaths::new(["objects/o1"]);
    assert!(!unrelated.intersects_prefix("attractions"), "disjoint subtrees must not intersect");
}

#[test]
fn touched_paths_intersects_any_matches_first_hit() {
    let touched = TouchedPaths::new(["attractions/a1"]);
    assert!(touched.intersects_any(&["objects", "attractions"]));
    assert!(!touched.intersects_any(&["objects", "vortices"]));
}

#[test]
fn touched_paths_default_is_empty_and_intersects_nothing() {
    let empty = TouchedPaths::default();
    assert!(empty.paths.is_empty());
    assert!(!empty.intersects_prefix("anything"));
}
//#endregion 🧪️InferenceLaws

//#region 🧪️OutcomeLaws
#[test]
fn command_outcome_default_is_empty() {
    let outcome: CommandOutcome<CounterDiff> = CommandOutcome::default();
    assert!(outcome.persistent.is_empty());
    assert!(outcome.shared_ui.is_empty());
    assert!(outcome.local_ui.is_empty());
    assert!(outcome.preview.is_empty());
    assert!(outcome.effects.is_empty());
}

#[test]
fn operation_event_serde_round_trip() {
    let event = MutationEvent {
        mutation_id: crate::os_spr::ids::MutationId("op-1".into()),
        state_class: crate::os_spr::StateClass::Transient,
        payload: protocol::value::DslValue::object([("kind".to_string(), protocol::value::DslValue::String("toast".to_string())), ("text".to_string(), protocol::value::DslValue::String("saved".to_string()))]),
    };
    let json = crate::os_pack::json::to_json_string(&event);
    let round_tripped: MutationEvent = crate::os_pack::json::from_json_str(&json).expect("deserialize");
    assert_eq!(round_tripped, event);
}
//#endregion 🧪️OutcomeLaws

//#region 🧪️CompositeLaws
#[test]
fn fold_plan_diff_equals_sequential_apply() {
    let base: i64 = 10;
    let kind = AddCounterTwice { delta: 3 };
    let diff = fold_plan_diff(&kind, &base);
    assert_eq!(diff.diff().apply(&base), Ok(16));

    let steps = plan_of(&kind, &base).expect("plan succeeds");
    let mut sequential = base;
    for step in &steps {
        if let PlanStep::Local(op) = step {
            sequential = op.diff(&sequential).diff().apply(&sequential).expect("valid planned diff");
        }
    }
    assert_eq!(diff.diff().apply(&base), Ok(sequential), "fold_plan_diff must equal sequential application of the plan's local steps");
}

#[test]
fn fold_plan_inverse_restores_base() {
    let base: i64 = 10;
    let kind = AddCounterTwice { delta: 3 };
    let forward = fold_plan_diff(&kind, &base).diff().apply(&base).expect("valid folded diff");
    assert_ne!(forward, base);
    let inverses = fold_plan_inverse(&kind, &base);
    let mut restored = forward;
    for op in inverses.iter().rev() {
        restored = op.diff(&restored).diff().apply(&restored).expect("valid inverse diff");
    }
    assert_eq!(restored, base, "fold_plan_inverse applied after the composite must restore base");
}

#[test]
fn composite_of_composite_nests_and_folds_identically_to_flattened_plan() {
    let base: i64 = 0;
    let quad = AddCounterFourTimes { delta: 2 };
    let diff = fold_plan_diff(&quad, &base);
    assert_eq!(diff.diff().apply(&base), Ok(8), "two nested AddCounterTwice{{delta:2}} embeds must fold to +8");

    let steps = plan_of(&quad, &base).expect("plan succeeds");
    let local_deltas: Vec<i64> = steps
        .iter()
        .filter_map(|step| match step {
            PlanStep::Local(CounterMutation::AddCounter(op)) => Some(op.delta),
            _ => None,
        })
        .collect();
    assert_eq!(local_deltas, vec![2, 2, 2, 2], "nesting must flatten to four local steps, identical to the un-nested plan");

    let inverses = fold_plan_inverse(&quad, &base);
    let mut restored = diff.diff().apply(&base).expect("valid folded diff");
    for op in inverses.iter().rev() {
        restored = op.diff(&restored).diff().apply(&restored).expect("valid inverse diff");
    }
    assert_eq!(restored, base);
}

#[test]
fn plan_depth_beyond_max_is_typed_error_never_panics() {
    let base: i64 = 0;
    let kind = AddCounterThenNotifyForeign { delta: 1, foreign_count: MAX_PLAN_DEPTH + 1 };
    let error = plan_of(&kind, &base).expect_err("a plan with more foreign hops than MAX_PLAN_DEPTH must be rejected, not panic");
    assert_eq!(error, PlanError::DepthExceeded(MAX_PLAN_DEPTH));
}

#[test]
fn plan_cycle_is_typed_error_never_panics() {
    let base: i64 = 0;
    let mut planner: Planner<i64, CounterMutation> = Planner::new(&base);
    planner.call_foreign(foreign_step_fixture(0)).expect("first hop to a fresh target succeeds");
    let error = planner.call_foreign(foreign_step_fixture(0)).expect_err("repeating the identical (mutation_id, payload) pair must be rejected as a cycle, not panic");
    assert_eq!(error, PlanError::Cycle("artifact-0".to_string()));
}

#[test]
fn foreign_steps_are_excluded_from_fold_plan_diff() {
    let base: i64 = 5;
    let kind = AddCounterThenNotifyForeign { delta: 4, foreign_count: 2 };
    let diff = fold_plan_diff(&kind, &base);
    assert_eq!(diff.diff().apply(&base), Ok(9), "only the local AddCounter{{delta:4}} may contribute to the folded diff");

    let foreign = plan_foreign_steps(&kind, &base);
    assert_eq!(foreign.len(), 2);
    assert_eq!(foreign[0].target.artifact_id, "artifact-0");
    assert_eq!(foreign[1].target.artifact_id, "artifact-1");
}

#[test]
fn derive_composite_mutation_wires_delegating_mutation_kind() {
    let base: i64 = 1;
    let kind = AddCounterTwice { delta: 5 };
    let diff = MutationKind::<i64, CounterMutation>::diff(&kind, &base);
    assert_eq!(diff.diff().apply(&base), Ok(11));
    let inverse = MutationKind::<i64, CounterMutation>::inverse(&kind, &base);
    let mut restored = diff.diff().apply(&base).expect("valid folded diff");
    for op in inverse.iter().rev() {
        restored = op.diff(&restored).diff().apply(&restored).expect("valid inverse diff");
    }
    assert_eq!(restored, base);
    assert_eq!(<AddCounterTwice as MutationKind<i64, CounterMutation>>::SEMANTICS.kind, "add-counter-twice");
    assert!(MutationKind::<i64, CounterMutation>::foreign_steps(&kind, &base).is_empty());
}
//#endregion 🧪️CompositeLaws
