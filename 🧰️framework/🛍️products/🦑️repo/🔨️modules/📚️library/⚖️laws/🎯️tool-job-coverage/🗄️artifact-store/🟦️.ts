import { toolJobRustBlock } from "../📋️inventory/🟦️.ts";
import { OS_STORE_ROOT_SOURCES, policyRustProductionSource, type PolicyRustSourceEvidence, policyRustTestSource } from "../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";

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
