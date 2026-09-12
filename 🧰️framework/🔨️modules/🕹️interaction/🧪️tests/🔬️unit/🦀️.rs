
use super::*;

#[semio_framework_async_macros::async_test]
async fn interaction_definition_round_trips_through_json() {
    let def = InteractionDefinition {
        id: "graph".into(),
        label: LocalizedLabel::native("Graph", "Graph"),
        granularities: vec![
            GranularityDefinition { id: "node".into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() },
            GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "minus".into() },
        ],
        hierarchy: HierarchyProvider::Topology,
        hover: HoverSpec::default(),
        selection: SelectionSpec {
            modes: vec![SelectionMode::Multiple, SelectionMode::Single],
            methods: vec![SelectionMethod::Pick],
            merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
            transitive: false,
            broadcast: true,
        },
    };
    let json = serde_json::to_string(&def).expect("serializes");
    assert!(json.contains("\"iconId\""), "{json}");
    assert!(json.contains("\"granularities\""), "{json}");
    let parsed: InteractionDefinition = serde_json::from_str(&json).expect("deserializes");
    assert_eq!(parsed, def);
}

#[semio_framework_async_macros::async_test]
async fn outline_projects_id_granularity_ids_and_selection_only() {
    let def = InteractionDefinition {
        id: "graph".into(),
        label: LocalizedLabel::native("Graph", "Graph"),
        granularities: vec![
            GranularityDefinition { id: "node".into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() },
            GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "minus".into() },
        ],
        hierarchy: HierarchyProvider::Flat,
        hover: HoverSpec::default(),
        selection: SelectionSpec { modes: vec![SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
    };
    let outline = def.outline().await;
    assert_eq!(outline.id, "graph");
    assert_eq!(outline.granularity_ids, vec!["node".to_string(), "edge".to_string()]);
    assert_eq!(outline.selection, def.selection);
}

//#region 🔖️MergeVocabularyLaws
/// 🎯️ The ONE selection merge vocabulary, read from `🧫️fixtures/🎯️merge-modes.json` — the same file the
/// language-neutral Node twin (`selectionMergeVocabularyOracle`, the renderer react target's
/// `📜️script.ts`) re-derives, so neither side can drift alone. Ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END (`📓️selection-merge-vocabulary-2026-09-12.md`): a second
/// `add`/`remove`/`toggle` vocabulary lived in the renderer host and the non-domain world path, so
/// every modifier-click on a domain-bound world scene faulted `interactionSelect: unknown merge '…'`.
const MERGE_MODES_FIXTURE: &str = include_str!("../../🧫️fixtures/🎯️merge-modes.json");

/// 🧬️ Schema-first: the five words are the `MergeMode` enum of this module's OWN schema, and the Rust
/// codec is generated from nothing else. A word added to `MergeMode` without the schema — or to the
/// schema without the codec — fails here before any implementation can spell it.
#[semio_framework_async_macros::async_test]
async fn merge_mode_wire_labels_are_exactly_the_owned_schema_enum() {
    let fixture: serde_json::Value = serde_json::from_str(MERGE_MODES_FIXTURE).expect("merge-modes fixture");
    let schema: serde_json::Value = serde_json::from_str(include_str!("../../🧬️schema/🔣️.json")).expect("owned interaction schema");
    let declared: Vec<String> = schema["$defs"]["MergeMode"]["enum"].as_array().expect("MergeMode enum").iter().map(|word| word.as_str().expect("enum word").to_string()).collect();
    let fixture_words: Vec<String> = fixture["vocabulary"].as_array().expect("fixture vocabulary").iter().map(|word| word.as_str().expect("word").to_string()).collect();
    assert_eq!(fixture_words, declared, "the fixture vocabulary IS the owned schema enum");
    let coded: Vec<String> = [MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range].iter().map(|mode| mode.wire_label().to_string()).collect();
    assert_eq!(coded, declared, "every MergeMode variant must encode to its schema word, in the schema's own order");
    for word in &declared {
        assert_eq!(MergeMode::from_wire_label(word).map(|mode| mode.wire_label()), Some(word.as_str()), "the codec must round-trip {word}");
    }
    for word in fixture["deletedWords"].as_array().expect("deleted words") {
        let word = word.as_str().expect("deleted word");
        assert!(MergeMode::from_wire_label(word).is_none(), "the deleted word '{word}' must decode to nothing — greenfield, no compatibility layer");
        assert!(!declared.iter().any(|declared| declared == word), "the deleted word '{word}' must not be back in the schema");
    }
}

/// 🧮️ Every merge mode's set algebra over the fixture's declared ordered topology, driving the REAL
/// reducer (`next_selection`) — including `range`'s anchor law and its documented degradation to a
/// single-target pick on a domain that orders neither endpoint (see the fixture's `range` block: the
/// decision is to ACCEPT `range` everywhere and degrade, never to fault).
#[semio_framework_async_macros::async_test]
async fn next_selection_obeys_the_merge_vocabulary_fixture_for_every_mode() {
    let fixture: serde_json::Value = serde_json::from_str(MERGE_MODES_FIXTURE).expect("merge-modes fixture");
    let granularity = fixture["topology"]["granularity"].as_str().expect("fixture granularity").to_string();
    let words = |value: &serde_json::Value| -> Vec<String> { value.as_array().expect("fixture id list").iter().map(|id| id.as_str().expect("fixture id").to_string()).collect() };
    let default_ordered = words(&fixture["topology"]["ordered"]);
    let spec = SelectionSpec {
        modes: vec![SelectionMode::Multiple],
        methods: vec![SelectionMethod::Pick],
        merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
        transitive: false,
        broadcast: true,
    };
    let cases = fixture["cases"].as_array().expect("fixture cases");
    for word in fixture["vocabulary"].as_array().expect("fixture vocabulary") {
        let word = word.as_str().expect("word");
        assert!(cases.iter().any(|case| case["merge"].as_str() == Some(word)), "every declared merge word needs a law — '{word}' has none");
    }
    for case in cases {
        let case_id = case["id"].as_str().expect("case id");
        let ordered = case.get("orderedTopology").map(&words).unwrap_or_else(|| default_ordered.clone());
        let topology = DomainTopology { ordered: ordered.iter().map(|id| TopologyNode { id: id.clone(), granularity: granularity.clone(), parent: None }).collect() };
        let current = DomainSelection { granularity: granularity.clone(), ids: words(&case["seed"]), anchor_id: case["anchor"].as_str().map(str::to_string) };
        let merge = MergeMode::from_wire_label(case["merge"].as_str().expect("case merge")).unwrap_or_else(|| panic!("{case_id}: the fixture names a merge outside the schema vocabulary"));
        let input = SelectionInput { targets: words(&case["targets"]).into_iter().map(|id| InteractionTarget { granularity: granularity.clone(), id }).collect(), merge, mode: SelectionMode::Multiple };
        let next = next_selection(&spec, &current, &topology, &input).await;
        assert_eq!(next.ids, words(&case["selectedIds"]), "{case_id}: {}", case["why"].as_str().unwrap_or_default());
        assert_eq!(next.anchor_id.as_deref(), case["anchorId"].as_str(), "{case_id}: published anchor");
        let mut distinct = next.ids.clone();
        distinct.sort();
        distinct.dedup();
        assert_eq!(distinct.len(), next.ids.len(), "{case_id}: a selection is a SET, got {:?}", next.ids);
    }
}

/// 🖱️ The ONE modifier→merge map every world viewport shares (`marqueeModeFromModifiers` in
/// `🖱️ui`'s react target, `pick_select_action`/`WorldMarqueePublishJob` in the wgpu world): shift is
/// additive, ctrl/cmd subtractive, the chord invertive, a bare click replaces — and none of them is
/// ever `range`, which needs an ordered topology a viewport does not have.
#[semio_framework_async_macros::async_test]
async fn the_world_modifier_policy_never_resolves_outside_the_vocabulary() {
    let fixture: serde_json::Value = serde_json::from_str(MERGE_MODES_FIXTURE).expect("merge-modes fixture");
    let never: Vec<&str> = fixture["modifierPolicy"]["neverEmitted"].as_array().expect("neverEmitted").iter().map(|word| word.as_str().expect("word")).collect();
    let rows = fixture["modifierPolicy"]["rows"].as_array().expect("modifier rows");
    assert!(rows.len() >= 4, "the modifier policy must cover the bare click and every multi-select chord");
    for row in rows {
        let row_id = row["id"].as_str().expect("row id");
        let shift = row["modifiers"]["shiftKey"].as_bool().unwrap_or(false);
        let control = row["modifiers"]["ctrlKey"].as_bool().unwrap_or(false) || row["modifiers"]["metaKey"].as_bool().unwrap_or(false);
        let resolved = if shift && control {
            MergeMode::Invertive
        } else if shift {
            MergeMode::Additive
        } else if control {
            MergeMode::Subtractive
        } else {
            MergeMode::Replace
        };
        assert_eq!(row["pick"].as_str(), Some(resolved.wire_label()), "{row_id}: a whole-instance pick");
        let component = if resolved == MergeMode::Replace { MergeMode::Invertive } else { resolved };
        assert_eq!(row["componentPick"].as_str(), Some(component.wire_label()), "{row_id}: a component pick differs ONLY in that a bare click toggles");
        for word in &never {
            assert_ne!(row["pick"].as_str(), Some(*word), "{row_id}: a world viewport must never resolve to '{word}'");
            assert_ne!(row["componentPick"].as_str(), Some(*word), "{row_id}: a world viewport must never resolve to '{word}'");
        }
    }
}
//#endregion 🔖️MergeVocabularyLaws
