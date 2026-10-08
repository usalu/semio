//! 👥️ Puzzle5dPresence — shareable live ephemeral state + mutations.

use protocol::Mutation;
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of puzzle camera state.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "puzzle5d.presence")]
#[dsl(layout = "lines")]
pub struct Puzzle5dPresence {
    pub camera2d_x: f64,
    pub camera2d_y: f64,
    pub camera2d_zoom: f64,
    pub camera3d_position: [f64; 3],
    pub camera3d_target: [f64; 3],
    pub camera3d_zoom: f64,
}

impl Default for Puzzle5dPresence {
    fn default() -> Self {
        Self { camera2d_x: 0.0, camera2d_y: 0.0, camera2d_zoom: 1.0, camera3d_position: [8.0, -8.0, 8.0], camera3d_target: [0.0, 0.0, 0.0], camera3d_zoom: 1.0}
    }
}


/// 🔺️ Sparse typed delta of the shareable live presence of a Puzzle 5D scene: names only the fields a mutation changes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dPresenceDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera2d_x: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera2d_y: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera2d_zoom: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera3d_position: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera3d_target: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera3d_zoom: Option<f64>,
}

impl Puzzle5dPresenceDiff {
    /// 🎯️ Every field set to `state`'s value.
    pub fn of(state: &Puzzle5dPresence) -> Self {
        Self { camera2d_x: Some(state.camera2d_x), camera2d_y: Some(state.camera2d_y), camera2d_zoom: Some(state.camera2d_zoom), camera3d_position: Some(state.camera3d_position), camera3d_target: Some(state.camera3d_target), camera3d_zoom: Some(state.camera3d_zoom) }
    }
    /// ✂️ The named fields that differ from `base`.
    pub fn changed(&self, base: &Puzzle5dPresence) -> Self {
        Self { camera2d_x: self.camera2d_x.as_ref().filter(|value| **value != base.camera2d_x).cloned(), camera2d_y: self.camera2d_y.as_ref().filter(|value| **value != base.camera2d_y).cloned(), camera2d_zoom: self.camera2d_zoom.as_ref().filter(|value| **value != base.camera2d_zoom).cloned(), camera3d_position: self.camera3d_position.as_ref().filter(|value| **value != base.camera3d_position).cloned(), camera3d_target: self.camera3d_target.as_ref().filter(|value| **value != base.camera3d_target).cloned(), camera3d_zoom: self.camera3d_zoom.as_ref().filter(|value| **value != base.camera3d_zoom).cloned() }
    }
    /// ↩️ The named fields at the values `base` holds.
    pub fn restoring(&self, base: &Puzzle5dPresence) -> Self {
        Self { camera2d_x: self.camera2d_x.as_ref().map(|_| base.camera2d_x), camera2d_y: self.camera2d_y.as_ref().map(|_| base.camera2d_y), camera2d_zoom: self.camera2d_zoom.as_ref().map(|_| base.camera2d_zoom), camera3d_position: self.camera3d_position.as_ref().map(|_| base.camera3d_position), camera3d_target: self.camera3d_target.as_ref().map(|_| base.camera3d_target), camera3d_zoom: self.camera3d_zoom.as_ref().map(|_| base.camera3d_zoom) }
    }
}

impl protocol::MutationDiff<Puzzle5dPresence> for Puzzle5dPresenceDiff {
    fn apply(&self, base: &Puzzle5dPresence, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle5dPresence> {
        let mut next = base.clone();
        if let Some(value) = &self.camera2d_x {
            next.camera2d_x = *value;
        }
        if let Some(value) = &self.camera2d_y {
            next.camera2d_y = *value;
        }
        if let Some(value) = &self.camera2d_zoom {
            next.camera2d_zoom = *value;
        }
        if let Some(value) = &self.camera3d_position {
            next.camera3d_position = *value;
        }
        if let Some(value) = &self.camera3d_target {
            next.camera3d_target = *value;
        }
        if let Some(value) = &self.camera3d_zoom {
            next.camera3d_zoom = *value;
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.camera2d_x.is_some() {
            self.camera2d_x = other.camera2d_x;
        }
        if other.camera2d_y.is_some() {
            self.camera2d_y = other.camera2d_y;
        }
        if other.camera2d_zoom.is_some() {
            self.camera2d_zoom = other.camera2d_zoom;
        }
        if other.camera3d_position.is_some() {
            self.camera3d_position = other.camera3d_position;
        }
        if other.camera3d_target.is_some() {
            self.camera3d_target = other.camera3d_target;
        }
        if other.camera3d_zoom.is_some() {
            self.camera3d_zoom = other.camera3d_zoom;
        }
    }
}

impl protocol::DiffAlgebra<Puzzle5dPresence> for Puzzle5dPresenceDiff {
    fn inverse(&self, base: &Puzzle5dPresence) -> Self {
        self.restoring(base)
    }
    fn between(base: &Puzzle5dPresence, other: &Puzzle5dPresence) -> Self {
        Self { camera2d_x: (base.camera2d_x != other.camera2d_x).then(|| other.camera2d_x), camera2d_y: (base.camera2d_y != other.camera2d_y).then(|| other.camera2d_y), camera2d_zoom: (base.camera2d_zoom != other.camera2d_zoom).then(|| other.camera2d_zoom), camera3d_position: (base.camera3d_position != other.camera3d_position).then(|| other.camera3d_position), camera3d_target: (base.camera3d_target != other.camera3d_target).then(|| other.camera3d_target), camera3d_zoom: (base.camera3d_zoom != other.camera3d_zoom).then(|| other.camera3d_zoom) }
    }
    fn is_empty(&self) -> bool {
        self.camera2d_x.is_none() && self.camera2d_y.is_none() && self.camera2d_zoom.is_none() && self.camera3d_position.is_none() && self.camera3d_target.is_none() && self.camera3d_zoom.is_none()
    }
}

impl store::ArtifactDsl for Puzzle5dPresence {
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

impl ArtifactPack for Puzzle5dPresence {
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
pub enum Puzzle5dPresenceMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        presence: Puzzle5dPresence,
    },
}

impl Mutation<Puzzle5dPresence> for Puzzle5dPresenceMutation {
    type Diff = Puzzle5dPresenceDiff;

    /// 🧷️ Hand-written (not `#[derive(dsl::Mutations)]`: this enum derives `dsl::DslOps`, a
    /// different derive that supplies `DslVariants` for the text/binary op codecs below, not
    /// `protocol::Mutation` — see the file's existing hand-written `diff`/`inverse` immediately
    /// here). ⚠️ PROVISIONAL: the `owner` path names a directory that does not exist on disk — this
    /// enum has no `🧬️mutations/<slug>` leaf triad of its own, so the entry is a metadata
    /// placeholder to satisfy `protocol::Mutation`, matching stdio's `🔊️wav`/`🏗️ifc` precedent.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/📄snapshot", semantic_kind: "snapshot", display_name: "Snapshot", emoji: "📄", aggregate_variant: "Snapshot", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Self::Snapshot { .. } => &Self::DESCRIPTORS[0],
        }
    }

    fn diff(&self, base: &Puzzle5dPresence) -> protocol::MutationOutcome<Puzzle5dPresenceDiff> {
        let Self::Snapshot { presence } = self;
        let diff = Puzzle5dPresenceDiff::of(presence).changed(base);
        if protocol::DiffAlgebra::<Puzzle5dPresence>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The presence already holds this state.");
        }
        protocol::MutationOutcome::new(diff)
    }

    fn inverse(&self, base: &Puzzle5dPresence) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self::Snapshot { presence: base.clone() }]
    
    })())
}
}

impl protocol::OpText for Puzzle5dPresenceMutation {
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

impl protocol::OpBinary for Puzzle5dPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation
