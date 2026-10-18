//! 🧭️ Backbone retention, bootstrap and command acks: the document backbone retention, the bootstrap identity, the frontier reach, the command rejection codes and the command ack outcome. See [`super`].

use super::*;

//#region 🔖️Protocol
pub fn decode_document_backbone_message_exact(message: &[u8]) -> Result<Vec<MutationEnvelope>, String> {
    match decode_hot_backbone_message_exact(message).map_err(|error| error.to_string())? {
        BackboneMessage::Mutations { envelopes } => decode_document_backbone_envelopes_exact(&envelopes).map_err(|error| error.to_string()),
        BackboneMessage::Genesis { .. } | BackboneMessage::Ack { .. } | BackboneMessage::Member { .. } | BackboneMessage::Retract { .. } => Err("document backbone requires a canonical mutation message".into()),
    }
}

#[derive(Default)]
pub struct DocumentBackboneRetentionV1 {
    pub entries: std::collections::HashMap<String, (usize, usize)>,
    pub bytes: usize,
    pub messages: usize,
}

impl DocumentBackboneRetentionV1 {
    pub fn retain(&mut self, message_bytes: usize, envelopes: &[MutationEnvelope]) -> Result<(), CommandAckOutcome> {
        if envelopes.is_empty() {
            return Ok(());
        }
        let refused = |code, reason: &str, limit: usize| CommandAckOutcome::local_rejected(code, reason, CommandRejectionDetail { envelopes: Some(envelopes.len() as u64), bytes: Some(message_bytes as u64), limit: Some(limit as u64) });
        if message_bytes == 0 || message_bytes > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES {
            return Err(refused(CommandRejectionCode::LocalBackboneCapacity, "document backbone message exceeds its hot byte limit", BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES));
        }
        let Some(next_bytes) = self.bytes.checked_add(message_bytes) else {
            return Err(refused(CommandRejectionCode::LocalBackboneCapacity, "document backbone byte accounting overflow", BACKBONE_CHANNEL_MAXIMUM_BYTES));
        };
        if self.messages >= BACKBONE_CHANNEL_MAXIMUM_MESSAGES || next_bytes > BACKBONE_CHANNEL_MAXIMUM_BYTES {
            return Err(refused(CommandRejectionCode::LocalBackboneCapacity, "document backbone pending capacity", BACKBONE_CHANNEL_MAXIMUM_BYTES));
        }
        let mut admitted = std::collections::HashSet::with_capacity(envelopes.len());
        if envelopes.iter().any(|envelope| !admitted.insert(envelope.mutation_id.0.as_str()) || self.entries.contains_key(&envelope.mutation_id.0)) {
            return Err(CommandAckOutcome::local_rejected(CommandRejectionCode::LocalBackboneDuplicate, "document backbone mutation identity is already retained", CommandRejectionDetail { envelopes: Some(envelopes.len() as u64), ..CommandRejectionDetail::default() }));
        }
        for (index, envelope) in envelopes.iter().enumerate() {
            self.entries.insert(envelope.mutation_id.0.clone(), if index == 0 { (message_bytes, 1) } else { (0, 0) });
        }
        self.bytes = next_bytes;
        self.messages += 1;
        Ok(())
    }

    pub fn release(&mut self, envelopes: &[MutationEnvelope]) {
        for envelope in envelopes {
            if let Some((bytes, messages)) = self.entries.remove(&envelope.mutation_id.0) {
                self.bytes = self.bytes.checked_sub(bytes).expect("document backbone retained byte ledger underflow");
                self.messages = self.messages.checked_sub(messages).expect("document backbone retained message ledger underflow");
            }
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.bytes = 0;
        self.messages = 0;
    }
}

pub const ARTIFACT_BOOTSTRAP_DEADLINE_MS: u64 = 15_000;

/// 🧬️ Binds a public artifact bootstrap to the already-selected document codec before
/// allocating or accepting any payload bytes.
pub fn validate_artifact_bootstrap_identity(bootstrap: &ArtifactBootstrap, document_id: &str, schema: &str, pack_schema_hash: [u8; 32], server_frontier: &RuntimeFrontierSummary) -> Result<(), String> {
    if bootstrap.artifact_schema != schema {
        return Err(format!("artifact schema mismatch: expected {schema:?}, got {:?}", bootstrap.artifact_schema));
    }
    if bootstrap.baseline_frontier.document_id.0 != document_id || bootstrap.required_tail_frontier.document_id.0 != document_id || server_frontier.document_id.0 != document_id {
        return Err("artifact bootstrap document mismatch".into());
    }
    if bootstrap.pack_schema_hash != pack_schema_hash || pack_schema_hash == [0u8; 32] {
        return Err("artifact bootstrap pack schema mismatch".into());
    }
    if &bootstrap.required_tail_frontier != server_frontier {
        return Err("artifact bootstrap required tail does not match welcome frontier".into());
    }
    Ok(())
}

/// 📌️ The hub's frontier in the Check In head grammar; `None` for a genesis (never edited) frontier.
pub fn acknowledged_edited_head(frontier: &RuntimeFrontierSummary) -> Option<semio_framework_os_directory::EditedArtifactFrontierV1> {
    let chain_sha256: String = frontier.chain_hash.iter().map(|byte| format!("{byte:02x}")).collect();
    let head = semio_framework_os_directory::EditedArtifactFrontierV1 { document_id: frontier.document_id.0.clone(), head_edit_ordinal: frontier.head_edit_ordinal, head_edit_id: frontier.head_edit_id.clone(), last_commit_seq: frontier.last_commit_seq, chain_sha256 };
    head.validate().then_some(head)
}

pub fn frontier_reaches(actual: &RuntimeFrontierSummary, required: &RuntimeFrontierSummary) -> bool {
    actual == required
}

/// 🧾️ Every class of refused command batch, one closed set for every producer: the hub (`hub.refused`, or `hub.unreadable`
/// when its diagnostics do not decode) and an actor's own refusals before any frame leaves it. Schema
/// `🧬️schema/🔣️command-rejection/🔣️.json`, TypeScript twin `COMMAND_REJECTION_CODES_V1` (`💻️os/🟦️.ts`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
pub enum CommandRejectionCode {
    #[value(rename = "hub.refused")]
    HubRefused,
    #[value(rename = "hub.unreadable")]
    HubUnreadable,
    #[value(rename = "local.read-only")]
    LocalReadOnly,
    #[value(rename = "local.queue-full")]
    LocalQueueFull,
    #[value(rename = "local.backbone-capacity")]
    LocalBackboneCapacity,
    #[value(rename = "local.backbone-duplicate")]
    LocalBackboneDuplicate,
    #[value(rename = "local.backbone-pair-unavailable")]
    LocalBackbonePairUnavailable,
    #[value(rename = "local.backbone-scope-mismatch")]
    LocalBackboneScopeMismatch,
    #[value(rename = "local.backbone-malformed")]
    LocalBackboneMalformed,
    #[value(rename = "local.socket-frame-ceiling")]
    LocalSocketFrameCeiling,
}

/// 🔢️ The typed counters a local refusal reports: the refused batch's envelopes, the bytes it or its queue holds, and the
/// limit it met — TypeScript twin `CommandRejectionDetailV1`.
#[derive(Clone, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
pub struct CommandRejectionDetail {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub envelopes: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

/// ⚖️ The client-side twin of `crate::os_spr::wire::ApplyOutcome`, minus the `Transformed`
/// envelope payload (already delivered separately as {@link ArtifactEvent::RemoteMutations} by
/// the time this fires — see {@link ArtifactEvent::CommandOutcome}). `Rejected` is the one rejection contract every
/// producer answers (`🧬️schema/🔣️command-rejection/🔣️.json`): its class, the producer's English diagnostic `reason` (never
/// shown to a human), the structured `MutationMessage`s the hub graded the batch with (decoded once, empty for a local
/// refusal) and a local refusal's counters.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum CommandAckOutcome {
    Accepted,
    Transformed,
    Rejected {
        code: CommandRejectionCode,
        reason: String,
        messages: Vec<MutationMessage>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        detail: Option<CommandRejectionDetail>,
    },
}

impl CommandAckOutcome {
    /// 🧾️ The hub's `ApplyOutcome::Rejected`, decoded exactly once: its `messages` bytes are the UTF-8 JSON of one
    /// `MutationMessage` array (`🌎️hub` `encode_messages`; empty for an admission or transport refusal), read into
    /// `hub.refused`, or `hub.unreadable` with no messages when they are not that array.
    pub fn hub_rejected(reason: String, messages: &[u8]) -> Self {
        let decoded = if messages.is_empty() { Some(Vec::new()) } else { std::str::from_utf8(messages).ok().and_then(|text| semio_framework_pack_json::from_json_str::<Vec<MutationMessage>>(text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()) };
        match decoded {
            Some(messages) => Self::Rejected { code: CommandRejectionCode::HubRefused, reason, messages, detail: None },
            None => Self::Rejected { code: CommandRejectionCode::HubUnreadable, reason, messages: Vec::new(), detail: None },
        }
    }

    /// 🚫️ A refusal an actor answers before any frame leaves it: no hub messages, only its typed counters (none when all
    /// are absent).
    pub fn local_rejected(code: CommandRejectionCode, reason: &str, detail: CommandRejectionDetail) -> Self {
        Self::Rejected { code, reason: reason.to_string(), messages: Vec::new(), detail: (detail != CommandRejectionDetail::default()).then_some(detail) }
    }
}
//#endregion 🔖️Protocol
