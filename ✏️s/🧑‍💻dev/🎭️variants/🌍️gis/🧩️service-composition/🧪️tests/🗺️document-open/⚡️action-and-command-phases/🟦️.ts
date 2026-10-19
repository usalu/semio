/** ⚡️ The view refresh, ui intent action and early command phases of the cold pair transfer.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | registerGisDocumentOpenTests}: a phase of the cold pair transfer test over the same registrar bindings, moved verbatim. */
import { workerLawFakesV1 } from "@semio-tech/framework-os/testing/worker-law-fakes";
import type { ServiceCompositionTestDependenciesV1 } from "../../🧰️dependencies/🟦️.ts";
import type { ColdPairProbe, ArtifactState, BackboneWorkerResponse, Vitest } from "../📐️document-open-types/🟦️.ts";

/** 🎬️ Applies the held action patch, answers the early command and runs the direct action and command invocations. */
export async function runActionPatchAndDirectCommandPhase({ vitest, dependencies, visibleViews, probe, uiStore, state, workerResponses, remoteDelivery, decodeBrowserActorHostEffectsV1, actionFixture, actionSequences, backboneIngress, remoteMessage, commandSequences, fields, windowKindId, createBrowserActorAppCommandRequestV1, binding, documentId, commandInvocations, earlyCommandInvocation, commandViews, earlyCommandView }: {
  readonly vitest: Vitest;
  readonly dependencies: ServiceCompositionTestDependenciesV1;
  readonly visibleViews: Record<string, unknown>[];
  readonly probe: ColdPairProbe;
  readonly uiStore: InstanceType<typeof import("@semio-tech/framework-os/modules/renderer/engine/elements/UiDocumentStore").UiDocumentStore>;
  readonly state: ArtifactState;
  readonly workerResponses: BackboneWorkerResponse[];
  readonly remoteDelivery: Promise<void>;
  readonly decodeBrowserActorHostEffectsV1: typeof import("@semio-tech/framework/browser-bundle/action-handoff/publication").decodeBrowserActorHostEffectsV1;
  readonly actionFixture: any;
  readonly actionSequences: bigint[];
  readonly backboneIngress: Uint8Array<ArrayBufferLike>[];
  readonly remoteMessage: Uint8Array<ArrayBufferLike>;
  readonly commandSequences: number[];
  readonly fields: Awaited<ReturnType<ReturnType<typeof workerLawFakesV1>["executionTargetLeaseFixture"]>>["manifest"];
  readonly windowKindId: string;
  readonly createBrowserActorAppCommandRequestV1: typeof import("@semio-tech/framework/browser-bundle/action-handoff/command").createBrowserActorAppCommandRequestV1;
  readonly binding: { readonly kind: "hub"; readonly dataClass: "persistedShared"; readonly baseUrl: string; readonly spaceId: "cold-browser-space"; readonly requestedSurfaceId: string; };
  readonly documentId: string;
  readonly commandInvocations: Record<string, unknown>[];
  readonly earlyCommandInvocation: any;
  readonly commandViews: Record<string, unknown>[];
  readonly earlyCommandView: any;
}) {
  const { expect, vi } = vitest;
  const { handleTsRequest } = dependencies;
  const windowRendersBeforeActions = visibleViews.length;
  const actionPatch = probe.heldActionPatch!;
  const actionApplied = uiStore.applyPatch(actionPatch.patches[0]!);
  expect(actionApplied.ok).toBe(true);
  handleTsRequest({
    kind: "browser-actor-ui-patch-result",
    clientInstanceId: state.openClientInstanceId,
    scope: actionPatch.scope,
    verifiedSurfaceId: actionPatch.verifiedSurfaceId,
    activationGeneration: actionPatch.activationGeneration,
    instanceId: actionPatch.instanceId,
    receipt: actionPatch.receipt,
    verdicts: [{ surface: actionPatch.patches[0]!.surface, outcome: "acknowledged", revision: uiStore.getRevisionSnapshot() }],
  });
  await vi.waitFor(() => expect(workerResponses.some((message) => message.kind === "browser-actor-action-result")).toBe(true));
  await remoteDelivery;
  const actionResult = workerResponses.find((message) => message.kind === "browser-actor-action-result") as Extract<BackboneWorkerResponse, { kind: "browser-actor-action-result" }>;
  expect(actionResult).toMatchObject({
    clientInstanceId: state.openClientInstanceId,
    actionSequence: 2,
    surfaceRevision: 1,
    outcome: "guest-applied",
    mutationCount: 0,
  });
  expect(decodeBrowserActorHostEffectsV1(actionResult.hostEffects)).toEqual([actionFixture.publication.projectedEffect]);
  expect(actionSequences).toEqual([9_007_199_254_740_993n]);
  expect(probe.actionPatchAcknowledged).toBe(true);
  expect(backboneIngress).toHaveLength(1);
  expect(backboneIngress[0]).toEqual(remoteMessage);
  expect(workerResponses.some((message) => message.kind === "event" && message.event.kind === "documentBackbone")).toBe(false);
  await vi.waitFor(() => expect(workerResponses.filter((message) => message.kind === "browser-actor-action-result")).toHaveLength(2));
  const earlyCommandResult = workerResponses.filter((message): message is Extract<BackboneWorkerResponse, { kind: "browser-actor-action-result" }> => message.kind === "browser-actor-action-result").at(-1)!;
  expect(earlyCommandResult).toMatchObject({ actionSequence: 3, surfaceRevision: 1, outcome: "guest-applied", mutationCount: 1 });
  expect(decodeBrowserActorHostEffectsV1(earlyCommandResult.hostEffects)).toEqual([actionFixture.publication.projectedEffect]);
  expect(visibleViews.length - windowRendersBeforeActions).toBe(1);
  expect(commandSequences).toEqual([3]);
  expect(backboneIngress).toHaveLength(1);

  const directActionInvocation = {
      ...actionFixture.actionInvocation,
      address: { ...actionFixture.actionInvocation.address, pluginId: fields.package.pluginId, appId: fields.surface.appId, windowKindId, windowInstanceId: windowKindId },
    },
    directCommandInvocation = {
      ...actionFixture.commandInvocation,
      address: { ...actionFixture.commandInvocation.address, owner: { app: { pluginId: fields.package.pluginId, appId: fields.surface.appId } } },
    },
    directCommandView = { ...actionFixture.commandViewState, activeWindowKindId: windowKindId, windowId: windowKindId };
  for (const [index, invocation] of [directActionInvocation, directCommandInvocation].entries()) {
    const commandRequest = createBrowserActorAppCommandRequestV1(
      {
        scope: { spaceId: binding.spaceId, documentId },
        verifiedSurfaceId: fields.surface.surfaceId,
        appChannelVersion: actionFixture.request.appChannelVersion,
        activationGeneration: probe.reservation!.generation.toString(),
        instanceId: 0,
        surfaceRevision: 2,
        actionSequence: index + 4,
      },
      invocation,
      directCommandView,
    );
    handleTsRequest({ ...commandRequest, clientInstanceId: state.openClientInstanceId });
    await vi.waitFor(() => expect(workerResponses.filter((message) => message.kind === "browser-actor-action-result")).toHaveLength(index + 3));
    const commandResult = workerResponses.filter((message): message is Extract<BackboneWorkerResponse, { kind: "browser-actor-action-result" }> => message.kind === "browser-actor-action-result").at(-1)!;
    expect(commandResult).toMatchObject({ actionSequence: index + 4, surfaceRevision: 2, outcome: "guest-applied", mutationCount: 0 });
    expect(decodeBrowserActorHostEffectsV1(commandResult.hostEffects)).toEqual([actionFixture.publication.projectedEffect]);
  }
  expect(commandSequences).toEqual([3, 4, 5]);
  expect(visibleViews.length - windowRendersBeforeActions).toBe(3);
  expect(commandInvocations).toEqual([earlyCommandInvocation, directActionInvocation, directCommandInvocation]);
  expect(commandViews).toEqual([earlyCommandView, directCommandView, directCommandView]);
}
