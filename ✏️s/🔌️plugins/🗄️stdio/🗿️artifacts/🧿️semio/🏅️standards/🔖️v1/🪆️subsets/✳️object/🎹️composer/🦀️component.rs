//! 🎹️ SemioObjectComposer (s.stdio.semio/v1/object) — analyzer-only compose (decodes the
//! subset's own JSON-pack payload) PLUS the real W4 semio↔format bridge entries (object↔json,
//! object↔xml, object↔csv), registered value-level via `deserializer_entry_of`/`serializer_entry_of`.

use semio_framework_plugin::{
    ArtifactComposer, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, ComposerEntry, Dialect, IoPayload, StandardId, SubsetId,
    SubsetValidator, SubsetValidatorEntry, register_composer_entries, register_subset_validator, subset_validator_entry_of, deserializer_entry_of, serializer_entry_of,
};
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::{ObjectId, SemioObjectSnapshot, SemioValue};
use crate::artifacts::semio::standards::v1::subsets::object::analyzer::SemioObjectAnalyzer;
use std::collections::HashSet;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("object") };

//#region 🔖️Composer
pub struct SemioObjectComposer;

impl ArtifactComposer for SemioObjectComposer {
    type Snapshot = SemioObjectSnapshot;
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
            return Err(ComposeError { message: "SemioObjectComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = SemioObjectAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "SemioObjectComposer: analysis produced no snapshot".into(),
            diagnostics: analysis.diagnostics.clone(),
        })?;
        Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
    }
}
//#endregion 🔖️Composer

//#region 🔖️SubsetValidator
/// 🕸️ Recursively collects every `Ref{id}` reachable from `value` — used against BOTH `root` and
/// every `objects` node's own `value` (a `Ref` can legally point from inside the graph back into
/// itself, or into a sibling node, not only from `root`).
fn collect_refs(value: &SemioValue, out: &mut Vec<ObjectId>) {
    match value {
        SemioValue::Ref { id } => out.push(id.clone()),
        SemioValue::List { items } => items.iter().for_each(|v| collect_refs(v, out)),
        SemioValue::Map { entries } => entries.iter().for_each(|e| collect_refs(&e.value, out)),
        _ => {}
    }
}

/// 🛡️ Decodes the payload as this subset's OWN `SemioObjectSnapshot`, then checks two real
/// referential invariants over its own collections: (1) every `Ref{id}` reachable from `root` or
/// from any `objects` node's value resolves to a real entry in `objects` (no dangling ids); (2)
/// `objects` carries no duplicate `id` (the graph's backing store is id-ADDRESSABLE, a duplicate
/// id makes resolution ambiguous).
pub struct SemioObjectValidator;

impl SubsetValidator for SemioObjectValidator {
    const DIALECT: Dialect = DIALECT;
    fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
        let decoded = match payload {
            IoPayload::Binary(bytes) => <SemioObjectSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
            IoPayload::Text(text) => <SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
        };
        let snapshot = match decoded {
            Some(snapshot) => snapshot,
            None => {
                return vec![dsl::Diagnostic::error(
                    "stdio.semio_object.validate-decode-failed",
                    dsl::TextSpan::at(1, 1),
                    "SemioObjectValidator: payload did not decode as a SemioObjectSnapshot".to_string(),
                )];
            }
        };

        let mut diagnostics = Vec::new();

        let known_ids: HashSet<&ObjectId> = snapshot.objects.iter().map(|n| &n.id).collect();
        let mut seen_ids: HashSet<&ObjectId> = HashSet::new();
        for node in &snapshot.objects {
            if !seen_ids.insert(&node.id) {
                diagnostics.push(dsl::Diagnostic::error(
                    "stdio.semio_object.validate-duplicate-id",
                    dsl::TextSpan::at(1, 1),
                    format!("SemioObjectValidator: duplicate object id '{}' in `objects`", node.id.value),
                ));
            }
        }

        let mut refs = Vec::new();
        collect_refs(&snapshot.root, &mut refs);
        for node in &snapshot.objects {
            collect_refs(&node.value, &mut refs);
        }
        let mut reported_dangling: HashSet<String> = HashSet::new();
        for id in refs {
            if !known_ids.contains(&id) && reported_dangling.insert(id.value.clone()) {
                diagnostics.push(dsl::Diagnostic::error(
                    "stdio.semio_object.validate-dangling-ref",
                    dsl::TextSpan::at(1, 1),
                    format!("SemioObjectValidator: Ref{{id: '{}'}} does not resolve to any entry in `objects`", id.value),
                ));
            }
        }

        diagnostics
    }
}

static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
fn validator_entry() -> &'static SubsetValidatorEntry { VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioObjectValidator>) }
//#endregion 🔖️SubsetValidator

//#region 🔖️Register
/// 📌️ Registers this subset's schema descriptor, document codec, and SubsetValidator. Called from
/// this artifact's standard-level `engine::register()`.
pub fn register() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::object::schema::semio_object_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<SemioObjectSnapshot, crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation>(crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA));
    register_subset_validator(validator_entry());
    register_composer_entries(io_bridge_entries());
}
//#endregion 🔖️Register

//#region 🔖️IoBridges
/// 🌉️ W4 real semio↔format bridge entries. Each `deserializer_entry_of`/`serializer_entry_of`
/// pair registers BOTH `IoKey` directions per `register_composer_entries`'s own doc comment.
fn io_bridge_entries() -> &'static [ComposerEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            vec![
                deserializer_entry_of::<crate::artifacts::semio::standards::v1::subsets::object::io::import::deserializers::artifacts::json::v_rfc8259::any::SemioObjectFromJson>(),
                serializer_entry_of::<crate::artifacts::semio::standards::v1::subsets::object::io::export::serializers::artifacts::json::v_rfc8259::any::SemioObjectToJson>(),
                deserializer_entry_of::<crate::artifacts::semio::standards::v1::subsets::object::io::import::deserializers::artifacts::xml::v1_0::any::SemioObjectFromXml>(),
                serializer_entry_of::<crate::artifacts::semio::standards::v1::subsets::object::io::export::serializers::artifacts::xml::v1_0::any::SemioObjectToXml>(),
                deserializer_entry_of::<crate::artifacts::semio::standards::v1::subsets::object::io::import::deserializers::artifacts::csv::v_rfc4180::any::SemioObjectFromCsv>(),
                serializer_entry_of::<crate::artifacts::semio::standards::v1::subsets::object::io::export::serializers::artifacts::csv::v_rfc4180::any::SemioObjectToCsv>(),
            ]
        })
        .as_slice()
}
//#endregion 🔖️IoBridges
