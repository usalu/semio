//! 📄️ Sourcing curate app commands — whole-document import, example switch, catalogue restock.

use crate::apps::curate::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use crate::apps::curate::EMPTY_EXAMPLE_ID;
use crate::artifacts::curate::engine::{available_modules, default_document, empty_document};
use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::CurateDocument;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

fn set_document(document: CurateDocument) -> Emit<SourcingMutation, SourcingCurateConfigMutation> {
    Emit::mutations(vec![SourcingMutation::SetDocument { document }])
}

//#region 🔖️SetDocumentJson
pub mod set_document_json {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "document-json")]
    pub struct SetDocumentJson {
        pub json: String,
    }

    /// 🛠️ Dev-only whole-document import — kept out of the command palette.
    pub fn handle(payload: &SetDocumentJson, _doc: &DocumentView<'_, CurateDocument>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
        match serde_json::from_str::<CurateDocument>(&payload.json) {
            Ok(document) => Ok(set_document(document)),
            Err(_) => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️SetDocumentJson

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, _doc: &DocumentView<'_, CurateDocument>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
        let next = if payload.example_id.is_empty() || payload.example_id == EMPTY_EXAMPLE_ID { empty_document() } else { default_document() };
        Ok(set_document(next))
    }
}
//#endregion 🔖️SetActiveExample

//#region 🔖️StockFromCatalogue
pub mod stock_from_catalogue {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "stock-from-catalogue")]
    pub struct StockFromCatalogue {}

    pub fn handle(_payload: &StockFromCatalogue, doc: &DocumentView<'_, CurateDocument>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
        let mut document = doc.projection.clone();
        let existing: HashSet<String> = document.stock.iter().map(|kind| kind.id.clone()).collect();
        for module in available_modules() {
            for kind in module.kinds {
                if !existing.contains(&kind.id) {
                    document.stock.push(kind);
                }
            }
        }
        Ok(set_document(document))
    }
}
//#endregion 🔖️StockFromCatalogue

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::curate::commands::document::{set_active_example, set_document_json, stock_from_catalogue};
    use crate::apps::curate::testkit::{dispatch, new_app};
    use crate::apps::curate::SourcingCurateCommand;
    use crate::apps::curate::DEMO_STOCK_EXAMPLE_ID;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn curate_and_example_actions_survive_registry_enforcement() {
        // 🧬️ A registry-backed wrapper so `setActiveExample`'s default materializes and the
        // document-mutating curate commands pass kind discipline (they are declared Operations, never Views).
        let mut app = crate::apps::curate::testkit::new_app_with_registry();
        app.dispatch_typed(SourcingCurateCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: DEMO_STOCK_EXAMPLE_ID.into() }), &semio_framework_plugin::testkit::meta("local")).expect("set example");
        assert!(!app.projection().expect("projection").stock.is_empty(), "demo-stock default materialized from the registry");
        let object_id = app.projection().expect("projection").stock[0].id.clone();
        let result = app
            .dispatch_typed(SourcingCurateCommand::CurateAdd(crate::apps::curate::commands::curation::curate_add::CurateAdd { object_id }), &semio_framework_plugin::testkit::meta("local"))
            .expect("curate");
        assert_eq!(result.mutations.len(), 1, "curateAdd is a document operation");
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
    }

    #[test]
    fn initial_document_has_populated_demo_stock() {
        let app = new_app();
        let document = app.projection().expect("projection");
        assert!(!document.stock.is_empty());
    }

    #[test]
    fn stock_from_catalogue_merges_built_in_kinds_without_duplicating() {
        let mut app = new_app();
        // Reset to the empty fixture so stockFromCatalogue starts from a genuinely empty stock.
        dispatch(&mut app, SourcingCurateCommand::SetDocumentJson(set_document_json::SetDocumentJson { json: serde_json::to_string(&empty_document()).unwrap() }));
        assert!(app.projection().expect("projection").stock.is_empty());

        dispatch(&mut app, SourcingCurateCommand::StockFromCatalogue(stock_from_catalogue::StockFromCatalogue {}));
        let expected: usize = crate::artifacts::curate::engine::sourcing_modules().iter().map(|module| module.demo_kinds().len()).sum();
        assert_eq!(app.projection().expect("projection").stock.len(), expected);

        dispatch(&mut app, SourcingCurateCommand::StockFromCatalogue(stock_from_catalogue::StockFromCatalogue {}));
        assert_eq!(app.projection().expect("projection").stock.len(), expected);
    }
}
//#endregion 🧪️Tests
