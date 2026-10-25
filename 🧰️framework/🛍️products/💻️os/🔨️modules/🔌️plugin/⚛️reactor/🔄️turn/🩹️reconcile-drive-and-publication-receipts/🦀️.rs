//! 🩹️ Reconcile drive and publication receipts of the reactor turn: the more-work drive steps and budget, the reconcile drive, the live patch receipt, the original publication receipts and the exchange and app frame routing. See [`super`].

use super::*;

/// 🚚️ How many consecutive guest turns ONE worker-owned `MoreWork` drive may run before it crosses
/// back anyway — the host's own granted wall for this crossing, divided by what a turn measures.
///
/// The grant is the authority: `Budget::deadline_us` is the wall the host declared when it posted
/// this turn, so a drive that spends it is spending what it was given, and the host's watchdog ladder
/// (`SHARD_LIVENESS_POLICY`) is already written against it. At least one step is always admitted —
/// a grant smaller than one turn still has to make progress, exactly as the turn-patch page always
/// admits its first patch.
pub const fn more_work_drive_steps(grant_wall_ms: u64, guest_turn_cost_ms: u64) -> usize {
    if guest_turn_cost_ms == 0 {
        return 1;
    }
    let steps = grant_wall_ms / guest_turn_cost_ms;
    if steps < 1 {
        1
    } else {
        steps as usize
    }
}

/// 🚚️ The WALL one worker-owned drive may spend — the whole grant, never the grant rounded down to
/// whole measured turns.
///
/// [`more_work_drive_steps`] is the grant's derived EXPECTATION and its job is the coherence law; it is
/// not the runtime cap, because rounding the wall down to `steps × cost` makes the drive cross back
/// EMPTY with part of its own grant unspent whenever turns come in cheaper than they measured — the
/// exact cost this whole lane exists to remove.
///
/// 🐛️ What that rounding cost, measured: a live tool run on the puzzle 3d shell requests window
/// refreshes every 100–400 ms, its world-window patches cost ~30 ms of host intake each, and they
/// arrived **250–800 ms** after the request rather than once per request (ticket
/// `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS` `📓️status.md` phase 8). A reconcile needing more turns
/// than the rounded wall admits is one the drive hands back unfinished.
pub const fn more_work_drive_budget_ms(grant_wall_ms: u64, guest_turn_cost_ms: u64) -> u64 {
    if grant_wall_ms < guest_turn_cost_ms {
        guest_turn_cost_ms
    } else {
        grant_wall_ms
    }
}

/// 🔁️ Drives the retained-surface reconcile ladder inside ONE turn: steps whatever the guest can step
/// by itself, extracts every ready output a free publication slot will take, and answers how many
/// opportunities it actually spent.
///
/// Three stops, in order, and the first two are the correction this function exists for:
///
/// 1. **nothing to do** — neither drivable work nor (a ready output AND a free publication slot). The
///    loop used to test `has_publishable_work()` alone, so a ready output with a FULL pending
///    authority kept it spinning `drive_one` on a tracker with nothing to drive for the whole
///    1 024-step budget or the whole wall deadline, at 6 ms of guest per crossing;
/// 2. **no progress** — an opportunity that neither drove a producer/job nor extracted a ready page
///    cannot be followed by one that does, so the loop ends rather than burning the rest of the
///    budget re-asking;
/// 3. **the hold** — the turn's own wall deadline, re-read every 64 opportunities, unchanged.
///
/// See `📓️reactor-reconcile-spin-2026-09-14.md` §1-§3.
///
/// 🩹️ `take_ready_patch_into` already committed this output to its closing
/// lifecycle; there is no `return_ready_patch` any more, so losing this
/// capacity race simply drops the extracted page instead of re-queueing it.
#[doc(hidden)]
pub fn drive_reconcile_within(patches: &patches::PatchTracker, opportunities: usize, cx:&mut semio_framework_job::StepContext<'_>) -> Result<usize,semio_framework_job::OriginalTurnFailure> {
    let mut spent = 0usize;
    for _ in 0..opportunities {
        if cx.should_yield(){break}
        let drivable = patches.has_drivable_work();
        let can_publish = with_pending_patches(|pending| pending.borrow().has_capacity());
        if !drivable && !(can_publish && patches.has_publishable_work()) {
            break;
        }
        spent += 1;
        if drivable {
            patches.drive_one(cx)?;
        }
        let mut extracted = false;
        if can_publish {
            if let Some((key, generation)) = patches.ready_patch_key().map_err(reactor_close_fault)? {
                let mut target = None;
                if patches.take_ready_patch_into(key, generation, &mut target, semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES).map_err(reactor_close_fault)? {
                    extracted = true;
                    if let Some(patch) = target {
                        match with_pending_patches(|pending| pending.borrow_mut().push_reconcile(patch)) {
                            Ok(()) => {}
                            Err(_dropped) => {}
                        }
                    }
                }
            }
        }
        if !drivable && !extracted {
            break;
        }
    }
    Ok(spent)
}

/// 🔁️ Whether the retained-surface reconcile drive owes ANOTHER host turn, stated as three named
/// terms instead of "anything is non-empty".
///
/// 1. the tracker can advance by itself (a producer, a job, an admissible deferred surface, an
///    unadmitted surface, a closing instance, a pending output fault) — the guest's own work;
/// 2. a patch is queued for a later turn to hand out, or a retirement run was cut short by this
///    turn's wall deadline — also the guest's own work;
/// 3. a ready output exists AND there is a free publication slot for it — publishable NEXT turn.
///
/// What is deliberately absent is the fourth state the measurement found: a ready output with NO free
/// publication slot. That is HOST-blocked — the slot frees on an acknowledgement, the acknowledgement
/// arrives as its own event-carrying turn, and arming for it bought exactly one round trip per turn
/// that carried nothing (`📓️reactor-reconcile-spin-2026-09-14.md` §1). Same family as
/// `📓️host-reconcile-silence-2026-09-12.md`, one layer out.
#[doc(hidden)]
pub fn reconcile_arms_turn(patches: &patches::PatchTracker) -> bool {
    if patches.has_drivable_work() {
        return true;
    }
    with_pending_patches(|pending| {
        let pending = pending.borrow();
        pending.has_undelivered() || pending.has_retiring() || (pending.has_capacity() && patches.has_publishable_work())
    })
}

pub(super) fn live_patch_receipt<PA: semio_framework_plugin_reactor_app::PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, receipt: ActorUiPatchReceipt) -> bool {
    runtime.guest_lifetimes.borrow().get(receipt.lifetime.instance_id).is_some_and(|slot| slot.cell.is_live() && slot.cell.lifetime() == receipt.lifetime)
}

pub(super) fn original_turn_progress(before:semio_framework_value::RetainedCloneProgress,after:semio_framework_value::RetainedCloneProgress)->Result<semio_framework_value::RetainedCloneProgress,semio_framework::Fault>{let delta=|left:usize,right:usize|right.checked_sub(left).ok_or_else(||reactor_close_fault("original reactor recipient currency moved backwards"));Ok(semio_framework_value::RetainedCloneProgress{copied_items:delta(before.copied_items,after.copied_items)?,copied_bytes:delta(before.copied_bytes,after.copied_bytes)?,retained_capacity_bytes:delta(before.retained_capacity_bytes,after.retained_capacity_bytes)?,released_bytes:delta(before.released_bytes,after.released_bytes)?})}

/// 🧾️ Keeps exact producer grants and original deltas together when their receiver refuses authority.
struct OriginalPublicationReceipts{
 pub receiving:Option<semio_framework_plugin_runtime_ops::ops_ingress::OriginalPublicationReceiving>,
 pub constructor:Option<(semio_framework_plugin_operation::original_catalog_constructor::ArtifactStoreConstructorKind,semio_framework_plugin_operation::original_catalog_constructor::ArtifactStoreConstructorPhase,semio_framework_value::RetainedCloneGrant,semio_framework_value::RetainedCloneProgress)>,
 pub preparation:Option<(semio_framework_job::OperationId,semio_framework_value::RetainedCloneGrant,semio_framework_value::RetainedCloneProgress)>,
 pub history:Option<(semio_framework_value::RetainedCloneGrant,semio_framework_value::RetainedCloneProgress)>,
 pub tool:Option<(semio_framework_value::RetainedCloneGrant,semio_framework_value::RetainedCloneProgress)>,
 pub retirement:Option<(semio_framework_job::OperationId,semio_framework_value::RetainedCloneGrant,semio_framework_value::RetainedCloneProgress)>,
}

pub(crate) struct OriginalPublicationReceiptRefusal{pub receipts:OriginalPublicationReceipts,pub error:semio_framework_job::OriginalTurnFailure}

pub(super) struct RefusedPublicationOwner{pub instance:u32,pub operation:semio_framework_job::OperationId,pub generation:semio_framework_job::Generation,pub output:std::mem::ManuallyDrop<semio_framework_plugin_runtime_ops::ops_ingress::PluginExchangeOutput>,pub receipts:std::mem::ManuallyDrop<OriginalPublicationReceipts>}

impl Drop for RefusedPublicationOwner{fn drop(&mut self){assert!(std::thread::panicking(),"original refused publication reached Drop before explicit funded retirement");}}

/// 📥️ Collects each actual producer delta once into the same original recipient before semantic publication.
#[doc(hidden)]
pub fn collect_original_publication_receipts(output:&mut semio_framework_plugin_runtime_ops::ops_ingress::PluginExchangeOutput,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),OriginalPublicationReceiptRefusal>{
 use semio_framework_plugin_runtime_ops::ops_ingress::OriginalPublicationFamily as Family;
 let receipts=OriginalPublicationReceipts{receiving:output.original_receiving.take(),constructor:output.constructor_receipt.take(),preparation:output.preparation_receipt.take(),history:output.history_command_receipt.take(),tool:output.tool_run_receipt.take(),retirement:output.retirement_receipt.take()};let mut refusal=None;let mut receipt_refusal=None;
 let actual=[receipts.constructor.map(|(_,_,grant,progress)|(Family::Constructor,grant,progress)),receipts.preparation.map(|(_,grant,progress)|(Family::Preparation,grant,progress)),receipts.history.map(|(grant,progress)|(Family::History,grant,progress)),receipts.tool.map(|(grant,progress)|(Family::Tool,grant,progress)),receipts.retirement.map(|(_,grant,progress)|(Family::Retirement,grant,progress))];
 if let Some(receiving)=receipts.receiving{let unique=actual.iter().flatten().count()==1&&actual.iter().flatten().any(|(family,_,_)|Some(*family)==receiving.family);let posted=receiving.before.checked_add(receiving.posted);if !unique||receiving.recipient!=cx.original_recipient_identity()||posted.as_ref().ok()!=Some(&cx.retained_progress()){return Err(OriginalPublicationReceiptRefusal{receipts,error:semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original posted publication receipt lacks its unique family or actual recipient binding").into()})}}
 for (family,grant,progress) in actual.into_iter().flatten(){
  let remaining=receipts.receiving.map_or_else(||cx.retained_grant(),|receiving|receiving.grant);
  if (grant.maximum_items>remaining.maximum_items||grant.maximum_copy_bytes>remaining.maximum_copy_bytes||grant.maximum_capacity_bytes>remaining.maximum_capacity_bytes||grant.maximum_release_bytes>remaining.maximum_release_bytes||grant.maximum_depth>remaining.maximum_depth||!progress.fits(grant))&&refusal.is_none(){refusal=Some(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original publication receipt exceeds its producer or remaining caller authority").with_retained_progress(progress));}
  let unposted=if let Some(receiving)=receipts.receiving.filter(|receiving|receiving.family==Some(family)){let subtract=|whole:usize,posted:usize|whole.checked_sub(posted).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original publication full receipt is smaller than its already received effects").with_retained_progress(progress));match (||Ok::<_,semio_framework_value::ValueError>(semio_framework_value::RetainedCloneProgress{copied_items:subtract(progress.copied_items,receiving.posted.copied_items)?,copied_bytes:subtract(progress.copied_bytes,receiving.posted.copied_bytes)?,retained_capacity_bytes:subtract(progress.retained_capacity_bytes,receiving.posted.retained_capacity_bytes)?,released_bytes:subtract(progress.released_bytes,receiving.posted.released_bytes)?}))(){Ok(progress)=>progress,Err(error)=>return Err(OriginalPublicationReceiptRefusal{receipts,error:error.into()})}}else{progress};
  if let Err(error)=cx.consume_retained(unposted){if refusal.is_none(){refusal=Some(error)}else if receipt_refusal.is_none(){receipt_refusal=Some(error)}}
 }
 match refusal{Some(error)=>Err(OriginalPublicationReceiptRefusal{receipts,error:{let original=semio_framework_job::OriginalTurnFailure::from(error);semio_framework_job::OriginalTurnFailure{receipt_refusal,context_refusal:None,..original}}}),None=>Ok(())}
}

pub(super) fn retain_refused_publication(instance:u32,output:semio_framework_plugin_runtime_ops::ops_ingress::PluginExchangeOutput,refusal:OriginalPublicationReceiptRefusal,cx:&semio_framework_job::StepContext<'_>)->semio_framework_job::OriginalTurnFailure{
 let OriginalPublicationReceiptRefusal{receipts,error}=refusal;
 REFUSED_PUBLICATION.with(|owner|{let mut owner=owner.borrow_mut();assert!(owner.is_none(),"original Reactor refusal slot cannot replace an earlier original output");*owner=Some(RefusedPublicationOwner{instance,operation:cx.operation(),generation:cx.generation(),output:std::mem::ManuallyDrop::new(output),receipts:std::mem::ManuallyDrop::new(receipts)});});error
}

pub(super) fn route_exchange_output(instance:u32,mut output:semio_framework_plugin_runtime_ops::ops_ingress::PluginExchangeOutput,effects:&mut Vec<Effect>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
 if let Err(refusal)=collect_original_publication_receipts(&mut output,cx){
  return Err(retain_refused_publication(instance,output,refusal,cx));
 }

    for frame_bytes in output.frames {
        route_app_frame(instance, &frame_bytes, effects);
    }
    for one in &output.effects {
        if let Ok(effect) = decode_wire_effect(one) {
            push_admitted_effect(effects, instance, effect);
        }
    }
    for one in &output.events {
        if let Ok(event) = decode_wire_app_event(one) {
            effects.push(Effect::PublishEvent { topic: event.kind, payload: store::pack_rt::encode_wire_value(&event.payload) });
        }
    }
    Ok(())
}

pub(super) fn same_command_cursor(left: &semio_framework::kernel::CommandPageCursor, right: &semio_framework::kernel::CommandPageCursor) -> bool {
    left.owner == right.owner
        && left.generation == right.generation
        && left.command_index == right.command_index
        && left.command_count == right.command_count
        && left.instance == right.instance
        && left.seq == right.seq
        && left.kind == right.kind
        && left.page_count == right.page_count
        && left.item_count == right.item_count
        && left.metadata == right.metadata
}

/// 🔀️ `AppFrame::UiPatch` → a real `kernel::UiPatch` passthrough into `PENDING_PATCHES` (the wire
/// frame is already `UiPatch`-shaped field-for-field — channel v12/A4 — so this is a decode, not a
/// render); `AppFrame::Effects`/`Events` no longer exist as frames (`poll` decodes
/// `plugin_exchange`'s `PluginExchangeOutput.effects`/`.events` directly instead — see there);
/// `AppFrame::UiSnapshotEnd` has no consumer yet in this wave (patches apply incrementally, no
/// snapshot-boundary bookkeeping); everything else → `Effect::SendMessage` to the shell, matching
/// design-abi.md §2's table verbatim.
///
/// 🚫️async: E5 executor bridge (× 3) — `protocol::{decode,encode}_app_frame` (`📡️spr/**`, out
/// of `path_scope`) and `store::pack_rt::decode_wire_value` stay genuinely `async fn`; safe to
/// resolve synchronously for the same reason as this file's other WIT-boundary bridges.
///
/// 🎯️ `sdk-flip` (26/08/20): `protocol::AppFrame::UiPatch` still carries the PRE-flip shape
/// (`kind: String`, `ops` pack-encoding the old `kernel::PatchOp`) — its crate, `📡️spr/**`, is
/// FORBIDDEN to this packet, so the frame struct itself is untouched. `kind` is bound but
/// dropped (the new `UiPatch` has no such field); `ops` decodes into `UiPatchOp` on the
/// OPTIMISTIC assumption the sender re-encodes with the new op set too — genuinely stale
/// until whichever packet updates `📡️spr/🧵️channel` re-frames this variant to match (flagged
/// in `📓️terra-wit-flip-report.md`'s consumer inventory; not fixed here, out of `OWNS`).
#[expect(clippy::result_large_err, reason = "A rejected external patch is returned intact by its fixed pending-queue callback.")]
pub(super) fn route_app_frame(instance: u32, frame_bytes: &[u8], effects: &mut Vec<Effect>) {
    let Ok(frame) = ::semio_framework_async::poll::resolve_ready(protocol::decode_app_frame(frame_bytes)) else {
        return;
    };
    match frame {
        protocol::AppFrame::UiPatch { surface, kind: _, revision, base_revision, ops, .. } => {
            let Ok(ops_value) = store::pack_rt::decode_wire_value(&ops) else { return };
            let Ok(ops) = serde_json::from_value::<ui_contract::UiPatchOps>(ops_value.into()) else { return };
            let Ok(surface) = ui_contract::SurfaceId::try_from(surface) else { return };
            let patch = UiPatch { surface, base_revision: ui_contract::UiRevision(base_revision), revision: ui_contract::UiRevision(revision), ops };
            if let Err(patch) = with_pending_patches(|pending| pending.borrow_mut().push_external(patch)) {
                effects.push(Effect::SendMessage {
                    target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
                    payload: format!("patch-capacity-refused:{}:{}", patch.surface.0, patch.revision.0).into_bytes(),
                });
            }
        }
        protocol::AppFrame::UiSnapshotEnd { .. } => {}
        other => {
            let payload = ::semio_framework_async::poll::resolve_ready(protocol::encode_app_frame(&other));
            effects.push(Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload });
        }
    }
}
