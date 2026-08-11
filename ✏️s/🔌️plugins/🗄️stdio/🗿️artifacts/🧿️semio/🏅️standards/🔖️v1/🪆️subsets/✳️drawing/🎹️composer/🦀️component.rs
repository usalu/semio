//! 🎹️ SemioDrawingComposer (s.stdio.semio/v1/drawing) — analyzer-only compose (decodes the
//! subset's own JSON-pack payload). W4 adds real cross-format compose sources once
//! semio↔format import/export leaves land.

use semio_framework_plugin::{
    ArtifactComposer, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, ComposerEntry, Dialect, IoPayload, StandardId, SubsetId,
    SubsetValidator, SubsetValidatorEntry, register_subset_validator, subset_validator_entry_of, register_composer_entries, deserializer_entry_of, serializer_entry_of,
};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};
use crate::artifacts::semio::standards::v1::subsets::drawing::analyzer::SemioDrawingAnalyzer;
use crate::artifacts::semio::standards::v1::subsets::drawing::io::import::deserializers::artifacts::svg::v1_1::any::SemioDrawingFromSvg;
use crate::artifacts::semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::svg::v1_1::any::SemioDrawingToSvg;
use crate::artifacts::semio::standards::v1::subsets::drawing::io::import::deserializers::artifacts::dxf::v_r12::any::SemioDrawingFromDxf;
use crate::artifacts::semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::dxf::v_r12::any::SemioDrawingToDxf;
use crate::artifacts::semio::standards::v1::subsets::drawing::io::import::deserializers::artifacts::pdf::v1_7::any::SemioDrawingFromPdf;
use crate::artifacts::semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::pdf::v1_7::any::SemioDrawingToPdf;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };

//#region 🔖️Composer
pub struct SemioDrawingComposer;

impl ArtifactComposer for SemioDrawingComposer {
    type Snapshot = SemioDrawingSnapshot;
    const WRITES: Dialect = DIALECT;

    fn reads() -> &'static [Dialect] { &[DIALECT] }

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        let native: Vec<AnalyzeSource<'_>> = sources
            .iter()
            .filter(|s| s.dialect == DIALECT)
            .map(|s| match &s.payload {
                AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
            })
            .collect();
        if native.is_empty() {
            return Err(ComposeError { message: "SemioDrawingComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = SemioDrawingAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "SemioDrawingComposer: analysis produced no snapshot".into(),
            diagnostics: analysis.diagnostics.clone(),
        })?;
        Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
    }
}
//#endregion 🔖️Composer

//#region 🔖️SubsetValidator
/// 🛡️ Decodes the payload as `SemioDrawingSnapshot` and checks two real referential invariants
/// (both real cross-collection lookups, not decode-only): (1) every `Path`/`Text` node's
/// `style` reference resolves to a name present in `styles` (dangling-ref detection); (2) every
/// `DrawLayer.id` is unique across `layers` (duplicate-id detection).
pub struct SemioDrawingValidator;

impl SubsetValidator for SemioDrawingValidator {
    const DIALECT: Dialect = DIALECT;
    fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
        let decoded = match payload {
            IoPayload::Binary(bytes) => <SemioDrawingSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
            IoPayload::Text(text) => <SemioDrawingSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
        };
        match decoded {
            Some(snapshot) => check_drawing_invariants(&snapshot),
            None => vec![dsl::Diagnostic::error(
                "stdio.semio_drawing.validate-decode-failed",
                dsl::TextSpan::at(1, 1),
                "SemioDrawingValidator: payload did not decode as a SemioDrawingSnapshot".to_string(),
            )],
        }
    }
}

/// 🔎️ Real referential-invariant checks over `SemioDrawingSnapshot`'s own collections (no
/// cross-artifact lookups needed -- both invariants are internal to this subset).
pub fn check_drawing_invariants(snapshot: &SemioDrawingSnapshot) -> Vec<dsl::Diagnostic> {
    let mut diagnostics = Vec::new();

    let mut seen_layer_ids = std::collections::HashSet::new();
    for layer in &snapshot.layers {
        if !seen_layer_ids.insert(layer.id.clone()) {
            diagnostics.push(dsl::Diagnostic::error(
                "stdio.semio_drawing.duplicate-layer-id",
                dsl::TextSpan::at(1, 1),
                format!("SemioDrawingValidator: duplicate layer id {:?}", layer.id),
            ));
        }
    }

    fn walk(node: &DrawNode, style_names: &std::collections::HashSet<&str>, diagnostics: &mut Vec<dsl::Diagnostic>) {
        match node {
            DrawNode::Path { style: Some(name), .. } | DrawNode::Text { style: Some(name), .. } => {
                if !style_names.contains(name.as_str()) {
                    diagnostics.push(dsl::Diagnostic::error(
                        "stdio.semio_drawing.dangling-style-ref",
                        dsl::TextSpan::at(1, 1),
                        format!("SemioDrawingValidator: node references undefined style {name:?}"),
                    ));
                }
            }
            DrawNode::Group { children, .. } => {
                for child in children {
                    walk(child, style_names, diagnostics);
                }
            }
            _ => {}
        }
    }
    let style_names: std::collections::HashSet<&str> = snapshot.styles.iter().map(|s| s.name.as_str()).collect();
    for layer in &snapshot.layers {
        walk(&layer.root, &style_names, &mut diagnostics);
    }

    diagnostics
}

static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
fn validator_entry() -> &'static SubsetValidatorEntry { VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioDrawingValidator>) }
//#endregion 🔖️SubsetValidator

//#region 🔖️IoEntries
/// 🚪️ W4 (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT, group
/// G4): drawing↔svg/dxf/pdf. svg is the richest (recursive scene-graph↔scene-graph); dxf is a
/// real entity↔path translation (exact circles, sampled-flattened curves on export); pdf is an
/// honestly text-only bridge (this codec's own snapshot never exposes decoded content-stream
/// vector ops) — see each pair's own leaf doc comment for the full rationale.
static IO_ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
fn io_entries() -> &'static [ComposerEntry] {
    IO_ENTRIES.get_or_init(|| vec![
        deserializer_entry_of::<SemioDrawingFromSvg>(), serializer_entry_of::<SemioDrawingToSvg>(),
        deserializer_entry_of::<SemioDrawingFromDxf>(), serializer_entry_of::<SemioDrawingToDxf>(),
        deserializer_entry_of::<SemioDrawingFromPdf>(), serializer_entry_of::<SemioDrawingToPdf>(),
    ]).as_slice()
}
//#endregion 🔖️IoEntries

//#region 🔖️Register
/// 📌️ Registers this subset's schema descriptor, document codec, SubsetValidator, and (W4) its
/// semio↔format io bridges. Called from this artifact's standard-level `engine::register()`.
pub fn register() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::drawing::schema::semio_drawing_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<SemioDrawingSnapshot, crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation>(crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::STDIO_SEMIODRAWING_DOCUMENT_SCHEMA));
    register_subset_validator(validator_entry());
    register_composer_entries(io_entries());
}
//#endregion 🔖️Register

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::engine::geometry::{SemioPoint2, SemioTransform};
    use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawLayer, DrawStyle};

    #[test]
    fn dangling_style_ref_and_duplicate_layer_id_are_both_reported() {
        let snapshot = SemioDrawingSnapshot {
            styles: vec![DrawStyle { name: "ok".into(), fill: None, stroke: None, stroke_width: None, opacity: None }],
            layers: vec![
                DrawLayer { id: "dup".into(), name: "a".into(), visible: true, root: DrawNode::Path { segments: vec![], style: Some("missing".into()) } },
                DrawLayer { id: "dup".into(), name: "b".into(), visible: true, root: DrawNode::Text { value: "t".into(), at: SemioPoint2::default(), style: Some("ok".into()) } },
            ],
            ..SemioDrawingSnapshot::default()
        };
        let diagnostics = check_drawing_invariants(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_drawing.dangling-style-ref"), "{diagnostics:?}");
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_drawing.duplicate-layer-id"), "{diagnostics:?}");
    }

    #[test]
    fn clean_snapshot_reports_no_diagnostics() {
        let snapshot = SemioDrawingSnapshot {
            styles: vec![DrawStyle { name: "ok".into(), fill: None, stroke: None, stroke_width: None, opacity: None }],
            layers: vec![DrawLayer { id: "l0".into(), name: "a".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children: vec![DrawNode::Path { segments: vec![], style: Some("ok".into()) }] } }],
            ..SemioDrawingSnapshot::default()
        };
        assert!(check_drawing_invariants(&snapshot).is_empty());
    }
}
//#endregion 🔖️Tests
