//! 🧬️ EnergyModel snapshot schema — artifact-lane fields only.

use crate::{energy_snapshot_with_state, EnergyStructureChild, EnergyZonesChild, ENERGY_MODEL_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;
use semio_framework_os_kernel::{from_dsl_value, to_dsl_value, DslValue, FromValue, ToValue, ValueError};

//#region 🔖️Snapshot
/// 📸️ Persisted energy-model document snapshot (persistent fields of the artifact). Ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`energy→C:value,table R:model`): the old
/// `model_json: String` opaque-JSON field is replaced by two fixed composed CHILD slots — this
/// artifact no longer defines its own persisted-value/table content model, it composes stdio's
/// `value`/`table` subsets instead (see the artifact root's `🔖️Composition` region for the full
/// before/after and the honest exception carve-out for `Surface.vertices_m`). `referenced_model` is
/// a new forward `ArtifactLink` slot. `#[child(...)]`/`#[link_slot(...)]` drive
/// `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written. Text and pack both encode
/// the derived `EnergyModelPackRecord` below.
#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.energy.model")]
pub struct EnergyModelSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub model: crate::model::Model,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub structure: EnergyStructureChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub zones: EnergyZonesChild,
    #[state(artifact)]
    #[link_slot(roles("model"))]
    pub referenced_model: Option<store::ArtifactLink>,
    /// 🌦️ Forward link to the `🌦️epw` stdio artifact this model is simulated against — a link slot
    /// exactly like `referenced_model`, never an inlined `EpwWeather` (ticket
    /// 26/09/06/ENERGY-PLUGIN-END-TO-END).
    #[state(artifact)]
    #[link_slot(roles("weather"))]
    pub weather_link: Option<store::ArtifactLink>,
}

impl Default for EnergyModelSnapshot {
    fn default() -> Self {
        energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, &crate::model::Model::default(), None)
    }
}

// 🌱️ Hand-written, not derived — `structure`/`zones` are `store::ArtifactChild<S>` and
// `referenced_model` is `Option<store::ArtifactLink>`, neither of which has a
// `#[derive(ToValue, FromValue)]`-reachable impl (fan-out playbook trap #3; `ArtifactLink` mirrors
// the same framework-exempt shape). `model: crate::model::Model` goes through `ToValue`/`FromValue`
// directly — `Model` now derives both.
impl ToValue for EnergyModelSnapshot {
    fn to_value(&self) -> DslValue {
        DslValue::object([
            ("schema".to_string(), self.schema.to_value()),
            ("model".to_string(), self.model.to_value()),
            ("structure".to_string(), to_dsl_value(&self.structure).unwrap_or(DslValue::Null)),
            ("zones".to_string(), to_dsl_value(&self.zones).unwrap_or(DslValue::Null)),
            ("referencedModel".to_string(), to_dsl_value(&self.referenced_model).unwrap_or(DslValue::Null)),
            ("weatherLink".to_string(), to_dsl_value(&self.weather_link).unwrap_or(DslValue::Null)),
        ])
    }
}
impl FromValue for EnergyModelSnapshot {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = DslValue::into_object(value)?;
        let field = |key: &str| entries.iter().find(|(k, _)| k == key).map_or(DslValue::Null, |(_, v)| v.clone());
        Ok(Self {
            schema: String::from_value(field("schema"))?,
            model: crate::model::Model::from_value(field("model"))?,
            structure: from_dsl_value(field("structure")).map_err(ValueError::new)?,
            zones: from_dsl_value(field("zones")).map_err(ValueError::new)?,
            referenced_model: from_dsl_value(field("referencedModel")).map_err(ValueError::new)?,
            weather_link: from_dsl_value(field("weatherLink")).map_err(ValueError::new)?,
        })
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️PackRecord
/// 🔋️ Derived pack record of an `EnergyModelSnapshot` — every field as persisted, with the typed
/// `model` carried as its first-party value.
#[derive(dsl::DslRecord)]
#[dsl(extension = "energy")]
struct EnergyModelPackRecord {
    schema: String,
    model: DslValue,
    structure: EnergyStructureChild,
    zones: EnergyZonesChild,
    referenced_model: Option<store::ArtifactLink>,
    weather_link: Option<store::ArtifactLink>,
}

impl EnergyModelPackRecord {
    fn from_snapshot(snapshot: &EnergyModelSnapshot) -> Self {
        Self { schema: snapshot.schema.clone(), model: snapshot.model.to_value(), structure: snapshot.structure.clone(), zones: snapshot.zones.clone(), referenced_model: snapshot.referenced_model.clone(), weather_link: snapshot.weather_link.clone() }
    }

    fn into_snapshot(self) -> Result<EnergyModelSnapshot, String> {
        let model = crate::model::Model::from_value(self.model).map_err(|error| error.to_string())?;
        Ok(EnergyModelSnapshot { schema: self.schema, model, structure: self.structure, zones: self.zones, referenced_model: self.referenced_model, weather_link: self.weather_link })
    }
}

/// 🖨️ The derived text body: the same `EnergyModelPackRecord` the pack encodes, printed by the spec-driven engine.
pub(crate) fn print_pack_record_text(snapshot: &EnergyModelSnapshot) -> String {
    dsl::print(&EnergyModelPackRecord::from_snapshot(snapshot).__dsl_to_record(), &EnergyModelPackRecord::__dsl_spec(), dsl::JoinMode::Document)
}

/// 📖️ Parses a derived text body back through `EnergyModelPackRecord`, with the same decode steps as the pack.
pub(crate) fn parse_pack_record_text(body: &str) -> Result<EnergyModelSnapshot, store::TextError> {
    let record = dsl::parse(body, &EnergyModelPackRecord::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
    EnergyModelPackRecord::__dsl_from_record(&record)?.into_snapshot().map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))
}
//#endregion 🔖️PackRecord

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for EnergyModelSnapshot {
    const EXTENSION: &'static str = "energy";
    fn envelope_id() -> &'static str {
        "energy.model"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_pack_record_text(body)
    }
    fn print_dsl(&self) -> String {
        let body = print_pack_record_text(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for EnergyModelSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&EnergyModelPackRecord::__dsl_spec(), &EnergyModelPackRecord::from_snapshot(self).__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &EnergyModelPackRecord::__dsl_spec(), options)?;
        EnergyModelPackRecord::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot().map_err(store::PackError::Schema)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(EnergyModelPackRecord::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️round-trip/🦀️.rs"]
mod round_trip_tests;
//#endregion 🧪️Tests

//#region 🌉️IdentityBridge
/// 🔁️ One JSON report of carrying `dsl_text` through this subset's own codecs, for a
/// language-neutral test adapter. Same reachability wall as `energy_model_mutation_report_json`:
/// `store::ArtifactDsl`/`store::ArtifactPack` and their error types are unnameable outside this
/// crate, so the identity law's evidence has to be produced here and handed over as text.
///
/// `canonicalText` is `print_dsl` of the parsed document and `canonicalTextAgain` is `print_dsl` of
/// re-parsing that — [`store::ArtifactDsl`]'s own documented LAW is that canonical output is a
/// `parse_dsl` fixpoint (hand-written text may normalize on the way in), so the two must be
/// byte-identical while neither is required to equal the committed file. `packDecoded` comes back
/// through a SEPARATE binary codec, so agreeing on one snapshot cannot be achieved by carrying text
/// bytes across.
pub fn energy_model_identity_report_json(dsl_text: &str) -> Result<String, String> {
    let parsed = <EnergyModelSnapshot as store::ArtifactDsl>::parse_dsl(dsl_text).map_err(|error| error.to_string())?;
    let canonical = <EnergyModelSnapshot as store::ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <EnergyModelSnapshot as store::ArtifactDsl>::parse_dsl(&canonical).map_err(|error| error.to_string())?;
    let canonical_again = <EnergyModelSnapshot as store::ArtifactDsl>::print_dsl(&reparsed);
    let packed = <EnergyModelSnapshot as store::ArtifactPack>::encode_pack(&reparsed);
    let unpacked = <EnergyModelSnapshot as store::ArtifactPack>::decode_pack(&packed).map_err(|error| error.to_string())?;
    let report = pack::json::object([
        ("parsed".to_string(), pack::json::from_dsl_value(&parsed.to_value())),
        ("reparsed".to_string(), pack::json::from_dsl_value(&reparsed.to_value())),
        ("packDecoded".to_string(), pack::json::from_dsl_value(&unpacked.to_value())),
        ("canonicalText".to_string(), pack::json::Value::String(canonical)),
        ("canonicalTextAgain".to_string(), pack::json::Value::String(canonical_again)),
    ]);
    Ok(pack::json::to_string(&report))
}
//#endregion 🌉️IdentityBridge

