//! 👥️ Puzzle2dPresence — shareable live ephemeral state + mutations.

use protocol::Mutation;
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of puzzle camera state.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "puzzle2d.presence")]
#[dsl(layout = "lines")]
pub struct Puzzle2dPresence {
    pub camera_x: f64,
    pub camera_y: f64,
    pub camera_zoom: f64,
}

impl Default for Puzzle2dPresence {
    fn default() -> Self {
        Self { camera_x: 0.0, camera_y: 0.0, camera_zoom: 1.0 }
    }
}


/// 🔺️ Sparse typed delta of the shareable live presence of a Puzzle 2D board: names only the fields a mutation changes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dPresenceDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera_x: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera_y: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera_zoom: Option<f64>,
}

impl Puzzle2dPresenceDiff {
    /// 🎯️ Every field set to `state`'s value.
    pub fn of(state: &Puzzle2dPresence) -> Self {
        Self { camera_x: Some(state.camera_x), camera_y: Some(state.camera_y), camera_zoom: Some(state.camera_zoom) }
    }
    /// ✂️ The named fields that differ from `base`.
    pub fn changed(&self, base: &Puzzle2dPresence) -> Self {
        Self { camera_x: self.camera_x.as_ref().filter(|value| **value != base.camera_x).cloned(), camera_y: self.camera_y.as_ref().filter(|value| **value != base.camera_y).cloned(), camera_zoom: self.camera_zoom.as_ref().filter(|value| **value != base.camera_zoom).cloned() }
    }
    /// ↩️ The named fields at the values `base` holds.
    pub fn restoring(&self, base: &Puzzle2dPresence) -> Self {
        Self { camera_x: self.camera_x.as_ref().map(|_| base.camera_x), camera_y: self.camera_y.as_ref().map(|_| base.camera_y), camera_zoom: self.camera_zoom.as_ref().map(|_| base.camera_zoom) }
    }
}

impl protocol::MutationDiff<Puzzle2dPresence> for Puzzle2dPresenceDiff {
    fn apply(&self, base: &Puzzle2dPresence, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle2dPresence> {
        let mut next = base.clone();
        if let Some(value) = &self.camera_x {
            next.camera_x = *value;
        }
        if let Some(value) = &self.camera_y {
            next.camera_y = *value;
        }
        if let Some(value) = &self.camera_zoom {
            next.camera_zoom = *value;
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.camera_x.is_some() {
            self.camera_x = other.camera_x;
        }
        if other.camera_y.is_some() {
            self.camera_y = other.camera_y;
        }
        if other.camera_zoom.is_some() {
            self.camera_zoom = other.camera_zoom;
        }
    }
}

impl protocol::DiffAlgebra<Puzzle2dPresence> for Puzzle2dPresenceDiff {
    fn inverse(&self, base: &Puzzle2dPresence) -> Self {
        self.restoring(base)
    }
    fn between(base: &Puzzle2dPresence, other: &Puzzle2dPresence) -> Self {
        Self { camera_x: (base.camera_x != other.camera_x).then(|| other.camera_x), camera_y: (base.camera_y != other.camera_y).then(|| other.camera_y), camera_zoom: (base.camera_zoom != other.camera_zoom).then(|| other.camera_zoom) }
    }
    fn is_empty(&self) -> bool {
        self.camera_x.is_none() && self.camera_y.is_none() && self.camera_zoom.is_none()
    }
}

impl store::ArtifactDsl for Puzzle2dPresence {
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

impl ArtifactPack for Puzzle2dPresence {
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
#[value(rename_all = "camelCase")]
pub enum Puzzle2dPresenceMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        presence: Puzzle2dPresence,
    },
}

impl Mutation<Puzzle2dPresence> for Puzzle2dPresenceMutation {
    type Diff = Puzzle2dPresenceDiff;

    /// 🧷️ Hand-written: this enum's `#[derive(dsl::DslOps)]` supplies `DslVariants` only, a
    /// distinct derive from the one that would supply this trait's items. ⚠️ PROVISIONAL: the
    /// `owner` leaf directory below does not exist on disk yet — a metadata placeholder to
    /// satisfy `protocol::Mutation`, not a real registration.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/📄snapshot",
        semantic_kind: "snapshot",
        display_name: "Snapshot",
        emoji: "📄",
        aggregate_variant: "Snapshot",
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
            Self::Snapshot { .. } => &Self::DESCRIPTORS[0],
        }
    }

    fn diff(&self, base: &Puzzle2dPresence) -> protocol::MutationOutcome<Puzzle2dPresenceDiff> {
        let Self::Snapshot { presence } = self;
        let diff = Puzzle2dPresenceDiff::of(presence).changed(base);
        if protocol::DiffAlgebra::<Puzzle2dPresence>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The presence already holds this state.");
        }
        protocol::MutationOutcome::new(diff)
    }

    fn inverse(&self, base: &Puzzle2dPresence) -> Self {
        self.restoring(base)
    }
    fn between(base: &Puzzle2dPresence, other: &Puzzle2dPresence) -> Self {
        Self { camera_x: (base.camera_x != other.camera_x).then(|| other.camera_x), camera_y: (base.camera_y != other.camera_y).then(|| other.camera_y), camera_zoom: (base.camera_zoom != other.camera_zoom).then(|| other.camera_zoom) }
    }
    fn is_empty(&self) -> bool {
        self.camera_x.is_none() && self.camera_y.is_none() && self.camera_zoom.is_none()
    }
}

impl store::ArtifactDsl for Puzzle2dPresence {
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

impl ArtifactPack for Puzzle2dPresence {
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
#[value(rename_all = "camelCase")]
pub enum Puzzle2dPresenceMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        presence: Puzzle2dPresence,
    },
}

impl Mutation<Puzzle2dPresence> for Puzzle2dPresenceMutation {
    type Diff = Puzzle2dPresenceDiff;

    /// 🧷️ Hand-written: this enum's `#[derive(dsl::DslOps)]` supplies `DslVariants` only, a
    /// distinct derive from the one that would supply this trait's items. ⚠️ PROVISIONAL: the
    /// `owner` leaf directory below does not exist on disk yet — a metadata placeholder to
    /// satisfy `protocol::Mutation`, not a real registration.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/📄snapshot",
        semantic_kind: "snapshot",
        display_name: "Snapshot",
        emoji: "📄",
        aggregate_variant: "Snapshot",
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
            Self::Snapshot { .. } => &Self::DESCRIPTORS[0],
        }
    }

    fn diff(&self, _base: &Puzzle2dPresence) -> protocol::MutationOutcome<Puzzle2dPresence> {
        protocol::MutationOutcome::new(match self {
            Self::Snapshot { presence } => presence.clone(),
        })
    }

    fn inverse(&self, base: &Puzzle2dPresence) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self::Snapshot { presence: base.clone() }]
    
    })())
}
}

impl protocol::OpText for Puzzle2dPresenceMutation {
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

impl protocol::OpBinary for Puzzle2dPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation
