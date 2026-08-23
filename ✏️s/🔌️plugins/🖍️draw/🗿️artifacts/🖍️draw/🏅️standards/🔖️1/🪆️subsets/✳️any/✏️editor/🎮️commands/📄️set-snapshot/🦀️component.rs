//! 📄️ 📄️ Draw play app commands command — `set-snapshot`.

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::{DrawSnapshot, DRAW_DOCUMENT_SCHEMA};
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️WholeDocumentReset
/// 🖼️ Whole-document replace is NOT expressible as an in-history `Mutation` (banned vocabulary —
/// see `.🦑️repo/🎫️tickets/26/08/12/SEMANTIC-MUTATIONS-OVERHAUL/📓️taxonomy.md`'s "Forbidden
/// vocabulary": `SetSnapshot` has no replacement mutation). File-open/paste-over/load-example
/// commands go through the sanctioned non-history `ArtifactStore::reset` path instead, which from
/// an `ArtifactApp::handle` this reaches via `Effect::LoadDocument` (pack+spr bytes) — the same
/// host-owned whole-store-swap primitive `engine::space`'s `open_space` command uses.
async fn load_document_effect(snapshot: DrawSnapshot) -> Emit<DrawMutation, DrawConfigMutation> {
    let envelope = store::ArtifactEnvelope::<DrawSnapshot, DrawMutation> {
        schema: DRAW_DOCUMENT_SCHEMA.into(),
        id: snapshot.id.clone(),
        vcs: store::ArtifactVcs {
            initial_snapshot: snapshot,
            edits: store::ArtifactHistoryLedger::new(),
            changes: store::ArtifactHistoryLedger::new(),
            checkpoints: store::ArtifactHistoryLedger::new(),
            alternatives: store::ArtifactHistoryLedger::new(),
        },
        backbone: None,
        active_alternative_id: None,
        cursor: None,
        dialect: None,
        migrated_from: None,
        owner: None,
        lanes: Default::default(),
        edit_messages: store::ArtifactEditMessageLedger::new(),
        conflicts: Vec::new(),
    };
    match store::print_document_pack(&envelope) {
        Ok(files) => Emit { effects: vec![Effect::LoadDocument { pack: files.pack, spr: files.spr }], ..Default::default() },
        Err(_) => Emit::default(),
    }
}
//#endregion 🔖️WholeDocumentReset

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct SetSnapshot {
    #[dsl(block)]
    pub snapshot: DrawSnapshot,
}

pub async fn handle(
    payload: &SetSnapshot,
    _doc: &ArtifactView<'_, DrawSnapshot>,
    _cfg: &ConfigView<'_, DrawConfig>,
    _session: &mut crate::editor::draw::commands::canvas_pointer_down::DrawSession,
) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    Ok(load_document_effect(payload.snapshot.clone()))
}
