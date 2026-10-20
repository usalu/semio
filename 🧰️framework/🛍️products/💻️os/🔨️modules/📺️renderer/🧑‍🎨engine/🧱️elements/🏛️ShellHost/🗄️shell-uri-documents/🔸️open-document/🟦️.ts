/** 🔸️ The open document phase of useShellUriAndDocuments. */
import { FolderArchivePersistenceV1, FolderReadBackRouteV1, LatestDocumentReplacementV1, parkDocumentOpeningReplacementV1, runDocumentOpeningAttemptV1, settleDocumentOpeningReplacementV1, type DocumentOpeningReceiptV1 } from "../../🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import { createArtifactCreationCatalogMountV1 } from "../../🌱️artifact-creation/🚪️ready-opening/🟦️.ts";
import { useCallback, useEffect } from "react";
import { type DocumentScope } from "@semio-tech/framework/document/scope";
import { encodeDocumentArchiveBytes, type DocumentArchivePack } from "@semio-tech/framework/wire-contracts";
import { type BackboneWorkerRequest, encodeBackboneWorkerRequest, type PersistenceBinding, documentRuntimeKeyV1 } from "@semio-tech/framework/backbone";
import { SpaceDirectoryHistoryV1 } from "../../📇️space-directory/🟦️.ts";
import { resolveDocumentOpeningBindings, resolveDocumentOpeningTarget, type DocumentOpeningReference } from "../../🧭️opening/🟦️.ts";
import { documentOpeningDeadlineMs } from "@semio-tech/framework/browser-bundle/child/schema";
import { canonicalSurfaceId } from "../../🔐️identity-binding/🟦️.ts";
import { SPACE_INDEX_DOCUMENT_ID } from "../../🧾️space-artifact-creation/🟦️.ts";
import type { OpenDocumentSessionTarget, OpenDocumentSession } from "../../🔒️identity-document-ports/🧰️identity-port-types/🟦️.ts";
import type { useScopeState } from "../🔶️scope/🟦️.ts";

/** 🔹 Builds openDocument of useShellUriAndDocuments. */
export function useOpenDocument({ resolveSyncTargetSession, loadedPlugins, ensureBackboneWorker, identityRef, hubEnv, openDocumentSessionsRef, browserActorUiByRuntimeKeyRef, retireBrowserActorUi, shellActorIdRef, socketActorReadyRef, postBrowserActorViewState, retireDocumentAttachment, spaceDirectoryHistoriesRef, directoryScopedOwnersRef, setSpaceArtifactCreationCatalog, setSpaceArtifactCreationCatalogUi, documentAttachmentLane, bindDocumentBackbone, dispatch, backboneWorkerRef, closeDocumentRef, openDocumentRef, sessionRef, activeToolId, activeUtilityByWindowId, extraWindowInstances, session }: Pick<ReturnType<typeof useScopeState>, "resolveSyncTargetSession" | "loadedPlugins" | "ensureBackboneWorker" | "identityRef" | "hubEnv" | "openDocumentSessionsRef" | "browserActorUiByRuntimeKeyRef" | "retireBrowserActorUi" | "shellActorIdRef" | "socketActorReadyRef" | "postBrowserActorViewState" | "retireDocumentAttachment" | "spaceDirectoryHistoriesRef" | "directoryScopedOwnersRef" | "setSpaceArtifactCreationCatalog" | "setSpaceArtifactCreationCatalogUi" | "documentAttachmentLane" | "bindDocumentBackbone" | "dispatch" | "backboneWorkerRef" | "closeDocumentRef" | "openDocumentRef" | "sessionRef" | "activeToolId" | "activeUtilityByWindowId" | "extraWindowInstances" | "session">) {
  /** 🧵️ Opens this exact document and optional shared space in its captured plugin session.
   * Shared requests require a signed-in identity and carry only a requested surface; the worker
   * verifies the Hub plan and catalog assets before acquiring socket authority. Explicit bindings
   * are used by the manual persistence picker. Local documents never inherit the current route. */
  const openDocument = useCallback(
    async (ref: DocumentOpeningReference, bindings?: readonly PersistenceBinding[], target?: OpenDocumentSessionTarget): Promise<DocumentOpeningReceiptV1 | null> => {
      const documentTarget = resolveDocumentOpeningTarget(target, resolveSyncTargetSession(), loadedPlugins);
      if (!documentTarget) return null;
      const { session: targetSession, plugin } = documentTarget;
      const worker = ensureBackboneWorker();
      const resolvedBindings = bindings ?? resolveDocumentOpeningBindings(ref, {
        identity: identityRef.current,
        dataDir: hubEnv?.dataDir,
        surface: targetSession.app.dialect ? canonicalSurfaceId(targetSession.app.dialect, targetSession.app.role) : undefined,
      });
      const hubBinding = resolvedBindings.find((binding): binding is Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }> => binding.kind === "hub");
      if (target?.expectedCatalogGenerationId !== undefined && hubBinding === undefined) return null;
      const creationMount = target?.expectedCatalogGenerationId === undefined ? null : createArtifactCreationCatalogMountV1(target.expectedCatalogGenerationId);
      const scope: DocumentScope | undefined = hubBinding === undefined ? undefined : { spaceId: hubBinding.spaceId, documentId: ref.documentId };
      const spaceIndexDocument = ref.documentId === SPACE_INDEX_DOCUMENT_ID;
      const workerBindings = spaceIndexDocument ? resolvedBindings.filter((binding) => binding.kind !== "hub") : resolvedBindings;
      const runtimeKey = scope === undefined ? ref.documentId : documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...scope });
      const openingAttempt = { clientInstanceId: crypto.randomUUID() };
      const { clientInstanceId } = openingAttempt;
      let resolveReady!: () => void, rejectReady!: (error: Error) => void;
      const ready = new Promise<void>((resolve, reject) => { resolveReady = resolve; rejectReady = reject; });
      void ready.catch(() => {});
      const persistToFolder = resolvedBindings.some((binding) => binding.kind === "folder");
      const persistence = new FolderArchivePersistenceV1(async () => {
        if (!persistToFolder || openDocumentSessionsRef.current.get(runtimeKey) !== entry) return;
        if (!plugin.readAppDocumentArchive) throw new Error("document-backbone.archive-reader-unavailable");
        const archive = await plugin.readAppDocumentArchive(targetSession.instanceId);
        if (openDocumentSessionsRef.current.get(runtimeKey) !== entry) return;
        const archiveBytes = encodeDocumentArchiveBytes(archive);
        worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId: entry.documentId, clientInstanceId: entry.clientInstanceId, ...(entry.scope === undefined ? {} : { spaceId: entry.scope.spaceId }), message: { kind: "localDocumentArchive", archive: Array.from(archiveBytes) } }) });
      });
      const entry: OpenDocumentSession = { openingReference: { ...ref }, persistenceBindings: [...resolvedBindings], session: targetSession, plugin, documentId: ref.documentId, clientInstanceId, ...(scope === undefined ? {} : { scope }), port: null, pending: [], pendingBytes: 0, replacements: new LatestDocumentReplacementV1(), persistence, readBack: new FolderReadBackRouteV1<DocumentArchivePack>(persistence), creationMount, ready, resolveReady, rejectReady };
      const parked = parkDocumentOpeningReplacementV1({ runtimeKey, plugin, instanceId: targetSession.instanceId, background: target?.background === true }, openDocumentSessionsRef.current, entry);
      if (parked === null) return null;
      const predecessorActor = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
      if (predecessorActor !== undefined && predecessorActor.clientInstanceId !== clientInstanceId) {
        retireBrowserActorUi(runtimeKey, "browser-actor-action: opening parked");
      }
      const request: BackboneWorkerRequest = {
        kind: "open",
        actorIdentityGrant: {maximumItems:1024,maximumCopyBytes:65536,maximumCapacityBytes:65536,maximumReleaseBytes:65536,maximumDepth:64},
        clientInstanceId,
        documentId: ref.documentId,
        schema: ref.schema,
        bindings: workerBindings,
        watchExternal: true,
        actor: shellActorIdRef.current,
      };
      const expectsSocketActor = workerBindings.some((binding) => binding.kind === "hub");
      const socketActor = expectsSocketActor
        ? new Promise<string>((resolve, reject) => socketActorReadyRef.current.set(runtimeKey, { clientInstanceId, resolve, reject }))
        : null;
      void socketActor?.catch(() => {});
      const uri = `actor://${runtimeKey}`;
      const committed = await runDocumentOpeningAttemptV1({
        deadlineMs: documentOpeningDeadlineMs(),
        current: () => openDocumentSessionsRef.current.get(runtimeKey) === entry && entry.creationMount?.current() !== false,
        socket: async () => {
          worker.postMessage({ wire: encodeBackboneWorkerRequest(request) });
          postBrowserActorViewState(worker, entry, targetSession);
          await socketActor;
        },
        retire: () => {
          const waiter = socketActorReadyRef.current.get(runtimeKey);
          if (waiter !== undefined && waiter.clientInstanceId === clientInstanceId) socketActorReadyRef.current.delete(runtimeKey);
        },
        close: () => {
          entry.creationMount?.close(new Error("document opening aborted"));
          entry.rejectReady(new Error("document opening aborted"));
          entry.replacements.invalidate();
          void entry.port?.retire().catch(() => {});
          const restored = settleDocumentOpeningReplacementV1(openDocumentSessionsRef.current, parked, false);
          const successorActor = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
          if (successorActor?.clientInstanceId === clientInstanceId) {
            retireBrowserActorUi(runtimeKey, "browser-actor-action: opening aborted");
          }
        },
        detach: () => retireDocumentAttachment(plugin, targetSession.instanceId, clientInstanceId),
        attach: async () => {
          if (hubBinding && scope) {
            if (scope.documentId === SPACE_INDEX_DOCUMENT_ID) {
              spaceDirectoryHistoriesRef.current.set(runtimeKey, new SpaceDirectoryHistoryV1(scope.spaceId));
              worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "directory-space-open", baseUrl: hubBinding.baseUrl, spaceId: scope.spaceId }) });
            } else {
              directoryScopedOwnersRef.current.set(runtimeKey, scope);
              worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "directory-scope-open", baseUrl: hubBinding.baseUrl, scope, since: 0 }) });
            }
            if (scope.documentId === SPACE_INDEX_DOCUMENT_ID && !target?.background) {
              setSpaceArtifactCreationCatalog(null);
              setSpaceArtifactCreationCatalogUi({ kind: "space-artifact-creation-catalog-status", clientInstanceId, spaceId: scope.spaceId, phase: "loading" });
              worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "space-artifact-creation-catalog-open", clientInstanceId, spaceId: scope.spaceId }) });
            }
          }
          if (hubBinding || entry.replacements.pending) {
            if (entry.creationMount === null) await entry.ready;
            else await entry.creationMount.attach(entry.ready);
          }
          else await documentAttachmentLane(plugin, targetSession.instanceId).attach(clientInstanceId, () => openDocumentSessionsRef.current.get(runtimeKey) === entry, async () => {
            if (entry.port === null || entry.port.closing) await bindDocumentBackbone(runtimeKey, entry);
          });
        },
        commit: () => {
          entry.creationMount = null;
          if (target?.background) return;
          dispatch({ type: "SET_SYNC_BACKBONE_URI", value: uri });
          dispatch({ type: "SET_SYNC_CARD_KIND", value: null });
        },
      });
      if (committed) {
        for (const [key, owner] of settleDocumentOpeningReplacementV1(openDocumentSessionsRef.current, parked, true)) {
          if (key === runtimeKey) {
            void owner.port?.retire().catch(() => {});
            void retireDocumentAttachment(owner.plugin, owner.session.instanceId, owner.clientInstanceId).catch((error) => undefined);
            backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "close", documentId: owner.documentId, clientInstanceId: owner.clientInstanceId, ...(owner.scope === undefined ? {} : { spaceId: owner.scope.spaceId }) }) });
            continue;
          }
          closeDocumentRef.current(key, owner.clientInstanceId);
        }
      }
      return committed ? { committed: true, runtimeKey, clientInstanceId } : null;
    },
    [bindDocumentBackbone, documentAttachmentLane, ensureBackboneWorker, loadedPlugins, postBrowserActorViewState, resolveSyncTargetSession, hubEnv, retireDocumentAttachment, retireBrowserActorUi],
  );
  openDocumentRef.current = openDocument;

  useEffect(() => {
    const worker = backboneWorkerRef.current;
    if (worker === null) return;
    for (const entry of openDocumentSessionsRef.current.values()) {
      if (entry.scope === undefined) continue;
      const current = sessionRef.current;
      const targetSession = current?.pluginId === entry.session.pluginId && current.instanceId === entry.session.instanceId ? current : entry.session;
      postBrowserActorViewState(worker, entry, targetSession);
    }
  }, [activeToolId, activeUtilityByWindowId, extraWindowInstances, postBrowserActorViewState, session?.viewState]);
  return { openDocument };
}
