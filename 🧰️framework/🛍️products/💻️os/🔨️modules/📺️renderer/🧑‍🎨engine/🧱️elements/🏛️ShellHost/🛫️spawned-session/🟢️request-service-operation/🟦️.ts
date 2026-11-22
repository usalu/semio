/** 🟢️ The request service operation phase of useSpawnedSession. */
import { DOCUMENT_SERVICE_TOPIC_V1 } from "@semio-tech/framework/inference/service";
import { admitDocumentServiceWireDeclarationV1 } from "@semio-tech/framework/inference/service/io";
import { directorySessionAuthorityIsCurrentV1 } from "@semio-tech/framework/directory/session-refresh";
import { useCallback } from "../../../⚛️react-port/🟦️.ts";
import { type ActionInvocation, type CommandInvocation, type Effect, type HistoryPatch } from "@semio-tech/framework";
import { decodePackWire } from "@semio-tech/framework/packs";
import { encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import type { ViewModel } from "../../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { browserActorWindowConfigDispatchUiScopeV1 } from "../../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION } from "@semio-tech/framework/browser-bundle/action-handoff";
import { decodeBrowserActorHostEffectsV1 } from "@semio-tech/framework/browser-bundle/action-handoff/publication";
import { InferencePortOpeningMailboxV1 } from "@semio-tech/framework/inference/opening";
import { browserActorProgramIsFocusedV1, browserActorProgramIsPublishedV1, browserActorSpawnedRowV1, browserDocumentActorIdentityV1, sameBrowserActorProgramV1 } from "./🪪️owner/🟦️.ts";
import { spawnedProgramViewStateV1 } from "../../🪟️spawned-program/🟦️.ts";
import type { BrowserActorActionResultV1 } from "@semio-tech/framework/browser-bundle/action-handoff";
import type { ShellCoreAfterContributionsAndRefresh } from "../../🔁️contributions-refresh/🟦️.ts";
import type { useDocumentServicesState } from "../🔶️document-services/🟦️.ts";

/** 🔵 Builds requestServiceOperation, directBrowserActorForSession, dispatchDirectBrowserActorCommand of useSpawnedSession. */
export function useRequestServiceOperationState({ documentServices, loadedPluginsRef, inferencePortOwnerRef, inferencePortOpeningRef, openDocumentSessionsRef, verifiedSessionAuthorityRef, identityRef, retiredSessionDocumentOwnersRef, inferencePortEpochRef, ensureBackboneWorker, inferencePortAuthorityRef, dispatch, browserActorUiByRuntimeKeyRef, shellStateRef, applyHistoryPatch, core }: Pick<ReturnType<typeof useDocumentServicesState>, "documentServices" | "loadedPluginsRef" | "inferencePortOwnerRef" | "inferencePortOpeningRef" | "openDocumentSessionsRef" | "verifiedSessionAuthorityRef" | "identityRef" | "retiredSessionDocumentOwnersRef" | "inferencePortEpochRef" | "ensureBackboneWorker" | "inferencePortAuthorityRef" | "dispatch" | "browserActorUiByRuntimeKeyRef" | "shellStateRef" | "applyHistoryPatch"> & { readonly core: ShellCoreAfterContributionsAndRefresh }) {
  const requestServiceOperation = useCallback(async (baseSession: ActiveSession, operation: Extract<Effect, { readonly requestServiceOperation: unknown }>["requestServiceOperation"], admit: () => boolean) => {
    if (!admit()) throw new Error("inference-opening: owner retired");
    if (operation.owner !== baseSession.pluginId) throw new Error("installed-service.foreign-owner");
    const presentations = documentServices.filter((entry) => entry.owner === operation.owner && entry.serviceId === operation.serviceId);
    if (presentations.length !== 1) throw new Error("installed-service.unavailable");
    const program = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === baseSession.pluginId);
    const declarations = (program?.manifest.topicContributions ?? []).filter((entry) => entry.topic === DOCUMENT_SERVICE_TOPIC_V1).map((entry) => admitDocumentServiceWireDeclarationV1(operation.owner, entry.payload));
    if (declarations.filter((entry) => entry.serviceId === operation.serviceId).length !== 1) throw new Error("installed-service.unavailable");
    if (inferencePortOwnerRef.current !== null || inferencePortOpeningRef.current !== null) throw new Error("inference.capacity");
    const owners = [...openDocumentSessionsRef.current.entries()].filter(([, entry]) => entry.session.pluginId === baseSession.pluginId && entry.session.instanceId === baseSession.instanceId && entry.scope !== undefined);
    const owner = owners.length === 1 ? owners[0] : undefined;
    const scope = owner?.[1].scope;
    if (!owner || !scope) throw new Error("inference-proposal: exact document owner required");
    const authority = verifiedSessionAuthorityRef.current;
    if (!directorySessionAuthorityIsCurrentV1(authority, authority) || authority?.userId !== identityRef.current?.userId || retiredSessionDocumentOwnersRef.current.has(owner[1])) throw new Error("inference-proposal: authenticated authority required");
    if (inferencePortEpochRef.current === Number.MAX_SAFE_INTEGER) throw new Error("inference-proposal: epoch exhausted");
    const worker = ensureBackboneWorker();
    const operationEpoch = ++inferencePortEpochRef.current;
    const [runtimeKey] = owner;
    const mailbox = new InferencePortOpeningMailboxV1((request) => worker.postMessage({ wire: encodeBackboneWorkerRequest(request) }));
    const opening = { owner: { owner: operation.owner, serviceId: operation.serviceId, operationEpoch, runtimeKey, scope }, clientInstanceId: owner[1].clientInstanceId, sessionInstanceId: baseSession.instanceId, mailbox };
    inferencePortOpeningRef.current = opening;
    try {
      await mailbox.open({ kind: "service-operation", owner: operation.owner, serviceId: operation.serviceId, action: "open", operationEpoch, payload: { scope } });
      const entry = openDocumentSessionsRef.current.get(runtimeKey);
      if (inferencePortOpeningRef.current !== opening || entry?.clientInstanceId !== opening.clientInstanceId || entry.session.instanceId !== opening.sessionInstanceId || retiredSessionDocumentOwnersRef.current.has(entry) || !directorySessionAuthorityIsCurrentV1(authority, verifiedSessionAuthorityRef.current) || !admit()) throw new Error("inference-opening: owner retired");
      inferencePortOwnerRef.current = opening.owner;
      inferencePortAuthorityRef.current = authority;
      dispatch({ type: "OPEN_INFERENCE_PORT", runtimeKey, operationEpoch });
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "service-operation", owner: operation.owner, serviceId: operation.serviceId, action: operation.action, operationEpoch, payload: operation.payload }) });
    } catch (error) {
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "service-operation", owner: operation.owner, serviceId: operation.serviceId, action: "close", operationEpoch, payload: {} }) });
      throw error;
    } finally {
      if (inferencePortOpeningRef.current === opening) inferencePortOpeningRef.current = null;
      mailbox.close("inference-opening: settled");
    }
  }, [ensureBackboneWorker]);

  const directBrowserActorForSession = useCallback((baseSession: ActiveSession) => {
    const matches = [...openDocumentSessionsRef.current.entries()].filter(([, entry]) =>
      entry.scope !== undefined && sameBrowserActorProgramV1(entry.session, baseSession));
    if (matches.length > 1) throw new Error("action-owner-mismatch: ambiguous document owner");
    const match = matches[0];
    if (!match) return null;
    const [runtimeKey, entry] = match;
    if (retiredSessionDocumentOwnersRef.current.has(entry) || !browserActorProgramIsPublishedV1(shellStateRef.current.pluginRuntime.session, baseSession)) {
      throw new Error("action-owner-mismatch: signed document owner retired");
    }
    const retained = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
    const identity = browserDocumentActorIdentityV1(entry, retained);
    if (!retained || identity === null) throw new Error("action-catching-up: signed document actor is not ready");
    const row = shellStateRef.current.pluginRuntime.session;
    const spawnedId = row === null ? undefined : browserActorSpawnedRowV1(row, baseSession)?.id;
    const plugin = entry.plugin;
    const current = (): boolean => {
      const primary = shellStateRef.current.pluginRuntime.session;
      const live = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
      return openDocumentSessionsRef.current.get(runtimeKey) === entry && !retiredSessionDocumentOwnersRef.current.has(entry)
        && loadedPluginsRef.current.find(candidate => candidate.handle.pluginId === baseSession.pluginId)?.handle === plugin
        && sameBrowserActorProgramV1(entry.session, baseSession) && browserActorProgramIsPublishedV1(primary, baseSession)
        && (primary === null ? undefined : browserActorSpawnedRowV1(primary, baseSession)?.id) === spawnedId
        && live?.actions === retained.actions && browserDocumentActorIdentityV1(entry, live) === identity;
    };
    if (!current()) throw new Error("action-owner-mismatch: signed document owner retired");
    return { runtimeKey, retained, entry, identity, current };
  }, []);
  type Actor = NonNullable<ReturnType<typeof directBrowserActorForSession>>;

  const captureBrowserActorEffectOwner = useCallback((actor: Actor, inputSeq: number | null = null) => {
    const { entry, current, retained, identity } = actor;
    const captured = core.captureEffectOwner(entry.session, core.captureDialogOrigin(shellStateRef.current.pluginRuntime.session), inputSeq);
    return {
      ...captured,
      browserActor: {
        current,
        takeSegmentedDownloadChunk: async (operationId: bigint) => {
          const store = retained.windows.get(identity.windowKindId);
          if (!current() || store === undefined) throw new Error("segmented-download-owner-retired");
          const chunk = await retained.actions.takeSegmentedDownloadChunk({
            scope: retained.scope,
            verifiedSurfaceId: retained.verifiedSurfaceId,
            activationGeneration: retained.activationGeneration,
            appChannelVersion: BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION,
            instanceId: identity.instanceId,
            surfaceRevision: store.getRevisionSnapshot(),
          }, operationId);
          if (!current()) throw new Error("segmented-download-owner-retired");
          return chunk;
        },
        refresh: (session: ActiveSession) => {
          if (current() && browserActorProgramIsFocusedV1(shellStateRef.current.pluginRuntime.session, session)) {
            core.postBrowserActorViewState(ensureBackboneWorker(), entry, session);
          }
        },
      },
    };
  }, [core.captureEffectOwner, core.captureDialogOrigin, core.postBrowserActorViewState, ensureBackboneWorker]);

  const publishBrowserActorResult = useCallback(async (actor: Actor, result: BrowserActorActionResultV1, effectOwner: ReturnType<typeof captureBrowserActorEffectOwner>, actionId?: string) => {
    const { entry, current } = actor;
    if (!current() || !core.isCurrentEffectOwner(effectOwner)) throw new Error("action-owner-mismatch: signed document owner retired");
    const effects = decodeBrowserActorHostEffectsV1(result.hostEffects);
    for (const patch of result.historyPatches) {
      if (!core.isCurrentEffectOwner(effectOwner)) return;
      applyHistoryPatch(decodePackWire(Uint8Array.from(patch), "$.historyPatch") as HistoryPatch, false, { pluginId: entry.session.pluginId, instanceId: entry.session.instanceId });
    }
    const primary = shellStateRef.current.pluginRuntime.session;
    if (!primary || !current()) return;
    const source = entry.session;
    const live = sameBrowserActorProgramV1(primary, source) ? primary : { ...source, viewState: spawnedProgramViewStateV1(primary.viewState, source.app) };
    const dirty = browserActorWindowConfigDispatchUiScopeV1(result, actionId);
    if (effects.length > 0 || dirty.kind !== "none") await core.applyHostEffectsRef.current(effects, live, dirty, effectOwner);
  }, [applyHistoryPatch, core.isCurrentEffectOwner]);

  const dispatchDirectBrowserActorCommand = useCallback(async (
    actor: Actor,
    invocation: ActionInvocation | CommandInvocation,
    viewState: ViewModel,
    inputSeq: number | null = null,
  ) => {
    const { retained, entry, identity, current } = actor;
    if (!current() || core.sealedInstancesRef.current.sealed(entry.plugin, entry.session.pluginId, entry.session.instanceId)) throw new Error("action-owner-mismatch: signed document owner retired");
    const effectOwner = captureBrowserActorEffectOwner(actor, inputSeq);
    const windowKindId = "windowKindId" in invocation.address ? invocation.address.windowKindId : typeof viewState.activeWindowKindId === "string" ? viewState.activeWindowKindId : identity.windowKindId;
    const store = retained.windows.get(windowKindId);
    if (store === undefined) throw new Error("action-catching-up: signed document window is unpainted");
    const release = core.sessionWorkRef.current.begin(entry.session.pluginId, entry.session.instanceId, "typed-operation");
    let result: BrowserActorActionResultV1;
    try {
      result = await retained.actions.dispatchCommand({
        scope: retained.scope,
        verifiedSurfaceId: retained.verifiedSurfaceId,
        activationGeneration: retained.activationGeneration,
        appChannelVersion: BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION,
        instanceId: identity.instanceId,
        surfaceRevision: store.getRevisionSnapshot(),
      }, invocation, viewState);
    } finally {
      release();
    }
    await publishBrowserActorResult(actor, result, effectOwner, "actionId" in invocation.address ? invocation.address.actionId : undefined);
    return result;
  }, [captureBrowserActorEffectOwner, publishBrowserActorResult]);
  return { requestServiceOperation, directBrowserActorForSession, dispatchDirectBrowserActorCommand, captureBrowserActorEffectOwner, publishBrowserActorResult };
}
