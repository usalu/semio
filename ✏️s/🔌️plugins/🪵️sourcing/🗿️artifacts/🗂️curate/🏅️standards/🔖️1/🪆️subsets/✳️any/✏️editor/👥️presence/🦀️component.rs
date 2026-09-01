//! 👥️ Sourcing curate presence — shareable live ephemeral state + mutations.

use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of sourcing curate view state (grid camera). Row selection now broadcasts
/// automatically through the framework's typed `PresenceInteraction` field for the "rows" interaction
/// domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — no longer mirrored here.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "sourcingcurate.presence")]
#[dsl(layout = "lines")]
pub struct SourcingCuratePresence {
    pub world_camera_position: [f64; 3],
    pub world_camera_target: [f64; 3],
    pub world_camera_fov: f64,
}

impl Default for SourcingCuratePresence {
    fn default() -> Self {
        Self { world_camera_position: [2.5, 2.0, 2.5], world_camera_target: [0.0, 0.0, 0.0], world_camera_fov: 50.0 }
    }
}

impl protocol::MutationDiff<SourcingCuratePresence> for SourcingCuratePresence {
    fn apply(&self, _base: &SourcingCuratePresence) -> protocol::MutationApplyResult<SourcingCuratePresence> {
        Ok({ self.clone() })
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

impl store::ArtifactDsl for SourcingCuratePresence {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for SourcingCuratePresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(rename_all = "camelCase")]
pub enum SourcingCuratePresenceMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        presence: SourcingCuratePresence,
    },
}

impl Mutation<SourcingCuratePresence> for SourcingCuratePresenceMutation {
    type Diff = SourcingCuratePresence;

    /// 🧷️ Hand-written (not `#[derive(dsl::Mutations)]`: a single whole-value snapshot replace, not a
    /// `dsl::Mutations`-eligible semantic-document vocabulary). ⚠️ PROVISIONAL: the `owner` path below
    /// names no directory on disk — this enum has no `🧬️mutations/<slug>` leaf triad of its own, so the
    /// entry is a metadata placeholder to satisfy `protocol::Mutation`, matching the sibling
    /// `🎚️config` enum and puzzle's `🖐️5d` precedent.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/📄snapshot", semantic_kind: "snapshot", display_name: "Snapshot", emoji: "📄", aggregate_variant: "Snapshot", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Self::Snapshot { .. } => &Self::DESCRIPTORS[0],
        }
    }

    /// 📦️ Whole-value snapshot replace — no target to be missing, so a message-free outcome per the
    /// contract's root-scoped shrink-only allowlist.
    fn diff(&self, _base: &SourcingCuratePresence) -> protocol::MutationOutcome<SourcingCuratePresence> {
        protocol::MutationOutcome::new(match self {
            Self::Snapshot { presence } => presence.clone(),
        })
    }

    fn inverse(&self, base: &SourcingCuratePresence) -> Vec<Self> {
        vec![Self::Snapshot { presence: base.clone() }]
    }
}

impl protocol::OpText for SourcingCuratePresenceMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl protocol::OpBinary for SourcingCuratePresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation

//#region 🧹️Retirement
const SOURCING_PRESENCE_BYTES: usize = 7 * std::mem::size_of::<f64>();
const _: () = assert!(std::mem::size_of::<SourcingCuratePresence>() == SOURCING_PRESENCE_BYTES && !std::mem::needs_drop::<SourcingCuratePresence>());

pub struct SourcingPresenceRetirementFactory;

impl store::SnapshotRetirementFactory<SourcingCuratePresence> for SourcingPresenceRetirementFactory {
    fn retire(&self, root: std::sync::Arc<SourcingCuratePresence>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(SourcingPresenceRetirement { root: std::mem::ManuallyDrop::new(Some(root)) })
    }
}

struct SourcingPresenceRetirement {
    root: std::mem::ManuallyDrop<Option<std::sync::Arc<SourcingCuratePresence>>>,
}

impl store::ErasedSnapshotRetirement for SourcingPresenceRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 || maximum_bytes < SOURCING_PRESENCE_BYTES {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let Some(root) = self.root.take() else { return Ok(store::SnapshotRetirementStep::Complete) };
        let released_bytes = if std::sync::Arc::into_inner(root).is_some() { SOURCING_PRESENCE_BYTES } else { 0 };
        Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes })
    }

    fn terminal_is_empty(&self) -> bool { self.root.is_none() }
}

impl Drop for SourcingPresenceRetirement {
    fn drop(&mut self) {
        if !std::thread::panicking() { assert!(self.root.is_none(), "Sourcing presence retirement requires exact terminal emptiness"); }
    }
}

pub struct SourcingPresenceStoreDisposer {
    terminal: Option<std::sync::Arc<SourcingCuratePresence>>,
    active: Option<store::PresenceStoreRetirement<SourcingCuratePresence>>,
}

impl SourcingPresenceStoreDisposer {
    pub fn new() -> Self { Self { terminal: Some(std::sync::Arc::new(SourcingCuratePresence::default())), active: None } }
}

impl semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<SourcingCuratePresence, SourcingCuratePresenceMutation>> for SourcingPresenceStoreDisposer {
    fn close_step(
        &mut self,
        owner: &mut store::PresenceStore<SourcingCuratePresence, SourcingCuratePresenceMutation>,
        maximum_items: usize,
        maximum_bytes: usize,
    ) -> Result<semio_framework_plugin::PluginCloseStep, semio_framework_plugin::Fault> {
        use semio_framework_plugin::PluginCloseStep;
        if maximum_items == 0 { return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }); }
        if let Some(active) = self.active.as_mut() {
            return active.close_step(1, maximum_bytes).map_err(semio_framework_plugin::Fault::from).map(|step| match step {
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => PluginCloseStep::Pending { released_items, released_bytes },
                store::SnapshotRetirementStep::Blocked => PluginCloseStep::Blocked { reason: "Sourcing presence retains local or peer readers" },
                store::SnapshotRetirementStep::Complete => PluginCloseStep::Complete,
            });
        }
        let terminal = self.terminal.take().expect("Sourcing presence terminal root");
        match owner.begin_retirement(terminal, |_| !std::mem::needs_drop::<SourcingCuratePresence>()) {
            Ok(active) => self.active = Some(active),
            Err((reason, terminal)) => { self.terminal = Some(terminal); return Err(semio_framework_plugin::Fault::from(reason)); }
        }
        Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
    }

    fn terminal_is_empty(&self, owner: &store::PresenceStore<SourcingCuratePresence, SourcingCuratePresenceMutation>) -> bool {
        self.terminal.is_none() && self.active.as_ref().is_some_and(store::PresenceStoreRetirement::terminal_is_empty) && owner.retirement_started() && owner.peers_root().is_empty()
    }
}
//#endregion 🧹️Retirement

//#region 🧪️RetirementTests
#[cfg(test)]
mod retirement_tests {
    use super::*;
    use std::sync::Arc;
    use store::{SnapshotRetirementFactory, SnapshotRetirementStep};

    #[test]
    fn sourcing_presence_retirement_preserves_shared_readers_and_exact_grants() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️retirement.json")).unwrap();
        let maximum_bytes = fixture["maximumBytes"].as_u64().unwrap() as usize;
        for value in fixture["snapshots"].as_array().unwrap() {
            let snapshot: SourcingCuratePresence = serde_json::from_value(value.clone()).unwrap();
            let packed = SourcingCuratePresence::decode_pack(&snapshot.encode_pack()).unwrap();
            assert_eq!(serde_json::to_value(packed).unwrap(), *value);
            let root = Arc::new(snapshot);
            let weak = Arc::downgrade(&root);
            let reader = root.clone();
            let mut first = SourcingPresenceRetirementFactory.retire(root);
            assert_eq!(first.close_step(0, maximum_bytes).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            assert_eq!(first.close_step(1, maximum_bytes - 1).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            assert_eq!(first.close_step(1, maximum_bytes).unwrap(), SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            assert_eq!(first.close_step(1, maximum_bytes).unwrap(), SnapshotRetirementStep::Complete);
            assert!(first.terminal_is_empty());
            assert_eq!(serde_json::to_value(reader.as_ref()).unwrap(), *value);
            let mut final_owner = SourcingPresenceRetirementFactory.retire(reader);
            assert_eq!(final_owner.close_step(1, maximum_bytes).unwrap(), SnapshotRetirementStep::Pending { released_items: 1, released_bytes: maximum_bytes });
            assert_eq!(final_owner.close_step(1, maximum_bytes).unwrap(), SnapshotRetirementStep::Complete);
            assert!(final_owner.terminal_is_empty());
            assert!(weak.upgrade().is_none());
        }
    }
}
//#endregion 🧪️RetirementTests
