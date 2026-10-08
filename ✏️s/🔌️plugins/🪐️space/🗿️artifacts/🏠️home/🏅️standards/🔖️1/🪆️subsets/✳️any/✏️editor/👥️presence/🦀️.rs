//! 👥️ Home presence — shareable live ephemeral state + mutations.
//!
//! Empty: the home launcher keeps panel tab and locale in [`crate::editor::home::config::HomeConfig`];
//! there is no multi-user shareable live surface on the launcher.

use protocol::Mutation;
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ No shareable live launcher state — chrome stays in `HomeConfig`.
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct HomePresence {}

impl protocol::MutationDiff<HomePresence> for HomePresence {
    fn apply(&self, base: &HomePresence) -> protocol::MutationApplyResult<HomePresence> {
        Ok(base.clone())
    }
    fn absorb(&mut self, _other: Self) {}
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
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

    type Diff = HomePresence;

    fn diff(&self, _base: &HomePresence) -> protocol::MutationOutcome<HomePresence> {
        protocol::MutationOutcome::new(HomePresence::default())
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

//#region ♻️Retirement
/// ♻️ Exact bounded retirement for one displaced `HomePresence` root.
///
/// `PresenceStore::local_read` fails closed with `presence local read requires a live exact local
/// retirement owner` while no factory is installed, and Home reads its own presence on the
/// `createStudio` path — so without this the local studio path is refused even though nothing is
/// wrong with the command. `HomePresence` owns no heap collections (the launcher keeps its chrome in
/// `HomeConfig`), so one root retires in a single step and reports one released item.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct HomePresenceRetirementFactory;

impl store::SnapshotRetirementFactory<HomePresence> for HomePresenceRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &std::sync::Arc<HomePresence>) -> usize { std::mem::size_of::<HomePresenceRetirement>() }

    fn retire(&self, root: std::sync::Arc<HomePresence>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(HomePresenceRetirement { root: std::mem::ManuallyDrop::new(Some(root)) })
    }
}

struct HomePresenceRetirement {
    root: std::mem::ManuallyDrop<Option<std::sync::Arc<HomePresence>>>,
}

impl store::ErasedSnapshotRetirement for HomePresenceRetirement {
    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        if let Some(root) = self.root.take() {
            drop(root);
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.root.is_none()
    }
}
//#endregion ♻️Retirement
