/** 📎️ The hub connection port, the inference port and the document backbone ports of the identity and document ports hook. */
import { shellDialogOriginIsCurrentV1, shellDialogOriginV1, shellEffectOwnerIsCurrentV1, type ShellDialogOriginV1, type ShellDialogV1 } from "../../🗨️dialog-origin/🟦️.ts";
import { replaceAttachedDocumentV1 } from "../../🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import { AGENT_CREDENTIAL_INSTALL_ENDPOINT_V1, AgentCredentialInstallUnavailableV1, parseAgentCredentialInstallReceiptV1 } from "@semio-tech/framework/directory/delegations";
import { useCallback, useMemo, useRef, useState } from "../../../⚛️react-port/🟦️.ts";
import { type AppRole, type AppCatalogue, type WindowEngagement, type WindowMeasure } from "@semio-tech/framework";
import { type DocumentScope } from "@semio-tech/framework/document/scope";
import { BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES, DOCUMENT_ARCHIVE_MAXIMUM_BYTES, encodeDocumentArchiveBytes, type DocumentArchivePack, type DocumentArchiveLoadStatus } from "@semio-tech/framework/wire-contracts";
import { decodeBackboneMessage, encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { type DirectorySessionAuthorityV1 } from "@semio-tech/framework/directory/schema/session-authority-v1";
import { type DirectoryEvent } from "@semio-tech/framework/directory/schema";
import { parseDirectorySpaceListEntryV1 } from "@semio-tech/framework/directory-client";
import { parseDirectoryCommandReceiptV1, sealDirectoryCommandRequestV1, directoryCommandRequestJson } from "@semio-tech/framework/directory/io/text";
import { hubDeviceInstanceIdV1 } from "@semio-tech/framework/directory/sign-in";
import { DOCUMENT_BACKBONE_RETENTION_LIMITS } from "@semio-tech/framework/replication";
import { SpaceDirectoryHistoryV1 } from "../../📇️space-directory/🟦️.ts";
import { programHistoryKeyV1, withProgramEntriesV1 } from "../../🪟️spawned-program/🟦️.ts";
import { EMPTY_LOCAL_CATALOG_V1 } from "../../🗂️local-catalog/🟦️.ts";
import { type Identity, type LocalCatalog } from "@semio-tech/framework/config/mutations";
import { type HubConnectionPortV1 } from "../../../🔗️HubConnection/🔌️connection-contract/🟦️.ts";
import { createHubConnectionFetchPortV1 } from "../../../🔗️HubConnection/🛰️fetch-port/🟦️.ts";
import { sharedDocumentOpeningRoleV1 } from "../../🧭️opening/🟦️.ts";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { EMPTY_APP_CATALOGUE } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { downloadMediaExport } from "../../../🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import { mergeRecordPreservingIdentity, preserveJsonIdentity, type UiRefreshCache } from "../../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { browserActorSectionValuesV1 } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { programHistoryProjectionsWithLoadV1 } from "../../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { parsePanelState } from "../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { type PluginWasmHandle } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { documentBackboneEffectV1, type ActorDocumentMessagePortV1 } from "@semio-tech/framework/backbone/binding";
import { InferencePortOpeningMailboxV1 } from "@semio-tech/framework/inference/opening";
import { type InferencePortOwnerV1 } from "../../🪪️host-bootstrap/🟦️.tsx";
import { agentBridgeOfferScopeFromDelegationsV1 } from "../../../🔗️AgentBridge/🟦️.tsx";
import { hubBootstrapOriginV1, readViteSEnv } from "../../🔐️identity-binding/🟦️.ts";
import { captureSpaceArtifactCreationCatalogAuthorityV1, mintDirectoryCommandRequestId } from "../../🧾️space-artifact-creation/🟦️.ts";
import { type ShellSpaceAdministrationStateV1 } from "../../🏠️space-administration/🟦️.ts";
import type { ShellCoreAfterShellCoreState } from "../../🧩️shell-core-state/🟦️.ts";
import type { useIdentityAgentBridgeAndDirectoryState } from "../🪪️identity-and-directory-state/🟦️.ts";
import type { MountedInferenceHistoryV1, OpenDocumentSession, RetainedBrowserActorUiV1 } from "../🧰️identity-port-types/🟦️.ts";

/**
 * 🔌 Builds the hub connection port, the inference and presence refs and the document backbone ports.
 *
 * 🎯️ The scope this shell asks the agent-bridge supervisor with (ticket 26/09/23, G12 session 14c — scoped offers): its
 * hub, the space it has open and the agents THIS human delegated there, read from the hub, which lists only the asking
 * human's own. A hub-bound gateway's offer is then served to its own human's shell and to no other.
 */
export function useHubPortInferenceAndDocumentBackbone({ hubSessionCapabilityRef, rememberHubSessionCapability, agentBridgeOfferScopeRef, hubEnv, openSpaceId, spaceAdministration, closeDocumentRef, backboneWorkerRef, applyRemoteMergeRef, documentPortsRef, documentAttachmentLane, core }: Pick<ReturnType<typeof useIdentityAgentBridgeAndDirectoryState>, "hubSessionCapabilityRef" | "rememberHubSessionCapability" | "agentBridgeOfferScopeRef" | "hubEnv" | "openSpaceId" | "spaceAdministration" | "closeDocumentRef" | "backboneWorkerRef" | "applyRemoteMergeRef" | "documentPortsRef" | "documentAttachmentLane"> & { readonly core: ShellCoreAfterShellCoreState }) {
  const hubConnectionPort = useMemo<HubConnectionPortV1>(
    () =>
      createHubConnectionFetchPortV1({
        request: (url, init, signal) => fetch(url, { ...init, credentials: "include", signal }),
        storage: typeof globalThis.localStorage === "undefined" ? null : globalThis.localStorage,
        bootstrapOrigin: hubBootstrapOriginV1(),
        capability: { read: () => hubSessionCapabilityRef.current, write: rememberHubSessionCapability },
        deviceInstanceId: hubDeviceInstanceIdV1(typeof globalThis.localStorage === "undefined" ? null : globalThis.localStorage),
        clientClass: "browser",
        parseSpaces: (body) => (JSON.parse(body) as unknown[]).map(parseDirectorySpaceListEntryV1),
        sealCommand: (command) => {
          const request = sealDirectoryCommandRequestV1(mintDirectoryCommandRequestId(), command);
          return { body: directoryCommandRequestJson(request), parseReceipt: (text) => parseDirectoryCommandReceiptV1(text, request) };
        },
        writeClipboard: typeof navigator === "undefined" || navigator.clipboard === undefined ? undefined : (text) => navigator.clipboard.writeText(text),
        saveFile: typeof document === "undefined"
          ? undefined
          : async (file) => {
              downloadMediaExport(file.fileName, file.mediaType, file.contents);
            },
        installAgentCredential: async (request, signal) => {
          const response = await fetch(AGENT_CREDENTIAL_INSTALL_ENDPOINT_V1, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify(request), signal, cache: "no-store" });
          if (response.status === 404 || response.status === 405) throw new AgentCredentialInstallUnavailableV1();
          if (!response.ok) throw new Error(`directory.delegations.install-refused.${response.status}`);
          return parseAgentCredentialInstallReceiptV1(await response.text());
        },
        uninstallAgentCredential: async (delegationId) => {
          await fetch(`${AGENT_CREDENTIAL_INSTALL_ENDPOINT_V1}/${encodeURIComponent(delegationId)}`, { method: "DELETE", cache: "no-store" });
        },
      }),
    [rememberHubSessionCapability],
  );
  agentBridgeOfferScopeRef.current = async (signal) => {
    if (hubEnv === null || openSpaceId === null || hubSessionCapabilityRef.current === null) return null;
    const hubOrigin = hubBootstrapOriginV1();
    return agentBridgeOfferScopeFromDelegationsV1(hubOrigin, openSpaceId, await hubConnectionPort.listAgentDelegations(hubOrigin, openSpaceId, signal), Date.now());
  };
  const spaceAdministrationRef = useRef<ShellSpaceAdministrationStateV1 | null>(null);
  spaceAdministrationRef.current = spaceAdministration;
  const spaceAdministrationEpochRef = useRef(0);
  const spaceAdministrationMessageRef = useRef(0);
  /** 💡️ Monotonic owner of the one worker-side inference port; a stale epoch's status is ignored. */
  const inferencePortEpochRef = useRef(0);
  /** 🗂️ Exact scope and runtime owner for the current inference epoch. */
  const inferencePortOwnerRef = useRef<InferencePortOwnerV1 | null>(null);
  const inferencePortAuthorityRef = useRef<DirectorySessionAuthorityV1 | null>(null);
  const retiredSessionDocumentOwnersRef = useRef(new WeakSet<object>());
  const inferencePortOpeningRef = useRef<{ readonly owner: InferencePortOwnerV1; readonly clientInstanceId: string; readonly sessionInstanceId: number; readonly mailbox: InferencePortOpeningMailboxV1 } | null>(null);
  
  const [inferenceHistoryByRuntimeKey, setInferenceHistoryByRuntimeKey] = useState<Readonly<Record<string, MountedInferenceHistoryV1>>>({});
  const inferenceHistoryByRuntimeKeyRef = useRef<Readonly<Record<string, MountedInferenceHistoryV1>>>({});
  inferenceHistoryByRuntimeKeyRef.current = inferenceHistoryByRuntimeKey;
  /** 🪪️ Settled once by the identity bootstrap effect's very first `documentArchiveReplaced` for
   * `IDENTITY_CONFIG_SCHEMA` (a previously-persisted session) — see that effect for the bounded
   * timeout that resolves it to `null` when no such file exists (never blocks the UI thread). */
  const identitySnapshotResolverRef = useRef<((value: Identity | null) => void) | null>(null);
  const identityClientInstanceIdRef = useRef<string | null>(null);
  const identityBootstrapAbortRef = useRef<AbortController | null>(null);
  /** 🗂️ The host-owned local document catalog (`os.config.local-catalog`, persisted local-only in `${S_DATA_DIR}/os`) —
   * `🗂️local-catalog/🟦️.ts`. Opened at mount, local-first, with or without a hub. `localDocumentOwnersRef` routes one kept
   * document's worker events (a write being verified, a read-back) by the client id it was opened under;
   * `rehydratedLocalDocumentsRef` remembers, per landing-app instance, which kept documents it was already handed. */
  const localDataDir = useMemo(() => readViteSEnv("VITE_S_DATA_DIR"), []);
  const [localCatalog, setLocalCatalog] = useState<LocalCatalog>(EMPTY_LOCAL_CATALOG_V1);
  const localCatalogRef = useRef<LocalCatalog>(EMPTY_LOCAL_CATALOG_V1);
  localCatalogRef.current = localCatalog;
  const localCatalogClientInstanceIdRef = useRef<string | null>(null);
  const localDocumentOwnersRef = useRef(new Map<string, { readonly status?: (persisted: boolean) => void; readonly archive?: (archive: Uint8Array) => void }>());
  const rehydratedLocalDocumentsRef = useRef(new Map<number, Set<string>>());
  const presenceConnectedAtMsRef = useRef(Date.now());
  const presenceCursorRef = useRef<{ readonly x: number; readonly y: number } | undefined>(undefined);
  /** 🐚️ terra-web-shellhost (finding 5) — per-document `latestWins` triggers for the presence-beat
   * effect below: keyed lazily on first beat so each document gets its own single-flight-with-trailing-
   * coalesce wrapper instead of sharing one across every open document, which would serialize them.
   * Cleared on that effect's own cleanup (identity/ephemeral change or unmount). */
  const presenceBeatTriggersRef = useRef<Map<string, () => Promise<void>>>(new Map());
  /** 🌐️ terra-web-shellhost (finding 4) — one `AbortController` for the whole component's lifetime,
   * aborted from the unmount-teardown effect below (never recreated: this file's own `presenceConnectedAtMsRef`
   * a few lines up is the same "construct once, keep for the component's life" idiom). Ties every
   * extension install fetch (`installExtension`/`installExtensionFromFile`/`uninstallExtension`) to
   * the component's own lifetime instead of leaving them free-running past unmount. */
  const extensionFetchAbortRef = useRef(new AbortController());
  /** 🧵 Cancels every active operation-owned download drain when this shell unmounts. */
  const segmentedDownloadAbortRef = useRef(new AbortController());
  /** 🗂️ Which session/plugin owns each exact document runtime, so equal document ids in different
   * spaces cannot share socket, bootstrap, presence or plugin-routing state. */
  const openDocumentSessionsRef = useRef<Map<string, OpenDocumentSession>>(new Map());
  const failDocumentBackbone = useCallback((runtimeKey: string, entry: OpenDocumentSession, failure: unknown) => {
    if (openDocumentSessionsRef.current.get(runtimeKey) !== entry) return;
    const error = failure instanceof Error ? failure : new Error(String(failure));
    if (entry.replacements.pending) {
      return;
    }
    entry.creationMount?.close(error);
    entry.rejectReady(error);
    closeDocumentRef.current(runtimeKey, entry.clientInstanceId);
  }, []);
  const receiveDocumentBackbone = useCallback((runtimeKey: string, entry: OpenDocumentSession, message: Uint8Array) => {
    if (openDocumentSessionsRef.current.get(runtimeKey) !== entry) return;
    try {
      if (!(message instanceof Uint8Array) || message.length === 0 || message.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES || !["mutations", "retract"].includes(decodeBackboneMessage(message).kind)) throw new Error("document-backbone.invalid-message");
      if (entry.port === null) {
        if (entry.pending.length >= DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumMessages || message.length > DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes - entry.pendingBytes) {
          return;
        }
        entry.pending.push(message.slice());
        entry.pendingBytes += message.length;
      } else void entry.persistence.ingested(entry.port.receive({ runtimeKey, clientInstanceId: entry.clientInstanceId, scope: entry.scope ?? null }, message)).catch(error => failDocumentBackbone(runtimeKey, entry, error));
    } catch (error) { failDocumentBackbone(runtimeKey, entry, error); }
  }, [failDocumentBackbone]);
  const bindDocumentBackbone = useCallback(async (runtimeKey: string, entry: OpenDocumentSession, admitted: () => boolean = () => true): Promise<void> => {
    const current = () => admitted() && openDocumentSessionsRef.current.get(runtimeKey) === entry;
    if (!current()) return;
    if (!entry.plugin.bindDocumentPort) throw new Error("document-backbone.binding-unavailable");
    const worker = backboneWorkerRef.current;
    if (worker === null) throw new Error("document-backbone.worker-unavailable");
    const preparation: { port?: ActorDocumentMessagePortV1 } = {};
    try { await entry.plugin.bindDocumentPort(entry.session.instanceId, {
      runtimeKey, clientInstanceId: entry.clientInstanceId, scope: entry.scope ?? null, current,
      send: message => {
        if (!current() || backboneWorkerRef.current !== worker) return;
        if (documentBackboneEffectV1(message) === "remote-ingest-receipt") return;
        worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId: entry.documentId, clientInstanceId: entry.clientInstanceId, ...(entry.scope === undefined ? {} : { spaceId: entry.scope.spaceId }), message: { kind: "documentBackbone", message } }) });
        void entry.persistence.published().catch(error => failDocumentBackbone(runtimeKey, entry, error));
      },
      merge: (conflicts, report) => { if (current()) applyRemoteMergeRef.current(conflicts, report); },
      prepared: port => {
        preparation.port = port;
        entry.port = port;
        let ports = documentPortsRef.current.get(entry.plugin);
        if (ports === undefined) { ports = new Map(); documentPortsRef.current.set(entry.plugin, ports); }
        ports.set(entry.session.instanceId, port);
        const pending = entry.pending;
        entry.pending = [];
        entry.pendingBytes = 0;
        for (const bytes of pending) receiveDocumentBackbone(runtimeKey, entry, bytes);
      },
    }); } catch (error) {
      const candidate = preparation.port;
      await candidate?.retire().catch(() => {});
      if (candidate !== undefined) {
        const ports = documentPortsRef.current.get(entry.plugin);
        if (ports?.get(entry.session.instanceId) === candidate) ports.delete(entry.session.instanceId);
        if (entry.port === candidate) {
          entry.port = null;
          entry.pending = [];
          entry.pendingBytes = 0;
        }
      }
      throw error;
    }
    if (current()) entry.resolveReady();
  }, [receiveDocumentBackbone]);
  /** 🗃️ Loads one archive into a program through the stepped, ACK-owned archive load: `task` hears every polled status and
   * cancels it; a cancelled or faulted load leaves the previous document exactly as it was and its backbone bound again. */
  const loadDocumentArchive = useCallback(async (plugin: PluginWasmHandle, instanceId: number, archive: DocumentArchivePack, current: () => boolean, task?: { readonly signal?: AbortSignal; readonly progress?: (status: DocumentArchiveLoadStatus) => void }): Promise<boolean> => {
    const archiveBytes = encodeDocumentArchiveBytes(archive);
    if (archiveBytes.length > DOCUMENT_ARCHIVE_MAXIMUM_BYTES) throw new Error("document-backbone.archive-capacity");
    if (!plugin.loadAppDocumentArchive) throw new Error("document-backbone.archive-loader-unavailable");
    const load = plugin.loadAppDocumentArchive;
    const owned = [...openDocumentSessionsRef.current].find(([, entry]) => entry.plugin === plugin && entry.session.instanceId === instanceId);
    const lane = documentAttachmentLane(plugin, instanceId);
    if (owned === undefined) {
      const owner = `cold:${crypto.randomUUID()}`;
      const unbound = () => current() && ![...openDocumentSessionsRef.current.values()].some(entry => entry.plugin === plugin && entry.session.instanceId === instanceId);
      let loaded = false;
      try { await lane.replace(owner, unbound, async () => { if (unbound()) { await load(instanceId, archive, task?.signal, task?.progress); loaded = unbound(); } }); }
      finally { await lane.close(owner); }
      return loaded;
    }
    const [runtimeKey, entry] = owned;
    const retirement = entry.port?.retire();
    void retirement?.catch(() => {});
    entry.port = null;
    return entry.replacements.replace(archive, (candidate, latest) => {
      const exact = () => latest() && current() && openDocumentSessionsRef.current.get(runtimeKey) === entry;
      return replaceAttachedDocumentV1(lane, entry.clientInstanceId, exact, { retired: retirement, load: () => load(instanceId, candidate, task?.signal, task?.progress), bind: () => bindDocumentBackbone(runtimeKey, entry, latest) });
    });
  }, [bindDocumentBackbone, documentAttachmentLane]);
  const loadDocumentPair = useCallback(async (plugin: PluginWasmHandle, instanceId: number, pack: Uint8Array, spr: Uint8Array, current: () => boolean, task?: { readonly signal?: AbortSignal; readonly progress?: (status: DocumentArchiveLoadStatus) => void }): Promise<boolean> =>
    loadDocumentArchive(plugin, instanceId, { parent_pack: Array.from(pack), parent_spr: Array.from(spr), members: [] }, current, task), [loadDocumentArchive]);
  const captureDialogOrigin = useCallback((target: ActiveSession | null): ShellDialogOriginV1 | null =>
    shellDialogOriginV1(target, [...openDocumentSessionsRef.current].map(([runtimeKey, entry]) => ({ runtimeKey, ...entry }))), []);
  const isCurrentDialogOrigin = useCallback((origin: ShellDialogOriginV1 | null): boolean =>
    shellDialogOriginIsCurrentV1(origin, captureDialogOrigin(core.shellStateRef.current.pluginRuntime.session)), [captureDialogOrigin]);
  return { captureDialogOrigin, loadDocumentArchive, isCurrentDialogOrigin, openDocumentSessionsRef, hubConnectionPort, spaceAdministrationRef, spaceAdministrationEpochRef, spaceAdministrationMessageRef, inferencePortEpochRef, inferencePortOwnerRef, inferencePortAuthorityRef, retiredSessionDocumentOwnersRef, inferencePortOpeningRef, setInferenceHistoryByRuntimeKey, inferenceHistoryByRuntimeKeyRef, identitySnapshotResolverRef, identityClientInstanceIdRef, identityBootstrapAbortRef, localDataDir, localCatalog, setLocalCatalog, localCatalogRef, localCatalogClientInstanceIdRef, localDocumentOwnersRef, rehydratedLocalDocumentsRef, presenceConnectedAtMsRef, presenceCursorRef, presenceBeatTriggersRef, extensionFetchAbortRef, segmentedDownloadAbortRef, failDocumentBackbone, receiveDocumentBackbone, bindDocumentBackbone, loadDocumentPair };
}

/** 💬 Builds the effect owner capture, the document archive ports and the owned dialogs. */
export function useEffectOwnersAndDialogs({ captureDialogOrigin, core, spaceArtifactCreationCatalogRef, spaceArtifactCreationCatalogUiRef, sessionRef, historyStore, loadDocumentArchive, refreshHistorySnapshot, liveDialogRef, dispatch, isCurrentDialogOrigin, dialogOpeningRef, openDocumentSessionsRef, identityRef, documentAccess }: Pick<ReturnType<typeof useHubPortInferenceAndDocumentBackbone>, "captureDialogOrigin" | "loadDocumentArchive" | "isCurrentDialogOrigin" | "openDocumentSessionsRef"> & { readonly core: ShellCoreAfterShellCoreState } & Pick<ReturnType<typeof useIdentityAgentBridgeAndDirectoryState>, "spaceArtifactCreationCatalogRef" | "spaceArtifactCreationCatalogUiRef" | "sessionRef" | "historyStore" | "refreshHistorySnapshot" | "liveDialogRef" | "dispatch" | "dialogOpeningRef" | "identityRef" | "documentAccess">) {
  /** 🔗️ `inputSeq` is the ledger entry (`🎯️input-ledger`) whose guest answer this owner's effect pass
   * applies — `onAction` passes its own entry, every other owner (a refresh, a tutorial, a route) is a
   * root (`null`) — so a guest follow-up (`replayShellCommand`, `dispatchAction`) can carry `causedBy`. */
  const captureEffectOwner = useCallback((source: ActiveSession, presentation: ShellDialogOriginV1 | null, inputSeq: number | null = null) => ({
    presentation,
    source: captureDialogOrigin(source),
    session: source,
    plugin: core.loadedPluginsRef.current.find((entry) => entry.handle.pluginId === source.pluginId)?.handle ?? null,
    creationCatalog: captureSpaceArtifactCreationCatalogAuthorityV1(spaceArtifactCreationCatalogRef.current, spaceArtifactCreationCatalogUiRef.current, presentation),
    inputSeq,
  }), [captureDialogOrigin]);
  const isCurrentEffectOwner = useCallback((owner: ReturnType<typeof captureEffectOwner>): boolean => {
    const primary = core.shellStateRef.current.pluginRuntime.session;
    return owner.plugin !== null
      && core.loadedPluginsRef.current.find((entry) => entry.handle.pluginId === owner.session.pluginId)?.handle === owner.plugin
      && shellEffectOwnerIsCurrentV1(owner, { presentation: captureDialogOrigin(primary), mounted: captureDialogOrigin(owner.session), primary, spawned: primary === null ? [] : parsePanelState(primary.viewState)?.spawnedApps ?? [] });
  }, [captureDialogOrigin]);
  /** 🎭️ The owner of a pass a PROGRAM runs on its own — its operation progress and completions, its deferred effects:
   * the primary session presents it, the program is its source ({@link shellEffectOwnerIsCurrentV1}). */
  const captureProgramEffectOwner = useCallback((program: ActiveSession) => captureEffectOwner(program, captureDialogOrigin(core.shellStateRef.current.pluginRuntime.session)), [captureDialogOrigin, captureEffectOwner]);
  /** 🚪️ What restoring an archive into a program drives ({@link FolderReadBackRouteV1}): the
   * stepped load through the document's attachment lane — every polled status is the program's `load` status outside the
   * History panel until the load ends, as for a file-open load — the program's history re-read and a full refresh of
   * every surface it renders, through the live session when the program is the primary one. */
  const documentArchivePorts = useCallback((plugin: PluginWasmHandle, program: ActiveSession, current: () => boolean) => {
    const live = (): ActiveSession => {
      const primary = sessionRef.current;
      return primary !== null && primary.pluginId === program.pluginId && primary.instanceId === program.instanceId ? primary : program;
    };
    const showLoad = (load: { readonly completed: number; readonly total: number } | null) => historyStore.update((projections) => programHistoryProjectionsWithLoadV1(projections, programHistoryKeyV1(program), load));
    return {
      load: (value: DocumentArchivePack) => loadDocumentArchive(plugin, program.instanceId, value, current, { progress: showLoad }).finally(() => showLoad(null)),
      history: () => refreshHistorySnapshot(program),
      refresh: () => core.applyHostEffectsRef.current([], live(), { kind: "full" }, captureProgramEffectOwner(live())),
    };
  }, [captureProgramEffectOwner, historyStore, loadDocumentArchive, refreshHistorySnapshot]);
  const closeOwnedDialog = useCallback((openingId: number): boolean => {
    if (liveDialogRef.current?.openingId !== openingId) return false;
    liveDialogRef.current = null;
    dispatch({ type: "CLOSE_DIALOG", openingId });
    return true;
  }, []);
  const makeOwnedDialog = useCallback((dialogId: string, origin: ShellDialogOriginV1 | null, seedArgs?: Readonly<Record<string, unknown>>): ShellDialogV1 | null => {
    const active = core.shellStateRef.current.pluginRuntime.session;
    if (origin === null || !isCurrentDialogOrigin(origin) || !active?.app.dialogs?.some((entry) => entry.id === dialogId)) return null;
    return { openingId: ++dialogOpeningRef.current, dialogId, origin, ...(seedArgs === undefined ? {} : { seedArgs }) };
  }, [isCurrentDialogOrigin]);
  
  const browserActorUiByRuntimeKeyRef = useRef(new Map<string, RetainedBrowserActorUiV1>());
  const [browserActorUiVersion, setBrowserActorUiVersion] = useState(0);
  /** 🎭️ The painted actor serving `session`'s document, if any: its reserved section stores are then the ONLY source of the
   * session's engagements, measures, tool measures and catalogue — the local instance never sees a hub document, so its
   * sections froze at the opening state (C13 P1: draw's "N layers" stayed 1 after committed `addLayer`s). */
  const browserActorForSessionV1 = (session: Pick<ActiveSession, "pluginId" | "instanceId">): RetainedBrowserActorUiV1 | null => {
    for (const [runtimeKey, retained] of browserActorUiByRuntimeKeyRef.current) {
      const entry = openDocumentSessionsRef.current.get(runtimeKey);
      if (entry?.clientInstanceId === retained.clientInstanceId && retained.sessionInstanceId === session.instanceId && entry.session.pluginId === session.pluginId && entry.session.instanceId === session.instanceId) return retained;
    }
    return null;
  };
  /** 🧩️ Dispatches `cache`'s reserved sections — after taking them from `retained`'s section stores when an actor serves the
   * session — exactly as a local refresh does, so window chrome and tool measures follow the live document in every open
   * window. The app-static catalogue is kept by identity, so a scene host subscribing through `AppCatalogueContext` never
   * re-renders on an unchanged one. */
  const publishUiRefreshSectionsV1 = (cache: UiRefreshCache, retained: RetainedBrowserActorUiV1 | null): void => {
    if (retained !== null) for (const [key, { revision, value }] of browserActorSectionValuesV1(retained.sections, `actor ${retained.verifiedSurfaceId}`)) cache.set(key, { hash: `actor:${revision}`, value });
    const engagements = (cache.get("engagements")?.value as Readonly<Record<string, WindowEngagement>> | undefined) ?? {},
      measures = (cache.get("measures")?.value as Readonly<Record<string, readonly WindowMeasure[]>> | undefined) ?? {},
      toolMeasures = (cache.get("tools")?.value as Readonly<Record<string, readonly WindowMeasure[]>> | undefined) ?? {},
      catalogue = (cache.get("catalogue")?.value as AppCatalogue | undefined) ?? EMPTY_APP_CATALOGUE;
    dispatch({ type: "SET_WINDOW_ENGAGEMENTS_BY_WINDOW_ID", value: (current) => mergeRecordPreservingIdentity(current, Object.entries(engagements)) });
    dispatch({ type: "SET_WINDOW_MEASURES_BY_WINDOW_ID", value: (current) => mergeRecordPreservingIdentity(current, Object.entries(measures)) });
    dispatch({ type: "SET_TOOL_MEASURES_BY_TOOL_ID", value: (current) => mergeRecordPreservingIdentity(current, withProgramEntriesV1(current, null, core.spawnedIdsRef.current, Object.entries(toolMeasures))) });
    dispatch({ type: "SET_APP_CATALOGUE", value: (current) => preserveJsonIdentity(current, catalogue) });
  };
  const retireBrowserActorUi = useCallback((runtimeKey: string, reason: string) => {
    browserActorUiByRuntimeKeyRef.current.get(runtimeKey)?.actions.close(reason);
    if (browserActorUiByRuntimeKeyRef.current.delete(runtimeKey)) setBrowserActorUiVersion((current) => current + 1);
  }, []);
  const directoryScopedOwnersRef = useRef<Map<string, DocumentScope>>(new Map());
  /** 📇️ The directory history of every mounted space index, by its runtime key ({@link SpaceDirectoryHistoryV1}). */
  const spaceDirectoryHistoriesRef = useRef<Map<string, SpaceDirectoryHistoryV1>>(new Map());
  const spaceDirectoryFoldPendingRef = useRef(false);
  const foldSpaceDirectoryRef = useRef<() => void>(() => {});
  /** 📇️ Collects directory events into every mounted space index's history of their space and folds the current
   * index's full history once per turn when it changed. */
  /** 👁️ The surface role the signed-in human may open each shared space's documents with ({@link sharedDocumentOpeningRoleV1}),
   * decided at every opening from the space index and kept current while an index of the space folds directory events. */
  const sharedSpaceAccessRef = useRef<Map<string, AppRole>>(new Map());
  const [sharedSpaceAccessVersion, setSharedSpaceAccessVersion] = useState(0);
  const adoptSharedSpaceAccess = useCallback((spaceId: string, role: AppRole | null): void => {
    if (role === null || sharedSpaceAccessRef.current.get(spaceId) === role) return;
    sharedSpaceAccessRef.current.set(spaceId, role);
    setSharedSpaceAccessVersion((version) => version + 1);
  }, []);
  const collectSpaceDirectoryEvents = useCallback((events: readonly DirectoryEvent[]): void => {
    let changed = false;
    for (const history of spaceDirectoryHistoriesRef.current.values()) {
      if (!history.add(events)) continue;
      changed = true;
      if (identityRef.current) adoptSharedSpaceAccess(history.spaceId, sharedDocumentOpeningRoleV1(history.events(), history.spaceId, identityRef.current.userId, documentAccess));
    }
    if (!changed || spaceDirectoryFoldPendingRef.current) return;
    spaceDirectoryFoldPendingRef.current = true;
    queueMicrotask(() => {
      spaceDirectoryFoldPendingRef.current = false;
      foldSpaceDirectoryRef.current();
    });
  }, [documentAccess]);
  return { captureEffectOwner, isCurrentEffectOwner, captureProgramEffectOwner, documentArchivePorts, closeOwnedDialog, makeOwnedDialog, browserActorUiByRuntimeKeyRef, browserActorUiVersion, setBrowserActorUiVersion, browserActorForSessionV1, publishUiRefreshSectionsV1, retireBrowserActorUi, directoryScopedOwnersRef, spaceDirectoryHistoriesRef, foldSpaceDirectoryRef, sharedSpaceAccessRef, sharedSpaceAccessVersion, adoptSharedSpaceAccess, collectSpaceDirectoryEvents };
}
