import { readFileSync } from "node:fs";
import { join } from "node:path";
import { toolJobArtifactEditMessageLedgerExact, toolJobArtifactEventSourcedMergeExact } from "../🟦️.ts";
import { TOOL_JOB_CHILD_RETIREMENT_INVENTORY, toolJobProductionSource, toolJobRustBlock } from "../../📋️inventory/🟦️.ts";
import { toolJobMountedDispatchOneTurnExact, toolJobRetainedDispatchSetup, toolJobTypedCaptureBody } from "../../📤️publication/🟦️.ts";

export function toolJobHistoryLedgerAdmissionExact(store: string, vcs: string): boolean {
  const checkpointPending = store.indexOf("let pending = uncommitted_edit_ids");
  const checkpointStart = checkpointPending < 0 ? -1 : store.lastIndexOf("ArtifactCommand::CommitCheckpoint { message, authors } => {", checkpointPending);
  const checkpointArm = checkpointStart < 0 ? "" : store.slice(checkpointStart, store.indexOf("ArtifactCommand::CreateAlternative { name } => {", checkpointPending));
  const pendingHash = checkpointArm.indexOf("content_addressed_checkpoint_id_with_pending_change(");
  const reservation = checkpointArm.indexOf("let change_reservation = self.reserve_change_history_slot()?");
  const reservationCommit = checkpointArm.indexOf("self.insert_reserved_checkpoint_history(checkpoint_reservation, checkpoint)?", reservation);
  return (
    vcs.includes("pub struct ArtifactHistoryReservation") &&
    vcs.includes("pub fn reserve_one(&mut self) -> Result<ArtifactHistoryReservation, ()>") &&
    vcs.includes("pub fn cancel_reservation(&mut self, reservation: ArtifactHistoryReservation)") &&
    vcs.includes("pub fn insert_reserved(&mut self, reservation: ArtifactHistoryReservation, value: T)") &&
    vcs.includes("reservation.authority != self.authority()") &&
    vcs.includes("fixed_history_reservation_returns_exact_rejected_owner_and_blocks_aba") &&
    vcs.includes("pub fn content_addressed_checkpoint_id_with_pending_change(") &&
    vcs.includes("pending_change_checkpoint_hash_is_byte_identical_before_history_reservation") &&
    store.includes("reservation: Option<ArtifactHistoryReservation>") &&
    store.includes("let reservation = match values.reserve_one()") &&
    store.includes("values.insert_reserved(reservation, value)") &&
    store.includes("values.cancel_reservation(reservation)") &&
    store.includes("fn reserve_owner_slots(&mut self, count: usize)") &&
    store.includes("fn push_owner_reserved(&mut self") &&
    store.includes("fn reserve_edit_history_slot(&mut self)") &&
    store.includes("fn insert_reserved_edit_history(&mut self") &&
    store.includes("displaced_owner_reservations_preserve_capacity_generation_and_interrupted_close") &&
    store.includes("rejected_history_mutation_and_metadata_owners_close_under_one_item_grants") &&
    pendingHash >= 0 &&
    reservation > pendingHash &&
    reservationCommit > reservation &&
    !checkpointArm.slice(reservation, reservationCommit).includes(".await")
  );
}

export function toolJobArtifactStoreStructuralOwnersExact(store: string, semio: string, causal: string, vcs: string): boolean {
  return (
    store.includes("struct ArtifactStoreConflictRetirement") &&
    store.includes("struct ArtifactStoreBackboneRetirement") &&
    store.includes("fn close_take_conflict_retirement(&mut self)") &&
    store.includes("fn close_take_tail_snapshot_retirement(&mut self)") &&
    store.includes("fn close_take_current_snapshot_retirement(&mut self)") &&
    store.includes("fn close_take_final_envelope_retirement(&mut self)") &&
    store.includes("fn close_take_backbone_retirement(&mut self)") &&
    store.includes("fn close_take_causal_owner_retirement(&mut self)") &&
    store.includes("const ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY: usize = 1_024") &&
    store.includes("struct ArtifactStoreDisplacedRetirements") &&
    store.includes("struct ArtifactStoreEnvelopeRetirement<P, Mutation>") &&
    store.includes("struct ArtifactStoreDocumentRootCommitAuthority<P, Mutation>") &&
    store.includes("fn prepare_document_root_commit(&mut self, additional_displaced_owners: usize)") &&
    store.includes("fn commit_document_roots_retained(") &&
    store.includes("fn replace_current_retained(&mut self, next: Arc<P>)") &&
    store.includes("fn replace_causal_dag_retained(&mut self, next: crate::os_spr::MutationDag)") &&
    store.includes("ArtifactStoreMessageLedgerRetirement::new(previous.edit_id, previous.messages)") &&
    store.includes("artifact store displaced-owner authority reached Drop before bounded retirement completed") &&
    store.includes("artifact store backbone retirement reached Drop before every exact URI, queue, message, channel, and byte owner was terminal-empty") &&
    causal.includes("pub fn terminal_is_empty(&self) -> bool") &&
    causal.includes("pub const MUTATION_DAG_CAPACITY: usize = 8_192") &&
    causal.includes("pub fn take_one_close_owner(&mut self) -> Option<MutationDagCloseOwner>") &&
    causal.includes("pub fn take_next_applied(&mut self) -> MutationDagAppliedStep") &&
    causal.includes("mutation dag reached Drop before every exact envelope and identity owner was cursor-retired") &&
    causal.includes("struct MutationDagFixedSlots") &&
    causal.includes("free: Box<[u16]>") &&
    causal.includes("generations: Box<[u32]>") &&
    causal.includes("struct MutationDagFixedSlotsIter") &&
    vcs.includes("pub const ARTIFACT_HISTORY_LEDGER_CAPACITY: usize = 64") &&
    vcs.includes("pub struct ArtifactHistoryLedger<T>") &&
    vcs.includes("slots: std::mem::ManuallyDrop<Vec<std::mem::MaybeUninit<ArtifactHistorySlot<T>>>>") &&
    vcs.includes("self.slot(index).generation.checked_add(1)") &&
    vcs.includes("pub fn try_push(&mut self, value: T) -> Result<ArtifactHistoryKey, T>") &&
    vcs.includes("pub fn remove_key(&mut self, key: ArtifactHistoryKey) -> Result<T, ArtifactHistoryKey>") &&
    vcs.includes("artifact history ledger reached Drop before every exact entry owner was retired") &&
    vcs.includes("pub edits: ArtifactHistoryLedger<Edit<Mutation>>") &&
    vcs.includes("pub changes: ArtifactHistoryLedger<Change>") &&
    vcs.includes("pub checkpoints: ArtifactHistoryLedger<Checkpoint>") &&
    vcs.includes("pub alternatives: ArtifactHistoryLedger<Alternative>") &&
    vcs.includes("fixed_history_ledger_preserves_order_capacity_and_aba_rejection") &&
    store.includes("values: Option<ArtifactHistoryLedger<T>>") &&
    store.includes("active_retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>") &&
    store.includes("struct ArtifactStoreVcsRetirement<P, Mutation>") &&
    toolJobHistoryLedgerAdmissionExact(store, vcs) &&
    !/#\[derive\([^\]]*(?:Clone|Deserialize)[^\]]*\)\]\s*#\[serde\([^\]]*\)\]\s*pub struct ArtifactVcs/.test(vcs) &&
    !causal.includes("ManuallyDrop<Vec<MutationEnvelope>>") &&
    !causal.includes("pub fn drain_applied_envelopes(") &&
    !causal.includes("pub fn ready(&self) -> Vec<") &&
    causal.includes("MutationDagInsertRejected { error: MutationDagError::Capacity, envelope }") &&
    !causal.includes("envelopes: std::collections::HashMap<String, MutationEnvelope>") &&
    !causal.includes("applied: std::collections::HashSet<String>") &&
    !store.includes("edit_messages: BTreeMap<String, Vec<crate::os_spr::MutationMessage>>") &&
    !store.includes("edit_messages: HashMap<String, Vec<crate::os_spr::MutationMessage>>") &&
    toolJobArtifactEditMessageLedgerExact(store) &&
    store.includes("pub edit_messages: ArtifactEditMessageLedger") &&
    store.includes("artifact edit-message fixed ledger reached Drop before every exact payload owner was cursor-retired") &&
    store.includes("ArtifactEditMessageTicket") &&
    store.includes("ARTIFACT_EDIT_MESSAGE_INDEX_TOMBSTONE") &&
    !store.includes("self.envelope.edit_messages.remove(") &&
    !store.includes("self.edit_messages.rebuild(") &&
    !store.includes("records.truncate(") &&
    !store.includes("self.applied_edit_ids.remove(") &&
    !store.includes("self.redo_edit_ids.remove(") &&
    !store.includes("self.redo_edit_ids.clear(") &&
    store.includes("fn take_string_at_retained(target: &mut std::mem::ManuallyDrop<Vec<String>>, position: usize) -> String") &&
    toolJobArtifactEventSourcedMergeExact(store) &&
    semio.includes("SemioStoreClosePhase::CausalIndex") &&
    semio.includes("SemioStoreClosePhase::DisplacedOwners") &&
    (semio.match(/impl RetireOwned for [a-z_]+_mutation::Semio[A-Za-z]+Mutation/g) ?? []).length === 18 &&
    !semio.includes("semio mutation owner has no generated field-by-field retirement cursor") &&
    store.includes("impl<P, Mutation> Drop for ArtifactStore<P, Mutation>") &&
    store.includes("artifact store reached Drop without its exact terminal-empty shallow-shell witness") &&
    store.includes("backbone: std::mem::ManuallyDrop<Option<Backbones>>") &&
    store.includes("dag: std::mem::ManuallyDrop<crate::os_spr::MutationDag>") &&
    store.includes("applied_edit_ids: std::mem::ManuallyDrop<Vec<String>>") &&
    store.includes("pending_report: std::mem::ManuallyDrop<PendingCommandReport>") &&
    !store.includes("*self.current =") &&
    !store.includes("*self.envelope = next") &&
    store.includes("fixed_edit_message_ledger_rejects_duplicate_and_capacity_plus_one_with_exact_retained_owners") &&
    store.includes("displaced_owner_reservations_preserve_capacity_generation_and_interrupted_close") &&
    store.includes("artifact_store_snapshot_roots_and_final_envelope_transfer_in_exact_close_order") &&
    causal.includes("fixed_causal_authority_rejects_capacity_plus_one_with_exact_identity_and_closes_one_owner_at_a_time") &&
    causal.includes("mutation_envelope_from_edit_falls_back_to_op_trait_and_structural_defaults_without_meta")
  );
}

export function toolJobChildRetirementInventoryExact(root: string, files: ReadonlyMap<string, string>): boolean {
  type InventoryVariant = { variant: string; dispatchKey: string; schema: string; snapshot: string; mutation: string };
  type InventoryCohort = { source: string; enum: string; ownerBindingStatus: string; variants: InventoryVariant[] };
  let inventory: { schemaVersion?: string; verdict?: string; productionCohorts?: InventoryCohort[]; counts?: Record<string, number> };
  try {
    inventory = JSON.parse(readFileSync(join(root, TOOL_JOB_CHILD_RETIREMENT_INVENTORY), "utf8"));
  } catch {
    return false;
  }
  const actual: InventoryCohort[] = [];
  for (const [sourcePath, raw] of files) {
    const source = toolJobProductionSource(raw);
    const invocation = /^\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*space_members!\s*\{/gm;
    let match: RegExpExecArray | null;
    while ((match = invocation.exec(source))) {
      const block = toolJobRustBlock(source, source.indexOf("{", match.index));
      if (!block) return false;
      const enumName = block.body.match(/^\s*pub enum\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{/m)?.[1];
      if (!enumName) return false;
      const variants: InventoryVariant[] = [];
      const row = /^\s*([A-Za-z_][A-Za-z0-9_]*)\("([^"]+)",\s*"([^"]+)"\)\s*=>\s*(?:[A-Za-z_][A-Za-z0-9_]*::)?ArtifactStore<([^,>]+(?:<[^>]+>)?),\s*([^>]+)>\s*,?$/gm;
      let variant: RegExpExecArray | null;
      while ((variant = row.exec(block.body))) variants.push({ variant: variant[1]!, dispatchKey: variant[2]!, schema: variant[3]!, snapshot: variant[4]!.trim(), mutation: variant[5]!.trim() });
      actual.push({ source: sourcePath, enum: enumName, ownerBindingStatus: "required", variants });
      invocation.lastIndex = block.end;
    }
  }
  const cohorts = inventory.productionCohorts ?? [];
  const canonical = (rows: InventoryCohort[]) =>
    JSON.stringify(
      rows
        .map((cohort) => ({ source: cohort.source, enum: cohort.enum, ownerBindingStatus: cohort.ownerBindingStatus, variants: cohort.variants.map((variant) => ({ ...variant })) }))
        .sort((left, right) => `${left.source}\0${left.enum}`.localeCompare(`${right.source}\0${right.enum}`)),
    );
  const variantCount = actual.reduce((sum, cohort) => sum + cohort.variants.length, 0);
  return (
    inventory.schemaVersion === "semio.phase8.child-snapshot-retirement-cohorts.v1" &&
    inventory.verdict === "fail-closed-pending-store-disassembly" &&
    canonical(actual) === canonical(cohorts) &&
    inventory.counts?.productionMacroCallsites === actual.length &&
    inventory.counts?.productionVariants === variantCount
  );
}

export function toolJobPeerCommitAuthorityExact(store: string, retirement: string): boolean {
  const start = store.indexOf("pub fn publish_peer_commit(");
  const body = start < 0 ? "" : toolJobRustBlock(store, store.indexOf("{", start))?.body ?? "";
  const recordStart = store.indexOf("pub struct PresencePeersCommit<P>");
  const record = recordStart < 0 ? undefined : toolJobRustBlock(store, store.indexOf("{", recordStart));
  const base = body.indexOf("!Arc::ptr_eq(&self.peers, &commit.base_root)");
  const factory = body.indexOf("!self.peer_retirement_factory.as_ref().is_some_and(|factory| Arc::ptr_eq(factory, &commit.factory))");
  const reject = body.indexOf("return Err(commit);");
  const swap = body.indexOf("std::mem::replace(&mut *self.peers, commit.root)");
  const retained = body.indexOf("*retirement.root = Some(previous);");
  return !!record && !/\bpub\b/.test(record.body) && record.body.includes("base_root: Arc<PresencePeersRoot<P>>")
    && record.body.includes("factory: Arc<dyn SnapshotRetirementFactory<P>>")
    && base >= 0 && factory >= 0 && reject > base && reject > factory && swap > reject && retained > swap && body.indexOf("drop(commit.base_root);") > retained
    && store.includes("PresencePeersPublication::new(&self.peers, factory.clone())")
    && store.includes('base_root: self.base_root.take().expect("peer candidate retains its exact immutable base")')
    && store.includes("self.base_root.is_none() && self.candidate.is_none()")
    && retirement.includes("base_root: std::mem::ManuallyDrop::new(Some(self.base_root))")
    && retirement.includes("if self.base_root.take().is_some()") && /self\.base_root\.is_none\(\)\s*&&\s*self\.local\.is_none\(\)/u.test(retirement);
}

export function toolJobPeerInteractionRootsExact(plugin: string, store: string, channel: string, retirement = ""): boolean {
  const setup = toolJobRetainedDispatchSetup(plugin);
  const capture = toolJobTypedCaptureBody(plugin);
  return !!(
    !!setup && toolJobMountedDispatchOneTurnExact(plugin) && toolJobPeerCommitAuthorityExact(store, retirement) &&
    plugin.includes("struct PeerPresenceRoot") &&
    plugin.includes("entries: [Option<std::sync::Arc<PeerPresenceEntry>>; PEER_PRESENCE_SLOTS]") &&
    plugin.includes("peer_presence: std::mem::ManuallyDrop<std::sync::Arc<PeerPresenceRoot>>") &&
    plugin.includes("peer_presence_retirements: ArtifactFixedRegistry<PeerPresenceRootRetirement>") &&
    plugin.includes("PeerPresenceRootRetirement::terminal_is_empty") &&
    plugin.includes("struct PeerRosterPublication") &&
    plugin.includes("fn reserve_presence_ingress(&mut self, seq: u64) -> Result<PresenceRosterAdmission, Fault>") &&
    plugin.includes("fn admit_presence_ingress(&mut self, admission: PresenceRosterAdmission, command: protocol::PresenceCommandCursor, now_ms: i64)") &&
    plugin.includes("fn push_presence_ingress(&mut self, generation: u64, page_index: u32, page: FixedCommandPage)") &&
    plugin.includes("fn reject_presence_ingress(&mut self, admission: PresenceRosterAdmission, command: FixedCommandPage, fault: Fault)") &&
    plugin.includes("PeerRosterPublication::<A>::rejected(admission, command, fault)") &&
    plugin.includes("peer_roster_reservations: [Option<(u64, u64)>; ARTIFACT_LIVE_OUTPUT_SLOTS]") &&
    plugin.includes("self.validate_peer_roster_publication(seq, generation, &cancel)") &&
    plugin.includes("resolve_ready(cancel.is_cancelled())") &&
    plugin.includes("self.peer_roster_outcomes.insert_admitted(generation, PresenceRosterOutcome { seq, fault })") &&
    plugin.includes("pub async fn plugin_reserve_presence_ingress<PA: PluginApp>") &&
    plugin.includes("pub async fn plugin_admit_reserved_presence<PA: PluginApp>") &&
    plugin.includes("protocol::PresenceCommandCursor::admit_page(admission.sequence(), own_color, item_count, page)") &&
    plugin.includes("pub async fn plugin_push_reserved_presence_page<PA: PluginApp>") &&
    plugin.includes("Result<(), (Fault, semio_framework::kernel::FixedCommandPage)>") &&
    plugin.includes("self.peer_roster_publications.insert_admitted(generation, PeerRosterPublication::<A>::rejected(admission, command, fault))") &&
    plugin.includes('AppCommand::Presence { .. } => unreachable!("Presence commands are admitted from their fixed header before whole-command decoding")') &&
    !plugin.includes("async fn adopt_presence") &&
    !plugin.includes("PeerPresenceRoot::from_peers") &&
    !plugin.includes("if let Ok(peer) = decoded") &&
    plugin.includes("peer_roster_publications: ArtifactFixedRegistry<PeerRosterPublication") &&
    store.includes("pub struct PresencePeersRoot<P>") &&
    store.includes("pub fn peers_root(&self) -> Arc<PresencePeersRoot<P>>") &&
    store.includes("pub struct PresencePeersCommit<P>") &&
    store.includes("pub fn publish_peer_commit(&mut self, commit: PresencePeersCommit<P>)") &&
    !store.includes("pub async fn adopt_peer(") &&
    !store.includes("pub async fn remove_peer(") &&
    !store.includes("pub async fn expire_peers(") &&
    !store.includes("pub async fn peers(&self) -> Vec<(&str, &P)>") &&
    channel.includes("pub const PRESENCE_ROSTER_MAXIMUM_ITEMS: usize = 64") &&
    channel.includes("pub const PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES: usize = 4_096") &&
    channel.includes("pub struct PresenceCommandCursor") &&
    channel.includes("pub fn admit_page(seq: u64, own_color: Option<u8>, item_count: u32, page: FixedCommandPage)") &&
    channel.includes("pub fn push_page(&mut self, page_index: u32, page: FixedCommandPage)") &&
    channel.includes("pub fn take_next(&mut self) -> Result<Option<FixedCommandPage>") &&
    channel.includes("pub fn close_release(&mut self, maximum_bytes: usize) -> (bool, usize)") &&
    !channel.includes("raw.truncate(raw.len() - released)") &&
    channel.includes('28 => return Err(malformed("channel presence command"') &&
    !channel.includes("async fn read_presence_roster") &&
    !channel.includes("#[derive(Clone, Debug, PartialEq)]\npub struct PresenceRosterWire") &&
    capture?.includes("peer_presence: std::sync::Arc::clone(&self.peer_presence)") &&
    capture?.includes("draft_snapshot: self.draft_store.snapshot_root()") &&
    capture?.includes("interaction_state: self.interaction_store.snapshot_root()") &&
    setup.includes("presence_local: Some(self.presence_store.local_read().map_err(Fault::from)?)") &&
    capture?.includes("presence_peers: self.presence_store.peers_root()") &&
    capture?.includes("transient: self.transient_store.current_root()") &&
    !setup.includes("self.presence_store.peers().await") &&
    !plugin.includes("let mut decoded: Vec<protocol::PresencePeer> = Vec::with_capacity(peers.len())")
  );
}

export function toolJobPagedIngressExact(kernel: string, reactor: string, plugin: string, host: string, channel: string, wit: string, mcp: string, run: string, wgpu: string): boolean {
  return (
    channel.includes("pub struct FixedCommandPage") &&
    channel.includes("bytes: [u8; COMMAND_PAGE_MAXIMUM_BYTES]") &&
    channel.includes("pages: std::collections::VecDeque<FixedCommandPage>") &&
    channel.includes("pub struct CommandEnvelopeSet") &&
    /commands\s*\.\s*try_reserve_exact\(COMMAND_BATCH_MAXIMUM_ITEMS\)/.test(channel) &&
    channel.includes("struct CommandBatchEntry") &&
    channel.includes("pub struct CommandDriverRegistry") &&
    channel.includes("CommandDriverRetentionState::Suspended") &&
    channel.includes("pub fn prepare_suspend") &&
    channel.includes("pub struct RejectedCommandBuildRegistry") &&
    !channel.includes("commands: std::collections::VecDeque<CommandEnvelope>") &&
    channel.includes("pub instance: u32") &&
    channel.includes("Result<Option<(CommandPageCursor, FixedCommandPage)>, crate::Fault>") &&
    channel.includes("pub struct PagedCommandReader") &&
    channel.includes("pub fn read_bounded_bytes(&mut self, maximum: usize)") &&
    !channel.includes("VecDeque<Vec<u8>>") &&
    !channel.includes("contiguous_if_single_page") &&
    !channel.includes("pub instance: PluginInstanceId") &&
    channel.includes("paged_generic_decoder_crosses_a_two_page_field_boundary_without_concatenation") &&
    channel.includes("paged_generic_decoder_faults_hostile_field_length_and_closes_exact_owner") &&
    channel.includes("current: [u8; COMMAND_PAGE_MAXIMUM_BYTES]") &&
    channel.includes("pub struct PagedAppCommandDecodeCursor") &&
    channel.includes("pub struct DecodedAppCommandOwner") &&
    channel.includes("PagedAppCommandDecodeState::RejectedFields") &&
    !channel.includes("current: Vec<u8>") &&
    !channel.includes("pub async fn decode_app_command") &&
    plugin.includes("pub enum PluginCommandIngress") &&
    plugin.includes("PluginCommandIngressStep::Pending") &&
    plugin.includes("PluginCommandIngressStep::TerminalFault") &&
    plugin.includes("command_terminal_fault") &&
    !plugin.includes("contiguous_if_single_page") &&
    !plugin.includes("protocol::decode_app_command") &&
    reactor.includes("CommandIngressOwner::GenericAssembly") &&
    reactor.includes("CommandIngressOwner::ClosingAssembly") &&
    reactor.includes("command: crate::plugin_runtime::PluginCommandIngress") &&
    reactor.includes("command.cancel(semio_framework::Fault::new") &&
    reactor.includes("output.command_terminal_fault.as_ref()") &&
    reactor.includes("PendingPresencePage") &&
    reactor.includes("presence_terminal_fault") &&
    reactor.includes("FixedCommandPage::try_from_array(bytes, page.length)") &&
    !reactor.includes("Vec<Vec<u8>>") &&
    host.includes("kernel_command_page_to_wit(cursor, bytes.as_slice())") &&
    wit.includes("record command-page-block") &&
    wit.includes("block-63: command-page-block") &&
    !wit.includes("bytes: list<u8>") &&
    [mcp, run, wgpu].every((source) => source.includes("CommandBatchDriver")) &&
    [mcp, run, wgpu].every((source) => source.includes("close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES)")) &&
    [mcp, run, wgpu].every((source) => source.includes("terminal_is_empty")) &&
    [mcp, run, wgpu].every((source) => source.includes("persistent_command_completion_port_ready")) &&
    mcp.includes("pending_exchanges") &&
    mcp.includes("PendingResponsePage") &&
    mcp.includes("RejectedCommandBuildRegistry<1>") &&
    !mcp.includes("response: Option<Result<store::AppFrame, Fault>>") &&
    run.includes("retained_command_closes") &&
    run.includes("RejectedCommandBuildRegistry<1>") &&
    wgpu.includes("retained_command_closes") &&
    wgpu.includes("queued_command_closes") &&
    wgpu.includes("KernelRequest::CloseRejectedCommandBuild") &&
    wgpu.includes("const KERNEL_REQUEST_QUEUE_CAPACITY: usize = 64") &&
    wgpu.includes("slots: [Option<(KernelRequest, Arc<ResponseSlot>)>; KERNEL_REQUEST_QUEUE_CAPACITY]") &&
    wgpu.includes("self.state.try_lock()") &&
    wgpu.includes("command_pages: usize") &&
    wgpu.includes("command_bytes: usize") &&
    wgpu.includes("COMMAND_MAXIMUM_PAGES") &&
    wgpu.includes("COMMAND_MAXIMUM_BYTES") &&
    wgpu.includes("fn shutdown_step(&self, maximum_bytes: usize)") &&
    wgpu.includes("yield_kernel_maintenance_turn") &&
    wgpu.includes("struct CreateAppRequestOwner") &&
    wgpu.includes("struct QueuedKernelEvent") &&
    wgpu.includes("KernelRequest::CloseRejectedEvents") &&
    wgpu.includes("struct KernelCloseSubmissionRegistry") &&
    wgpu.includes("slots: Mutex<[Option<(u32, u64, Arc<KernelCloseSubmission>)>; KERNEL_CLOSE_SUBMISSION_CAPACITY]>") &&
    wgpu.includes("Result<(), Arc<KernelCloseSubmission>>") &&
    wgpu.includes("pub(crate) struct KernelCloseHandle") &&
    wgpu.includes("#[must_use = \"kernel close ownership must be polled through terminal completion\"]") &&
    (wgpu.includes("pub(crate) fn begin_destroy_app(&self, instance: u32) -> KernelCloseHandle") ||
      wgpu.includes("pub(crate) fn begin_destroy_app(&self, instance: u32) -> Result<KernelCloseHandle, String>")) &&
    wgpu.includes("owner: Arc<KernelCloseSubmission>") &&
    wgpu.includes("KernelRequest::DestroyApp { owner: self.clone() }") &&
    wgpu.includes("owner.finish(KernelCloseStatus::Complete)") &&
    wgpu.includes("owner.finish(KernelCloseStatus::Fault)") &&
    wgpu.includes("ProgramBridge must retain and poll begin_destroy_app") &&
    !wgpu.includes("let _rejected_close = self.queue.try_push(KernelRequest::DestroyApp") &&
    !wgpu.includes("pending: Mutex<std::collections::VecDeque<(KernelRequest, Arc<ResponseSlot>)>>")
  );
}
