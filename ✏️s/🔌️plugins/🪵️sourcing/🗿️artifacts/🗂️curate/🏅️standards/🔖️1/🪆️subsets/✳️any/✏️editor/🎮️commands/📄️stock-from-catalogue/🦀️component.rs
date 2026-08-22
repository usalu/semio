//! 📄️ 📄️ Sourcing curate app commands command — `stock-from-catalogue`.

use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::schema::available_modules;
use crate::artifacts::curate::CurateSnapshot;
use crate::editor::sourcing::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use crate::editor::sourcing::reset_document_effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "stock-from-catalogue")]
pub struct StockFromCatalogue {}

/// 🧺️ `stock` is a bulk-populated reference catalogue, never hand-authored item-by-item — this
/// merges in every not-yet-present catalogue kind, leaving `curated` untouched, and (like every
/// other whole-document-replace command in this file) goes through `reset_document_effect`
/// rather than a targeted mutation: there is no `create-object-kind` mutation to emit one-by-one.
pub async fn handle(_payload: &StockFromCatalogue, doc: &ArtifactView<'_, CurateSnapshot>, cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
    let mut stock = crate::artifacts::curate::stock_of(doc.snapshot);
    let existing: HashSet<String> = stock.iter().map(|kind| kind.id.clone()).collect();
    for module in available_modules(&cfg.snapshot.contributions_json) {
        for kind in module.kinds {
            if !existing.contains(&kind.id) {
                stock.push(kind);
            }
        }
    }
    let document = crate::artifacts::curate::curate_snapshot_from_stock(stock, doc.snapshot.curated.clone());
    Ok(Emit { effects: vec![reset_document_effect(&document)], ..Default::default() })
}
