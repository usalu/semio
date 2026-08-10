//! 🧬️ Imperative snapshot schema — persistent fields only.

use crate::artifacts::imperative::{Dictionary, Path};
use neural_engine::Value;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Snapshot
/// 📸️ Persisted imperative document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.imperative.imperative")]
pub struct ImperativeSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub path: Path,
    #[state(persistent)]
    #[serde(default)]
    pub seed: BTreeMap<String, Value>,
}

impl Default for ImperativeSnapshot {
    fn default() -> Self {
        Self {
            schema: "imperative.document".into(),
            path: Path::new(),
            seed: BTreeMap::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️DslMirror
use crate::artifacts::imperative::dsl::{dictionary_to_option_dsl_map, option_dsl_map_to_dictionary, step_node_dsl_to_step, step_to_step_node_dsl, StepNodeDsl, ValueDsl};

/// 📄️ Local mirror of [`ImperativeSnapshot`] for DSL/pack codecs.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(extension = "imperative")]
#[dsl(id = "imperative.imperative")]
#[dsl(layout = "lines")]
pub struct ImperativeSnapshotDsl {
    pub schema: String,
    pub seed: Option<BTreeMap<String, ValueDsl>>,
    #[dsl(statements, block)]
    pub steps: Vec<StepNodeDsl>,
}

pub fn snapshot_to_dsl(snapshot: &ImperativeSnapshot) -> ImperativeSnapshotDsl {
    let seed_dict = seed_map_as_dictionary(&snapshot.seed);
    ImperativeSnapshotDsl {
        schema: snapshot.schema.clone(),
        seed: dictionary_to_option_dsl_map(&seed_dict),
        steps: snapshot.path.steps.iter().map(step_to_step_node_dsl).collect(),
    }
}

pub fn snapshot_from_dsl(mirror: ImperativeSnapshotDsl) -> ImperativeSnapshot {
    ImperativeSnapshot {
        schema: mirror.schema,
        path: Path { steps: mirror.steps.into_iter().map(step_node_dsl_to_step).collect() },
        seed: dictionary_to_seed_map(&option_dsl_map_to_dictionary(mirror.seed)),
    }
}

fn seed_map_as_dictionary(seed: &BTreeMap<String, Value>) -> Dictionary {
    serde_json::from_value(serde_json::to_value(seed).expect("seed serializes")).expect("seed is a dictionary")
}

fn dictionary_to_seed_map(dict: &Dictionary) -> BTreeMap<String, Value> {
    serde_json::from_value(serde_json::to_value(dict).expect("dictionary serializes")).expect("dictionary is a seed map")
}
//#endregion 🔖️DslMirror

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for ImperativeSnapshotDsl {
    const EXTENSION: &'static str = "imperative";
    fn envelope_id() -> &'static str {
        "imperative.imperative"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for ImperativeSnapshotDsl {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl store::ArtifactDsl for ImperativeSnapshot {
    const EXTENSION: &'static str = "imperative";
    fn envelope_id() -> &'static str {
        "imperative.imperative"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        Ok(snapshot_from_dsl(<ImperativeSnapshotDsl as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn print_dsl(&self) -> String {
        <ImperativeSnapshotDsl as store::ArtifactDsl>::print_dsl(&snapshot_to_dsl(self))
    }
}

impl store::ArtifactPack for ImperativeSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <ImperativeSnapshotDsl as store::ArtifactPack>::encode_pack_with(&snapshot_to_dsl(self), options)
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        Ok(snapshot_from_dsl(<ImperativeSnapshotDsl as store::ArtifactPack>::decode_pack_with(bytes, options)?))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
