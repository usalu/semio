/** 📤️ The retained, ephemeral, fresh turn, host only, config and presence publication contract checks. */
import { toolJobPublicationContracts, toolJobPublicationAuthorityReady, toolJobStoreBatchPublicationBounded, toolJobEphemeralOneItemPublicationBounded, toolJobPublicationFreshnessBeforeEveryTurn } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/⚖️laws/🟦️.ts";

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
  pub fn begin_close(&mut self) {}
  pub fn close_step(&mut self, grant: RetainedCloneGrant) { let child = RetainedCloneGrant { maximum_items: 1, ..grant }; if grant.maximum_copy_bytes > 0 && grant.maximum_capacity_bytes > 0 && grant.maximum_release_bytes > 0 { self.maximum_items = child.maximum_items; } }
  pub fn terminal_is_empty(&self) -> bool { self.phase == ArtifactStoreOneItemPublicationPhase::Complete && self.marker.is_empty() && self.maximum_items == 0 && self.maximum_bytes == 0 && self.generation == 0 }
}
impl<P, Mutation> Drop for ArtifactStoreBatchPublication<P, Mutation> { fn drop(&mut self) { assert!(self.terminal_is_empty()); } }
impl<P, Mutation> ArtifactStore<P, Mutation> {
  pub fn admit_apply_batch(&self, mutations: &mut Vec<Mutation>, grant: RetainedCloneGrant) -> Option<ArtifactStoreBatchPublication<P, Mutation>> { let original = actor.as_ref().filter(|owner| owner.original_is_untouched()); if grant.maximum_copy_bytes<copied_bytes { return None; } let source = std::mem::take(mutations); Some(ArtifactStoreBatchPublication { generation, maximum_items, maximum_bytes, marker: PhantomData }) }
  fn batch_item_base<P, Mutation>(&self) -> SnapshotRead<P> { self.snapshot_read() }
  pub fn advance_apply_batch<P, Mutation>(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>) { publication.generation += 1; }
  pub fn cancel_apply_batch(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>) { publication.begin_close(); }
  pub fn fold_batch_item(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>) { authority.validate_prepared(candidate); let item = original.inverse.pop(); target.inverse.push_reserved(item); publication.staged += 1; }
}`;
  const retainedPluginPublication = `
struct Host {
  artifact_one_item_factory: Option<ArtifactFactory>,
  config_one_item_factory: Option<ConfigFactory>,
  draft_one_item_factory: Option<DraftFactory>,
  unsupported_publication_contracts: Contracts,
}
struct MountedTypedCommandFullOperation<A> { pending_artifact_publication: Option<ArtifactStoreBatchPublication<A::Snapshot, A::Mutation>> }
fn publish_mounted_typed_operation_unit() {
  if let Some(publication) = self.stores.store.admit_apply_batch(operation, generation, revision, &mut mounted.actor_capture, &mut emit.artifact_mutations, HistoryLane::Document, self.stores.artifact_one_item_factory.as_ref(), &mut emit.transaction, Some(&mounted.verb), outbound, retained).map_err(fault)? { mounted.pending_artifact_publication = Some(publication); }
  self.stores.config_store.admit_apply_batch(operation, generation, revision, &mut mounted.actor_capture, &mut emit.config_mutations, HistoryLane::Document, self.stores.config_one_item_factory.as_ref(), &mut transaction, Some(&mounted.verb), false, retained).map_err(fault)?;
  self.stores.draft_store.admit_apply_batch(operation, generation, revision, &mut mounted.actor_capture, &mut emit.draft_mutations, HistoryLane::Document, self.stores.draft_one_item_factory.as_ref(), &mut transaction, Some(&mounted.verb), false, retained).map_err(fault)?;
  self.stores.store.advance_apply_batch(mounted.pending_artifact_publication.as_mut().unwrap(), grant).map_err(fault)?;
}`;
  if (!toolJobStoreBatchPublicationBounded(retainedStorePublication, retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test retained Store batched publication was falsely rejected.");
  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("pub trait ArtifactStoreOneItemPreparationFactory {}", ""), retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test Store publication without a domain preparation factory was falsely accepted.");
  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("publication.generation += 1;", "apply_command(publication);"), retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test Store publication that restores whole apply_command was falsely accepted.");
  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("publication.generation += 1;", "cursor.try_reserve_exact(total); publication.generation += 1;"), retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test Store publication that hides whole-vector growth in a phase was falsely accepted.");
  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("pub fn terminal_is_empty", "pub fn terminal_owner_remains"), retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test Store publication without terminal-empty witness was falsely accepted.");
  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("impl<P, Mutation> Drop for ArtifactStoreBatchPublication", "impl<P, Mutation> Closed for ArtifactStoreBatchPublication"), retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test Store publication without terminal Drop enforcement was falsely accepted.");
  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("self.maximum_items = child.maximum_items;", "owners.clear(); self.maximum_items = child.maximum_items;"), retainedPluginPublication))
    throw new Error("[verify interactivity tool-jobs] self-test Store publication with whole-vector close was falsely accepted.");
  if (toolJobStoreBatchPublicationBounded(retainedStorePublication, retainedPluginPublication.replace("advance_apply_batch", "apply_one")))
    throw new Error("[verify interactivity tool-jobs] self-test plugin publisher that restores monolithic apply_one was falsely accepted.");
  if (toolJobStoreBatchPublicationBounded(retainedStorePublication, retainedPluginPublication.replace("self.stores.artifact_one_item_factory.as_ref()", "A::build_artifact_store_one_item_preparation_factory()")))
    throw new Error("[verify interactivity tool-jobs] self-test per-publication durable factory reconstruction was falsely accepted.");
  if (toolJobStoreBatchPublicationBounded(retainedStorePublication, retainedPluginPublication.replace("&mut emit.artifact_mutations,", "&mut emit.artifact_mutations.clone(),")))
    throw new Error("[verify interactivity tool-jobs] self-test batched publisher that clones instead of draining its whole gesture lane was falsely accepted.");
  if (toolJobStoreBatchPublicationBounded(retainedStorePublication.replace("let item = original.inverse.pop(); target.inverse.push_reserved(item);", ""), retainedPluginPublication))
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
fn qualified_tool_proof(&self, verb: &str) { if let Some(lane) = stores.unsupported_publication_contracts.get(verb) { reject(lane); } }
fn publish_mounted_typed_operation_unit() {
  PendingArtifactStorePublication::Presence;
  PendingArtifactStorePublication::Transient;
  self.stores.presence_one_item_factory.as_deref();
  self.stores.transient_one_item_factory.as_deref();
  self.stores.presence_local_root_retirement_factory.clone();
  self.stores.transient_local_root_retirement_factory.clone();
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
  if (toolJobEphemeralOneItemPublicationBounded(retainedEphemeralPublication, retainedEphemeralPublisher.replace("self.stores.presence_one_item_factory.as_deref();", "A::build_presence_store_one_item_preparation_factory();")))
    throw new Error("[verify interactivity tool-jobs] self-test per-publication Presence factory reconstruction was falsely accepted.");
}

/** 🔔 Checks publication freshness before every turn and the host only, config and presence publication contracts. */
export function freshTurnAndHostOnlyPublicationChecks() {
  const freshPublicationTurn = `
fn publish_mounted_typed_operation_unit() {
  let live_revision = self.stores.store.content_revision_now();
  validate_commit(&mounted.operation, live_revision);
  if mounted.canonical_revision != live_revision { return Err(stale); }
  if let Some(pending) = mounted.pending_artifact_publication.as_mut() { advance(pending); }
  self.stores.store.admit_apply_batch();
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
fn publish_mounted_typed_operation_unit() {
  if !typed_operation_document_is_fresh(&mounted.operation, mounted.canonical_revision, live_revision, live_generation) { return Err(stale); }
  if let Some(pending) = mounted.pending_artifact_publication.as_mut() { advance(pending); }
  self.stores.store.admit_apply_batch();
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
