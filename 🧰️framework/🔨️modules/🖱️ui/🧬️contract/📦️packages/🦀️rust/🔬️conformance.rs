//! @emoji 🧪️ The conformance-corpus harness — loads every fixture under
//! `📚️examples/🧪️conformance/` and proves the contract crate's own `validate_snapshot`/
//! `apply_patch` treat it exactly as its sibling `📓️terra-conformance-corpus-report.md` documents.
//! React DOM and the GPU renderer family both consume the identical JSON files later; this file is
//! what keeps the Rust side of that promise honest.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1.
//!
//! Entirely `#[cfg(test)]`: the corpus is authored JSON read from disk via `std::fs`, which does not
//! exist on `wasm32-unknown-unknown` and is meaningless to compile into a shipped renderer either way
//! — `cargo check --target wasm32-*` never builds `#[cfg(test)]` code, so this file costs the wasm
//! gates nothing. Not behind `typegen`: this must run under a plain `cargo test`.

//#region 🔖️Conformance
#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::OnceLock;

    // 🌱️ `FromValue` here is the first-party analog of `Deserialize` below, for ticket
    // 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
    use semio_framework_value_derive::FromValue;
    use serde::Deserialize;

    //#region 🗂️Corpus
    /// 📂️ `📚️examples/🧪️conformance/`, resolved from this crate's own manifest dir so the harness
    /// works regardless of the caller's working directory.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn corpus_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../📚️examples/🧪️conformance")
    }

    const GROUPS_WITHOUT_PATCH: &[&str] = &["🧩️component", "🖥️composite", "📐️layout", "♿️accessibility"];
    const GROUPS_WITH_PATCH: &[&str] = &["🩹️patch", "🚫️rejection"];

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct CorpusCatalog {
        version: u32,
        roles: BTreeMap<String, String>,
        groups: BTreeMap<String, CorpusGroup>,
    }

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct CorpusGroup {
        patch: bool,
        cases: BTreeMap<String, String>,
    }

    fn catalog() -> &'static CorpusCatalog {
        static CATALOG: OnceLock<CorpusCatalog> = OnceLock::new();
        CATALOG.get_or_init(|| read_json(&corpus_dir().join("📇️catalog.json")))
    }

    /// 🔤️ The schema-owned case identities in deterministic order, independent of directory glyphs.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn case_slugs(group: &str) -> Vec<String> {
        catalog().groups[group].cases.keys().cloned().collect()
    }

    fn case_path(group: &str, id: &str, role: &str) -> PathBuf {
        corpus_dir().join(group).join(&catalog().groups[group].cases[id]).join(&catalog().roles[role])
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> T {
        let text = fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path:?}: {error}"));
        serde_json::from_str(&text).unwrap_or_else(|error| panic!("parse {path:?}: {error}"))
    }
    //#endregion 🗂️Corpus

    //#region 📄️Expectation
    /// 📄️ One row of an `expect.json`'s `tree.shape` array — the semantic tree shape any renderer
    /// must produce, deliberately Rust-agnostic (a bare component-type string, not `crate::Component`
    /// itself) so a TypeScript conformance test can load the exact same file.
    #[derive(Deserialize, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(crate = "::protocol::value", rename_all = "camelCase")]
    struct ExpectedNode {
        id: crate::UiNodeId,
        #[serde(rename = "type")]
        #[value(rename = "type")]
        component_type: String,
        children: Vec<crate::UiNodeId>,
    }

    #[derive(Deserialize, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(crate = "::protocol::value", rename_all = "camelCase")]
    struct ExpectedTree {
        root: crate::UiNodeId,
        node_count: usize,
        shape: Vec<ExpectedNode>,
    }

    /// ♿️ One row of an `expect.json`'s `accessibility` array — resolved role/label/description/
    /// live/shortcut/hidden for one node. `role` is deliberately absent, mirroring
    /// `crate::AccessibilitySpec` itself: the role is implied by the node's component type.
    #[derive(Deserialize, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(crate = "::protocol::value", rename_all = "camelCase")]
    struct ExpectedAccessibility {
        id: crate::UiNodeId,
        label: Option<String>,
        description: Option<String>,
        live: crate::Liveness,
        shortcut: Option<String>,
        hidden: bool,
    }

    /// 📄️ The declarative, renderer-neutral expectation every fixture carries — see
    /// `📓️terra-conformance-corpus-report.md`'s "decisions" section for why this shape and not a
    /// Rust-specific one. `violations`/`patch_rejection` reuse the contract's own wire types directly
    /// (`crate::PatchRejection` etc.) rather than re-describing them, so a corpus expectation can never
    /// name a violation shape the contract itself does not actually produce.
    #[derive(Deserialize, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(crate = "::protocol::value", rename_all = "camelCase")]
    struct Expectation {
        #[allow(dead_code)]
        case: String,
        #[allow(dead_code)]
        kind: String,
        #[allow(dead_code)]
        description: String,
        outcome: String,
        limits: Option<crate::UiDocumentLimits>,
        #[serde(default)]
        #[value(default)]
        tree: Option<ExpectedTree>,
        #[serde(default)]
        #[value(default)]
        accessibility: Vec<ExpectedAccessibility>,
        #[serde(default)]
        #[value(default)]
        action_ids: Vec<String>,
        #[serde(default)]
        #[value(default)]
        patch_rejection: Option<crate::PatchRejection>,
    }

    /// 🏷️ The wire `type` tag `crate::Component` serializes to — mirrors `component.rs`'s
    /// `#[serde(tag = "type", rename_all = "camelCase")]` exactly, so a mismatch here is a real
    /// contract-vs-corpus drift, never a harness typo the compiler could not have caught.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn component_type_tag(component: &crate::Component) -> &'static str {
        match component {
            crate::Component::Container(_) => "container",
            crate::Component::Text(_) => "text",
            crate::Component::Button(_) => "button",
            crate::Component::Separator(_) => "separator",
            crate::Component::Input(_) => "input",
            crate::Component::Select(_) => "select",
            crate::Component::Toggle(_) => "toggle",
            crate::Component::KeyValueList(_) => "keyValueList",
            crate::Component::Slider(_) => "slider",
            crate::Component::NumberStepper(_) => "numberStepper",
            crate::Component::Ring(_) => "ring",
            crate::Component::IconSelect(_) => "iconSelect",
            crate::Component::Tree(_) => "tree",
            crate::Component::TreeSection(_) => "treeSection",
            crate::Component::TreeItem(_) => "treeItem",
            crate::Component::Image(_) => "image",
            crate::Component::Surface(_) => "surface",
            crate::Component::Extension(_) => "extension",
        }
    }

    /// ✅️ Asserts `state`'s reachable-and-otherwise document content matches `expectation`'s `tree`/
    /// `accessibility`/`actionIds` — the shared assertion body for both a fresh snapshot and a
    /// post-`apply_patch` state, so the two acceptance paths can never silently drift apart.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn assert_matches_expectation(case_id: &str, state: &crate::UiSnapshotState, expectation: &Expectation) {
        let tree = expectation.tree.as_ref().unwrap_or_else(|| panic!("{case_id}: accept case has no `tree` in its expectation"));
        assert_eq!(state.root(), Some(tree.root), "{case_id}: root mismatch");
        assert_eq!(state.nodes.len(), tree.node_count, "{case_id}: node count mismatch");
        for expected in &tree.shape {
            let record = state.get(expected.id).unwrap_or_else(|| panic!("{case_id}: expected node {:?} missing from state", expected.id));
            assert_eq!(component_type_tag(&record.component), expected.component_type, "{case_id}: node {:?} component type mismatch", expected.id);
            let children: Vec<_> = record.children.iter().copied().collect();
            assert_eq!(children, expected.children, "{case_id}: node {:?} children mismatch", expected.id);
        }
        for expected in &expectation.accessibility {
            let record = state.get(expected.id).unwrap_or_else(|| panic!("{case_id}: expected node {:?} missing from state", expected.id));
            let a = &record.accessibility;
            assert_eq!(a.label.as_ref().map(|label| label.0.as_str()), expected.label.as_deref(), "{case_id}: node {:?} label mismatch", expected.id);
            assert_eq!(a.description.as_ref().map(|label| label.0.as_str()), expected.description.as_deref(), "{case_id}: node {:?} description mismatch", expected.id);
            assert_eq!(a.live, expected.live, "{case_id}: node {:?} live mismatch", expected.id);
            assert_eq!(a.shortcut.as_deref(), expected.shortcut.as_deref(), "{case_id}: node {:?} shortcut mismatch", expected.id);
            assert_eq!(a.hidden, expected.hidden, "{case_id}: node {:?} hidden mismatch", expected.id);
        }
        let mut ids: Vec<crate::UiNodeId> = state.nodes.keys().copied().collect();
        ids.sort();
        let action_ids: Vec<String> = ids
            .iter()
            .flat_map(|id| state.nodes.get(id).expect("enumerated node remains present").bindings.iter().map(|binding| binding.action.to_string()))
            .collect();
        assert_eq!(action_ids, expectation.action_ids, "{case_id}: reachable action ids mismatch");
    }
    //#endregion 📄️Expectation

    //#region 🌳️AcceptFixtures
    /// 🌳️ Every `🧩️component`/`🖥️composite`/`📐️layout`/`♿️accessibility` fixture: deserializes, passes
    /// `validate_snapshot` cleanly, and matches its expectation's tree/accessibility/action-id shape.
    #[test]
    fn snapshot_only_fixtures_are_valid_and_match_their_expectations() {
        let limits = crate::UiDocumentLimits::default();
        for group in GROUPS_WITHOUT_PATCH {
            for slug in case_slugs(group) {
                let snapshot: crate::UiSnapshot = read_json(&case_path(group, &slug, "snapshot"));
                let expectation: Expectation = read_json(&case_path(group, &slug, "expect"));
                assert_eq!(expectation.outcome, "accept", "{group}/{slug}: this group is accept-only");
                let case_limits = expectation.limits.unwrap_or(limits);
                crate::validate_snapshot(&snapshot, &case_limits).unwrap_or_else(|violations| panic!("{group}/{slug}: expected a valid snapshot, got violations {violations:?}"));
                let state: crate::UiSnapshotState = snapshot.into();
                assert_matches_expectation(&format!("{group}/{slug}"), &state, &expectation);
            }
        }
    }

    /// 🩹️ Every `🩹️patch` fixture: its base snapshot applies its patch cleanly through the crate's own
    /// `apply_patch`, and the resulting state matches the expectation's post-patch shape.
    #[test]
    fn patch_fixtures_apply_cleanly_and_match_their_expectations() {
        let default_limits = crate::UiDocumentLimits::default();
        let group = "🩹️patch";
        for slug in case_slugs(group) {
            let snapshot: crate::UiSnapshot = read_json(&case_path(group, &slug, "snapshot"));
            let patch: crate::UiPatch = read_json(&case_path(group, &slug, "patch"));
            let expectation: Expectation = read_json(&case_path(group, &slug, "expect"));
            assert_eq!(expectation.outcome, "accept", "{group}/{slug}: 🩹️patch fixtures are always accept cases — rejections live in 🚫️rejection");
            let limits = expectation.limits.unwrap_or(default_limits);
            let mut state: crate::UiSnapshotState = snapshot.into();
            crate::apply_patch(&mut state, &patch, &limits).unwrap_or_else(|rejection| panic!("{group}/{slug}: expected the patch to apply, got {rejection:?}"));
            assert_matches_expectation(&format!("{group}/{slug}"), &state, &expectation);
        }
    }
    //#endregion 🌳️AcceptFixtures

    //#region 🚫️RejectionFixtures
    /// 🚫️ Every `🚫️rejection` fixture: its base snapshot's patch is rejected by `apply_patch`, the
    /// receiver's state is left byte-for-byte unchanged, and the rejection reason matches the
    /// expectation's `patchRejection` exactly — a renderer that accepts one of these is broken.
    #[test]
    fn rejection_fixtures_are_rejected_with_the_named_violation_and_leave_state_unchanged() {
        let default_limits = crate::UiDocumentLimits::default();
        let group = "🚫️rejection";
        for slug in case_slugs(group) {
            let snapshot: crate::UiSnapshot = read_json(&case_path(group, &slug, "snapshot"));
            let patch: crate::UiPatch = read_json(&case_path(group, &slug, "patch"));
            let expectation: Expectation = read_json(&case_path(group, &slug, "expect"));
            assert_eq!(expectation.outcome, "reject", "{group}/{slug}: 🚫️rejection fixtures are always reject cases");
            let limits = expectation.limits.unwrap_or(default_limits);
            let before: crate::UiSnapshotState = snapshot.into();
            let mut after = before.credited_clone().expect("credited fixture clone");
            let rejection = crate::apply_patch(&mut after, &patch, &limits).expect_err(&format!("{group}/{slug}: expected the patch to be rejected, but it applied"));
            let expected = expectation.patch_rejection.as_ref().unwrap_or_else(|| panic!("{group}/{slug}: reject case has no `patchRejection` in its expectation"));
            assert_eq!(&rejection, expected, "{group}/{slug}: rejection reason mismatch");
            assert_eq!(after, before, "{group}/{slug}: state must be byte-for-byte unchanged after a rejected patch");
        }
    }
    //#endregion 🚫️RejectionFixtures

    //#region 🔍️Pairing
    /// 🔍️ Independently checks the neutral catalog through serde and every exact case/role identity.
    #[test]
    fn corpus_has_no_orphan_fixtures() {
        let catalog = catalog();
        assert_eq!(catalog.version, 1);
        assert_eq!(catalog.roles, BTreeMap::from([("snapshot".into(), "📸️snapshot.json".into()), ("expect".into(), "🎯️expect.json".into()), ("patch".into(), "🩹️patch.json".into())]));
        let names = |dir: &Path| -> BTreeSet<String> {
            fs::read_dir(dir).unwrap().map(|entry| entry.unwrap().file_name().into_string().unwrap()).collect()
        };
        let expected_groups: BTreeSet<String> = GROUPS_WITHOUT_PATCH.iter().chain(GROUPS_WITH_PATCH.iter()).map(|group| (*group).into()).collect();
        assert_eq!(catalog.groups.keys().cloned().collect::<BTreeSet<_>>(), expected_groups);
        assert_eq!(names(&corpus_dir()), expected_groups.into_iter().chain(["📇️catalog.json".into(), "🧬️catalog.schema.json".into()]).collect());
        let mut count = 0;
        for (group, definition) in &catalog.groups {
            assert_eq!(definition.patch, GROUPS_WITH_PATCH.contains(&group.as_str()));
            let directories: BTreeSet<String> = definition.cases.values().cloned().collect();
            assert_eq!(directories.len(), definition.cases.len());
            assert_eq!(names(&corpus_dir().join(group)), directories);
            for (id, directory) in &definition.cases {
                let roles: BTreeSet<String> = ["snapshot", "expect"].into_iter().chain(definition.patch.then_some("patch")).map(|role| catalog.roles[role].clone()).collect();
                assert_eq!(names(&corpus_dir().join(group).join(directory)), roles);
                let expectation: Expectation = read_json(&case_path(group, id, "expect"));
                assert_eq!(&expectation.case, id);
                count += 1;
            }
        }
        assert_eq!(count, 62);
    }
    //#endregion 🔍️Pairing

    //#region 📊️Coverage
    /// 📊️ Every `crate::Component` variant must appear in at least one fixture, across the WHOLE
    /// corpus — adding a variant to the contract without a fixture must fail this, loudly, rather than
    /// the corpus silently going stale.
    #[test]
    fn every_component_variant_appears_in_the_corpus() {
        let all_groups: Vec<&str> = GROUPS_WITHOUT_PATCH.iter().chain(GROUPS_WITH_PATCH.iter()).copied().collect();
        let mut seen: BTreeSet<String> = BTreeSet::new();
        for group in &all_groups {
            for slug in case_slugs(group) {
                let path = case_path(group, &slug, "snapshot");
                let snapshot: crate::UiSnapshot = read_json(&path);
                for record in &snapshot.nodes {
                    seen.insert(component_type_tag(&record.component).to_string());
                }
            }
        }
        let expected: BTreeSet<&str> =
            ["container", "text", "button", "separator", "input", "select", "toggle", "keyValueList", "slider", "numberStepper", "ring", "iconSelect", "tree", "treeSection", "treeItem", "image", "surface", "extension"].into_iter().collect();
        let missing: Vec<&&str> = expected.iter().filter(|tag| !seen.contains(**tag)).collect();
        assert!(missing.is_empty(), "Component variants with no fixture anywhere in the corpus: {missing:?}");
    }

    /// 📊️ Every `crate::UiPatchOp` variant must appear in at least one 🩹️patch or 🚫️rejection case's
    /// ops — the same "adding a variant silently goes untested" guard, for patches instead of nodes.
    #[test]
    fn every_ui_patch_op_variant_appears_in_a_patch_case() {
        // 🏷️ The wire `type` tag every `crate::UiPatchOp` variant serializes to.
        fn op_type_tag(op: &crate::UiPatchOp) -> &'static str {
            match op {
                crate::UiPatchOp::Upsert(_) => "upsert",
                crate::UiPatchOp::SetComponent { .. } => "setComponent",
                crate::UiPatchOp::SetLayout { .. } => "setLayout",
                crate::UiPatchOp::SetActivity { .. } => "setActivity",
                crate::UiPatchOp::SetChildren { .. } => "setChildren",
                crate::UiPatchOp::SetStyle { .. } => "setStyle",
                crate::UiPatchOp::SetAccessibility { .. } => "setAccessibility",
                crate::UiPatchOp::SetBindings { .. } => "setBindings",
                crate::UiPatchOp::SetMenu { .. } => "setMenu",
                crate::UiPatchOp::Remove { .. } => "remove",
                crate::UiPatchOp::SetRoot { .. } => "setRoot",
            }
        }

        let mut seen: BTreeSet<String> = BTreeSet::new();
        for group in GROUPS_WITH_PATCH {
            for slug in case_slugs(group) {
                let path = case_path(group, &slug, "patch");
                let patch: crate::UiPatch = read_json(&path);
                for op in &patch.ops {
                    seen.insert(op_type_tag(op).to_string());
                }
            }
        }
        let expected: BTreeSet<&str> = ["upsert", "setComponent", "setLayout", "setActivity", "setChildren", "setStyle", "setAccessibility", "setBindings", "setMenu", "remove", "setRoot"].into_iter().collect();
        let missing: Vec<&&str> = expected.iter().filter(|tag| !seen.contains(**tag)).collect();
        assert!(missing.is_empty(), "UiPatchOp variants with no fixture anywhere in the corpus: {missing:?}");
    }
    //#endregion 📊️Coverage
}
//#endregion 🔖️Conformance
