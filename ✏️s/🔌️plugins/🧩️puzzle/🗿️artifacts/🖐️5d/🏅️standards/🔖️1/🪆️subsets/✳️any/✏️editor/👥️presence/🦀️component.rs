//! 👥️ Puzzle5dPresence — shareable live ephemeral state + mutations.

use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of puzzle view state (selection, hover, camera, active utility).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "puzzle5d.presence")]
#[dsl(layout = "lines")]
pub struct Puzzle5dPresence {
    pub camera2d_x: f64,
    pub camera2d_y: f64,
    pub camera2d_zoom: f64,
    pub camera3d_position: [f64; 3],
    pub camera3d_target: [f64; 3],
    pub camera3d_zoom: f64,
    pub active_utility_id: String,
}

impl Default for Puzzle5dPresence {
    fn default() -> Self {
        Self {
            camera2d_x: 0.0,
            camera2d_y: 0.0,
            camera2d_zoom: 1.0,
            camera3d_position: [8.0, -8.0, 8.0],
            camera3d_target: [0.0, 0.0, 0.0],
            camera3d_zoom: 1.0,
            active_utility_id: String::new(),
        }
    }
}

impl protocol::MutationDiff<Puzzle5dPresence> for Puzzle5dPresence {
    async fn apply(&self, _base: &Puzzle5dPresence) -> protocol::MutationApplyResult<Puzzle5dPresence> {
        Ok({
            self.clone()
        })
    }
    async fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

impl store::ArtifactDsl for Puzzle5dPresence {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    async fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    async fn print_dsl(&self) -> String {
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

impl ArtifactPack for Puzzle5dPresence {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
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
    async fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(rename_all = "camelCase")]
pub enum Puzzle5dPresenceMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        presence: Puzzle5dPresence,
    },
}

impl Mutation<Puzzle5dPresence> for Puzzle5dPresenceMutation {
    type Diff = Puzzle5dPresence;

    async fn diff(&self, _base: &Puzzle5dPresence) -> protocol::MutationOutcome<Puzzle5dPresence> {
        protocol::MutationOutcome::new(match self {
            Self::Snapshot { presence } => presence.clone(),
        })
    }

    async fn inverse(&self, base: &Puzzle5dPresence) -> Vec<Self> {
        vec![Self::Snapshot { presence: base.clone() }]
    }
}

impl protocol::OpText for Puzzle5dPresenceMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let body = if line.len() > keyword.len() {
                    line[keyword.len()..].trim_start()
                } else {
                    ""
                };
                let record = dsl::parse(
                    body,
                    &spec_fn(),
                    &dsl::ParseOptions {
                        limits: dsl::Limits::default(),
                        mode: dsl::SourceMode::Inline,
                    },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants
            .iter()
            .find(|(k, _)| k == &keyword)
            .map(|(_, s)| *s)
            .expect("variant spec must exist for its own keyword");
        let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl protocol::OpBinary for Puzzle5dPresenceMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation
