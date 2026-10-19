/** 🔌️ The hub session, the open artifact, the execution target lease and the fake socket the cold pair transfer runs against.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | registerGisDocumentOpenTests}: a phase of the cold pair transfer test over the same registrar bindings, moved verbatim. */
import { stubFetch } from "@semio-tech/framework-os/testing/fetch-stub";
import { workerLawFakesV1, executionTargetBytes, WORKER_LAW_CAPABILITY_TWO, FakeHubWebSocket, WORKER_LAW_ACTOR_IDENTITY_GRANT } from "@semio-tech/framework-os/testing/worker-law-fakes";
import type { TestSource } from "@semio-tech/framework-os/testing/worker-law-fakes";
import type { ServiceCompositionTestDependenciesV1 } from "../../🧰️dependencies/🟦️.ts";
import type { ColdPairProbe, BackboneWorkerResponse, BrowserActorUiPatchOfferV1, Vitest } from "../📐️document-open-types/🟦️.ts";

/** 🌉️ Installs the session capability and the fetch stub, opens the artifact, mints the lease and wires the fake socket and worker sink. */
export async function openColdPairSession({ dependencies, vitest, fakes, source, actorBytes, fixture, hostView, descriptorByteSha256, descriptor, exact, publishedPack, publishedSpr, statuses, workerResponses, historyStatuses, probe, patchOffers, uiStore }: {
  readonly dependencies: ServiceCompositionTestDependenciesV1;
  readonly vitest: Vitest;
  readonly fakes: ReturnType<typeof workerLawFakesV1>;
  readonly source: TestSource;
  readonly actorBytes: Uint8Array<ArrayBuffer>;
  readonly fixture: Awaited<ReturnType<ReturnType<typeof workerLawFakesV1>["executionTargetLeaseFixture"]>>;
  readonly hostView: any;
  readonly descriptorByteSha256: string;
  readonly descriptor: Uint8Array<ArrayBuffer>;
  readonly exact: { packLength: number; sprLength: number; pageCount: number; packPattern: { multiplier: number; addend: number; }; sprPattern: { multiplier: number; addend: number; }; packSha256: string; sprSha256: string; aggregateSha256: string; };
  readonly publishedPack: Uint8Array<ArrayBuffer>;
  readonly publishedSpr: Uint8Array<ArrayBuffer>;
  readonly statuses: Extract<BackboneWorkerResponse, { kind: "execution-target-status" }>[];
  readonly workerResponses: BackboneWorkerResponse[];
  readonly historyStatuses: Extract<BackboneWorkerResponse, { kind: "inference-history-status" }>[];
  readonly probe: ColdPairProbe;
  readonly patchOffers: BrowserActorUiPatchOfferV1[];
  readonly uiStore: InstanceType<typeof import("@semio-tech/framework-os/modules/renderer/engine/elements/UiDocumentStore").UiDocumentStore>;
}) {
  const { clearHubSessionCapability, installHubSessionCapability, hubSessionFetch, testSeams, openArtifact, artifactState, artifacts, documentRuntimeKeyForConfig, handleTsRequest, DocumentExecutionTargetLease, documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1 } = dependencies;
  const { expect } = vitest;
  const { executionTargetBodyResponse } = fakes;
  clearHubSessionCapability();
  expect(installHubSessionCapability(WORKER_LAW_CAPABILITY_TWO)).toBe(true);
  const authorityFixture = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
  const authority = authorityFixture.rows.find((row: { accepted: boolean }) => row.accepted).value;
  const authorityBody = JSON.stringify(authority);
  globalThis.fetch = stubFetch(async () => new Response(authorityBody, { status: 200, headers: { "content-length": String(new TextEncoder().encode(authorityBody).byteLength) } }));
  await hubSessionFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000, accept: testSeams.acceptBrowserSessionAuthority });
  const sessionFence = testSeams.captureBrowserSessionOperationFence();
  if (sessionFence === null) throw new Error("browser session operation fence missing after an accepted authority");
  globalThis.fetch = stubFetch(async (input) => {
    expect(String(input).endsWith("/execution-target/browser-actor")).toBe(true);
    return executionTargetBodyResponse(actorBytes);
  });
  const documentId = "cold-browser-pair";
  openArtifact({ documentId, schema: fixture.manifest.artifact.schema, bindings: [], actor: "untrusted-ui-actor", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT });
  const state = artifactState(documentId)!;
  artifacts.delete(state.runtimeKey);
  const binding = { kind: "hub", dataClass: "persistedShared", baseUrl: fixture.hubOrigin, spaceId: "cold-browser-space", requestedSurfaceId: fixture.manifest.surface.surfaceId } as const;
  state.config = { ...state.config, documentId, schema: fixture.manifest.artifact.schema, bindings: [binding] };
  state.runtimeKey = documentRuntimeKeyForConfig(state.config);
  artifacts.set(state.runtimeKey, state);
  const hostRequest = { kind: "browser-actor-view-state", clientInstanceId: state.openClientInstanceId, scope: { spaceId: binding.spaceId, documentId }, viewState: hostView } as const;
  handleTsRequest({ ...hostRequest, clientInstanceId: "00000000-0000-4000-8000-000000000000" });
  expect(state.browserActorViewState).toBeNull();
  handleTsRequest(hostRequest);
  expect(state.browserActorViewState).toEqual(hostView);
  const fields = structuredClone(fixture.manifest);
  fields.scope = { spaceId: binding.spaceId, documentId };
  fields.grant = { ...fields.grant, write: true };
  fields.surface = { ...fields.surface, role: "editor" };
  fields.package = { ...fields.package, descriptorByteSha256, componentReceipt: { ...fields.package.componentReceipt, semanticDescriptorSha256: descriptorByteSha256 } };
  fields.descriptor = { sha256: descriptorByteSha256, byteLength: descriptor.byteLength };
  if (fields.browserActor.kind !== "none") fields.browserActor = { ...fields.browserActor, sourceDescriptorByteSha256: descriptorByteSha256 };
  fields.checkpoint = {
    ...fields.checkpoint!,
    aggregateSha256: exact.aggregateSha256,
    baselineFrontier: { ...fields.checkpoint!.baselineFrontier, documentId },
  };
  const lease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1(fields), fixture.hubOrigin, executionTargetBytes(fixture.componentHex), descriptor);
  state.executionTargetLease = lease;
  state.currentPack = publishedPack;
  state.currentSpr = publishedSpr;
  state.frontier = {
    document_id: documentId,
    head_edit_ordinal: fields.checkpoint.baselineFrontier.headEditOrdinal,
    head_edit_id: fields.checkpoint.baselineFrontier.headEditId,
    last_commit_seq: fields.checkpoint.baselineFrontier.lastCommitSeq,
    chain_hash: fields.checkpoint.baselineFrontier.chainHash,
  };
  const intent = { ...structuredClone(fixture.intent), scope: { spaceId: binding.spaceId, documentId } },
    actorFixtureExpiresAtMs = Date.now() + 600_000,
    receipt = { ...structuredClone(fixture.socketGrant), expiresAtMs: actorFixtureExpiresAtMs };
  lease.admitBrowserActor(documentExecutionTargetLeaseMintToken, receipt, actorFixtureExpiresAtMs + 5_000, { binding, intent, assertCurrent() {} });
  state.actor = receipt.actorId;
  state.hubActorReady = true;
  state.pendingSocketActorId = null;
  const socket = new FakeHubWebSocket("ws://hub.test/cold");
  state.socket = socket as unknown as WebSocket;
  socket.open();
  testSeams.executionTargetStatusObserver = (status) => statuses.push(status);
  testSeams.workerPostTestSink = (message) => {
    workerResponses.push(message);
    if (message.kind === "inference-history-status") {
      historyStatuses.push(message);
      return;
    }
    if (message.kind === "browser-actor-ui-mounted") {
      expect(probe.patchAcknowledged).toBe(true);
      return;
    }
    if (message.kind !== "browser-actor-ui-patch") return;
    patchOffers.push(message);
    expect(message.patches).toHaveLength(1);
    const offered = message.patches[0]!;
    if (offered.baseRevision === 1 && offered.revision === 2) {
      probe.heldActionPatch = message;
      return;
    }
    const before = uiStore.getState();
    const applied = uiStore.applyPatch(offered);
    if (applied.ok) {
      handleTsRequest({
        kind: "browser-actor-ui-patch-result",
        clientInstanceId: message.clientInstanceId,
        scope: message.scope,
        verifiedSurfaceId: message.verifiedSurfaceId,
        activationGeneration: message.activationGeneration,
        instanceId: message.instanceId,
        receipt: message.receipt,
        verdicts: [{ surface: offered.surface, outcome: "acknowledged", revision: uiStore.getRevisionSnapshot() }],
      });
      return;
    }
    expect(uiStore.getState()).toBe(before);
    handleTsRequest({
      kind: "browser-actor-ui-patch-result",
      clientInstanceId: message.clientInstanceId,
      scope: message.scope,
      verifiedSurfaceId: message.verifiedSurfaceId,
      activationGeneration: message.activationGeneration,
      instanceId: message.instanceId,
      receipt: message.receipt,
      verdicts: [{ surface: offered.surface, outcome: "rejected", revision: uiStore.getRevisionSnapshot(), reason: applied.rejection.type }],
    });
  };
  return { state, fields, documentId, lease, binding, sessionFence, hostRequest };
}
