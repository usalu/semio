/** 🗺️ The trusted gis retained browser, gis map collaboration contract and gis publication fixture proofs. */
import { createHash, webcrypto } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { readHubScriptModule } from "../../../🧪️testing/📜️script-sources/🟦️.ts";

/** 🌐️ Binds a browser proof to one retained trusted generation rather than a second build. */
export async function proveTrustedGisRetainedBrowserFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🧬️retained-gis-browser");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  assert.deepEqual(Object.keys(fixture), ["schema", "bootstrapFixture", "contract", "cases", "nonclaims"]);
  assert.equal(fixture.schema, "semio.hub.retained-gis-browser/v1");
  assert.equal(fixture.bootstrapFixture, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json");
  assert.deepEqual(fixture.contract, {
    generation: "published-current-exact",
    actorPath: "packages/gis/browser/closed-actor.mjs",
    descriptorPath: "packages/gis/descriptor.semio",
    componentPath: "packages/gis/component.wasm",
    actor: "retained-not-rebuilt",
    browser: "chromium-dedicated-child",
    result: "normalized-describe-equals-retained-descriptor",
    terminalActors: 0,
    terminalBytes: 0,
  });
  assert.deepEqual(fixture.nonclaims, ["authenticated-worker", "cold-pair-applied", "shell-renderer", "map-mutation", "two-peer-collaboration"]);
  assert.equal(new Set(fixture.nonclaims).size, fixture.nonclaims.length);
  const bootstrap = JSON.parse(readFileSync(join(repoRoot, fixture.bootstrapFixture), "utf8"));
  const gis = bootstrap.profile.packages.find((row: any) => row.pluginId === "gis");
  const expected = { generation: bootstrap.profile.generationId, component: gis.componentSha256, descriptor: gis.descriptorSha256, actor: gis.browserActor.sha256, parent: gis.browserActor.sourceComponentSha256 };
  const equal = (await import("fast-deep-equal")).default;
  assert.equal(createHash("sha256").update("abc").digest("hex"), expected.actor, "neutral actor identity must be independently reproducible");
  const digest = Buffer.from(await webcrypto.subtle.digest("SHA-256", new TextEncoder().encode("abc"))).toString("hex");
  assert.equal(digest, expected.actor);
  assert.deepEqual(fixture.cases.map((row: any) => row.name), ["exact", "replaced-component", "replaced-descriptor", "replaced-actor", "changed-generation", "changed-parent-digest"]);
  const mutations: Record<string, keyof typeof expected> = { "replaced-component": "component", "replaced-descriptor": "descriptor", "replaced-actor": "actor", "changed-generation": "generation", "changed-parent-digest": "parent" };
  for (const row of fixture.cases) {
    const value = { ...expected };
    const field = mutations[row.name];
    if (field) value[field] = "ff".repeat(32);
    assert.equal(equal(value, expected), row.accepted, row.name);
  }
  const source = readHubScriptModule(repoRoot, "🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication");
  const start = source.indexOf("\nasync function proveTrustedGisClosedActorV1(");
  const end = source.indexOf("\n/** 🧳️ Copies one already verified immutable selection", start);
  assert(start >= 0 && end > start, "retained GIS browser proof must exist before its native mode can qualify it");
  const selected = source.slice(start, end);
  const conforms = (value: string): boolean => value.includes("trustedBootstrapCurrent(dataRoot)")
    && value.includes("published.currentSha256 !== initialCurrent.currentSha256")
    && value.includes("trustedBootstrapVerifyGeneration(")
    && value.includes('"packages/gis/browser/closed-actor.mjs"')
    && value.includes("gis.browserActor.sourceComponentSha256 !== gis.component.sha256")
    && value.includes("gis.browserActor.sourceDescriptorByteSha256 !== gis.descriptor.sha256")
    && value.includes("await proveBrowserActorGisChildV1(")
    && value.split("verifyRetained();").length === 3
    && !value.includes("produceFreshComponentV1(")
    && !value.includes("buildClosedBrowserActorArtifactV1(");
  assert(conforms(selected), "retained browser proof must verify current and exact generation bytes on both sides of the genuine child");
  const hostiles = [
    selected.replaceAll("trustedBootstrapCurrent(dataRoot)", "undefined"),
    selected.replace("published.currentSha256 !== initialCurrent.currentSha256", "false"),
    selected.replaceAll("trustedBootstrapVerifyGeneration(", "omittedGenerationCheck("),
    selected.replaceAll('"packages/gis/browser/closed-actor.mjs"', '"caller-selected.mjs"'),
    selected.replace("gis.browserActor.sourceComponentSha256 !== gis.component.sha256", "false"),
    selected.replace("gis.browserActor.sourceDescriptorByteSha256 !== gis.descriptor.sha256", "false"),
    selected.replace("await proveBrowserActorGisChildV1(", "void proveBrowserActorGisChildV1("),
    selected.replace("verifyRetained();", ""),
  ];
  for (const hostile of hostiles) assert(hostile !== selected && !conforms(hostile), "retained browser hostile must fail");
  const gateStart = source.indexOf("\nclass TrustedStdioGisBundleCheckScript");
  const gate = source.slice(gateStart, source.indexOf("\nclass TrustedCatalogBootstrapScript", gateStart));
  const mode = gate.lastIndexOf('if (segments[0] === "--browser" || segments[0] === "--two-author-shell")');
  assert(mode >= 0 && gate.indexOf("const initialPlan = await validateAndPublishTrustedStdioGisCandidate") < mode);
  assert(gate.slice(mode).includes("await proveTrustedGisClosedActorV1(this.repoRoot, dataRoot, receipt, artifactPath)"));
  console.log("trusted-retained-gis-browser: pinned-tables=3 deep-equal=" + fixture.cases.length + " SHA256=node+webcrypto source-hostiles=" + hostiles.length + "; neutral bytes only, no browser execution claim");
}

/** 🤝️ Pins the genuine two-peer acceptance specification without claiming any observed runtime trace. */
export async function proveTrustedGisMapCollaborationContractFixture(repoRoot: string): Promise<void> {
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const equal = (await import("fast-deep-equal")).default;
  const root = join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🤝️gis-map-collaboration");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const pinned = {
    schema: "semio.hub.gis-map-collaboration/v1",
    prerequisites: {
      bootstrapFixture: "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json",
      browserFixture: "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🧬️retained-gis-browser/🔣️.json",
      approvalFixture: "🌎️hub/🧪️tests/🧫️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json",
    },
    selection: {
      profileId: "local-stdio-gis-open-v1",
      packageId: "semio:gis",
      artifactKind: "s.gis.gismap",
      artifactSchema: "gis.map",
      parentDialect: "s.gis.gismap@1/*",
      surfaceId: "s.gis.gismap@1/*#editor",
      grantedMode: "read-write-observe",
      serviceId: "s.gis.gismap.inference",
      generation: "same-published-current-through-teardown",
      actor: "retained-not-rebuilt",
    },
    peers: [
      { name: "author-a", role: "author", language: "en" },
      { name: "author-b", role: "author", language: "de" },
    ],
    authority: {
      sessions: "distinct-authenticated-react-relay-profiles",
      capabilities: "host-only-never-page-worker-or-report",
      createDocument: "production-directory-command",
      initialCheckpoint: "hub-materialized-check-in",
      open: "production-plan-asset-grant-session",
      pair: "actual-terminal-pack-spr-receipt",
      load: "genuine-plugin-load-document-pack",
      scene: "mounted-shell-tiled-map-host",
      sceneAck: "scope-client-surface-session-bound-ui-owner",
      concurrency: "both-sockets-live-before-mutation",
    },
    lifecycle: [
      "publish-qualified-generation", "create-shared-document", "publish-initial-checkpoint", "open-two-authenticated-shells",
      "observe-two-initial-gis-scenes", "cancel-separate-owner-job", "deny-peer-access-to-owner-private-job",
      "observe-server-stamped-create-region-proposal", "approve-owner-proposal", "observe-committed-wal-event",
      "observe-actor-frontier", "observe-public-checkpoint", "observe-ledger-applied", "observe-two-live-rebootstrap-frames",
      "observe-two-fresh-plan-grant-sessions", "observe-two-later-cold-pairs", "observe-two-equal-region-scenes",
      "undo-via-ordinary-document-command", "observe-two-equal-undone-scenes", "close-all-owners", "verify-unchanged-catalog-generation",
    ],
    equalBindings: [
      "profileId", "catalogGenerationId", "packageId", "componentSha256", "descriptorByteSha256", "browserActorSha256",
      "browserActorByteLength", "documentId", "surfaceId", "parentDialect", "coldPairManifestHash", "frontier", "visibleMapScene",
    ],
    refreshedBindings: ["sessionId", "socketGrant", "coldPairReceiptGeneration"],
    approval: {
      command: "CreateRegion",
      regionId: "inference-<owner-job-id>",
      kind: "inference-bounds",
      inverse: "DeleteRegion",
      effectCount: 1,
      peerMaySubmitOwnJob: true,
      peerMayReadOwnerJob: false,
      peerMayCancelOwnerJob: false,
      peerMayApproveOwnerJob: false,
    },
    teardown: { pages: 0, workers: 0, relays: 0, childActors: 0, childBytes: 0, retainedDocumentOwners: 0, hubProcesses: 0 },
    forbiddenSubstitutes: [
      "sequential-peers", "controlled-actor", "independently-rebuilt-actor", "direct-artifact-store-write",
      "direct-ui-document-store-patch", "injected-rebootstrap-frame", "patch-positions-as-ai-approval", "raw-capability-in-page",
    ],
    nonclaims: ["all-plugin-assets-qualified", "external-model-provider-qualified", "postgres-live-qualified", "neo4j-live-qualified"],
  };
  const validate = (candidate: unknown): boolean => equal(candidate, pinned) && JSON.stringify(Object.keys(candidate as object)) === JSON.stringify(Object.keys(pinned));
  assert(validate(fixture), "collaboration contract drifted from its pinned specification");
  const approval = JSON.parse(readFileSync(join(repoRoot, fixture.prerequisites.approvalFixture), "utf8"));
  const browser = JSON.parse(readFileSync(join(repoRoot, fixture.prerequisites.browserFixture), "utf8"));
  for (const key of ["packageId", "artifactKind", "artifactSchema", "surfaceId", "grantedMode", "serviceId"]) {
    assert.equal(fixture.selection[key], approval.binding[key]);
    assert(equal(fixture.selection[key], approval.binding[key]));
  }
  assert.equal(fixture.selection.actor, browser.contract.actor);
  assert(equal(fixture.lifecycle.filter((phase: string) => ["observe-committed-wal-event", "observe-actor-frontier", "observe-public-checkpoint", "observe-ledger-applied"].includes(phase)).map((phase: string) => phase.slice(8)), approval.committer.publicationOrder));
  const hostiles = [
    { ...fixture, authority: { ...fixture.authority, concurrency: "sequential" } },
    { ...fixture, selection: { ...fixture.selection, actor: "rebuilt" } },
    { ...fixture, approval: { ...fixture.approval, command: "patchPositions" } },
    { ...fixture, approval: { ...fixture.approval, peerMaySubmitOwnJob: false } },
    { ...fixture, approval: { ...fixture.approval, peerMayReadOwnerJob: true } },
    { ...fixture, lifecycle: fixture.lifecycle.filter((phase: string) => !phase.includes("undo")) },
    { ...fixture, peers: fixture.peers.slice(0, 1) },
    { ...fixture, teardown: { ...fixture.teardown, childBytes: 1 } },
  ];
  for (const hostile of hostiles) assert(!validate(hostile), "collaboration contract hostile must fail");
  console.log("trusted-gis-collaboration-contract: pinned-specification=1 hostile=" + hostiles.length + " cross-fixture-equality=8; specification only, no observed collaboration trace");
}

/** 🛂️ Checks one mandatory exact-generation proof before every new current publication. */
export async function proveTrustedGisPublicationFixture(repoRoot: string, fixture: Record<string, any>): Promise<void> {
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  assert.deepEqual(fixture.entryPoints, ["development", "bootstrap", "native", "process-rotation"]);
  assert.deepEqual(fixture.steps, ["materialized", "exact-gis-cold-proof", "candidate-validated", "current-published"]);
  assert.deepEqual(fixture.cases.map((row: any) => row.name), ["exact", "omitted", "late", "wrong-generation", "proof-failed"]);
  const { Database } = await import("bun:sqlite");
  const oracle = new Database(":memory:");
  try {
    oracle.run("CREATE TABLE qualification (phase INTEGER, generation TEXT, current TEXT)");
    for (const entry of fixture.entryPoints) {
      for (const row of fixture.cases) {
        oracle.run("DELETE FROM qualification");
        oracle.run("INSERT INTO qualification VALUES (0, 'candidate', 'previous')");
        for (const step of row.steps) {
          const index = fixture.steps.indexOf(step);
          const allowed = step !== "exact-gis-cold-proof" || (row.proofSucceeds && row.proofGeneration === "same");
          const changed = allowed ? oracle.run("UPDATE qualification SET phase = phase + 1 WHERE phase = ?1", [index]).changes : 0;
          if (changed !== 1) break;
          if (step === "current-published") oracle.run("UPDATE qualification SET current = generation WHERE phase = 4");
        }
        const result = oracle.query("SELECT phase, current FROM qualification").get() as { phase: number; current: string };
        assert.equal(result.phase === 4, row.published, entry + ":" + row.name);
        assert.equal(result.current, row.retained, entry + ":" + row.name);
      }
    }
  } finally {
    oracle.close();
  }
  const source = readHubScriptModule(repoRoot, "🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication");
  const body = (start: string, end: string): string => {
    const first = source.indexOf(start), last = source.indexOf(end, first);
    assert(first >= 0 && last > first, start);
    return source.slice(first, last);
  };
  const candidate = body("\nasync function validateAndPublishTrustedStdioGisCandidate", "\n}\n");
  const proof="await proveTrustedGisColdMapComponentV1(invocation,control,repoRoot,receipt);";
  const stage="stageTrustedBootstrapCandidateCurrent(candidateContext.dataRoot,receipt,()=>{checkResourceLeaseControl(candidateContext.control);});";
  const conforms = (value: string): boolean => value.includes(proof)
    && value.indexOf(proof) < value.indexOf(stage)
    && value.indexOf(proof)<value.indexOf("await startLocalHub(")
    && value.includes("withLocalHubHostCandidateV1(host,control")
    && value.includes("startLocalHub(candidateHost,[profile],{capture:true})")
    && !value.slice(0, value.indexOf(stage)).includes("catch");
  assert(conforms(candidate), "every trusted candidate must prove its exact GIS generation before staging or startup");
  const hostiles = [
    candidate.replace(proof, ""),
    candidate.replace(proof, "").replace(stage, stage + "\n  " + proof),
    candidate.replace(proof, proof.replace("repoRoot,receipt", "repoRoot,{...receipt,generationId:'different'}")),
    candidate.replace(proof, proof.replace("await ", "void ")),
    candidate.replace(proof, proof.slice(0, -1) + ".catch(() => undefined);"),
  ];
  for (const hostile of hostiles) assert(hostile !== candidate && !conforms(hostile), "publication source admitted missing, late, substituted, detached or swallowed proof");
  const development = body("\nclass DevScript", "\nclass SecureLocalSmokeScript");
  const bootstrap = body("\nclass TrustedCatalogBootstrapScript", "\nclass AdminBackendCheckScript");
  const native = body("\nclass TrustedStdioGisBundleCheckScript", "\nclass TrustedCatalogBootstrapScript");
  for (const [name, caller] of [["development", development], ["bootstrap", bootstrap]] as const) {
    assert(caller.includes("await validateAndPublishTrustedStdioGisCandidate(host,actualPhase,receipt)"), name + " must join mandatory proof");
    assert(!caller.includes("publishTrustedBootstrapCurrent("), name + " cannot publish directly");
  }
  assert(native.includes("await validateAndPublishTrustedStdioGisCandidate(host,actualPhase,receipt)"), "native must join mandatory proof");
  assert(native.includes("await validateAndPublishTrustedStdioGisCandidate(host,actualPhase,rotated,initialPlan)"), "rotation must join mandatory proof");
  console.log("trusted-gis-publication: AJV=1 sqlite-traces=" + fixture.entryPoints.length * fixture.cases.length + " source-hostiles=" + hostiles.length + "; no native component or browser claim");
}
