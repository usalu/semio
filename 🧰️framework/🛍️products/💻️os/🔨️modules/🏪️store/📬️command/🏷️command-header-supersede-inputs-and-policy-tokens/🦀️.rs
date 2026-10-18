//! 🏷️ Command header, supersede inputs and policy tokens: the command header line, the supersede input lines with their print, parse, write and read, and the undo policy, history lane, merge policy and conflict resolution tokens and ordinals. See [`super`].

use super::*;

/// 🕹️ One structural `ArtifactCommand` line — `apply`/`undo`/`redo`/`commit-checkpoint`/
/// `create-alternative`/`switch-alternative`/`checkout` — the command-level twin of
/// `OpsHeaderLine`, re-derived on the same `dsl_schema` grammar engine. `Apply` carries no
/// operations here (those follow as 2-space-indented `Op::print_op` lines, exactly like
/// `print_edit_lines`); `Undo`'s `policy` is `None` for the plain `undo` command and `Some(token)`
/// for `UndoWithPolicy` (token = kebab of the `UndoPolicy` variant name), optionally followed by an
/// indented nested command block for `semantic-undo`/`compensating-action`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
pub enum CommandHeaderLine {
    /// ▶️ `transaction`/`tool` are the [`protocol::TransactionRef`] halves, present together or not at all.
    Apply {
        transaction: Option<String>,
        tool: Option<String>,
    },
    Undo {
        policy: Option<String>,
    },
    Redo,
    CommitCheckpoint {
        message: Option<String>,
        by: Vec<OpsAuthor>,
    },
    CreateAlternative {
        name: String,
    },
    SwitchAlternative {
        #[dsl(positional)]
        id: String,
    },
    Checkout {
        #[dsl(positional)]
        id: String,
    },
    PruneDrafts,
    /// 🛤️ Text twin of `ArtifactCommand::ApplyInLane` — `lane` is a kebab token
    /// (`history_lane_to_token`/`parse_history_lane_token`), matching `Undo.policy`'s convention.
    ApplyInLane {
        lane: String,
        transaction: Option<String>,
        tool: Option<String>,
    },
    UndoInLane {
        lane: String,
    },
    RedoInLane {
        lane: String,
    },
    /// ⚖️ Text twin of `ArtifactCommand::SetMergePolicy` — `policy` is a kebab token
    /// (`merge_policy_to_token`/`parse_merge_policy_token`), matching `Undo.policy`'s convention.
    SetMergePolicy {
        policy: String,
    },
    /// ⚔️ Text twin of `ArtifactCommand::ResolveConflict` — `resolution` is a kebab token
    /// (`conflict_resolution_to_token`/`parse_conflict_resolution_token`).
    ResolveConflict {
        #[dsl(positional)]
        conflict_id: String,
        resolution: String,
    },
    /// ✏️ Text twin of `ArtifactCommand::Supersede`; its inputs follow as [`SupersedeInputLine`]s.
    Supersede {
        scope: Option<String>,
    },
    /// 🌿️ Text twin of `ArtifactCommand::CreateAlternativeWithSupersede`; its inputs follow as [`SupersedeInputLine`]s.
    CreateAlternativeWithSupersede {
        name: String,
    },
    /// 🎞️ Text twin of `ArtifactCommand::AppendTransaction` — the [`protocol::TransactionRef`] halves; its operations follow
    /// as indented op lines.
    AppendTransaction {
        transaction: String,
        tool: String,
    },
    /// ✅️ Text twin of `ArtifactCommand::CommitTransaction`.
    CommitTransaction {
        #[dsl(positional)]
        id: String,
    },
    /// 🧨️ Text twin of `ArtifactCommand::AbortTransaction`.
    AbortTransaction {
        #[dsl(positional)]
        id: String,
    },
}

/// ✏️ One 2-space-indented input line under a `supersede` command: `replace <target>` followed by the replacement's
/// 4-space-indented `print_op` line, or `withdraw <target>`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
pub enum SupersedeInputLine {
    Replace {
        #[dsl(positional)]
        target: String,
    },
    Withdraw {
        #[dsl(positional)]
        target: String,
    },
}

impl OpText for SupersedeInputLine {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown supersede input line '{line}'")).to_string(),TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

/// 📤️ Prints supersede inputs as their indented input lines (see [`SupersedeInputLine`]).
pub fn print_supersede_inputs<Op: OpText>(out: &mut String, inputs: &[SupersedeInput<Op>]) -> Result<(), VcsError> {
    for input in inputs {
        let line = match &input.replacement {
            Some(_) => SupersedeInputLine::Replace { target: input.target.0.clone() },
            None => SupersedeInputLine::Withdraw { target: input.target.0.clone() },
        };
        out.push_str("  ");
        out.push_str(&line.print_op());
        out.push('\n');
        if let Some(replacement) = &input.replacement {
            let printed = replacement.print_op();
            if printed.contains('\n') {
                return Err(VcsError::Serialize("op-text print_op must not contain a newline".into()));
            }
            out.push_str("    ");
            out.push_str(&printed);
            out.push('\n');
        }
    }
    Ok(())
}

/// 📥️ Parses the indented input lines [`print_supersede_inputs`] prints; at least one input is required. All-or-nothing:
/// on any failure every replacement parsed so far retires through its technology's cold disposal.
pub fn parse_supersede_inputs<P, Op: OpText + Mutation<P>>(lines: &[&str]) -> Result<Vec<SupersedeInput<Op>>, TextError> {
    let mut inputs: Vec<SupersedeInput<Op>> = Vec::new();
    match parse_supersede_input_lines(lines, &mut inputs) {
        Ok(()) => Ok(inputs),
        Err(error) => {
            retire_supersede_inputs::<P, Op>(inputs);
            Err(error)
        }
    }
}

/// 🧺️ Parses supersede input lines into `inputs`, leaving every parsed input there on failure.
pub fn parse_supersede_input_lines<Op: OpText>(lines: &[&str], inputs: &mut Vec<SupersedeInput<Op>>) -> Result<(), TextError> {
    let mut awaiting_replacement = false;
    for raw in lines {
        if let Some(operation) = raw.strip_prefix("    ") {
            if !awaiting_replacement {
                return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("replacement operation without a replace line: {raw:?}")).to_string(),TextSpan::at(1,1)));
            }
            let input = inputs.last_mut().expect("a replace line precedes its replacement");
            input.replacement = Some(Op::parse_op(operation.trim())?);
            awaiting_replacement = false;
            continue;
        }
        if awaiting_replacement {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("a replace line needs its 4-space-indented replacement operation").to_string(),TextSpan::at(1,1)));
        }
        let Some(line) = raw.strip_prefix("  ") else {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected a 2-space-indented supersede input line, got: {raw:?}")).to_string(),TextSpan::at(1,1)));
        };
        match SupersedeInputLine::parse_op(line.trim())? {
            SupersedeInputLine::Replace { target } => {
                inputs.push(SupersedeInput { target: MutationId(target), replacement: None });
                awaiting_replacement = true;
            }
            SupersedeInputLine::Withdraw { target } => inputs.push(SupersedeInput { target: MutationId(target), replacement: None }),
        }
    }
    if awaiting_replacement {
        return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("a replace line needs its 4-space-indented replacement operation").to_string(),TextSpan::at(1,1)));
    }
    if inputs.is_empty() {
        return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("supersede requires at least one input line").to_string(),TextSpan::at(1,1)));
    }
    Ok(())
}

/// 🎯️ Binary twin of [`print_supersede_inputs`]: `count varint | (target str | 0 u8 | op len varint | op bytes | 1 u8)*`.
pub fn write_supersede_inputs<Op: OpBinary>(out: &mut Vec<u8>, inputs: &[SupersedeInput<Op>]) -> Result<(), protocol::ProtocolError> {
    semio_framework_os_pack::write_varint_u64(out, inputs.len() as u64);
    for input in inputs {
        write_command_str(out, &input.target.0);
        match &input.replacement {
            Some(replacement) => {
                out.push(0);
                let bytes = replacement.encode_op()?;
                semio_framework_os_pack::write_varint_u64(out, bytes.len() as u64);
                out.extend_from_slice(&bytes);
            }
            None => out.push(1),
        }
    }
    Ok(())
}

/// 🗳️ Reads a supersede command's inputs all-or-nothing: on any failure every replacement decoded so far retires through
/// its technology's cold disposal, so a refused command never drops a live operation.
pub fn read_supersede_inputs<P, Op: OpBinary + Mutation<P>>(reader: &mut semio_framework_os_pack::ByteReader<'_>) -> Result<Vec<SupersedeInput<Op>>, CommandDecodeError> {
    let count = reader.read_varint_u64()?;
    let mut inputs = Vec::with_capacity(count.min(reader.remaining() as u64) as usize);
    for index in 0..count as usize {
        match read_supersede_input(reader) {
            Ok(input) => inputs.push(input),
            Err(error) => {
                retire_supersede_inputs::<P, Op>(inputs);
                return Err(match error {
                    CommandDecodeError::Operation { error, .. } => CommandDecodeError::Operation { index, error },
                    layout => layout,
                });
            }
        }
    }
    Ok(inputs)
}

/// 🎫️ One supersede input: its target and its replacement operation or withdrawal tag.
pub fn read_supersede_input<Op: OpBinary>(reader: &mut semio_framework_os_pack::ByteReader<'_>) -> Result<SupersedeInput<Op>, CommandDecodeError> {
    let target = MutationId(read_command_str(reader)?);
    let replacement = match reader.read_u8()? {
        0 => Some(read_command_op(reader).map_err(|error| CommandDecodeError::Operation { index: 0, error })?),
        1 => None,
        tag => return Err(protocol::ProtocolError::Malformed { what: "supersede input tag", offset: reader.position() as u64, detail: format!("unknown supersede input tag {tag}") }.into()),
    };
    Ok(SupersedeInput { target, replacement })
}

/// 🎞️ Handcrafted OpText (P6).
impl OpText for CommandHeaderLine {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl OpBinary for CommandHeaderLine {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1.ordinary)();
        let body = semio_framework_os_pack::record::encode_record_body(&spec, &record, &PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        semio_framework_os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = semio_framework_os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let (record, _report) = semio_framework_os_pack::record::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let record_offset = reader.position() as u64;
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })
    }
}

pub async fn undo_policy_to_token(policy: UndoPolicy) -> &'static str {
    match policy {
        UndoPolicy::ExactBaseOnly => "exact-base-only",
        UndoPolicy::TransformAgainstConcurrent => "transform-against-concurrent",
        UndoPolicy::SemanticUndo => "semantic-undo",
        UndoPolicy::CompensatingAction => "compensating-action",
    }
}

pub async fn parse_undo_policy_token(token: &str) -> Result<UndoPolicy, TextError> {
    match token {
        "exact-base-only" => Ok(UndoPolicy::ExactBaseOnly),
        "transform-against-concurrent" => Ok(UndoPolicy::TransformAgainstConcurrent),
        "semantic-undo" => Ok(UndoPolicy::SemanticUndo),
        "compensating-action" => Ok(UndoPolicy::CompensatingAction),
        other => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown undo policy token {other:?}")).to_string(),TextSpan::at(1,1))),
    }
}

pub fn undo_policy_ordinal(policy: UndoPolicy) -> u8 {
    match policy {
        UndoPolicy::ExactBaseOnly => 0,
        UndoPolicy::TransformAgainstConcurrent => 1,
        UndoPolicy::SemanticUndo => 2,
        UndoPolicy::CompensatingAction => 3,
    }
}

pub fn undo_policy_from_ordinal(ordinal: u8) -> Result<UndoPolicy, protocol::ProtocolError> {
    match ordinal {
        0 => Ok(UndoPolicy::ExactBaseOnly),
        1 => Ok(UndoPolicy::TransformAgainstConcurrent),
        2 => Ok(UndoPolicy::SemanticUndo),
        3 => Ok(UndoPolicy::CompensatingAction),
        other => Err(protocol::ProtocolError::Malformed { what: "undo policy ordinal", offset: 0, detail: format!("unknown undo policy ordinal {other}") }),
    }
}

/// 🛤️ `HistoryLane`'s text token — the `ApplyInLane`/`UndoInLane`/
/// `RedoInLane` command lines' `lane=...` value, mirroring `undo_policy_to_token`'s convention.
pub async fn history_lane_to_token(lane: HistoryLane) -> &'static str {
    match lane {
        HistoryLane::Document => "document",
        HistoryLane::Interaction => "interaction",
    }
}

pub async fn parse_history_lane_token(token: &str) -> Result<HistoryLane, TextError> {
    match token {
        "document" => Ok(HistoryLane::Document),
        "interaction" => Ok(HistoryLane::Interaction),
        other => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown history lane token {other:?}")).to_string(),TextSpan::at(1,1))),
    }
}

/// 🛤️ `HistoryLane`'s binary ordinal, mirroring `undo_policy_ordinal`'s convention.
pub fn history_lane_ordinal(lane: HistoryLane) -> u8 {
    match lane {
        HistoryLane::Document => 0,
        HistoryLane::Interaction => 1,
    }
}

pub fn history_lane_from_ordinal(ordinal: u8) -> Result<HistoryLane, protocol::ProtocolError> {
    match ordinal {
        0 => Ok(HistoryLane::Document),
        1 => Ok(HistoryLane::Interaction),
        other => Err(protocol::ProtocolError::Malformed { what: "history lane ordinal", offset: 0, detail: format!("unknown history lane ordinal {other}") }),
    }
}

/// ⚖️ `protocol::MergePolicy`'s text token — the `SetMergePolicy` command line's
/// `policy=...` value, mirroring `undo_policy_to_token`'s convention.
pub async fn merge_policy_to_token(policy: protocol::MergePolicy) -> &'static str {
    match policy {
        protocol::MergePolicy::LaissezFaire => "laissez-faire",
        protocol::MergePolicy::Normal => "normal",
        protocol::MergePolicy::Vigilant => "vigilant",
    }
}

pub async fn parse_merge_policy_token(token: &str) -> Result<protocol::MergePolicy, TextError> {
    match token {
        "laissez-faire" => Ok(protocol::MergePolicy::LaissezFaire),
        "normal" => Ok(protocol::MergePolicy::Normal),
        "vigilant" => Ok(protocol::MergePolicy::Vigilant),
        other => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown merge policy token {other:?}")).to_string(),TextSpan::at(1,1))),
    }
}

/// ⚔️ `protocol::ConflictResolution`'s text token — the `ResolveConflict` command line's
/// `resolution=...` value, mirroring `undo_policy_to_token`'s convention.
pub async fn conflict_resolution_to_token(resolution: protocol::ConflictResolution) -> &'static str {
    match resolution {
        protocol::ConflictResolution::Accept => "accept",
        protocol::ConflictResolution::Discard => "discard",
    }
}

pub async fn parse_conflict_resolution_token(token: &str) -> Result<protocol::ConflictResolution, TextError> {
    match token {
        "accept" => Ok(protocol::ConflictResolution::Accept),
        "discard" => Ok(protocol::ConflictResolution::Discard),
        other => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown conflict resolution token {other:?}")).to_string(),TextSpan::at(1,1))),
    }
}

/// ⚔️ `protocol::ConflictResolution`'s binary ordinal, mirroring
/// `undo_policy_ordinal`'s convention.
pub fn conflict_resolution_ordinal(resolution: protocol::ConflictResolution) -> u8 {
    match resolution {
        protocol::ConflictResolution::Accept => 0,
        protocol::ConflictResolution::Discard => 1,
    }
}

pub fn conflict_resolution_from_ordinal(ordinal: u8) -> Result<protocol::ConflictResolution, protocol::ProtocolError> {
    match ordinal {
        0 => Ok(protocol::ConflictResolution::Accept),
        1 => Ok(protocol::ConflictResolution::Discard),
        other => Err(protocol::ProtocolError::Malformed { what: "conflict resolution ordinal", offset: 0, detail: format!("unknown conflict resolution ordinal {other}") }),
    }
}
