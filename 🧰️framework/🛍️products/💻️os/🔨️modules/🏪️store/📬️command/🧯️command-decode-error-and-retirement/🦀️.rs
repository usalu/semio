//! 🧯️ Command decode error and retirement: the command decode error with its conversions, the operation and command retirement, the operation envelope serde and the scratch retirements. See [`super`].

use super::*;

/// 🧯️ Why a binary [`ArtifactCommand`] did not decode ([`ArtifactCommand::decode_command`]). Decoding is all-or-nothing:
/// every operation decoded before the failure retired through its technology's cold disposal.
#[derive(Clone, Debug, PartialEq)]
pub enum CommandDecodeError {
    /// 📐️ The command's own layout — its format, ordinal, a field, the framing of a list, or bytes past its end.
    Layout(protocol::ProtocolError),
    /// 🧩️ Entry `index` of the command's operation list or supersede inputs does not decode as one operation.
    Operation { index: usize, error: protocol::ProtocolError },
}

impl std::fmt::Display for CommandDecodeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Layout(error) => write!(formatter, "command layout: {error}"),
            Self::Operation { index, error } => write!(formatter, "command operation {index}: {error}"),
        }
    }
}

semio_framework_diagnostic::fault_from_error!(CommandDecodeError, semio_framework_diagnostic::FaultOrigin::Module, "module.store.command");

impl std::error::Error for CommandDecodeError {
    fn source(&self)->Option<&(dyn std::error::Error+'static)>{match self{Self::Layout(error)|Self::Operation{error,..}=>Some(error)}}
}

impl From<protocol::ProtocolError> for CommandDecodeError {
    fn from(error: protocol::ProtocolError) -> Self {
        Self::Layout(error)
    }
}

impl From<PackError> for CommandDecodeError {
    fn from(error: PackError) -> Self {
        Self::Layout(error.into())
    }
}

impl From<PackRefusal> for CommandDecodeError {
    fn from(error:PackRefusal)->Self{Self::Layout(error.into())}
}

/// 🧊️ Retires operations nobody will apply through their technology's cold disposal.
pub fn retire_operations<P, Op: Mutation<P>>(operations: impl IntoIterator<Item = Op>) {
    for operation in operations {
        Op::retire_cold(operation);
    }
}

/// 🪦️ Retires every operation a command owns — its operation list, its supersede replacements, its nested command.
pub fn retire_command<P, Op: Mutation<P>>(command: ArtifactCommand<Op>) {
    match command {
        ArtifactCommand::Apply { mutations, .. } | ArtifactCommand::ApplyInLane { mutations, .. } | ArtifactCommand::AppendTransaction { mutations, .. } => retire_operations::<P, Op>(mutations),
        ArtifactCommand::Supersede { inputs, .. } | ArtifactCommand::CreateAlternativeWithSupersede { inputs, .. } => retire_supersede_inputs::<P, Op>(inputs),
        ArtifactCommand::UndoWithPolicy { semantic_command: Some(nested), .. } => retire_command::<P, Op>(*nested),
        ArtifactCommand::UndoWithPolicy { semantic_command: None, .. }
        | ArtifactCommand::Undo
        | ArtifactCommand::Redo
        | ArtifactCommand::UndoInLane { .. }
        | ArtifactCommand::RedoInLane { .. }
        | ArtifactCommand::CommitCheckpoint { .. }
        | ArtifactCommand::CreateAlternative { .. }
        | ArtifactCommand::SwitchAlternative { .. }
        | ArtifactCommand::CheckoutCheckpoint { .. }
        | ArtifactCommand::IngestRemote { .. }
        | ArtifactCommand::PruneDrafts
        | ArtifactCommand::SetMergePolicy { .. }
        | ArtifactCommand::ResolveConflict { .. }
        | ArtifactCommand::CommitTransaction { .. }
        | ArtifactCommand::AbortTransaction { .. } => {}
    }
}

pub(super) mod operation_envelope_serde {
    /// 🧵️ `ToValue` twin of the former `serialize` — identical byte framing, wire shape is
    /// `DslValue::Array` of `DslValue::Number` (one per byte), matching `Vec<u8>`'s own
    /// `ToValue` codec exactly (so this is equivalent to `ToValue::to_value(&bytes)`).
    pub fn to_value(envelope: &protocol::MutationEnvelope) -> semio_framework_value::DslValue {
        let mut bytes = Vec::new();
        protocol::encode_envelope(envelope, &mut bytes);
        semio_framework_value::ToValue::to_value(&bytes)
    }

    /// 🧵️ `FromValue` twin of `to_value` above.
    pub fn from_value(value: semio_framework_value::DslValue) -> Result<protocol::MutationEnvelope, semio_framework_value::ValueError> {
        let bytes: Vec<u8> = semio_framework_value::FromValue::from_value(value)?;
        let mut pos = 0;
        protocol::decode_envelope(&bytes, &mut pos).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))
    }
}

/// 🫗️ Retires every typed replacement of a refused supersession's `inputs`, none of which was encoded.
pub fn retire_supersede_inputs<P, Mutation>(inputs: Vec<SupersedeInput<Mutation>>)
where
    Mutation: protocol::Mutation<P>,
{
    for replacement in inputs.into_iter().filter_map(|input| input.replacement) {
        protocol::Mutation::<P>::retire_cold(replacement);
    }
}

/// 🧊️ Cold-retires scratch operations (a rebased or discarded inverse): an operation may own a
/// fail-closed root, so a displaced copy never reaches `Drop` owning it.
pub fn retire_scratch_operations<P, Mutation>(operations: impl IntoIterator<Item = Mutation>)
where
    Mutation: protocol::Mutation<P>,
{
    for operation in operations {
        protocol::Mutation::<P>::retire_cold(operation);
    }
}

pub fn retire_scratch_edits<P, Mutation>(edits: impl IntoIterator<Item = protocol::Edit<Mutation>>)
where
    Mutation: protocol::Mutation<P>,
{
    for edit in edits {
        retire_scratch_operations::<P, Mutation>(edit.forwards);
        retire_scratch_operations::<P, Mutation>(edit.inverse);
    }
}
