//! 📄️ 📄️ Draw play app commands command — `set-fixture-json`.

use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::{DrawSnapshot, DRAW_DOCUMENT_SCHEMA};
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
        vcs: store::ArtifactVcs { initial_snapshot: snapshot, edits: Vec::new(), changes: Vec::new(), checkpoints: Vec::new(), alternatives: Vec::new() },
        backbone: None,
        active_alternative_id: None,
        cursor: None,
        dialect: None,
        migrated_from: None,
        owner: None,
        lanes: Default::default(),
        edit_messages: Vec::new(),
        conflicts: Vec::new(),
    };
    match store::print_document_pack(&envelope) {
        Ok(files) => Emit { effects: vec![Effect::LoadDocument { pack: files.pack, spr: files.spr }], ..Default::default() },
        Err(_) => Emit::default(),
    }
}
//#endregion 🔖️WholeDocumentReset





#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "fixture-json")]
pub struct SetFixtureJson {
    pub json: String,
}

/// 🌡 Parsed as JSON (falling back to a no-op when it isn't valid or doesn't carry the draw schema)
/// — mirrors every other plugin's fixture-injection command.
pub async fn handle(payload: &SetFixtureJson, _doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut crate::editor::draw::commands::canvas_pointer_down::DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    if payload.json.contains(DRAW_DOCUMENT_SCHEMA) {
        if let Ok(snapshot) = serde_json::from_str::<DrawSnapshot>(&payload.json) {
            return Ok(load_document_effect(snapshot));
        }
    }
    Ok(Emit::default())
}
