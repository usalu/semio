//! 👥️ Space presence — shareable live ephemeral state + mutations.
//!
//! Selection/hover broadcast automatically via the framework's typed `PresenceInteraction` (ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — see `create_space_app`'s `.interaction(...)`
//! declaration for the `graph` domain.

use crate::engine::space::config::SpaceWindowCamera;
use protocol::Mutation;
use std::collections::BTreeMap;
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of studio view state (node selection, hover, camera, active/focused node).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "space.presence")]
#[dsl(layout = "lines")]
#[derive(Default)]
pub struct SpacePresence {
    pub camera: BTreeMap<String, SpaceWindowCamera>,
    pub active_node_id: Option<String>,
    pub focused_node_id: Option<String>,
    pub collapsed_node_ids: Vec<String>,
    pub preview_off_node_ids: Vec<String>,
}


store::sparse_record_diff! {
    record: SpacePresence,
    diff: SpacePresenceDiff,
    fields: {
        active_node_id: Option<String>,
        focused_node_id: Option<String>,
        collapsed_node_ids: Vec<String>,
        preview_off_node_ids: Vec<String>,
    },
    keyed: { camera: SpaceWindowCamera },
}

impl store::ArtifactDsl for SpacePresence {
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

impl ArtifactPack for SpacePresence {
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
/// 👥️ `SpacePresence`'s operation enum — one variant per settled interaction, each setting only the slots (or keyed camera rows) of the fields it owns, with the same
/// variant carrying the base values as its inverse. There is no whole-presence variant.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
pub enum SpacePresenceMutation {
    #[dsl(key = "active-node")]
    SetActiveNode { node_id: Option<String> },
    #[dsl(key = "focused-node")]
    SetFocusedNode { node_id: Option<String> },
    #[dsl(key = "collapsed")]
    SetCollapsed { node_ids: Vec<String> },
    #[dsl(key = "preview-off")]
    SetPreviewOff { node_ids: Vec<String> },
    #[dsl(key = "camera")]
    SetCamera {
        window_id: String,
        #[dsl(block)]
        camera: SpaceWindowCamera,
    },
    #[dsl(key = "remove-camera")]
    RemoveCamera { window_id: String },
}

/// 🧷️ The leaf descriptor of one presence operation.
const fn descriptor(owner: &'static str, kind: &'static str, display_name: &'static str, variant: &'static str) -> protocol::MutationLeafDescriptor {
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner,
        semantic_kind: kind,
        display_name,
        emoji: "👥️",
        aggregate_variant: variant,
        payload_schema: "🧬️schema/🔣️.json",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }
}

impl Mutation<SpacePresence> for SpacePresenceMutation {
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        descriptor("🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/👥️presence/👥️set-active-node", "set-active-node", "Set Active Node", "SetActiveNode"),
        descriptor("🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/👥️presence/👥️set-focused-node", "set-focused-node", "Set Focused Node", "SetFocusedNode"),
        descriptor("🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/👥️presence/👥️set-collapsed", "set-collapsed", "Set Collapsed", "SetCollapsed"),
        descriptor("🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/👥️presence/👥️set-preview-off", "set-preview-off", "Set Preview Off", "SetPreviewOff"),
        descriptor("🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/👥️presence/👥️set-camera", "set-camera", "Set Camera", "SetCamera"),
        descriptor("🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/👥️presence/👥️remove-camera", "remove-camera", "Remove Camera", "RemoveCamera"),
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Self::SetActiveNode { .. } => &Self::DESCRIPTORS[0],
            Self::SetFocusedNode { .. } => &Self::DESCRIPTORS[1],
            Self::SetCollapsed { .. } => &Self::DESCRIPTORS[2],
            Self::SetPreviewOff { .. } => &Self::DESCRIPTORS[3],
            Self::SetCamera { .. } => &Self::DESCRIPTORS[4],
            Self::RemoveCamera { .. } => &Self::DESCRIPTORS[5],
        }
    }

    type Diff = SpacePresenceDiff;

    fn diff(&self, base: &SpacePresence) -> protocol::MutationOutcome<SpacePresenceDiff> {
        protocol::MutationOutcome::new(match self {
            Self::SetActiveNode { node_id } => SpacePresenceDiff { active_node_id: (base.active_node_id != *node_id).then(|| node_id.clone()), ..Default::default() },
            Self::SetFocusedNode { node_id } => SpacePresenceDiff { focused_node_id: (base.focused_node_id != *node_id).then(|| node_id.clone()), ..Default::default() },
            Self::SetCollapsed { node_ids } => SpacePresenceDiff { collapsed_node_ids: (base.collapsed_node_ids != *node_ids).then(|| node_ids.clone()), ..Default::default() },
            Self::SetPreviewOff { node_ids } => SpacePresenceDiff { preview_off_node_ids: (base.preview_off_node_ids != *node_ids).then(|| node_ids.clone()), ..Default::default() },
            Self::SetCamera { window_id, camera } => match base.camera.get(window_id) {
                Some(prior) if prior == camera => SpacePresenceDiff::default(),
                Some(_) => SpacePresenceDiff { camera: [(window_id.clone(), protocol::KeyedRow::Replace(*camera))].into(), ..Default::default() },
                None => SpacePresenceDiff { camera: [(window_id.clone(), protocol::KeyedRow::Insert(*camera))].into(), ..Default::default() },
            },
            Self::RemoveCamera { window_id } => {
                if !base.camera.contains_key(window_id) {
                    return protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMissing, "the window has no camera row", ["camera", window_id.as_str()]);
                }
                SpacePresenceDiff { camera: [(window_id.clone(), protocol::KeyedRow::Remove)].into(), ..Default::default() }
            }
        })
    }

    fn inverse(&self, base: &SpacePresence) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![match self {
            Self::SetActiveNode { .. } => Self::SetActiveNode { node_id: base.active_node_id.clone() },
            Self::SetFocusedNode { .. } => Self::SetFocusedNode { node_id: base.focused_node_id.clone() },
            Self::SetCollapsed { .. } => Self::SetCollapsed { node_ids: base.collapsed_node_ids.clone() },
            Self::SetPreviewOff { .. } => Self::SetPreviewOff { node_ids: base.preview_off_node_ids.clone() },
            Self::SetCamera { window_id, .. } | Self::RemoveCamera { window_id } => match base.camera.get(window_id) {
                Some(camera) => Self::SetCamera { window_id: window_id.clone(), camera: *camera },
                None if matches!(self, Self::RemoveCamera { .. }) => return Ok(Vec::new()),
                None => Self::RemoveCamera { window_id: window_id.clone() },
            },
        }])
    }
}

impl protocol::OpText for SpacePresenceMutation {
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
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown operation line '{line}'"),semio_framework_diagnostic::TextSpan::at(1,1)))
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

impl protocol::OpBinary for SpacePresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
