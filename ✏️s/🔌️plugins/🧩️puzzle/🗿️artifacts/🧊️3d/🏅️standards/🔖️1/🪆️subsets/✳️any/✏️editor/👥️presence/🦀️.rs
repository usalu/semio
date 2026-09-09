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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "puzzle3d.presence")]
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

impl protocol::MutationDiff<Puzzle3dPresence> for Puzzle3dPresence {
    fn apply(&self, _base: &Puzzle3dPresence) -> protocol::MutationApplyResult<Puzzle3dPresence> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

impl store::ArtifactDsl for Puzzle3dPresence {
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

impl ArtifactPack for Puzzle3dPresence {
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslOps)]
#[value(rename_all = "camelCase")]
pub enum Puzzle3dPresenceMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        presence: Puzzle3dPresence,
    },
}

impl Mutation<Puzzle3dPresence> for Puzzle3dPresenceMutation {
    type Diff = Puzzle3dPresence;

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

    fn diff(&self, _base: &Puzzle3dPresence) -> protocol::MutationOutcome<Puzzle3dPresence> {
        protocol::MutationOutcome::new(match self {
            Self::Snapshot { presence } => presence.clone(),
        })
    }

    fn inverse(&self, base: &Puzzle3dPresence) -> Vec<Self> {
        vec![Self::Snapshot { presence: base.clone() }]
    }
}

impl protocol::OpText for Puzzle3dPresenceMutation {
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

/// 👥️ Exact local and peer root ownership for puzzle3d presence: one bounded turn returns the
/// variable-length active-tool identifier, a second returns the inline root.
pub struct Puzzle3dPresenceRetirementFactory;

impl store::SnapshotRetirementFactory<Puzzle3dPresence> for Puzzle3dPresenceRetirementFactory {
    fn retire(&self, root: std::sync::Arc<Puzzle3dPresence>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(Puzzle3dPresenceRetirement { root: std::mem::ManuallyDrop::new(Some(root)), tool_id: std::mem::ManuallyDrop::new(None) })
    }
}

struct Puzzle3dPresenceRetirement {
    root: std::mem::ManuallyDrop<Option<std::sync::Arc<Puzzle3dPresence>>>,
    tool_id: std::mem::ManuallyDrop<Option<String>>,
}

impl store::ErasedSnapshotRetirement for Puzzle3dPresenceRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        if let Some(root) = self.root.take() {
            let released = std::sync::Arc::into_inner(root).and_then(|value| value.active_tool_id);
            let released_bytes = released.as_ref().map_or(0, String::len);
            if released_bytes > maximum_bytes {
                self.tool_id = std::mem::ManuallyDrop::new(released);
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            drop(released);
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
        }
        if let Some(tool_id) = self.tool_id.take() {
            let released_bytes = tool_id.len();
            if released_bytes > maximum_bytes {
                self.tool_id = std::mem::ManuallyDrop::new(Some(tool_id));
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            drop(tool_id);
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.root.is_none() && self.tool_id.is_none()
    }
}

impl Drop for Puzzle3dPresenceRetirement {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.root.is_none() && self.tool_id.is_none(), "puzzle3d presence retirement requires its exact terminal-empty witness");
        }
    }
}

/// 🧹️ The close-lane disposer paired with [`Puzzle3dPresenceRetirementFactory`].
pub fn puzzle3d_presence_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Puzzle3dPresence, Puzzle3dPresenceMutation>>> {
    Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Puzzle3dPresence::default()), puzzle3d_presence_is_terminal_empty).expect("the default puzzle3d presence root holds no active tool identifier"))
}
//#endregion 🧹️Retirement
