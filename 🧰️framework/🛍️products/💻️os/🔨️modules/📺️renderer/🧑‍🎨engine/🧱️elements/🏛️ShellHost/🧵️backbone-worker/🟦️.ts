/** 🧵️ Plugin supervisor and install progress refs, the document service inventory and `ensureBackboneWorker`, which boots and wires the store worker.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useBackboneWorker` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { shellDialogSessionIsCurrentV1 } from "../🗨️dialog-origin/🟦️.ts";
import { browserDocumentMountIsCurrentV1 } from "../🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import { isTerminalDocumentCheckInPhaseV1 } from "@semio-tech/framework/document/check-in";
import { directorySessionAuthorityIsCurrentV1 } from "@semio-tech/framework/directory/session-refresh";
import { useCallback, useEffect, useRef, useState } from "react";
import { type PluginModuleAcquisitionProgress } from "@semio-tech/framework";
// #region 🔌️Adapters
import {
  decodePackValue
} from "@semio-tech/framework/packs";
import {
  decodeDocumentArchiveBytes,
  type DocumentArchivePack
} from "@semio-tech/framework/wire-contracts";
import {
  DocumentArchiveFaultError
} from "@semio-tech/framework/app-channel";
import {
  type BackboneWorkerResponse,
  decodeBackboneWorkerResponse,
  encodeBackboneWorkerRequest,
  documentRuntimeKeyV1
} from "@semio-tech/framework/backbone";
import {
  parseDirectorySpaceAdministrationPageV1
} from "@semio-tech/framework/directory/io/text";
import { scopedPresencePeersV1, type ScopedPresencePeerV1 } from "../👥️presence-scope/🟦️.ts";
import { publishArtifactPresenceRosterV1, publishLocalPresenceActorV1 } from "../../👕️canvas-presence/🟦️.ts";
import { programHistoryKeyV1, programHistoryProjectionV1 } from "../🪟️spawned-program/🟦️.ts";
import { IDENTITY_CONFIG_SCHEMA, foldIdentityEvent } from "@semio-tech/framework/backbone/config-lane";
import { decodeLocalCatalogArchiveV1, decodeLocalCatalogPayloadV1 } from "../🗂️local-catalog/🟦️.ts";
import { LOCAL_CATALOG_CONFIG_SCHEMA, type LocalCatalog } from "@semio-tech/framework/config/mutations";
import { type ActiveSession, type LoadedProgramState, type PluginSupervisorState } from "../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { documentTransferNoticeTextV1, documentLoadCancelledV1 } from "../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import {
  applyBrowserActorUiPatchesV1,
  browserActorPanelKeysV1,
  commandRejectionNoticeV1,
  appFaultNoticeV1,
  historyLaneNoticeV1,
  historyRefusalCodeV1,
  historyRefusalNoticeV1
} from "../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { EMPTY_SHELL_HISTORY_PROJECTION_V1 } from "../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { resolveManifestLabel, shellLabel } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
// #endregion 🔌️Adapters
import { isPluginInstanceRetiredV1 } from "../../🔌️PluginRuntime/🟦️.tsx";
import { BrowserActorActionMailboxV1 } from "@semio-tech/framework/browser-bundle/action-handoff/requests";
import { StreamMuxChannelV1 } from "@semio-tech/framework/io/stream-mux";
import { liveInstanceWindowFaultV1 } from "../🩺️fault/🟦️.ts";
import {
  inferencePortStatusRuntimeKeyV1,
  reduceBootstrapUiState,
  reduceExecutionTargetUiState,
  type BootstrapUiState,
  type ExecutionTargetUiState
} from "../🪪️host-bootstrap/🟦️.tsx";
import { reduceArtifactCreationProgressUiV1 } from "../🌱️artifact-creation/🟦️.tsx";
import { announcedPageStreams, canonicalSurfaceId, decodeIdentityPayload, scopeRuntimeKey } from "../🔐️identity-binding/🟦️.ts";
import {
  captureSpaceArtifactCreationCatalogAuthorityV1,
  retainDirectoryCommandResult,
  SPACE_INDEX_DOCUMENT_ID,
  spaceArtifactCreationCatalogRefreshRequestV1,
  spaceArtifactCreationOwnerAcceptsStatus,
  type SpaceArtifactCreationOwnerV1,
  spaceArtifactCreationReadyOpening
} from "../🧾️space-artifact-creation/🟦️.ts";
import { copyDirectoryInviteCapabilityV1, reduceShellSpaceAdministrationState, shellSpaceAdministrationCapabilityAllowed } from "../🏠️space-administration/🟦️.ts";
import type { ShellCoreAfterIdentityAndDocumentPorts } from "../🔒️identity-document-ports/🟦️.ts";

export type ShellCoreAfterBackboneWorker = ShellCoreAfterIdentityAndDocumentPorts & ReturnType<typeof useBackboneWorker>;

//#region FrameworkOsShell
export function useBackboneWorker(core: ShellCoreAfterIdentityAndDocumentPorts) {
  const {
    backboneWorkerFactory, dispatch, historyStore, historyOrderRef, loadedPlugins, pluginSupervisorById,
    uiRefreshCacheRef, sessionRef, backboneWorkerRef, backboneWorkerStreamsDetachRef, setIdentity,
    verifiedSessionAuthorityRef, sessionRefreshRef, hubEnv, handleDirectoryEventPageRef, handlePreferenceLaneMessageRef,
    handleDirectoryBootstrapFailureRef, uiLocaleRef, uiTerminologyRef, showTransientNoticeRef, closeDocumentRef,
    openReadySpaceArtifactCreationRef, spaceArtifactCreationOwnersRef, setSpaceArtifactCreationUi,
    setSpaceArtifactCreationCatalog, setSpaceArtifactCreationCatalogUi, spaceArtifactCreationCatalogRef,
    spaceArtifactCreationCatalogUiRef, cancelSpaceArtifactCreationsForRuntime, setDirectoryPendingCommands,
    directoryCommandResultsRef, setSpaceAdministration, spaceAdministrationRef, spaceAdministrationEpochRef,
    spaceAdministrationMessageRef, inferencePortEpochRef, inferencePortOwnerRef, inferencePortAuthorityRef,
    retiredSessionDocumentOwnersRef, inferencePortOpeningRef, setInferenceHistoryByRuntimeKey,
    identitySnapshotResolverRef, identityClientInstanceIdRef, setLocalCatalog, localCatalogClientInstanceIdRef,
    localDocumentOwnersRef, openDocumentSessionsRef, failDocumentBackbone, receiveDocumentBackbone, captureDialogOrigin,
    documentArchivePorts, browserActorUiByRuntimeKeyRef, setBrowserActorUiVersion, publishUiRefreshSectionsV1,
    retireBrowserActorUi, directoryScopedOwnersRef, collectSpaceDirectoryEvents
  } = core;
  const socketActorReadyRef = useRef<Map<string, { readonly clientInstanceId: string; resolve(actorId: string): void; reject(error: Error): void }>>(new Map());
  const [bootstrapUiByDocument, setBootstrapUiByDocument] = useState<BootstrapUiState>({});
  const [executionTargetUiByDocument, setExecutionTargetUiByDocument] = useState<ExecutionTargetUiState>({});
  const [presencePeersByRuntimeKey, setPresencePeersByRuntimeKey] = useState<Readonly<Record<string, readonly ScopedPresencePeerV1[]>>>({});
  const rebootstrapDiscardedSessionsRef = useRef<Map<string, ActiveSession>>(new Map());
  /** 🐚️ Mirrors `loadedPlugins` for the unmount-cleanup effect below, which needs the latest value at
   * teardown time without depending on it (a dependency would tear down and re-run on every reload). */
  const loadedPluginsRef = useRef<readonly LoadedProgramState[]>([]);
  core.loadedPluginsRef = loadedPluginsRef;
  loadedPluginsRef.current = loadedPlugins;
  /** 🩺️ Read at catch time by the fault classifier so a crashed/quarantined supervisor outranks the
   * plugin's own fault code, without making every refresh effect depend on the supervisor roster. */
  const pluginSupervisorByIdRef = useRef<Readonly<Record<string, PluginSupervisorState>>>({});
  pluginSupervisorByIdRef.current = pluginSupervisorById;

  /** 🧯️ Where every FIRE-AND-FORGET refresh sends its rejection. A `void refreshUi(…)` without a
   * `catch` is an unhandled promise rejection, and an unhandled rejection out of a React subtree stops
   * that subtree applying updates — so one guest that declines to render takes the whole shell with it,
   * silently, with nothing on screen to say so.
   *
   * Measured 2026-09-20 (ticket 26/09/18, S2 §3.3): `🪐️space`'s Home answers
   * `s.home.session-identity-required` and `settlePluginTurn` throws "stopped without publishing
   * requested UI surfaces"; the rejection escaped, the tree froze, and the human's subsequent
   * successful hub sign-in could no longer re-establish anything — every recovery path in this file was
   * intact and simply never re-entered. A guest refusal is a WINDOW FAULT: it is shown, and the shell
   * keeps running so the next identity/session change can recover from it.
   *
   * A refresh that lands on a RETIRED instance ({@link isPluginInstanceRetiredV1}) is not a fault: the
   * window it would have refreshed is already gone, exactly the drop `dropForRetiredInstance` makes.
   * Reporting it raised the shell's boot error beacon for ~1 s on every `de` boot of the `s` host (a
   * locale-driven re-open retires the Home instance while its refresh is in flight; measured
   * `wp-g10/generated/boot-probe-de-1.txt`). */
  const reportRefreshFault = useCallback((pluginId: string, error: unknown): void => {
    const fault = liveInstanceWindowFaultV1(error, pluginSupervisorByIdRef.current[pluginId], isPluginInstanceRetiredV1);
    if (fault !== null) dispatch({ type: "SET_ERROR", value: fault.message, fault });
  }, []);
  /** 🔌️ The exact (possibly cache-busted `?v=`) module URL each currently-loaded plugin was acquired
   * at — `LoadedProgramState`/`PluginWasmHandle` carry no URL of their own. 🧬️ H1-react: its old
   * reader, `evictPluginModule` (the deleted refcounted module-URL lease pool, packet H2's "must not
   * exist" list), is gone — `reloadPlugin`/`uninstallPlugin` now free resources entirely through
   * `handle.dispose()` (`ShardClient.dispose` per activated actor), so this map is unread bookkeeping
   * kept for a future consumer rather than removed mid-packet. */
  const pluginModuleUrlByIdRef = useRef<Map<string, string>>(new Map());
  /** 🔁️ The `rebuiltAt` each currently-loaded plugin's artifact was acquired at, so the `PluginSource`
   * pump can tell a genuine rebuild from a replayed availability snapshot
   * ({@link pluginAvailabilityRouteV1}). Absent means "acquired unbusted", i.e. the first load, which
   * names no build. */
  const pluginArtifactRebuiltAtRef = useRef<Map<string, number>>(new Map());
  /** 🔁️ Records what an acquisition was built from, forgetting it again when the acquisition named no
   * build — so the map never claims a build the loaded artifact does not have. */
  const recordPluginArtifactRebuiltAt = useCallback((pluginId: string, rebuiltAt: number | undefined) => {
    if (rebuiltAt === undefined) pluginArtifactRebuiltAtRef.current.delete(pluginId);
    else pluginArtifactRebuiltAtRef.current.set(pluginId, rebuiltAt);
  }, []);
  /** 🔌️ Per-pluginId mutual exclusion across `installPlugin`/`reloadPlugin`/`uninstallPlugin` — the
   * boot effect and the `PluginSource` subscription effect can both request the same pluginId around
   * mount (e.g. the host plugin already appears in the connect-time `snapshot`), and without this guard
   * both calls would independently acquire a module lease, race their `UPSERT_LOADED_PLUGIN` dispatches,
   * and leak whichever lease lost the race (nothing left holding a reference to release it). */
  const pluginOpInFlightRef = useRef<Set<string>>(new Set());
  /** 🛑️ The abort of every install currently in flight, by pluginId — what the install band's cancel
   * control fires. A ref, not reducer state: nothing renders from it (the band renders from
   * `pluginStatusById`'s own `"installing"`), and an abort must reach the exact load that is running
   * rather than whatever a render pass last captured. */
  const pluginInstallAbortsRef = useRef<Map<string, AbortController>>(new Map());
  /** 📈️ Bytes every downloading install has verified so far — what the install band shows beside its
   * cancel. Chunk reports land in a ref and reach state at most every 120 ms, so a 60 MB module does
   * not re-render the shell once per 64 KiB chunk; a finished install clears its row at once. */
  const [pluginInstallProgressById, setPluginInstallProgressById] = useState<Readonly<Record<string, PluginModuleAcquisitionProgress>>>({});
  const pluginInstallProgressRef = useRef<Map<string, PluginModuleAcquisitionProgress>>(new Map());
  const pluginInstallProgressFlushRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const notePluginInstallProgress = useCallback((pluginId: string, progress: PluginModuleAcquisitionProgress | null) => {
    if (progress) pluginInstallProgressRef.current.set(pluginId, progress);
    else if (!pluginInstallProgressRef.current.delete(pluginId)) return;
    if (progress && pluginInstallProgressFlushRef.current !== null) return;
    if (pluginInstallProgressFlushRef.current !== null) clearTimeout(pluginInstallProgressFlushRef.current);
    pluginInstallProgressFlushRef.current = setTimeout(
      () => {
        pluginInstallProgressFlushRef.current = null;
        setPluginInstallProgressById(Object.fromEntries(pluginInstallProgressRef.current));
      },
      progress ? 120 : 0,
    );
  }, []);
  useEffect(
    () => () => {
      if (pluginInstallProgressFlushRef.current !== null) clearTimeout(pluginInstallProgressFlushRef.current);
    },
    [],
  );

  const documentServiceInventoryRef = useRef<Array<{owner:string;serviceId:string}>>([]);
  const ensureBackboneWorker = useCallback((): Worker => {
    if (backboneWorkerRef.current) return backboneWorkerRef.current;
    const worker = backboneWorkerFactory?.() ?? new Worker(new URL("../../../../../🏪️store/👷️worker/🟦️.ts", import.meta.url), { type: "module" });
    worker.postMessage({wire:encodeBackboneWorkerRequest({kind:"service-contributions",services:documentServiceInventoryRef.current})});
    const streams = announcedPageStreams();
    if (streams instanceof StreamMuxChannelV1) {
      const streamPipe = new MessageChannel();
      backboneWorkerStreamsDetachRef.current = streams.attachPort(streamPipe.port1);
      worker.postMessage({ kind: "semio-stream-mux-port", port: streamPipe.port2 }, [streamPipe.port2]);
    }
    worker.onmessage = (messageEvent: MessageEvent<BackboneWorkerResponse | { readonly wire: Uint8Array }>) => {
      const message = "wire" in messageEvent.data ? decodeBackboneWorkerResponse(messageEvent.data.wire) : messageEvent.data;
      if (message.kind === "browser-actor-action-result") {
        const runtimeKey = documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...message.scope });
        const retained = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        if (retained === undefined || entry === undefined || retained.clientInstanceId !== message.clientInstanceId || entry.clientInstanceId !== message.clientInstanceId || retained.sessionInstanceId !== entry.session.instanceId) return;
        const { clientInstanceId: _clientInstanceId, ...result } = message;
        retained.actions.settle(result);
        return;
      }
      if (message.kind === "browser-actor-ui-patch") {
        const runtimeKey = documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", spaceId: message.scope.spaceId, documentId: message.scope.documentId });
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        const expectedSurfaceId = entry?.scope !== undefined && entry.session.app.dialect ? canonicalSurfaceId(entry.session.app.dialect, entry.session.app.role) : null;
        // 🩻️ An offer this shell cannot apply is REFUSED by name, never dropped. Dropping it left the
        // worker's 15 s patch-result deadline to close the child with no reason anywhere, so a guest
        // whose surface the session disagreed about looked exactly like a guest that had stopped.
        // Only an offer addressed to no live client of this shell stays silent: there is no session
        // left to answer for.
        if (entry === undefined || entry.clientInstanceId !== message.clientInstanceId) return;
        const retained = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
        const refusal =
          entry.scope === undefined || entry.scope.spaceId !== message.scope.spaceId || entry.scope.documentId !== message.scope.documentId
            ? "scope-mismatch"
            : expectedSurfaceId !== message.verifiedSurfaceId
              ? `verified-surface-mismatch: session ${expectedSurfaceId ?? "<none>"} offer ${message.verifiedSurfaceId}`
              : message.instanceId !== 0
                  ? `instance-mismatch: ${message.instanceId}`
                  : retained !== undefined && (retained.clientInstanceId !== message.clientInstanceId || retained.activationGeneration !== message.activationGeneration || retained.verifiedSurfaceId !== message.verifiedSurfaceId || retained.sessionInstanceId !== entry.session.instanceId)
                    ? "retained-owner-mismatch"
                    : null;
        const answer = (verdicts: readonly { readonly surface: string; readonly outcome: "acknowledged" | "rejected"; readonly revision: number; readonly reason?: string }[]) =>
          worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "browser-actor-ui-patch-result", clientInstanceId: entry.clientInstanceId, scope: message.scope, verifiedSurfaceId: message.verifiedSurfaceId, activationGeneration: message.activationGeneration, instanceId: message.instanceId, receipt: message.receipt, verdicts }) });
        if (refusal !== null) {
          answer(message.patches.map((patch) => ({ surface: patch.surface, outcome: "rejected" as const, revision: 0, reason: refusal })));
          return;
        }
        const applied = applyBrowserActorUiPatchesV1(message.patches, new Set(entry.session.app.windowKinds.map((kind) => kind.id)), browserActorPanelKeysV1(entry.session.app), retained === undefined ? null : { windows: retained.windows, panels: retained.panels, sections: retained.sections });
        if (applied.stores !== null && retained === undefined) {
          const actions = new BrowserActorActionMailboxV1((request) => worker.postMessage({ wire: encodeBackboneWorkerRequest({ ...request, clientInstanceId: entry.clientInstanceId }) }));
          browserActorUiByRuntimeKeyRef.current.set(runtimeKey, { clientInstanceId: message.clientInstanceId, activationGeneration: message.activationGeneration, verifiedSurfaceId: message.verifiedSurfaceId, scope: { ...message.scope }, sessionInstanceId: entry.session.instanceId, windows: applied.stores.windows, panels: applied.stores.panels, sections: applied.stores.sections, identity: null, actions });
        }
        if (applied.stores !== null && (retained === undefined || applied.surfacesAdded)) setBrowserActorUiVersion((current) => current + 1);
        const live = browserActorUiByRuntimeKeyRef.current.get(runtimeKey),
          shown = core.shellStateRef.current.pluginRuntime.session;
        if (applied.sectionsChanged && live !== undefined && shown?.pluginId === entry.session.pluginId && shown.instanceId === entry.session.instanceId) publishUiRefreshSectionsV1(uiRefreshCacheRef.current, live);
        answer(applied.verdicts);
        return;
      }
      if (message.kind === "browser-actor-ui-mounted") {
        const runtimeKey = documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...message.scope });
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        const retained = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
        if (entry === undefined || retained === undefined || !browserDocumentMountIsCurrentV1(
          { clientInstanceId: entry.clientInstanceId, scope: entry.scope, instanceId: entry.session.instanceId },
          { clientInstanceId: retained.clientInstanceId, scope: retained.scope, instanceId: retained.sessionInstanceId, activationGeneration: retained.activationGeneration, verifiedSurfaceId: retained.verifiedSurfaceId, revision: retained.windows.get(message.windowKindId)?.getRevisionSnapshot() ?? 0 },
          { ...message, revision: message.uiRevision },
        )) return;
        if (entry.creationMount !== null && !entry.creationMount.accept(message.catalogGenerationId)) {
          failDocumentBackbone(runtimeKey, entry, new Error("artifact-creation.catalog-generation-mismatch"));
          return;
        }
        browserActorUiByRuntimeKeyRef.current.set(runtimeKey, { ...retained, identity: message });
        entry.resolveReady();
        const discarded = rebootstrapDiscardedSessionsRef.current.get(runtimeKey);
        if (discarded !== undefined) {
          rebootstrapDiscardedSessionsRef.current.delete(runtimeKey);
          dispatch({ type: "SET_SESSION", value: (current) => current ?? discarded });
        }
        setBootstrapUiByDocument((current) => reduceBootstrapUiState(current, { kind: "snapshot-replaced", documentId: message.scope.documentId, scope: message.scope }));
        // 🪪️ The execution-target live region describes the INSTALL, and the install is over: this
        // message is the verified component mounted and painted. It used to be cleared only when the
        // document detached, so a document that opened correctly kept announcing "Verifying document
        // component…" under a live surface for the rest of the session.
        setExecutionTargetUiByDocument((current) => reduceExecutionTargetUiState(current, { kind: "execution-target-cleared", documentId: message.scope.documentId, scope: message.scope }));
        setBrowserActorUiVersion((current) => current + 1);
        return;
      }
      if (message.kind === "socket-actor") {
        const runtimeKey = scopeRuntimeKey(message);
        if (runtimeKey === null) return;
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        const waiter = socketActorReadyRef.current.get(runtimeKey);
        if (entry?.clientInstanceId !== message.clientInstanceId || waiter?.clientInstanceId !== message.clientInstanceId) return;
        waiter.resolve(message.actorId);
        publishLocalPresenceActorV1(runtimeKey, message.actorId);
        if (socketActorReadyRef.current.get(runtimeKey)?.clientInstanceId === message.clientInstanceId) socketActorReadyRef.current.delete(runtimeKey);
        return;
      }
      if (message.kind === "inference-port-opened") {
        inferencePortOpeningRef.current?.mailbox.settle(message);
        return;
      }
      if (message.kind === "inference-port-closed") {
        const owner = inferencePortOwnerRef.current;
        if (owner === null || owner.owner !== message.owner || owner.serviceId !== message.serviceId || owner.operationEpoch !== message.operationEpoch || owner.scope.spaceId !== message.scope.spaceId || owner.scope.documentId !== message.scope.documentId) return;
        inferencePortOwnerRef.current = null;
        inferencePortAuthorityRef.current = null;
        dispatch({ type: "CLEAR_INFERENCE_PORT_FOR_DOCUMENT", runtimeKey: owner.runtimeKey });
        return;
      }
      if (message.kind === "inference-port-status") {
        if (!directorySessionAuthorityIsCurrentV1(inferencePortAuthorityRef.current, verifiedSessionAuthorityRef.current)) return;
        const runtimeKey = inferencePortStatusRuntimeKeyV1(inferencePortOwnerRef.current, inferencePortEpochRef.current, message);
        if (runtimeKey === null) return;
        dispatch({ type: "SET_INFERENCE_PORT_FOR_DOCUMENT", runtimeKey, status: message.status });
        return;
      }
      if (message.kind === "inference-history-status") {
        const runtimeKey = documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...message.scope });
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        const authority = verifiedSessionAuthorityRef.current;
        if (authority === null || !directorySessionAuthorityIsCurrentV1(authority, authority) || entry === undefined || retiredSessionDocumentOwnersRef.current.has(entry) || entry.clientInstanceId !== message.clientInstanceId || entry.scope?.spaceId !== message.scope.spaceId || entry.scope.documentId !== message.scope.documentId) return;
        setInferenceHistoryByRuntimeKey((current) => {
          if (!directorySessionAuthorityIsCurrentV1(authority, verifiedSessionAuthorityRef.current) || retiredSessionDocumentOwnersRef.current.has(entry)) return current;
          const previous = current[runtimeKey];
          if (previous !== undefined && (message.historyEpoch < previous.historyEpoch || previous.clientInstanceId !== message.clientInstanceId)) return current;
          const order = previous?.historyEpoch === message.historyEpoch ? previous.order : ++historyOrderRef.current;
          return { ...current, [runtimeKey]: { historyEpoch: message.historyEpoch, clientInstanceId: message.clientInstanceId, scope: message.scope, sessionInstanceId: entry.session.instanceId, status: message.status, owner: message.owner, serviceId: message.serviceId, order, authority } };
        });
        return;
      }
      if (message.kind === "execution-target-status") {
        const runtimeKey = scopeRuntimeKey(message);
        const entry = runtimeKey === null ? undefined : openDocumentSessionsRef.current.get(runtimeKey);
        if (runtimeKey === null || message.scope?.spaceId !== message.spaceId || entry?.clientInstanceId !== message.clientInstanceId) return;
        setExecutionTargetUiByDocument((current) => reduceExecutionTargetUiState(current, message));
        return;
      }
      if (message.kind === "socket-actor-failed") {
        const runtimeKey = scopeRuntimeKey(message);
        if (runtimeKey === null) return;
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        const waiter = socketActorReadyRef.current.get(runtimeKey);
        if (entry?.clientInstanceId !== message.clientInstanceId || waiter?.clientInstanceId !== message.clientInstanceId) return;
        waiter.reject(new Error(`socket actor unavailable (${message.code})`));
        if (socketActorReadyRef.current.get(runtimeKey)?.clientInstanceId === message.clientInstanceId) socketActorReadyRef.current.delete(runtimeKey);
        return;
      }
      if (message.kind === "space-artifact-creation-catalog-refresh-required") {
        const owner = spaceArtifactCreationOwnersRef.current.get(message.requestId) ?? null;
        const origin = captureDialogOrigin(core.shellStateRef.current.pluginRuntime.session);
        const authority = captureSpaceArtifactCreationCatalogAuthorityV1(
          spaceArtifactCreationCatalogRef.current,
          spaceArtifactCreationCatalogUiRef.current,
          origin,
        );
        const request = spaceArtifactCreationCatalogRefreshRequestV1(owner, authority, origin, message);
        if (request === null) return;
        const loading = {
          kind: "space-artifact-creation-catalog-status",
          clientInstanceId: request.clientInstanceId,
          spaceId: request.spaceId,
          phase: "loading",
        } as const;
        spaceArtifactCreationCatalogRef.current = null;
        spaceArtifactCreationCatalogUiRef.current = loading;
        setSpaceArtifactCreationCatalog(null);
        setSpaceArtifactCreationCatalogUi(loading);
        worker.postMessage({ wire: encodeBackboneWorkerRequest(request) });
        return;
      }
      if (message.kind === "document-check-in-status") {
        const owner = core.documentCheckInOwnersRef.current.get(message.status.requestId);
        const entry = owner === undefined ? undefined : openDocumentSessionsRef.current.get(owner.runtimeKey);
        if (owner === undefined || entry === undefined || entry.clientInstanceId !== message.clientInstanceId || entry.scope?.spaceId !== message.scope.spaceId || entry.scope.documentId !== message.scope.documentId) return;
        if (isTerminalDocumentCheckInPhaseV1(message.status.phase)) core.documentCheckInOwnersRef.current.delete(message.status.requestId);
        core.setDocumentCheckInUi((current) => ({ ...current, [owner.runtimeKey]: { requestId: message.status.requestId, scope: message.scope, status: message.status } }));
        return;
      }
      if (message.kind === "space-artifact-creation-status") {
        const owner = spaceArtifactCreationOwnersRef.current.get(message.requestId);
        if (owner === undefined || !spaceArtifactCreationOwnerAcceptsStatus(owner, message)) return;
        const entry = openDocumentSessionsRef.current.get(owner.runtimeKey);
        if (
          entry === undefined
          || entry.clientInstanceId !== owner.clientInstanceId
          || entry.session.instanceId !== owner.sessionInstanceId
          || entry.scope?.spaceId !== owner.spaceId
          || entry.scope.documentId !== SPACE_INDEX_DOCUMENT_ID
        ) {
          spaceArtifactCreationOwnersRef.current.delete(owner.requestId);
          setSpaceArtifactCreationUi((current) => reduceArtifactCreationProgressUiV1(current, { kind: "cleared", requestId: owner.requestId }));
          if (message.phase !== "ready") worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "space-artifact-create-cancel", requestId: owner.requestId, spaceId: owner.spaceId }) });
          return;
        }
        setSpaceArtifactCreationUi((current) => reduceArtifactCreationProgressUiV1(current, { kind: "status", message }));
        if (message.phase === "failed" || message.phase === "cancelled" || message.phase === "indeterminate") {
          spaceArtifactCreationOwnersRef.current.delete(owner.requestId);
          return;
        }
        const openingArgs = spaceArtifactCreationReadyOpening(message);
        if (openingArgs === null || owner.opening) return;
        const firstReady = owner.ready === null;
        const readyOwner: SpaceArtifactCreationOwnerV1 = firstReady ? { ...owner, ready: message } : owner;
        spaceArtifactCreationOwnersRef.current.set(owner.requestId, readyOwner);
        if (firstReady) openReadySpaceArtifactCreationRef.current(owner.requestId);
        return;
      }
      if (message.kind === "space-artifact-creation-catalog") {
        const runtimeKey = documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", spaceId: message.spaceId, documentId: SPACE_INDEX_DOCUMENT_ID });
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        if (entry?.clientInstanceId !== message.clientInstanceId || entry.scope?.spaceId !== message.spaceId || entry.scope.documentId !== SPACE_INDEX_DOCUMENT_ID) return;
        setSpaceArtifactCreationCatalog(message);
        return;
      }
      if (message.kind === "space-artifact-creation-catalog-status") {
        const runtimeKey = documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", spaceId: message.spaceId, documentId: SPACE_INDEX_DOCUMENT_ID });
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        if (entry?.clientInstanceId !== message.clientInstanceId || entry.scope?.spaceId !== message.spaceId || entry.scope.documentId !== SPACE_INDEX_DOCUMENT_ID) return;
        if (message.phase !== "ready") setSpaceArtifactCreationCatalog(null);
        setSpaceArtifactCreationCatalogUi(message);
        return;
      }
      // 📇️ §C6 directory lane — the worker's `directory-*` responses never carry a `documentId` this
      // shell already has an `openDocumentSessionsRef` entry for (they're not artifact-sync events at
      // all), so they're routed here, ahead of the artifact-event early return below.
      if (message.kind === "preference-lane-page" || message.kind === "preference-lane-recorded" || message.kind === "preference-lane-failed") {
        handlePreferenceLaneMessageRef.current(message);
        return;
      }
      if (message.kind === "directory-event-page") {
        handleDirectoryEventPageRef.current(message);
        return;
      }
      if (message.kind === "directory-bootstrap-failed") {
        handleDirectoryBootstrapFailureRef.current(message);
        return;
      }
      if (message.kind === "directory-message") {
        if (message.message.kind === "event") collectSpaceDirectoryEvents([message.message.event]);
        return;
      }
      if (message.kind === "directory-space-events") {
        collectSpaceDirectoryEvents(message.events);
        return;
      }
      if (message.kind === "directory-scope-revoked") {
        const key = documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", spaceId: message.scope.spaceId, documentId: message.scope.documentId });
        if (!directoryScopedOwnersRef.current.delete(key)) return;
        cancelSpaceArtifactCreationsForRuntime(key, worker);
        setSpaceArtifactCreationCatalog((catalog) => catalog?.spaceId === message.scope.spaceId ? null : catalog);
        setSpaceArtifactCreationCatalogUi((status) => status?.spaceId === message.scope.spaceId ? null : status);
        const entry = openDocumentSessionsRef.current.get(key);
        if (entry !== undefined) closeDocumentRef.current(key, entry.clientInstanceId);
        retireBrowserActorUi(key, "browser-actor-action: scope revoked");
        rebootstrapDiscardedSessionsRef.current.delete(key);
        setInferenceHistoryByRuntimeKey((current) => {
          if (!(key in current)) return current;
          const next = { ...current };
          delete next[key];
          return next;
        });
        setPresencePeersByRuntimeKey((current) => {
          if (!(key in current)) return current;
          const next = { ...current };
          delete next[key];
          return next;
        });
        setBootstrapUiByDocument((current) => reduceBootstrapUiState(current, { kind: "detached", documentId: message.scope.documentId, scope: message.scope }));
        setExecutionTargetUiByDocument((current) => reduceExecutionTargetUiState(current, { kind: "execution-target-cleared", documentId: message.scope.documentId, scope: message.scope }));
        return;
      }
      if (message.kind === "directory-status") {
        setDirectoryPendingCommands(message.pendingCommands);
        return;
      }
      if (message.kind === "directory-administration-state") {
        const epoch = spaceAdministrationEpochRef.current;
        const administrationMessage = message;
        if (administrationMessage.operationEpoch !== epoch || worker !== backboneWorkerRef.current || administrationMessage.spaceId !== spaceAdministrationRef.current?.spaceId) return;
        const revision = ++spaceAdministrationMessageRef.current;
        const verificationIsCurrent = (): boolean => epoch === spaceAdministrationEpochRef.current && revision === spaceAdministrationMessageRef.current && worker === backboneWorkerRef.current;
        if (administrationMessage.canonicalJson === undefined) {
          setSpaceAdministration((current) => verificationIsCurrent() ? reduceShellSpaceAdministrationState(current, administrationMessage, epoch, current?.page ?? null) : current);
          return;
        }
        void parseDirectorySpaceAdministrationPageV1(administrationMessage.canonicalJson)
          .then((page) => setSpaceAdministration((current) => verificationIsCurrent() ? reduceShellSpaceAdministrationState(current, administrationMessage, epoch, page) : current))
          .catch(() => setSpaceAdministration((current) => verificationIsCurrent() ? reduceShellSpaceAdministrationState(current, { ...administrationMessage, phase: "failed", code: "invalid" }, epoch, null) : current));
        return;
      }
      if (message.kind === "directory-administration-capability") {
        if (!shellSpaceAdministrationCapabilityAllowed(spaceAdministrationRef.current, message.operationEpoch) || message.operationEpoch !== spaceAdministrationEpochRef.current) return;
        const { operationEpoch, transferEpoch, inviteToken } = message;
        void copyDirectoryInviteCapabilityV1(inviteToken).then((copied) => {
          const currentWorker = backboneWorkerRef.current;
          if (currentWorker === null || operationEpoch !== spaceAdministrationEpochRef.current) return;
          currentWorker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "directory-administration-capability-result", operationEpoch, transferEpoch, copied }) });
        });
        return;
      }
      if (message.kind === "directory-administration-capability-rejected") {
        return;
      }
      if (message.kind === "directory-command-failed") {
        retainDirectoryCommandResult(directoryCommandResultsRef.current, message.requestId, { kind: "failed", code: message.code });
        console.error("[os-shell] directory command failed", message.requestId, message.code);
        return;
      }
      if (message.kind === "directory-command-receipt") {
        // 📇️ The retained slot is the receipt's owner: it is filled FIRST, and only an `accepted`
        // receipt's durable events are folded. Defense-in-depth (ticket 26/08/16/HUB-SPACES-LIVE-
        // PRESENCE-AND-COLLABORATIVE-STUDIOS w4-h): the ORIGINATING client folds its own accepted
        // command's events instead of depending entirely on the live `/directory/socket/v1`
        // broadcast finding its way back to the same socket — correct only if that subscription is
        // guaranteed already-open at command-issue time, which a fresh page load racing identity
        // bootstrap does not guarantee. The live broadcast path (`directory-message` above) still
        // relays the same events for every OTHER client; a duplicate here is harmless — the Space
        // index fold re-derives the same config snapshot from the same batch. Home is never folded:
        // its sealed page lane wakes on the broadcast instead (`dispatchDirectoryEventBatch`).
        retainDirectoryCommandResult(directoryCommandResultsRef.current, message.requestId, { kind: "receipt", receipt: message.receipt });
        if (message.receipt.outcome === "accepted" && message.receipt.events.length > 0) collectSpaceDirectoryEvents(message.receipt.events);
        return;
      }
      if (message.kind === "artifact-bootstrap-progress" || message.kind === "artifact-bootstrap-failed" || message.kind === "artifact-rebootstrap-required") {
        const runtimeKey = scopeRuntimeKey(message);
        if (runtimeKey === null) return;
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        if (!entry || entry.clientInstanceId !== message.clientInstanceId) return;
        setBootstrapUiByDocument((current) => reduceBootstrapUiState(current, message));
        if (entry.creationMount !== null && message.kind !== "artifact-bootstrap-progress") {
          failDocumentBackbone(runtimeKey, entry, new Error(message.kind === "artifact-bootstrap-failed" ? message.message : "artifact-creation.rebootstrap-required"));
          return;
        }
        if (message.kind === "artifact-bootstrap-failed") {
          retireBrowserActorUi(runtimeKey, "browser-actor-action: bootstrap failed");
          entry.replacements.invalidate();
          entry.rejectReady(new Error(message.message));
          const retirement = entry.port?.retire();
          void retirement?.catch(error => failDocumentBackbone(runtimeKey, entry, error));
          entry.port = null;
          entry.pending = [];
          entry.pendingBytes = 0;
        }
        if (message.kind === "artifact-rebootstrap-required") {
          entry.replacements.invalidate();
          const retirement = entry.port?.retire();
          void retirement?.catch(error => failDocumentBackbone(runtimeKey, entry, error));
          entry.port = null;
          entry.pending = [];
          entry.pendingBytes = 0;
          cancelSpaceArtifactCreationsForRuntime(runtimeKey, worker);
          retireBrowserActorUi(runtimeKey, "browser-actor-action: rebootstrap required");
          const active = sessionRef.current;
          if (shellDialogSessionIsCurrentV1(active, entry.session)) {
            rebootstrapDiscardedSessionsRef.current.set(runtimeKey, entry.session);
          }
        }
        return;
      }
      // 🪪️ §C3 identity facet — opened directly (never through `openDocument`, so it never gets an
      // `openDocumentSessionsRef` entry: it has no plugin/session, only the shell itself). Routed here,
      // ahead of the `!entry` early return below, which would otherwise silently drop every one of its
      // events. `documentArchiveReplaced` (whole-record archive, the folder read-back) resolves the bootstrap
      // effect's one-shot wait; `remoteMutations` (another tab's sign-in/out, echoed over this
      // document's `BroadcastChannel`) folds via {@link foldIdentityEvent} like `OpeningPreferences`.
      if (message.kind === "event" && message.documentId === LOCAL_CATALOG_CONFIG_SCHEMA) {
        if (localCatalogClientInstanceIdRef.current !== message.clientInstanceId) return;
        const catalogEvent = message.event;
        if (catalogEvent.kind === "documentArchiveReplaced") {
          const decoded = decodeLocalCatalogArchiveV1(new Uint8Array(catalogEvent.archive));
          if (decoded !== undefined) setLocalCatalog(decoded);
        } else if (catalogEvent.kind === "remoteMutations") {
          const latest = catalogEvent.envelopes.map((envelope) => decodeLocalCatalogPayloadV1(envelope.diff.payload)).filter((catalog): catalog is LocalCatalog => catalog !== undefined).at(-1);
          if (latest !== undefined) setLocalCatalog(latest);
        }
        return;
      }
      if (message.kind === "event" && localDocumentOwnersRef.current.has(message.clientInstanceId)) {
        const owner = localDocumentOwnersRef.current.get(message.clientInstanceId);
        if (message.event.kind === "status") owner?.status?.(message.event.persisted);
        else if (message.event.kind === "documentArchiveReplaced") owner?.archive?.(new Uint8Array(message.event.archive));
        return;
      }
      if (message.kind === "event" && message.documentId === IDENTITY_CONFIG_SCHEMA) {
        if (identityClientInstanceIdRef.current !== message.clientInstanceId) return;
        const identityEvent = message.event;
        if (identityEvent.kind === "documentArchiveReplaced") {
          const archive = decodeDocumentArchiveBytes(new Uint8Array(identityEvent.archive));
          const decoded = decodeIdentityPayload(decodePackValue(new Uint8Array(archive.parent_pack)));
          if (decoded !== undefined) {
            if (verifiedSessionAuthorityRef.current === null) setIdentity(decoded);
            identitySnapshotResolverRef.current?.(decoded);
            identitySnapshotResolverRef.current = null;
          }
        } else if (identityEvent.kind === "remoteMutations") {
          if (verifiedSessionAuthorityRef.current === null) setIdentity((current) => foldIdentityEvent(current, identityEvent, decodeIdentityPayload));
          void sessionRefreshRef.current?.refresh();
        }
        return;
      }
      if (message.kind !== "event") return;
      const runtimeKey = message.scope === undefined ? message.documentId : scopeRuntimeKey(message);
      if (runtimeKey === null) return;
      const entry = openDocumentSessionsRef.current.get(runtimeKey);
      if (!entry || entry.clientInstanceId !== message.clientInstanceId) return;
      const { event } = message;
      if (event.kind === "status") {
        // 🧭️ `typeof` in a TYPE position reads the DECLARED type of `event`, not the narrowed one, so
        // `typeof event.remote` asked the whole `ArtifactEvent` union for a field only its `status`
        // member has. A local binding carries the narrowing into the type.
        const status = event;
        (window as unknown as { semioWgpuHubProjection?: { publishDocumentStatus: (documentKey: string, remote: typeof status.remote | null) => boolean } }).semioWgpuHubProjection?.publishDocumentStatus(runtimeKey, status.remote);
        dispatch({ type: "SET_SYNC_STATUS_FOR_DOCUMENT", documentId: runtimeKey, status: { persisted: event.persisted, pendingMutations: event.pendingMutations, remote: event.remote } });
      } else if (event.kind === "presence") {
        const peers = entry.scope === undefined ? [] : scopedPresencePeersV1(message, entry.scope, entry.session.app.dialect ? canonicalSurfaceId(entry.session.app.dialect, entry.session.app.role) : null);
        setPresencePeersByRuntimeKey((current) => {
          if (current[runtimeKey] === peers) return current;
          return { ...current, [runtimeKey]: peers };
        });
        // 👕️ Full wire peers (views + interaction) for in-canvas overlays — roster chrome stays on the slim map above.
        publishArtifactPresenceRosterV1(runtimeKey, event.peers);
      } else if (event.kind === "documentBackbone") {
        receiveDocumentBackbone(runtimeKey, entry, event.message);
      } else if (event.kind === "documentArchiveReplaced") {
        const archiveBytes = new Uint8Array(event.archive);
        void (async () => {
          try {
            const archive = decodeDocumentArchiveBytes(archiveBytes);
            const held = () => openDocumentSessionsRef.current.get(runtimeKey) === entry;
            const merge = async (value: DocumentArchivePack) => {
              if (!entry.plugin.mergeAppDocumentArchive) throw new Error("document-backbone.archive-merger-unavailable");
              return entry.plugin.mergeAppDocumentArchive(entry.session.instanceId, value);
            };
            const file = resolveManifestLabel(entry.session.app.label, uiTerminologyRef.current, uiLocaleRef.current) || entry.session.app.id;
            const detach = async (failure: Readonly<{ error: unknown }> | null): Promise<void> => {
              if (!held()) return;
              if (failure !== null) {
                console.error("[os-shell] a folder read-back could not be restored", message.documentId, failure.error instanceof Error ? failure.error.message : String(failure.error));
                const told = failure.error instanceof DocumentArchiveFaultError && failure.error.fault !== null ? appFaultNoticeV1(failure.error.fault, entry.session.app, uiTerminologyRef.current) : null;
                if (told === null) core.documentTransferRef.current.notifyDocumentTransfer("load-failed", file, "error");
                else showTransientNoticeRef.current(told.text, "error", told.code);
              }
              core.documentTransferRef.current.notifyDocumentTransfer("folder-detached", file, "info");
              closeDocumentRef.current(runtimeKey, entry.clientInstanceId);
              if (core.shellStateRef.current.sync.syncBackboneUri === `actor://${runtimeKey}`) {
                dispatch({ type: "SET_SYNC_BACKBONE_URI", value: null });
                dispatch({ type: "SET_SYNC_CARD_KIND", value: null });
              }
            };
            const outcome = await entry.readBack.archive(archive, held, { ...documentArchivePorts(entry.plugin, entry.session, held), merge, cancelled: documentLoadCancelledV1, detach });
            if (outcome === "cancelled" && held()) core.documentTransferRef.current.notifyDocumentTransfer("load-cancelled", file, "info");
            if (outcome !== "held") return;
            if (openDocumentSessionsRef.current.get(runtimeKey) !== entry) return;
            const discarded = rebootstrapDiscardedSessionsRef.current.get(runtimeKey);
            if (discarded) {
              rebootstrapDiscardedSessionsRef.current.delete(runtimeKey);
              dispatch({ type: "SET_SESSION", value: (current) => current ?? discarded });
            }
            setBootstrapUiByDocument((current) => reduceBootstrapUiState(current, { kind: "snapshot-replaced", documentId: message.documentId, ...(message.scope === undefined ? {} : { scope: message.scope }) }));
          } catch (replacementError) {
            if (openDocumentSessionsRef.current.get(runtimeKey)?.clientInstanceId !== message.clientInstanceId) return;
            console.error("[os-shell] a folder read-back could not be restored", message.documentId, replacementError instanceof Error ? replacementError.message : String(replacementError));
            if (entry.creationMount !== null) {
              failDocumentBackbone(runtimeKey, entry, replacementError);
              return;
            }
            entry.rejectReady(replacementError instanceof Error ? replacementError : new Error(String(replacementError)));
            setBootstrapUiByDocument((current) => reduceBootstrapUiState(current, {
              kind: "artifact-bootstrap-failed",
              documentId: message.documentId,
              clientInstanceId: message.clientInstanceId,
              ...(message.scope === undefined ? {} : { scope: message.scope }),
              code: "invalid-bootstrap",
              message: (replacementError instanceof DocumentArchiveFaultError && replacementError.fault !== null ? appFaultNoticeV1(replacementError.fault, entry.session.app, uiTerminologyRef.current)?.text : undefined)
                ?? documentTransferNoticeTextV1("load-failed", resolveManifestLabel(entry.session.app.label, uiTerminologyRef.current, uiLocaleRef.current) || entry.session.app.id, uiLocaleRef.current),
              retryable: false,
            }));
          }
        })();
      } else if (event.kind === "documentArchiveAbsent") {
        void entry.readBack.absent().catch(error => failDocumentBackbone(runtimeKey, entry, error));
      } else if (event.kind === "commandOutcome") {
        // 🌐️ A refused or transformed batch — refused by the hub, or by the document's actor before it left this device: the
        // worker already discarded the refused edit (rollback, or a rebootstrap of the document's actor), so the human is TOLD,
        // in their language, and why.
        if (event.outcome.kind === "rejected") {
          const notice = commandRejectionNoticeV1(event.outcome);
          console.warn("[os-shell] a command batch was refused", message.documentId, event.outcome.code, event.outcome.reason, event.outcome.detail ?? {});
          showTransientNoticeRef.current(notice.text, notice.kind, notice.code);
        }
        else if (event.outcome.kind === "transformed") showTransientNoticeRef.current(shellLabel("ui.conflict.hubTransformed"), "info", "sync.command.transformed");
      } else if (event.kind === "conflict") {
        // ⚖️ Investigated for contract freeze `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-
        // CLASS-CONFLICTS` §C6/§C9 (lane L1): this is `🏪️store/🔄️sync/🦀️.rs`'s hub-relay
        // `ArtifactEvent::Conflict(MutationMessage)` — a TRANSPORT-level diagnostic (external folder
        // divergence / a hub `ServerFrame::Error`), never a first-class `Conflict` roster (no
        // `id`/`status`/`actors` exists for either source event, and `protocol_wire::ServerFrame` has
        // no `MergeReport`/`Conflicts` variant to carry one). The REAL roster/merge-outcome delivery
        // this contract added is `AppCommand::ApplyEnvelopes`'s own reply — see the `remoteMutations`
        // branch above (`applyRemoteMergeRef`) — so this branch stays a passive log, except for a refused history
        // transition (`history.transition-refused` & co.), which the human is told about in their language.
        console.warn("[os-shell] sync conflict", message.documentId, event.message);
        const refusal = historyRefusalCodeV1(event.code);
        const notice = refusal !== null ? historyRefusalNoticeV1(refusal) : historyLaneNoticeV1({ code: typeof event.code === "string" ? event.code : undefined }, programHistoryProjectionV1(historyStore.get(), programHistoryKeyV1(sessionRef.current), EMPTY_SHELL_HISTORY_PROJECTION_V1).editCount);
        if (notice !== null) showTransientNoticeRef.current(notice.text, notice.kind, notice.code);
      }
    };
    const failBrowserActorActions = () => {
      for (const retained of browserActorUiByRuntimeKeyRef.current.values()) retained.actions.close("browser-actor-action: worker completion unconfirmed");
    };
    worker.addEventListener("error", failBrowserActorActions);
    worker.addEventListener("messageerror", failBrowserActorActions);
    backboneWorkerRef.current = worker;
    return worker;
  }, [cancelSpaceArtifactCreationsForRuntime, captureDialogOrigin, collectSpaceDirectoryEvents, failDocumentBackbone, hubEnv, documentArchivePorts, receiveDocumentBackbone, retireBrowserActorUi]);

  return {
    socketActorReadyRef, bootstrapUiByDocument, setBootstrapUiByDocument, executionTargetUiByDocument,
    setExecutionTargetUiByDocument, presencePeersByRuntimeKey, setPresencePeersByRuntimeKey,
    rebootstrapDiscardedSessionsRef, pluginSupervisorByIdRef, reportRefreshFault, pluginModuleUrlByIdRef,
    pluginArtifactRebuiltAtRef, recordPluginArtifactRebuiltAt, pluginOpInFlightRef, pluginInstallAbortsRef,
    pluginInstallProgressById, notePluginInstallProgress, documentServiceInventoryRef, ensureBackboneWorker
  };
}
//#endregion FrameworkOsShell
