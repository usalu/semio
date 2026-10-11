//! 👥️ Puzzle3dPresence — shareable live ephemeral state + mutations.

use protocol::Mutation;
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of puzzle camera pose and selected tool.
/// 🕹️ Selection and hover are deliberately ABSENT — they are framework-owned and already broadcast
/// generically on `protocol::PresencePeer.interaction` (wire bit 7, assembled by `VcsArtifactApp`'s
/// heartbeat from `InteractionState` via `protocol::assemble_presence_interaction`, read back through
/// `InteractionView::peers_selecting`/`peers_hovering`). Adding app-owned copies here would be a
/// second, divergent authority over the same state — see
/// `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`.
#[derive(semio_framework_value::RetireOwned, semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "puzzle3d.presence")]
#[dsl(layout = "lines")]
pub struct Puzzle3dPresence {
    pub camera_position: [f64; 3],
    pub camera_target: [f64; 3],
    pub camera_zoom: f64,
    pub active_tool_id: Option<String>,
}

impl Default for Puzzle3dPresence {
    fn default() -> Self {
        Self { camera_position: [0.0, 0.0, 0.0], camera_target: [0.0, 0.0, 0.0], camera_zoom: 1.0, active_tool_id: None }
    }
}

/// 🕳️ Tri-state decode of every `Option<Option<T>>` diff slot: a missing key is the unchanged slot (`None`) and a PRESENT
/// `null` is the clear `Some(None)`, never the unchanged slot the blanket `Option<T>` decode would fold it into.
fn deserialize_double_option<T: semio_framework_value::FromValue>(value: semio_framework_value::DslValue) -> Result<Option<Option<T>>, semio_framework_value::ValueError> {
    <Option<T> as semio_framework_value::FromValue>::from_value(value).map(Some)
}

/// 🔺️ Sparse typed delta of the shareable live presence of a Puzzle 3D scene: names only the fields a mutation changes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dPresenceDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera_position: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera_target: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera_zoom: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub active_tool_id: Option<Option<String>>,
}

impl Puzzle3dPresenceDiff {
    /// 🎯️ Every field set to `state`'s value.
    pub fn of(state: &Puzzle3dPresence) -> Self {
        Self { camera_position: Some(state.camera_position), camera_target: Some(state.camera_target), camera_zoom: Some(state.camera_zoom), active_tool_id: Some(state.active_tool_id.clone()) }
    }
    /// ✂️ The named fields that differ from `base`.
    pub fn changed(&self, base: &Puzzle3dPresence) -> Self {
        Self { camera_position: self.camera_position.as_ref().filter(|value| **value != base.camera_position).cloned(), camera_target: self.camera_target.as_ref().filter(|value| **value != base.camera_target).cloned(), camera_zoom: self.camera_zoom.as_ref().filter(|value| **value != base.camera_zoom).cloned(), active_tool_id: self.active_tool_id.as_ref().filter(|value| **value != base.active_tool_id).cloned() }
    }
    /// ↩️ The named fields at the values `base` holds.
    pub fn restoring(&self, base: &Puzzle3dPresence) -> Self {
        Self { camera_position: self.camera_position.as_ref().map(|_| base.camera_position), camera_target: self.camera_target.as_ref().map(|_| base.camera_target), camera_zoom: self.camera_zoom.as_ref().map(|_| base.camera_zoom), active_tool_id: self.active_tool_id.as_ref().map(|_| base.active_tool_id.clone()) }
    }
}

impl protocol::MutationDiff<Puzzle3dPresence> for Puzzle3dPresenceDiff {
    fn apply(&self, base: &Puzzle3dPresence, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle3dPresence> {
        let mut next = base.clone();
        if let Some(value) = &self.camera_position {
            next.camera_position = *value;
        }
        if let Some(value) = &self.camera_target {
            next.camera_target = *value;
        }
        if let Some(value) = &self.camera_zoom {
            next.camera_zoom = *value;
        }
        if let Some(value) = &self.active_tool_id {
            next.active_tool_id = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.camera_position.is_some() {
            self.camera_position = other.camera_position;
        }
        if other.camera_target.is_some() {
            self.camera_target = other.camera_target;
        }
        if other.camera_zoom.is_some() {
            self.camera_zoom = other.camera_zoom;
        }
        if other.active_tool_id.is_some() {
            self.active_tool_id = other.active_tool_id;
        }
    }
}

impl protocol::DiffAlgebra<Puzzle3dPresence> for Puzzle3dPresenceDiff {
    fn inverse(&self, base: &Puzzle3dPresence) -> Self {
        self.restoring(base)
    }
    fn is_empty(&self) -> bool {
        self.camera_position.is_none() && self.camera_target.is_none() && self.camera_zoom.is_none() && self.active_tool_id.is_none()
    }
}

/// 📸️ Native snapshot codec: the canonical JSON of the value projection, no hand-written frame.
impl store::ArtifactPresenceSnapshot for Puzzle3dPresence {}

impl store::ArtifactDsl for Puzzle3dPresence {
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

impl ArtifactPack for Puzzle3dPresence {
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
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
#[value(rename_all = "camelCase")]
pub enum Puzzle3dPresenceMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        presence: Puzzle3dPresence,
    },
}

impl Mutation<Puzzle3dPresence> for Puzzle3dPresenceMutation {
    type Diff = Puzzle3dPresenceDiff;

    /// 🧷️ Hand-written (not `#[derive(dsl::Mutations)]`: this enum carries `dsl::DslOps`, not
    /// `dsl::Mutations` — the derive that would have generated this). One leaf, the whole-
    /// presence replace. ⚠️ PROVISIONAL: no leaf directory is authored on disk yet; `owner`
    /// below names a path that does not exist, matching stdio's own `⚠️ PROVISIONAL` precedent.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🟤️set-snapshot",
        semantic_kind: "set-snapshot",
        display_name: "Set Snapshot",
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

    fn diff(&self, base: &Puzzle3dPresence) -> protocol::MutationOutcome<Puzzle3dPresenceDiff> {
        let Self::Snapshot { presence } = self;
        let diff = Puzzle3dPresenceDiff::of(presence).changed(base);
        if protocol::DiffAlgebra::<Puzzle3dPresence>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The presence already holds this state.");
        }
        protocol::MutationOutcome::new(diff)
    }

    fn inverse(&self, base: &Puzzle3dPresence) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self::Snapshot { presence: base.clone() }]
    
    })())
}
}

impl protocol::OpText for Puzzle3dPresenceMutation {
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

impl protocol::OpBinary for Puzzle3dPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation

//#region 🧹️Retirement
/// 📏️ The one heap owner a `Puzzle3dPresence` root can hold: `active_tool_id`'s UTF-8 buffer. Every
/// other field is a plain `f64` array, so a root whose tool id is absent owns nothing beyond its own
/// inline bytes — which is exactly the terminal-empty predicate the close lane fences on.
pub fn puzzle3d_presence_is_terminal_empty(presence: &Puzzle3dPresence) -> bool {
    presence.active_tool_id.is_none()
}

/// 👥️ Exact local and peer root ownership for puzzle3d presence: the original typed fields and their retained physical backing retire through the shared admitted owner.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct Puzzle3dPresenceRetirementFactory;

impl store::SnapshotRetirementFactory<Puzzle3dPresence> for Puzzle3dPresenceRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &std::sync::Arc<Puzzle3dPresence>) -> usize {
        semio_framework_value::retirement::shared::shared_retirement_birth_bytes::<Puzzle3dPresence>()
    }

    fn retire(&self, root: std::sync::Arc<Puzzle3dPresence>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<Puzzle3dPresence>)> {
        semio_framework_value::retirement::shared::admit_shared_retirement(root, grant, true)
    }
}

/// 🧹️ The close-lane disposer paired with [`Puzzle3dPresenceRetirementFactory`].
pub fn puzzle3d_presence_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Puzzle3dPresence, Puzzle3dPresenceMutation>>> {
    Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Puzzle3dPresence::default()), puzzle3d_presence_is_terminal_empty).expect("the default puzzle3d presence root holds no active tool identifier"))
}
//#endregion 🧹️Retirement
