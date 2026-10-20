/** 🟢️ The request service operation phase of useSpawnedSession. */
import { DOCUMENT_SERVICE_TOPIC_V1 } from "@semio-tech/framework/inference/service";
import { admitDocumentServiceWireDeclarationV1 } from "@semio-tech/framework/inference/service/io";
import { shellDialogSessionIsCurrentV1 } from "../../🗨️dialog-origin/🟦️.ts";
import { directorySessionAuthorityIsCurrentV1 } from "@semio-tech/framework/directory/session-refresh";
import { useCallback } from "react";
import { type ActionInvocation, type CommandInvocation, type Effect, type HistoryPatch } from "@semio-tech/framework";
import { decodePackWire } from "@semio-tech/framework/packs";
import { encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import type { ViewModel } from "../../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { browserActorWindowConfigDispatchUiScopeV1 } from "../../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION } from "@semio-tech/framework/browser-bundle/action-handoff";
import { publishBrowserActorHostEffectsV1 } from "@semio-tech/framework/browser-bundle/action-handoff/publication";
import { InferencePortOpeningMailboxV1 } from "@semio-tech/framework/inference/opening";
import type { useDocumentServicesState } from "../🔶️document-services/🟦️.ts";

/** 🔵 Builds requestServiceOperation, directBrowserActorForSession, dispatchDirectBrowserActorCommand of useSpawnedSession. */
export function useRequestServiceOperationState({ documentServices, loadedPluginsRef, inferencePortOwnerRef, inferencePortOpeningRef, openDocumentSessionsRef, verifiedSessionAuthorityRef, identityRef, retiredSessionDocumentOwnersRef, inferencePortEpochRef, ensureBackboneWorker, inferencePortAuthorityRef, dispatch, browserActorUiByRuntimeKeyRef, shellStateRef, applyHistoryPatch, applyLeftoverInteractionView, refreshUi, leftoverReplaceRefreshBodiesV1 }: Pick<ReturnType<typeof useDocumentServicesState>, "documentServices" | "loadedPluginsRef" | "inferencePortOwnerRef" | "inferencePortOpeningRef" | "openDocumentSessionsRef" | "verifiedSessionAuthorityRef" | "identityRef" | "retiredSessionDocumentOwnersRef" | "inferencePortEpochRef" | "ensureBackboneWorker" | "inferencePortAuthorityRef" | "dispatch" | "browserActorUiByRuntimeKeyRef" | "shellStateRef" | "applyHistoryPatch" | "applyLeftoverInteractionView" | "refreshUi" | "leftoverReplaceRefreshBodiesV1">) {
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
    const matches = [...browserActorUiByRuntimeKeyRef.current.entries()].flatMap(([runtimeKey, retained]) => {
      const entry = openDocumentSessionsRef.current.get(runtimeKey);
      return retained.identity !== null && entry?.clientInstanceId === retained.clientInstanceId && entry.session.pluginId === baseSession.pluginId && entry.session.instanceId === baseSession.instanceId && retained.sessionInstanceId === baseSession.instanceId && shellDialogSessionIsCurrentV1(shellStateRef.current.pluginRuntime.session, entry.session)
        ? [{ runtimeKey, retained, entry, identity: retained.identity }]
        : [];
    });
    if (matches.length > 1) throw new Error("browser-actor-action: ambiguous document owner");
    return matches[0] ?? null;
  }, []);

  const dispatchDirectBrowserActorCommand = useCallback(async (
    owned: NonNullable<ReturnType<typeof directBrowserActorForSession>>,
    invocation: ActionInvocation | CommandInvocation,
    viewState: ViewModel,
  ) => {
    const { runtimeKey, retained, entry, identity } = owned;
    const current = (): boolean => {
      const live = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
      return live?.actions === retained.actions && live.identity === identity && openDocumentSessionsRef.current.get(runtimeKey) === entry && shellDialogSessionIsCurrentV1(shellStateRef.current.pluginRuntime.session, entry.session);
    };
    if (!current()) throw new Error("browser-actor-action: owner retired");
    const windowKindId = "windowKindId" in invocation.address ? invocation.address.windowKindId : typeof viewState.activeWindowKindId === "string" ? viewState.activeWindowKindId : identity.windowKindId;
    const store = retained.windows.get(windowKindId);
    if (store === undefined) throw new Error("browser-actor-action: window unpainted");
    const result = await retained.actions.dispatchCommand({
      scope: retained.scope,
      verifiedSurfaceId: retained.verifiedSurfaceId,
      activationGeneration: retained.activationGeneration,
      appChannelVersion: BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION,
      instanceId: identity.instanceId,
      surfaceRevision: store.getRevisionSnapshot(),
    }, invocation, viewState);
    for (const patch of result.historyPatches) if (current()) applyHistoryPatch(decodePackWire(Uint8Array.from(patch), "$.historyPatch") as HistoryPatch, false, { pluginId: entry.session.pluginId, instanceId: entry.session.instanceId });
    await publishBrowserActorHostEffectsV1(result.hostEffects, current, async (effect) => {
      if ("requestServiceOperation" in effect) await requestServiceOperation(entry.session, effect.requestServiceOperation, current);
      else window.open(effect.openExternalUrl.url, "_blank", "noopener,noreferrer");
    });
    const actionId = "actionId" in invocation.address ? invocation.address.actionId : undefined;
    applyLeftoverInteractionView(("output" in result ? (result as { output?: unknown }).output : undefined) ?? null);
    const dirty = browserActorWindowConfigDispatchUiScopeV1(result, actionId);
    if (dirty.kind !== "none" && current()) {
      await refreshUi({ ...entry.session, viewState }, dirty, undefined, leftoverReplaceRefreshBodiesV1());
    }
    return result;
  }, [applyHistoryPatch, directBrowserActorForSession, refreshUi, requestServiceOperation]);
  return { requestServiceOperation, directBrowserActorForSession, dispatchDirectBrowserActorCommand };
}
