/** 🚚️ The activation, cold pair install and lazy render phases of the cold pair transfer.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | registerGisDocumentOpenTests}: a phase of the cold pair transfer test over the same registrar bindings, moved verbatim. */
import { parseGisMapInferenceApprovalReceiptV1, idleGisMapInferencePortStatusV1 } from "@semio-tech/gis-gismap-js/inference/schema";
import { documentServiceRequestV1 } from "@semio-tech/framework/inference/service/io";
import { gisMapDocumentServiceDeclarationV1 } from "@semio-tech/gis-gismap-js/inference/client";
import { workerLawFakesV1, executionTargetBytes } from "@semio-tech/framework-os/testing/worker-law-fakes";
import type { ServiceCompositionTestDependenciesV1, InferenceOperationV1 } from "../../🧰️dependencies/🟦️.ts";
import type { ColdPairProbe, ArtifactState, BackboneWorkerResponse, BrowserActorUiPatchOfferV1, MutationEnvelope, WireArtifactBootstrap, Vitest } from "../📐️document-open-types/🟦️.ts";

/** 🚀️ Activates the reservation, installs the verified cold pair and asserts the exact page receipts and lazy render events. */
export async function runColdPairActivationPhase({ dependencies, vitest, probe, state, fields, exact, pack, spr, lease, publishedPack, publishedSpr, documentId, binding, sessionFence, historyStatuses, renderEvents, corpus, pageIndexes, received, statuses, patchOffers, documentControls }: {
  readonly dependencies: ServiceCompositionTestDependenciesV1;
  readonly vitest: Vitest;
  readonly probe: ColdPairProbe;
  readonly state: ArtifactState;
  readonly fields: Awaited<ReturnType<ReturnType<typeof workerLawFakesV1>["executionTargetLeaseFixture"]>>["manifest"];
  readonly exact: { packLength: number; sprLength: number; pageCount: number; packPattern: { multiplier: number; addend: number; }; sprPattern: { multiplier: number; addend: number; }; packSha256: string; sprSha256: string; aggregateSha256: string; };
  readonly pack: Uint8Array<ArrayBufferLike>;
  readonly spr: Uint8Array<ArrayBufferLike>;
  readonly lease: InstanceType<ServiceCompositionTestDependenciesV1["DocumentExecutionTargetLease"]>;
  readonly publishedPack: Uint8Array<ArrayBuffer>;
  readonly publishedSpr: Uint8Array<ArrayBuffer>;
  readonly documentId: string;
  readonly binding: { readonly kind: "hub"; readonly dataClass: "persistedShared"; readonly baseUrl: string; readonly spaceId: "cold-browser-space"; readonly requestedSurfaceId: string; };
  readonly sessionFence: Readonly<{ token: object; sessionBindingSha256: string; authorizationGeneration: number; }>;
  readonly historyStatuses: Extract<BackboneWorkerResponse, { kind: "inference-history-status" }>[];
  readonly renderEvents: string[];
  readonly corpus: any;
  readonly pageIndexes: number[];
  readonly received: Uint8Array<ArrayBuffer>;
  readonly statuses: Extract<BackboneWorkerResponse, { kind: "execution-target-status" }>[];
  readonly patchOffers: BrowserActorUiPatchOfferV1[];
  readonly documentControls: string[];
}) {
  const { reserveDocumentBrowserActorChild, VerifiedColdDocumentPair, verifiedColdDocumentPairMintToken, parseGisMapInferenceApprovalReceiptV1, executionTargetHex, hubSessionFetch, testSeams, idleGisMapInferencePortStatusV1, retainInferenceApprovalUndo } = dependencies;
  const { expect } = vitest;
  probe.reservation = await reserveDocumentBrowserActorChild(state);
  expect(probe.reservation).not.toBeNull();
  await probe.reservation!.activate(state.socket!);
  const bootstrap: WireArtifactBootstrap = {
    format_version: 1,
    descriptor_hash: Array.from(executionTargetBytes(fields.descriptorDigestV1)),
    artifact_schema: fields.artifact.schema,
    artifact_kind: fields.artifact.kind,
    pack_schema_hash: Array.from(executionTargetBytes(fields.artifact.packSchemaHash)),
    baseline_frontier: state.frontier!,
    pack_hash: Array.from(executionTargetBytes(exact.packSha256)),
    spr_hash: Array.from(executionTargetBytes(exact.sprSha256)),
    pack_length: pack.byteLength,
    spr_length: spr.byteLength,
    chunk_count: 2,
    aggregate_hash: Array.from(executionTargetBytes(exact.aggregateSha256)),
    required_tail_frontier: state.frontier!,
    inline: null,
  };
  probe.owner = new VerifiedColdDocumentPair(verifiedColdDocumentPairMintToken, state, lease, bootstrap, { pack, spr }, { pack: publishedPack, spr: publishedSpr });
  state.verifiedColdPair = probe.owner;
  const approvalReceipt = parseGisMapInferenceApprovalReceiptV1({
    schema: "semio.hub.inference-approval-receipt/v1",
    jobId: "1".repeat(32),
    mutationId: "2".repeat(32),
    commandHash: "3".repeat(64),
    proposalHash: "4".repeat(64),
    applied: true,
    undo: {
      targetId: "5".repeat(32),
      expectedCurrent: {
        documentId,
        headEditOrdinal: fields.checkpoint.baselineFrontier.headEditOrdinal,
        headEditId: fields.checkpoint.baselineFrontier.headEditId,
        lastCommitSeq: fields.checkpoint.baselineFrontier.lastCommitSeq,
        chainSha256: executionTargetHex(Uint8Array.from(fields.checkpoint.baselineFrontier.chainHash)),
      },
    },
  });
  const operation: InferenceOperationV1 = {
    transport: { async call(action, payload, options) {
      const request = documentServiceRequestV1(gisMapDocumentServiceDeclarationV1(), { spaceId: binding.spaceId, documentId }, action, payload);
      return await hubSessionFetch(request.path, { method: request.method, ...(request.body === undefined ? {} : { headers: { "content-type": "application/json" }, body: request.body }) }, options);
    } },
    operationEpoch: 1,
    scope: { spaceId: binding.spaceId, documentId },
    abort: new AbortController(),
    sessionEpoch: testSeams.directorySessionEpoch,
    sessionFence,
    clientInstanceId: state.openClientInstanceId,
    leaseFields: lease.fields(),
    request: null,
    closeRequested: false,
    reconcileRequired: false,
    status: idleGisMapInferencePortStatusV1(),
    turns: 0,
    pollTimer: null,
    inFlight: false,
    cancelSent: false,
    closed: false,
  };
  retainInferenceApprovalUndo(operation, approvalReceipt);
  expect(testSeams.inferenceApprovalUndoOwner).toMatchObject({ phase: "awaiting-mount", clientInstanceId: state.openClientInstanceId });
  expect(historyStatuses).toEqual([]);
  expect(probe.owner.pageCount).toBe(exact.pageCount);
  await probe.reservation!.installColdPair(probe.owner);
  expect(testSeams.inferenceApprovalUndoOwner).toMatchObject({ phase: "available", clientInstanceId: state.openClientInstanceId });
  expect(historyStatuses.at(-1)?.status).toEqual({ phase: "available", canUndo: true, code: null });
  expect(renderEvents).toEqual(corpus.browserRender.events);
  await probe.reservation!.installColdPair(probe.owner);
  expect(renderEvents).toEqual(corpus.browserRender.events);
  expect(pageIndexes).toEqual(Array.from({ length: exact.pageCount }, (_unused, index) => index));
  expect(Buffer.from(received.subarray(0, pack.byteLength)).equals(Buffer.from(pack))).toBe(true);
  expect(Buffer.from(received.subarray(pack.byteLength)).equals(Buffer.from(spr))).toBe(true);
  expect(statuses.filter((status) => status.code === "renderer-unavailable")).toHaveLength(0);
  expect(patchOffers).toHaveLength(2);
  expect(probe.patchAcknowledged).toBe(true);
  expect(probe.patchRejected).toBe(true);
  expect(documentControls).toEqual(["bind"]);
  return { operation, approvalReceipt, bootstrap };
}

/** 🔁️ Refreshes the host view, offers a ui intent action and queues an early command behind the held patch. */
export async function runViewRefreshAndEarlyCommandPhase({ vitest, dependencies, fakes, workerResponses, fields, uiStore, rendererOracle, state, hostRequest, hostView, visibleViews, probe, createBrowserActorUiIntentRequestV1, binding, documentId, actionFixture, windowKindId, fixture, backboneIngress, createBrowserActorAppCommandRequestV1, commandSequences }: {
  readonly vitest: Vitest;
  readonly dependencies: ServiceCompositionTestDependenciesV1;
  readonly fakes: ReturnType<typeof workerLawFakesV1>;
  readonly workerResponses: BackboneWorkerResponse[];
  readonly fields: Awaited<ReturnType<ReturnType<typeof workerLawFakesV1>["executionTargetLeaseFixture"]>>["manifest"];
  readonly uiStore: InstanceType<typeof import("@semio-tech/framework-os/modules/renderer/engine/elements/UiDocumentStore").UiDocumentStore>;
  readonly rendererOracle: { revision: number; nodeKind: string | null; };
  readonly state: ArtifactState;
  readonly hostRequest: { readonly kind: "browser-actor-view-state"; readonly clientInstanceId: string; readonly scope: { readonly spaceId: "cold-browser-space"; readonly documentId: "cold-browser-pair"; }; readonly viewState: any; };
  readonly hostView: any;
  readonly visibleViews: Record<string, unknown>[];
  readonly probe: ColdPairProbe;
  readonly createBrowserActorUiIntentRequestV1: typeof import("@semio-tech/framework/browser-bundle/action-handoff/intent").createBrowserActorUiIntentRequestV1;
  readonly binding: { readonly kind: "hub"; readonly dataClass: "persistedShared"; readonly baseUrl: string; readonly spaceId: "cold-browser-space"; readonly requestedSurfaceId: string; };
  readonly documentId: string;
  readonly actionFixture: any;
  readonly windowKindId: string;
  readonly fixture: Awaited<ReturnType<ReturnType<typeof workerLawFakesV1>["executionTargetLeaseFixture"]>>;
  readonly backboneIngress: Uint8Array<ArrayBufferLike>[];
  readonly createBrowserActorAppCommandRequestV1: typeof import("@semio-tech/framework/browser-bundle/action-handoff/command").createBrowserActorAppCommandRequestV1;
  readonly commandSequences: number[];
}) {
  const { expect, vi } = vitest;
  const { handleTsRequest } = dependencies;
  const { exactDocumentBackboneMessage } = fakes;
  const mounted = workerResponses.find((message) => message.kind === "browser-actor-ui-mounted") as Extract<BackboneWorkerResponse, { kind: "browser-actor-ui-mounted" }>;
  expect(mounted).toMatchObject({
    scope: fields.scope,
    activeCheckpointId: fields.checkpoint.checkpointId,
    descriptorDigestV1: fields.checkpoint.descriptorDigestV1,
    frontier: fields.checkpoint.baselineFrontier,
    uiRevision: 1,
  });
  expect(uiStore.getRevisionSnapshot()).toBe(rendererOracle.revision);
  expect(uiStore.getState().nodes.get(uiStore.getState().root!)?.component).toMatchObject({ type: "surface", kind: rendererOracle.nodeKind });

  const beforeConfig = JSON.stringify(state.config);
  const beforeFrontier = JSON.stringify(state.frontier);
  handleTsRequest({ ...hostRequest, viewState: { ...hostView, locale: "en", terminology: "native" } });
  await vi.waitFor(() => expect(visibleViews.at(-1)?.locale).toBe("en"));
  expect(visibleViews.at(-1)?.terminology).toBe("native");
  expect(JSON.stringify(state.config)).toBe(beforeConfig);
  expect(JSON.stringify(state.frontier)).toBe(beforeFrontier);
  await vi.waitFor(() => expect((probe.reservation as unknown as { viewRefresh: Promise<void> | null }).viewRefresh).toBeNull());

  const actionRequest = createBrowserActorUiIntentRequestV1(
    {
      scope: { spaceId: binding.spaceId, documentId },
      verifiedSurfaceId: fields.surface.surfaceId,
      appChannelVersion: actionFixture.request.appChannelVersion,
      activationGeneration: probe.reservation!.generation.toString(),
      instanceId: 0,
      surfaceRevision: 1,
      actionSequence: 2,
    },
    windowKindId,
    { ...actionFixture.uiIntent, surface: windowKindId, revision: 1, seq: 9_007_199_254_740_993n },
  );
  handleTsRequest({ ...actionRequest, clientInstanceId: state.openClientInstanceId });
  await vi.waitFor(() => expect(probe.heldActionPatch !== null || workerResponses.some((message) => message.kind === "browser-actor-action-result")).toBe(true));
  expect(workerResponses.find((message) => message.kind === "browser-actor-action-result")).toBeUndefined();
  const remoteEnvelope: MutationEnvelope = {
    id: "remote-direct-turn",
    actor: "remote-actor",
    document: documentId,
    schemaVersion: fixture.manifest.artifact.schema,
    deps: [],
    payloadHash: "unused",
    diff: { schemaId: fixture.manifest.artifact.schema, payload: { source: "remote" } },
    inverse: { targetOperation: "remote-direct-turn", inverseDiff: { schemaId: fixture.manifest.artifact.schema, payload: { source: "before" } }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
  };
  const remoteMessage = exactDocumentBackboneMessage(remoteEnvelope),
    remoteDelivery = probe.reservation!.receiveBackbone(remoteMessage);
  await Promise.resolve();
  expect(backboneIngress).toHaveLength(0);
  const earlyCommandInvocation = {
      ...actionFixture.commandInvocation,
      address: { ...actionFixture.commandInvocation.address, owner: { app: { pluginId: fields.package.pluginId, appId: fields.surface.appId } } },
    },
    earlyCommandView = { ...actionFixture.commandViewState, activeWindowKindId: windowKindId, windowId: windowKindId },
    earlyCommandRequest = createBrowserActorAppCommandRequestV1(
      {
        scope: { spaceId: binding.spaceId, documentId },
        verifiedSurfaceId: fields.surface.surfaceId,
        appChannelVersion: actionFixture.request.appChannelVersion,
        activationGeneration: probe.reservation!.generation.toString(),
        instanceId: 0,
        surfaceRevision: 1,
        actionSequence: 3,
      },
      earlyCommandInvocation,
      earlyCommandView,
    );
  handleTsRequest({ ...earlyCommandRequest, clientInstanceId: state.openClientInstanceId });
  await Promise.resolve();
  await Promise.resolve();
  expect(workerResponses.filter((message) => message.kind === "browser-actor-action-result")).toHaveLength(0);
  expect(commandSequences).toEqual([]);
  return { remoteDelivery, remoteMessage, earlyCommandInvocation, earlyCommandView };
}
