/** 📤️ The retained, ephemeral, fresh turn, host only, config and presence publication contract checks. */
import { toolJobPublicationContracts, toolJobPublicationAuthorityReady } from "../../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
import { toolJobStoreBatchPublicationBounded, toolJobEphemeralOneItemPublicationBounded, toolJobPublicationFreshnessBeforeEveryTurn } from "../../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📤️publication/🟦️.ts";

/** 📰 Checks the retained store and plugin publication bounds. */
export function retainedPublicationChecks() {
  const retainedStorePublication = `
pub trait ArtifactStoreOneItemPreparationFactory {}
pub trait ArtifactStoreOneItemPreparation {}
pub struct ArtifactStoreOneItemPrepared;
trait ArtifactStoreBatchSource {}
pub enum ArtifactStoreOneItemPublicationPhase { Prepare, Commit, Outbound, Retiring, Complete }
pub struct ArtifactStoreBatchPublication<P, Mutation> { generation: u64, maximum_items: usize, maximum_bytes: usize, marker: PhantomData<(P, Mutation)> }
impl<P, Mutation> ArtifactStoreBatchPublication<P, Mutation> {
  fn begin_close(&mut self) {}
  fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) { let items = maximum_items.min(1); if items > 0 && maximum_bytes > 0 { self.maximum_items = items; } }
  fn terminal_is_empty(&self) -> bool { self.phase == ArtifactStoreOneItemPublicationPhase::Complete && self.marker.is_empty() && self.maximum_items == 0 && self.maximum_bytes == 0 && self.generation == 0 }
}
impl<P, Mutation> Drop for ArtifactStoreBatchPublication<P, Mutation> { fn drop(&mut self) { assert!(self.terminal_is_empty()); } }
impl<P, Mutation> ArtifactStore<P, Mutation> {
  fn begin_apply_batch(&mut self) -> ArtifactStoreBatchPublication<P, Mutation> { self.begin_apply_batch_owned() }
  fn begin_apply_batch_owned(&mut self) -> ArtifactStoreBatchPublication<P, Mutation> { let footprint = source.footprint(lane); ArtifactStoreBatchPublication { generation, maximum_items, maximum_bytes, marker: PhantomData } }
  fn batch_item_base(&self) -> SnapshotRead<P> { self.snapshot_read() }
  fn advance_apply_batch(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>) { publication.generation += 1; }
  fn cancel_apply_batch(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>) { publication.begin_close(); }
  fn fold_batch_item(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>) { authority.validate_prepared(candidate); inverse.reverse(); publication.staged += 1; }
}`;
  const retainedPluginPublication = `
struct Host {
  artifact_one_item_factory: Option<ArtifactFactory>,
  config_one_item_factory: Option<ConfigFactory>,
  draft_one_item_factory: Option<DraftFactory>,
  unsupported_publication_contracts: Contracts,
}
struct MountedTypedCommandFullOperation<A> { pending_artifact_publication: Option<ArtifactStoreBatchPublication<A::Snapshot, A::Mutation>> }
async fn publish_mounted_typed_operation_unit() {
  self.artifact_one_item_factory.as_ref();
  self.config_one_item_factory.as_ref();
  self.draft_one_item_factory.as_ref();
  mounted.pending_artifact_publication = Some(self.store.begin_apply_batch(std::mem::take(&mut emit.artifact_mutations)));
  self.config_store.begin_apply_batch(std::mem::take(&mut emit.config_mutations));
  self.draft_store.begin_apply_batch(std::mem::take(&mut emit.draft_mutations));
  self.store.advance_apply_batch(mounted.pending_artifact_publication.as_mut().unwrap());
}`;
  if (!toolJobStoreBatchPublicationBounded(retainedStorePublication, retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test retained Store batched publication was falsely rejected.");
  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("pub trait ArtifactStoreOneItemPreparationFactory {}", ""), retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test Store publication without a domain preparation factory was falsely accepted.");
  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("publication.generation += 1;", "apply_command(publication);"), retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test Store publication that restores whole apply_command was falsely accepted.");
  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("publication.generation += 1;", "cursor.try_reserve_exact(total); publication.generation += 1;"), retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test Store publication that hides whole-vector growth in a phase was falsely accepted.");
  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("fn terminal_is_empty", "fn terminal_owner_remains"), retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test Store publication without terminal-empty witness was falsely accepted.");
  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("impl<P, Mutation> Drop for ArtifactStoreBatchPublication", "impl<P, Mutation> Closed for ArtifactStoreBatchPublication"), retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test Store publication without terminal Drop enforcement was falsely accepted.");
  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("self.maximum_items = items;", "owners.clear(); self.maximum_items = items;"), retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test Store publication with whole-vector close was falsely accepted.");
  if (toolJobStoreBatchPublicationBounded(retainedStorePublication, retainedPluginPublication.replace("advance_apply_batch", "apply_one")))
    throw new Error("[verify interactivity tool-jobs] self-test plugin publisher that restores monolithic apply_one was falsely accepted.");
  if (toolJobStoreBatchPublicationBounded(retainedStorePublication, retainedPluginPublication.replace("self.artifact_one_item_factory.as_ref();", "A::build_artifact_store_one_item_preparation_factory();")))
    throw new Error("[verify interactivity tool-jobs] self-test per-publication durable factory reconstruction was falsely accepted.");
  if (toolJobStoreBatchPublicationBounded(retainedStorePublication, retainedPluginPublication.replace("std::mem::take(&mut emit.artifact_mutations)", "emit.artifact_mutations.last().cloned()")))
    throw new Error("[verify interactivity tool-jobs] self-test batched publisher that clones instead of draining its whole gesture lane was falsely accepted.");
  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("inverse.reverse();", ""), retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test batched fold without the replay_mutations inverse ordering was falsely accepted.");
}

/** 💨 Checks the retained ephemeral one item publication and publisher bounds. */
export function ephemeralPublicationChecks() {
  const retainedEphemeralPublication = `
pub trait ArtifactEphemeralOneItemPreparationFactory {}
pub trait ArtifactEphemeralOneItemPreparation {}
pub struct ArtifactEphemeralOneItemPrepared;
pub struct ArtifactEphemeralOneItemPublication<P, Mutation>(PhantomData<(P, Mutation)>);
impl<P, Mutation> ArtifactEphemeralOneItemPublication<P, Mutation> {
  fn close_step(&mut self) { let released = 1; }
  fn terminal_is_empty(&self) -> bool { true }
}
impl<P, Mutation> Drop for ArtifactEphemeralOneItemPublication<P, Mutation> { fn drop(&mut self) { assert!(self.terminal_is_empty()); } }
impl<P, Mutation> PresenceStore<P, Mutation> {
  pub fn begin_publish_one(&self) {}
  pub fn advance_publish_one(&mut self) { let retirement = previous; publication = retirement; }
  pub fn cancel_publish_one(&mut self) {}
}
impl<P, Mutation> TransientStore<P, Mutation> {
  pub fn begin_publish_one(&self) {}
  pub fn advance_publish_one(&mut self) { let retirement = previous; publication = retirement; }
  pub fn cancel_publish_one(&mut self) {}
}`;
  const retainedEphemeralPublisher = `
struct Host {
  presence_one_item_factory: Option<PresenceFactory>,
  transient_one_item_factory: Option<TransientFactory>,
  presence_local_root_retirement_factory: Option<PresenceRetirementFactory>,
  transient_local_root_retirement_factory: Option<TransientRetirementFactory>,
  unsupported_publication_contracts: Contracts,
}
fn qualified_tool_proof(&self, verb: &str) { if let Some(lane) = self.unsupported_publication_contracts.get(verb) { reject(lane); } }
async fn publish_mounted_typed_operation_unit() {
  PendingArtifactStorePublication::Presence;
  PendingArtifactStorePublication::Transient;
  self.presence_one_item_factory.as_deref();
  self.transient_one_item_factory.as_deref();
  self.presence_local_root_retirement_factory.clone();
  self.transient_local_root_retirement_factory.clone();
  presence_store.begin_publish_one();
  presence_store.advance_publish_one();
  transient_store.begin_publish_one();
  transient_store.advance_publish_one();
}`;
  if (!toolJobEphemeralOneItemPublicationBounded(retainedEphemeralPublication, retainedEphemeralPublisher))
    throw new Error("[verify interactivity tool-jobs] self-test retained Presence/Transient one-item publication was falsely rejected.");
  if (toolJobEphemeralOneItemPublicationBounded(retainedEphemeralPublication.replace("let retirement = previous;", "drop(previous);"), retainedEphemeralPublisher))
    throw new Error("[verify interactivity tool-jobs] self-test immediate ephemeral displaced-root drop was falsely accepted.");
  if (toolJobEphemeralOneItemPublicationBounded(retainedEphemeralPublication, retainedEphemeralPublisher.replace("transient_store.advance_publish_one();", "transient_store.apply_one();")))
    throw new Error("[verify interactivity tool-jobs] self-test monolithic Transient publication was falsely accepted.");
  if (toolJobEphemeralOneItemPublicationBounded(retainedEphemeralPublication, retainedEphemeralPublisher.replace("self.presence_one_item_factory.as_deref();", "A::build_presence_store_one_item_preparation_factory();")))
    throw new Error("[verify interactivity tool-jobs] self-test per-publication Presence factory reconstruction was falsely accepted.");
}

/** 🔔 Checks publication freshness before every turn and the host only, config and presence publication contracts. */
export function freshTurnAndHostOnlyPublicationChecks() {
  const freshPublicationTurn = `
async fn publish_mounted_typed_operation_unit() {
  let live_revision = self.store.content_revision_now();
  validate_commit(&mounted.operation, live_revision);
  if mounted.canonical_revision != live_revision { return Err(stale); }
  if let Some(pending) = mounted.pending_artifact_publication.as_mut() { advance(pending); }
  self.store.begin_apply_batch();
}`;
  if (!toolJobPublicationFreshnessBeforeEveryTurn(freshPublicationTurn))
    throw new Error("[verify interactivity tool-jobs] self-test publication freshness before every resumed or new turn was falsely rejected.");
  const staleResumedPublicationTurn = freshPublicationTurn
    .replace("  validate_commit(&mounted.operation, live_revision);\n", "")
    .replace(
      "  if let Some(pending) = mounted.pending_artifact_publication.as_mut() { advance(pending); }",
      "  if let Some(pending) = mounted.pending_artifact_publication.as_mut() { advance(pending); }\n  validate_commit(&mounted.operation, live_revision);",
    );
  if (toolJobPublicationFreshnessBeforeEveryTurn(staleResumedPublicationTurn))
    throw new Error("[verify interactivity tool-jobs] self-test resumed publication advanced before freshness validation was falsely accepted.");
  if (toolJobPublicationFreshnessBeforeEveryTurn(freshPublicationTurn.replace("mounted.canonical_revision != live_revision", "false")))
    throw new Error("[verify interactivity tool-jobs] self-test publication freshness without canonical/live revision comparison was falsely accepted.");
  const helperFreshPublicationTurn = `
fn typed_operation_document_is_fresh(operation: &Operation, canonical_revision: Revision, live_revision: Revision, live_generation: u64) -> bool {
  canonical_revision == live_revision && matches!(validate_commit(operation, live_revision, Generation(live_generation)), CommitValidation::Accepted)
}
async fn publish_mounted_typed_operation_unit() {
  if !typed_operation_document_is_fresh(&mounted.operation, mounted.canonical_revision, live_revision, live_generation) { return Err(stale); }
  if let Some(pending) = mounted.pending_artifact_publication.as_mut() { advance(pending); }
  self.store.begin_apply_batch();
}`;
  if (!toolJobPublicationFreshnessBeforeEveryTurn(helperFreshPublicationTurn))
    throw new Error("[verify interactivity tool-jobs] self-test exact extracted publication freshness guard was falsely rejected.");
  if (toolJobPublicationFreshnessBeforeEveryTurn(helperFreshPublicationTurn.replace("validate_commit(operation", "accept_without_validation(operation")))
    throw new Error("[verify interactivity tool-jobs] self-test extracted publication freshness guard without commit validation was falsely accepted.");
  const hostOnlyPublication = `const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "setView", lanes: &[ArtifactToolPublicationLane::HostOnly] }];`;
  const hostOnlyContracts = toolJobPublicationContracts(hostOnlyPublication, hostOnlyPublication, new Map());
  if (hostOnlyContracts?.get("setView")?.join(",") !== "HostOnly" || !toolJobPublicationAuthorityReady("", hostOnlyContracts.get("setView")!))
    throw new Error("[verify interactivity tool-jobs] self-test exact host-only publication contract was falsely rejected.");
  const namedHostOnlyPublication = `const VIEW_PUBLICATIONS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "setView", lanes: &[ArtifactToolPublicationLane::HostOnly] }]; const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = VIEW_PUBLICATIONS;`;
  if (toolJobPublicationContracts(namedHostOnlyPublication, namedHostOnlyPublication.slice(namedHostOnlyPublication.indexOf("const PUBLICATION_CONTRACTS")), new Map())?.get("setView")?.join(",") !== "HostOnly")
    throw new Error("[verify interactivity tool-jobs] self-test named exact publication contract was falsely rejected.");
  const mixedHostPublication = hostOnlyPublication.replace("ArtifactToolPublicationLane::HostOnly]", "ArtifactToolPublicationLane::HostOnly, ArtifactToolPublicationLane::Config]");
  if (toolJobPublicationContracts(mixedHostPublication, mixedHostPublication, new Map()))
    throw new Error("[verify interactivity tool-jobs] self-test mixed host-only/Store publication contract was falsely accepted.");
  const configPublication = hostOnlyPublication.replace("setView", "setLocale").replace("HostOnly", "Config");
  const configContracts = toolJobPublicationContracts(configPublication, configPublication, new Map())!;
  if (toolJobPublicationAuthorityReady("", configContracts.get("setLocale")!))
    throw new Error("[verify interactivity tool-jobs] self-test Config publication without an app-owned preparation factory was falsely accepted.");
  const configOwner = "fn build_config_store_one_item_preparation_factory() -> Option<Factory> { Some(Factory::new()) }";
  if (!toolJobPublicationAuthorityReady(configOwner, configContracts.get("setLocale")!))
    throw new Error("[verify interactivity tool-jobs] self-test exact Config publication preparation authority was falsely rejected.");
  const presencePublication = hostOnlyPublication.replace("setView", "setPresence").replace("HostOnly", "Presence");
  const presenceLanes = toolJobPublicationContracts(presencePublication, presencePublication, new Map())!.get("setPresence")!;
  const presencePreparationOnly = "fn build_presence_store_one_item_preparation_factory() -> Option<Factory> { Some(Factory::new()) }";
  if (toolJobPublicationAuthorityReady(presencePreparationOnly, presenceLanes))
    throw new Error("[verify interactivity tool-jobs] self-test Presence publication without displaced-root retirement authority was falsely accepted.");
  const presenceComplete = `${presencePreparationOnly} fn build_presence_local_root_retirement_factory() -> Option<Factory> { Some(Factory::new()) }`;
  if (!toolJobPublicationAuthorityReady(presenceComplete, presenceLanes))
    throw new Error("[verify interactivity tool-jobs] self-test exact Presence preparation and root-retirement authority was falsely rejected.");
}
