/** 🗺️ GIS document open: the GIS wasm lease, the viewer's renderer-unavailable state and the cold pair transfer with inference approval Undo. */

import { parseGisMapInferenceApprovalReceiptV1, idleGisMapInferencePortStatusV1 } from "@semio-tech/gis-gismap-js/inference/schema";
import { workerLawFakesV1, executionTargetBytes, FakeHubWebSocket, sampleEnvelope } from "@semio-tech/framework-os/testing/worker-law-fakes";
import type { TestSource } from "@semio-tech/framework-os/testing/worker-law-fakes";
import type { ServiceCompositionTestDependenciesV1 } from "../🧰️dependencies/🟦️.ts";
import type { ActorInstanceLifetime, ArtifactEvent, Vitest } from "./📐️document-open-types/🟦️.ts";
import { prepareColdPairFixtures } from "./🧱️cold-pair-fixtures/🟦️.ts";
import { createColdPairWorkerClass } from "./🪤️cold-pair-worker-fake/🟦️.ts";
import { openColdPairSession } from "./🔌️cold-pair-session-setup/🟦️.ts";
import { runColdPairActivationPhase, runViewRefreshAndEarlyCommandPhase } from "./🚚️transfer-and-render-phases/🟦️.ts";
import { runActionPatchAndDirectCommandPhase } from "./⚡️action-and-command-phases/🟦️.ts";
import { runApprovalUndoOwnerPhase, runHostileRenderPhase } from "./↩️undo-owner-phases/🟦️.ts";

export async function registerGisDocumentOpenTests(vitest: Vitest, dependencies: ServiceCompositionTestDependenciesV1, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it, vi } = vitest;
  const fakes = workerLawFakesV1(vitest, dependencies, source);
  const { resetWorkerLaw, executionTargetLeaseFixture, executionTargetHarness, executionTargetBodyResponse, exactDocumentBackboneMessage } = fakes;
  const { testSeams, requestDocumentSocketAuthority, sameLeaseFieldsV1, decodePackValue, encodePackValue, executionTargetSha256Hex, clearHubSessionCapability, installHubSessionCapability, hubSessionFetch, openArtifact, artifactState, artifacts, documentRuntimeKeyForConfig, handleTsRequest, DocumentExecutionTargetLease, documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1, VerifiedColdDocumentPair, reserveDocumentBrowserActorChild, verifiedColdDocumentPairMintToken, parseGisMapInferenceApprovalReceiptV1, executionTargetHex, idleGisMapInferencePortStatusV1, retainInferenceApprovalUndo, reissueInferenceApprovalUndoForRebootstrap, documentRuntimeKeyV1, bindInferenceApprovalUndoToMountedPair, undoInferenceApproval, dropVerifiedColdDocumentPair, closeArtifactRuntime, browserActorChildCapacity, DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1, documentExecutionTargetStatusRoleV1, relayMutationsToHub } = dependencies;
  //#region 🔖️OfflineResilienceTests
  describe("backbone-worker offline resilience", () => {

    beforeEach(resetWorkerLaw);

    /** 🚚️ The backing buffer of a view the worker transfers, refusing a shared one.
     * `Uint8Array.buffer` is an `ArrayBufferLike`, and a `SharedArrayBuffer` is not `Transferable`;
     * `instanceof` is what turns that into a checked narrowing rather than an assertion. */
    function transferableBuffer(bytes: Uint8Array): ArrayBuffer {
      const buffer = bytes.buffer;
      if (!(buffer instanceof ArrayBuffer)) throw new Error("worker transfer requires a non-shared buffer");
      return buffer;
    }

    it("browser execution target lease verifies GIS wasm bytes before plan exchange", async () => {
      const fixture = await executionTargetLeaseFixture();
      const component = executionTargetBytes(fixture.componentHex);
      const descriptor = executionTargetBytes(fixture.descriptorHex);
      const plan = { ...structuredClone(fixture.plan), expiresAtUnixMs: Date.now() + 30_000 };
      const grant = { ...fixture.socketGrant, expiresAtMs: Date.now() + 25_000 };
      const harness = await executionTargetHarness(fixture, (url) => {
        if (url.endsWith("/open-plan")) return Response.json(plan);
        if (url.endsWith("/execution-target/manifest")) return Response.json(fixture.manifest);
        if (url.endsWith("/execution-target/component")) return executionTargetBodyResponse(component);
        if (url.endsWith("/execution-target/descriptor")) return executionTargetBodyResponse(descriptor);
        return Response.json(grant);
      });
      try {
        expect(fixture.plan.surface.rendererTarget).toBe("wasm");
        expect(fixture.manifest.grant.write).toBe(false);
        const authority = await requestDocumentSocketAuthority(harness.state, harness.binding);
        expect(authority.surfaceId).toBe(fixture.manifest.surface.surfaceId);
        expect(harness.requests.map(({ url }) => url)).toEqual([`/_semio/hub${fixture.expected.openPlanPath}`, ...fixture.expected.assetPaths.map((path) => `/_semio/hub${path}`), `/_semio/hub${fixture.expected.socketGrantPath}`]);
        for (const index of [1, 2, 3]) {
          expect(harness.requests[index]!.method).toBe("POST");
          expect(JSON.parse(harness.requests[index]!.body)).toEqual(fixture.intent);
          expect(harness.requests[index]!.body).not.toContain(plan.receipt);
        }
        expect(JSON.parse(harness.requests.at(-1)!.body).planReceipt).toBe(plan.receipt);
        expect(harness.state.executionTargetLease).not.toBeNull();
        expect(harness.state.executionTargetLease!.live).toBe(true);
        expect(sameLeaseFieldsV1(harness.state.executionTargetLease!.fields(), fixture.manifest)).toBe(true);
        const observed = harness.statuses.map(({ code, progress }) => `${code}:${progress?.stage ?? "-"}`);
        expect(observed.at(-1)).toBe(`${fixture.expected.rendererState}:-`);
        expect(new Set(harness.statuses.flatMap(({ progress }) => (progress ? [progress.stage] : [])))).toEqual(new Set(fixture.expected.progressStages));
        const payload = JSON.stringify(harness.statuses);
        for (const fragment of fixture.expected.forbiddenStatusFragments) expect(payload).not.toContain(fragment);
      } finally {
        harness.release();
      }
    });

    it("browser document actor transfers one verified cold pair only after lifecycle ACK and exact page receipts", async () => {
      const { actorBytes, fixture, hostView, descriptorByteSha256, descriptor, exact, publishedPack, publishedSpr, statuses, workerResponses, historyStatuses, probe, patchOffers, uiStore, guest, decodeAppCommand, commandSequences, commandInvocations, commandViews, corpus, pageIndexes, received, windowKindId, panelVisible, sectionVisible, actionSequences, backboneIngress, bodyKey, visibleViews, secondWindowVisible, renderEvents, lifecycleClose, decodeDocumentBackboneControlV1, documentControls, encodeDocumentBackboneControlV1, encodeAppFrame, actionFixture, pack, spr, rendererOracle, createBrowserActorUiIntentRequestV1, createBrowserActorAppCommandRequestV1, decodeBrowserActorHostEffectsV1, originalFetch, originalWorker } = await prepareColdPairFixtures({ fakes, dependencies, source });
      const { state, fields, documentId, lease, binding, sessionFence, hostRequest } = await openColdPairSession({ dependencies, vitest, fakes, source, actorBytes, fixture, hostView, descriptorByteSha256, descriptor, exact, publishedPack, publishedSpr, statuses, workerResponses, historyStatuses, probe, patchOffers, uiStore });
      const { ColdPairWorker } = createColdPairWorkerClass({ vitest, dependencies, fakes, probe, actorBytes, guest, decodeAppCommand, commandSequences, commandInvocations, commandViews, corpus, pageIndexes, received, windowKindId, panelVisible, state, sectionVisible, actionSequences, backboneIngress, exact, bodyKey, visibleViews, secondWindowVisible, renderEvents, lifecycleClose, decodeDocumentBackboneControlV1, documentControls, encodeDocumentBackboneControlV1, fields, documentId, encodeAppFrame, actionFixture, transferableBuffer });
      (globalThis as unknown as { Worker: unknown }).Worker = ColdPairWorker;
      try {
        const { operation, approvalReceipt, bootstrap } = await runColdPairActivationPhase({ dependencies, vitest, probe, state, fields, exact, pack, spr, lease, publishedPack, publishedSpr, documentId, binding, sessionFence, historyStatuses, renderEvents, corpus, pageIndexes, received, statuses, patchOffers, documentControls });
        const { remoteDelivery, remoteMessage, earlyCommandInvocation, earlyCommandView } = await runViewRefreshAndEarlyCommandPhase({ vitest, dependencies, fakes, workerResponses, fields, uiStore, rendererOracle, state, hostRequest, hostView, visibleViews, probe, createBrowserActorUiIntentRequestV1, binding, documentId, actionFixture, windowKindId, fixture, backboneIngress, createBrowserActorAppCommandRequestV1, commandSequences });
        await runActionPatchAndDirectCommandPhase({ vitest, dependencies, visibleViews, probe, uiStore, state, workerResponses, remoteDelivery, decodeBrowserActorHostEffectsV1, actionFixture, actionSequences, backboneIngress, remoteMessage, commandSequences, fields, windowKindId, createBrowserActorAppCommandRequestV1, binding, documentId, commandInvocations, earlyCommandInvocation, commandViews, earlyCommandView });

        const { reissuedOwner, submittingOwner } = await runApprovalUndoOwnerPhase({ dependencies, vitest, operation, approvalReceipt, state, fixture, probe, fields, historyStatuses, lease });
        await runHostileRenderPhase({ vitest, dependencies, reissuedOwner, historyStatuses, submittingOwner, probe, corpus, windowKindId, bodyKey, state, secondWindowVisible, panelVisible, sectionVisible, pageIndexes, bootstrap, exact, renderEvents, received });
      } finally {
        closeArtifactRuntime(state.runtimeKey);
        expect(await probe.reservation?.retirementOutcome).toBe("retired");
        expect(state.browserActorViewState).toBeNull();
        expect(probe.terminated).toBe(1);
        expect(documentControls).toEqual(["bind", "retire"]);
        expect(lifecycleClose).toEqual(["close", "accepted-ack", "retired-ack"]);
        expect(browserActorChildCapacity()).toEqual({ actors: 0, bytes: 0 });
        testSeams.executionTargetStatusObserver = null;
        testSeams.workerPostTestSink = null;
        clearHubSessionCapability();
        globalThis.fetch = originalFetch;
        (globalThis as unknown as { Worker: unknown }).Worker = originalWorker;
      }
    });

    it("browser GIS viewer exposes localized renderer-unavailable after verified lease", async () => {
      const fixture = await executionTargetLeaseFixture();
      const component = executionTargetBytes(fixture.componentHex);
      const descriptor = executionTargetBytes(fixture.descriptorHex);
      const plan = { ...structuredClone(fixture.plan), expiresAtUnixMs: Date.now() + 30_000 };
      const harness = await executionTargetHarness(fixture, (url) => {
        if (url.endsWith("/open-plan")) return Response.json(plan);
        if (url.endsWith("/execution-target/manifest")) return Response.json(fixture.manifest);
        if (url.endsWith("/execution-target/component")) return executionTargetBodyResponse(component);
        if (url.endsWith("/execution-target/descriptor")) return executionTargetBodyResponse(descriptor);
        return Response.json({ ...fixture.socketGrant, expiresAtMs: Date.now() + 25_000 });
      });
      const events: ArtifactEvent[] = [];
      const originalPostMessage = (globalThis as unknown as { postMessage?: unknown }).postMessage;
      try {
        await requestDocumentSocketAuthority(harness.state, harness.binding);
        const terminal = harness.statuses.at(-1)!;
        expect(terminal.code).toBe(fixture.expected.rendererState);
        for (const [code, text] of Object.entries(fixture.expected.status)) {
          expect(DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1[code as keyof typeof DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1]).toEqual(text);
          expect(documentExecutionTargetStatusRoleV1(code as keyof typeof DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1)).toBe(fixture.expected.statusRoles[code]);
        }
        expect(Object.keys(DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1).sort()).toEqual(Object.keys(fixture.expected.status).sort());
        const workerSource = (await import("@semio-tech/framework-compiler-ts/syntax/module-tree")).readModuleTreeSource(new URL("./🔨️modules/🏪️store/👷️worker/🟦️.ts", source.url));
        const leaseRegion = workerSource.slice(workerSource.indexOf("//#region 🪪️ExecutionTargetLease"), workerSource.indexOf("//#endregion 🪪️ExecutionTargetLease"));
        expect(leaseRegion).not.toContain("loadPluginModule");
        expect(leaseRegion).not.toContain("ActivationRegistry");
        expect(leaseRegion).not.toContain("load_wasm_plugins");
        expect(leaseRegion).not.toContain("attach_backbone");
        expect(Object.values(fixture.expected.rendererClaims).every((claim) => claim === false)).toBe(true);
        harness.state.hubActorReady = true;
        const originalEmit = FakeHubWebSocket.instances.length;
        relayMutationsToHub(harness.state, [sampleEnvelope()]);
        expect(harness.state.outbox).toHaveLength(0);
        expect(FakeHubWebSocket.instances).toHaveLength(originalEmit);
        expect(events).toHaveLength(0);
      } finally {
        (globalThis as unknown as { postMessage?: unknown }).postMessage = originalPostMessage;
        harness.release();
      }
    });

  });
  //#endregion 🔖️OfflineResilienceTests
}
