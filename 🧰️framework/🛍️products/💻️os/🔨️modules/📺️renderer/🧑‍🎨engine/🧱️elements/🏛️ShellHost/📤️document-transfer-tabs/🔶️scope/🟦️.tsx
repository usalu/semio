/** 🔶️ The scope phase of useDocumentTransferAndTabs. */
import { type AppDefinition, type Severity } from "@semio-tech/framework";
import { decodePackValue } from "@semio-tech/framework/packs";
import { decodeDocumentArchiveBytes, encodeDocumentArchiveBytes, type DocumentArchivePack } from "@semio-tech/framework/wire-contracts";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { DOCUMENT_ARCHIVE_FILE_EXTENSION, DOCUMENT_ARCHIVE_MEDIA_TYPE, documentArchiveFileNameV1, beginDocumentTransferV1, naturalFileNameV1, naturalFileFormatV1, naturalMediaDescriptorMatchesV1, openNaturalFileOwnerV1, bytesOfDataUrlV1, documentTransferNoticeTextV1, documentTransferNoticeCodeV1, type DocumentTransferNoticeV1 } from "../../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { NATURAL_FILE_BROWSER_MAXIMUM_BYTES_V1, NATURAL_FILE_BROWSER_READ_CHUNK_BYTES_V1, readBlobBytesBoundedV1, requestFileSelectionV1, downloadMediaExportBytes, requestFileOpen } from "../../../🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import { buildSpacePanelState, parsePanelState, studioPanelFocusingSpawned } from "../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { type PluginWasmHandle } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { documentTransferTasksV1, type TaskManagerDocumentTransferV1 } from "../../../🧵️TaskManager/🟦️.tsx";
import { openPickedFileV1, pickedFileRefusalCodeV1, pickedFileRefusalTextV1, readPickedFileHeadV1, routePickedFileV1 } from "../../📂️picked-file-opening/🟦️.ts";
import { requiredHostPanelLeafId } from "../../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterThemeDriverSettings } from "../../🎨️theme-driver-settings/🟦️.tsx";

/** 🔷 Builds scope, localDocumentOwnerRegistry, hostMode, dispatch of useDocumentTransferAndTabs. */
export function buildScopeState({ core }: { readonly core: ShellCoreAfterThemeDriverSettings }) {
  const {
    scope, localDocumentOwnerRegistry, hostMode, dispatch, loadedPlugins, session, uiLocale, uiTerminology,
    uiKeybindingOverrides, hostApp, landingAppId, hostAppId, spacePrograms, appLabelsOverlay, openWithFocusRole,
    clipboardFragmentRef, panelTreeConfigCacheRef, treeWindowGeneration, treeWindowHost, agentBridge, uiLocaleRef,
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
  /** 🪟️ Focuses a freshly created program instance in the space panel, built from the latest panel at commit time; `false` when no session is mounted. */
  const focusSpawnedProgram = (spawned: { readonly pluginId: string; readonly appId: string; readonly instanceId: number; readonly label: string | undefined; readonly breadcrumb: readonly string[] | undefined }): boolean =>
    updateSpacePanelFrom((latest) => studioPanelFocusingSpawned(latest ?? buildSpacePanelState([], requiredHostPanelLeafId(hostApp)), {
      id: `${spawned.pluginId}-${spawned.instanceId}`,
      pluginId: spawned.pluginId,
      instanceId: spawned.instanceId,
      appId: spawned.appId,
      label: spawned.label ?? spawned.appId,
      breadcrumb: spawned.breadcrumb ?? [],
    }));
  const publishDocumentTransfers = (): void => {
    documentTransferTasksRef.current = documentTransferTasksV1([...documentTransfersRef.current.values()].map((entry) => entry.transfer));
    for (const listener of [...taskManagerListenersRef.current]) listener();
  };
  const notifyDocumentTransfer = (notice: DocumentTransferNoticeV1, file: string, kind: Severity): void => {
    showTransientNoticeRef.current(documentTransferNoticeTextV1(notice, file, uiLocaleRef.current), kind, documentTransferNoticeCodeV1(notice));
  };
  /** 🧵️ One file moving into a program as a cancellable Tasks-window task (`documentTransfer` lane): `progress` states
   * `completed` of `total` units (`null` = open-ended), Cancel aborts `signal`, `finish` removes the row. */
  const trackDocumentTransfer = (file: string, pluginId: string): { readonly signal: AbortSignal; readonly progress: (completed: number, total: number | null) => void; readonly finish: () => void } => {
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
    try {
      if (!owned.handle.readAppDocumentArchive) throw new Error("document-backbone.archive-reader-unavailable");
      downloadMediaExportBytes(file, DOCUMENT_ARCHIVE_MEDIA_TYPE, encodeDocumentArchiveBytes(await owned.handle.readAppDocumentArchive(owned.target.instanceId)));
      notifyDocumentTransfer("exported", file, "info");
    } catch (error) {
      console.error(`[os-shell] ${documentTransferNoticeCodeV1("export-failed")} ${owned.target.pluginId}#${owned.target.instanceId}`, error);
      notifyDocumentTransfer("export-failed", file, "error");
    }
  };
  /** 📥️ Import Document: a picked archive opens as a NEW document of the focused program, loaded with progress and
   * cancellation from the Tasks window; the focused document stays untouched. */
  const importDocumentArchive = async (): Promise<void> => {
    const owned = documentTransferTarget();
    if (owned === null || session === null) return notifyDocumentTransfer("no-document", "", "info");
    let picked: Awaited<ReturnType<typeof requestFileOpen>>[number] | undefined;
    try {
      [picked] = await requestFileOpen(`${DOCUMENT_ARCHIVE_FILE_EXTENSION},${DOCUMENT_ARCHIVE_MEDIA_TYPE}`, "dataUrl", false);
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
  const importDocumentArchiveOnce = async (owned: NonNullable<ReturnType<typeof documentTransferTarget>>, _session: ActiveSession, picked: { readonly name: string; readonly contents: string }): Promise<void> => {
    const load = owned.handle.loadAppDocumentArchive;
    let archive: DocumentArchivePack;
    try {
      archive = decodeDocumentArchiveBytes(bytesOfDataUrlV1(picked.contents));
    } catch (error) {
      console.warn(`[os-shell] ${documentTransferNoticeCodeV1("import-unreadable")} ${picked.name}`, error);
      return notifyDocumentTransfer("import-unreadable", picked.name, "warning");
    }
    const task = trackDocumentTransfer(picked.name, owned.target.pluginId);
    let instanceId: number | null = null;
    try {
      if (!load) throw new Error("document-backbone.archive-loader-unavailable");
      instanceId = await owned.handle.createApp(owned.target.app.id);
      await load(instanceId, archive, task.signal, (status) => task.progress(status.completed, status.total > 0 ? status.total : null));
      const program = spacePrograms.find((entry) => entry.pluginId === owned.target.pluginId && entry.appId === owned.target.app.id);
      if (!focusSpawnedProgram({ pluginId: owned.target.pluginId, appId: owned.target.app.id, instanceId, label: program?.label, breadcrumb: program?.breadcrumb })) throw new Error("import.panel-unavailable");
      notifyDocumentTransfer("imported", picked.name, "info");
    } catch (error) {
      if (instanceId !== null) await owned.handle.destroyApp(instanceId).catch((destroyError: unknown) => console.error(`[os-shell] imported instance ${instanceId} could not be closed`, destroyError));
      const notice: DocumentTransferNoticeV1 = task.signal.aborted ? "import-cancelled" : "import-failed";
      console.warn(`[os-shell] ${documentTransferNoticeCodeV1(notice)} ${picked.name}`, error);
      notifyDocumentTransfer(notice, picked.name, task.signal.aborted ? "info" : "error");
    } finally {
      task.finish();
    }
  };
  const saveNaturalFile = async (): Promise<void> => {
    const owned = documentTransferTarget();
    if (owned === null) return notifyDocumentTransfer("no-document", "", "info");
    const format = naturalFileFormatV1(owned.target.app);
    const exportMedia = owned.handle.exportAppMedia;
    const file = format === null ? "" : naturalFileNameV1(owned.target.app.id, format.extension, new Date());
    const task = trackDocumentTransfer(file, owned.target.pluginId);
    try {
      if (format === null || exportMedia === undefined) throw new Error("natural-file.exporter-unavailable");
      if (task.signal.aborted) throw task.signal.reason;
      task.progress(0, 1);
      const artifact = await exportMedia(owned.target.instanceId, "artifact:native");
      if (artifact === null || task.signal.aborted) throw task.signal.reason ?? new Error("natural-file.export-empty");
      if (!naturalMediaDescriptorMatchesV1(decodePackValue(artifact.descriptor), format)) throw new Error("natural-file.export-descriptor-mismatch");
      downloadMediaExportBytes(file, format.mediaType, artifact.data);
      task.progress(1, 1);
      notifyDocumentTransfer("file-saved", file, "info");
    } catch (error) {
      console.error(`[os-shell] ${documentTransferNoticeCodeV1("file-save-failed")} ${owned.target.pluginId}#${owned.target.instanceId}`, error);
      notifyDocumentTransfer("file-save-failed", file, "error");
    } finally {
      task.finish();
    }
  };
  /** 📂️ Open file: the picked file is matched to its format through the shared resolution law, routed to the owner editor by the `OpeningResolver`, and imported as a new document; no editor has to be chosen first. The whole gesture runs through `openPickedFileV1`, so every failure is told and every created instance is released. */
  const openNaturalFile = (): Promise<void> => {
    const catalog = core.hostPorts?.formatCatalog;
    const acceptedExtensions = catalog === undefined ? "" : catalog.descriptors().flatMap((descriptor) => descriptor.extensions).join(",");
    if (session === null) return Promise.resolve(notifyDocumentTransfer("no-document", "", "info"));
    return openPickedFileV1<{ readonly handle: PluginWasmHandle; readonly app: AppDefinition }, File>({
      select: async () => (await requestFileSelectionV1(acceptedExtensions))[0],
      claim: () => beginDocumentTransferV1(transferInFlightRef),
      release: () => { transferInFlightRef.current = false; },
      track: (name) => trackDocumentTransfer(name, "open-file"),
      route: (picked, signal) => routePickedFileV1(catalog, picked, (bytes) => readPickedFileHeadV1(picked, bytes, signal)),
      resolveOwner: async (route) => {
        const opening = await core.resolveArtifactOpeningWithActivationRef.current("os.open-artifact", { artifactRef: route.artifactRef, role: "editor" });
        const owner = opening === null ? undefined : core.loadedPluginsRef.current.find((entry) => entry.handle.pluginId === opening.app.pluginId);
        const app = opening === null ? undefined : owner?.manifest.apps.find((candidate) => candidate.id === opening.app.appId);
        return owner === undefined || app === undefined ? null : { handle: owner.handle, app };
      },
      importInto: async (owner, picked, task) => {
        const format = naturalFileFormatV1(owner.app);
        const importMedia = owner.handle.importAppMedia;
        if (format === null || importMedia === undefined) throw new Error("natural-file.importer-unavailable");
        if (task.signal.aborted) throw task.signal.reason;
        const bytes = await readBlobBytesBoundedV1(picked, { signal: task.signal, maximumBytes: NATURAL_FILE_BROWSER_MAXIMUM_BYTES_V1, chunkBytes: NATURAL_FILE_BROWSER_READ_CHUNK_BYTES_V1, progress: task.progress });
        return openNaturalFileOwnerV1({
          signal: task.signal,
          create: () => owner.handle.createApp(owner.app.id),
          importBytes: (freshInstanceId) => importMedia(freshInstanceId, "artifact:native", { portId: "artifact:native", kindId: format.formatKind, wire: { kind: "binary", format_kind: format.formatKind } }, bytes),
          retire: (freshInstanceId) => owner.handle.destroyApp(freshInstanceId),
        });
      },
      commit: (owner, instanceId) => {
        const program = spacePrograms.find((entry) => entry.pluginId === owner.handle.pluginId && entry.appId === owner.app.id);
        return focusSpawnedProgram({ pluginId: owner.handle.pluginId, appId: owner.app.id, instanceId, label: program?.label, breadcrumb: program?.breadcrumb });
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
  return { exportDocumentArchive, importDocumentArchive, saveNaturalFile, openNaturalFile, trackDocumentTransfer, notifyDocumentTransfer, dispatch, agentBridge, session, focusedApp, focusedWindowInstancesRef, activeWindowIdRef, focusedSpawnedId, effectiveModeLayoutRef, uiKeybindingOverrides, activeUtilityByWindowIdRef, onAction, utilityGenerationFor, activeToolIdRef, toolGeneration, appLabelsOverlay, uiTerminology, uiLocale, showTransientNotice, clipboardFragmentRef, actionPaneExpandedByWindowIdRef, actionPaneStagedArgsByKeyRef, scope, openDocumentSessionsRef, localDocumentOwnerRegistry, panel, foldSpaceDirectoryRef, browserActorUiByRuntimeKeyRef, browserActorUiVersion, onBrowserActorIntent, onActionStable, focusedPanelBodyStores, treeWindowHost, panelTreeConfigCacheRef, hostMode, hostAppId, openWithFocusRole, openArtifactWithAppRef, dispatchClearDefaultApp, dispatchSetDefaultApp, openWithEntries, treeWindowGeneration, frameworkSettingsTab };
}
