//! 🔁️ Transaction commands: prepare, commit, rollback, undo and redo of one plugin exchange. See [`super`].

use super::*;

/// 🧾️ Prepares a transaction with its operations and origin.
pub(super) async fn arm_transaction_prepare<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::TransactionPrepare { seq, txn_id, mutation_id, payload, prepared_ops, origin, prepared_child_ops } = command else { unreachable!("arm_transaction_prepare is dispatched only for its own command") };
    let decoded_origin = if origin.is_empty() { Ok(None) } else { decode_wire_serialized::<protocol::MutationOrigin>(&origin).await.map(Some) };
    match decoded_origin {
        Err(fault) => push_app_fault(frames, Some(seq), fault).await,
        Ok(decoded_origin) => {
            let outcome = with_instances_mut(runtime, |list| {
                let mut instance = find_instance(list, instance_id)?;
                Ok(::semio_framework_async::poll::resolve_ready(instance.app.transaction_prepare(&txn_id, &mutation_id, &payload, &prepared_ops, &prepared_child_ops, decoded_origin)))
            });
            match outcome.await {
                Ok(outcome) => {
                    let mut foreign = Vec::with_capacity(outcome.foreign.len());
                    for step in outcome.foreign.iter() {
                        foreign.push(encode_wire_serialized(step));
                    }
                    let rejection = match outcome.rejection {
                        Some(fault) => encode_wire_serialized(&fault),
                        None => Vec::new(),
                    };
                    frames.push(protocol::AppFrame::TransactionPrepared { txn_id, foreign, rejection });
                    frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                }
                Err(fault) => push_app_fault(frames, Some(seq), fault).await,
            }
        }
    }
    Ok(())
}

/// 💾️ Commits a prepared transaction.
pub(super) async fn arm_transaction_commit<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
    frames: &mut Vec<protocol::AppFrame>,
    flags: &mut ExchangeFlags,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::TransactionCommit { seq, txn_id } = command else { unreachable!("arm_transaction_commit is dispatched only for its own command") };
    let meta = ActionMeta { actor: instance_actor(runtime, instance_id).await?.into(), instance_id, view_state: None };
    let outcome = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.transaction_commit(&txn_id, &meta, identity))
    });
    match outcome.await {
        Ok(edit_id) => {
            flags.mutated = true;
            frames.push(protocol::AppFrame::TransactionCommitted { txn_id, edit_id });
            frames.push(protocol::AppFrame::Done { in_reply_to: seq });
        }
        Err(fault) => push_app_fault(frames, Some(seq), fault).await,
    }
    Ok(())
}

/// ↩️ Rolls a prepared transaction back.
pub(super) async fn arm_transaction_rollback<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::TransactionRollback { seq, txn_id } = command else { unreachable!("arm_transaction_rollback is dispatched only for its own command") };
    let outcome = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.transaction_rollback(&txn_id))
    });
    match outcome.await {
        Ok(()) => {
            frames.push(protocol::AppFrame::TransactionRolledBack { txn_id });
            frames.push(protocol::AppFrame::Done { in_reply_to: seq });
        }
        Err(fault) => push_app_fault(frames, Some(seq), fault).await,
    }
    Ok(())
}

/// ⏪️ Undoes a transaction group.
pub(super) async fn arm_transaction_undo<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
    frames: &mut Vec<protocol::AppFrame>,
    flags: &mut ExchangeFlags,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::TransactionUndo { seq, group_id } = command else { unreachable!("arm_transaction_undo is dispatched only for its own command") };
    let outcome = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.transaction_undo(&group_id, identity))
    });
    match outcome.await {
        Ok(()) => {
            flags.mutated = true;
            frames.push(protocol::AppFrame::Done { in_reply_to: seq });
        }
        Err(fault) => push_app_fault(frames, Some(seq), fault).await,
    }
    Ok(())
}

/// ⏩️ Redoes a transaction group.
pub(super) async fn arm_transaction_redo<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
    frames: &mut Vec<protocol::AppFrame>,
    flags: &mut ExchangeFlags,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::TransactionRedo { seq, group_id } = command else { unreachable!("arm_transaction_redo is dispatched only for its own command") };
    let outcome = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.transaction_redo(&group_id, identity))
    });
    match outcome.await {
        Ok(()) => {
            flags.mutated = true;
            frames.push(protocol::AppFrame::Done { in_reply_to: seq });
        }
        Err(fault) => push_app_fault(frames, Some(seq), fault).await,
    }
    Ok(())
}
