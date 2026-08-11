//! 🎹️ SemioModelComposer (s.stdio.semio/v1/model) — analyzer-only compose (decodes the subset's
//! own JSON-pack payload) PLUS the real W4 semio↔format bridge entries (model↔ifc, model↔bcf),
//! registered value-level via `deserializer_entry_of`/`serializer_entry_of`.

use semio_framework_plugin::{
    ArtifactComposer, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, ComposerEntry, Dialect, IoPayload, StandardId, SubsetId,
    SubsetValidator, SubsetValidatorEntry, register_composer_entries, register_subset_validator, subset_validator_entry_of, deserializer_entry_of, serializer_entry_of,
};
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::artifacts::semio::standards::v1::subsets::model::analyzer::SemioModelAnalyzer;
use std::collections::HashSet;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("model") };

//#region 🔖️Composer
pub struct SemioModelComposer;

impl ArtifactComposer for SemioModelComposer {
    type Snapshot = SemioModelSnapshot;
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
            return Err(ComposeError { message: "SemioModelComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = SemioModelAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "SemioModelComposer: analysis produced no snapshot".into(),
            diagnostics: analysis.diagnostics.clone(),
        })?;
        Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
    }
}
//#endregion 🔖️Composer

//#region 🔖️SubsetValidator
/// 🛡️ Real referential-invariant checks over `model`'s OWN collections (decode + dangling-id
/// checks): a spatial node's `parent_id`, an element's `spatial_id`, and a relation's `from`/`to`
/// must all resolve within THIS snapshot's own `spatial`/`elements` id spaces. Cross-subset
/// references (`GeometryRef::Brep{brep_id}`/`Mesh{mesh_id}` into the sibling `brep`/`mesh`
/// subsets) are NOT checked here — they are not decodable from a `model` snapshot alone, per the
/// snapshot module's own doc comment.
pub struct SemioModelValidator;

/// 🔎️ Dangling-reference diagnostics for a decoded snapshot — split out from `validate()` so it's
/// directly unit-testable against a typed `SemioModelSnapshot` (not just through the `IoPayload`
/// wire boundary).
pub fn semio_model_referential_diagnostics(snapshot: &SemioModelSnapshot) -> Vec<dsl::Diagnostic> {
    let spatial_ids: HashSet<&str> = snapshot.spatial.iter().map(|n| n.id.as_str()).collect();
    let element_ids: HashSet<&str> = snapshot.elements.iter().map(|e| e.id.as_str()).collect();
    let mut diagnostics = Vec::new();

    for node in &snapshot.spatial {
        if let Some(parent) = &node.parent_id {
            if parent == &node.id {
                diagnostics.push(dsl::Diagnostic::error(
                    "stdio.semio_model.validate-self-parent",
                    dsl::TextSpan::at(1, 1),
                    format!("spatial node {:?} is its own parent", node.id),
                ));
            } else if !spatial_ids.contains(parent.as_str()) {
                diagnostics.push(dsl::Diagnostic::error(
                    "stdio.semio_model.validate-dangling-parent",
                    dsl::TextSpan::at(1, 1),
                    format!("spatial node {:?} references missing parent {:?}", node.id, parent),
                ));
            }
        }
    }

    for element in &snapshot.elements {
        if let Some(spatial_id) = &element.spatial_id {
            if !spatial_ids.contains(spatial_id.as_str()) {
                diagnostics.push(dsl::Diagnostic::error(
                    "stdio.semio_model.validate-dangling-spatial-ref",
                    dsl::TextSpan::at(1, 1),
                    format!("element {:?} references missing spatial node {:?}", element.id, spatial_id),
                ));
            }
        }
    }

    for relation in &snapshot.relations {
        let endpoint_known = |id: &str| element_ids.contains(id) || spatial_ids.contains(id);
        if !endpoint_known(&relation.from) {
            diagnostics.push(dsl::Diagnostic::error(
                "stdio.semio_model.validate-dangling-relation-from",
                dsl::TextSpan::at(1, 1),
                format!("relation {:?} references missing from-id {:?}", relation.id, relation.from),
            ));
        }
        if !endpoint_known(&relation.to) {
            diagnostics.push(dsl::Diagnostic::error(
                "stdio.semio_model.validate-dangling-relation-to",
                dsl::TextSpan::at(1, 1),
                format!("relation {:?} references missing to-id {:?}", relation.id, relation.to),
            ));
        }
    }

    diagnostics
}

impl SubsetValidator for SemioModelValidator {
    const DIALECT: Dialect = DIALECT;
    fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
        let decoded = match payload {
            IoPayload::Binary(bytes) => <SemioModelSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
            IoPayload::Text(text) => <SemioModelSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
        };
        match decoded {
            Some(snapshot) => semio_model_referential_diagnostics(&snapshot),
            None => vec![dsl::Diagnostic::error(
                "stdio.semio_model.validate-decode-failed",
                dsl::TextSpan::at(1, 1),
                "SemioModelValidator: payload did not decode as a SemioModelSnapshot".to_string(),
            )],
        }
    }
}

static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
fn validator_entry() -> &'static SubsetValidatorEntry { VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioModelValidator>) }
//#endregion 🔖️SubsetValidator

//#region 🔖️Register
/// 📌️ Registers this subset's schema descriptor, document codec, and SubsetValidator. Called from
/// this artifact's standard-level `engine::register()`.
pub fn register() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::model::schema::semio_model_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<SemioModelSnapshot, crate::artifacts::semio::standards::v1::subsets::model::schema::mutations::SemioModelMutation>(crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::STDIO_SEMIOMODEL_DOCUMENT_SCHEMA));
    register_subset_validator(validator_entry());
    register_composer_entries(io_bridge_entries());
}
//#endregion 🔖️Register

//#region 🔖️IoBridges
/// 🌉️ W4 real semio↔format bridge entries. Each `deserializer_entry_of`/`serializer_entry_of`
/// pair registers BOTH `IoKey` directions per `register_composer_entries`'s own doc comment (a
/// deserializer writing `model`/reading `<format>` also gives `<format>`-exports-to-`model`; its
/// mirror serializer gives the other two) — four `IoKey`s per (subset, format) pair from these two
/// rows, no hand-written reverse registration needed.
fn io_bridge_entries() -> &'static [ComposerEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            vec![
                deserializer_entry_of::<crate::artifacts::semio::standards::v1::subsets::model::io::import::deserializers::artifacts::ifc::v4::any::SemioModelFromIfc>(),
                serializer_entry_of::<crate::artifacts::semio::standards::v1::subsets::model::io::export::serializers::artifacts::ifc::v4::any::SemioModelToIfc>(),
                deserializer_entry_of::<crate::artifacts::semio::standards::v1::subsets::model::io::import::deserializers::artifacts::bcf::v2_1::any::SemioModelFromBcf>(),
                serializer_entry_of::<crate::artifacts::semio::standards::v1::subsets::model::io::export::serializers::artifacts::bcf::v2_1::any::SemioModelToBcf>(),
            ]
        })
        .as_slice()
}
//#endregion 🔖️IoBridges

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::{ElementClass, GeometryRef, ModelRelation, RelationKind, SemioModelElement, SpatialKind, SpatialNode};
    use crate::artifacts::semio::standards::v1::engine::geometry::SemioTransform;

    fn clean_snapshot() -> SemioModelSnapshot {
        SemioModelSnapshot {
            schema: SemioModelSnapshot::default().schema,
            spatial: vec![SpatialNode { id: "s1".into(), kind: SpatialKind::Site, name: "Site".into(), parent_id: None, placement: SemioTransform::identity() }],
            elements: vec![SemioModelElement { id: "e1".into(), class: ElementClass::Wall, placement: SemioTransform::identity(), geometry: GeometryRef::None, spatial_id: Some("s1".into()), psets: vec![] }],
            relations: vec![ModelRelation { id: "r1".into(), kind: RelationKind::Aggregates, from: "e1".into(), to: "s1".into() }],
        }
    }

    #[test]
    fn clean_snapshot_has_no_referential_diagnostics() {
        let diagnostics = semio_model_referential_diagnostics(&clean_snapshot());
        assert!(diagnostics.is_empty(), "expected no diagnostics, got {diagnostics:?}");
    }

    #[test]
    fn dangling_spatial_parent_is_flagged() {
        let mut snap = clean_snapshot();
        snap.spatial[0].parent_id = Some("missing".into());
        let diagnostics = semio_model_referential_diagnostics(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_model.validate-dangling-parent"), "got {diagnostics:?}");
    }

    #[test]
    fn dangling_element_spatial_ref_is_flagged() {
        let mut snap = clean_snapshot();
        snap.elements[0].spatial_id = Some("missing".into());
        let diagnostics = semio_model_referential_diagnostics(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_model.validate-dangling-spatial-ref"), "got {diagnostics:?}");
    }

    #[test]
    fn dangling_relation_endpoints_are_flagged() {
        let mut snap = clean_snapshot();
        snap.relations[0].from = "missing-from".into();
        snap.relations[0].to = "missing-to".into();
        let diagnostics = semio_model_referential_diagnostics(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_model.validate-dangling-relation-from"), "got {diagnostics:?}");
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_model.validate-dangling-relation-to"), "got {diagnostics:?}");
    }

    #[test]
    fn validator_validate_runs_the_same_checks_through_the_io_payload_boundary() {
        let bytes = <SemioModelSnapshot as store::ArtifactPack>::encode_pack(&clean_snapshot());
        let diagnostics = SemioModelValidator::validate(&IoPayload::Binary(bytes));
        assert!(diagnostics.is_empty(), "clean snapshot must validate through the wire boundary too: {diagnostics:?}");
    }
}
//#endregion 🔖️Tests
