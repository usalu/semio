//! 📄️ Sourcing curate app commands — whole-document import, example switch, catalogue restock.
//!
//! 🧬️ All three commands here replace the whole document (or, for `stockFromCatalogue`, the whole
//! bulk-populated `stock` catalogue — never hand-authored item-by-item, see
//! `🧬️mutations/🦀️component.rs`'s own doc comment). The former whole-snapshot-replace variant is
//! banned outright from the `SourcingMutation` enum with NO replacement (`📓️taxonomy.md`), so every command below builds
//! `crate::apps::curate::reset_document_effect` (a `HostEffect::LoadDocument`, outside undo history)
//! instead of an `artifact_mutations` entry.

use crate::apps::curate::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use crate::apps::curate::{reset_document_effect, EMPTY_EXAMPLE_ID};
use crate::artifacts::curate::schema::{available_modules, default_document, empty_document};
use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::CurateSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

//#region 🔖️SetArtifactJson
pub mod set_artifact_json {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "document-json")]
    pub struct SetArtifactJson {
        pub json: String,
    }

    /// 🛠️ Dev-only whole-document import — kept out of the command palette.
    pub fn handle(payload: &SetArtifactJson, _doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
        match serde_json::from_str::<CurateSnapshot>(&payload.json) {
            Ok(document) => Ok(Emit { effects: vec![reset_document_effect(&document)], ..Default::default() }),
            Err(_) => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️SetArtifactJson

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
        let next = if payload.example_id.is_empty() || payload.example_id == EMPTY_EXAMPLE_ID { empty_document() } else { default_document() };
        Ok(Emit { effects: vec![reset_document_effect(&next)], ..Default::default() })
    }
}
//#endregion 🔖️SetActiveExample

//#region 🔖️StockFromCatalogue
pub mod stock_from_catalogue {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "stock-from-catalogue")]
    pub struct StockFromCatalogue {}

    /// 🧺️ `stock` is a bulk-populated reference catalogue, never hand-authored item-by-item — this
    /// merges in every not-yet-present catalogue kind, leaving `curated` untouched, and (like every
    /// other whole-document-replace command in this file) goes through `reset_document_effect`
    /// rather than a targeted mutation: there is no `create-object-kind` mutation to emit one-by-one.
    pub fn handle(_payload: &StockFromCatalogue, doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
        let mut stock = crate::artifacts::curate::stock_of(doc.snapshot);
        let existing: HashSet<String> = stock.iter().map(|kind| kind.id.clone()).collect();
        for module in available_modules() {
            for kind in module.kinds {
                if !existing.contains(&kind.id) {
                    stock.push(kind);
                }
            }
        }
        let document = crate::artifacts::curate::curate_snapshot_from_stock(stock, doc.snapshot.curated.clone());
        Ok(Emit { effects: vec![reset_document_effect(&document)], ..Default::default() })
    }
}
//#endregion 🔖️StockFromCatalogue

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::curate::commands::document::{set_active_example, set_artifact_json, stock_from_catalogue};
    use crate::apps::curate::testkit::new_app;
    use crate::apps::curate::SourcingCurateCommand;
    use crate::apps::curate::DEMO_STOCK_EXAMPLE_ID;
    use semio_framework::kernel::HostEffect;
    use semio_framework_plugin::{HistoryView, PluginApp};

    fn empty_view() -> (CurateSnapshot, HistoryView) {
        (CurateSnapshot::default(), HistoryView::empty())
    }

    /// 🧬️ Decodes the `HostEffect::LoadDocument` an `Emit` carries — every command in this file
    /// replaces the whole document outside undo history, so this is the shared assertion helper.
    fn load_document_pack(emit: &Emit<SourcingMutation, SourcingCurateConfigMutation>) -> CurateSnapshot {
        let HostEffect::LoadDocument { pack, .. } = emit.effects.first().expect("expected a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        <CurateSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack")
    }

    /// 🧬️ Whole-document replace is not an in-history mutation (the former whole-snapshot-replace
    /// variant is banned outright — see `📓️taxonomy.md`'s forbidden vocabulary), so this now surfaces as a `HostEffect::LoadDocument`
    /// carrying the replacement document's pack bytes, not an `artifact_mutations` entry — `dispatch`'s
    /// in-process `VcsArtifactApp` never applies `effects` to its own store (that's the real host's
    /// job), so this asserts on `requested_effects` rather than through `app.snapshot()`.
    #[test]
    fn curate_and_example_actions_survive_registry_enforcement() {
        let mut app = crate::apps::curate::testkit::new_app_with_registry();
        let result = app
            .dispatch_typed(SourcingCurateCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: DEMO_STOCK_EXAMPLE_ID.into() }), &semio_framework_plugin::testkit::meta("local"))
            .expect("set example");
        let HostEffect::LoadDocument { pack, .. } = result.requested_effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <CurateSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert!(!loaded.stock_extra.is_empty(), "demo-stock default materialized from the registry");
        let object_id = loaded.stock_extra[0].id.clone();
        let result = app
            .dispatch_typed(SourcingCurateCommand::CurateAdd(crate::apps::curate::commands::curation::curate_add::CurateAdd { object_id }), &semio_framework_plugin::testkit::meta("local"))
            .expect("curate");
        assert_eq!(result.mutations.len(), 1, "curateAdd is a document operation");
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
    }

    #[test]
    fn initial_document_has_populated_demo_stock() {
        let app = new_app();
        let document = app.snapshot().expect("snapshot");
        assert!(!document.stock_extra.is_empty());
    }

    #[test]
    fn set_active_example_loads_the_demo_stock_or_empty_curation_fixture() {
        let (snapshot, history) = empty_view();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = SourcingCurateConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let emit = set_active_example::handle(&set_active_example::SetActiveExample { example_id: DEMO_STOCK_EXAMPLE_ID.into() }, &doc, &cfg).expect("handle");
        assert!(!load_document_pack(&emit).stock_extra.is_empty());
        let emit = set_active_example::handle(&set_active_example::SetActiveExample { example_id: EMPTY_EXAMPLE_ID.into() }, &doc, &cfg).expect("handle");
        assert!(load_document_pack(&emit).curated.is_empty());
    }

    #[test]
    fn set_artifact_json_emits_a_load_document_effect_for_the_parsed_snapshot() {
        let (snapshot, history) = empty_view();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = SourcingCurateConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let expected = empty_document();
        let emit = set_artifact_json::handle(&set_artifact_json::SetArtifactJson { json: serde_json::to_string(&expected).unwrap() }, &doc, &cfg).expect("handle");
        assert_eq!(load_document_pack(&emit), expected);
    }

    #[test]
    fn stock_from_catalogue_merges_built_in_kinds_without_duplicating() {
        let (empty, history) = (empty_document(), HistoryView::empty());
        let doc = ArtifactView::new(&empty, &history);
        let cfg_snapshot = SourcingCurateConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let emit = stock_from_catalogue::handle(&stock_from_catalogue::StockFromCatalogue {}, &doc, &cfg).expect("handle");
        let loaded = load_document_pack(&emit);
        let expected: usize = crate::artifacts::curate::schema::sourcing_modules().iter().map(|module| module.demo_kinds().len()).sum();
        assert_eq!(loaded.stock_extra.len(), expected);

        let doc2 = ArtifactView::new(&loaded, &history);
        let emit2 = stock_from_catalogue::handle(&stock_from_catalogue::StockFromCatalogue {}, &doc2, &cfg).expect("handle");
        assert_eq!(load_document_pack(&emit2).stock_extra.len(), expected, "re-running against an already-full stock does not duplicate");
    }
}
//#endregion 🧪️Tests
