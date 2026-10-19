/** ↩️ The direct action, approval undo owner and hostile render phases of the cold pair transfer.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | registerGisDocumentOpenTests}: a phase of the cold pair transfer test over the same registrar bindings, moved verbatim. */
import { stubFetch } from "@semio-tech/framework-os/testing/fetch-stub";
import { parseGisMapInferenceApprovalReceiptV1 } from "@semio-tech/gis-gismap-js/inference/schema";
import { workerLawFakesV1, executionTargetBytes } from "@semio-tech/framework-os/testing/worker-law-fakes";
import type { ServiceCompositionTestDependenciesV1, InferenceOperationV1 } from "../../🧰️dependencies/🟦️.ts";
import type { ColdPairProbe, ArtifactState, BackboneWorkerResponse, BrowserActorChildValue, DocumentBrowserActorChild, ResolvedPluginViewState, WireArtifactBootstrap, Vitest } from "../📐️document-open-types/🟦️.ts";

/** 🔙️ Rebinds the approval undo owner across rebootstrap, share lease rotation and an in-flight undo. */
export async function runApprovalUndoOwnerPhase({ dependencies, vitest, operation, approvalReceipt, state, fixture, probe, fields, historyStatuses, lease }: {
  readonly dependencies: ServiceCompositionTestDependenciesV1;
  readonly vitest: Vitest;
  readonly operation: InferenceOperationV1;
  readonly approvalReceipt: ReturnType<typeof parseGisMapInferenceApprovalReceiptV1>;
  readonly state: ArtifactState;
  readonly fixture: Awaited<ReturnType<ReturnType<typeof workerLawFakesV1>["executionTargetLeaseFixture"]>>;
  readonly probe: ColdPairProbe;
  readonly fields: Awaited<ReturnType<ReturnType<typeof workerLawFakesV1>["executionTargetLeaseFixture"]>>["manifest"];
  readonly historyStatuses: Extract<BackboneWorkerResponse, { kind: "inference-history-status" }>[];
  readonly lease: InstanceType<ServiceCompositionTestDependenciesV1["DocumentExecutionTargetLease"]>;
}) {
  const { retainInferenceApprovalUndo, reissueInferenceApprovalUndoForRebootstrap, testSeams, documentRuntimeKeyV1, bindInferenceApprovalUndoToMountedPair, DocumentExecutionTargetLease, documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1, undoInferenceApproval } = dependencies;
  const { expect, vi } = vitest;
  retainInferenceApprovalUndo(operation, approvalReceipt);
  reissueInferenceApprovalUndoForRebootstrap(state);
  const ownerAwaitingA = testSeams.inferenceApprovalUndoOwner!;
  const stateB: ArtifactState = {
    ...state,
    runtimeKey: documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", spaceId: "unrelated-space", documentId: "unrelated-document" }),
    config: { ...state.config, documentId: "unrelated-document", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: fixture.hubOrigin, spaceId: "unrelated-space" }] },
    openClientInstanceId: "unrelated-client",
    browserActorReservation: probe.reservation,
    verifiedColdPair: probe.owner,
  };
  bindInferenceApprovalUndoToMountedPair(stateB, probe.reservation!, probe.owner!);
  expect(testSeams.inferenceApprovalUndoOwner).toBe(ownerAwaitingA);
  expect(testSeams.inferenceApprovalUndoOwner?.phase).toBe("awaiting-mount");
  probe.reservation!.bindApprovalUndoIfMounted();
  expect(testSeams.inferenceApprovalUndoOwner?.phase).toBe("available");

  for (const field of ["directoryRevision", "membershipGeneration", "sessionGeneration"] as const) {
    retainInferenceApprovalUndo(operation, approvalReceipt);
    const originalOwner = testSeams.inferenceApprovalUndoOwner!;
    reissueInferenceApprovalUndoForRebootstrap(state);
    const reissued = testSeams.inferenceApprovalUndoOwner!;
    expect(originalOwner.abort.signal.aborted).toBe(true);
    expect(reissued).toMatchObject({ phase: "awaiting-mount", idempotencyKey: originalOwner.idempotencyKey });
    const rotatedFields = structuredClone(fields);
    rotatedFields.revalidation[field] = (rotatedFields.revalidation[field] ?? 0) + 1;
    const rotatedLease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1(rotatedFields), fixture.hubOrigin, executionTargetBytes(fixture.componentHex), executionTargetBytes(fixture.descriptorHex));
    state.executionTargetLease = rotatedLease;
    probe.reservation!.bindApprovalUndoIfMounted();
    expect(testSeams.inferenceApprovalUndoOwner).toBeNull();
    expect(historyStatuses.at(-1)?.status).toEqual({ phase: "unavailable", canUndo: false, code: null });
    rotatedLease.drop();
    state.executionTargetLease = lease;
  }

  const shareFields = structuredClone(fields);
  delete shareFields.revalidation.sessionGeneration;
  shareFields.revalidation.shareGeneration = 1;
  const shareLease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1(shareFields), fixture.hubOrigin, executionTargetBytes(fixture.componentHex), executionTargetBytes(fixture.descriptorHex));
  state.executionTargetLease = shareLease;
  retainInferenceApprovalUndo(operation, approvalReceipt);
  reissueInferenceApprovalUndoForRebootstrap(state);
  const rotatedShareFields = structuredClone(shareFields);
  rotatedShareFields.revalidation.shareGeneration = (rotatedShareFields.revalidation.shareGeneration ?? 0) + 1;
  const rotatedShareLease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1(rotatedShareFields), fixture.hubOrigin, executionTargetBytes(fixture.componentHex), executionTargetBytes(fixture.descriptorHex));
  state.executionTargetLease = rotatedShareLease;
  probe.reservation!.bindApprovalUndoIfMounted();
  expect(testSeams.inferenceApprovalUndoOwner).toBeNull();
  rotatedShareLease.drop();
  shareLease.drop();
  state.executionTargetLease = lease;

  retainInferenceApprovalUndo(operation, approvalReceipt);
  probe.reservation!.bindApprovalUndoIfMounted();
  const submittingOwner = testSeams.inferenceApprovalUndoOwner!;
  expect(submittingOwner.phase).toBe("available");
  const answerUndo: { resolve: ((response: Response) => void) | null } = { resolve: null };
  globalThis.fetch = stubFetch(async (input) => {
    expect(String(input).endsWith("/inference/gis-map/approval-undos")).toBe(true);
    return await new Promise<Response>((resolve) => {
      answerUndo.resolve = resolve;
    });
  });
  const pendingUndo = undoInferenceApproval(submittingOwner.historyEpoch, submittingOwner.clientInstanceId, submittingOwner.scope);
  await vi.waitFor(() => {
    expect(testSeams.inferenceApprovalUndoOwner?.phase).toBe("submitting");
    expect(answerUndo.resolve).not.toBeNull();
  });
  reissueInferenceApprovalUndoForRebootstrap(state);
  const reissuedOwner = testSeams.inferenceApprovalUndoOwner!;
  expect(reissuedOwner.idempotencyKey).toBe(submittingOwner.idempotencyKey);
  expect(submittingOwner.abort.signal.aborted).toBe(true);
  const respondUndo = answerUndo.resolve;
  if (respondUndo === null) throw new Error("approval undo fetch did not enter");
  respondUndo(
    Response.json(
      {
        schema: "semio.hub.gis-map-approval-undo-receipt/v1",
        targetId: approvalReceipt.undo.targetId,
        originalJobId: approvalReceipt.jobId,
        mutationId: "6".repeat(32),
        commandHash: "7".repeat(64),
        applied: true,
        replayed: false,
        frontier: { ...approvalReceipt.undo.expectedCurrent, headEditOrdinal: approvalReceipt.undo.expectedCurrent.headEditOrdinal + 1, headEditId: "undo-edit" },
      },
      {},
    ),
  );
  await pendingUndo;
  return { reissuedOwner, submittingOwner };
}

/** 🧨️ Rejects hostile browser render rows and closes the render driver child. */
export async function runHostileRenderPhase({ vitest, dependencies, reissuedOwner, historyStatuses, submittingOwner, probe, corpus, windowKindId, bodyKey, state, secondWindowVisible, panelVisible, sectionVisible, pageIndexes, bootstrap, exact, renderEvents, received }: {
  readonly vitest: Vitest;
  readonly dependencies: ServiceCompositionTestDependenciesV1;
  readonly reissuedOwner: NonNullable<ServiceCompositionTestDependenciesV1["testSeams"]["inferenceApprovalUndoOwner"]>;
  readonly historyStatuses: Extract<BackboneWorkerResponse, { kind: "inference-history-status" }>[];
  readonly submittingOwner: NonNullable<ServiceCompositionTestDependenciesV1["testSeams"]["inferenceApprovalUndoOwner"]>;
  readonly probe: ColdPairProbe;
  readonly corpus: any;
  readonly windowKindId: string;
  readonly bodyKey: string;
  readonly state: ArtifactState;
  readonly secondWindowVisible: (view: ResolvedPluginViewState) => { tag: string; val: { surface: { instance: number; surface: string; }; bodyKey: string; viewState: Uint8Array<ArrayBuffer>; }; };
  readonly panelVisible: (view: ResolvedPluginViewState) => { tag: string; val: { surface: { instance: number; surface: string; }; bodyKey: string; viewState: Uint8Array<ArrayBuffer>; }; }[];
  readonly sectionVisible: (view: ResolvedPluginViewState, mount: boolean) => { tag: string; val: { surface: { instance: number; surface: string; }; bodyKey: string; viewState: Uint8Array<ArrayBuffer>; }; }[];
  readonly pageIndexes: number[];
  readonly bootstrap: WireArtifactBootstrap;
  readonly exact: { packLength: number; sprLength: number; pageCount: number; packPattern: { multiplier: number; addend: number; }; sprPattern: { multiplier: number; addend: number; }; packSha256: string; sprSha256: string; aggregateSha256: string; };
  readonly renderEvents: string[];
  readonly received: Uint8Array<ArrayBuffer>;
}) {
  const { expect } = vitest;
  const { testSeams, encodePackValue, dropVerifiedColdDocumentPair } = dependencies;
  expect(testSeams.inferenceApprovalUndoOwner).toBe(reissuedOwner);
  expect(historyStatuses.some((message) => message.historyEpoch === submittingOwner.historyEpoch && message.status.phase === "applied")).toBe(false);
  expect(testSeams.inferenceApprovalUndoOwner?.phase).toBe("awaiting-mount");

  const renderDriver = probe.reservation as unknown as { child: DocumentBrowserActorChild; renderSurface(child: DocumentBrowserActorChild, assertCurrent: () => void): Promise<void> };
  const retainedChild = renderDriver.child;
  for (const row of corpus.browserRender.hostile) {
    let turns = 0,
      current = row.name !== "stale-before-turn";
    const child = {
      async invoke(path: readonly string[], args: BrowserActorChildValue[]): Promise<BrowserActorChildValue> {
        expect(path).toEqual(["reactor", "poll"]);
        const events = args[0] as Record<string, any>[];
        expect(events).toEqual(turns === 0 ? [{ tag: "surface-visible", val: { surface: { instance: 0, surface: windowKindId }, bodyKey, viewState: encodePackValue(state.browserActorViewState!) } }, secondWindowVisible(state.browserActorViewState!), ...panelVisible(state.browserActorViewState!), ...sectionVisible(state.browserActorViewState!, false)] : [{ tag: "wake" }]);
        turns++;
        if (row.name === "stale-after-turn") current = false;
        return {
          uiPatches: [],
          uiPatchReceipt: null,
          lifecycleReceipt: null,
          status: { tag: row.name === "faulted-turn" ? "faulted" : "more-work" },
          coldPairIngress: row.name === "cold-ingress" ? { tag: "page-accepted", val: { lifetime: probe.activeLifetime!, transferGeneration: probe.owner!.transferGeneration, pageIndex: 0, pageCount: probe.owner!.pageCount } } : { tag: "idle" },
        };
      },
    } as unknown as DocumentBrowserActorChild;
    renderDriver.child = child;
    try {
      await expect(
        renderDriver.renderSurface(child, () => {
          if (!current) throw new Error("fixture-stale-render");
        }),
      ).rejects.toThrow(row.error);
    } finally {
      renderDriver.child = retainedChild;
    }
    expect(turns).toBe(row.turns);
  }
  dropVerifiedColdDocumentPair(state);
  expect(() => probe.owner!.page({ activationGeneration: probe.reservation!.generation, instanceId: 0, guestLifetime: 1n }, 0)).toThrow("stale owner");
  expect({ loaded: probe.loaded, described: probe.described, lifecycleAcknowledged: probe.lifecycleAcknowledged, pages: pageIndexes.length, networkChunks: bootstrap.chunk_count, terminated: probe.terminated }).toEqual({
    loaded: 1,
    described: 1,
    lifecycleAcknowledged: true,
    pages: exact.pageCount,
    networkChunks: 2,
    terminated: 0,
  });
  expect(probe.panelRefreshes).toBeGreaterThan(0);
  console.log(
    `browser-cold-pair-transfer: lazy-render-events=${renderEvents.join(",")} pages=${pageIndexes.length} bytes=${received.byteLength} lifecycle-ack=1 applied=1 patch-ack=1 patch-rejected=1 tiled-map=1 network-chunks=${bootstrap.chunk_count}`,
  );
}
