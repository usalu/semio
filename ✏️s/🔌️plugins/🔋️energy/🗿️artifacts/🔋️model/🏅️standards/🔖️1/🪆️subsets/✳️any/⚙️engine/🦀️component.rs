//! ⚙️ EnergyModel artifact — headless BEM compute engine owning a real `EnergyModelArtifact`.

use crate::artifacts::model::{
    EnergyModelArtifact, EnergyModelDiff, EnergyModelMutation, EnergyModelSnapshot,
    ENERGY_MODEL_DOCUMENT_SCHEMA,
};
use crate::model::Model;
use crate::results::Results;

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_energy_model_snapshot() -> EnergyModelSnapshot {
    EnergyModelSnapshot::default()
}

/// 🏢️ Decode the typed `Model` from a snapshot's opaque JSON body.
pub fn model_from_snapshot(snapshot: &EnergyModelSnapshot) -> Result<Model, String> {
    serde_json::from_str(&snapshot.model_json).map_err(|e| e.to_string())
}

/// 📕️ Encode a typed `Model` into snapshot form.
pub fn snapshot_from_model(model: &Model) -> Result<EnergyModelSnapshot, String> {
    Ok(EnergyModelSnapshot {
        schema: ENERGY_MODEL_DOCUMENT_SCHEMA.into(),
        model_json: serde_json::to_string(model).map_err(|e| e.to_string())?,
    })
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers `s.energy.model`'s pack↔dsl document codec directly against `store`'s process-wide
/// registry — no `ArtifactApp` to bind through `register_document_codec_for_app` (see `declaration`'s
/// own doc). Called from the plugin root's narrowed `.setup()`.
pub fn register_document_codec() {
    store::register_document_codec(store::ArtifactCodec::of::<
        EnergyModelSnapshot,
        EnergyModelMutation,
    >(ENERGY_MODEL_DOCUMENT_SCHEMA));
}

//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚡️ Headless energy-model artifact engine — owns a real `EnergyModelArtifact`.
pub struct EnergyModelEngine {
    artifact_state: EnergyModelArtifact,
    snapshot_state: EnergyModelSnapshot,
}

impl EnergyModelEngine {
    /// 🏗️ Builds an engine from a persisted snapshot (preview results start empty).
    pub fn new(snapshot: EnergyModelSnapshot) -> Self {
        let artifact_state = EnergyModelArtifact::from_snapshot(snapshot.clone());
        Self {
            artifact_state,
            snapshot_state: snapshot,
        }
    }

    /// 🚀️ Runs the BEM simulation, updates preview `results_json`, returns owned results.
    pub fn run_simulation(
        &mut self,
        config: &crate::kernel::SimulationConfig,
    ) -> Result<Results, crate::error::Error> {
        let model = model_from_snapshot(&self.snapshot_state).map_err(crate::error::Error::severe)?;
        let results = crate::sim::Engine::run(&model, config)?;
        self.artifact_state.results_json =
            serde_json::to_string(&results).map_err(|e| crate::error::Error::severe(e.to_string()))?;
        Ok(results)
    }

    /// 📤️ The engine's owned `EnergyModelArtifact` (distinct from the persisted snapshot —
    /// carries preview `results_json` too).
    pub fn artifact(&self) -> &EnergyModelArtifact {
        &self.artifact_state
    }

    /// 📤️ The persisted snapshot this engine was built from.
    pub fn snapshot(&self) -> &EnergyModelSnapshot {
        &self.snapshot_state
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_energy_model_snapshot();
        assert_eq!(snapshot.schema, ENERGY_MODEL_DOCUMENT_SCHEMA);
    }

    #[test]
    fn engine_owns_artifact_not_snapshot_alias() {
        let engine = EnergyModelEngine::new(empty_energy_model_snapshot());
        assert_eq!(engine.artifact().schema, ENERGY_MODEL_DOCUMENT_SCHEMA);
        assert!(engine.artifact().results_json.is_empty());
        assert_eq!(engine.snapshot().model_json, "{}");
    }

    #[test]
    fn example_fixture_parses() {
        let document = crate::artifacts::model::dsl::parse_dsl(
            crate::artifacts::model::dsl::SEMIO_ENERGY_MODEL_EXAMPLE_TEXT,
        )
        .expect("parse");
        assert_eq!(document.schema, ENERGY_MODEL_DOCUMENT_SCHEMA);
        assert!(!document.model_json.is_empty());
    }
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::model::standards::v1::subsets::any::schema::EnergyModelComposer as EnergyModelAnyComposer;
    use crate::artifacts::model::standards::v1::subsets::any::schema::ModelBuilder as EnergyModelAnyBuilder;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    //#region 🔖️ExportEntries
    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: the typed registry (W11-W14) only ever grew
    /// IMPORT-direction entries (each composer's own `reads()`) -- nothing registers the REVERSE
    /// ("this domain artifact can be exported AS format Y"), because `ArtifactComposer` only models
    /// "produce my own snapshot." These entries wrap the artifact's EXISTING `🚪️io/📤️export/🧵️serializers`
    /// leaves (which already convert this artifact's snapshot straight to target-format bytes/text) as
    /// their own `ComposerEntry` rows: `writes` = the target format's dialect, `reads` = just this
    /// artifact's own dialect. `register_composer_entries` already inserts BOTH an Import key (target
    /// reads from us) and an Export key (we export to target) per entry, so no framework change was
    /// needed, only populating the missing direction. Generated by generators/w15_add_export_entries.py
    /// -- hand-validated pattern on note/json first (see that file's own tests), pilot kept as reference.
    const MODEL_DIALECT: Dialect = Dialect { artifact_kind: "s.model", standard: StandardId("1"), subset: SubsetId("*") };
    const MODEL_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::model::EnergyModelSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == MODEL_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => EnergyModelAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => EnergyModelAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "EnergyModelComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == MODEL_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::model::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "EnergyModelComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_ZIP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
    fn compose_export_zip(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::model::io::export::serializers::artifacts::zip::v2_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_ZIP_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId("*") };
    fn compose_export_csv(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::model::io::export::serializers::artifacts::csv::v_rfc4180::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_CSV_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_XLSX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    fn compose_export_xlsx(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::model::io::export::serializers::artifacts::xlsx::v_ecma_376::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_XLSX_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::model::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries


    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<EnergyModelAnyComposer>(),
            ComposerEntry { writes: EXPORT_ZIP_DIALECT, reads: &[MODEL_DIALECT], compose: compose_export_zip },
            ComposerEntry { writes: EXPORT_CSV_DIALECT, reads: &[MODEL_DIALECT], compose: compose_export_csv },
            ComposerEntry { writes: EXPORT_XLSX_DIALECT, reads: &[MODEL_DIALECT], compose: compose_export_xlsx },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[MODEL_DIALECT], compose: compose_export_json },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
