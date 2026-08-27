//! 📄️ 📄️ Sourcing curate app commands command — `set-artifact-json`.

use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::schema::sourcing_json_envelope_is_bounded;
use crate::artifacts::curate::CurateSnapshot;
use crate::editor::sourcing::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use crate::editor::sourcing::reset_document_effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "document-json")]
pub struct SetArtifactJson {
    pub json: String,
}

/// 🛠️ Dev-only whole-document import — kept out of the command palette.
pub fn handle(payload: &SetArtifactJson, _doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
    if !sourcing_json_envelope_is_bounded(&payload.json) {
        return Err(Fault::from("sourcing.invalid-payload: document JSON exceeds byte, depth, string, or cardinality limit"));
    }
    match serde_json::from_str::<CurateSnapshot>(&payload.json) {
        Ok(document) => Ok(Emit { effects: vec![reset_document_effect(&document)], ..Default::default() }),
        Err(_) => Err(Fault::from("sourcing.invalid-payload: document schema mismatch")),
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::curate::schema::{empty_document, SourcingModule};
    use crate::editor::sourcing::commands::{set_active_example, stock_from_catalogue};
    use crate::editor::sourcing::testkit::new_app;
    use crate::editor::sourcing::SourcingCurateCommand;
    use crate::editor::sourcing::{DEMO_STOCK_EXAMPLE_ID, EMPTY_EXAMPLE_ID};
    use semio_framework::kernel::Effect;
    use semio_framework_plugin::{HistoryView, PluginApp};

    #[semio_framework_async_macros::async_test]
    async fn pre_deserialization_envelope_accepts_exact_limits_and_rejects_plus_one() {
        use crate::artifacts::curate::schema::{SOURCING_JSON_MAX_BYTES, SOURCING_JSON_MAX_DEPTH, SOURCING_JSON_MAX_ITEMS, SOURCING_JSON_MAX_STRING_BYTES};

        let raw_max = format!("{{}}{}", " ".repeat(SOURCING_JSON_MAX_BYTES - 2));
        assert!(sourcing_json_envelope_is_bounded(&raw_max));
        assert!(!sourcing_json_envelope_is_bounded(&(raw_max + " ")));

        let depth_max = format!("{}0{}", "[".repeat(SOURCING_JSON_MAX_DEPTH), "]".repeat(SOURCING_JSON_MAX_DEPTH));
        assert!(sourcing_json_envelope_is_bounded(&depth_max));
        let depth_plus_one = format!("{}0{}", "[".repeat(SOURCING_JSON_MAX_DEPTH + 1), "]".repeat(SOURCING_JSON_MAX_DEPTH + 1));
        assert!(!sourcing_json_envelope_is_bounded(&depth_plus_one));

        let string_max = format!("\"{}\"", "x".repeat(SOURCING_JSON_MAX_STRING_BYTES));
        assert!(sourcing_json_envelope_is_bounded(&string_max));
        let string_plus_one = format!("\"{}\"", "x".repeat(SOURCING_JSON_MAX_STRING_BYTES + 1));
        assert!(!sourcing_json_envelope_is_bounded(&string_plus_one));

        let items_max = format!("[{}]", vec!["0"; SOURCING_JSON_MAX_ITEMS - 1].join(","));
        assert!(sourcing_json_envelope_is_bounded(&items_max));
        let items_plus_one = format!("[{}]", vec!["0"; SOURCING_JSON_MAX_ITEMS].join(","));
        assert!(!sourcing_json_envelope_is_bounded(&items_plus_one));
    }

    fn empty_view() -> (CurateSnapshot, HistoryView) {
        (CurateSnapshot::default(), HistoryView::empty())
    }

    /// 🧬️ Decodes the `Effect::LoadDocument` an `Emit` carries — every command in this file
    /// replaces the whole document outside undo history, so this is the shared assertion helper.
    fn load_document_pack(emit: &Emit<SourcingMutation, SourcingCurateConfigMutation>) -> CurateSnapshot {
        let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("expected a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        <CurateSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack")
    }

    /// 🧬️ Whole-document replace is not an in-history mutation (the former whole-snapshot-replace
    /// variant is banned outright — see `📓️taxonomy.md`'s forbidden vocabulary), so this now surfaces as a `Effect::LoadDocument`
    /// carrying the replacement document's pack bytes, not an `artifact_mutations` entry — `dispatch`'s
    /// in-process `VcsArtifactApp` never applies `effects` to its own store (that's the real host's
    /// job), so this asserts on `requested_effects` rather than through `app.snapshot()`.
    #[semio_framework_async_macros::async_test]
    async fn curate_and_example_actions_survive_registry_enforcement() {
        let mut app = crate::editor::sourcing::testkit::new_app_with_registry().await;
        let result = app.dispatch_typed(SourcingCurateCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: DEMO_STOCK_EXAMPLE_ID.into() }), &semio_framework_plugin::testkit::meta("local")).await.expect("set example");
        let Effect::LoadDocument { pack, .. } = result.requested_effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <CurateSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert!(!loaded.stock_extra.is_empty(), "demo-stock default materialized from the registry");
        let object_id = loaded.stock_extra[0].id.clone();
        let result = app.dispatch_typed(SourcingCurateCommand::CurateAdd(crate::editor::sourcing::commands::curate_add::CurateAdd { object_id }), &semio_framework_plugin::testkit::meta("local")).await.expect("curate");
        assert_eq!(result.mutations.len(), 1, "curateAdd is a document operation");
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).await.expect("undo");
    }

    #[semio_framework_async_macros::async_test]
    async fn initial_document_has_populated_demo_stock() {
        let app = new_app().await;
        let document = app.snapshot().expect("snapshot");
        assert!(!document.stock_extra.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn set_active_example_loads_the_demo_stock_or_empty_curation_fixture() {
        let (snapshot, history) = empty_view();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = SourcingCurateConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let emit = set_active_example::handle(&set_active_example::SetActiveExample { example_id: DEMO_STOCK_EXAMPLE_ID.into() }, &doc, &cfg).expect("handle");
        assert!(!load_document_pack(&emit).stock_extra.is_empty());
        let emit = set_active_example::handle(&set_active_example::SetActiveExample { example_id: EMPTY_EXAMPLE_ID.into() }, &doc, &cfg).expect("handle");
        assert!(load_document_pack(&emit).curated.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn set_artifact_json_emits_a_load_document_effect_for_the_parsed_snapshot() {
        let (snapshot, history) = empty_view();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = SourcingCurateConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let expected = empty_document();
        let emit = handle(&SetArtifactJson { json: serde_json::to_string(&expected).unwrap() }, &doc, &cfg).expect("handle");
        assert_eq!(load_document_pack(&emit), expected);
    }

    #[semio_framework_async_macros::async_test]
    async fn stock_from_catalogue_merges_built_in_kinds_without_duplicating() {
        let (empty, history) = (empty_document(), HistoryView::empty());
        let doc = ArtifactView::new(&empty, &history);
        let cfg_snapshot = SourcingCurateConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let emit = stock_from_catalogue::handle(&stock_from_catalogue::StockFromCatalogue {}, &doc, &cfg).expect("handle");
        let loaded = load_document_pack(&emit);
        let expected: usize = crate::artifacts::curate::schema::sourcing_modules("[]").iter().map(|module| module.demo_kinds().len()).sum();
        assert_eq!(loaded.stock_extra.len(), expected);

        let doc2 = ArtifactView::new(&loaded, &history);
        let emit2 = stock_from_catalogue::handle(&stock_from_catalogue::StockFromCatalogue {}, &doc2, &cfg).expect("handle");
        assert_eq!(load_document_pack(&emit2).stock_extra.len(), expected, "re-running against an already-full stock does not duplicate");
    }
}
//#endregion 🧪️Tests
