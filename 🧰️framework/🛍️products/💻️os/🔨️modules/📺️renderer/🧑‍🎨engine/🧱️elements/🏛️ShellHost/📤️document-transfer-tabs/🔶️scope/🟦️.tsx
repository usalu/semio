/** 🔶️ The scope phase of useDocumentTransferAndTabs. */
import { programRetirementKeepsDocumentV1, type ProgramRetirementOwnerV1 } from "../../🚪️program-retirement/🟦️.ts";
import { NATURAL_FILE_PORT_ID, parseSurfaceAppId, type AppDefinition, type Severity } from "@semio-tech/framework";
import { decodePackValue } from "@semio-tech/framework/packs";
import { DOCUMENT_ARCHIVE_MAXIMUM_BYTES, decodeDocumentArchiveBytes, encodeDocumentArchiveBytes, type DocumentArchivePack } from "@semio-tech/framework/wire-contracts";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { DOCUMENT_ARCHIVE_FILE_EXTENSION, DOCUMENT_ARCHIVE_MEDIA_TYPE, documentArchiveFileNameV1, beginDocumentTransferV1, naturalFileSaveNameV1, naturalFileFormatV1, naturalMediaDescriptorMatchesV1, openNaturalFileOwnerV1, documentTransferNoticeTextV1, documentTransferNoticeCodeV1, type DocumentTransferNoticeV1 } from "../../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { naturalFileMediaInputV1 } from "../../../🛠️ShellHelpers/🎞️media-import/🟦️.ts";
import { NATURAL_FILE_BROWSER_READ_CHUNK_BYTES_V1, readBlobBytesBoundedV1, requestFileSelectionV1, downloadMediaExportBytes, shellSegmentedDownloadSinkFactory } from "../../../🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import { buildSpacePanelState, parsePanelState, studioPanelFocusingSpawned } from "../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { type PluginWasmHandle } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { documentTransferTasksV1, type TaskManagerDocumentTransferV1 } from "../../../🧵️TaskManager/🟦️.tsx";
import { openPickedFileV1, pickedFileRefusalCodeV1, pickedFileRefusalTextV1, readPickedFileHeadV1, readPickedFileRangeV1, routePickedFileV1 } from "../../📂️picked-file-opening/🟦️.ts";
import { spacePanelOwnerLeafIdV1 } from "../../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterThemeDriverSettings } from "../../🎨️theme-driver-settings/🟦️.tsx";
import { admitLocalDocument, retireLocalDocument, type LocalDocument } from "@semio-tech/framework/config/mutations";
import { keepArtifactOpeningV1 } from "../../🗂️local-catalog/📥️artifact/💾️opening/🟦️.ts";
import { programHistoryKeyV1, programHistoryProjectionV1, spawnedProgramViewStateV1 } from "../../🪟️spawned-program/🟦️.ts";
import { shellUncommittedEditCountV1 } from "../../🏁️operation-settle/🟦️.ts";
import { EMPTY_SHELL_HISTORY_PROJECTION_V1 } from "../../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { openedProgramPublicationV1 } from "../../📂️opened-program/🟦️.ts";
import { localCatalogBindingV1 } from "../../🗂️local-catalog/🟦️.ts";
import { LOCAL_CATALOG_WRITE_DEADLINE_MS_V1, sameLocalDocumentArchiveV1 } from "../../🗂️local-catalog/🟦️.ts";
import { awaitVerifiedReadbackV1 } from "../../🗂️local-catalog/💾️acknowledgment/🟦️.ts";
import { FOLDER_ENDPOINT_PATH } from "../../../../../../🏪️store/👷️worker/⚙️constants/🟦️.ts";
import { readSignedDocumentArchiveV1 } from "../📦️signed-archive/🟦️.ts";
import { readOwnedDocumentArchiveV1 } from "../../🗄️shell-uri-documents/🔸️open-document/🔏️archive-owner/🟦️.ts";
import { encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { declaredMediaFileExportV1, declaredNaturalMediaFileExportV1, downloadDeclaredMediaFileV1, deliverNaturalMediaFileV1, type MediaFileExportActionIdV1 } from "../../../../🎬️media/📤️file/🟦️.ts";

/** 🔷 Builds scope, localDocumentOwnerRegistry, hostMode, dispatch of useDocumentTransferAndTabs. */
export function buildScopeState({ core }: { readonly core: ShellCoreAfterThemeDriverSettings }) {
  const {
    scope, localDocumentOwnerRegistry, hostMode, dispatch, loadedPlugins, session, uiLocale, uiTerminology,
    uiKeybindingOverrides, hostApp, landingAppId, hostAppId, spacePrograms, appLabelsOverlay, openWithFocusRole,
    clipboardFragmentRef, clipboardQueue, panelTreeConfigCacheRef, treeWindowGeneration, treeWindowHost, agentBridge, uiLocaleRef,
    showTransientNoticeRef, openDocumentSessionsRef, browserActorUiByRuntimeKeyRef, browserActorUiVersion,
    foldSpaceDirectoryRef, panel, focusedApp, focusedSpawnedId, focusedPanelBodyStores, focusedWindowInstancesRef,
    activeUtilityByWindowIdRef, activeToolIdRef, utilityGenerationFor, toolGeneration, activeWindowIdRef,
    effectiveModeLayoutRef, actionPaneExpandedByWindowIdRef, actionPaneStagedArgsByKeyRef, updateSpacePanel,
    resolveSyncTargetSession, updateSpacePanelFrom, onAction, onActionStable, onBrowserActorIntent, dispatchSetDefaultApp,
    dispatchClearDefaultApp, openArtifactWithAppRef, openWithEntries, showTransientNotice, frameworkSettingsTab,
    taskManagerListenersRef, documentTransfersRef, documentTransferFlightRef, documentTransferTasksRef
  } = core;
  //#region FrameworkOsShell
  //#region 📤️DocumentTransfer
  const transferInFlightRef = documentTransferFlightRef;
  /** 🪟️ Focuses a freshly created program instance in the space panel, built from the latest panel at commit time; `false` when no session is mounted. A standalone program host has no host app, so its first panel comes from the mounted session's own app. */
  const focusSpawnedProgram = (spawned: { readonly pluginId: string; readonly appId: string; readonly instanceId: number; readonly label: string | undefined; readonly breadcrumb: readonly string[] | undefined }): boolean =>
    updateSpacePanelFrom((latest) => studioPanelFocusingSpawned(latest ?? buildSpacePanelState([], spacePanelOwnerLeafIdV1(hostApp, core.sessionRef.current?.app)), {
      id: `${spawned.pluginId}-${spawned.instanceId}`,
      pluginId: spawned.pluginId,
      instanceId: spawned.instanceId,
      appId: spawned.appId,
      label: spawned.label ?? spawned.appId,
      breadcrumb: spawned.breadcrumb ?? [],
    }));
  /** 📂️ Shows the program of a document the user opened ({@link openedProgramPublicationV1}): a host focuses it as a tab; a standalone program host adopts it as its one program through the session switch, which persists and retires the predecessor, so every window of the layout follows the opened document. The predecessor's uncommitted edits are checkpointed first, while it still owns the canvas: the check-in that runs once the session changed would address the program that replaced it and be refused. `false` when it could not be shown. */
  const publishOpenedProgram = async (opened: Readonly<{ handle: PluginWasmHandle; app: AppDefinition; instanceId: number; label: string | undefined; breadcrumb: readonly string[] | undefined }>, task: Readonly<{ signal: AbortSignal; progress: (completed: number, total: number | null) => void }>, current: () => boolean): Promise<boolean> => {
    const primary = core.sessionRef.current;
    const publication = openedProgramPublicationV1(hostMode, primary === null ? null : { pluginId: primary.pluginId, appId: primary.app.id, instanceId: primary.instanceId }, { pluginId: opened.handle.pluginId, appId: opened.app.id, instanceId: opened.instanceId });
    if (publication === "focus-tab") return focusSpawnedProgram({ pluginId: opened.handle.pluginId, appId: opened.app.id, instanceId: opened.instanceId, label: opened.label, breadcrumb: opened.breadcrumb });
    if (publication === "already-primary") return true;
    if (primary !== null && shellUncommittedEditCountV1(programHistoryProjectionV1(core.historyStore.get(), programHistoryKeyV1(primary), EMPTY_SHELL_HISTORY_PROJECTION_V1).entries) > 0) {
      const identity = core.identityRef.current;
      await onAction({ controllerId: primary.app.controllerId, action: "commitCheckpoint", args: { message: "auto", authors: identity ? [{ id: identity.userId, name: identity.displayName }] : [] } });
      if (!current() || task.signal.aborted) return false;
    }
    let published = false;
    await core.adoptPreparedPrimarySession(opened.app.id, {
      handle: opened.handle, instanceId: opened.instanceId, current: () => current() && !task.signal.aborted,
      retirement: { signal: task.signal, persist: (owner, signal) => persistProgramDocument(owner, { signal, progress: task.progress }) },
      onPublished: () => { published = true; },
    });
    return published;
  };
  const publishDocumentTransfers = (): void => {
    documentTransferTasksRef.current = documentTransferTasksV1([...documentTransfersRef.current.values()].map((entry) => entry.transfer));
    for (const listener of [...taskManagerListenersRef.current]) listener();
  };
  const notifyDocumentTransfer = (notice: DocumentTransferNoticeV1, file: string, kind: Severity): void => {
    showTransientNoticeRef.current(documentTransferNoticeTextV1(notice, file, uiLocaleRef.current), kind, documentTransferNoticeCodeV1(notice));
  };
  /** 🧵️ One file moving into a program as a cancellable Tasks-window task (`documentTransfer` lane): `progress` states
   * `completed` of `total` units (`null` = open-ended), Cancel aborts `signal`, `finish` removes the row. */
  const trackDocumentTransfer = (file: string, pluginId: string): { readonly signal: AbortSignal; readonly cancel: () => void; readonly progress: (completed: number, total: number | null) => void; readonly finish: () => void } => {
    const id = crypto.randomUUID();
    const abort = new AbortController();
    const update = (patch: Partial<TaskManagerDocumentTransferV1>): void => {
      const entry = documentTransfersRef.current.get(id);
      if (entry === undefined) return;
      documentTransfersRef.current.set(id, { ...entry, transfer: { ...entry.transfer, ...patch } });
      publishDocumentTransfers();
    };
    documentTransfersRef.current.set(id, { transfer: { id, file, pluginId, startedAtMs: Date.now(), progress: null, cancelling: false }, abort });
    abort.signal.addEventListener("abort", () => update({ cancelling: true }), { once: true });
    publishDocumentTransfers();
    return {
      signal: abort.signal,
      cancel: () => abort.abort(),
      progress: (completed, total) => update({ progress: { completed, total, text: `${completed}/${total ?? "?"}` } }),
      finish: () => {
        documentTransfersRef.current.delete(id);
        publishDocumentTransfers();
      },
    };
  };
  /** 🎯️ The focused document program, or `null` when the Home or the space host has the canvas. */
  const documentTransferTarget = (): { readonly target: ActiveSession; readonly handle: PluginWasmHandle } | null => {
    const target = resolveSyncTargetSession();
    if (target === null || target.app.id === hostAppId || target.app.id === landingAppId) return null;
    const handle = loadedPlugins.find((entry) => entry.handle.pluginId === target.pluginId)?.handle;
    return handle === undefined ? null : { target, handle };
  };
  /** 📤️ Export Document: the focused program's archive, written as one `.semio-archive` file. */
  const exportDocumentArchive = async (): Promise<void> => {
    const owned = documentTransferTarget();
    if (owned === null) return notifyDocumentTransfer("no-document", "", "info");
    const file = documentArchiveFileNameV1(owned.target.app.id, new Date());
    const task = trackDocumentTransfer(file, owned.target.pluginId);
    try {
      const actor = core.directBrowserActorForSession(owned.target);
      let bytes: Uint8Array;
      if (actor === null) {
        if (!owned.handle.readAppDocumentArchive) throw new Error("document-backbone.archive-reader-unavailable");
        bytes = encodeDocumentArchiveBytes(await owned.handle.readAppDocumentArchive(owned.target.instanceId));
      } else {
        const { entry, current } = actor;
        if (entry.scope === undefined) throw new Error("document-backbone.archive-owner-unavailable");
        bytes = await readSignedDocumentArchiveV1(core.ensureBackboneWorker(), entry.scope, entry.clientInstanceId, current, task.signal, task.progress);
        if (!current()) throw new Error("document-backbone.archive-owner-retired");
      }
      task.signal.throwIfAborted();
      downloadMediaExportBytes(file, DOCUMENT_ARCHIVE_MEDIA_TYPE, bytes);
      notifyDocumentTransfer("exported", file, "info");
    } catch (error) {
      if (task.signal.aborted) return notifyDocumentTransfer("export-cancelled", file, "info");
      console.error(`[os-shell] ${documentTransferNoticeCodeV1("export-failed")} ${owned.target.pluginId}#${owned.target.instanceId}`, error);
      notifyDocumentTransfer("export-failed", file, "error");
    } finally {
      task.finish();
    }
  };
  /** 📥️ Import Document: a picked archive opens as a NEW document of the focused program, loaded with progress and
   * cancellation from the Tasks window; the focused document stays untouched. */
  const importDocumentArchive = async (): Promise<void> => {
    const owned = documentTransferTarget();
    if (owned === null || session === null) return notifyDocumentTransfer("no-document", "", "info");
    let picked: File | undefined;
    try {
      [picked] = await requestFileSelectionV1(`${DOCUMENT_ARCHIVE_FILE_EXTENSION},${DOCUMENT_ARCHIVE_MEDIA_TYPE}`);
    } catch (error) {
      console.warn(`[os-shell] ${documentTransferNoticeCodeV1("import-failed")} file selection`, error);
      return notifyDocumentTransfer("import-failed", "", "error");
    }
    if (picked === undefined) return;
    if (!beginDocumentTransferV1(transferInFlightRef)) return notifyDocumentTransfer("transfer-busy", picked.name, "info");
    try {
      await importDocumentArchiveOnce(owned, session, picked);
    } finally {
      transferInFlightRef.current = false;
    }
  };
  const importDocumentArchiveOnce = async (owned: NonNullable<ReturnType<typeof documentTransferTarget>>, hostOwner: ActiveSession, picked: File): Promise<void> => {
    const current = (): boolean => {
      const host = core.sessionRef.current;
      return host?.pluginId === hostOwner.pluginId && host.app.id === hostOwner.app.id && host.instanceId === hostOwner.instanceId && core.loadedPluginsRef.current.some(entry => entry.handle === owned.handle) && !core.pluginOpInFlightRef.current.has(owned.handle.pluginId);
    };
    const load = owned.handle.loadAppDocumentArchive;
    const task = trackDocumentTransfer(picked.name, owned.target.pluginId);
    let instanceId: number | null = null;
    let published = false;
    try {
      if (!current()) throw new Error("archive-file.owner-retired");
      if (!load) throw new Error("document-backbone.archive-loader-unavailable");
      let archive: DocumentArchivePack;
      try {
        const bytes = await readBlobBytesBoundedV1(picked, {
          signal: task.signal, maximumBytes: DOCUMENT_ARCHIVE_MAXIMUM_BYTES, chunkBytes: NATURAL_FILE_BROWSER_READ_CHUNK_BYTES_V1, progress: task.progress,
        });
        task.signal.throwIfAborted();
        archive = decodeDocumentArchiveBytes(bytes);
      } catch (error) {
        task.signal.throwIfAborted();
        console.warn(`[os-shell] ${documentTransferNoticeCodeV1("import-unreadable")} ${picked.name}`, error);
        return notifyDocumentTransfer("import-unreadable", picked.name, "warning");
      }
      if (!current()) throw new Error("archive-file.owner-retired");
      instanceId = await owned.handle.createApp(owned.target.app.id);
      if (!current() || task.signal.aborted) throw task.signal.reason ?? new Error("archive-file.owner-retired");
      await load(instanceId, archive, task.signal, (status) => task.progress(status.completed, status.total > 0 ? status.total : null));
      if (!current() || task.signal.aborted) throw task.signal.reason ?? new Error("archive-file.owner-retired");
      const program = spacePrograms.find((entry) => entry.pluginId === owned.target.pluginId && entry.appId === owned.target.app.id);
      published = await publishOpenedProgram({ handle: owned.handle, app: owned.target.app, instanceId, label: program?.label, breadcrumb: program?.breadcrumb }, task, current);
      if (!published) throw task.signal.reason ?? new Error("import.publication-unavailable");
      notifyDocumentTransfer("imported", picked.name, "info");
    } catch (error) {
      const notice: DocumentTransferNoticeV1 = task.signal.aborted ? "import-cancelled" : "import-failed";
      console.warn(`[os-shell] ${documentTransferNoticeCodeV1(notice)} ${picked.name}`, error);
      notifyDocumentTransfer(notice, picked.name, task.signal.aborted ? "info" : "error");
      if (instanceId !== null && !published) await owned.handle.destroyApp(instanceId).catch((destroyError: unknown) => console.error(`[os-shell] imported instance ${instanceId} could not be closed`, destroyError));
    } finally {
      task.finish();
    }
  };
  const keepProgramDocument = async (owner: Readonly<{ handle: PluginWasmHandle; app: AppDefinition }>, instanceId: number, name: string, task: Readonly<{ signal: AbortSignal; progress: (completed: number, total: number | null) => void }>, hostSession: ActiveSession | null, held: () => boolean, publish: (current: () => boolean) => boolean | Promise<boolean>): Promise<boolean> => {
    const current = (): boolean => {
      const host = core.sessionRef.current;
      return held() && hostSession !== null && host?.pluginId === hostSession.pluginId && host?.app.id === hostSession.app.id && host?.instanceId === hostSession.instanceId && core.loadedPluginsRef.current.some((entry) => entry.handle === owner.handle);
    };
    const schema = owner.app.io?.artifactSchema;
    if (!schema || !owner.handle.readAppDocumentArchive || !current()) throw new Error("natural-file.persistence-owner-unavailable");
    let documentId: string | null = null;
    return keepArtifactOpeningV1({
      name, schema, surface: { pluginId: owner.handle.pluginId, appId: owner.app.id }, dataDir: core.localDataDir, nowMs: Date.now(),
      signal: task.signal, current, progress: task.progress,
      identity: async () => { const identity = await owner.handle.readAppDocumentIdentity(instanceId); documentId = identity.parent_document_id; return identity; },
      archive: async () => {
        const before = await owner.handle.readAppDocumentIdentity(instanceId);
        if (before.parent_document_id !== documentId) throw new Error("natural-file.archive-owner-changed");
        const archive = await owner.handle.readAppDocumentArchive!(instanceId);
        const after = await owner.handle.readAppDocumentIdentity(instanceId);
        if (after.parent_document_id !== documentId) throw new Error("natural-file.archive-owner-changed");
        return archive;
      },
      write: (admission) => {
        if (core.localCatalogRef.current.documents.some((entry) => entry.documentId === admission.document.documentId)) throw new Error("natural-file.document-already-kept");
        return core.writeLocalDocumentRef.current(admission);
      },
      attach: (admission) => core.openDocumentRef.current({ documentId: admission.document.documentId, schema }, [admission.binding], { plugin: owner.handle, session: { pluginId: owner.handle.pluginId, app: owner.app, instanceId, viewState: spawnedProgramViewStateV1(hostSession!.viewState, owner.app) } }),
      record: async (document) => (await core.recordLocalCatalogRef.current(admitLocalDocument(document))) !== null,
      publish: () => publish(current),
      close: async (receipt) => { core.closeDocumentRef.current(receipt.runtimeKey, receipt.clientInstanceId); await core.documentAttachmentLane(owner.handle, instanceId).drain(); },
      unlist: async (document) => { if ((await core.recordLocalCatalogRef.current(retireLocalDocument(document.documentId))) === null) throw new Error("natural-file.catalog-retirement-failed"); },
      report: (error) => console.error("[os-shell] imported document retirement failed", error),
    });
  };
  const keepOrReattachProgramDocument = async (owner: Readonly<{ handle: PluginWasmHandle; app: AppDefinition }>, instanceId: number, name: string, task: Readonly<{ signal: AbortSignal; progress: (completed: number, total: number | null) => void }>, hostOwner: ActiveSession | null, held: () => boolean): Promise<void> => {
    const current = (): boolean => {
      const host = core.sessionRef.current;
      return !task.signal.aborted && held() && hostOwner !== null && host?.pluginId === hostOwner.pluginId && host.app.id === hostOwner.app.id && host.instanceId === hostOwner.instanceId && core.loadedPluginsRef.current.some(entry => entry.handle === owner.handle);
    };
    const identity = await owner.handle.readAppDocumentIdentity(instanceId);
    if (!current()) throw task.signal.reason ?? new Error("natural-file.persistence-owner-unavailable");
    const kept = core.localCatalogRef.current.documents.find(entry => entry.documentId === identity.parent_document_id);
    if (kept === undefined) { await keepProgramDocument(owner, instanceId, name, task, hostOwner, held, () => true); return; }
    if (kept.surface?.pluginId !== owner.handle.pluginId || kept.surface.appId !== owner.app.id || kept.schema !== owner.app.io?.artifactSchema || !owner.handle.readAppDocumentArchive) throw new Error("natural-file.kept-owner-mismatch");
    const stillKept = (): boolean => {
      const live = core.localCatalogRef.current.documents.find(entry => entry.documentId === kept.documentId);
      return current() && live?.schema === kept.schema && live.target === kept.target && live.admittedAtMs === kept.admittedAtMs && live.surface?.pluginId === kept.surface?.pluginId && live.surface?.appId === kept.surface?.appId;
    };
    await core.documentAttachmentLane(owner.handle, instanceId).drain();
    if (!stillKept()) throw task.signal.reason ?? new Error("natural-file.persistence-owner-unavailable");
    const archive = encodeDocumentArchiveBytes(await owner.handle.readAppDocumentArchive(instanceId));
    const after = await owner.handle.readAppDocumentIdentity(instanceId);
    if (!stillKept() || after.parent_document_id !== kept.documentId) throw new Error("natural-file.archive-owner-changed");
    if (!await core.writeLocalDocumentRef.current({ document: kept, archive, binding: localCatalogBindingV1(kept) })) throw new Error("natural-file.archive-not-persisted");
    if (!stillKept()) throw task.signal.reason ?? new Error("natural-file.persistence-owner-unavailable");
    const fresh = encodeDocumentArchiveBytes(await owner.handle.readAppDocumentArchive(instanceId));
    if (!stillKept() || !sameLocalDocumentArchiveV1(fresh, archive)) throw new Error("natural-file.saved-revision-changed");
    const receipt = await core.openDocumentRef.current({ documentId: kept.documentId, schema: kept.schema }, [localCatalogBindingV1(kept)], { plugin: owner.handle, session: { pluginId: owner.handle.pluginId, app: owner.app, instanceId, viewState: spawnedProgramViewStateV1(hostOwner!.viewState, owner.app) } });
    if (receipt === null) throw new Error("natural-file.attachment-refused");
    if (!stillKept()) {
      core.closeDocumentRef.current(receipt.runtimeKey, receipt.clientInstanceId);
      await core.documentAttachmentLane(owner.handle, instanceId).drain();
      throw task.signal.reason ?? new Error("natural-file.persistence-owner-unavailable");
    }
  };
  const persistAttachedProgramDocument = async (owned: { readonly handle: PluginWasmHandle; readonly target: ActiveSession }, signal: AbortSignal) => {
    const entries = [...core.openDocumentSessionsRef.current.entries()].filter(([, entry]) => entry.plugin === owned.handle && entry.session.instanceId === owned.target.instanceId && entry.session.app.id === owned.target.app.id);
      if (entries.length !== 1) throw new Error("natural-file.archive-owner-ambiguous");
      const [runtimeKey, entry] = entries[0]!;
      const archive = await readOwnedDocumentArchiveV1(owned.handle, entry, runtimeKey, core.openDocumentSessionsRef.current);
      if (archive === null) throw new Error("natural-file.archive-owner-retired");
      const expected = encodeDocumentArchiveBytes(archive), worker = core.ensureBackboneWorker();
      const heldArchive = (): boolean => core.openDocumentSessionsRef.current.get(runtimeKey) === entry && core.backboneWorkerRef.current === worker && core.loadedPluginsRef.current.some((candidate) => candidate.handle === owned.handle);
      const readback = await awaitVerifiedReadbackV1<Uint8Array>({
        signal: signal, timeoutMs: LOCAL_CATALOG_WRITE_DEADLINE_MS_V1,
        current: heldArchive,
        accepts: (bytes) => sameLocalDocumentArchiveV1(bytes, expected),
        subscribe: (listener) => {
          if (core.localDocumentOwnersRef.current.has(entry.clientInstanceId)) throw new Error("natural-file.save-owner-busy");
          const observer = { status: listener.status, archive: listener.archive };
          core.localDocumentOwnersRef.current.set(entry.clientInstanceId, observer);
          return () => { if (core.localDocumentOwnersRef.current.get(entry.clientInstanceId) === observer) core.localDocumentOwnersRef.current.delete(entry.clientInstanceId); };
        },
        submit: () => { const wire = encodeBackboneWorkerRequest({ kind: "send", documentId: entry.documentId, clientInstanceId: entry.clientInstanceId, ...(entry.scope === undefined ? {} : { spaceId: entry.scope.spaceId }), message: { kind: "localDocumentArchive", archive: expected } }); worker.postMessage({ wire }, [wire.buffer]); },
        read: async () => {
          const folder = entry.persistenceBindings.find((binding) => binding.kind === "folder" && binding.dataClass === "persistedLocalOnly");
          if (folder === undefined || folder.kind !== "folder") return null;
          const response = await fetch(`${FOLDER_ENDPOINT_PATH}?uri=${encodeURIComponent(`folder://${folder.path}`)}&documentId=${encodeURIComponent(entry.documentId)}`);
          if (!response.ok || response.status === 204) return null;
          return new Uint8Array(await response.arrayBuffer());
        },
        probe: () => worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId: entry.documentId, clientInstanceId: entry.clientInstanceId, ...(entry.scope === undefined ? {} : { spaceId: entry.scope.spaceId }), message: { kind: "externalChanged" } }) }),
      });
      if (readback === null) throw signal.reason ?? new Error("natural-file.archive-not-persisted");
    return { runtimeKey, entry, expected, heldArchive };
  };
  const persistProgramDocument = async (owner: ProgramRetirementOwnerV1, task: Pick<ReturnType<typeof trackDocumentTransfer>, "signal" | "progress">): Promise<void> => {
    const held = (): boolean => {
      const host = core.sessionRef.current;
      return core.loadedPluginsRef.current.some(entry => entry.handle === owner.handle) && host !== null && ((host.pluginId === owner.session.pluginId && host.instanceId === owner.session.instanceId && host.app.id === owner.session.app.id) || parsePanelState(host.viewState)?.spawnedApps.some(entry => entry.pluginId === owner.session.pluginId && entry.appId === owner.session.app.id && entry.instanceId === owner.session.instanceId) === true);
    };
    if (task.signal.aborted || !held()) throw task.signal.reason ?? new Error("program-retirement.owner-stale");
    const entries = [...core.openDocumentSessionsRef.current.values()].filter(entry => entry.plugin === owner.handle && entry.session.instanceId === owner.session.instanceId && entry.session.app.id === owner.session.app.id);
    if (entries.length === 0) {
      const identity = await owner.handle.readAppDocumentIdentity(owner.session.instanceId);
      if (task.signal.aborted || !held()) throw task.signal.reason ?? new Error("program-retirement.owner-stale");
      if (!programRetirementKeepsDocumentV1(core.localDataDir, identity.parent_document_id)) return;
      await keepOrReattachProgramDocument({ handle: owner.handle, app: owner.session.app }, owner.session.instanceId, owner.name, task, core.sessionRef.current, held);
      await core.documentAttachmentLane(owner.handle, owner.session.instanceId).drain();
      if (task.signal.aborted || !held()) throw task.signal.reason ?? new Error("program-retirement.owner-stale");
    } else {
      const { runtimeKey, entry, expected, heldArchive } = await persistAttachedProgramDocument({ handle: owner.handle, target: owner.session }, task.signal);
      const archive = await readOwnedDocumentArchiveV1(owner.handle, entry, runtimeKey, core.openDocumentSessionsRef.current);
      if (task.signal.aborted || !held() || !heldArchive() || archive === null || !sameLocalDocumentArchiveV1(encodeDocumentArchiveBytes(archive), expected)) throw task.signal.reason ?? new Error("program-retirement.saved-revision-changed");
    }
  };
  core.persistProgramDocumentRef.current = async (owner: ProgramRetirementOwnerV1, signal: AbortSignal): Promise<void> => {
    if (!beginDocumentTransferV1(transferInFlightRef)) throw new Error("program-retirement.transfer-busy");
    const task = trackDocumentTransfer(owner.name, owner.handle.pluginId), cancel = (): void => task.cancel();
    signal.addEventListener("abort", cancel, { once: true });
    if (signal.aborted) cancel();
    try {
      await persistProgramDocument(owner, task);
    } finally {
      signal.removeEventListener("abort", cancel);
      task.finish();
      transferInFlightRef.current = false;
    }
  };
  /** 📤️ Downloads one declared output from the exact app and saved revision captured by the input funnel. */
  const exportMediaFile = async (target: ActiveSession, current: () => boolean, actionId: MediaFileExportActionIdV1): Promise<void> => {
    const declaration = declaredMediaFileExportV1(target.app, actionId);
    const handle = core.loadedPluginsRef.current.find(entry => entry.handle.pluginId === target.pluginId)?.handle;
    const file = declaration?.filename ?? "";
    if (declaration === null || handle === undefined) throw new Error("media-export.declaration-unavailable");
    if (!beginDocumentTransferV1(transferInFlightRef)) return notifyDocumentTransfer("transfer-busy", file, "info");
    const task = trackDocumentTransfer(file, target.pluginId);
    const held = (): boolean => current() && core.loadedPluginsRef.current.some(entry => entry.handle === handle) && !core.pluginOpInFlightRef.current.has(handle.pluginId);
    try {
      if (await downloadDeclaredMediaFileV1({ instanceId: target.instanceId, declaration, port: handle, signal: task.signal, current: held, progress: task.progress, sinkFactory: shellSegmentedDownloadSinkFactory })) notifyDocumentTransfer("exported", file, "info");
    } catch (error) {
      if (!task.signal.aborted && held()) {
        notifyDocumentTransfer("export-failed", file, "error");
        throw error;
      }
    } finally {
      task.finish();
      transferInFlightRef.current = false;
    }
  };
  const saveNaturalFile = async (): Promise<void> => {
    const owned = documentTransferTarget();
    if (owned === null) return notifyDocumentTransfer("no-document", "", "info");
    const format = naturalFileFormatV1(owned.target.app);
    const exportMedia = owned.handle.exportAppMedia;
    const keptName = core.localCatalogRef.current.documents.find((document) => [...core.openDocumentSessionsRef.current.values()].some((entry) => entry.plugin === owned.handle && entry.session.instanceId === owned.target.instanceId && entry.session.app.id === owned.target.app.id && entry.documentId === document.documentId))?.name;
    const file = format === null ? "" : naturalFileSaveNameV1(keptName, owned.target.app.id, format.extension, new Date());
    const declaration = format === null ? null : declaredNaturalMediaFileExportV1(owned.target.app, format, file);
    if (!beginDocumentTransferV1(transferInFlightRef)) return notifyDocumentTransfer("transfer-busy", file, "info");
    const task = trackDocumentTransfer(file, owned.target.pluginId);
    try {
      if (format === null || (declaration === null && exportMedia === undefined)) throw new Error("natural-file.exporter-unavailable");
      if (task.signal.aborted) throw task.signal.reason;
      task.progress(0, 1);
      const sessions = () => [...core.openDocumentSessionsRef.current.entries()].filter(([, entry]) => entry.plugin === owned.handle && entry.session.instanceId === owned.target.instanceId && entry.session.app.id === owned.target.app.id);
      if (sessions().length === 0) {
        const hostOwner = core.sessionRef.current;
        const held = (): boolean => {
          const host = core.sessionRef.current;
          if (host?.pluginId === owned.target.pluginId && host.instanceId === owned.target.instanceId && host.app.id === owned.target.app.id) return true;
          return host !== null && parsePanelState(host.viewState)?.spawnedApps.some((entry) => entry.pluginId === owned.target.pluginId && entry.appId === owned.target.app.id && entry.instanceId === owned.target.instanceId) === true;
        };
        await keepOrReattachProgramDocument({ handle: owned.handle, app: owned.target.app }, owned.target.instanceId, file, task, hostOwner, held);
      }
      await core.documentAttachmentLane(owned.handle, owned.target.instanceId).drain();
      if (task.signal.aborted) throw task.signal.reason;
      const { runtimeKey, entry, expected, heldArchive } = await persistAttachedProgramDocument(owned, task.signal);
      const verifyDelivery = async (): Promise<void> => {
        if (!heldArchive()) throw new Error("natural-file.export-owner-retired");
        const exportedArchive = await readOwnedDocumentArchiveV1(owned.handle, entry, runtimeKey, core.openDocumentSessionsRef.current);
        if (task.signal.aborted || !heldArchive() || exportedArchive === null || !sameLocalDocumentArchiveV1(encodeDocumentArchiveBytes(exportedArchive), expected)) throw task.signal.reason ?? new Error("natural-file.saved-revision-changed");
      };
      await deliverNaturalMediaFileV1(declaration === null ? null : () =>
        downloadDeclaredMediaFileV1({ instanceId: owned.target.instanceId, declaration, port: owned.handle, signal: task.signal, current: heldArchive, progress: task.progress, sinkFactory: shellSegmentedDownloadSinkFactory, beforeDelivery: verifyDelivery }),
      verifyDelivery, async () => {
        if (exportMedia === undefined) throw new Error("natural-file.exporter-unavailable");
        const artifact = await exportMedia(owned.target.instanceId, NATURAL_FILE_PORT_ID);
        if (artifact === null || task.signal.aborted) throw task.signal.reason ?? new Error("natural-file.export-empty");
        await verifyDelivery();
        if (!naturalMediaDescriptorMatchesV1(decodePackValue(artifact.descriptor), format)) throw new Error("natural-file.export-descriptor-mismatch");
        downloadMediaExportBytes(file, format.mediaType, artifact.data);
      });
      task.progress(1, 1);
      notifyDocumentTransfer("file-saved", file, "info");
    } catch (error) {
      console.error(`[os-shell] ${documentTransferNoticeCodeV1("file-save-failed")} ${owned.target.pluginId}#${owned.target.instanceId}`, error);
      notifyDocumentTransfer(task.signal.aborted ? "file-save-cancelled" : "file-save-failed", file, task.signal.aborted ? "info" : "error");
    } finally {
      task.finish();
      transferInFlightRef.current = false;
    }
  };
  /** 📂️ Reopens the kept archive and exact selected surface under its persisted document identity. */
  const openKeptFile = async (document: LocalDocument, signal: AbortSignal): Promise<void> => {
    if (!beginDocumentTransferV1(transferInFlightRef)) return notifyDocumentTransfer("transfer-busy", document.name, "info");
    const task = trackDocumentTransfer(document.name, document.surface?.pluginId ?? "open-file");
    const cancel = (): void => task.cancel();
    signal.addEventListener("abort", cancel, { once: true });
    if (signal.aborted) task.cancel();
    const hostOwner = core.sessionRef.current;
    let handle: PluginWasmHandle | null = null, instanceId: number | null = null;
    let receipt: Readonly<{ runtimeKey: string; clientInstanceId: string }> | null = null;
    let published = false;
    const surface = document.surface;
    const keptOwnerHeld = (): boolean => {
      const host = core.sessionRef.current;
      const kept = core.localCatalogRef.current.documents.find((entry) => entry.documentId === document.documentId);
      return surface !== undefined && hostOwner !== null && host?.pluginId === hostOwner.pluginId && host.app.id === hostOwner.app.id && host.instanceId === hostOwner.instanceId && kept?.schema === document.schema && kept.target === document.target && kept.admittedAtMs === document.admittedAtMs && kept.surface?.pluginId === surface.pluginId && kept.surface?.appId === surface.appId && (handle === null || core.loadedPluginsRef.current.some((entry) => entry.handle === handle));
    };
    const checkpoint = (completed: number): void => {
      task.signal.throwIfAborted();
      if (surface !== undefined && core.pluginOpInFlightRef.current.has(surface.pluginId)) throw new Error("natural-file.program-retiring");
      if (!keptOwnerHeld()) throw new Error("natural-file.kept-owner-retired");
      task.progress(completed, 4);
    };
    try {
      checkpoint(0);
      const role = parseSurfaceAppId(surface!.appId).role;
      const opening = await core.resolveArtifactOpeningWithActivationRef.current("os.open-artifact", { artifactRef: surface!.appId, role, pluginId: surface!.pluginId, appId: surface!.appId });
      checkpoint(0);
      const owner = core.loadedPluginsRef.current.find((entry) => entry.handle.pluginId === surface!.pluginId);
      const app = owner?.manifest.apps.find((entry) => entry.id === surface!.appId);
      if (core.pluginOpInFlightRef.current.has(surface!.pluginId)) throw new Error("natural-file.program-retiring");
      if (opening?.app.pluginId !== surface!.pluginId || opening.app.appId !== surface!.appId || owner === undefined || app?.io?.artifactSchema !== document.schema) throw new Error("natural-file.kept-surface-unavailable");
      handle = owner.handle;
      const existing = core.openDocumentSessionsRef.current.get(document.documentId);
      if (existing !== undefined) {
        if (existing.plugin !== handle || existing.session.app.id !== app.id || !await publishOpenedProgram({ handle, app, instanceId: existing.session.instanceId, label: document.name, breadcrumb: app.breadcrumb }, task, keptOwnerHeld)) throw new Error("natural-file.kept-document-owner-mismatch");
        published = true;
        return notifyDocumentTransfer("file-opened", document.name, "info");
      }
      const bytes = await core.readLocalDocumentRef.current(document);
      checkpoint(1);
      if (bytes === null || !handle.loadAppDocumentArchive) throw new Error("natural-file.kept-archive-unavailable");
      const archive = decodeDocumentArchiveBytes(bytes);
      instanceId = await handle.createApp(app.id);
      checkpoint(1);
      await handle.loadAppDocumentArchive(instanceId, archive, task.signal, (progress) => task.progress(progress.completed, progress.total));
      checkpoint(2);
      const identity = await handle.readAppDocumentIdentity(instanceId);
      checkpoint(2);
      if (identity.parent_document_id !== document.documentId) throw new Error("natural-file.kept-archive-identity-mismatch");
      receipt = await core.openDocumentRef.current({ documentId: document.documentId, schema: document.schema }, [localCatalogBindingV1(document)], { plugin: handle, session: { pluginId: surface!.pluginId, app, instanceId, viewState: spawnedProgramViewStateV1(hostOwner!.viewState, app) } });
      checkpoint(3);
      if (receipt === null || !await publishOpenedProgram({ handle, app, instanceId, label: document.name, breadcrumb: app.breadcrumb }, task, keptOwnerHeld)) throw new Error("natural-file.kept-open-refused");
      published = true;
      task.progress(4, 4);
      notifyDocumentTransfer("file-opened", document.name, "info");
    } catch (error) {
      if (receipt !== null) core.closeDocumentRef.current(receipt.runtimeKey, receipt.clientInstanceId);
      if (instanceId !== null && handle !== null && !published) {
        await core.documentAttachmentLane(handle, instanceId).drain().catch((failure) => console.error("[os-shell] kept document attachment retirement failed", failure));
        await handle.destroyApp(instanceId).catch((failure) => console.error("[os-shell] kept document instance retirement failed", failure));
      }
      console.warn("[os-shell] kept file opening failed", error);
      notifyDocumentTransfer(task.signal.aborted ? "open-cancelled" : "file-open-failed", document.name, task.signal.aborted ? "info" : "error");
    } finally {
      signal.removeEventListener("abort", cancel);
      task.finish();
      transferInFlightRef.current = false;
    }
  };
  /** 📂️ Opens an imported natural file through durable storage before publishing its editor. */
  const openNaturalFile = (dropped?: File): Promise<void> => {
    const catalog = core.hostPorts?.formatCatalog;
    const acceptedExtensions = catalog === undefined ? "" : catalog.descriptors().flatMap((descriptor) => descriptor.extensions).join(",");
    if (session === null) return Promise.resolve(notifyDocumentTransfer("no-document", "", "info"));
    const hostSession = core.sessionRef.current;
    let pickedName = "";
    return openPickedFileV1<{ readonly handle: PluginWasmHandle; readonly app: AppDefinition }, File>({
      select: async () => dropped ?? (await requestFileSelectionV1(acceptedExtensions))[0],
      claim: () => beginDocumentTransferV1(transferInFlightRef),
      release: () => { transferInFlightRef.current = false; },
      track: (name) => trackDocumentTransfer(name, "open-file"),
      route: (picked, signal) => routePickedFileV1(catalog, picked, (bytes) => readPickedFileHeadV1(picked, bytes, signal), { source: { size: picked.size, read: (start, end) => readPickedFileRangeV1(picked, start, end, signal) }, signal }),
      resolveOwner: async (route) => {
        const opening = await core.resolveArtifactOpeningWithActivationRef.current("os.open-artifact", { artifactRef: route.artifactRef, role: "editor" });
        const owner = opening === null ? undefined : core.loadedPluginsRef.current.find((entry) => entry.handle.pluginId === opening.app.pluginId);
        const app = opening === null ? undefined : owner?.manifest.apps.find((candidate) => candidate.id === opening.app.appId);
        return owner === undefined || app === undefined || core.pluginOpInFlightRef.current.has(owner.handle.pluginId) ? null : { handle: owner.handle, app };
      },
      importInto: async (owner, picked, task) => {
        pickedName = picked.name;
        const format = naturalFileFormatV1(owner.app);
        const importMedia = owner.handle.importAppMedia;
        if (format === null || importMedia === undefined) throw new Error("natural-file.importer-unavailable");
        task.signal.throwIfAborted();
        const input = naturalFileMediaInputV1(format.formatKind);
        const bytes = await readBlobBytesBoundedV1(picked, {
          signal: task.signal, maximumBytes: input.maximumBytes, chunkBytes: NATURAL_FILE_BROWSER_READ_CHUNK_BYTES_V1, progress: task.progress,
        });
        return openNaturalFileOwnerV1({
          signal: task.signal,
          create: () => owner.handle.createApp(owner.app.id),
          importBytes: (freshInstanceId) => importMedia(freshInstanceId, input.port, input.descriptor, bytes, task.signal),
          retire: (freshInstanceId) => owner.handle.destroyApp(freshInstanceId),
        });
      },
      commit: async (owner, instanceId, task) => {
        if (core.pluginOpInFlightRef.current.has(owner.handle.pluginId)) throw new Error("natural-file.program-retiring");
        const program = spacePrograms.find((entry) => entry.pluginId === owner.handle.pluginId && entry.appId === owner.app.id);
        return keepProgramDocument(owner, instanceId, pickedName, task, hostSession, () => !core.pluginOpInFlightRef.current.has(owner.handle.pluginId), (current) => publishOpenedProgram({ handle: owner.handle, app: owner.app, instanceId, label: program?.label, breadcrumb: program?.breadcrumb }, task, current));
      },
      destroy: (owner, instanceId) => owner.handle.destroyApp(instanceId),
      report: (message, error) => console.warn(`[os-shell] ${documentTransferNoticeCodeV1("file-open-failed")} ${message}`, error),
      notify: (outcome) => {
        if (outcome.kind === "refused") return showTransientNoticeRef.current(pickedFileRefusalTextV1(outcome.reason, outcome.name, uiLocaleRef.current), "warning", pickedFileRefusalCodeV1(outcome.reason));
        const notice: DocumentTransferNoticeV1 = outcome.kind === "opened" ? "file-opened" : outcome.kind === "cancelled" ? "open-cancelled" : outcome.kind === "busy" ? "transfer-busy" : "file-open-failed";
        notifyDocumentTransfer(notice, outcome.name, outcome.kind === "failed" ? "error" : "info");
      },
    });
  };
  return { openKeptFile, exportDocumentArchive, importDocumentArchive, exportMediaFile, saveNaturalFile, openNaturalFile, trackDocumentTransfer, notifyDocumentTransfer, dispatch, agentBridge, session, focusedApp, focusedWindowInstancesRef, activeWindowIdRef, focusedSpawnedId, effectiveModeLayoutRef, uiKeybindingOverrides, activeUtilityByWindowIdRef, onAction, utilityGenerationFor, activeToolIdRef, toolGeneration, appLabelsOverlay, uiTerminology, uiLocale, showTransientNotice, clipboardFragmentRef, clipboardQueue, actionPaneExpandedByWindowIdRef, actionPaneStagedArgsByKeyRef, scope, openDocumentSessionsRef, localDocumentOwnerRegistry, panel, foldSpaceDirectoryRef, browserActorUiByRuntimeKeyRef, browserActorUiVersion, onBrowserActorIntent, onActionStable, focusedPanelBodyStores, treeWindowHost, panelTreeConfigCacheRef, hostMode, hostAppId, openWithFocusRole, openArtifactWithAppRef, dispatchClearDefaultApp, dispatchSetDefaultApp, openWithEntries, treeWindowGeneration, frameworkSettingsTab };
}
