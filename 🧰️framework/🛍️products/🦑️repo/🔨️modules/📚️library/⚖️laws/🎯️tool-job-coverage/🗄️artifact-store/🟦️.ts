/** 🧭️ Tool-job coverage: artifact store, envelope and retained-ownership laws per app.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { OS_STORE_ROOT_SOURCES, policyRustProductionSource, type PolicyRustSourceEvidence, policyRustTestSource } from "../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import { TOOL_JOB_CHILD_RETIREMENT_INVENTORY, toolJobProductionSource, toolJobRustBlock } from "../📋️inventory/🟦️.ts";
import { toolJobMountedDispatchOneTurnExact, toolJobRetainedDispatchSetup, toolJobTypedCaptureBody } from "../📤️publication/🟦️.ts";

export function toolJobImmutableOperationRootsExact(store: string): boolean {
  const blockOf = (needle: string) => {
    const start = store.indexOf(needle);
    const open = start < 0 ? -1 : store.indexOf("{", start);
    return open < 0 ? undefined : toolJobRustBlock(store, open);
  };
  const local = blockOf("pub fn local_read(&self) -> Result<SnapshotRead<P>, String>");
  const transient = blockOf("pub fn current_root(&self) -> Arc<P>");
  const snapshot = blockOf("pub fn snapshot_root(&self) -> Arc<P>");
  const owner = blockOf("pub fn snapshot_owner(&self) -> Arc<P>");
  const group = blockOf("fn durable_group_read_root(&self)");
  const groupRoot = owner?.body.trim() === 'self.durable_group_read_root().map_or_else(|| Arc::clone(&*self.current), |root| Arc::clone(root.current.as_ref().expect("unadopted durable group root retains its current snapshot")))' && group?.body.trim() === "self.durable_group_root.as_ref().filter(|root| root.visibility.committed() && !root.adopted)";
  const arcClone = (body: string | undefined) => body?.trim() === "self.current.clone()" || body?.trim() === "Arc::clone(&*self.current)";
  return (
    !!local &&
    !!transient &&
    !!snapshot &&
    store.includes("local: std::mem::ManuallyDrop<Arc<P>>") &&
    store.includes("current: Arc<P>") &&
    local.body.includes("self.local_reads.try_issue(owner.clone())") &&
    local.body.includes("SnapshotRead::new(owner, lease)") &&
    local.body.includes("self.close_started || self.local_retirement_factory.is_none()") &&
    !store.includes("pub fn local_root(") &&
    transient.body.trim() === "self.current.clone()" &&
    (arcClone(snapshot.body) || (snapshot.body.trim() === "self.snapshot_owner()" && (arcClone(owner?.body) || groupRoot))) &&
    store.includes("self.local = Arc::new(candidate)") &&
    store.includes("self.current = Arc::new(candidate)") &&
    store.includes("pub fn content_revision_now(&self) -> [u8; 32]") &&
    !local.body.includes("Arc::new") &&
    !transient.body.includes("Arc::new") &&
    !snapshot.body.includes("Arc::new")
  );
}

export function toolJobChildContentRootExact(store: string, plugin: string): boolean {
  const start = plugin.indexOf("pub struct ChildContentView");
  const open = start < 0 ? -1 : plugin.indexOf("{", start);
  const view = open < 0 ? undefined : toolJobRustBlock(plugin, open);
  const dispatchStart = plugin.lastIndexOf("async fn dispatch_typed_command_inner");
  const dispatchOpen = dispatchStart < 0 ? -1 : plugin.indexOf("{", dispatchStart);
  const dispatch = dispatchOpen < 0 ? undefined : toolJobRustBlock(plugin, dispatchOpen);
  return (
    !!view &&
    !!dispatch &&
    view.body.includes("root: Option<std::sync::Arc<ChildContentRoot>>") &&
    !view.body.includes("HashMap") &&
    plugin.includes("const CHILD_CONTENT_SLOTS: usize = 1_024") &&
    plugin.includes("const CHILD_CONTENT_PAGE_SLOTS: usize = 32") &&
    plugin.includes("pages: [Option<std::sync::Arc<ChildContentPage>>; CHILD_CONTENT_PAGES]") &&
    plugin.includes("entries: [Option<std::sync::Arc<ChildContentEntry>>; CHILD_CONTENT_PAGE_SLOTS]") &&
    plugin.includes("slot.len() > CHILD_CONTENT_ID_BYTES || child_id.len() > CHILD_CONTENT_ID_BYTES") &&
    plugin.includes("for offset in 0..CHILD_CONTENT_SLOTS") &&
    plugin.includes("let mut page = next.pages[page_index].as_deref().cloned().unwrap_or_default()") &&
    plugin.includes("child_content_root: std::mem::ManuallyDrop<ChildContentView>") &&
    plugin.includes("child_content_retirements: ArtifactFixedRegistry<ChildContentRetirement>") &&
    plugin.includes("struct ChildMemberRegistry<M>") &&
    plugin.includes("slots: Box<[std::mem::MaybeUninit<ChildMemberEntry<M>>]>") &&
    plugin.includes("reserved: [u64; CHILD_CONTENT_SLOTS / 64]") &&
    plugin.includes("generations: [u64; CHILD_CONTENT_SLOTS]") &&
    plugin.includes("fn admit(&mut self, key: &(String, String)) -> Result<ChildMemberAdmission, Fault>") &&
    plugin.includes("fn cancel_admission(&mut self, admission: &ChildMemberAdmission) -> bool") &&
    plugin.includes("pub(crate) children: ChildMemberRegistry<M>") &&
    plugin.includes("close_child_member: std::mem::ManuallyDrop<Option<ChildMemberRetirement<M>>>") &&
    plugin.includes("self.children.take_at(index)") &&
    plugin.includes("fixed child-member registry reached Drop before every exact member was transferred to bounded retirement") &&
    !plugin.includes("pub(crate) children: HashMap<(String, String), (ArtifactDialect, M)>") &&
    plugin.includes("fn admit_child_content_publication_span(&self, count: usize)") &&
    plugin.includes("async fn publish_child_content_member") &&
    plugin.includes("self.publish_child_content_member(publication_generation") &&
    store.includes("pub trait ErasedSnapshotRetirement: Send") &&
    store.includes("fn terminal_is_empty(&self) -> bool;") &&
    store.includes("pub trait SnapshotRetirementFactory<P>: Send + Sync") &&
    store.includes("const SNAPSHOT_READ_LEASE_CAPACITY: usize = 1_024") &&
    store.includes("slots: Box<[std::mem::MaybeUninit<SnapshotReadLeaseSlot>; SNAPSHOT_READ_LEASE_CAPACITY]>") &&
    store.includes("fn try_issue<T: Send + Sync + 'static>(self: &Arc<Self>, owner: Arc<T>) -> Result<SnapshotReadLease, Arc<T>>") &&
    store.includes("pub struct SnapshotReadRef<'a, T: ?Sized>") &&
    store.includes("fn take_returned_snapshot_read_retirement(&mut self) -> Result<Option<Box<dyn ErasedSnapshotRetirement>>, String>;") &&
    store.includes("fn snapshot_read_leases_terminal_is_empty(&self) -> bool;") &&
    store.includes("fn close_owned_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String>;") &&
    store.includes("fn close_owned_terminal_is_empty(&self) -> bool;") &&
    !store.includes("#[derive(Clone)]\npub struct SnapshotRead") &&
    !store.includes("#[derive(Clone)]\npub struct ErasedSnapshotRead") &&
    /pub trait SpaceMember[\s\S]*?fn retire_snapshot_read_erased\(&mut self, snapshot: ErasedSnapshotRead\) -> Result<Box<dyn ErasedSnapshotRetirement>, SnapshotRetirementRejected>;/.test(store) &&
    store.includes("return Err(SnapshotRetirementRejected { snapshot,") &&
    plugin.includes("pending: std::mem::ManuallyDrop<Option<ChildContentEntry>>") &&
    plugin.includes("active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>") &&
    plugin.includes("std::sync::Arc::try_unwrap(root)") &&
    plugin.includes("std::sync::Arc::try_unwrap(entry)") &&
    plugin.includes("match member.retire_snapshot_read_erased(snapshot)") &&
    plugin.includes("snapshot: rejected.snapshot") &&
    plugin.includes("if !active.terminal_is_empty()") &&
    plugin.includes("take_returned_snapshot_read_retirement") &&
    plugin.includes("snapshot_read_leases_terminal_is_empty") &&
    plugin.includes("self.child_content_retirements.next_id_from(self.maintenance_child_root_cursor)") &&
    plugin.includes("self.child_content_retirements.next_id_from(self.close_child_root_cursor)") &&
    plugin.includes("close_child_root_detached") &&
    !plugin.includes('FaultCode::new("interactive-job.close-child-root-disposer-missing")') &&
    plugin.includes("child_content_publication_path_copies_fixed_pages_and_command_capture_retains_one_root") &&
    plugin.includes("child_snapshot_retirement_rejection_preserves_exact_erased_owner") &&
    plugin.includes("child_root_maintenance_requires_terminal_empty_before_reclaim") &&
    plugin.includes("fixed_child_member_registry_admits_exact_capacity_rejects_plus_one_and_cursor_detaches_every_owner") &&
    plugin.includes("fixed_child_member_registry_resolves_hash_collisions_without_replacement") &&
    plugin.includes("stale_child_member_admission_cannot_cancel_a_reused_slot_generation") &&
    plugin.includes("incomplete_child_member_registry_drop_faults_in_release_instead_of_destroying_nested_owners") &&
    !plugin.includes("entry.snapshot.clone()") &&
    dispatch.body.includes("let children = std::sync::Arc::new(ChildContentView::clone(&self.child_content_root)") &&
    !dispatch.body.includes("ChildContentView::new(&self.children)")
  );
}

/** 🔀️ Remote ingest is event sourced: replicas fold operation and transition envelopes into one causal log,
 * a refusal retires its candidate causal graph, a quarantine Accept re-folds the log — never a snapshot merge. */
export function toolJobArtifactEventSourcedMergeExact(store: string): boolean {
  return (
    store.includes("pub fn fold_event_log<P, Mutation>(") &&
    store.includes("async fn admit_remote_transitions(") &&
    store.includes("fn refuse_with_candidate_dag<T>(&mut self, candidate_dag: crate::os_spr::MutationDag, error: VcsError)") &&
    store.includes("async fn accept_quarantined_events(") &&
    store.includes("Genesis {") &&
    !store.includes("merge_remote_snapshot") &&
    !store.includes("ResolutionCandidate")
  );
}

export function toolJobArtifactEditMessageLedgerExact(store: string): boolean {
  return (
    store.includes("struct ArtifactEditMessageLedger") &&
    store.includes("buckets: Box<[u16]>") &&
    store.includes("generations: Box<[u64]>") &&
    store.includes("self.generations[slot].checked_add(1)") &&
    store.includes("artifact edit-message ledger contains a duplicate identity") &&
    store.includes("const ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES: usize = 4_096") &&
    store.includes("const ARTIFACT_EDIT_MESSAGE_LEDGER_BYTES: usize") &&
    store.includes("struct ArtifactEditMessageLedgerRejected") &&
    store.includes("entries: std::mem::ManuallyDrop<Vec<crate::os_spr::EditMessages>>") &&
    store.includes("impl ErasedSnapshotRetirement for ArtifactEditMessageLedgerRejected") &&
    store.includes("rejected artifact edit-message authority reached Drop before every exact payload owner was cursor-retired") &&
    store.includes("artifact edit-message fixed ledger reached Drop before every exact payload owner was cursor-retired") &&
    !store.includes("self.generations[slot].wrapping_add(1)") &&
    !store.includes("struct ArtifactEditMessageLedgerRejected {\n    entries: Vec<")
  );
}

export function toolJobArtifactEnvelopeOwnedCodecExact(store: string, rust: Map<string, string>): boolean {
  const artifactEnvelopeAliases = new Set<string>(["ArtifactEnvelope"]);
  for (const source of rust.values()) {
    for (const match of source.matchAll(/\btype\s+([A-Za-z0-9_]+Envelope)\s*=\s*(?:store::|crate::os_store::)?ArtifactEnvelope\b/g)) artifactEnvelopeAliases.add(match[1]!);
  }
  const envelopeType = [...artifactEnvelopeAliases].map((name) => name.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join("|");
  const directEnvelopePattern = new RegExp(
    `(?:let\\s+[A-Za-z0-9_]+\\s*:\\s*(?:${envelopeType})(?:<[^;\\n]+>)?[^;\\n]*=\\s*serde_json::from_(?:str|slice|value)|serde_json::from_(?:str|slice|value)::<[^;\\n>]*(?:${envelopeType}))`,
  );
  const directEnvelopeSerde = [...rust.entries()].filter(
    ([file, source]) =>
      !file.includes("/🧪️tests/") &&
      !(OS_STORE_ROOT_SOURCES as readonly string[]).includes(file) &&
      directEnvelopePattern.test(source),
  );
  const placeholderEnvelopeIngress = [...rust.entries()].filter(
    ([file, source]) => !(OS_STORE_ROOT_SOURCES as readonly string[]).includes(file) && source.includes("reject_whole_buffer_artifact_envelope_ingress"),
  );
  return (
    store.includes("pub struct OwnedSchemaDecodePages") &&
    store.includes("slots: Box<[std::mem::MaybeUninit<OwnedSchemaDecodePage>]>") &&
    store.includes("pub fn admit_page(&mut self, page: OwnedSchemaDecodePage) -> Result<(), (OwnedSchemaDecodeAdmissionFault, OwnedSchemaDecodePage)>") &&
    store.includes("pub struct OwnedSchemaTokenCursor") &&
    store.includes("pub fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> OwnedSchemaTokenStep") &&
    store.includes("pub struct OwnedSchemaRecordSpec") &&
    store.includes("pub struct OwnedSchemaStringAuthority<const MAXIMUM_BYTES: usize>") &&
    store.includes("schema-json.stale-string-authority") &&
    store.includes("schema-json.string-byte-capacity") &&
    store.includes("fn accept_field_token(") &&
    store.includes("fn finish_record(&mut self, cx: &mut semio_framework_job::StepContext<'_>)") &&
    store.includes("fn terminal_is_empty(&self) -> bool;") &&
    store.includes("pub trait ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>: Send + Sync") &&
    store.includes("pub trait ArtifactEnvelopeVcsFieldAuthority<P, Mutation>: Send") &&
    store.includes("pub trait ArtifactEnvelopeSnapshotFieldAuthority<P>: Send") &&
    store.includes("pub trait ArtifactEnvelopeMutationFieldAuthority<Mutation>: Send") &&
    store.includes("pub trait ArtifactEnvelopeSprConflictAuthority: Send") &&
    store.includes("pub struct ArtifactEnvelopeOwners<P, Mutation>") &&
    store.includes("owners: std::mem::ManuallyDrop<ArtifactEnvelopeOwners<P, Mutation>>") &&
    store.includes("artifact envelope terminal shell reached Drop before its app-owned bounded retirement authority detached every nested owner") &&
    store.includes("pub trait ArtifactEnvelopeCompletedRecordTarget<P, Mutation>") &&
    store.includes("fn try_publish_to(&mut self, target: &mut dyn ArtifactEnvelopeCompletedRecordTarget<P, Mutation>) -> bool") &&
    store.includes("pub fn try_request_close(&self, ticket: ArtifactEnvelopeCompletedRecordTicket)") &&
    store.includes("fn publish_vcs_reserved(") &&
    store.includes("fn publish_edit_messages_reserved(") &&
    store.includes("fn publish_conflicts_reserved(") &&
    store.includes("schema-json.unknown-field") &&
    store.includes("schema-json.duplicate-field") &&
    store.includes("schema-json.invalid-utf8") &&
    store.includes("pub fn close_step(&mut self, maximum_pages: usize) -> SnapshotRetirementStep") &&
    !store.includes("struct OwnedSchemaDecodePages {\n    pages: Vec<Vec<u8>>") &&
    !store.includes("ArtifactEnvelopeDecodeAuthority::new(bytes: Vec<u8>") &&
    !store.includes("serde_json::from_slice(token_bytes)") &&
    !store.includes("fn take_envelope(&mut self) -> Option<ArtifactEnvelope") &&
    !/#\[derive\([^\]]*Clone[^\]]*\)\]\s*#\[serde\([^\]]*\)\]\s*pub struct ArtifactEnvelope/.test(store) &&
    store.includes("pub struct ArtifactEnvelopeDecodeAuthority<P, Mutation>") &&
    store.includes("struct ArtifactEnvelopeDecodePage") &&
    store.includes("impl<P, Mutation> semio_framework_job::InteractiveJob for ArtifactEnvelopeDecodeAuthority<P, Mutation>") &&
    store.includes("struct ArtifactEnvelopeDecodeRejected") &&
    store.includes("impl<P, Mutation> ErasedSnapshotRetirement for ArtifactEnvelopeDecodeRejected<P, Mutation>") &&
    store.includes("artifact envelope decode authority reached Drop before terminal publication or retained close") &&
    store.includes("artifact envelope decode rejection reached Drop before every exact page owner was cursor-retired") &&
    toolJobArtifactEnvelopeRejectionTransferExact(store) &&
    store.includes("ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES: usize = 4_096") &&
    store.includes("ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES") &&
    !/#\[derive\([^\]]*Deserialize[^\]]*\)\]\s*#\[serde\([^\]]*\)\]\s*pub struct ArtifactEnvelope</.test(store) &&
    directEnvelopeSerde.length === 0 &&
    placeholderEnvelopeIngress.length === 0
  );
}

export function toolJobArtifactEnvelopeRejectionTransferExact(storeEvidence: string | PolicyRustSourceEvidence): boolean {
  const store = policyRustProductionSource(storeEvidence);
  const tests = policyRustTestSource(storeEvidence);
  const body = (source: string, signature: string, start = 0) => {
    const found = source.indexOf(signature, start);
    const open = found < 0 ? -1 : source.indexOf("{", found);
    return open < 0 ? undefined : toolJobRustBlock(source, open);
  };
  const ordered = (source: string, tokens: string[]) => {
    let cursor = -1;
    for (const token of tokens) {
      cursor = source.indexOf(token, cursor + 1);
      if (cursor < 0) return false;
    }
    return true;
  };
  const authority = store.indexOf("pub struct ArtifactEnvelopeDecodeAuthority<P, Mutation>");
  const rejected = store.indexOf("pub struct ArtifactEnvelopeDecodeRejected<P, Mutation>", authority);
  const reject = body(store, "pub fn reject(self, diagnostic: OwnedSchemaDecodeDiagnostic)", authority);
  const authorityDrop = body(store, "impl<P, Mutation> Drop for ArtifactEnvelopeDecodeAuthority<P, Mutation>", authority);
  const rejectedClose = body(store, "fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize)", rejected);
  const rejectedDrop = body(store, "impl<P, Mutation> Drop for ArtifactEnvelopeDecodeRejected<P, Mutation>", rejected);
  const publicLaw = body(tests, "fn artifact_envelope_public_rejection_preserves_record_lease_ticket_double_return_and_generation_reuse()");
  const successLaw = body(tests, "fn artifact_envelope_decode_withholds_success_until_field_and_page_owners_are_terminal_empty()");
  const cancelLaw = body(tests, "fn cancelled_and_rejected_envelope_decodes_close_one_exact_owner_per_grant()");
  const generationLaw = body(tests, "fn envelope_field_registry_retains_late_return_and_generation_reuse_until_bounded_app_reclaim()");
  return (
    authority >= 0 &&
    rejected > authority &&
    store.slice(authority, rejected).includes("record: std::mem::ManuallyDrop<Option<OwnedSchemaRecordCursor>>") &&
    store.slice(authority, rejected).includes("fields: std::mem::ManuallyDrop<Option<ArtifactEnvelopeFieldDecoderLease<P, Mutation>>>") &&
    store.slice(rejected).includes("record: std::mem::ManuallyDrop<Option<OwnedSchemaRecordCursor>>") &&
    store.slice(rejected).includes("fields: std::mem::ManuallyDrop<Option<ArtifactEnvelopeFieldDecoderLease<P, Mutation>>>") &&
    !!reject &&
    ordered(reject.body, ["return Err(self)", "std::mem::ManuallyDrop::new(self)", "source.record.take()", "source.fields.take()", "Arc::clone(&source.field_registry)", "source.field_ticket", "source.state = ArtifactEnvelopeDecodeState::Transferred", "std::mem::ManuallyDrop::drop(&mut source)", "Ok(ArtifactEnvelopeDecodeRejected"]) &&
    !reject.body.includes("source.field_returned = true") &&
    !!authorityDrop &&
    ordered(authorityDrop.body, ["ArtifactEnvelopeDecodeState::Transferred", "self.record.is_none()", "self.fields.is_none()", "return;", "self.field_registry.ticket_reclaimed(self.field_ticket)"]) &&
    !authorityDrop.body.includes("return_now()") &&
    !!rejectedClose &&
    ordered(rejectedClose.body, ["if maximum_items == 0", "if !fields.return_now()", "self.field_returned = true", "self.field_registry.ticket_reclaimed(self.field_ticket)", "record.close_step(1)", "self.record.take()", "SnapshotRetirementStep::Complete"]) &&
    !rejectedClose.body.includes("drop(self.record.take())") &&
    !!rejectedDrop &&
    !rejectedDrop.body.includes("return_now()") &&
    !!publicLaw &&
    [
      "record.tokens.pages.slots.as_ptr()",
      "ArtifactEnvelopeFieldDecoderLease::ticket",
      "Arc::as_ptr(&rejected.field_registry)",
      "rejected.diagnostic, diagnostic",
      "rejected.close_step(0, 0)",
      "released_items <= 1",
      "registry.ticket_reclaimed(field_ticket)",
      "reused_ticket.index(), field_ticket.index()",
      "reused_ticket.generation(), field_ticket.generation()",
      "reused.return_now()",
      "!reused.return_now()",
    ].every((token) => publicLaw.body.includes(token)) &&
    (publicLaw.body.match(/Some\(record_identity\)/g) ?? []).length >= 2 &&
    !!successLaw &&
    successLaw.body.includes("semio_framework_job::StepOutcome::Complete(_)") &&
    successLaw.body.includes("registry.terminal_is_empty()") &&
    !!cancelLaw &&
    cancelLaw.body.includes("semio_framework_job::StepOutcome::Cancelled") &&
    cancelLaw.body.includes("reject(diagnostic)") &&
    !!generationLaw &&
    generationLaw.body.includes("second_ticket.index(), first_ticket.index()") &&
    generationLaw.body.includes("second_ticket.generation(), first_ticket.generation()")
  );
}



export function toolJobPresentationEnvelopeCallerRetainedExact(presentation: string, wasm: string, plugin: string): boolean {
  return (
    presentation.includes("pub struct PresentationEnvelopeOwnedFieldCatalog") &&
    presentation.includes("impl store::os_store::owned_schema_decode::ArtifactEnvelopeOwnedFieldCatalog<PresentationSnapshot, PresentationMutation> for PresentationEnvelopeOwnedFieldCatalog") &&
    presentation.includes("pub struct PresentationEnvelopeMaterializeHandle") &&
    presentation.includes("session: std::mem::ManuallyDrop<Option<semio_framework_job::WorkerJobSession<PresentationEnvelopeMaterializeJob>>>") &&
    presentation.includes("session.try_submit_step(pool, semio_framework_job::Lane::Interactive)") &&
    presentation.includes("pub struct PresentationEnvelopeMaterializeRegistry") &&
    presentation.includes("slots: [PresentationEnvelopeMaterializeSlot; PRESENTATION_ENVELOPE_MATERIALIZE_CAPACITY]") &&
    presentation.includes("handle: std::mem::MaybeUninit<PresentationEnvelopeMaterializeHandle>") &&
    presentation.includes("pub fn try_submit(") &&
    presentation.includes("pub fn maintenance_step(") &&
    presentation.includes("pub fn try_publish_to(") &&
    presentation.includes("pub fn close_step(") &&
    presentation.includes("Presentation envelope materialize handle reached Drop before worker, result, and fault owners were terminal empty") &&
    presentation.includes("Presentation envelope materialize registry reached Drop before every retained caller was closed and reclaimed") &&
    presentation.includes("retained_presentation_envelope_caller_faults_and_zero_grant_closes_malformed_pack") &&
    presentation.includes("retained_presentation_envelope_caller_cancels_and_zero_grant_closes_without_output") &&
    presentation.includes("retained_presentation_envelope_registry_preserves_collision_capacity_and_exact_rejected_pages") &&
    presentation.includes("retained_presentation_envelope_wrong_owner_abort_and_interrupted_close_preserve_live_slot") &&
    presentation.includes("retained_presentation_envelope_publication_retries_backpressure_exactly_once") &&
    presentation.includes("retained_presentation_envelope_stale_generation_cancels_and_unpublished_output_closes") &&
    presentation.includes("retained_presentation_envelope_materializes_populated_history_in_order") &&
    presentation.includes("PresentationEnvelopeMaterializeState::Materialize") &&
    presentation.includes("PresentationEnvelopeMaterializeState::RetireEnvelopeComplete") &&
    presentation.includes("presentation_envelope_decode_owner_bundle().retire_envelope(envelope)") &&
    presentation.includes("let (diff, messages) = mutation.diff(current).into_parts()") &&
    presentation.includes("match diff.apply(current)") &&
    !presentation.includes("envelope.try_into_fresh_snapshot()") &&
    plugin.includes("fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>>") &&
    plugin.includes("envelope_decode_jobs: ArtifactFixedRegistry<ActiveArtifactEnvelopeDecode<A::Snapshot, A::Mutation>>") &&
    plugin.includes("pub fn submit_artifact_envelope_decode(") &&
    plugin.includes("pub fn try_publish_artifact_envelope_decode(") &&
    plugin.includes("fn drive_envelope_decode_jobs(") &&
    plugin.includes("close_envelope_decode_jobs_drained") &&
    plugin.includes("active envelope decode reached Drop before its worker and rejected owner were terminal empty") &&
    !presentation.includes("pub fn begin_materialize_presentation_projection(") &&
    !presentation.includes("materialize_presentation_projection_json") &&
    !wasm.includes("materializePresentationProjectionJson") &&
    !wasm.includes("materialize_presentation_projection_json") &&
    !wasm.includes("createPresentationEnvelopeJson") &&
    !wasm.includes("envelope_json") &&
    !wasm.includes("reject_whole_buffer_artifact_envelope_ingress")
  );
}

function toolJobStoreInitializerRetainedExact(plugin: string): boolean {
  return (
    plugin.includes("session: std::mem::ManuallyDrop<Option<semio_framework_job::MountedWorkerJobSession<ArtifactStoreInitializationJob<P, Mutation>>>>") &&
    plugin.includes("session_rejected: std::mem::ManuallyDrop<Option<semio_framework_job::WorkerJobSessionAdmissionRejected<ArtifactStoreInitializationJob<P, Mutation>>>>") &&
    plugin.includes("MountedWorkerJobSession::try_new(job, params)") &&
    plugin.includes("self.jobs.insert_admitted(self.operation.0, ActiveArtifactStoreReplacement::new(self.operation, self.generation, job))") &&
    plugin.includes("artifact store replacement reached Drop before initializer/candidate/displaced-store ownership was terminal empty")
  );
}

export function toolJobWriterEnvelopeCallerRetainedExact(writer: string, editor: string, wasm: string, plugin: string): boolean {
  return (
    writer.includes("struct WriterStoreInitializationAuthority") &&
    writer.includes("impl semio_framework_plugin::ArtifactStoreInitializationAuthority<WriterSnapshot, WriterMutation> for WriterStoreInitializationAuthority") &&
    writer.includes("WriterStoreInitializationPhase::ValidateEdit { index }") &&
    writer.includes("self.edit_index.admit(&envelope.vcs.edits, index,") &&
    writer.includes("WriterStoreInitializationPhase::SeedHistory") &&
    writer.includes("WriterStoreInitializationPhase::BuildCandidate") &&
    writer.includes("ArtifactStoreInitializationRuntime::new") &&
    writer.includes("ArtifactStore::from_initialized_runtime_with_owners") &&
    writer.includes("self.generation.0.checked_add(1)") &&
    writer.includes("Writer store initialization authority reached Drop before exact candidate handoff or retained rejection close") &&
    writer.includes("writer_store_initializer_publishes_exact_next_generation_and_candidate_closes_incrementally") &&
    writer.includes("writer_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty") &&
    editor.includes("fn build_document_store_initialization_job(") &&
    editor.includes("writer_document_store_initialization_job(envelope, operation, generation)") &&
    wasm.includes("pub struct WriterEnvelopeLoadHandle") &&
    wasm.includes("pub fn begin_envelope_load(") &&
    wasm.includes("begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)") &&
    wasm.includes("source: &js_sys::Uint8Array") &&
    wasm.includes("pub fn admit_envelope_page(") &&
    wasm.includes("admit_artifact_envelope_ingress_page(handle.runtime_handle(), page)") &&
    wasm.includes("pub fn seal_envelope_load(") &&
    wasm.includes("seal_artifact_envelope_ingress(handle.runtime_handle())") &&
    wasm.includes("pub fn poll_envelope_load(") &&
    wasm.includes("app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
    wasm.includes("advance_artifact_envelope_load(handle.runtime_handle())") &&
    wasm.includes("acknowledge_artifact_store_replacement(handle.runtime_handle())") &&
    wasm.includes("pub fn cancel_envelope_load(") &&
    wasm.includes("pub fn close_step(&self)") &&
    editor.includes("writer_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed") &&
    editor.includes("writer_live_envelope_cancel_closes_retained_pages_without_publication") &&
    plugin.includes("envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>") &&
    plugin.includes("pub fn begin_artifact_envelope_ingress(") &&
    plugin.includes("pub fn seal_artifact_envelope_ingress(") &&
    plugin.includes("pub fn advance_artifact_envelope_load(") &&
    plugin.includes("fn drive_envelope_ingress(") &&
    plugin.includes("saturated decoder keeps the sealed ingress owner in its original fixed slot for retry") &&
    toolJobStoreInitializerRetainedExact(plugin) &&
    plugin.includes("artifact envelope ingress reached Drop before every admitted page was transferred or bounded-closed") &&
    plugin.includes("artifact_envelope_ingress_saturation_returns_exact_plus_one_owner_and_closes_fifo_slots") &&
    plugin.includes("artifact_envelope_ingress_cancel_and_interrupted_close_release_one_real_page_per_grant") &&
    !wasm.includes("envelope_json: &str") &&
    !wasm.includes("reject_whole_buffer_artifact_envelope_ingress") &&
    !wasm.includes("ArtifactStore::new") &&
    !wasm.includes("while let") &&
    !wasm.includes("loop {")
  );
}

export function toolJobJackEnvelopeCallerRetainedExact(store: string, jack: string, editor: string, wasm: string, plugin: string): boolean {
  return (
    store.includes("pub fn artifact_owned_spr_edit_history_decoder") &&
    store.includes("struct ArtifactOwnedSprMutationArrayAuthority") &&
    store.includes("self.scalar_entry") &&
    store.includes("artifact-spr.mutation-array-cancelled") &&
    store.includes("SPR edit decode reached Drop before exact publication or bounded retirement") &&
    store.includes("pub fn preflight_page_bytes(&self, page_bytes: usize)") &&
    store.includes("pub fn admit_preflighted_page(&mut self, page: OwnedSchemaDecodePage)") &&
    jack.includes("pub struct JackEnvelopeOwnedFieldCatalog") &&
    jack.includes("artifact_owned_spr_edit_history_decoder") &&
    jack.includes("struct JackMutationDecodeAuthority") &&
    jack.includes("OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>") &&
    jack.includes("struct JackSnapshotCloneAuthority") &&
    jack.includes("struct JackStoreInitializationAuthority") &&
    jack.includes("impl semio_framework_plugin::ArtifactStoreInitializationAuthority<JackSnapshot, TrinityGraphMutation> for JackStoreInitializationAuthority") &&
    jack.includes("JackStoreInitializationPhase::ValidateEdit { index }") &&
    jack.includes("self.edit_index.admit(&envelope.vcs.edits, index,") &&
    jack.includes("JackStoreInitializationPhase::SeedHistory") &&
    jack.includes("JackStoreInitializationPhase::BuildCandidate") &&
    jack.includes("ArtifactStoreInitializationRuntime::new") &&
    jack.includes("ArtifactStore::from_initialized_runtime_with_owners") &&
    jack.includes("self.generation.0.checked_add(1)") &&
    jack.includes("Jack store initialization authority reached Drop before exact candidate handoff or retained rejection close") &&
    jack.includes("jack_store_initializer_publishes_exact_next_generation_and_candidate_closes_incrementally") &&
    jack.includes("jack_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty") &&
    jack.includes("jack_nested_mutation_and_child_snapshot_retire_one_exact_owner_per_grant") &&
    !jack.includes("artifact_bounded_history_entry_decoder") &&
    editor.includes("fn build_document_store_initialization_job(") &&
    editor.includes("jack_document_store_initialization_job(envelope, operation, generation)") &&
    editor.includes("jack_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed") &&
    editor.includes("jack_live_envelope_cancel_closes_retained_pages_without_publication") &&
    wasm.includes("pub struct JackEnvelopeLoadHandle") &&
    wasm.includes("pub fn begin_envelope_load(") &&
    wasm.includes("begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)") &&
    wasm.includes("source: &js_sys::Uint8Array") &&
    wasm.includes("pub fn admit_envelope_page(") &&
    wasm.includes("admit_artifact_envelope_ingress_page(handle.runtime_handle(), page)") &&
    wasm.includes("pub fn seal_envelope_load(") &&
    wasm.includes("seal_artifact_envelope_ingress(handle.runtime_handle())") &&
    wasm.includes("pub fn poll_envelope_load(") &&
    wasm.includes("app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
    wasm.includes("advance_artifact_envelope_load(handle.runtime_handle())") &&
    wasm.includes("acknowledge_artifact_store_replacement(handle.runtime_handle())") &&
    wasm.includes("pub fn cancel_envelope_load(") &&
    wasm.includes("pub fn close_step(&self)") &&
    plugin.includes("envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>") &&
    toolJobStoreInitializerRetainedExact(plugin) &&
    !wasm.includes("envelope_json: &str") &&
    !wasm.includes("reject_whole_buffer_artifact_envelope_ingress") &&
    !wasm.includes("ArtifactStore::new") &&
    !wasm.includes("while let") &&
    !wasm.includes("loop {")
  );
}

export function toolJobTrinityRewriteEnvelopeCallerRetainedExact(store: string, jack: string, editor: string, rewrite: string, plugin: string): boolean {
  return (
    store.includes("pub fn artifact_owned_spr_edit_history_decoder") &&
    store.includes("struct ArtifactOwnedSprMutationArrayAuthority") &&
    store.includes("self.scalar_entry") &&
    store.includes("artifact-spr.mutation-array-cancelled") &&
    store.includes("SPR edit decode reached Drop before exact publication or bounded retirement") &&
    jack.includes("pub struct JackEnvelopeOwnedFieldCatalog") &&
    jack.includes("artifact_owned_spr_edit_history_decoder") &&
    jack.includes("struct JackMutationDecodeAuthority") &&
    jack.includes("OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>") &&
    jack.includes("struct JackSnapshotCloneAuthority") &&
    jack.includes("struct JackStoreInitializationAuthority") &&
    jack.includes("impl semio_framework_plugin::ArtifactStoreInitializationAuthority<JackSnapshot, TrinityGraphMutation> for JackStoreInitializationAuthority") &&
    jack.includes("JackStoreInitializationPhase::ValidateEdit { index }") &&
    jack.includes("self.edit_index.admit(&envelope.vcs.edits, index,") &&
    jack.includes("JackStoreInitializationPhase::SeedHistory") &&
    jack.includes("JackStoreInitializationPhase::BuildCandidate") &&
    jack.includes("ArtifactStoreInitializationRuntime::new") &&
    jack.includes("ArtifactStore::from_initialized_runtime_with_owners") &&
    jack.includes("self.generation.0.checked_add(1)") &&
    jack.includes("Jack store initialization authority reached Drop before exact candidate handoff or retained rejection close") &&
    jack.includes("jack_store_initializer_publishes_exact_next_generation_and_candidate_closes_incrementally") &&
    jack.includes("jack_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty") &&
    jack.includes("jack_nested_mutation_and_child_snapshot_retire_one_exact_owner_per_grant") &&
    editor.includes("fn build_document_store_initialization_job(") &&
    editor.includes("jack_document_store_initialization_job(envelope, operation, generation)") &&
    editor.includes("jack_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed") &&
    editor.includes("jack_live_envelope_cancel_closes_retained_pages_without_publication") &&
    rewrite.includes("type TrinityRewriteApp = VcsArtifactApp<EditorApp<TrinityJackPlayApp>>") &&
    rewrite.includes("pub struct TrinityRewriteEnvelopeLoadHandle") &&
    rewrite.includes("fn runtime_handle(&self) -> ArtifactEnvelopeDecodeOperationHandle") &&
    rewrite.includes("TRINITY_REWRITE_ENVELOPE_MAXIMUM_PAGES") &&
    rewrite.includes("TRINITY_REWRITE_ENVELOPE_MAXIMUM_BYTES") &&
    rewrite.includes("trinity_rewrite_envelope_credits_admit_exact_caps_and_reject_zero_or_plus_one") &&
    rewrite.includes("pub async fn new()") &&
    rewrite.includes("VcsArtifactApp::new(EditorApp::<TrinityJackPlayApp>::default()).await") &&
    rewrite.includes("pub fn begin_envelope_load(") &&
    rewrite.includes("begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)") &&
    rewrite.includes("source: &js_sys::Uint8Array") &&
    rewrite.includes("pub enum TrinityRewriteEnvelopePageFault") &&
    rewrite.includes("pub struct TrinityRewriteEnvelopePageAdmission") &&
    rewrite.includes("TrinityRewriteCallerPageOwner::new(owner)") &&
    rewrite.includes("source.clone()") &&
    rewrite.includes("source.copy_to(&mut bytes[..len])") &&
    rewrite.includes("let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]") &&
    rewrite.includes("ArtifactEnvelopeDecodePage::from_preflighted_array(bytes, len)") &&
    rewrite.includes("construct_and_admit_artifact_envelope_ingress_page(handle.runtime_handle(), len") &&
    rewrite.includes("pub fn retry_envelope_page(") &&
    rewrite.includes("pub fn is_same_page(") &&
    rewrite.includes("pub fn take_page(&mut self) -> Option<js_sys::Uint8Array>") &&
    rewrite.includes("pub fn close_step(&mut self) -> bool") &&
    rewrite.includes("pub fn terminal_is_empty(&self) -> bool") &&
    rewrite.includes("trinity_rewrite_rejected_page_preserves_pointer_content_and_retry_owner") &&
    rewrite.includes("trinity_rewrite_page_cap_plus_one_rejects_before_owner_construction") &&
    rewrite.includes("trinity_rewrite_stale_generation_and_slot_aba_never_match") &&
    rewrite.includes("trinity_rewrite_checked_out_drop_preserves_raw_caller_authority") &&
    rewrite.includes("trinity_rewrite_rejected_page_close_retires_one_owner_per_grant") &&
    rewrite.indexOf("construct_and_admit_artifact_envelope_ingress_page(handle.runtime_handle(), len") < rewrite.indexOf("source.copy_to(&mut bytes[..len])") &&
    rewrite.includes("seal_artifact_envelope_ingress(handle.runtime_handle())") &&
    rewrite.includes("app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
    rewrite.includes("advance_artifact_envelope_load(handle.runtime_handle())") &&
    rewrite.includes("acknowledge_artifact_store_replacement(handle.runtime_handle())") &&
    rewrite.includes("cancel_artifact_envelope_load(handle.runtime_handle())") &&
    rewrite.includes("close_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
    plugin.includes("envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>") &&
    plugin.includes("pub fn preflight_artifact_envelope_ingress_page(") &&
    plugin.includes("pub fn construct_and_admit_artifact_envelope_ingress_page<Build>(") &&
    toolJobStoreInitializerRetainedExact(plugin) &&
    plugin.includes("artifact_envelope_ingress_saturation_returns_exact_plus_one_owner_and_closes_fifo_slots") &&
    plugin.includes("artifact_envelope_ingress_cancel_and_interrupted_close_release_one_real_page_per_grant") &&
    !rewrite.includes("envelope_json: Option<String>") &&
    !rewrite.includes("RefCell<TrinityGraphStore>") &&
    !rewrite.includes("reject_whole_buffer_artifact_envelope_ingress") &&
    !rewrite.includes("dispatch_text(command_text)") &&
    !rewrite.includes("dispatch_binary(command_bytes)") &&
    !rewrite.includes("map_err(|(fault, _page)|") &&
    !rewrite.includes("source.to_vec()") &&
    !rewrite.includes("Uint8Array::new(source)") &&
    !rewrite.includes("while let") &&
    !rewrite.includes("loop {")
  );
}

export function toolJobGisMapEnvelopeCallerRetainedExact(store: string, gisMap: string, editor: string, wasm: string, plugin: string): boolean {
  const variants = [
    "CreatePosition(payload)",
    "DeletePosition(payload)",
    "ReorderPositions(payload)",
    "ReplacePositionData(payload)",
    "CreateRoute(payload)",
    "DeleteRoute(payload)",
    "ReorderRoutes(payload)",
    "ReplaceRouteData(payload)",
    "CreateRegion(payload)",
    "DeleteRegion(payload)",
    "ReorderRegions(payload)",
    "ReplaceRegionData(payload)",
  ];
  return (
    store.includes("pub fn artifact_owned_spr_edit_history_decoder") &&
    store.includes("struct ArtifactOwnedSprMutationArrayAuthority") &&
    store.includes("self.scalar_entry") &&
    store.includes("artifact-spr.mutation-array-cancelled") &&
    store.includes("SPR edit decode reached Drop before exact publication or bounded retirement") &&
    gisMap.includes("pub struct GisMapEnvelopeOwnedFieldCatalog") &&
    gisMap.includes("artifact_owned_spr_edit_history_decoder") &&
    gisMap.includes("GisMapSnapshotDecodeAuthority") &&
    gisMap.includes("GisMapMutationDecodeAuthority") &&
    gisMap.includes("OwnedSchemaHexAuthority<GIS_MAP_OWNED_FIELD_BYTES>") &&
    gisMap.includes("struct GisMapSnapshotCloneAuthority") &&
    gisMap.includes("struct GisMapStoreInitializationAuthority") &&
    gisMap.includes("impl semio_framework_plugin::ArtifactStoreInitializationAuthority<GisMapSnapshot, GisMapMutation> for GisMapStoreInitializationAuthority") &&
    gisMap.includes("GisMapStoreInitializationPhase::ValidateEdit { index }") &&
    gisMap.includes("self.edit_index.admit(&envelope.vcs.edits, index,") &&
    gisMap.includes("GisMapStoreInitializationPhase::SeedHistory") &&
    gisMap.includes("GisMapStoreInitializationPhase::BuildCandidate") &&
    gisMap.includes("ArtifactStoreInitializationRuntime::new") &&
    gisMap.includes("ArtifactStore::from_initialized_runtime_with_owners") &&
    gisMap.includes("self.generation.0.checked_add(1)") &&
    gisMap.includes("value.positions.pop()") &&
    gisMap.includes("value.routes.pop()") &&
    gisMap.includes("value.regions.pop()") &&
    gisMap.includes("Self::child_step(&mut value.drawing") &&
    gisMap.includes("Self::child_step(image") &&
    gisMap.includes("Self::child_step(&mut value.value") &&
    gisMap.includes("dsl::DslValue::Array(values)") &&
    gisMap.includes("dsl::DslValue::Object(values)") &&
    variants.every((variant) => gisMap.includes(variant)) &&
    gisMap.includes("GIS store initialization authority reached Drop before exact candidate handoff or retained rejection close") &&
    gisMap.includes("gis_map_store_initializer_publishes_next_generation_and_candidate_closes_incrementally") &&
    gisMap.includes("gis_map_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty") &&
    gisMap.includes("gis_map_nested_value_mutation_and_all_child_handles_retire_one_owner_per_grant") &&
    gisMap.includes("gis_map_all_twelve_mutation_variants_preserve_catalog_order_and_zero_grant_ownership") &&
    !gisMap.includes("artifact_bounded_history_entry_decoder") &&
    editor.includes("fn build_document_store_initialization_job(") &&
    editor.includes("gis_map_document_store_initialization_job(envelope, operation, generation)") &&
    editor.includes("gis_map_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed") &&
    editor.includes("gis_map_live_envelope_cancel_closes_retained_pages_without_publication") &&
    editor.includes("duplicate GIS load acknowledgement is a no-op") &&
    wasm.includes("pub struct GisMapEnvelopeLoadHandle") &&
    wasm.includes("pub fn begin_envelope_load(") &&
    wasm.includes("begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)") &&
    wasm.includes("source: &js_sys::Uint8Array") &&
    wasm.includes("pub fn admit_envelope_page(") &&
    wasm.includes("admit_artifact_envelope_ingress_page(handle.runtime_handle(), page)") &&
    wasm.includes("pub fn seal_envelope_load(") &&
    wasm.includes("seal_artifact_envelope_ingress(handle.runtime_handle())") &&
    wasm.includes("pub fn poll_envelope_load(") &&
    wasm.includes("app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
    wasm.includes("advance_artifact_envelope_load(handle.runtime_handle())") &&
    wasm.includes("acknowledge_artifact_store_replacement(handle.runtime_handle())") &&
    wasm.includes("pub fn cancel_envelope_load(") &&
    wasm.includes("pub fn close_step(&self)") &&
    plugin.includes("envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>") &&
    toolJobStoreInitializerRetainedExact(plugin) &&
    !wasm.includes("envelope_json: &str") &&
    !wasm.includes("reject_whole_buffer_artifact_envelope_ingress") &&
    !wasm.includes("ArtifactStore::new") &&
    !wasm.includes("while let") &&
    !wasm.includes("loop {")
  );
}

export function toolJobRasterEnvelopeCallerRetainedExact(store: string, raster: string, editor: string, plugin: string): boolean {
  const variants = [
    "CreateLayer(payload)",
    "DeleteLayer(payload)",
    "ReorderLayers(payload)",
    "RenameLayer(payload)",
    "ChangeLayerVisible(payload)",
    "ChangeLayerOpacity(payload)",
    "ChangeLayerBlendMode(payload)",
    "MoveLayer(payload)",
    "ResizeLayer(payload)",
    "ChangeLayerAdjustmentKind(payload)",
    "AddLayerAsset(payload)",
    "RemoveLayerAsset(payload)",
  ];
  const credits = editor.indexOf("begin_artifact_envelope_ingress(pages, ");
  const page = editor.indexOf("let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]");
  const construct = editor.indexOf("store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, ");
  const admit = editor.indexOf("admit_artifact_envelope_ingress_page(handle, page)");
  const rasterOwnedMapSerializeBound = /impl(?:\s*<[^>]*>)?\s*(?:serde::)?Serialize\s+for\s+RasterOwnedMap\b/s.test(raster);
  const rasterOwnedMapWholeSerializeMap = /serialize_map\s*\(\s*Some\s*\(\s*self\.length\s*\)\s*\)/s.test(raster);
  const rasterOwnedMapWholeSerializeEntry = /\.serialize_entry\s*\(/s.test(raster);
  const rasterOwnedMapSerdeGuards = (raster.match(/serde\(serialize_with = "crate::artifacts::raster::serialize_empty_owned_map"\)/g) ?? []).length;
  const rasterOwnedMapFieldSites = (raster.match(/^\s*(?:pub )?[a-z_]+: RasterOwnedMap</gm) ?? []).length;
  const rasterWholeJoinedOutput = /\.collect\s*::\s*<\s*Vec\s*<\s*_\s*>\s*>\s*\(\s*\)\s*\.join\s*\(/s.test(raster);
  const rasterPerValueJsonOutput = /serde_json::to_vec\s*\(\s*v\s*\)/s.test(raster);
  const rasterDirectMapOutput = /for\s*\(\s*k\s*,\s*v\s*\)\s*in\s*(?:map|params)\b/s.test(raster);
  const rasterEmptyMapOutputGuards = (raster.match(/assert!\((?:map|params)\.is_empty\(\), "\{RASTER_POPULATED_OUTPUT_ERROR\}"\);/g) ?? []).length;
  const rasterEmptyLayerOutputGuards = (raster.match(/assert!\(list\.is_empty\(\), "\{RASTER_POPULATED_OUTPUT_ERROR\}"\);/g) ?? []).length;
  const rasterSnapshotOutputPreflights = (raster.match(/s\.require_empty_output_shell\(\)\.expect\(RASTER_POPULATED_OUTPUT_ERROR\);/g) ?? []).length;
  const rasterMountedOutputEntryPoints = (raster.match(/pub fn serialize_bytes\(snapshot: &RasterSnapshot\) -> Result<Vec<u8>, String> \{/g) ?? []).length;
  const rasterMountedPixelComposites = (raster.match(/raster_composite_image\(snapshot\)/g) ?? []).length;
  const rasterMountedPixelDialects = (raster.match(/semio_image_to_format\(&image, /g) ?? []).length;
  const rasterMountedStdioEncoders = (raster.match(/semio_s_artifact_stdio_[a-z0-9_]*::[A-Za-z0-9_:]*io::encode_[a-z0-9]+\(/g) ?? []).length;
  const rasterMountedVectorComposites = (raster.match(/raster_document_json_to_svg\(snapshot\)/g) ?? []).length;
  const rasterMountedTypedDeclines = (raster.match(/Err\(RASTER_(?:PDF|DWG)_EXPORT_UNSUPPORTED\.to_string\(\)\)/g) ?? []).length;
  const rasterMountedOutputGuards = (raster.match(/snapshot\.require_empty_output_shell\(\)\.map_err\(str::to_owned\)\?;/g) ?? []).length;
  const rasterMountedOutputCallers = (raster.match(/Ok\(<RasterSnapshot as store::os_store::serialization::ArtifactDsl>::print_dsl\(snapshot\)\.into_bytes\(\)\)/g) ?? []).length;
  const rasterOutputFixtureStart = raster.indexOf("fn raster_populated_snapshot_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact()");
  const rasterOutputFixtureEnd = raster.indexOf("fn raster_maximum_combined_layer_and_value_depth_retires_to_terminal()", rasterOutputFixtureStart);
  const rasterOutputFixture = rasterOutputFixtureStart >= 0 && rasterOutputFixtureEnd > rasterOutputFixtureStart ? raster.slice(rasterOutputFixtureStart, rasterOutputFixtureEnd) : "";
  const rasterRejectedParamValueCapture = rasterOutputFixture.indexOf("let plus_one_param_value_pointer = plus_one_param_value.as_ptr();");
  const rasterRejectedParamInsert = rasterOutputFixture.indexOf("params.insert(plus_one_param_key, dsl::DslValue::String(plus_one_param_value))");
  const rasterRejectedParamValueBinding = rasterOutputFixture.indexOf("let rejected_param_value = match &rejected_param.value");
  const rasterRejectedParamValueAssertion = rasterOutputFixture.indexOf(
    'assert_eq!(rejected_param_value.as_ptr(), plus_one_param_value_pointer, "rejected output parameter returns the exact value allocation");',
  );
  const rasterRejectedParamRetirement = rasterOutputFixture.indexOf(
    "RasterOwnedRetirement::new(RasterRetirementOwner::ValueEntry { key: rejected_param.key, value: Some(rejected_param.value) })",
  );
  const rasterRejectedAssetChildCapture = rasterOutputFixture.indexOf("let plus_one_asset_child_pointer = plus_one_asset_child.child_id.as_ptr();");
  const rasterRejectedAssetInsert = rasterOutputFixture.indexOf("assets.insert(plus_one_asset_key, plus_one_asset_child)");
  const rasterRejectedAssetChildBinding = rasterOutputFixture.indexOf("rejected_asset.value.child_id.as_ptr()");
  const rasterRejectedAssetChildAssertion = rasterOutputFixture.indexOf(
    ', plus_one_asset_child_pointer, "rejected output asset returns the exact child allocation");',
    rasterRejectedAssetChildBinding,
  );
  const rasterRejectedAssetRetirement = rasterOutputFixture.indexOf(
    "RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key: rejected_asset.key, child: Some(rejected_asset.value) })",
  );
  return (
    store.includes("pub fn artifact_owned_spr_edit_history_decoder") &&
    store.includes("struct ArtifactOwnedSprMutationArrayAuthority") &&
    raster.includes("pub struct RasterEnvelopeOwnedFieldCatalog") &&
    raster.includes("artifact_owned_spr_edit_history_decoder") &&
    raster.includes("RasterSnapshotDecodeAuthority") &&
    raster.includes("RasterMutationDecodeAuthority") &&
    raster.includes("OwnedSchemaHexAuthority<RASTER_OWNED_FIELD_BYTES>") &&
    raster.includes("struct RasterSnapshotBoundsAuthority") &&
    raster.includes("struct RasterSnapshotCloneAuthority") &&
    raster.includes("struct RasterMutationCandidateAuthority") &&
    raster.includes("RASTER_RETIREMENT_STACK_CAPACITY") &&
    raster.includes("pages: std::mem::ManuallyDrop<[Option<Box<RasterRetirementFramePage>>; RASTER_RETIREMENT_STACK_PAGE_COUNT]>") &&
    raster.includes("pending_push: std::mem::ManuallyDrop<Option<RasterRetirementOwner>>") &&
    raster.includes("struct RasterStoreInitializationAuthority") &&
    raster.includes("impl semio_framework_plugin::ArtifactStoreInitializationAuthority<RasterSnapshot, RasterMutation> for RasterStoreInitializationAuthority") &&
    raster.includes("RasterStoreInitializationPhase::ValidateEdit { index }") &&
    raster.includes("self.edit_index.admit(&envelope.vcs.edits, index,") &&
    raster.includes("RasterStoreInitializationPhase::SeedHistory") &&
    raster.includes("RasterStoreInitializationPhase::BuildCandidate") &&
    (raster.match(/if cx\.should_yield\(\)/g) ?? []).length === 3 &&
    raster.includes("fn raster_reserve_unit(cx: &mut semio_framework_job::StepContext<'_>) -> bool") &&
    raster.includes("cx.consume_fuel(1)") &&
    raster.includes("self.cancel_requested || cx.is_cancelled()") &&
    raster.includes("ArtifactStoreInitializationRuntime::new") &&
    raster.includes("ArtifactStore::from_initialized_runtime_with_owners") &&
    raster.includes("self.generation.0.checked_add(1)") &&
    raster.includes("value.layers.pop()") &&
    raster.includes("value.assets.take_last_entry()") &&
    raster.includes("value.assets.take_empty_page_backing()") &&
    raster.includes("RasterRetirementOwner::AssetMapPage(page)") &&
    raster.includes("RasterRetirementOwner::ValueMapPage(page)") &&
    raster.includes("RasterOwnedMap::<V>::conservative_page_credit_bytes()") &&
    raster.includes("fn observe_candidate_capacity(") &&
    raster.includes("fn raster_exact_string_from_parts(") &&
    raster.includes("const RASTER_CONTROL_BACKING_BYTES: usize = RASTER_OWNED_FIELD_BYTES") &&
    raster.includes("const RASTER_MAXIMUM_CONTROL_BACKINGS: usize = RASTER_RETIREMENT_STACK_PAGE_COUNT + RASTER_NON_STACK_CONTROL_BACKINGS") &&
    raster.includes("const RASTER_MAXIMUM_CONTROL_BYTES: usize = RASTER_MAXIMUM_CONTROL_BACKINGS * RASTER_CONTROL_BACKING_BYTES") &&
    raster.includes("source_control_bytes: usize") &&
    raster.includes("self.source_control_bytes = RASTER_MAXIMUM_CONTROL_BYTES") &&
    raster.includes("raster-store.control-backing-double-reservation") &&
    raster.includes("static RASTER_RETIREMENT_PROCESS_PAGES: std::sync::atomic::AtomicUsize") &&
    raster.includes("static RASTER_INITIALIZATION_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize") &&
    raster.includes("static RASTER_STANDALONE_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize") &&
    raster.includes("held_items: usize") &&
    raster.includes("held_bytes: usize") &&
    (raster.match(/control: std::mem::ManuallyDrop<Option<RasterStandaloneControlCredit>>/g) ?? []).length === 2 &&
    (raster.match(/let control = RasterStandaloneControlCredit::try_claim\(\)\.ok\(\);/g) ?? []).length === 2 &&
    raster.includes("fn claim_control_if_available(&mut self) -> Result<bool, String>") &&
    raster.includes("Err(\"raster-store.standalone-control-capacity\") => Ok(false)") &&
    raster.includes("control_returned: bool") &&
    raster.includes("remaining: RASTER_NON_STACK_CONTROL_BACKINGS") &&
    raster.includes("control_reservation: std::mem::ManuallyDrop<Option<RasterInitializationControlReservation>>") &&
    raster.includes("normal completion returns every non-stack process control credit") &&
    raster.includes("standalone Box control credit is returned before terminal-empty") &&
    raster.includes("standalone Arc and inner Box control credits return before terminal-empty") &&
    raster.includes("fn reserve_page_credit(&mut self, page_index: usize)") &&
    raster.includes("compare_exchange(current, next") &&
    raster.includes("self.return_page_credit(page_index)?") &&
    raster.includes("pages: std::mem::ManuallyDrop<[Option<Box<RasterOwnedMapPage<V>>>; RASTER_OWNED_MAP_PAGE_COUNT]>") &&
    raster.includes("Raster owned map reached Drop before every entry and page backing was explicitly retired") &&
    raster.includes("Populated Raster owned maps require the retained page clone authority") &&
    raster.includes("Raster maps require the retained page decoder") &&
    raster.includes("impl<V> dsl::ToValue for RasterOwnedMap<V>") &&
    raster.includes("impl<V> dsl::FromValue for RasterOwnedMap<V>") &&
    raster.includes("Populated Raster owned map serialization is forbidden; interactive production routes require the retained page output authority") &&
    rasterOwnedMapFieldSites === 3 &&
    (rasterOwnedMapSerdeGuards === 0 || raster.includes("fn serialize_empty_owned_map<S: serde::Serializer, V>")) &&
    !rasterOwnedMapSerializeBound &&
    !rasterOwnedMapWholeSerializeMap &&
    !rasterOwnedMapWholeSerializeEntry &&
    raster.includes("pub(crate) fn require_empty_output_shell(&self) -> Result<(), &'static str>") &&
    raster.includes("self.layers.is_empty() && self.assets.is_empty()") &&
    raster.includes("Populated Raster snapshot output is forbidden; interactive production routes require the retained page output authority") &&
    raster.includes("pub(crate) fn enc_asset_map(map: &RasterOwnedMap<RasterAssetChild>) -> String") &&
    raster.includes("pub(crate) fn enc_params(params: &RasterOwnedMap<dsl::DslValue>) -> String") &&
    raster.includes("fn write_asset_map(out: &mut Vec<u8>, map: &RasterOwnedMap<RasterAssetChild>)") &&
    raster.includes("fn write_params(out: &mut Vec<u8>, params: &RasterOwnedMap<dsl::DslValue>)") &&
    raster.includes("fn print_raster_snapshot_body(s: &RasterSnapshot) -> String") &&
    raster.includes("fn encode_raster_snapshot_binary(s: &RasterSnapshot) -> Vec<u8>") &&
    !raster.includes("async fn write_asset_map(out: &mut Vec<u8>, map: &RasterOwnedMap<RasterAssetChild>)") &&
    !raster.includes("async fn write_params(out: &mut Vec<u8>, params: &RasterOwnedMap<dsl::DslValue>)") &&
    !raster.includes("async fn print_raster_snapshot_body(s: &RasterSnapshot) -> String") &&
    !raster.includes("async fn encode_raster_snapshot_binary(s: &RasterSnapshot) -> Vec<u8>") &&
    raster.includes("self.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);") &&
    raster.includes("self.require_empty_output_shell().map_err(|error| store::PackError::Schema(error.to_owned()))?;") &&
    rasterEmptyMapOutputGuards === 4 &&
    rasterEmptyLayerOutputGuards === 2 &&
    rasterSnapshotOutputPreflights === 2 &&
    rasterMountedOutputEntryPoints === 8 &&
    rasterMountedPixelComposites === 5 &&
    rasterMountedPixelDialects === 5 &&
    rasterMountedStdioEncoders === 5 &&
    rasterMountedVectorComposites === 1 &&
    rasterMountedTypedDeclines === 2 &&
    raster.includes("pub const RASTER_PDF_EXPORT_UNSUPPORTED: &str =") &&
    raster.includes("pub const RASTER_DWG_EXPORT_UNSUPPORTED: &str =") &&
    rasterMountedOutputGuards === 0 &&
    rasterMountedOutputCallers === 0 &&
    rasterRejectedParamValueCapture >= 0 &&
    rasterRejectedParamInsert > rasterRejectedParamValueCapture &&
    rasterRejectedParamValueBinding > rasterRejectedParamInsert &&
    rasterRejectedParamValueAssertion > rasterRejectedParamValueBinding &&
    rasterRejectedParamRetirement > rasterRejectedParamValueAssertion &&
    rasterRejectedAssetChildCapture >= 0 &&
    rasterRejectedAssetInsert > rasterRejectedAssetChildCapture &&
    rasterRejectedAssetChildBinding > rasterRejectedAssetInsert &&
    rasterRejectedAssetChildAssertion > rasterRejectedAssetChildBinding &&
    rasterRejectedAssetRetirement > rasterRejectedAssetChildAssertion &&
    !rasterWholeJoinedOutput &&
    !rasterPerValueJsonOutput &&
    !rasterDirectMapOutput &&
    raster.includes("Populated Raster owned map DSL materialization is forbidden; interactive production routes require the retained page output authority") &&
    raster.includes("dsl::FieldValue::Map(Vec::new())") &&
    raster.includes("pub fn remove_entry(&mut self, key: &str) -> Option<RasterOwnedMapEntry<V>>") &&
    !raster.includes("pub fn remove(&mut self, key: &str)") &&
    !raster.includes("result.insert(key.clone(), value.clone())") &&
    !raster.includes("while let Some((key, value)) = source.next_entry") &&
    raster.includes("RasterRetirementOwner::BoxedLayer(Some(layer))") &&
    raster.includes("const RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY: usize = RASTER_RETIREMENT_LAYER_FRAMES + RASTER_RETIREMENT_VALUE_FRAMES + RASTER_RETIREMENT_WRAPPER_FRAMES") &&
    raster.includes("const RASTER_RETIREMENT_STACK_CAPACITY: usize = RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY + RASTER_RETIREMENT_REJECTED_OWNER_MARGIN") &&
    raster.includes("dsl::DslValue::Array(values)") &&
    raster.includes("dsl::DslValue::Object(values)") &&
    variants.every((variant) => raster.includes(variant)) &&
    raster.includes("Raster store initialization authority reached Drop before exact candidate handoff or retained rejection close") &&
    raster.includes("raster_store_initializer_publishes_next_generation_and_candidate_closes_incrementally") &&
    raster.includes("raster_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty") &&
    raster.includes("raster_store_initializer_zero_budget_advances_no_owner_or_phase") &&
    raster.includes("raster_nested_snapshot_and_child_handles_retire_one_owner_per_grant") &&
    raster.includes("raster_owner_caps_and_all_mutation_variants_retire_one_owner_per_grant") &&
    raster.includes("raster_envelope_caps_and_plus_one_page_return_the_exact_fixed_owner") &&
    raster.includes("raster_snapshot_bounds_and_clone_advance_one_pre_admitted_unit_with_low_nonzero_fuel") &&
    raster.includes("raster_empty_bounds_and_mounted_sixty_four_fuel_progress_across_second_map_page") &&
    raster.includes("raster_expired_deadline_advances_no_bounds_clone_or_mutation_owner") &&
    raster.includes("raster_small_mutation_against_deep_snapshot_is_cursorized_and_atomic") &&
    raster.includes("raster_cancel_after_complete_retires_the_unclaimed_candidate_before_terminal") &&
    raster.includes("raster_retirement_uses_allocation_capacity_and_fixed_iterative_depth") &&
    raster.includes("raster_nested_owner_item_and_byte_capacity_plus_one_reject_before_clone") &&
    raster.includes("raster_retirement_page_credit_is_claimed_before_allocation_and_returned_with_backing") &&
    raster.includes("raster_owned_map_removal_returns_exact_pair_and_populated_drop_refuses") &&
    raster.includes("raster_empty_asset_map_retirement_has_no_hidden_allocation_release") &&
    raster.includes("raster_owned_map_cap_plus_one_returns_exact_owner_and_populated_pages_retire_explicitly") &&
    raster.includes("raster_observed_capacity_and_combined_retirement_depth_are_exact") &&
    raster.includes("raster_box_and_arc_control_backings_require_and_report_fixed_credit") &&
    raster.includes("raster_standalone_control_max_plus_one_returns_exact_owner_and_resumes_after_full_saturation") &&
    raster.includes("raster_arc_factory_full_saturation_preserves_exact_producer_through_every_control_phase") &&
    raster.includes("raster_populated_dsl_materialization_max_plus_one_nested_cancel_fault_panic_and_close_are_exact") &&
    raster.includes("raster_populated_serde_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact") &&
    raster.includes("raster_populated_snapshot_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact") &&
    raster.includes("std::mem::size_of::<RasterOwnedRetirement>() <= RASTER_CONTROL_BACKING_BYTES") &&
    raster.includes("std::mem::size_of::<RasterRetirementFramePage>() <= RASTER_CONTROL_BACKING_BYTES") &&
    raster.includes("raster_maximum_combined_layer_and_value_depth_retires_to_terminal") &&
    (raster.match(/RasterOwnedMapInsert::Replaced/g) ?? []).length >= 1 &&
    raster.includes("replacement returns the exact displaced pair") &&
    raster.includes("assert_eq!(previous_key.as_ptr(), old_key_pointer)") &&
    raster.includes("candidate_disposer") &&
    raster.includes("let released_bytes = value.capacity();") &&
    raster.includes("let bytes = value.capacity();") &&
    !raster.includes("serde_json::to_vec(source)") &&
    !raster.includes("source.clone()") &&
    !raster.includes("source.assets.clone()") &&
    !raster.includes("serde_json::to_value(&source.assets)") &&
    !raster.includes("source.assets.to_value()") &&
    !raster.includes("entries.push((key.clone(), value.to_value()))") &&
    !raster.includes("drop(populated_map)") &&
    !raster.includes("operation.diff(current)") &&
    !raster.includes("diff.apply(current)") &&
    !raster.includes("active: Box<RasterOwnedRetirement>") &&
    !raster.includes("artifact_bounded_history_entry_decoder") &&
    editor.includes("fn build_envelope_decode_owner_bundle()") &&
    editor.includes("raster_envelope_decode_owner_bundle()") &&
    editor.includes("fn build_document_store_initialization_job(") &&
    editor.includes("raster_document_store_initialization_job(envelope, operation, generation)") &&
    editor.includes("-> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle") &&
    editor.includes("assert_eq!(handle.generation, base_generation)") &&
    editor.includes("chunks(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
    credits >= 0 &&
    page > credits &&
    construct > page &&
    admit > construct &&
    editor.includes("seal_artifact_envelope_ingress(handle)") &&
    editor.includes("app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
    editor.includes("advance_artifact_envelope_load(handle)") &&
    editor.includes("acknowledge_artifact_store_replacement(handle)") &&
    editor.includes("duplicate Raster load acknowledgement is a no-op") &&
    editor.includes("cancel_artifact_envelope_load(handle)") &&
    editor.includes("close_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
    editor.includes("raster_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed") &&
    editor.includes("raster_live_envelope_cancel_closes_retained_pages_without_publication") &&
    plugin.includes("envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>") &&
    toolJobStoreInitializerRetainedExact(plugin) &&
    plugin.includes("artifact store initialization job reached Drop before exact candidate handoff or terminal retained close") &&
    plugin.includes("artifact_envelope_ingress_saturation_returns_exact_plus_one_owner_and_closes_fifo_slots") &&
    plugin.includes("artifact_envelope_ingress_cancel_and_interrupted_close_release_one_real_page_per_grant") &&
    !editor.includes("envelope_json: &str") &&
    !editor.includes("reject_whole_buffer_artifact_envelope_ingress") &&
    !editor.includes("ArtifactStore::new") &&
    !editor.includes("source.to_vec()") &&
    !editor.includes("while let") &&
    !editor.includes("loop {")
  );
}

export function toolJobDrawingEnvelopeCallerRetainedExact(store: string, drawing: string, editor: string, plugin: string, editorTestLaws: string = editor): boolean {
  const variants = [
    "SetLayerVisible(payload)",
    "SetLayerLocked(payload)",
    "SetLayerOpacity(payload)",
    "SetLayerBlendMode(payload)",
    "RenameLayer(payload)",
    "UpdateLayerTransform(payload)",
    "ReplaceLayerFill(payload)",
    "ReplaceLayerStroke(payload)",
    "SetLayerBooleanOperation(payload)",
    "UpdateLayerTraceParams(payload)",
    "CreateLayer(payload)",
    "DuplicateLayer(payload)",
    "DeleteLayer(payload)",
    "ReorderLayer(payload)",
  ];
  const owners = [
    "DrawingRetirementOwner::Layer(value)",
    "DrawingRetirementOwner::Fill(fill)",
    "DrawingRetirementOwner::Stroke(stroke)",
    "DrawingRetirementOwner::Segments(values)",
    "DrawingRetirementOwner::Stops(values)",
    "DrawingRetirementOwner::Points(values)",
    "DrawingRetirementOwner::AssetEntry",
  ];
  const digestFields = [
    "FillStyle::Solid { color }",
    "FillStyle::LinearGradient { x1, y1, x2, y2, stops }",
    "FillStyle::RadialGradient { cx: center_x, cy: center_y, r, stops }",
    "stop.offset",
    "stop.color",
    "value.color[(self.phase - 1) as usize]",
    "value.width",
    "observe_owned_string(digest, 246, &value.cap",
    "observe_owned_string(digest, 247, &value.join",
    "value.dash.is_some()",
    "base.visible",
    "base.locked",
    "base.opacity",
    "observe_owned_string(digest, 106, &base.blend_mode",
    "base.transform.rotation",
    "observe_owned_string(digest, 340, &value.shape_kind",
    "value.rect.is_some()",
    "value.ellipse.is_some()",
    "value.circle.is_some()",
    "value.line.is_some()",
    "value.polygon.is_some()",
    "PathSegment::Move",
    "PathSegment::Line",
    "PathSegment::Quad",
    "PathSegment::Cubic",
    "PathSegment::Arc",
    "PathSegment::Close",
    "observe_owned_string(digest, 382, &value.content",
    "observe_owned_string(digest, 390, &value.image_key",
    "observe_owned_string(digest, 410, &value.operation",
    "value.children.len()",
    "observe_owned_string(digest, 420, &value.source_key",
    "value.params.threshold",
    "Self::variant(mutation)",
    "value.visible",
    "value.locked",
    "value.opacity",
    "observe_owned_string(digest, 3, &value.blend_mode",
    "observe_owned_string(digest, 3, &value.new_name",
    "value.transform.rotation",
    "value.fill.as_ref()",
    "value.stroke.as_ref()",
    "observe_owned_string(digest, 3, &value.boolean_operation",
    "value.params.simplify_epsilon",
    "value.parent_id.is_some()",
    "value.index",
    "value.layer",
  ];
  const requestStart = drawing.indexOf("pub fn request_drawing_mutation_arena_pool()");
  const requestAuthority = requestStart >= 0 ? toolJobRustBlock(drawing, drawing.indexOf("{", requestStart)) : undefined;
  const requestBlock = requestAuthority?.body ?? "";
  const borrowStart = drawing.indexOf("fn borrow_drawing_mutation_arena()");
  const borrowAuthority = borrowStart >= 0 ? toolJobRustBlock(drawing, drawing.indexOf("{", borrowStart)) : undefined;
  const borrowBlock = borrowAuthority?.body ?? "";
  const defaultStart = editor.indexOf("impl Default for DrawingPlayApp");
  const defaultAuthority = defaultStart >= 0 ? toolJobRustBlock(editor, editor.indexOf("{", defaultStart)) : undefined;
  const defaultBlock = defaultAuthority?.body ?? "";
  const governedBootstrapArm = /DrawingStoreInitializationPhase::InitializeArena\s*=>\s*\{\s*match\s+self\.arena_bootstrap_job\.step\(cx\)/s.test(drawing);
  const exactSourceUndo = /if let Some\(undo\) = self\.source_undo\s*\{[\s\S]*?self\.start_rebuild\(source, undo\.parent, None, Some\(undo\.index\), DrawingContainerRebuildRole::CloseSourceUndo\)\?;/s.test(drawing);
  return (
    store.includes("pub fn artifact_owned_spr_edit_history_decoder") &&
    store.includes("struct ArtifactOwnedSprMutationArrayAuthority") &&
    store.includes("self.scalar_entry") &&
    store.includes("artifact-spr.mutation-array-cancelled") &&
    store.includes("id_digest: [u8; 32]") &&
    store.includes('Self::push_revision_record(&mut self.revision.applied, identity_digest, b"applied", &id, edit_digest)?') &&
    drawing.includes("pub struct DrawingEnvelopeOwnedFieldCatalog") &&
    drawing.includes("artifact_owned_spr_edit_history_decoder") &&
    drawing.includes("DrawingSnapshotDecodeAuthority") &&
    drawing.includes("DrawingMutationDecodeAuthority") &&
    drawing.includes("OwnedSchemaHexAuthority<DRAWING_OWNED_FIELD_BYTES>") &&
    drawing.includes("const DRAWING_MAXIMUM_NESTED_ITEMS: usize = 4_096") &&
    drawing.includes("const DRAWING_MAXIMUM_LAYER_DEPTH: usize = 64") &&
    drawing.includes("struct DrawingSnapshotBoundsAuthority") &&
    drawing.includes("struct DrawingFixedOwnerCensus") &&
    drawing.includes("slots: [DrawingOwnerCreditSlot; DRAWING_MAXIMUM_NESTED_ITEMS]") &&
    drawing.includes("const DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY: usize = 16") &&
    drawing.includes("const DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY: usize = 64") &&
    drawing.includes("const DRAWING_MUTATION_ARENA_POOL_CAPACITY: usize = 4") &&
    drawing.includes("struct DrawingMutationArenaOwner") &&
    drawing.includes("struct DrawingMutationArenaPool {") &&
    drawing.includes("DrawingMutationArenaProcessState::Building") &&
    drawing.includes("#[cfg(test)]\n    fn try_new()") &&
    drawing.includes("struct DrawingMutationArenaOwnerBuilder") &&
    drawing.includes("struct DrawingMutationArenaPoolBootstrap") &&
    drawing.includes("struct DrawingMutationArenaBootstrapJob") &&
    drawing.includes("fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingMutationArenaBootstrapStep") &&
    drawing.includes("DrawingMutationArenaProcessState::Inert") &&
    drawing.includes("DrawingMutationArenaBootstrapAdmission::fixed()") &&
    drawing.includes("DrawingMutationArenaBootstrapStep::Blocked") &&
    governedBootstrapArm &&
    exactSourceUndo &&
    requestBlock.includes("DrawingMutationArenaProcessState::Inert") &&
    requestBlock.includes("DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED.store(true") &&
    !requestBlock.includes("bootstrap.step(") &&
    !requestBlock.includes("try_reserve_exact") &&
    !requestBlock.includes("DrawingMutationArenaPoolBootstrap::production") &&
    borrowBlock.includes("DrawingMutationArenaBorrowError::NotReady") &&
    borrowBlock.includes("request_drawing_mutation_arena_pool()") &&
    !borrowBlock.includes("bootstrap.step(") &&
    !borrowBlock.includes("try_reserve_exact") &&
    !borrowBlock.includes("DrawingMutationArenaPoolBootstrap::production") &&
    drawing.includes("DrawingStoreInitializationPhase::InitializeArena") &&
    drawing.includes("DrawingMutationArenaProcessTransition::Retire") &&
    drawing.includes("drawing_mutation_arena_pool_fault") &&
    drawing.includes("borrow_drawing_mutation_arena_from") &&
    drawing.includes("drawing-store.mutation-arena-pool-saturated") &&
    drawing.includes("drawing-store.mutation-arena-stale-generation") &&
    drawing.includes("owner.terminal_is_empty()") &&
    drawing.includes("arena_return_phase: u8") &&
    drawing.includes("Ok(Some(false)) => return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })") &&
    drawing.includes("reverse.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY)") &&
    drawing.includes("pages.try_reserve_exact(DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY)") &&
    drawing.includes("page.try_reserve_exact(DRAWING_MUTATION_RETAINED_PAGE_BYTES)") &&
    drawing.includes("total.checked_add(page.capacity())") &&
    drawing.includes("fn write_overlay_string") &&
    drawing.includes("target.capacity() < source.len()") &&
    drawing.includes("struct DrawingMutationOverlayPatch") &&
    drawing.includes("drawing-store.mutation-overlay-owner-changed") &&
    drawing.includes("struct DrawingAssetBoundsCursor") &&
    drawing.includes("assets.range::<str, _>") &&
    drawing.includes("drawing-store.preflight-depth-capacity") &&
    drawing.includes("drawing-store.preflight-item-capacity") &&
    drawing.includes("drawing-store.preflight-byte-capacity") &&
    drawing.includes("struct DrawingLayerCloneAuthority") &&
    drawing.includes("self.bounds.step(source, cx)") &&
    drawing.includes("layer.step(source.layers.get(self.index)") &&
    drawing.includes("target.children.push(Self::skeleton(child)?)") &&
    drawing.includes("struct DrawingStoreInitializationAuthority") &&
    drawing.includes("struct DrawingSemanticDigestCredit") &&
    drawing.includes("semantic: Option<semio_framework_hash::Sha256>") &&
    drawing.includes('digest.observe(b"drawing.semantic.sha256")') &&
    drawing.includes("struct DrawingFillDigestAuthority") &&
    drawing.includes("struct DrawingStrokeDigestAuthority") &&
    drawing.includes("struct DrawingPathSegmentDigestAuthority") &&
    drawing.includes("struct DrawingLayerVariantDigestAuthority") &&
    drawing.includes("struct DrawingLayerDigestAuthority") &&
    drawing.includes("struct DrawingMutationDigestAuthority") &&
    drawing.includes("struct DrawingMutationAggregateReservation") &&
    drawing.includes("mutation_source_items") &&
    drawing.includes("mutation_derived_items") &&
    drawing.includes("duplicate_candidate_items") &&
    drawing.includes("source_owner_bytes") &&
    drawing.includes("derived_owner_bytes") &&
    drawing.includes("DRAWING_MUTATION_AGGREGATE_ITEMS") &&
    drawing.includes("DRAWING_MUTATION_AGGREGATE_BYTES") &&
    drawing.includes("DRAWING_MUTATION_RETAINED_PAGE_ITEMS") &&
    drawing.includes("DRAWING_MUTATION_RETAINED_PAGE_BYTES") &&
    drawing.includes("drawing-store.mutation-aggregate-item-capacity") &&
    drawing.includes("drawing-store.mutation-aggregate-byte-capacity") &&
    drawing.includes("struct DrawingMutationCandidateAuthority") &&
    drawing.includes("drawing-store.mutation-candidate-stale-authority") &&
    drawing.includes("drawing-store.mutation-candidate-cancelled") &&
    drawing.includes("struct DrawingDuplicateRewriteAuthority") &&
    drawing.includes("material: [u8; DRAWING_DUPLICATE_MATERIAL_BYTES]") &&
    drawing.includes("self.hash_cursor + DRAWING_OWNED_FIELD_BYTES") &&
    drawing.includes('update(b"semio.drawing.duplicate-id.v1")') &&
    drawing.includes("update(&(self.id_len as u64).to_be_bytes())") &&
    drawing.includes("update(&(self.name_len as u64).to_be_bytes())") &&
    drawing.includes("pending_name: std::mem::ManuallyDrop<Option<String>>") &&
    drawing.includes("drawing-store.duplicate-destination-capacity") &&
    drawing.includes("DrawingMutationCandidatePhase::PreflightSource") &&
    drawing.includes("DrawingMutationCandidatePhase::PreflightMutation") &&
    drawing.includes("struct DrawingContainerRebuildAuthority") &&
    drawing.includes("const DRAWING_CONTAINER_REBUILD_MOVE_CAPACITY") &&
    drawing.includes("enum DrawingContainerRebuildMove") &&
    drawing.includes("rebuild.rollback_step()?") &&
    drawing.includes("DrawingContainerRebuildRole::CloseSourceUndo") &&
    drawing.includes("source_undo: Option<DrawingContainerSourceUndo>") &&
    drawing.includes("source.capacity() < output_capacity") &&
    drawing.includes("self.source.as_mut().ok_or(\"drawing-store.container-source\")?.push(value)") &&
    drawing.includes("DrawingStoreInitializationPhase::ValidateEditId") &&
    drawing.includes("DrawingStoreInitializationPhase::ValidateEditMeta") &&
    drawing.includes("DrawingStoreInitializationPhase::PrepareApplied") &&
    drawing.includes("DrawingStoreInitializationPhase::PrepareRedo") &&
    drawing.includes("impl semio_framework_plugin::ArtifactStoreInitializationAuthority<DrawingSnapshot, DrawingMutation> for DrawingStoreInitializationAuthority") &&
    drawing.includes("DrawingStoreInitializationPhase::ValidateEdit { index }") &&
    drawing.includes("self.edit_index.admit(&envelope.vcs.edits, index,") &&
    drawing.includes("DrawingStoreInitializationPhase::SeedHistory") &&
    drawing.includes("DrawingStoreInitializationPhase::BuildCandidate") &&
    drawing.includes("ArtifactStore::from_initialized_runtime_with_owners") &&
    drawing.includes("ArtifactStoreInitializationOwnerCatalog::try_new()") &&
    drawing.includes("ArtifactStoreInitializationRuntime::new_with_owner_catalog") &&
    drawing.includes("DrawingStoreInitializationPhase::MoveInitialOwner") &&
    drawing.includes("self.generation.0.checked_add(1)") &&
    owners.every((owner) => drawing.includes(owner)) &&
    digestFields.every((field) => drawing.includes(field)) &&
    variants.every((variant) => drawing.includes(variant)) &&
    drawing.includes("retained_drawing_mutation_candidate_covers_all_fourteen_variants_and_returns_exact_owners") &&
    drawing.includes("retained_drawing_depth_plus_one_and_hostile_fields_fault_then_close_terminal_empty") &&
    drawing.includes("retained_drawing_container_false_terminal_saturation_and_interrupted_close_preserve_exact_owner") &&
    drawing.includes("retained_drawing_schema_digest_distinguishes_every_nested_semantic_field") &&
    drawing.includes("retained_drawing_aggregate_credit_admits_exact_4096_rejects_plus_one_with_owner_handback") &&
    drawing.includes("retained_drawing_duplicate_hash_frames_domain_id_and_name_lengths_without_concatenation_collision") &&
    drawing.includes("retained_drawing_process_arena_pool_cap_plus_one_returns_exact_slots_and_rejects_stale_aba") &&
    drawing.includes("retained_drawing_duplicate_name_uses_preadmitted_page_and_returns_exact_rejection_owner") &&
    drawing.includes("retained_drawing_cancel_stale_each_replay_candidate_container_stage_preserves_last_valid") &&
    drawing.includes("retained_drawing_rebuild_fault_after_every_phase_rolls_back_exact_container_and_reuses_pool_slot") &&
    drawing.includes("retained_drawing_reorder_fault_after_source_handoff_restores_exact_nested_fifo_and_pool_roots") &&
    drawing.includes("retained_drawing_arena_bootstrap_failure_at_each_allocation_retires_one_exact_root_per_grant") &&
    drawing.includes("retained_drawing_arena_bootstrap_failure_after_each_bundle_keeps_every_root_until_terminal_close") &&
    drawing.includes("retained_drawing_arena_bootstrap_advances_one_allocation_per_turn_and_withholds_incomplete_pool") &&
    drawing.includes("retained_drawing_arena_bootstrap_exact_cap_and_plus_one_rejection_preserve_every_owner_until_close") &&
    drawing.includes("retained_drawing_arena_default_second_app_and_borrow_only_request_without_allocation") &&
    drawing.includes("retained_drawing_arena_bootstrap_job_cancel_budget_contention_and_saturation_are_governed") &&
    !drawing.includes("source.clone()") &&
    !drawing.includes("snapshot.clone()") &&
    !drawing.includes("operation.encode_op()") &&
    !drawing.includes("operation.diff(") &&
    !drawing.includes("diff.apply(") &&
    !drawing.includes("serde_json::from_value") &&
    !drawing.includes("serde_json::from_str") &&
    !drawing.includes("serde_json::to_vec") &&
    !drawing.includes("mutation_meta.iter().any") &&
    !drawing.includes("exact_for_test") &&
    !drawing.includes("iter().nth") &&
    !drawing.includes("structural_copies") &&
    !drawing.includes("DRAWING_ASSET_MAP_NODE_POINTERS") &&
    !drawing.includes("DrawingStoreInitializationPhase::CloneInitial") &&
    !drawing.includes("base.name.try_reserve_exact") &&
    !drawing.includes("DrawingMutationArenaOwner::try_new()?") &&
    !drawing.includes("rposition(Option::is_some)") &&
    !store.includes("id.clone(), edit_digest") &&
    !drawing.includes("drop(value.layers)") &&
    editor.includes("drawing_document_store_initialization_job(envelope, operation, generation)") &&
    editorTestLaws.includes("drawing_live_envelope_submit_recursive_clone_swap_displaced_store_and_exact_ack_succeed") &&
    editorTestLaws.includes("drawing_live_envelope_cancel_closes_retained_pages_without_publication") &&
    editorTestLaws.includes("drawing_live_envelope_rejects_single_and_final_edit_id_plus_one_before_mutation_candidate") &&
    editorTestLaws.includes("drawing_live_initializer_candidate_container_commit_ack_cancel_stale_preserve_last_valid_and_exact_handle") &&
    editor.includes("arena_boot_fault: Option<&'static str>") &&
    editor.includes("drawing_mutation_arena_pool_fault") &&
    editorTestLaws.includes('"forwards": [crate::artifacts::drawing::mutations::DrawingMutation::RenameLayer') &&
    editor.includes("pub struct DrawingEnvelopeLoadHandle") &&
    editor.includes("source: &js_sys::Uint8Array") &&
    editor.includes("begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)") &&
    editor.includes("admit_artifact_envelope_ingress_page(handle.runtime_handle(), page)") &&
    editor.includes("seal_artifact_envelope_ingress(handle.runtime_handle())") &&
    editor.includes("app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
    editor.includes("advance_artifact_envelope_load(handle.runtime_handle())") &&
    editor.includes("acknowledge_artifact_store_replacement(handle.runtime_handle())") &&
    editor.includes("cancel_artifact_envelope_load(handle.runtime_handle())") &&
    editor.includes("request_drawing_mutation_arena_pool()") &&
    defaultBlock.includes("request_drawing_mutation_arena_pool()") &&
    !defaultBlock.includes("bootstrap.step(") &&
    !defaultBlock.includes("try_reserve_exact") &&
    !defaultBlock.includes("initialize_drawing_mutation_arena_pool") &&
    plugin.includes("envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>") &&
    toolJobStoreInitializerRetainedExact(plugin) &&
    !editor.includes("reject_whole_buffer_artifact_envelope_ingress") &&
    !editor.includes("envelope_json: Option<String>") &&
    !editor.includes("while let") &&
    !editor.includes("loop {")
  );
}

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
