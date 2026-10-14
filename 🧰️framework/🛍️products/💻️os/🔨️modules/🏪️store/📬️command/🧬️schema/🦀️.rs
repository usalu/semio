//! 📬 Typed command vocabulary and binary codec owned with its retirement.
use crate::*;
use protocol::{MutationId, UndoPolicy, Mutation, OpBinary};
use semio_framework_os_vcs::Author;
use semio_framework_os_store_projection::ArtifactProjectionCause;
use crate::operation_envelope_serde;

/// 🛤️ Which undo/redo cursor a recorded edit belongs to. `Document` is the default — the
/// ordinary main history `Undo`/`Redo` travels. Any other lane (starting with `Interaction`) is a
/// persisted, replayable SIDE history: mutations recorded under it stay in `ArtifactVcs.edits` and
/// `ArtifactEnvelope.lanes` forever (never dropped, never un-persisted), but default `Undo`/`Redo`
/// skip past them to the nearest `Document`-lane entry instead of reverting them — see
/// `ArtifactStore::dispatch`'s `Undo`/`Redo` arms, and `ArtifactCommand::UndoInLane`/`RedoInLane`
/// for the explicit lane-scoped API that CAN walk a non-`Document` lane. Deliberately an
/// extensible general store mechanism — e.g. a future framework-owned selection/interaction state
/// that must survive reload but must never be what a document editor's undo reverts — not a
/// hover/selection special case baked into this crate. See ticket
/// `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM/📋️master.md` decision 1.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub enum HistoryLane {
    #[default]
    Document,
    Interaction,
}

impl HistoryLane {
    pub async fn is_document(&self) -> bool {
        matches!(self, HistoryLane::Document)
    }
}


#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ArtifactCommand<Mutation> {
    /// ▶️ Records `mutations` as one edit; `transaction` names the committed tool transaction that authored them, stamped
    /// on every operation's metadata and wire envelope.
    Apply {
        mutations: Vec<Mutation>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        transaction: Option<protocol::TransactionRef>,
    },
    Undo,
    Redo,
    UndoWithPolicy {
        policy: UndoPolicy,
        #[value(default, skip_serializing_if = "Option::is_none")]
        semantic_command: Option<Box<ArtifactCommand<Mutation>>>,
    },
    /// 🛤️ `Apply`'s lane-tagged twin — records the resulting edit under `lane` (via
    /// `ArtifactEnvelope.lanes`) instead of the implicit `HistoryLane::Document` a plain `Apply`
    /// gets.
    ApplyInLane {
        mutations: Vec<Mutation>,
        #[value(default)]
        lane: HistoryLane,
        #[value(default, skip_serializing_if = "Option::is_none")]
        transaction: Option<protocol::TransactionRef>,
    },
    /// 🛤️ Explicit lane-scoped undo: mirrors plain `Undo`'s selective (own-author) semantics
    /// but searches `applied_edit_ids` for the nearest local entry whose `HistoryLane` is exactly
    /// `lane`, instead of `HistoryLane::Document`. Lets a caller walk a non-`Document` lane on
    /// purpose — the completing half of "default `Undo`/`Redo` skip non-`Document` lanes" (see
    /// `ArtifactStore::dispatch`'s `Undo` arm).
    UndoInLane {
        lane: HistoryLane,
    },
    /// 🛤️ `UndoInLane`'s redo-direction sibling — mirrors plain `Redo`, but pops the
    /// nearest-to-top `redo_edit_ids` entry whose `HistoryLane` is exactly `lane`.
    RedoInLane {
        lane: HistoryLane,
    },
    CommitCheckpoint {
        #[value(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        authors: Vec<Author>,
    },
    CreateAlternative {
        name: String,
    },
    SwitchAlternative {
        alternative_id: String,
    },
    CheckoutCheckpoint {
        #[value(rename = "checkpointId")]
        checkpoint_id: String,
    },
    /// 🕸️ Feeds a remote MutationEnvelope through the causal DAG into the edit timeline.
    IngestRemote {
        #[value(serialize_with = "operation_envelope_serde::to_value", deserialize_with = "operation_envelope_serde::from_value")]
        envelope: protocol::MutationEnvelope,
    },
    /// 🧹 Clears volatile draft-lane history that must never enter a Change/Checkpoint.
    PruneDrafts,
    /// ⚖️ Sets this store's local `protocol::MergePolicy` — authority-local state, never
    /// carried on a `protocol::MutationEnvelope`/`BackboneMessage`, never part of shared
    /// history (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6). Ordinal
    /// 15 — frozen, appended after `PruneDrafts`.
    SetMergePolicy {
        policy: protocol::MergePolicy,
    },
    /// ⚔️ Resolves an `Open` `protocol::Conflict` by id — see
    /// `ArtifactStore::resolve_conflict` for what `Accept`/`Discard` do for each conflict kind.
    /// Ordinal 16 — frozen.
    ResolveConflict {
        conflict_id: String,
        resolution: protocol::ConflictResolution,
    },
    /// ✏️ Supersedes the effective input of applied operations, non-destructively: each input replaces its target's
    /// input (`None` withdraws it) in every alternative, or only within the alternative `scope`. Refused atomically
    /// with `Rejected { policy: Normal }` when the Report replay of everything downstream raises an `Error` or `Fatal`.
    /// Ordinal 17.
    Supersede {
        #[value(default, skip_serializing_if = "Option::is_none")]
        scope: Option<String>,
        inputs: Vec<SupersedeInput<Mutation>>,
    },
    /// 🌿️ Commits the pending edits, branches the alternative `name` at the head and supersedes `inputs` within it —
    /// one atomic dispatch leaving in one outbound batch. Ordinal 18.
    CreateAlternativeWithSupersede {
        name: String,
        inputs: Vec<SupersedeInput<Mutation>>,
    },
    /// 🎞️ Appends `mutations` to the open edit of the tool transaction `transaction`, opening it with its first append:
    /// a streamed tool's ticks become one edit, every operation stamped with the ref and none announced before
    /// `CommitTransaction`. While it is open the store refuses every command but its appends, its commit or abort, a remote
    /// ingest and a merge-policy change (`VcsError::TransactionOpen`). Ordinal 19.
    AppendTransaction {
        mutations: Vec<Mutation>,
        transaction: protocol::TransactionRef,
    },
    /// ✅️ Closes the open edit of the tool transaction `transaction_id` and announces every operation it holds. Ordinal 20.
    CommitTransaction {
        transaction_id: String,
    },
    /// 🧨️ Reverts the open edit of the tool transaction `transaction_id` with zero trace: no edit, no operation, no
    /// announcement, the projection and revision of before its first append. Ordinal 21.
    AbortTransaction {
        transaction_id: String,
    },
}

/// 🎞️ The tool transaction a store holds open ([`ArtifactCommand::AppendTransaction`]): its ref, the one edit its
/// appends grow — kept at the tail of the local applied history until its commit announces it or its abort removes it.
#[derive(Clone, Debug, PartialEq)]
pub struct OpenToolTransaction {
    pub transaction: protocol::TransactionRef,
    pub edit_id: String,
}


/// ✏️ One input of [`ArtifactCommand::Supersede`]: the superseded operation and the replacement it folds as from now
/// on (`None` withdraws it).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SupersedeInput<Mutation> {
    pub target: MutationId,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<Mutation>,
}

impl<Mutation> ArtifactCommand<Mutation> {
    /// 🔮️ Names the projection-invalidating transition this command performs, if it has one.
    pub async fn projection_cause(&self) -> Option<ArtifactProjectionCause> {
        match self {
            Self::Apply { .. } | Self::ApplyInLane { .. } => Some(ArtifactProjectionCause::Apply),
            Self::IngestRemote { .. } => Some(ArtifactProjectionCause::RemoteIngest),
            Self::Undo | Self::UndoWithPolicy { .. } | Self::UndoInLane { .. } => Some(ArtifactProjectionCause::Undo),
            Self::Redo | Self::RedoInLane { .. } => Some(ArtifactProjectionCause::Redo),
            Self::CheckoutCheckpoint { .. } | Self::CreateAlternative { .. } | Self::SwitchAlternative { .. } => Some(ArtifactProjectionCause::Checkout),
            Self::CommitCheckpoint { .. } => Some(ArtifactProjectionCause::Checkpoint),
            Self::PruneDrafts => Some(ArtifactProjectionCause::PruneDrafts),
            Self::SetMergePolicy { .. } => Some(ArtifactProjectionCause::PolicyChange),
            Self::ResolveConflict { .. } => Some(ArtifactProjectionCause::RemoteIngest),
            Self::Supersede { .. } | Self::CreateAlternativeWithSupersede { .. } => Some(ArtifactProjectionCause::Replay),
            Self::AppendTransaction { .. } | Self::CommitTransaction { .. } => Some(ArtifactProjectionCause::Apply),
            Self::AbortTransaction { .. } => Some(ArtifactProjectionCause::Undo),
        }
    }

    /// 🚦️ Whether this command moves history positions — an undo, redo, checkpoint, alternative, checkout, supersession or
    /// conflict resolution: what a store whose local history step still replays refuses ([`VcsError::HistoryReplaying`]).
    pub fn moves_history(&self) -> bool {
        !self.history_transition_kinds().is_empty() || matches!(self, Self::ResolveConflict { .. })
    }

    /// 🏷️ Every kind of history transition this command may author, in authoring order: what the store's
    /// [`protocol::HistoryShape`] must admit before the command runs.
    pub fn history_transition_kinds(&self) -> &'static [protocol::HistoryTransitionKind] {
        use protocol::HistoryTransitionKind::{Branch, Checkout, Commit, Reinstate, Revert, Supersede};
        match self {
            Self::Undo | Self::UndoWithPolicy { .. } | Self::UndoInLane { .. } => &[Revert],
            Self::Redo | Self::RedoInLane { .. } => &[Reinstate],
            Self::CommitCheckpoint { .. } => &[Commit],
            Self::CreateAlternative { .. } => &[Commit, Branch],
            Self::SwitchAlternative { .. } | Self::CheckoutCheckpoint { .. } => &[Checkout],
            Self::Supersede { .. } => &[Supersede],
            Self::CreateAlternativeWithSupersede { .. } => &[Commit, Branch, Supersede],
            Self::Apply { .. }
            | Self::ApplyInLane { .. }
            | Self::IngestRemote { .. }
            | Self::PruneDrafts
            | Self::SetMergePolicy { .. }
            | Self::ResolveConflict { .. }
            | Self::AppendTransaction { .. }
            | Self::CommitTransaction { .. }
            | Self::AbortTransaction { .. } => &[],
        }
    }
}


/// 🎯️ B-R6 "one documented generic impl": `ArtifactCommand<Op>` cannot go through
/// `#[derive(crate::os_dsl::DslOps)]` like every concrete per-technology `Mutation` enum does — it is generic
/// over a FOREIGN `Op: OpBinary` from whichever technology is dispatching, and the derive
/// only lowers a CONCRETE type's own fields to a `RecordSpec`; there is no way to describe "some
/// other crate's already-`OpBinary` type" as a `crate::os_dsl::DslField` shape. This hand-written impl is the
/// one place `ArtifactCommand`'s binary layout is still spelled out by hand — everywhere else in the
/// workspace, encoding is derive-generated. Byte layout matches the convention exactly: `format u8
/// (=1, see `COMMAND_BINARY_FORMAT`) | fixed variant ordinal varint (one stable number per variant; LEB128 varints
/// under 128 are single bytes) | body`. The binary twin of `print_command`/`parse_command`. LAW:
/// `ArtifactCommand::decode_command(&command.encode_command()?) == Ok(command)`. Not an `OpBinary`: decoding needs the
/// operations' projection `P` to retire what a refused command decoded, which the P-agnostic `OpBinary` cannot name.
impl<Op: OpBinary> ArtifactCommand<Op> {
    /// 🛫️ The command's binary form.
    pub fn encode_command(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut out = vec![COMMAND_BINARY_FORMAT];
        match self {
            ArtifactCommand::Apply { mutations, transaction } => {
                semio_framework_os_pack::write_varint_u64(&mut out, 0);
                out.push(u8::from(transaction.is_some()) << 1);
                write_command_transaction(&mut out, transaction.as_ref());
                write_command_ops(&mut out, mutations)?;
            }
            ArtifactCommand::Undo => semio_framework_os_pack::write_varint_u64(&mut out, 1),
            ArtifactCommand::Redo => semio_framework_os_pack::write_varint_u64(&mut out, 2),
            ArtifactCommand::UndoWithPolicy { policy, semantic_command } => {
                semio_framework_os_pack::write_varint_u64(&mut out, 3);
                out.push(undo_policy_ordinal(*policy));
                out.push(if semantic_command.is_some() { 0b01 } else { 0 });
                if let Some(nested) = semantic_command {
                    let nested_bytes = nested.encode_command()?;
                    semio_framework_os_pack::write_varint_u64(&mut out, nested_bytes.len() as u64);
                    out.extend_from_slice(&nested_bytes);
                }
            }
            ArtifactCommand::CommitCheckpoint { message, authors } => {
                semio_framework_os_pack::write_varint_u64(&mut out, 4);
                out.push(if message.is_some() { 0b01 } else { 0 });
                if let Some(text) = message {
                    write_command_str(&mut out, text);
                }
                semio_framework_os_pack::write_varint_u64(&mut out, authors.len() as u64);
                for author in authors {
                    write_command_str(&mut out, &author.id);
                    write_command_str(&mut out, &author.name);
                }
            }
            ArtifactCommand::CreateAlternative { name } => {
                semio_framework_os_pack::write_varint_u64(&mut out, 5);
                write_command_str(&mut out, name);
            }
            ArtifactCommand::SwitchAlternative { alternative_id } => {
                semio_framework_os_pack::write_varint_u64(&mut out, 6);
                write_command_str(&mut out, alternative_id);
            }
            ArtifactCommand::CheckoutCheckpoint { checkpoint_id } => {
                semio_framework_os_pack::write_varint_u64(&mut out, 7);
                write_command_str(&mut out, checkpoint_id);
            }
            ArtifactCommand::IngestRemote { envelope } => {
                semio_framework_os_pack::write_varint_u64(&mut out, 9);
                let mut bytes = Vec::new();
                protocol::encode_envelope(envelope, &mut bytes);
                semio_framework_os_pack::write_varint_u64(&mut out, bytes.len() as u64);
                out.extend_from_slice(&bytes);
            }
            ArtifactCommand::PruneDrafts => semio_framework_os_pack::write_varint_u64(&mut out, 10),
            ArtifactCommand::ApplyInLane { mutations, lane, transaction } => {
                semio_framework_os_pack::write_varint_u64(&mut out, 11);
                out.push(u8::from(transaction.is_some()) << 1);
                write_command_transaction(&mut out, transaction.as_ref());
                out.push(history_lane_ordinal(*lane));
                write_command_ops(&mut out, mutations)?;
            }
            ArtifactCommand::UndoInLane { lane } => {
                semio_framework_os_pack::write_varint_u64(&mut out, 13);
                out.push(history_lane_ordinal(*lane));
            }
            ArtifactCommand::RedoInLane { lane } => {
                semio_framework_os_pack::write_varint_u64(&mut out, 14);
                out.push(history_lane_ordinal(*lane));
            }
            ArtifactCommand::SetMergePolicy { policy } => {
                semio_framework_os_pack::write_varint_u64(&mut out, 15);
                out.push(policy.as_u8());
            }
            ArtifactCommand::ResolveConflict { conflict_id, resolution } => {
                semio_framework_os_pack::write_varint_u64(&mut out, 16);
                write_command_str(&mut out, conflict_id);
                out.push(conflict_resolution_ordinal(*resolution));
            }
            ArtifactCommand::Supersede { scope, inputs } => {
                semio_framework_os_pack::write_varint_u64(&mut out, 17);
                out.push(if scope.is_some() { 0b01 } else { 0 });
                if let Some(scope) = scope {
                    write_command_str(&mut out, scope);
                }
                write_supersede_inputs(&mut out, inputs)?;
            }
            ArtifactCommand::CreateAlternativeWithSupersede { name, inputs } => {
                semio_framework_os_pack::write_varint_u64(&mut out, 18);
                write_command_str(&mut out, name);
                write_supersede_inputs(&mut out, inputs)?;
            }
            ArtifactCommand::AppendTransaction { mutations, transaction } => {
                semio_framework_os_pack::write_varint_u64(&mut out, 19);
                write_command_str(&mut out, &transaction.id);
                write_command_str(&mut out, &transaction.tool);
                write_command_ops(&mut out, mutations)?;
            }
            ArtifactCommand::CommitTransaction { transaction_id } => {
                semio_framework_os_pack::write_varint_u64(&mut out, 20);
                write_command_str(&mut out, transaction_id);
            }
            ArtifactCommand::AbortTransaction { transaction_id } => {
                semio_framework_os_pack::write_varint_u64(&mut out, 21);
                write_command_str(&mut out, transaction_id);
            }
        }
        Ok(out)
    }

    /// 🛬️ Decodes one whole binary command, all-or-nothing: bytes past its end refuse it, and every operation decoded
    /// before a failure retires through its technology's cold disposal — a refusal never drops a live operation.
    pub fn decode_command<P>(bytes: &[u8]) -> Result<Self, CommandDecodeError>
    where
        Op: Mutation<P>,
    {
        let mut reader = semio_framework_os_pack::ByteReader::new(bytes);
        let command = Self::read_command::<P>(&mut reader)?;
        if reader.remaining() != 0 {
            let detail = format!("{} bytes past the command's end", reader.remaining());
            retire_command::<P, Op>(command);
            return Err(CommandDecodeError::Layout(protocol::ProtocolError::Malformed { what: "command", offset: reader.position() as u64, detail }));
        }
        Ok(command)
    }

    /// 📖️ The command at `reader`; its operation lists come last, so nothing can fail after one was decoded.
    fn read_command<P>(reader: &mut semio_framework_os_pack::ByteReader<'_>) -> Result<Self, CommandDecodeError>
    where
        Op: Mutation<P>,
    {
        let format = reader.read_u8()?;
        if format != COMMAND_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "command format", offset: 0, detail: format!("unsupported command format {format}") }.into());
        }
        let ordinal = reader.read_varint_u64()?;
        match ordinal {
            0 => {
                let presence = reader.read_u8()?;
                let transaction = read_command_transaction(reader, presence)?;
                let mutations = read_command_ops::<P, Op>(reader)?;
                Ok(ArtifactCommand::Apply { mutations, transaction })
            }
            1 => Ok(ArtifactCommand::Undo),
            2 => Ok(ArtifactCommand::Redo),
            3 => {
                let policy = undo_policy_from_ordinal(reader.read_u8()?)?;
                let presence = reader.read_u8()?;
                let semantic_command = if presence & 0b01 != 0 {
                    let len = reader.read_varint_u64()?;
                    let nested_bytes = reader.read_bytes(len as usize)?;
                    Some(Box::new(ArtifactCommand::<Op>::decode_command::<P>(nested_bytes)?))
                } else {
                    None
                };
                Ok(ArtifactCommand::UndoWithPolicy { policy, semantic_command })
            }
            4 => {
                let presence = reader.read_u8()?;
                let message = if presence & 0b01 != 0 { Some(read_command_str(reader)?) } else { None };
                let author_count = reader.read_varint_u64()?;
                let mut authors = Vec::with_capacity(author_count as usize);
                for _ in 0..author_count {
                    let id = read_command_str(reader)?;
                    let name = read_command_str(reader)?;
                    authors.push(Author { id, name, avatar: None });
                }
                Ok(ArtifactCommand::CommitCheckpoint { message, authors })
            }
            5 => Ok(ArtifactCommand::CreateAlternative { name: read_command_str(reader)? }),
            6 => Ok(ArtifactCommand::SwitchAlternative { alternative_id: read_command_str(reader)? }),
            7 => Ok(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: read_command_str(reader)? }),
            9 => {
                let len = reader.read_varint_u64()?;
                let bytes = reader.read_bytes(len as usize)?;
                let mut pos = 0;
                let envelope = protocol::decode_envelope(bytes, &mut pos).map_err(|error| protocol::ProtocolError::Malformed { what: "ingest envelope", offset: 0, detail: error.to_string() })?;
                Ok(ArtifactCommand::IngestRemote { envelope })
            }
            10 => Ok(ArtifactCommand::PruneDrafts),
            11 => {
                let presence = reader.read_u8()?;
                let transaction = read_command_transaction(reader, presence)?;
                let lane = history_lane_from_ordinal(reader.read_u8()?)?;
                let mutations = read_command_ops::<P, Op>(reader)?;
                Ok(ArtifactCommand::ApplyInLane { mutations, lane, transaction })
            }
            13 => Ok(ArtifactCommand::UndoInLane { lane: history_lane_from_ordinal(reader.read_u8()?)? }),
            14 => Ok(ArtifactCommand::RedoInLane { lane: history_lane_from_ordinal(reader.read_u8()?)? }),
            15 => {
                let policy = protocol::MergePolicy::from_u8(reader.read_u8()?).ok_or_else(|| protocol::ProtocolError::Malformed { what: "merge policy ordinal", offset: 1, detail: "unknown merge policy ordinal".to_string() })?;
                Ok(ArtifactCommand::SetMergePolicy { policy })
            }
            16 => {
                let conflict_id = read_command_str(reader)?;
                let resolution = conflict_resolution_from_ordinal(reader.read_u8()?)?;
                Ok(ArtifactCommand::ResolveConflict { conflict_id, resolution })
            }
            17 => {
                let presence = reader.read_u8()?;
                let scope = if presence & 0b01 != 0 { Some(read_command_str(reader)?) } else { None };
                Ok(ArtifactCommand::Supersede { scope, inputs: read_supersede_inputs::<P, Op>(reader)? })
            }
            18 => {
                let name = read_command_str(reader)?;
                Ok(ArtifactCommand::CreateAlternativeWithSupersede { name, inputs: read_supersede_inputs::<P, Op>(reader)? })
            }
            19 => {
                let transaction = protocol::TransactionRef { id: read_command_str(reader)?, tool: read_command_str(reader)? };
                Ok(ArtifactCommand::AppendTransaction { mutations: read_command_ops::<P, Op>(reader)?, transaction })
            }
            20 => Ok(ArtifactCommand::CommitTransaction { transaction_id: read_command_str(reader)? }),
            21 => Ok(ArtifactCommand::AbortTransaction { transaction_id: read_command_str(reader)? }),
            other => Err(protocol::ProtocolError::Malformed { what: "command variant", offset: 1, detail: format!("unknown command ordinal {other}") }.into()),
        }
    }
}
