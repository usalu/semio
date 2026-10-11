//! 👥️ Home presence — shareable live ephemeral state + mutations.
//!
//! Empty: the home launcher keeps panel tab and locale in [`crate::editor::home::config::HomeConfig`];
//! there is no multi-user shareable live surface on the launcher.

use protocol::Mutation;
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ No shareable live launcher state — chrome stays in `HomeConfig`.
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct HomePresence {}

/// 🔺️ Sparse delta over [`HomePresence`]: the presence record carries no field, so its delta names nothing.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct HomePresenceDiff {}

impl protocol::MutationDiff<HomePresence> for HomePresenceDiff {
    fn apply(&self, base: &HomePresence, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<HomePresence> {
        Ok(base.clone())
    }
    fn absorb(&mut self, _other: Self) {}
}

impl protocol::DiffAlgebra<HomePresence> for HomePresenceDiff {
    fn inverse(&self, _base: &HomePresence) -> Self {
        Self {}
    }
    fn is_empty(&self) -> bool {
        true
    }
}

impl store::ArtifactDsl for HomePresence {
    const EXTENSION: &'static str = "home.presence";
    fn envelope_id() -> &'static str {
        "home.presence"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "home presence", semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        String::new()
    }
}

impl ArtifactPack for HomePresence {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        Ok(Vec::new())
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        if !inner.is_empty() {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "home presence pack must be empty")));
        }
        Ok(Self::default())
    }
}
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub enum HomePresenceMutation {
    Noop,
}

impl Mutation<HomePresence> for HomePresenceMutation {
    /// 🧷️ Provisional per-variant leaf metadata — one entry per variant, in declaration order.
    /// ⚠️ PROVISIONAL: mirrors the sibling `🪐️space` presence aggregate's own provisional descriptors
    /// (`⚙️engine/🪐️space/👥️presence/🦀️.rs`) — no variant below has an authored leaf
    /// directory on disk yet.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/⚙️noop", semantic_kind: "noop", display_name: "Noop", emoji: "⚙️", aggregate_variant: "Noop", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::SelfInvertible, diff_participation: protocol::MutationDiffParticipation::None, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            HomePresenceMutation::Noop => &Self::DESCRIPTORS[0],
        }
    }

    type Diff = HomePresenceDiff;

    fn diff(&self, _base: &HomePresence) -> protocol::MutationOutcome<HomePresenceDiff> {
        protocol::MutationOutcome::new(HomePresenceDiff {})
    }

    fn inverse(&self, _base: &HomePresence) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![HomePresenceMutation::Noop]
    
    })())
}
}

impl protocol::OpText for HomePresenceMutation {
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

impl protocol::OpBinary for HomePresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation

//#region 🫴️NativePresence
impl store::ArtifactPresenceSnapshot for HomePresence {}
//#endregion 🫴️NativePresence
