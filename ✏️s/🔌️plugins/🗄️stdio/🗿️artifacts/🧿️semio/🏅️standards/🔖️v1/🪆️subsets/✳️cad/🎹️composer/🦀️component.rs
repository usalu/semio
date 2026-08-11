//! 🎹️ SemioCadComposer (s.stdio.semio/v1/cad) — analyzer-only compose (decodes the subset's own
//! JSON-pack payload). W4 adds real cross-format compose sources once semio↔format import/export
//! leaves land (dxf/dwg/step↔cad); this composer's `reads()`/`compose()` shape already accepts an
//! arbitrary source list, so that wave is additive, not a restructure.

use semio_framework_plugin::{
    ArtifactComposer, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, ComposerEntry, Dialect, IoPayload, StandardId, SubsetId,
    SubsetValidator, SubsetValidatorEntry, register_subset_validator, subset_validator_entry_of, register_composer_entries, deserializer_entry_of, serializer_entry_of,
};
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::{CadEntity, SemioCadSnapshot};
use crate::artifacts::semio::standards::v1::subsets::cad::analyzer::SemioCadAnalyzer;
use crate::artifacts::semio::standards::v1::subsets::cad::io::import::deserializers::artifacts::dxf::v_r12::any::SemioCadFromDxf;
use crate::artifacts::semio::standards::v1::subsets::cad::io::export::serializers::artifacts::dxf::v_r12::any::SemioCadToDxf;
use crate::artifacts::semio::standards::v1::subsets::cad::io::import::deserializers::artifacts::dwg::v_ac1024::any::SemioCadFromDwg;
use crate::artifacts::semio::standards::v1::subsets::cad::io::export::serializers::artifacts::dwg::v_ac1024::any::SemioCadToDwg;
use crate::artifacts::semio::standards::v1::subsets::cad::io::import::deserializers::artifacts::step::v_ap214::any::SemioCadFromStep;
use crate::artifacts::semio::standards::v1::subsets::cad::io::export::serializers::artifacts::step::v_ap214::any::SemioCadToStep;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("cad") };

//#region 🔖️Composer
pub struct SemioCadComposer;

impl ArtifactComposer for SemioCadComposer {
    type Snapshot = SemioCadSnapshot;
    const WRITES: Dialect = DIALECT;

    fn reads() -> &'static [Dialect] { &[DIALECT] }

    fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        let native: Vec<AnalyzeSource<'_>> = sources
            .iter()
            .filter(|s| s.dialect == DIALECT)
            .map(|s| match &s.payload {
                AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
            })
            .collect();
        if native.is_empty() {
            return Err(ComposeError { message: "SemioCadComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = SemioCadAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "SemioCadComposer: analysis produced no snapshot".into(),
            diagnostics: analysis.diagnostics.clone(),
        })?;
        Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
    }
}
//#endregion 🔖️Composer

//#region 🔖️SubsetValidator
/// 🛡️ Real referential-invariant checks over the subset's OWN collections: every
/// `CadEntityRecord.layer` (top-level AND nested inside a block) must name a real `CadLayer`;
/// every `CadEntity::Insert.block_name` must name a real `CadBlock` and must not name its OWN
/// containing block (a self-referential insert is an infinite-recursion cycle, not valid content).
fn cad_referential_diagnostics(snapshot: &SemioCadSnapshot) -> Vec<dsl::Diagnostic> {
    let mut diagnostics = Vec::new();
    let layer_names: std::collections::BTreeSet<&str> = snapshot.layers.iter().map(|l| l.name.as_str()).collect();
    let block_names: std::collections::BTreeSet<&str> = snapshot.blocks.iter().map(|b| b.name.as_str()).collect();

    let check_record = |diagnostics: &mut Vec<dsl::Diagnostic>, owning_block: Option<&str>, rec: &crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::CadEntityRecord| {
        if !layer_names.contains(rec.layer.as_str()) {
            diagnostics.push(dsl::Diagnostic::error(
                "stdio.semio_cad.dangling-layer",
                dsl::TextSpan::at(1, 1),
                format!("entity {:?} (handle {:?}) references undefined layer {:?}", owning_block.unwrap_or("<top-level>"), rec.handle, rec.layer),
            ));
        }
        if let CadEntity::Insert { block_name, .. } = &rec.entity {
            if !block_names.contains(block_name.as_str()) {
                diagnostics.push(dsl::Diagnostic::error(
                    "stdio.semio_cad.dangling-block-insert",
                    dsl::TextSpan::at(1, 1),
                    format!("entity handle {:?} inserts undefined block {:?}", rec.handle, block_name),
                ));
            }
            if owning_block == Some(block_name.as_str()) {
                diagnostics.push(dsl::Diagnostic::error(
                    "stdio.semio_cad.self-referential-insert",
                    dsl::TextSpan::at(1, 1),
                    format!("block {:?} contains an Insert of itself (handle {:?}) -- infinite recursion", block_name, rec.handle),
                ));
            }
        }
    };

    for rec in &snapshot.entities {
        check_record(&mut diagnostics, None, rec);
    }
    for block in &snapshot.blocks {
        for rec in &block.entities {
            check_record(&mut diagnostics, Some(block.name.as_str()), rec);
        }
    }
    diagnostics
}

pub struct SemioCadValidator;

impl SubsetValidator for SemioCadValidator {
    const DIALECT: Dialect = DIALECT;
    fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
        let decoded = match payload {
            IoPayload::Binary(bytes) => <SemioCadSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
            IoPayload::Text(text) => <SemioCadSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
        };
        match decoded {
            Some(snapshot) => cad_referential_diagnostics(&snapshot),
            None => vec![dsl::Diagnostic::error(
                "stdio.semio_cad.validate-decode-failed",
                dsl::TextSpan::at(1, 1),
                "SemioCadValidator: payload did not decode as a SemioCadSnapshot".to_string(),
            )],
        }
    }
}

static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
fn validator_entry() -> &'static SubsetValidatorEntry { VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioCadValidator>) }
//#endregion 🔖️SubsetValidator

//#region 🔖️IoEntries
/// 🚪️ W4 (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT, group
/// G4): cad↔dxf/dwg/step. dxf is a real, complete entity-shaped bridge; dwg is an honestly
/// unsupported-content bridge (this codec's D1/D2 decode depth never reaches entity bitcode);
/// step bridges only the two AP214 curve entities (LINE/CIRCLE) with a real B-rep/solid
/// equivalent — see each pair's own leaf doc comment for the full rationale.
static IO_ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
fn io_entries() -> &'static [ComposerEntry] {
    IO_ENTRIES.get_or_init(|| vec![
        deserializer_entry_of::<SemioCadFromDxf>(), serializer_entry_of::<SemioCadToDxf>(),
        deserializer_entry_of::<SemioCadFromDwg>(), serializer_entry_of::<SemioCadToDwg>(),
        deserializer_entry_of::<SemioCadFromStep>(), serializer_entry_of::<SemioCadToStep>(),
    ]).as_slice()
}
//#endregion 🔖️IoEntries

//#region 🔖️Register
/// 📌️ Registers this subset's schema descriptor, document codec, SubsetValidator, and (W4) its
/// semio↔format io bridges. Called from this artifact's standard-level `engine::register()`.
pub fn register() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::cad::schema::semio_cad_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<SemioCadSnapshot, crate::artifacts::semio::standards::v1::subsets::cad::schema::mutations::SemioCadMutation>(crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA));
    register_subset_validator(validator_entry());
    register_composer_entries(io_entries());
}
//#endregion 🔖️Register

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint2;
    use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::{CadBlock, CadEntityRecord, CadLayer};

    #[test]
    fn validator_accepts_a_fully_referenced_snapshot() {
        let snapshot = SemioCadSnapshot {
            schema: crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
            layers: vec![CadLayer { name: "0".into(), color_index: 7, line_type: "CONTINUOUS".into(), visible: true }],
            blocks: vec![CadBlock { name: "door".into(), base_point: SemioPoint2::default(), entities: Vec::new() }],
            entities: vec![CadEntityRecord { handle: "h1".into(), layer: "0".into(), entity: CadEntity::Insert { block_name: "door".into(), insertion_point: SemioPoint2::default(), scale: SemioPoint2 { x: 1.0, y: 1.0 }, rotation: 0.0 } }],
        };
        assert!(cad_referential_diagnostics(&snapshot).is_empty());
    }

    #[test]
    fn validator_flags_dangling_layer_and_dangling_block_insert() {
        let snapshot = SemioCadSnapshot {
            schema: crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
            layers: Vec::new(),
            blocks: Vec::new(),
            entities: vec![CadEntityRecord { handle: "h1".into(), layer: "missing".into(), entity: CadEntity::Insert { block_name: "missing-block".into(), insertion_point: SemioPoint2::default(), scale: SemioPoint2 { x: 1.0, y: 1.0 }, rotation: 0.0 } }],
        };
        let diagnostics = cad_referential_diagnostics(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_cad.dangling-layer"));
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_cad.dangling-block-insert"));
    }

    #[test]
    fn validator_flags_self_referential_block_insert() {
        let snapshot = SemioCadSnapshot {
            schema: crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
            layers: vec![CadLayer { name: "0".into(), color_index: 7, line_type: "CONTINUOUS".into(), visible: true }],
            blocks: vec![CadBlock {
                name: "loopy".into(),
                base_point: SemioPoint2::default(),
                entities: vec![CadEntityRecord { handle: "h1".into(), layer: "0".into(), entity: CadEntity::Insert { block_name: "loopy".into(), insertion_point: SemioPoint2::default(), scale: SemioPoint2 { x: 1.0, y: 1.0 }, rotation: 0.0 } }],
            }],
            entities: Vec::new(),
        };
        let diagnostics = cad_referential_diagnostics(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_cad.self-referential-insert"));
    }
}
//#endregion 🔖️Tests
