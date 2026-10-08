//! 👥️ Drawing presence — shareable live ephemeral state + mutations.

use protocol::Mutation;
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of drawing camera and rename input state — layer
/// selection/hover moved to the framework's typed `PresencePeer.interaction` broadcast (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM); this facet keeps only genuinely
/// drawing-specific presence.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "drawing.presence")]
#[dsl(layout = "lines")]
pub struct DrawingPresence {
    pub engagement_input: String,
    #[dsl(block)]
    pub camera: store::Viewport2d,
}

impl Default for DrawingPresence {
    fn default() -> Self {
        Self { engagement_input: String::new(), camera: store::Viewport2d { x: 512.0, y: 512.0, zoom: 0.75 } }
    }
}


/// 🔺️ Sparse delta of the shareable presence: only the fields a mutation actually changes.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct DrawingPresenceDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub engagement_input: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera: Option<store::Viewport2d>,
}

impl protocol::MutationDiff<DrawingPresence> for DrawingPresenceDiff {
    fn apply(&self, base: &DrawingPresence, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<DrawingPresence> {
        Ok(DrawingPresence { engagement_input: self.engagement_input.clone().unwrap_or_else(|| base.engagement_input.clone()), camera: self.camera.clone().unwrap_or_else(|| base.camera.clone()) })
    }
    fn absorb(&mut self, other: Self) {
        if other.engagement_input.is_some() {
            self.engagement_input = other.engagement_input;
        }
        if other.camera.is_some() {
            self.camera = other.camera;
        }
    }
}

impl protocol::DiffAlgebra<DrawingPresence> for DrawingPresenceDiff {
    fn inverse(&self, base: &DrawingPresence) -> Self {
        Self { engagement_input: self.engagement_input.as_ref().map(|_| base.engagement_input.clone()), camera: self.camera.as_ref().map(|_| base.camera.clone()) }
    }
    fn between(base: &DrawingPresence, other: &DrawingPresence) -> Self {
        Self { engagement_input: (base.engagement_input != other.engagement_input).then(|| other.engagement_input.clone()), camera: (base.camera != other.camera).then(|| other.camera.clone()) }
    }
    fn is_empty(&self) -> bool {
        self.engagement_input.is_none() && self.camera.is_none()
    }
}

impl semio_framework_value::retirement::RetireOwned for DrawingPresence {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        let store::Viewport2d { x, y, zoom } = self.camera;
        semio_framework_value::retirement::sequence(vec![
            semio_framework_value::retirement::RetireOwned::retirement(self.engagement_input),
            semio_framework_value::retirement::leaf(x),
            semio_framework_value::retirement::leaf(y),
            semio_framework_value::retirement::leaf(zoom),
        ])
    }
}

impl store::ArtifactDsl for DrawingPresence {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for DrawingPresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum)]
#[value(rename_all = "camelCase")]
pub enum DrawingPresenceMutation {
    #[dsl(key = "set")]
    Set {
        engagement_input: String,
        #[dsl(block)]
        camera: store::Viewport2d,
    },
}

impl semio_framework_value::retirement::RetireOwned for DrawingPresenceMutation {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        match self {
            Self::Set { engagement_input, camera: store::Viewport2d { x, y, zoom } } => semio_framework_value::retirement::sequence(vec![
                semio_framework_value::retirement::leaf(0u8),
                semio_framework_value::retirement::RetireOwned::retirement(engagement_input),
                semio_framework_value::retirement::leaf(x),
                semio_framework_value::retirement::leaf(y),
                semio_framework_value::retirement::leaf(zoom),
            ]),
        }
    }
}

impl Mutation<DrawingPresence> for DrawingPresenceMutation {
    type Diff = DrawingPresenceDiff;

    /// 🧷️ Hand-written: `dsl::DslOps` supplies `DslVariants` only, not this trait's leaf metadata.
    /// ⚠️ PROVISIONAL: the `owner` leaf directory does not exist on disk — a placeholder that
    /// satisfies `protocol::Mutation`, not a real registration.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence",
        semantic_kind: "set-presence",
        display_name: "Set Presence",
        emoji: "👥️",
        aggregate_variant: "Set",
        payload_schema: "🧬️schema/🔣️.json",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Self::Set { .. } => &Self::DESCRIPTORS[0],
        }
    }

    fn diff(&self, base: &DrawingPresence) -> protocol::MutationOutcome<DrawingPresenceDiff> {
        match self {
            Self::Set { engagement_input, camera } => protocol::MutationOutcome::new(DrawingPresenceDiff { engagement_input: (&base.engagement_input != engagement_input).then(|| engagement_input.clone()), camera: (&base.camera != camera).then(|| camera.clone()) }),
        }
    }

    fn inverse(&self, base: &DrawingPresence) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![Self::Set { engagement_input: base.engagement_input.clone(), camera: base.camera.clone() }])
    }
}

impl protocol::OpText for DrawingPresenceMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                let record = semio_framework_dsl_record::parse(body, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        let body = semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline);
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl protocol::OpBinary for DrawingPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
