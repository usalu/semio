import { expect, test } from "bun:test";
import { join } from "node:path";
import { readFileSync } from "node:fs";
import { WORKSPACE_ROOT } from "../../🗂️workspaces/📍️checkout-root/🟦️.ts";
import { toolJobRustBlock } from "../../⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
import { toolJobImmutableOperationRootsExact } from "../../⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🟦️.ts";
import { toolJobPeerCommitAuthorityExact, toolJobPeerInteractionRootsExact } from "../../⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🏛️structural-owners/🟦️.ts";
import { frameworkSourcePath } from "@semio-tech/framework/test/source-text";
import { readModuleTreeSource, readSourceFamilySource } from "@semio-tech/framework/compiler/syntax/module-tree";

/** 🧱️ One retirement unit a CAD presence lane contributes, or `null` when the lane is absent. */
type CadPresenceRetirementPart = { readonly unit: string; readonly repeat: number } | null;

/** 📐️ `🧫️fixtures/♻️retirement/🔣️.json` — the CAD-owned presence byte laws. */
interface CadPresenceRetirementFixture {
  readonly grant: { readonly maximumItems: number; readonly maximumBytes: number };
  readonly cases: readonly {
    readonly name: string;
    readonly activeUtility?: CadPresenceRetirementPart;
    readonly engagementStep?: CadPresenceRetirementPart;
    readonly engagementPane?: CadPresenceRetirementPart;
    readonly expectedBytes: number;
  }[];
  readonly storeCases: readonly {
    readonly name: string;
    readonly local: string;
    readonly peers: readonly { readonly presence: string; readonly actor: string }[];
    readonly expectedBytes: number;
  }[];
}

/** 🏪️ `🏪️store/👥️presence/🧫️fixtures/🧹️retirement.json` — the framework-side roster ownership laws. */
interface PresenceStoreRetirementFixture {
  readonly maximumItems: number;
  readonly maximumBytes: number;
  readonly cases: readonly { readonly name: string; readonly peers: readonly { readonly actor: string }[]; readonly expectedSnapshots: number; readonly expectedActorBytes: number }[];
  readonly overlap: {
    readonly first: readonly { readonly actor: string; readonly value: number }[];
    readonly second: readonly { readonly actor: string; readonly value: number }[];
    readonly expectedPeerSnapshots: number;
    readonly expectedActorBytes: number;
  };
  readonly localCapture: { readonly value: number; readonly expectedValueWhileOpen: number; readonly expectedWorkerTerminal: boolean };
  readonly localReplacements: { readonly values: readonly number[] };
  readonly closeFactoryBinding: { readonly expectedLocal: number; readonly expectedPeer: number; readonly expectedForeign: number; readonly returnReadAfterDetach: boolean };
  readonly readerReturn: { readonly eventOrder: readonly string[]; readonly contendedTransferPreservesUnreturned: boolean; readonly transferPublishesReturn: boolean };
}

/** 🛂️ `🏪️store/👥️presence/🧫️fixtures/🛂️peer-admission.json` — the peer actor admission byte laws. */
interface PresencePeerAdmissionFixture {
  readonly maximumItems: number;
  readonly maximumBytes: number;
  readonly cases: readonly {
    readonly name: string;
    readonly actor: { readonly unit: string; readonly repeat: number; readonly minimumCapacity: number };
    readonly state: string;
    readonly accepted: boolean;
    readonly expectedActorBytes: number;
  }[];
  readonly factoryBinding: { readonly expectedOriginalRetirements: number; readonly expectedForeignRetirements: number };
  readonly requiresCapacitySizedByteGrant: boolean;
}

/** 📌️ `🏪️store/👥️presence/🧫️fixtures/📌️peer-commit.json` — the peer-commit freshness laws. */
interface PresencePeerCommitFixture {
  readonly maximumItems: number;
  readonly maximumBytes: number;
  readonly cases: readonly { readonly name: string; readonly sameStore: boolean; readonly sameFactory: boolean; readonly stale: boolean; readonly accepted: boolean; readonly expectedSnapshots: number }[];
}

/** 🧹️ Cross-checks CAD domain retirement byte counts independently of its Rust ownership cursor. */
function cadPresenceRetirementChecks(): number {
  const base = join(WORKSPACE_ROOT, "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence");
  const fixture: CadPresenceRetirementFixture = JSON.parse(readFileSync(join(base, "🧪️tests/🧫️fixtures/♻️retirement/🔣️.json"), "utf8"));
  
  
  const counts = new Map<string, number>();
  for (const law of fixture.cases) {
    const bytes = [law.activeUtility, law.engagementStep, law.engagementPane].reduce((sum, part) => sum + (part === null || part === undefined ? 0 : Buffer.byteLength(part.unit.repeat(part.repeat), "utf8")), 0);
    if (counts.has(law.name) || bytes !== law.expectedBytes) throw new Error(`CAD presence byte oracle: ${law.name}`);
    counts.set(law.name, bytes);
  }
  for (const law of fixture.storeCases) {
    if (!counts.has(law.local) || law.peers.some((peer) => !counts.has(peer.presence))) throw new Error(`CAD presence unknown fixture root: ${law.name}`);
    const bytes = counts.get(law.local)! + law.peers.reduce((sum, peer) => sum + counts.get(peer.presence)! + Buffer.byteLength(peer.actor, "utf8"), 0);
    if (bytes !== law.expectedBytes) throw new Error(`CAD presence roster byte oracle: ${law.name}`);
  }
  if (fixture.grant.maximumItems !== 1 || fixture.grant.maximumBytes !== 4096) throw new Error("CAD retirement grant example differs from the one-item native cursor");
  const storeBase = frameworkSourcePath("🛍️products/💻️os/🔨️modules/🏪️store/👥️presence");
  const storeFolder = join(storeBase, "..");
  const storeFixture: PresenceStoreRetirementFixture = JSON.parse(readFileSync(join(storeBase, "🧪️tests/🧫️fixtures/🧹️retirement.json"), "utf8"));
  for (const law of storeFixture.cases) {
    if (law.expectedSnapshots !== law.peers.length + 1 || law.expectedActorBytes !== law.peers.reduce((sum, peer) => sum + Buffer.byteLength(peer.actor, "utf8"), 0)) throw new Error(`presence Store retirement oracle: ${law.name}`);
  }
  const distinctPeers = new Map<string, { actor: string }>([...storeFixture.overlap.first, ...storeFixture.overlap.second].map((peer) => [JSON.stringify(peer), peer]));
  if (distinctPeers.size !== storeFixture.overlap.expectedPeerSnapshots || [...distinctPeers.values()].reduce((sum, peer) => sum + Buffer.byteLength(peer.actor), 0) !== storeFixture.overlap.expectedActorBytes) throw new Error("presence overlapping-roster ownership oracle");
  const readerReturn = storeFixture.readerReturn;
  const readerOwnershipOracle = (value: typeof readerReturn): boolean => {
    let readerAliases = 1, registryAliases = 1, returned = false;
    for (const event of value.eventOrder) {
      if (event === "reader-alias-released") readerAliases--;
      else if (event === "return-published") { if (readerAliases !== 0 || registryAliases !== 1) return false; returned = true; }
      else if (event === "registry-owner-taken") { if (!returned || readerAliases !== 0) return false; }
      else if (event === "final-owner-retired") registryAliases--;
    }
    return readerAliases === 0 && registryAliases === 0 && value.contendedTransferPreservesUnreturned && !value.transferPublishesReturn;
  };
  if (!readerOwnershipOracle(readerReturn)) throw new Error("read ownership oracle retains an alias or publishes premature return");
  for (const hostile of [
    { ...readerReturn, eventOrder: ["return-published", "reader-alias-released", "registry-owner-taken", "final-owner-retired"] },
    { ...readerReturn, contendedTransferPreservesUnreturned: false },
    { ...readerReturn, transferPublishesReturn: true },
  ]) if (readerOwnershipOracle(hostile)) throw new Error("presence read oracle accepted premature return or lost transfer authority");
  const readSource = readFileSync(frameworkSourcePath("🔨️modules/🌱️value/📸️snapshot/👁️read/🦀️.rs"), "utf8");
  const exactReadReturn = (source: string): boolean => {
    const block = (needle: string): string => { const start = source.indexOf(needle); return start < 0 ? "" : toolJobRustBlock(source, source.indexOf("{", start))?.body ?? ""; };
    const release = block("fn return_snapshot_read<T:");
    const transfer = block("fn into_typed<T:");
    const drop = release.indexOf("drop(owner.take());");
    const publish = release.indexOf("lease.return_now()");
    return drop >= 0 && publish > drop && release.indexOf("after_alias_release();") > drop
      && (source.match(/return_snapshot_read\(&mut self\.owner, &mut self\.lease, \|\| \{\}\)/g) ?? []).length === 4
      && transfer.includes("registry.try_take(lease.index, lease.generation)") && !transfer.includes("return_now")
      && transfer.includes("self.owner = Some(owner);") && transfer.includes("self.lease = Some(lease);");
  };
  if (!exactReadReturn(readSource)) throw new Error("opaque snapshot read return/transfer lost its exact final-owner ordering");
  const readHostiles = [
    readSource.replace("drop(owner.take());\n    after_alias_release();", "after_alias_release();"),
    readSource.replace("let guard = match registry.try_take(lease.index, lease.generation)", "let _ = lease.return_now();\n        let guard = match registry.try_take(lease.index, lease.generation)"),
    readSource.replace("            self.lease = Some(lease);", "            drop(lease);"),
  ];
  for (const hostile of readHostiles) if (hostile === readSource || exactReadReturn(hostile)) throw new Error("opaque snapshot read source guard accepted a premature return or lost retained lease");
  const presenceSource = readFileSync(join(storeBase, "🦀️.rs"), "utf8");
  const ephemeralSource = readFileSync(join(storeFolder, "🫧️ephemeral/👥️presence-store-and-peers-publication/🦀️.rs"), "utf8");
  const oneItemSource = readFileSync(join(storeFolder, "🫧️ephemeral/📤️ephemeral-one-item-publication/🦀️.rs"), "utf8");
  const publicationSource = readFileSync(join(storeBase, "🚪️io/📬️publication/🦀️.rs"), "utf8");
  const rejectionSource = readFileSync(join(storeBase, "🚫️rejection/🦀️.rs"), "utf8");
  const retirementSource = readFileSync(join(storeBase, "♻️retirement/🦀️.rs"), "utf8");
  const storeFamily = readSourceFamilySource(storeFolder);
  const peerOwnership = `${presenceSource}\n${ephemeralSource}`;
  const exactPeerRelease = (source: string): boolean => {
    const cursorStart = source.indexOf("impl<P: Send + Sync + 'static> PresencePeersRetirement<P>");
    const cursor = cursorStart < 0 ? "" : toolJobRustBlock(source, source.indexOf("{", cursorStart))?.body ?? "";
    const publicationStart = source.indexOf("impl<P: Send + Sync + 'static> PresencePeersPublication<P>");
    const publication = publicationStart < 0 ? "" : toolJobRustBlock(source, source.indexOf("{", publicationStart))?.body ?? "";
    return cursor.includes("Arc::into_inner(self.waiting.take().unwrap())") && !cursor.includes("Arc::try_unwrap(self.waiting")
      && cursor.includes("Arc::try_unwrap(self.root.take().unwrap())") && cursor.includes("Arc::into_inner(self.root.take().unwrap())") && cursor.includes("self.owned_root.as_mut()")
      && cursor.includes("Arc::weak_count(waiting) != 0") && cursor.includes("self.root_custody == PresencePeerRootCustody::Displaced && Arc::strong_count(root) != 1")
      && source.includes("*retirement.root = Some(previous)") && source.includes("presence peer entry requires exact final-owner retirement")
      && source.includes("actor: std::mem::ManuallyDrop<String>") && source.includes("presence: std::mem::ManuallyDrop<Option<Arc<P>>>")
      && !source.includes("impl<P> Clone for PresencePeersRoot<P>") && !source.includes("pub fn clone_aliases(&self)")
      && (publication.match(/PresencePeersRetirement::new\(PresencePeersRetiredEntries::one/g) ?? []).length === 2;
  };
  if (!exactPeerRelease(peerOwnership)) throw new Error("presence peer roots lost exact shared-entry final-owner retirement");
  const peerHostiles = [
    peerOwnership.replace("if let Some(entry) = Arc::into_inner(self.waiting.take().unwrap())", "if let Some(entry) = Arc::try_unwrap(self.waiting.take().unwrap()).ok()"),
    peerOwnership.replace("*retirement.root = Some(previous)", "drop(previous)"),
    peerOwnership.replace("pub(crate) fn clone_aliases(&self)", "pub fn clone_aliases(&self)"),
    peerOwnership.replace("if Arc::weak_count(waiting) != 0", "if false"),
  ];
  for (const hostile of peerHostiles) if (hostile === peerOwnership || exactPeerRelease(hostile)) throw new Error("presence peer guard accepted shared-owner waiting, implicit root drop, or public owner cloning");
  if (storeFixture.localCapture.expectedValueWhileOpen !== storeFixture.localCapture.value || !storeFixture.localCapture.expectedWorkerTerminal) throw new Error("presence local capture/replacement independent owner ledger");
  const cadSource = readFileSync(join(base, "♻️retirement/🦀️.rs"), "utf8");
  const storeLawSource = readFileSync(frameworkSourcePath("🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌌️bin-04/🔬️unit/🛤️history-lane-interaction-store-and-immutable-root-laws/🦀️.rs"), "utf8");
  const retirementLawSource = readFileSync(join(storeBase, "♻️retirement/🧪️tests/🔬️unit/🦀️.rs"), "utf8");
  const ephemeralLanes = `${ephemeralSource}\n${oneItemSource}`;
  const exactLocal = (family: string, lanes: string, retirement: string, cad: string): boolean =>
    toolJobImmutableOperationRootsExact(family)
    && storeLawSource.includes("artifact_snapshot_root_is_o1_and_generation_stable_until_the_next_event")
    && storeLawSource.includes("presence_local_read_is_o1_and_never_clones_the_payload_at_capture")
    && storeLawSource.includes("transient_root_is_o1_and_retains_the_exact_pre_reset_value")
    && lanes.includes("pub base: ArtifactEphemeralBaseRead<P>")
    && lanes.includes("Presence(SnapshotRead<P>)")
    && lanes.includes("Arc::ptr_eq(installed, &root_retirement_factory)")
    && lanes.includes("previous.return_to_registry()")
    && retirement.includes("advance_returned_local(reads, &mut self.active_returned")
    && retirement.includes("self.active_returned_local.take()")
    && retirement.includes("presence store requires its exact detached terminal-empty owner before Drop")
    && retirement.includes("Err(SnapshotReadLeaseRefusal::Busy) => Ok(RetainedCloneStep::Progress(Default::default()))")
    && retirementLawSource.includes("MountedWorkerJobSession::try_admit_owned(&mut job,&mut params,&mut admission)")
    && retirementLawSource.includes("owner.maintenance_local_reads_step(CLOSE_GRANT)")
    && cad.includes("semio_framework_value::retirement::arc_custody::admit_shared_retirement(source, grant, true)")
    && cad.includes("PresenceStoreOwnedDisposer::new(Arc::new(empty_terminal()), terminal_is_empty)") && !cad.includes("Arc::try_unwrap(root)");
  if (!exactLocal(storeFamily, ephemeralLanes, retirementSource, cadSource)) throw new Error("presence local read, live return, detached close or final-owner authority is incomplete");
  const localHostiles = [
    [storeFamily.replace("pub base: ArtifactEphemeralBaseRead<P>", "pub base: Arc<P>"), ephemeralLanes.replace("pub base: ArtifactEphemeralBaseRead<P>", "pub base: Arc<P>"), retirementSource, cadSource],
    [storeFamily, ephemeralLanes.replaceAll("previous.return_to_registry()", "drop(previous)"), retirementSource, cadSource],
    [storeFamily, ephemeralLanes.replace("Arc::ptr_eq(installed, &root_retirement_factory)", "true"), retirementSource, cadSource],
    [storeFamily, ephemeralLanes, retirementSource.replace("self.active_returned_local.take()", "None"), cadSource],
    [storeFamily, ephemeralLanes, retirementSource, cadSource.replace("admit_shared_retirement(source, grant, true)", "Arc::try_unwrap(source).ok()")],
  ] as const;
  for (const [index, [family, lanes, retirement, cad]] of localHostiles.entries()) if (exactLocal(family, lanes, retirement, cad)) throw new Error(`presence local guard accepted hostile ${index}: a raw alias, foreign factory, lost returned owner or shared wait`);
  const closeBinding = storeFixture.closeFactoryBinding;
  const closeCounts = (["local", "peer"] as const).reduce((counts, lane) => ({ ...counts, [lane]: counts[lane] + 1 }), { local: 0, peer: 0, foreign: 0 });
  if (JSON.stringify(closeCounts) !== JSON.stringify({ local: closeBinding.expectedLocal, peer: closeBinding.expectedPeer, foreign: closeBinding.expectedForeign })) throw new Error("Presence close factory oracle substituted an installed owner");
  const closeSchemaHostiles = [
    { ...storeFixture, closeFactoryBinding: { ...closeBinding, expectedForeign: 2 } },
    { ...storeFixture, closeFactoryBinding: { ...closeBinding, expectedLocal: 0 } },
    { ...storeFixture, closeFactoryBinding: { ...closeBinding, returnReadAfterDetach: false } },
  ];
  for (const hostile of closeSchemaHostiles) if (hostile.closeFactoryBinding.expectedLocal === closeCounts.local && hostile.closeFactoryBinding.expectedPeer === closeCounts.peer && hostile.closeFactoryBinding.expectedForeign === closeCounts.foreign && hostile.closeFactoryBinding.returnReadAfterDetach) throw new Error("Presence close oracle admitted foreign or lost returned-read ownership");
  const exactCloseFactories = (source: string): boolean => {
    const start = source.indexOf("pub fn begin_retirement(");
    const open = source.indexOf("{", start);
    const signature = source.slice(start, open);
    const body = start < 0 ? "" : toolJobRustBlock(source, open)?.body ?? "";
    const closeStart = source.indexOf("pub fn close_step(");
    const close = closeStart < 0 ? "" : toolJobRustBlock(source, source.indexOf("{", closeStart))?.body ?? "";
    return !signature.includes("factory:") && body.includes("if self.local_retirement_factory.is_none()")
      && body.includes("local_factory: self.local_retirement_factory.take()") && body.includes("peer_factory: self.peer_retirement_factory.take()")
      && body.includes("!peers.is_empty() && self.peer_retirement_factory.is_none()")
      && close.includes("advance_returned_local(reads, &mut self.active_returned, self.local_factory.as_ref(), child)")
      && close.includes('self.local_factory.as_ref().expect("observed original local factory").retire(original, child)')
      && close.includes("PresencePeersRetirement::from_root(root, self.peer_factory.clone(), custody)");
  };
  if (!exactCloseFactories(retirementSource)) throw new Error("Presence close lost exact original local/peer factory separation");
  const closeSourceHostiles = [
    retirementSource.replace("pub fn begin_retirement(&mut self, terminal_local: Arc<P>, terminal_is_empty: fn(&P) -> bool)", "pub fn begin_retirement(&mut self, terminal_local: Arc<P>, terminal_is_empty: fn(&P) -> bool, factory: Arc<dyn SnapshotRetirementFactory<P>>)"),
    retirementSource.replace("local_factory: self.local_retirement_factory.take()", "local_factory: self.peer_retirement_factory.take()"),
    retirementSource.replace("peer_factory: self.peer_retirement_factory.take()", "peer_factory: self.local_retirement_factory.take()"),
    retirementSource.replace("advance_returned_local(reads, &mut self.active_returned, self.local_factory.as_ref(), child)", "advance_returned_local(reads, &mut self.active_returned, self.peer_factory.as_ref(), child)"),
    retirementSource.replace("!peers.is_empty() && self.peer_retirement_factory.is_none()", "false"),
  ];
  for (const hostile of closeSourceHostiles) if (hostile === retirementSource || exactCloseFactories(hostile)) throw new Error("Presence close guard admitted factory substitution or wrong returned-read retirement");
  const closeFactoryChecks = 2 + closeSchemaHostiles.length + closeSourceHostiles.length;
  const commitFixture: PresencePeerCommitFixture = JSON.parse(readFileSync(join(storeBase, "🧪️tests/🧫️fixtures/📌️peer-commit.json"), "utf8"));
  for (const law of commitFixture.cases) if (law.accepted !== (law.sameStore && law.sameFactory && !law.stale) || law.expectedSnapshots !== 3 + Number(law.stale)) throw new Error(`Presence peer commit independent identity oracle: ${law.name}`);
  const commitSchemaHostiles = [{ ...commitFixture, maximumBytes: 8192 }, { ...commitFixture, cases: commitFixture.cases.map(law => ({ ...law, accepted: true })) }];
  for (const hostile of commitSchemaHostiles.slice(1)) if (hostile.cases.every(law => law.accepted === (law.sameStore && law.sameFactory && !law.stale))) throw new Error("Presence peer commit oracle admitted forged freshness");
  const exactPeerCommit = toolJobPeerCommitAuthorityExact;
  if (!exactPeerCommit(storeFamily, retirementSource)) throw new Error("Presence peer commit lost exact base/factory join or retained alias handoff");
  const commitSourceHostiles = [
    [storeFamily.replace('!Arc::ptr_eq(self.peers.as_ref().expect("live peer presence owner"), &commit.base_root)', "false"), retirementSource],
    [storeFamily.replace("Arc::ptr_eq(factory, &commit.factory)", "true"), retirementSource],
    [storeFamily.replace("*retirement.root = Some(previous);\n        drop(base_root);", "drop(base_root);\n        *retirement.root = Some(previous);"), retirementSource],
    [storeFamily, retirementSource.replace("base_root: std::mem::ManuallyDrop::new(Some(base_root))", "base_root: std::mem::ManuallyDrop::new(None)")],
    [storeFamily, retirementSource.replace("let custody = if self.base_root.is_some() { PresencePeerRootCustody::BaseAlias } else { PresencePeerRootCustody::Displaced };", "let custody = PresencePeerRootCustody::Displaced;")],
  ];
  for (const [store, retirement] of commitSourceHostiles) if (exactPeerCommit(store, retirement)) throw new Error("Presence peer commit guard admitted foreign/stale publication or lost base ownership");
  const commitChecks = 1 + commitFixture.cases.length + 1 + commitSourceHostiles.length;
  const peerFixture: PresencePeerAdmissionFixture = JSON.parse(readFileSync(join(storeBase, "🧪️tests/🧫️fixtures/🛂️peer-admission.json"), "utf8"));
  for (const law of peerFixture.cases) {
    const bytes = Buffer.byteLength(law.actor.unit.repeat(law.actor.repeat), "utf8");
    if (bytes !== law.expectedActorBytes || law.accepted !== (law.state === "ready" && bytes > 0 && bytes <= 256) || law.actor.minimumCapacity <= peerFixture.maximumBytes) throw new Error(`peer actor admission independent byte oracle: ${law.name}`);
  }
  const pluginPeerSource = readFileSync(join(storeBase, "../../🔌️plugin/👥️peer/🦀️.rs"), "utf8");
  const exactRejectedActor = (publication: string, rejection: string, peer: string): boolean => {
    const start = publication.indexOf("pub fn adopt(&mut self, actor: &mut Option<String>, presence: &mut Option<P>");
    const adopt = start < 0 ? "" : toolJobRustBlock(publication, publication.indexOf("{", start))?.body ?? "";
    const admission = adopt.lastIndexOf("return Err(");
    return admission >= 0
      && (adopt.match(/actor\.take\(\)/g) ?? []).length === 1 && adopt.indexOf("actor.take()") > admission
      && (adopt.match(/presence\.take\(\)/g) ?? []).length === 1 && adopt.indexOf("presence.take()") > admission
      && !publication.includes("pub fn retire_rejected(") && !rejection.includes("PresencePeerAdmissionRejected::new(\"")
      && rejection.includes("pub fn into_retirement(self, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Self)>")
      && rejection.includes("actor: std::mem::ManuallyDrop::new(Some(std::mem::take(&mut *original.actor).into_bytes()))")
      && rejection.includes("RetirementDemand { release_bytes: actor.capacity(), depth: 1, ..Default::default() }")
      && rejection.includes("let bytes = actor.capacity();") && rejection.includes("released_bytes: bytes")
      && rejection.includes("self.factory.as_ref().expect(\"original presence factory\").retire(original_arc, child)")
      && peer.includes("artifact_retirement_admit_owned(&mut self.pending_actor, &mut self.rejected, grant)")
      && peer.includes("artifact_retirement_admit_owned(&mut self.pending_presence, &mut self.rejected, grant)");
  };
  if (!exactRejectedActor(publicationSource, rejectionSource, pluginPeerSource)) throw new Error("peer rejection lost its exact actor/presence owner or byte cursor");
  const rejectedHostiles = [
    [publicationSource.replace('let source = actor.as_ref().ok_or_else(missing)?;', 'let source = actor.take().ok_or_else(missing)?;'), rejectionSource, pluginPeerSource],
    [publicationSource, rejectionSource.replace("RetirementDemand { release_bytes: actor.capacity(), depth: 1, ..Default::default() }", "RetirementDemand { release_bytes: 0, depth: 1, ..Default::default() }"), pluginPeerSource],
    [publicationSource, rejectionSource.replace("released_bytes: bytes", "released_bytes: 0"), pluginPeerSource],
    [publicationSource, rejectionSource.replace("std::mem::take(&mut *original.actor).into_bytes()", "String::new().into_bytes()"), pluginPeerSource],
    [publicationSource, rejectionSource, pluginPeerSource.replace("artifact_retirement_admit_owned(&mut self.pending_actor, &mut self.rejected, grant)", "Ok(PluginLifecycleStep::Complete)")],
    [publicationSource, rejectionSource.replace('self.factory.as_ref().expect("original presence factory").retire(original_arc, child)', "foreign_factory.retire(original_arc, child)"), pluginPeerSource],
  ] as const;
  for (const [publication, rejection, peer] of rejectedHostiles) if (exactRejectedActor(publication, rejection, peer)) throw new Error("peer rejection guard accepted dropped identity, false byte credit or missing mounted owner");
  const channelSource = readSourceFamilySource(join(storeBase, "../../📡️spr/🧵️channel"));
  const pluginLawSource = readModuleTreeSource(join(storeBase, "../../🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-5/🦀️.rs"));
  if (!pluginLawSource.includes("peer_presence_capture_is_one_arc_and_retirement_waits_for_then_drains_the_exact_root") || !pluginLawSource.includes("peer_roster_saturation_cancel_stale_and_interrupted_close_preserve_exact_authority")) throw new Error("peer capture native laws are missing");
  const pluginFamily = readSourceFamilySource(join(storeBase, "../../🔌️plugin"));
  const captureProof = (plugin: string, store: string, channel: string, retirement: string): boolean => toolJobPeerInteractionRootsExact(plugin, store, channel, retirement);
  if (!captureProof(pluginFamily, storeFamily, channelSource, retirementSource)) throw new Error("peer capture census rejected its real exact helper/base/factory authority");
  const captureHostiles = [
    [pluginFamily.replace("assert!(self.tools.tool_operations.can_insert(operation_id.0),", "A::ephemeral(command).await; assert!(self.tools.tool_operations.can_insert(operation_id.0),"), storeFamily, channelSource, retirementSource],
    [pluginFamily.replaceAll("let canonical_base_revision = self.stores.store.content_revision();", "let canonical_base_revision = self.stores.store.content_revision(); self.decode_unbounded().await?;"), storeFamily, channelSource, retirementSource],
    [pluginFamily.replaceAll("self.capture_typed_command_roots(command.as_ref(), meta).await?;", "self.foreign_capture_typed_command_roots(command.as_ref(), meta).await?;"), storeFamily, channelSource, retirementSource],
    [pluginFamily.replace("self.as_dispatch().start_typed_command_operation(command, admission, &pending.meta, pending.operation.operation, lease, cx).await", "self.as_dispatch().foreign_command_operation(command, admission, &pending.meta, pending.operation.operation, lease, cx).await"), storeFamily, channelSource, retirementSource],
    [pluginFamily.replace("presence_peers: self.stores.presence_store.peers_root(),", "presence_peers: self.stores.presence_store.peers().await,"), storeFamily, channelSource, retirementSource],
    [pluginFamily, storeFamily.replace('!Arc::ptr_eq(self.peers.as_ref().expect("live peer presence owner"), &commit.base_root)', "false"), channelSource, retirementSource],
    [pluginFamily, storeFamily.replace("Arc::ptr_eq(factory, &commit.factory)", "true"), channelSource, retirementSource],
    [pluginFamily, storeFamily.replace("pub(super) base_root: std::mem::ManuallyDrop<Arc<PresencePeersRoot<P>>>,\n    pub(super) root:", "pub base_root: std::mem::ManuallyDrop<Arc<PresencePeersRoot<P>>>,\n    pub(super) root:"), channelSource, retirementSource],
    [pluginFamily, storeFamily.replace("pub struct PresencePeersCommit<P> {", "pub struct PresencePeersCommit<P> { pub factory_override: Arc<dyn SnapshotRetirementFactory<P>>,"), channelSource, retirementSource],
    [pluginFamily, storeFamily.replace("*retirement.root = Some(previous);\n        drop(base_root);", "drop(base_root);\n        *retirement.root = Some(previous);"), channelSource, retirementSource],
    [pluginFamily, storeFamily, channelSource, retirementSource.replace("base_root: std::mem::ManuallyDrop::new(Some(base_root))", "base_root: std::mem::ManuallyDrop::new(None)")],
    [pluginFamily.replace("self.roster.validate_peer_roster_publication(&*self.stores, seq, generation, &cancel)", "self.accept_unchecked_roster(seq, generation, &cancel)"), storeFamily, channelSource, retirementSource],
    [pluginFamily, storeFamily, channelSource.replace("pub fn admit_page(seq: u64, own_color: Option<u8>, item_count: u32, page: FixedCommandPage)", "pub fn decode_before_admission(seq: u64, own_color: Option<u8>, item_count: u32, page: FixedCommandPage)"), retirementSource],
  ];
  for (const [caseIndex, [plugin, store, channel, retirement]] of captureHostiles.entries()) {
    if (plugin === pluginFamily && store === storeFamily && channel === channelSource && retirement === retirementSource) throw new Error(`peer capture hostile ${caseIndex} missed its exact source target`);
    if (captureProof(plugin, store, channel, retirement)) throw new Error(`peer capture census admitted hostile ${caseIndex}: forged helper, public authority, stale root, foreign factory or bypassed ingress`);
  }
  return 1 + fixture.cases.length + fixture.storeCases.length + 2 + 1 + storeFixture.cases.length + 30 + peerFixture.cases.length + closeFactoryChecks + commitChecks + 1 + captureHostiles.length;
}

test("CAD presence retirement byte oracles and presence Store source guards hold and reject every hostile mutation", () => {
  expect(cadPresenceRetirementChecks()).toBeGreaterThan(0);
}, 120_000);
