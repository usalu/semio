/** 🗳️ The fixture schema, committer law and proposal geometry checks of the GIS Map proposal approval fixture proof. */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { hubSchemaExport } from "../../../📦️packages/🟦️typescript/🟦️.ts";

/** 🧾 Loads the proposal approval fixture and checks its schema, binding, limits, preview and lifecycle kinds. */
export function checkProposalFixtureSchema({ repoRoot }: {
  readonly repoRoot: string;
}) {
  const root = join(repoRoot, "🌎️hub", "🧪️tests/🧫️fixtures", "🗳️gis-map-proposal-approval-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validateBinding = hubSchemaExport(repoRoot, "schema://hub.inference/InferenceBindingIdentityV1");
  const validateLimits = hubSchemaExport(repoRoot, "schema://hub.inference/InferenceLimitsV1");
  const validatePreview = hubSchemaExport(repoRoot, "schema://hub.inference/GisMapInferencePreviewV1");
  const validateSummary = hubSchemaExport(repoRoot, "schema://hub.inference/InferenceMapSummaryV1");
  const validateLifecycleKind = hubSchemaExport(repoRoot, "schema://hub.inference/InferenceLifecycleKindV1");
  if (fixture.schema !== "semio.hub.gis-map-proposal-approval-fixture/v1" || fixture.version !== 1) throw new Error("GIS Map proposal fixture envelope drifted");
  if (!validateBinding(fixture.binding)) throw new Error("GIS Map proposal binding is not a hub.inference/InferenceBindingIdentityV1");
  if (!validateLimits(fixture.limits)) throw new Error("GIS Map proposal limits are not a hub.inference/InferenceLimitsV1");
  if (!validatePreview(fixture.preview)) throw new Error("GIS Map proposal preview is not a hub.inference/GisMapInferencePreviewV1");
  if (!validateSummary(fixture.base.expectedInference)) throw new Error("GIS Map proposal base summary is not a hub.inference/InferenceMapSummaryV1");
  for (const event of [...fixture.lifecycle, ...fixture.cancelLifecycle]) if (!validateLifecycleKind(event.kind)) throw new Error(`GIS Map proposal lifecycle kind ${event.kind} is not a hub.inference/InferenceLifecycleKindV1`);
  return { fixture };
}

/** ⚖ Checks the scalar and ordered committer law tables and the fixture row counts. */
export function checkCommitterLawTables({ fixture }: {
  readonly fixture: any;
}) {
  const scalarLaws: readonly (readonly [string, unknown])[] = [
    ["limits.proposalMaxBytes", 4096],
    ["binding.grantedMode", "read-write-observe"],
    ["binding.surfaceId", "s.gis.gismap@1/*#editor"],
    ["binding.parentDialect.subset", "*"],
    ["committer.capacity", 64],
    ["committer.witnessSource", "sole-committed-wal-event"],
    ["committer.retryOwner", "same-document-three-store-host"],
    ["committer.frontierChain", "canonical-lowercase-hex-with-genesis-zero"],
    ["committer.documentFence", "mounted-actor-generation-including-zero"],
    ["committer.genesisPublication", "active-indexed-genesis-parent-before-first-edit"],
    ["committer.beforePublicAck.ledgerApplied", false],
    ["committer.beforePublicAck.documentWrite", "retained"],
    ["committer.beforePublicAck.state", "publishing"],
    ["committer.publicationFailure.ledgerApplied", false],
    ["committer.publicationFailure.documentWrite", "retained"],
    ["committer.publicationFailure.retry", "same-event-same-owner"],
    ["committer.publicationFailure.retryDeadline", "process-owned-bounded-attempt"],
    ["committer.ingressAuthority.owner", "hub-private-sorted-binding-guards"],
    ["committer.ingressAuthority.documentWrite", "runtime-acquired"],
    ["committer.ingressAuthority.cutover", "durable-wal-receipt"],
    ["committer.ingressAuthority.delivery", "fresh-revalidation-under-same-guards"],
    ["committer.ingressAuthority.recovery", "committed-wal-provenance"],
    ["committer.abandonedRequest.owner", "runtime-document-maintenance"],
    ["committer.abandonedRequest.preparedOwner", "armed-before-committer-preflight"],
    ["committer.abandonedRequest.driver", "document-write-request-token-single-poller"],
    ["committer.abandonedRequest.cleanupCapacity", 128],
    ["committer.abandonedRequest.runtimeShutdown", "reinsert-exact-owner-before-task-drop"],
    ["committer.abandonedRequest.terminalClose", "fence-new-admission-until-cleanup-empty"],
    ["committer.abandonedRequest.outbox", "abandoned-until-fresh-identical-retry"],
    ["committer.abandonedRequest.commitCutover", "awaiting-ack"],
    ["committer.storeIdentities.parentDialect", "s.gis.gismap@1/*"],
    ["committer.storeIdentities.childOwnerParent", "exact-document-parent-reference"],
    ["committer.peerNotification.frame", "rebootstrap-required"],
    ["committer.peerNotification.source", "active-checkpoint"],
    ["committer.peerNotification.command", "server-stamped-create-region"],
    ["committer.peerNotification.peers", 2],
    ["committer.peerNotification.publicationFailure", "none"],
    ["committer.peerNotification.publicationCancellation", "none"],
  ];
  for (const [name, expected] of scalarLaws) {
    const actual = name.split(".").reduce<any>((at, key) => at?.[key], fixture);
    if (actual !== expected) throw new Error(`GIS Map proposal committer law ${name} is ${JSON.stringify(actual)}, expected ${JSON.stringify(expected)}`);
  }
  const orderedLaws: readonly (readonly [string, readonly unknown[]])[] = [
    ["committer.roles", ["map", "drawing", "value"]],
    ["committer.children", ["gismap-drawing", "gismap-value"]],
    ["committer.phases", ["ready", "assembly", "journal", "verification", "publishing", "published", "closing"]],
    ["committer.publicationOrder", ["committed-wal-event", "actor-frontier", "public-checkpoint", "ledger-applied"]],
    ["committer.peerNotification.after", ["committed-wal-event", "actor-frontier", "public-checkpoint", "ledger-applied"]],
    ["committer.ingressAuthority.bindings", ["user", "session", "space-authority", "membership"]],
    ["committer.abandonedRequest.abortPhases", ["no-runtime-unpolled", "unpolled", "rejected-preflight", "ready", "preflight", "assembly", "journal-no-receipt"]],
    ["committer.abandonedRequest.postCutover", ["verification", "public-checkpoint", "ledger-applied"]],
    ["committer.storeIdentities.childReferences", ["gismap-drawing!s.stdio.semio@v1/drawing", "gismap-value!s.stdio.semio@v1/value"]],
    ["committer.storeIdentities.childOwnerRoles", ["drawing", "value"]],
    ["nonclaims", ["no-external-model-provider", "no-wgpu-rendering", "no-client-supplied-map-pack", "no-auto-apply"]],
  ];
  for (const [name, expected] of orderedLaws) {
    const actual = name.split(".").reduce<any>((at, key) => at?.[key], fixture);
    if (JSON.stringify(actual) !== JSON.stringify(expected)) throw new Error(`GIS Map proposal ordered law ${name} is ${JSON.stringify(actual)}, expected ${JSON.stringify(expected)}`);
  }
  const committerLaws = scalarLaws.length + orderedLaws.length;
  if (fixture.lifecycle.length !== 5 || fixture.cancelLifecycle.length !== 4 || fixture.visibility.length !== 7 || fixture.errors.length !== 11 || fixture.approvalRejections.length !== 11 || new Set(fixture.nonclaims).size !== 4)
    throw new Error("GIS Map proposal fixture inventory drifted");
  for (const row of [...fixture.errors, ...fixture.approvalRejections, ...fixture.visibility.filter((entry: { expectedCode: string | null }) => entry.expectedCode !== null).map((entry: { expectedCode: string }) => ({ code: entry.expectedCode }))])
    if (!/^(inference|approval)\.[a-z-]+$/.test(row.code)) throw new Error(`GIS Map proposal error code ${row.code} is outside the closed inference/approval namespace`);
  return { committerLaws };
}

/** 📐 Checks the proposal and inverse hashes, the created region, the base bounds and the lifecycle transitions. */
export function checkProposalHashAndGeometry({ fixture }: {
  readonly fixture: any;
}) {
  const hash = (value: string): string => createHash("sha256").update(value, "utf8").digest("hex");
  if (hash(fixture.proposalCanonical) !== fixture.proposalHash || hash(fixture.inverseCanonical) !== fixture.inverseHash) throw new Error("independent proposal/inverse canonical hashes differ");
  const proposal = JSON.parse(fixture.proposalCanonical);
  const inverse = JSON.parse(fixture.inverseCanonical);
  const region = proposal.CreateRegion;
  const regionId = `inference-${fixture.sampleJobId}`;
  if (Object.keys(proposal).length !== 1 || !region || region.index !== fixture.base.snapshot.regions.length || region.item.id !== regionId || region.item.data.id !== regionId || region.item.data.kind !== "inference-bounds") {
    throw new Error("proposal is not exactly one server-stamped CreateRegion for this job");
  }
  if (!Array.isArray(inverse) || inverse.length !== 1 || Object.keys(inverse[0]).length !== 1 || inverse[0].DeleteRegion?.id !== regionId) throw new Error("inverse is not exactly one DeleteRegion for the created id");
  const points: number[][] = [];
  const scan = (value: unknown): void => {
    if (Array.isArray(value)) {
      if (value.length === 2 && value.every((part) => typeof part === "number" && Number.isFinite(part))) points.push(value as number[]);
      else value.forEach(scan);
    } else if (typeof value === "object" && value !== null) {
      const row = value as Record<string, unknown>;
      if (typeof row.lon === "number" && Number.isFinite(row.lon) && typeof row.lat === "number" && Number.isFinite(row.lat)) points.push([row.lon, row.lat]);
      Object.values(row).forEach(scan);
    }
  };
  for (const item of [...fixture.base.snapshot.positions, ...fixture.base.snapshot.routes, ...fixture.base.snapshot.regions]) scan(item.data);
  const fold = points.reduce((box, [lon, lat]) => [Math.min(box[0]!, lon!), Math.max(box[1]!, lon!), Math.min(box[2]!, lat!), Math.max(box[3]!, lat!)], [Infinity, -Infinity, Infinity, -Infinity]);
  const sorted = { lon: [...points.map((point) => point[0]!)].sort((a, b) => a - b), lat: [...points.map((point) => point[1]!)].sort((a, b) => a - b) };
  const bounds = { lonMin: sorted.lon[0]!, lonMax: sorted.lon[sorted.lon.length - 1]!, latMin: sorted.lat[0]!, latMax: sorted.lat[sorted.lat.length - 1]! };
  if (fold[0] !== bounds.lonMin || fold[1] !== bounds.lonMax || fold[2] !== bounds.latMin || fold[3] !== bounds.latMax) throw new Error("two independent bound folds disagree");
  const expected = fixture.base.expectedInference;
  if (JSON.stringify({ positionCount: fixture.base.snapshot.positions.length, routeCount: fixture.base.snapshot.routes.length, regionCount: fixture.base.snapshot.regions.length, bounds }) !== JSON.stringify(expected))
    throw new Error("independent counts/bounds differ from the fixture");
  const ring: number[][] = region.item.data.ring;
  const corners = [
    [bounds.lonMin, bounds.latMin],
    [bounds.lonMax, bounds.latMin],
    [bounds.lonMax, bounds.latMax],
    [bounds.lonMin, bounds.latMax],
    [bounds.lonMin, bounds.latMin],
  ];
  if (ring.length !== 5 || JSON.stringify(ring) !== JSON.stringify(corners)) throw new Error("proposal ring is not the closed bounds rectangle of the base snapshot");
  if (
    fixture.preview.schema !== "semio.hub.gis-map-inference-preview/v1" ||
    fixture.preview.jobId !== fixture.sampleJobId ||
    fixture.preview.proposalHash !== fixture.proposalHash ||
    fixture.preview.regionId !== regionId ||
    JSON.stringify(fixture.preview.ring) !== JSON.stringify(corners)
  ) {
    throw new Error("owner preview is not the exact bounded projection of the canonical proposal");
  }
  const transitions: Record<string, readonly string[]> = {
    accepted: ["running", "cancel-requested", "cancelled", "failed"],
    running: ["succeeded", "cancel-requested", "cancelled", "failed"],
    succeeded: ["approval-prepared", "cancel-requested", "proposal-cancelled", "proposal-stale"],
    "cancel-requested": ["cancelled", "proposal-cancelled"],
    "approval-prepared": ["approved"],
  };
  for (const trace of [fixture.lifecycle, fixture.cancelLifecycle]) {
    if (trace[0].kind !== "accepted" || trace[0].ordinal !== 1) throw new Error("every private job stream starts at accepted");
    for (let index = 1; index < trace.length; index++) {
      if (trace[index].ordinal !== index + 1) throw new Error("private job event ordinals are not dense");
      if (!(transitions[trace[index - 1].kind] ?? []).includes(trace[index].kind)) throw new Error(`illegal private job transition ${trace[index - 1].kind} -> ${trace[index].kind}`);
    }
    if (trace.length > 6) throw new Error("private job stream exceeds its bounded event ordinal");
  }
}
