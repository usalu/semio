//! 🎹️ SemioPresentationComposer (s.stdio.semio/v1/presentation) — analyzer-only compose (decodes
//! the subset's own JSON-pack payload; native-dialect only until W4 lands semio↔pptx import/export
//! leaves, matching every other subset's compose surface at this stage of the program).

use dsl::{Diagnostic, TextSpan};
use semio_framework_plugin::{
    ArtifactComposer, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId,
    SubsetValidator, SubsetValidatorEntry, register_subset_validator, subset_validator_entry_of,
    ComposerEntry, ArtifactDeserializer as _, ArtifactSerializer as _, deserializer_entry_of, serializer_entry_of, register_composer_entries,
};
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
use crate::artifacts::semio::standards::v1::subsets::presentation::analyzer::SemioPresentationAnalyzer;
use super::io::import::deserializers::artifacts::pptx::v_ecma_376::any::SemioPresentationFromPptx;
use super::io::export::serializers::artifacts::pptx::v_ecma_376::any::SemioPresentationToPptx;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("presentation") };

//#region 🔖️Composer
pub struct SemioPresentationComposer;

impl ArtifactComposer for SemioPresentationComposer {
    type Snapshot = SemioPresentationSnapshot;
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
            return Err(ComposeError { message: "SemioPresentationComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = SemioPresentationAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "SemioPresentationComposer: analysis produced no snapshot".into(),
            diagnostics: analysis.diagnostics.clone(),
        })?;
        Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
    }
}
//#endregion 🔖️Composer

//#region 🔖️SubsetValidator
/// 🛡️ Referential-invariant checks over a decoded `SemioPresentationSnapshot`: every
/// `layout.master_id` must resolve to a real `masters` entry, every `slide.layout_id` (when set)
/// must resolve to a real `layouts` entry, and `masters`/`layouts` ids must each be unique (both
/// collections are name-keyed in the diff facet — a duplicate id would silently corrupt any future
/// `between()`/`apply()` on this snapshot). Real structural checks, not a decode-only stub.
pub fn check_presentation_referential_integrity(snapshot: &SemioPresentationSnapshot) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    let mut seen_master_ids = std::collections::HashSet::new();
    for master in &snapshot.masters {
        if !seen_master_ids.insert(master.id.as_str()) {
            diagnostics.push(Diagnostic::error(
                "stdio.semio_presentation.duplicate-master-id",
                TextSpan::at(1, 1),
                format!("duplicate master id {:?}", master.id),
            ));
        }
    }
    let mut seen_layout_ids = std::collections::HashSet::new();
    for layout in &snapshot.layouts {
        if !seen_layout_ids.insert(layout.id.as_str()) {
            diagnostics.push(Diagnostic::error(
                "stdio.semio_presentation.duplicate-layout-id",
                TextSpan::at(1, 1),
                format!("duplicate layout id {:?}", layout.id),
            ));
        }
        if !seen_master_ids.contains(layout.master_id.as_str()) {
            diagnostics.push(Diagnostic::error(
                "stdio.semio_presentation.dangling-layout-master",
                TextSpan::at(1, 1),
                format!("layout {:?} references unknown master {:?}", layout.id, layout.master_id),
            ));
        }
    }
    for slide in &snapshot.slides {
        if let Some(layout_id) = &slide.layout_id {
            if !seen_layout_ids.contains(layout_id.as_str()) {
                diagnostics.push(Diagnostic::error(
                    "stdio.semio_presentation.dangling-slide-layout",
                    TextSpan::at(1, 1),
                    format!("slide {:?} references unknown layout {:?}", slide.id, layout_id),
                ));
            }
        }
    }
    diagnostics
}

pub struct SemioPresentationValidator;

impl SubsetValidator for SemioPresentationValidator {
    const DIALECT: Dialect = DIALECT;
    fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
        let decoded = match payload {
            IoPayload::Binary(bytes) => <SemioPresentationSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
            IoPayload::Text(text) => <SemioPresentationSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
        };
        match decoded {
            Some(snapshot) => check_presentation_referential_integrity(&snapshot),
            None => vec![Diagnostic::error(
                "stdio.semio_presentation.validate-decode-failed",
                TextSpan::at(1, 1),
                "SemioPresentationValidator: payload did not decode as a SemioPresentationSnapshot".to_string(),
            )],
        }
    }
}

static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
fn validator_entry() -> &'static SubsetValidatorEntry { VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioPresentationValidator>) }
//#endregion 🔖️SubsetValidator

//#region 🔖️IoEntries
/// 🚪️ presentation<->pptx bridge row (W4 G6) — one `deserializer_entry_of` (pptx -> semio) +
/// one `serializer_entry_of` (semio -> pptx); `register_composer_entries` derives all 4 `IoKey`s
/// from these 2 rows (see `document`'s own composer for the fuller doc comment on this mechanism).
static IO_ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
fn io_entries() -> &'static [ComposerEntry] {
    IO_ENTRIES.get_or_init(|| vec![deserializer_entry_of::<SemioPresentationFromPptx>(), serializer_entry_of::<SemioPresentationToPptx>()])
}
//#endregion 🔖️IoEntries

//#region 🔖️Register
/// 📌️ Registers this subset's schema descriptor, document codec, SubsetValidator, and the
/// presentation<->pptx io bridge row. Called from this artifact's standard-level `engine::register()`.
pub fn register() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::presentation::schema::semio_presentation_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<SemioPresentationSnapshot, crate::artifacts::semio::standards::v1::subsets::presentation::schema::mutations::SemioPresentationMutation>(crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA));
    register_subset_validator(validator_entry());
    register_composer_entries(io_entries());
}
//#endregion 🔖️Register

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::{Slide, SlideLayout, SlideMaster};

    fn clean_snapshot() -> SemioPresentationSnapshot {
        SemioPresentationSnapshot {
            schema: "s.stdio.semio.presentation".into(),
            masters: vec![SlideMaster { id: "m1".into(), shapes: Vec::new() }],
            layouts: vec![SlideLayout { id: "l1".into(), master_id: "m1".into(), shapes: Vec::new() }],
            slides: vec![Slide { id: "s1".into(), layout_id: Some("l1".into()), shapes: Vec::new(), notes: Vec::new() }],
        }
    }

    #[test]
    fn clean_snapshot_has_no_diagnostics() {
        assert!(check_presentation_referential_integrity(&clean_snapshot()).is_empty());
    }

    #[test]
    fn dangling_layout_master_is_flagged() {
        let mut snap = clean_snapshot();
        snap.layouts[0].master_id = "missing".into();
        let diagnostics = check_presentation_referential_integrity(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_presentation.dangling-layout-master"), "got {diagnostics:?}");
    }

    #[test]
    fn dangling_slide_layout_is_flagged() {
        let mut snap = clean_snapshot();
        snap.slides[0].layout_id = Some("missing".into());
        let diagnostics = check_presentation_referential_integrity(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_presentation.dangling-slide-layout"), "got {diagnostics:?}");
    }

    #[test]
    fn duplicate_master_id_is_flagged() {
        let mut snap = clean_snapshot();
        snap.masters.push(SlideMaster { id: "m1".into(), shapes: Vec::new() });
        let diagnostics = check_presentation_referential_integrity(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_presentation.duplicate-master-id"), "got {diagnostics:?}");
    }

    #[test]
    fn slide_with_no_layout_is_never_flagged() {
        let mut snap = clean_snapshot();
        snap.slides[0].layout_id = None;
        assert!(check_presentation_referential_integrity(&snap).is_empty());
    }

    #[test]
    fn validator_roundtrips_through_pack_payload() {
        let bytes = <SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(&clean_snapshot());
        let diagnostics = SemioPresentationValidator::validate(&IoPayload::Binary(bytes));
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
    }

    /// 🔁️ W4 G6 fixture-backed round trip: pptx1 -(deserialize)-> semio1 -(serialize)-> pptx2
    /// -(deserialize)-> semio2, asserting semio1 == semio2 (masters/layouts/slide id/notes/Table
    /// shapes are the documented lossy fields — this fixture avoids them by construction, so the
    /// comparison exercises TextBox/Picture/Placeholder shape fidelity end to end).
    #[test]
    fn pptx_round_trip_is_stable() {
        use crate::artifacts::pptx::PptxSnapshot;
        use crate::artifacts::pptx::schema::snapshot::{PptxParagraph, PptxPresentation, PptxRun, PptxShape, PptxSlide, PptxTransform};
        use crate::artifacts::zip::opc::OpcPackage;

        let pptx1 = PptxSnapshot::from_parts(
            OpcPackage::default(),
            PptxPresentation {
                slides: vec![PptxSlide {
                    shapes: vec![
                        PptxShape::TextBox { text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "Hello".into(), bold: true, italic: false, font_size: Some(24) }] }], position: PptxTransform { x: 0, y: 0, cx: 100, cy: 20 } },
                        PptxShape::Picture { blip_rel_id: "rId2".into(), position: PptxTransform { x: 0, y: 30, cx: 50, cy: 50 } },
                        PptxShape::Placeholder { kind: "title".into(), text_frame: Vec::new(), position: PptxTransform { x: 0, y: 0, cx: 200, cy: 40 } },
                    ],
                }],
            },
        );
        let semio1 = SemioPresentationFromPptx::deserialize(&pptx1).expect("deserialize");
        let pptx2 = SemioPresentationToPptx::serialize(&semio1).expect("serialize");
        let semio2 = SemioPresentationFromPptx::deserialize(&pptx2).expect("deserialize round 2");
        assert_eq!(semio1, semio2);
    }
}
//#endregion 🧪️Tests
