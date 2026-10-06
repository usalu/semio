//! 📦️ EnergyModel artifact — binary document surface + laws.

use crate::EnergyModelSnapshot;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes an `EnergyModelSnapshot` to its binary pack form.
pub fn encode(document: &EnergyModelSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes an `EnergyModelSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<EnergyModelSnapshot, PackError> {
    <EnergyModelSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{energy_snapshot_with_state, EnergyStructureChild, EnergyZonesChild, ENERGY_MODEL_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;

/// 🔋️ Derived pack record of an `EnergyModelSnapshot` — every field as persisted, with the typed
/// `model` carried as its first-party value.
#[derive(semio_framework_dsl_record_derive::DslRecord)]
#[dsl(extension = "energy")]
pub(crate) struct EnergyModelPackRecord {
    schema: String,
    model: DslValue,
    structure: EnergyStructureChild,
    zones: EnergyZonesChild,
    referenced_model: Option<store::ArtifactLink>,
    weather_link: Option<store::ArtifactLink>,
}

impl EnergyModelPackRecord {
    pub(crate) fn from_snapshot(snapshot: &EnergyModelSnapshot) -> Self {
        Self { schema: snapshot.schema.clone(), model: snapshot.model.to_value(), structure: snapshot.structure.clone(), zones: snapshot.zones.clone(), referenced_model: snapshot.referenced_model.clone(), weather_link: snapshot.weather_link.clone() }
    }

    pub(crate) fn into_snapshot(self) -> Result<EnergyModelSnapshot, String> {
        let model = crate::model::Model::from_value(self.model).map_err(|error| error.to_string())?;
        Ok(EnergyModelSnapshot { schema: self.schema, model, structure: self.structure, zones: self.zones, referenced_model: self.referenced_model, weather_link: self.weather_link })
    }
}

impl store::ArtifactPack for EnergyModelSnapshot {
    fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&EnergyModelPackRecord::__dsl_spec(), &EnergyModelPackRecord::from_snapshot(self).__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &EnergyModelPackRecord::__dsl_spec(), options)?;
        EnergyModelPackRecord::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot().map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error)))
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(EnergyModelPackRecord::__dsl_spec())
    }
}
}
pub use snapshot_codec::*;
