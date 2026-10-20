/** 🛡️ The visibility, error status, committer ledger and committer source checks of the GIS Map proposal approval fixture proof. */
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { readHubInferenceRuntimeSource } from "../../../🧪️testing/📖️source-readers/🟦️.ts";

/** 👁 Checks the role visibility rows, the error statuses and the committer ledger rows. */
export function checkVisibilityErrorsAndCommitterLedger({ fixture, repoRoot }: {
  readonly fixture: any;
  readonly repoRoot: string;
}) {
  const owners = fixture.visibility.filter((row: { readProposal: boolean }) => row.readProposal);
  if (owners.length !== 1 || owners[0].role !== "author-owner" || !owners[0].approve || owners[0].expectedCode !== null) throw new Error("exactly one original Author owner may read and approve");
  for (const row of fixture.visibility) if (row.role !== "author-owner" && (row.readEvents || row.readProposal || row.approve || row.expectedCode !== "inference.denied")) throw new Error(`non-owner ${row.role} was granted private job access`);
  const statuses = new Map<string, number>();
  for (const row of fixture.errors) {
    if (statuses.has(row.code) && statuses.get(row.code) !== row.status) throw new Error(`error code ${row.code} maps to two statuses`);
    statuses.set(row.code, row.status);
  }
  if (statuses.get("approval.commit-unavailable") !== 503) throw new Error("a missing composition transaction must fail closed with 503");
  for (const rejection of fixture.approvalRejections) if (!statuses.has(rejection.code)) throw new Error(`approval rejection ${rejection.name} uses an unpublished code`);
  const ledger = JSON.parse(readFileSync(join(repoRoot, "🌎️hub", "🧪️tests/🧫️fixtures", "🗺️gis-inference-job-v1", "🔣️.json"), "utf8"));
  const committer = fixture.committer;
  if (
    committer.capacity !== fixture.limits.documentGateCapacity ||
    JSON.stringify(committer.roles) !== JSON.stringify(["map", "drawing", "value"]) ||
    JSON.stringify(committer.children) !== JSON.stringify(["gismap-drawing", "gismap-value"]) ||
    JSON.stringify(committer.phases) !== JSON.stringify(["ready", "assembly", "journal", "verification", "publishing", "published", "closing"]) ||
    committer.witnessSource !== "sole-committed-wal-event" ||
    committer.retryOwner !== "same-document-three-store-host" ||
    committer.frontierChain !== "canonical-lowercase-hex-with-genesis-zero" ||
    ledger.identity.chainHash !== "0".repeat(64) ||
    JSON.stringify(committer.storeIdentities) !==
      JSON.stringify({
        parentDialect: "s.gis.gismap@1/*",
        childReferences: ["gismap-drawing!s.stdio.semio@v1/drawing", "gismap-value!s.stdio.semio@v1/value"],
        childOwnerRoles: ["drawing", "value"],
        childOwnerParent: "exact-document-parent-reference",
      }) ||
    JSON.stringify(committer.publicationOrder) !== JSON.stringify(["committed-wal-event", "actor-frontier", "public-checkpoint", "ledger-applied"]) ||
    JSON.stringify(committer.beforePublicAck) !== JSON.stringify({ ledgerApplied: false, documentWrite: "retained", state: "publishing" }) ||
    JSON.stringify(committer.publicationFailure) !== JSON.stringify({ ledgerApplied: false, documentWrite: "retained", retry: "same-event-same-owner", retryDeadline: "process-owned-bounded-attempt" }) ||
    JSON.stringify(committer.ingressAuthority) !==
      JSON.stringify({
        owner: "hub-private-sorted-binding-guards",
        bindings: ["user", "session", "space-authority", "membership"],
        documentWrite: "runtime-acquired",
        cutover: "durable-wal-receipt",
        delivery: "fresh-revalidation-under-same-guards",
        recovery: "committed-wal-provenance",
      }) ||
    JSON.stringify(committer.abandonedRequest) !==
      JSON.stringify({
        owner: "runtime-document-maintenance",
        preparedOwner: "armed-before-committer-preflight",
        driver: "document-write-request-token-single-poller",
        cleanupCapacity: 128,
        runtimeShutdown: "reinsert-exact-owner-before-task-drop",
        terminalClose: "fence-new-admission-until-cleanup-empty",
        abortPhases: ["no-runtime-unpolled", "unpolled", "rejected-preflight", "ready", "preflight", "assembly", "journal-no-receipt"],
        outbox: "abandoned-until-fresh-identical-retry",
        commitCutover: "awaiting-ack",
        postCutover: ["verification", "public-checkpoint", "ledger-applied"],
      }) ||
    committer.documentFence !== "mounted-actor-generation-including-zero" ||
    committer.genesisPublication !== "active-indexed-genesis-parent-before-first-edit" ||
    JSON.stringify(committer.peerNotification) !==
      JSON.stringify({
        after: ["committed-wal-event", "actor-frontier", "public-checkpoint", "ledger-applied"],
        frame: "rebootstrap-required",
        source: "active-checkpoint",
        command: "server-stamped-create-region",
        peers: 2,
        publicationFailure: "none",
        publicationCancellation: "none",
      })
  )
    throw new Error("retained fixed-three committer contract drifted");
  return { ledger };
}

/** 🧬 Checks the retained GIS Map runtime, binding, wal and sqlite sources. */
export function checkRetainedRuntimeSources({ repoRoot }: {
  readonly repoRoot: string;
}) {
  const runtime = readHubInferenceRuntimeSource(repoRoot);
  const gisBinding = readModuleTreeSource(join(repoRoot, "🌎️hub", "🧩️compositions", "🌍️gis", "💡️inference", "🦀️.rs"));
  if (runtime.includes("semio_s_artifact_") || !runtime.includes("impl<B: ComposedMapCommitBindingV1> RetainedGisMapApprovalCommitterV1<B>"))
    throw new Error("the retained committer names an artifact crate instead of staying generic over the registered composed-map binding");
  for (const symbol of ["impl ComposedMapCommitBindingV1 for GisMapInferenceBindingV1", "impl MapInferenceProposalBindingV1 for GisMapInferenceBindingV1", "gis_map_parent_stamped_one_item_preparation_factory"])
    if (!gisBinding.includes(symbol)) throw new Error(`GIS Map inference binding source is missing ${symbol}`);
  for (const symbol of [
    "RetainedGisMapApprovalCommitterV1",
    "RetainedGisMapDocumentStateV1",
    "DurableOwnedThreeStoreMapAssemblyV1",
    "durable_group_journal_sink",
    "InferenceWalVerifierV1",
    "parse_frontier_chain_hash",
    "GisMapApprovalCheckpointPublisherV1",
    "checkpoint_applied",
    "GisMapApprovalIngressAuthorityV1",
    "GisMapApprovalRequestOwnerV1",
    "GisMapPreparedApprovalV1",
    "GisMapAbandonedRequestTaskV1",
    "GisMapPreparedApprovalTaskV1",
    "resume_parked_requests",
    "resume_parked_prepared",
    "drive_abandoned_request",
    "drive_abandoned_prepared",
    "abandon_prepared_approval",
    "owners.fence.clone()",
    "RetainedGisMapDocumentStateV1::Publishing",
    "B::parent_preparation_factory(",
    "sole committed WAL event",
  ])
    if (!runtime.includes(symbol)) throw new Error(`retained GIS Map committer source is missing ${symbol}`);
  const walRuntime = readFileSync(join(repoRoot, "🌎️hub", "💡️inference", "🧾️wal", "🦀️.rs"), "utf8");
  if (!walRuntime.includes("active: AtomicBool") || !walRuntime.includes("self.active.store(false, Ordering::Release)") || walRuntime.includes("self.generation.store(0, Ordering::Release)")) {
    throw new Error("mounted actor generation zero is not independently fenced from invalidation");
  }
  const sqliteRuntime = readModuleTreeSource(join(repoRoot, "🌎️hub", "💡️inference", "🪶️sqlite", "🦀️.rs"));
  if (sqliteRuntime.includes("document_generation == 0") || !sqliteRuntime.includes("witness.matches(&scope, document_generation")) {
    throw new Error("initial actor generation zero cannot reach exact fenced ledger reconciliation");
  }
  const retainedCommitter = runtime.slice(runtime.indexOf("pub struct RetainedGisMapApprovalCommitterV1"), runtime.indexOf("/// 🗺️ The server-materialized Map base"));
  const storeConstruction = gisBinding.slice(gisBinding.indexOf("fn parent_store("), gisBinding.indexOf("pub fn gis_map_region_preview("));
  const verify = retainedCommitter.slice(retainedCommitter.indexOf("async fn verify("), retainedCommitter.indexOf("async fn commit_retained("));
  const publishCall = verify.indexOf("self.publish_checkpoint(");
  const reconcileCall = verify.indexOf(".reconcile_committed_approval(");
  const checkpointAppliedCall = verify.indexOf(".checkpoint_applied(");
  const documentWriteRelease = verify.indexOf("drop(document_write)");
  if (
    !runtime.includes("const GIS_MAP_COMMITTER_CAPACITY: usize = 64;") ||
    !storeConstruction.includes('artifact_kind: "s.gis.gismap"') ||
    !storeConstruction.includes('artifact_kind: "s.stdio.semio"') ||
    !storeConstruction.includes('slot: "drawing"') ||
    !storeConstruction.includes('slot: "value"') ||
    !gisBinding.includes("let drawing_id = snapshot.drawing.child_id.clone();") ||
    !gisBinding.includes("let value_id = snapshot.value.child_id.clone();") ||
    !retainedCommitter.includes("B::member_stores(&key, snapshot, actor)") ||
    retainedCommitter.includes("ArtifactHandle::submit") ||
    publishCall < 0 ||
    reconcileCall <= publishCall ||
    !retainedCommitter.includes("document_write: GisMapDocumentWriteLeaseV1") ||
    checkpointAppliedCall <= reconcileCall ||
    documentWriteRelease <= checkpointAppliedCall
  )
    throw new Error("retained GIS Map committer capacity, write authority, typed journal, or publication-before-apply boundary drifted");
}
