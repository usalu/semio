//! 📝️ Command print, parse and codec: the indented ops, the command print and parse, the transaction text and the command write and read over strings and operations. See [`super`].

use super::*;

/// 📤️ Prints every 2-space-indented `Op::print_op` line for one `apply`/`amend` body,
/// erroring exactly like `print_edit_lines` if any op prints a line containing a newline.
pub async fn print_indented_ops<Op: OpText>(out: &mut String, mutations: &[Op]) -> Result<(), VcsError> {
    for mutation in mutations {
        let printed = mutation.print_op();
        if printed.contains('\n') {
            return Err(VcsError::Serialize("op-text print_op must not contain a newline".into()));
        }
        out.push_str("  ");
        out.push_str(&printed);
        out.push('\n');
    }
    Ok(())
}

/// 📥️ Parses every already-2-space-indented body line of an `apply`/`amend` command as one
/// operation each — the command-level twin of `replay_ops`'s indented-op-line branch.
/// All-or-nothing: on any failure every operation parsed so far retires through its technology's cold disposal.
pub async fn parse_indented_ops<P, Op: OpText + Mutation<P>>(body_lines: &[&str]) -> Result<Vec<Op>, TextError> {
    let mut mutations = Vec::with_capacity(body_lines.len());
    for raw in body_lines {
        let parsed = match raw.starts_with("  ") {
            true => Op::parse_op(raw.trim()),
            false => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected a 2-space-indented op line, got: {raw:?}")).to_string(),TextSpan::at(1,1))),
        };
        match parsed {
            Ok(operation) => mutations.push(operation),
            Err(error) => {
                retire_operations::<P, Op>(mutations);
                return Err(error);
            }
        }
    }
    Ok(mutations)
}

/// 📥️ Strips exactly one 2-space indent level from every line, joining them back into a
/// standalone command text — used to recurse `parse_command` into a `semantic-undo`/
/// `compensating-action` nested command block.
pub async fn dedent_command_lines(lines: &[&str]) -> Result<String, TextError> {
    let mut out = String::new();
    for raw in lines {
        if !raw.starts_with("  ") {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected a 2-space-indented nested command line, got: {raw:?}")).to_string(),TextSpan::at(1,1)));
        }
        out.push_str(&raw[2..]);
        out.push('\n');
    }
    Ok(out)
}

/// 📤️ Prints a `ArtifactCommand` as its one-line-per-structural-field header, plus any
/// 2-space-indented operation lines (`Apply`) or a further-indented nested command
/// block (`UndoWithPolicy`'s `semantic_command`) — the maximum-token-efficient textual twin of
/// `encode_command`. `Author::avatar` is never printed, mirroring `OpsAuthor`'s `by=[...]` law.
pub async fn print_command<Op: OpText>(command: &ArtifactCommand<Op>) -> Result<String, VcsError> {
    let mut out = String::new();
    match command {
        ArtifactCommand::Apply { mutations, transaction } => {
            let (transaction, tool) = transaction_text(transaction.as_ref());
            out.push_str(&CommandHeaderLine::Apply { transaction, tool }.print_op());
            out.push('\n');
            print_indented_ops(&mut out, mutations).await?;
        }
        ArtifactCommand::Undo => {
            out.push_str(&CommandHeaderLine::Undo { policy: None }.print_op());
            out.push('\n');
        }
        ArtifactCommand::Redo => {
            out.push_str(&CommandHeaderLine::Redo.print_op());
            out.push('\n');
        }
        ArtifactCommand::UndoWithPolicy { policy, semantic_command } => {
            out.push_str(&CommandHeaderLine::Undo { policy: Some(undo_policy_to_token(*policy).await.to_string()) }.print_op());
            out.push('\n');
            if let Some(nested) = semantic_command {
                let nested_text = Box::pin(print_command(nested)).await?;
                for line in nested_text.lines() {
                    out.push_str("  ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
        }
        ArtifactCommand::CommitCheckpoint { message, authors } => {
            let header = CommandHeaderLine::CommitCheckpoint { message: message.clone(), by: authors.iter().map(OpsAuthor::from).collect() };
            out.push_str(&header.print_op());
            out.push('\n');
        }
        ArtifactCommand::CreateAlternative { name } => {
            out.push_str(&CommandHeaderLine::CreateAlternative { name: name.clone() }.print_op());
            out.push('\n');
        }
        ArtifactCommand::SwitchAlternative { alternative_id } => {
            out.push_str(&CommandHeaderLine::SwitchAlternative { id: alternative_id.clone() }.print_op());
            out.push('\n');
        }
        ArtifactCommand::CheckoutCheckpoint { checkpoint_id } => {
            out.push_str(&CommandHeaderLine::Checkout { id: checkpoint_id.clone() }.print_op());
            out.push('\n');
        }
        ArtifactCommand::ApplyInLane { mutations, lane, transaction } => {
            let (transaction, tool) = transaction_text(transaction.as_ref());
            out.push_str(&CommandHeaderLine::ApplyInLane { lane: history_lane_to_token(*lane).await.to_string(), transaction, tool }.print_op());
            out.push('\n');
            print_indented_ops(&mut out, mutations).await?;
        }
        ArtifactCommand::UndoInLane { lane } => {
            out.push_str(&CommandHeaderLine::UndoInLane { lane: history_lane_to_token(*lane).await.to_string() }.print_op());
            out.push('\n');
        }
        ArtifactCommand::RedoInLane { lane } => {
            out.push_str(&CommandHeaderLine::RedoInLane { lane: history_lane_to_token(*lane).await.to_string() }.print_op());
            out.push('\n');
        }
        ArtifactCommand::IngestRemote { .. } => {
            return Err(VcsError::Serialize("IngestRemote has no text command form".into()));
        }
        ArtifactCommand::PruneDrafts => {
            out.push_str(&CommandHeaderLine::PruneDrafts.print_op());
            out.push('\n');
        }
        ArtifactCommand::SetMergePolicy { policy } => {
            out.push_str(&CommandHeaderLine::SetMergePolicy { policy: merge_policy_to_token(*policy).await.to_string() }.print_op());
            out.push('\n');
        }
        ArtifactCommand::ResolveConflict { conflict_id, resolution } => {
            out.push_str(&CommandHeaderLine::ResolveConflict { conflict_id: conflict_id.clone(), resolution: conflict_resolution_to_token(*resolution).await.to_string() }.print_op());
            out.push('\n');
        }
        ArtifactCommand::Supersede { scope, inputs } => {
            out.push_str(&CommandHeaderLine::Supersede { scope: scope.clone() }.print_op());
            out.push('\n');
            print_supersede_inputs(&mut out, inputs)?;
        }
        ArtifactCommand::CreateAlternativeWithSupersede { name, inputs } => {
            out.push_str(&CommandHeaderLine::CreateAlternativeWithSupersede { name: name.clone() }.print_op());
            out.push('\n');
            print_supersede_inputs(&mut out, inputs)?;
        }
        ArtifactCommand::AppendTransaction { mutations, transaction } => {
            out.push_str(&CommandHeaderLine::AppendTransaction { transaction: transaction.id.clone(), tool: transaction.tool.clone() }.print_op());
            out.push('\n');
            print_indented_ops(&mut out, mutations).await?;
        }
        ArtifactCommand::CommitTransaction { transaction_id } => {
            out.push_str(&CommandHeaderLine::CommitTransaction { id: transaction_id.clone() }.print_op());
            out.push('\n');
        }
        ArtifactCommand::AbortTransaction { transaction_id } => {
            out.push_str(&CommandHeaderLine::AbortTransaction { id: transaction_id.clone() }.print_op());
            out.push('\n');
        }
    }
    Ok(out)
}

/// 📥️ Parses a `print_command`-produced (or hand-authored) command text back into a
/// `ArtifactCommand`. LAW: `parse_command(&print_command(c)?) == Ok(c)` for every `c`.
pub async fn parse_command<P, Op: OpText + Mutation<P>>(text: &str) -> Result<ArtifactCommand<Op>, TextError> {
    let all_lines: Vec<&str> = text.lines().collect();
    let mut header: Option<(u32, &str)> = None;
    let mut body_start = all_lines.len();
    for (index, raw) in all_lines.iter().enumerate() {
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        header = Some((index as u32 + 1, trimmed));
        body_start = index + 1;
        break;
    }
    let (header_line_no, header_text) = header.ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("empty command text").to_string(),TextSpan::at(1,1)))?;
    let header_line = CommandHeaderLine::parse_op(header_text).map_err(|error| TextError { span: TextSpan::at(header_line_no, error.span.column), ..error })?;
    let body_lines: Vec<&str> = all_lines[body_start..].iter().filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#')).copied().collect();

    match header_line {
        CommandHeaderLine::Apply { transaction, tool } => {
            let transaction = transaction_from_text(transaction, tool)?;
            let mutations = parse_indented_ops::<P, Op>(&body_lines).await?;
            if mutations.is_empty() {
                return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("apply requires at least one operation line").to_string(),TextSpan::at(1,1)));
            }
            Ok(ArtifactCommand::Apply { mutations, transaction })
        }
        CommandHeaderLine::Undo { policy: None } => Ok(ArtifactCommand::Undo),
        CommandHeaderLine::Undo { policy: Some(token) } => {
            let policy = parse_undo_policy_token(&token).await?;
            let semantic_command = if body_lines.is_empty() {
                None
            } else {
                let dedented = dedent_command_lines(&body_lines).await?;
                Some(Box::new(Box::pin(parse_command::<P, Op>(&dedented)).await?))
            };
            Ok(ArtifactCommand::UndoWithPolicy { policy, semantic_command })
        }
        CommandHeaderLine::Redo => Ok(ArtifactCommand::Redo),
        CommandHeaderLine::CommitCheckpoint { message, by } => Ok(ArtifactCommand::CommitCheckpoint { message, authors: by.into_iter().map(Author::from).collect() }),
        CommandHeaderLine::CreateAlternative { name } => Ok(ArtifactCommand::CreateAlternative { name }),
        CommandHeaderLine::SwitchAlternative { id } => Ok(ArtifactCommand::SwitchAlternative { alternative_id: id }),
        CommandHeaderLine::Checkout { id } => Ok(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: id }),
        CommandHeaderLine::PruneDrafts => Ok(ArtifactCommand::PruneDrafts),
        CommandHeaderLine::ApplyInLane { lane, transaction, tool } => {
            let transaction = transaction_from_text(transaction, tool)?;
            let lane = parse_history_lane_token(&lane).await?;
            let mutations = parse_indented_ops::<P, Op>(&body_lines).await?;
            if mutations.is_empty() {
                return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("apply-in-lane requires at least one operation line").to_string(),TextSpan::at(1,1)));
            }
            Ok(ArtifactCommand::ApplyInLane { mutations, lane, transaction })
        }
        CommandHeaderLine::UndoInLane { lane } => Ok(ArtifactCommand::UndoInLane { lane: parse_history_lane_token(&lane).await? }),
        CommandHeaderLine::RedoInLane { lane } => Ok(ArtifactCommand::RedoInLane { lane: parse_history_lane_token(&lane).await? }),
        CommandHeaderLine::SetMergePolicy { policy } => Ok(ArtifactCommand::SetMergePolicy { policy: parse_merge_policy_token(&policy).await? }),
        CommandHeaderLine::ResolveConflict { conflict_id, resolution } => Ok(ArtifactCommand::ResolveConflict { conflict_id, resolution: parse_conflict_resolution_token(&resolution).await? }),
        CommandHeaderLine::Supersede { scope } => Ok(ArtifactCommand::Supersede { scope, inputs: parse_supersede_inputs::<P, Op>(&body_lines)? }),
        CommandHeaderLine::CreateAlternativeWithSupersede { name } => Ok(ArtifactCommand::CreateAlternativeWithSupersede { name, inputs: parse_supersede_inputs::<P, Op>(&body_lines)? }),
        CommandHeaderLine::AppendTransaction { transaction, tool } => {
            let mutations = parse_indented_ops::<P, Op>(&body_lines).await?;
            if mutations.is_empty() {
                return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("append-transaction requires at least one operation line").to_string(),TextSpan::at(1,1)));
            }
            Ok(ArtifactCommand::AppendTransaction { mutations, transaction: protocol::TransactionRef { id: transaction, tool } })
        }
        CommandHeaderLine::CommitTransaction { id } => Ok(ArtifactCommand::CommitTransaction { transaction_id: id }),
        CommandHeaderLine::AbortTransaction { id } => Ok(ArtifactCommand::AbortTransaction { transaction_id: id }),
    }
}

/// 🧾️ A transaction reference as its two text attributes.
pub fn transaction_text(transaction: Option<&protocol::TransactionRef>) -> (Option<String>, Option<String>) {
    match transaction {
        Some(transaction) => (Some(transaction.id.clone()), Some(transaction.tool.clone())),
        None => (None, None),
    }
}

/// 🧾️ Inverse of [`transaction_text`]: both attributes or neither.
pub fn transaction_from_text(transaction: Option<String>, tool: Option<String>) -> Result<Option<protocol::TransactionRef>, TextError> {
    match (transaction, tool) {
        (Some(id), Some(tool)) => Ok(Some(protocol::TransactionRef { id, tool })),
        (None, None) => Ok(None),
        _ => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("a transaction needs both its id and its tool").to_string(),TextSpan::at(1,1))),
    }
}

/// 🧾️ Binary twin of [`transaction_text`]: the id and tool strings after a `0b10` presence bit; every other presence bit
/// is unassigned and refused.
pub fn write_command_transaction(out: &mut Vec<u8>, transaction: Option<&protocol::TransactionRef>) {
    if let Some(transaction) = transaction {
        write_command_str(out, &transaction.id);
        write_command_str(out, &transaction.tool);
    }
}

pub fn read_command_transaction(reader: &mut semio_framework_os_pack::ByteReader<'_>, presence: u8) -> Result<Option<protocol::TransactionRef>, protocol::ProtocolError> {
    if presence & !0b10 != 0 {
        return Err(protocol::ProtocolError::Malformed { what: "command presence", offset: 0, detail: format!("presence bits {presence:#04b} are unassigned beyond the 0b10 transaction bit") });
    }
    if presence & 0b10 == 0 {
        return Ok(None);
    }
    let id = read_command_str(reader)?;
    let tool = read_command_str(reader)?;
    Ok(Some(protocol::TransactionRef { id, tool }))
}

/// 🎯️ Format byte every encoded command starts with — matches `crate::os_dsl::op_rt::OP_BINARY_FORMAT`
/// (B-R6 "one wire convention": `format u8 | ordinal varint | record body`).
pub const COMMAND_BINARY_FORMAT: u8 = 1;

pub fn write_command_str(out: &mut Vec<u8>, s: &str) {
    semio_framework_os_pack::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}

pub fn read_command_str(reader: &mut semio_framework_os_pack::ByteReader<'_>) -> Result<String, protocol::ProtocolError> {
    let len = reader.read_varint_u64()?;
    let bytes = reader.read_bytes(len as usize)?;
    std::str::from_utf8(bytes).map(str::to_string).map_err(|error| protocol::ProtocolError::Malformed { what: "command string", offset: 0, detail: error.to_string() })
}

pub fn write_command_ops<Op: OpBinary>(out: &mut Vec<u8>, mutations: &[Op]) -> Result<(), protocol::ProtocolError> {
    semio_framework_os_pack::write_varint_u64(out, mutations.len() as u64);
    for mutation in mutations {
        let bytes = mutation.encode_op()?;
        semio_framework_os_pack::write_varint_u64(out, bytes.len() as u64);
        out.extend_from_slice(&bytes);
    }
    Ok(())
}

/// 🪣️ Reads a command's operation list all-or-nothing: on any failure every operation decoded so far retires through its
/// technology's cold disposal, so a refused list never drops a live operation.
pub fn read_command_ops<P, Op: OpBinary + Mutation<P>>(reader: &mut semio_framework_os_pack::ByteReader<'_>) -> Result<Vec<Op>, CommandDecodeError> {
    let count = reader.read_varint_u64()?;
    let mut mutations = Vec::with_capacity(count.min(reader.remaining() as u64) as usize);
    for index in 0..count as usize {
        match read_command_op(reader) {
            Ok(operation) => mutations.push(operation),
            Err(error) => {
                retire_operations::<P, Op>(mutations);
                return Err(CommandDecodeError::Operation { index, error });
            }
        }
    }
    Ok(mutations)
}

/// 🔹️ One length-prefixed operation of a command.
pub fn read_command_op<Op: OpBinary>(reader: &mut semio_framework_os_pack::ByteReader<'_>) -> Result<Op, protocol::ProtocolError> {
    let len = reader.read_varint_u64()?;
    Op::decode_op(reader.read_bytes(len as usize)?)
}
