/** 💾️ Actual scope integration with controlled persistence ports. */
import { afterEach, expect, it, vi } from "vitest";
import Ajv from "ajv";
import { decodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { EXPORT_MEDIA_FILE_ACTION_ID } from "@semio-tech/framework";
import { encodePackValue } from "@semio-tech/framework/packs";
import { DOCUMENT_ARCHIVE_MAXIMUM_BYTES, encodeDocumentArchiveBytes } from "@semio-tech/framework/wire-contracts";
import { NATURAL_FILE_BROWSER_READ_CHUNK_BYTES_V1 } from "../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import { buildScopeState } from "../../🧱️elements/🏛️ShellHost/📤️document-transfer-tabs/🔶️scope/🟦️.tsx";
import fixture from "./🔣️.json";
import opened from "../../🧱️elements/🏛️ShellHost/📂️opened-program/🔣️.json";

const selection = vi.hoisted(() => ({ files: [] as File[], downloads: [] as string[], segmented: [] as number[][], aborted: false }));
vi.mock("../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts", async importOriginal => ({ ...await importOriginal<object>(), downloadMediaExportBytes: (file: string) => { selection.downloads.push(file); }, shellSegmentedDownloadSinkFactory: async (file: string) => ({ write: async (bytes: Uint8Array) => { selection.segmented.push([...bytes]); }, close: async () => { selection.downloads.push(file); }, abort: async () => { selection.aborted = true; } }), requestFileSelectionV1: async () => selection.files }));
vi.mock("../../🧱️elements/🏛️ShellHost/📂️picked-file-opening/🟦️.ts", async importOriginal => ({ ...await importOriginal<object>(), routePickedFileV1: async () => ({ kind: "routed", artifactRef: "s.stdio.txt@utf-8/*" }) }));
vi.mock("../../🧱️elements/🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts", async importOriginal => ({ ...await importOriginal<object>(), naturalFileFormatV1: (app: { actions: unknown[] }) => app.actions.length ? ({ formatKind: "s.stdio.pdf@1.7", extension: ".pdf", mediaType: "application/pdf", binary: true }) : ({ formatKind: "txt", extension: ".txt", mediaType: "text/plain", binary: false }) }));
vi.mock("../../🧱️elements/🛠️ShellHelpers/📌️panel/🟦️.ts", async importOriginal => ({ ...await importOriginal<object>(), studioPanelFocusingSpawned: (_latest: unknown, spawned: unknown) => spawned }));
afterEach(() => vi.restoreAllMocks());
const deferred = <T,>() => { let resolve!: (value: T) => void; const promise = new Promise<T>(done => { resolve = done; }); return { promise, resolve }; };
const archive = { parent_pack: new Uint8Array([1, 2]), parent_spr: new Uint8Array([3]), members: [] };
const document = { documentId: "01900000-0000-7000-8000-000000000001", schema: "stdio.txt", name: "Chosen.txt", storage: "folder" as const, target: "/data/kept", admittedAtMs: 42, surface: { pluginId: "stdio-text", appId: "s.stdio.txt@utf-8/*#editor" } };
function setup(segmented = false, hostMode = true) {
  const calls: string[] = [];
  const media = { outputPort: "artifact:native", filename: "document.pdf", mediaType: "application/pdf", formatKind: "s.stdio.pdf@1.7" };
  const actions = segmented ? [{ id: EXPORT_MEDIA_FILE_ACTION_ID, args: Object.entries(media).map(([id, value]) => ({ id, default: value })) }] : [];
  const ports = segmented ? [{ id: media.outputPort, direction: "out", kindId: media.formatKind, mediaType: { class: "data", form: "document" } }] : [];
  const app = { id: document.surface.appId, io: { artifactSchema: document.schema, ports }, actions, windowKinds: [], modes: [], breadcrumb: [] };
  const host = { pluginId: "host", app: { id: "host" }, instanceId: 1, viewState: {} };
  const handle = {
    pluginId: document.surface.pluginId,
    createApp: vi.fn(async (..._args: unknown[]) => { calls.push("create"); return 7; }),
    importAppMedia: vi.fn(async (..._args: unknown[]) => { calls.push("import"); }),
    readAppDocumentIdentity: vi.fn(async () => ({ parent_document_id: document.documentId })),
    readAppDocumentArchive: vi.fn(async () => archive),
    loadAppDocumentArchive: vi.fn(async (..._args: unknown[]) => { calls.push("load"); }),
    destroyApp: vi.fn(async () => { calls.push("destroy"); }),
    exportAppMedia: vi.fn(async (..._args: unknown[]) => { calls.push("export"); return { descriptor: encodePackValue({ portId: "artifact:native", kindId: "txt", wire: { kind: "binary", format_kind: "txt" } }), data: new Uint8Array([65]) }; }),
  };
  const target = { pluginId: handle.pluginId, app, instanceId: 7, viewState: {} };
  const worker = { postMessage: vi.fn((_message: unknown) => {}) };
  const core = {
    pluginOpInFlightRef: { current: new Set<string>() },
    persistProgramDocumentRef: { current: async (..._args: unknown[]) => {} },
    historyStore: { get: () => ({}) as Record<string, unknown> }, identityRef: { current: null },
    onAction: vi.fn(async (action: { action: string }) => { calls.push(action.action); return { kind: "applied" }; }),
    hostMode, hostAppId: "host", landingAppId: "landing", resolveSyncTargetSession: () => target,
    adoptPreparedPrimarySession: vi.fn(async (appId: string, prepared: { instanceId: number; current: () => boolean; onPublished: () => void }) => {
      calls.push("adopt");
      if (!prepared.current()) return;
      core.sessionRef.current = { pluginId: handle.pluginId, app: { id: appId }, instanceId: prepared.instanceId, viewState: {} };
      prepared.onPublished();
    }),
    ensureBackboneWorker: () => worker, backboneWorkerRef: { current: worker }, localDocumentOwnersRef: { current: new Map() },
    session: host, sessionRef: { current: host }, uiLocaleRef: { current: "en" }, uiLocale: "en",
    loadedPluginsRef: { current: [{ handle, manifest: { apps: [app] } }] }, loadedPlugins: [{ handle }],
    hostPorts: { formatCatalog: { descriptors: () => [] } }, spacePrograms: [],
    resolveArtifactOpeningWithActivationRef: { current: vi.fn(async () => ({ app: { pluginId: handle.pluginId, appId: app.id } })) },
    localCatalogRef: { current: { documents: [] as typeof document[] } }, localDataDir: "/data",
    writeLocalDocumentRef: { current: vi.fn(async (..._args: unknown[]) => { calls.push("write"); return true; }) },
    readLocalDocumentRef: { current: vi.fn(async () => encodeDocumentArchiveBytes(archive)) },
    openDocumentRef: { current: vi.fn(async (..._args: unknown[]) => { calls.push("attach"); return { runtimeKey: "runtime", clientInstanceId: "client" }; }) },
    closeDocumentRef: { current: vi.fn(() => { calls.push("close"); }) },
    recordLocalCatalogRef: { current: vi.fn(async () => { calls.push("catalog"); return { documents: [document] }; }) },
    documentAttachmentLane: vi.fn(() => ({ drain: async () => { calls.push("drain"); } })),
    openDocumentSessionsRef: { current: new Map() },
    updateSpacePanelFrom: vi.fn((update: (value: object) => unknown) => { calls.push("focus"); update({}); return true; }),
    documentTransferFlightRef: { current: false }, documentTransfersRef: { current: new Map() }, documentTransferTasksRef: { current: [] }, taskManagerListenersRef: { current: new Set() },
    showTransientNoticeRef: { current: vi.fn((_text: string, _kind: string, code: string) => { calls.push(code); }) },
  };
  selection.files = [new File(["hi"], document.name)]; selection.downloads = []; selection.segmented = []; selection.aborted = false;
  return { core, handle, calls, worker, target, state: buildScopeState({ core } as unknown as Parameters<typeof buildScopeState>[0]) };
}
it("validates neutral lifecycle cases using Ajv", () => { expect(new Ajv().validate(fixture.schema, fixture.cases)).toBe(true); });
it("keeps selected metadata and waits for archive, attachment and durable catalog before focus", async () => {
  const view = setup(), ack = deferred<{ documents: typeof document[] }>();
  view.core.recordLocalCatalogRef.current.mockImplementation(async () => { view.calls.push("catalog"); return ack.promise; });
  const pending = view.state.openNaturalFile();
  await vi.waitFor(() => expect(view.calls).toContain("catalog"));
  expect(view.calls.filter(call => ["import", "write", "attach", "catalog", "focus"].includes(call))).toEqual(fixture.expectedBeforeAck);
  expect(view.calls).not.toContain("shell.documentTransfer.file-opened");
  const admission = view.core.writeLocalDocumentRef.current.mock.calls[0]![0] as unknown as { document: typeof document };
  expect(admission.document).toMatchObject({ documentId: document.documentId, name: document.name, surface: document.surface });
  ack.resolve({ documents: [document] }); await pending;
  expect(view.calls.slice(-2)).toEqual(fixture.expectedPublished);
});
it("refuses duplicate identity before any archive write", async () => {
  const view = setup(); view.core.localCatalogRef.current.documents = [document];
  await view.state.openNaturalFile();
  expect(view.core.writeLocalDocumentRef.current).not.toHaveBeenCalled();
  expect(view.handle.destroyApp).toHaveBeenCalledWith(7);
  expect(view.calls).not.toContain("focus");
});
it("reopens the same saved UUID and surface, and focuses an existing owner without reading", async () => {
  const view = setup(); view.core.localCatalogRef.current.documents = [document];
  await view.state.openKeptFile(document, new AbortController().signal);
  expect(view.handle.createApp).toHaveBeenCalledWith(document.surface.appId);
  expect(view.handle.loadAppDocumentArchive.mock.calls[0]?.[1]).toEqual(archive);
  expect(view.core.openDocumentRef.current.mock.calls[0]?.[0]).toEqual({ documentId: document.documentId, schema: document.schema });
  view.core.openDocumentSessionsRef.current.set(document.documentId, { plugin: view.handle, session: { app: view.core.loadedPluginsRef.current[0]!.manifest.apps[0], instanceId: 9 } });
  view.core.readLocalDocumentRef.current.mockClear(); view.handle.createApp.mockClear();
  await view.state.openKeptFile(document, new AbortController().signal);
  expect(view.handle.createApp).not.toHaveBeenCalled(); expect(view.core.readLocalDocumentRef.current).not.toHaveBeenCalled();
});
for (const failure of ["load", "identity", "stale", "cancel"] as const) it("retires failed reopening: " + failure, async () => {
  const view = setup(), abort = new AbortController(); view.core.localCatalogRef.current.documents = [document];
  view.handle.loadAppDocumentArchive.mockImplementation(async () => {
    if (failure === "load") throw Error("bad archive");
    if (failure === "identity") view.handle.readAppDocumentIdentity.mockResolvedValue({ parent_document_id: "different" });
    if (failure === "stale") view.core.sessionRef.current = { ...view.core.sessionRef.current, instanceId: 99 };
    if (failure === "cancel") abort.abort();
  });
  await view.state.openKeptFile(document, abort.signal);
  expect(view.calls.filter(call => fixture.expectedFailureCleanup.includes(call))).toEqual(fixture.expectedFailureCleanup);
  expect(view.calls).not.toContain("focus");
  expect(view.core.documentTransferFlightRef.current).toBe(false);
});
it("reports the archive failure while the imported instance is still retiring", async () => {
  const view = setup();
  const cleanup = deferred<void>();
  selection.files = [new File([encodeDocumentArchiveBytes(archive).slice().buffer], "fixture.semio-archive")];
  view.handle.loadAppDocumentArchive.mockRejectedValueOnce(new Error("archive initialization failed"));
  view.handle.destroyApp.mockImplementation(async () => {
    view.calls.push("destroy");
    await cleanup.promise;
  });
  vi.spyOn(console, "warn").mockImplementation(() => undefined);
  const importing = view.state.importDocumentArchive();
  try {
    await vi.waitFor(() => expect(view.handle.destroyApp).toHaveBeenCalledWith(7));
    expect(view.calls).toEqual(fixture.archiveFailureBeforeRetirement);
    expect(view.handle.loadAppDocumentArchive.mock.calls[0]?.[1]).toEqual(archive);
    expect(view.core.documentTransferFlightRef.current).toBe(true);
  } finally {
    cleanup.resolve();
    await importing;
  }
  expect(view.core.documentTransferFlightRef.current).toBe(false);
});

it("cancels an archive between file slices before creating a program", async () => {
  const view = setup(), chunkBytes = NATURAL_FILE_BROWSER_READ_CHUNK_BYTES_V1;
  const file = new File([new Uint8Array(chunkBytes + 1)], "cancelled.semio-archive");
  const slice = vi.spyOn(file, "slice");
  selection.files = [file];
  const progress: number[] = [];
  const transfers = view.core.documentTransfersRef.current as Map<string, { transfer: { progress: { completed: number } | null }; abort: AbortController }>;
  (view.core.taskManagerListenersRef.current as Set<() => void>).add(() => {
    for (const { transfer, abort } of transfers.values()) {
      const completed = transfer.progress?.completed;
      if (completed !== undefined && !abort.signal.aborted) {
        progress.push(completed);
        if (completed === chunkBytes) abort.abort();
      }
    }
  });
  vi.spyOn(console, "warn").mockImplementation(() => undefined);
  await view.state.importDocumentArchive();
  expect(progress).toEqual([0, chunkBytes]);
  expect(slice).toHaveBeenCalledExactlyOnceWith(0, chunkBytes);
  expect(view.handle.createApp).not.toHaveBeenCalled();
  expect(view.calls).toContain("shell.documentTransfer.import-cancelled");
  expect(transfers.size).toBe(0);
});

it("refuses archives above the wire limit before slicing or creating a program", async () => {
  const view = setup();
  const file = new File([], "oversized.semio-archive");
  vi.spyOn(file, "size", "get").mockReturnValue(DOCUMENT_ARCHIVE_MAXIMUM_BYTES + 1);
  const slice = vi.spyOn(file, "slice");
  selection.files = [file];
  vi.spyOn(console, "warn").mockImplementation(() => undefined);
  await view.state.importDocumentArchive();
  expect(slice).not.toHaveBeenCalled();
  expect(view.handle.createApp).not.toHaveBeenCalled();
  expect(view.calls).toContain("shell.documentTransfer.import-unreadable");
  expect(view.core.documentTransfersRef.current.size).toBe(0);
});

it("awaits genuine native import acknowledgment before any persistence or opened notice", async () => {
  const view = setup(), imported = deferred<void>();
  view.handle.importAppMedia.mockImplementation(async () => { view.calls.push("import"); await imported.promise; });
  const pending = view.state.openNaturalFile();
  await vi.waitFor(() => expect(view.calls).toContain("import"));
  expect(view.core.writeLocalDocumentRef.current).not.toHaveBeenCalled();
  expect(view.calls).not.toContain("focus");
  expect(view.calls).not.toContain("shell.documentTransfer.file-opened");
  imported.resolve(); await pending;
  expect(view.calls).toContain("shell.documentTransfer.file-opened");
});

function setupSave(bound = true, segmented = false) {
  const view = setup(segmented);
  const entry = { documentId: document.documentId, clientInstanceId: "01900000-0000-4000-8000-000000000002", plugin: view.handle, session: view.target };
  if (bound) view.core.openDocumentSessionsRef.current.set(document.documentId, entry);
  else {
    view.core.sessionRef.current = view.target;
    view.core.openDocumentRef.current.mockImplementation(async (..._args: unknown[]) => {
      view.calls.push("attach"); view.core.openDocumentSessionsRef.current.set(document.documentId, entry);
      return { runtimeKey: document.documentId, clientInstanceId: entry.clientInstanceId };
    });
  }
  const observer = () => view.core.localDocumentOwnersRef.current.get(entry.clientInstanceId) as { status: (persisted: boolean) => void; archive: (bytes: Uint8Array) => void };
  const requests = () => view.worker.postMessage.mock.calls.map(([message]) => decodeBackboneWorkerRequest((message as { wire: Uint8Array }).wire));
  return { ...view, entry, observer, requests };
}
it("Save waits for exact fresh archive readback; persisted and older content do not permit download", async () => {
  const view = setupSave();
  const saving = view.state.saveNaturalFile();
  await vi.waitFor(() => expect(view.observer()).toBeDefined());
  expect(view.requests().filter(request => request.kind === "send").map(request => request.kind === "send" && request.message.kind)).toEqual(fixture.saveRequests);
  const count = view.requests().length;
  view.observer().status(false); expect(view.requests()).toHaveLength(count);
  view.observer().status(true); await vi.waitFor(() => expect(view.requests()).toHaveLength(count + 1));
  view.observer().archive(encodeDocumentArchiveBytes({ ...archive, parent_pack: [9] }));
  await Promise.resolve();
  expect(view.handle.exportAppMedia).not.toHaveBeenCalled();
  expect(selection.downloads).toEqual([]);
  expect(view.calls).not.toContain("shell.documentTransfer.file-saved");
  view.observer().archive(encodeDocumentArchiveBytes(archive));
  await saving;
  expect(selection.downloads).toHaveLength(1);
  expect(view.calls.slice(-2)).toEqual(fixture.savePublished);
  expect(view.core.localDocumentOwnersRef.current.size).toBe(0);
});
for (const failure of ["cancel", "stale", "archive"] as const) it("Save refuses download after " + failure, async () => {
  const view = setupSave();
  if (failure === "archive") view.handle.readAppDocumentArchive.mockRejectedValue(Error("disk unavailable"));
  const saving = view.state.saveNaturalFile();
  if (failure !== "archive") {
    await vi.waitFor(() => expect(view.observer()).toBeDefined());
    if (failure === "cancel") {
      const task = [...view.core.documentTransfersRef.current.values()][0] as { abort: AbortController };
      task.abort.abort();
    } else { const observer = view.observer(); view.core.openDocumentSessionsRef.current.clear(); observer.archive(encodeDocumentArchiveBytes(archive)); }
  }
  await saving;
  expect(selection.downloads).toEqual([]);
  expect(view.calls).not.toContain("shell.documentTransfer.file-saved");
  expect(view.core.documentTransferFlightRef.current).toBe(false);
  expect(view.core.localDocumentOwnersRef.current.size).toBe(0);
});
it("Save persists and attaches a palette draft before catalog acknowledgment and export", async () => {
  const view = setupSave(false), ack = deferred<{ documents: typeof document[] }>();
  view.core.recordLocalCatalogRef.current.mockImplementation(async () => { view.calls.push("catalog"); return ack.promise; });
  const saving = view.state.saveNaturalFile();
  await vi.waitFor(() => expect(view.calls).toContain("catalog"));
  expect(view.calls.filter(call => ["write", "attach", "catalog", "export"].includes(call))).toEqual(fixture.saveDraftBeforeAck);
  expect(selection.downloads).toEqual([]);
  ack.resolve({ documents: [document] });
  await vi.waitFor(() => expect(view.observer()).toBeDefined());
  expect(view.handle.exportAppMedia).not.toHaveBeenCalled();
  view.observer().archive(encodeDocumentArchiveBytes(archive));
  await saving;
  expect(view.calls).toContain("export");
  expect(view.calls).toContain("shell.documentTransfer.file-saved");
});
it("Save refuses an owner retired during native export", async () => {
  const view = setupSave(), exported = deferred<Awaited<ReturnType<typeof view.handle.exportAppMedia>>>();
  const artifact = await view.handle.exportAppMedia();
  view.handle.exportAppMedia.mockClear();
  view.handle.exportAppMedia.mockImplementation(async () => exported.promise);
  const saving = view.state.saveNaturalFile();
  await vi.waitFor(() => expect(view.observer()).toBeDefined());
  view.observer().archive(encodeDocumentArchiveBytes(archive));
  await vi.waitFor(() => expect(view.handle.exportAppMedia).toHaveBeenCalledTimes(1));
  view.core.openDocumentSessionsRef.current.clear();
  exported.resolve(artifact);
  await saving;
  expect(selection.downloads).toEqual([]);
  expect(view.calls).not.toContain("shell.documentTransfer.file-saved");
});

it("Save refuses natural bytes exported after the proved document was edited", async () => {
  const view = setupSave(), exported = deferred<Awaited<ReturnType<typeof view.handle.exportAppMedia>>>();
  const artifact = await view.handle.exportAppMedia();
  view.handle.exportAppMedia.mockClear(); view.calls.length = 0;
  view.handle.exportAppMedia.mockImplementation(async () => exported.promise);
  const saving = view.state.saveNaturalFile();
  await vi.waitFor(() => expect(view.observer()).toBeDefined());
  view.observer().archive(encodeDocumentArchiveBytes(archive));
  await vi.waitFor(() => expect(view.handle.exportAppMedia).toHaveBeenCalledTimes(1));
  view.handle.readAppDocumentArchive.mockResolvedValue({ ...archive, parent_pack: [9] });
  exported.resolve({ ...artifact, data: new Uint8Array([66]) });
  await saving;
  expect(selection.downloads).toEqual([]);
  expect(view.calls).not.toContain("shell.documentTransfer.file-saved");
});

for (const retired of ["plugin", "worker"] as const) it("Save refuses " + retired + " replacement during final archive read", async () => {
  const view = setupSave(), finalRead = deferred<typeof archive>();
  let reads = 0;
  view.handle.readAppDocumentArchive.mockImplementation(async () => ++reads === 1 ? archive : finalRead.promise);
  const saving = view.state.saveNaturalFile();
  await vi.waitFor(() => expect(view.observer()).toBeDefined());
  view.observer().archive(encodeDocumentArchiveBytes(archive));
  await vi.waitFor(() => expect(reads).toBe(2));
  if (retired === "plugin") view.core.loadedPluginsRef.current = [];
  else view.core.backboneWorkerRef.current = { postMessage: vi.fn() };
  finalRead.resolve(archive);
  await saving;
  expect(selection.downloads).toEqual([]);
  expect(view.calls).not.toContain("shell.documentTransfer.file-saved");
});

it("retirement persists the captured unbound draft without focusing or exporting", async () => {
  const view = setupSave(false); view.core.resolveSyncTargetSession = () => ({ ...view.target, instanceId: 99 });
  await view.core.persistProgramDocumentRef.current({ handle: view.handle, session: view.target, name: "Captured.txt" }, new AbortController().signal);
  expect(view.core.writeLocalDocumentRef.current).toHaveBeenCalledTimes(1);
  expect(view.core.openDocumentRef.current).toHaveBeenCalledTimes(1);
  expect(view.core.recordLocalCatalogRef.current).toHaveBeenCalledTimes(1);
  expect(view.handle.exportAppMedia).not.toHaveBeenCalled(); expect(selection.downloads).toEqual([]); expect(view.calls).not.toContain("focus");
});
it("retirement accepts an authoritative documentless identity without creating storage", async () => {
  const view = setupSave(false); view.handle.readAppDocumentIdentity.mockResolvedValue({ parent_document_id: null } as never);
  await view.core.persistProgramDocumentRef.current({ handle: view.handle, session: view.target, name: "Tool" }, new AbortController().signal);
  expect(view.core.writeLocalDocumentRef.current).not.toHaveBeenCalled(); expect(view.handle.readAppDocumentArchive).not.toHaveBeenCalled();
});
it("retirement refuses identity failure rather than treating it as a documentless tool", async () => {
  const view = setupSave(false); view.handle.readAppDocumentIdentity.mockRejectedValue(Error("channel unavailable"));
  await expect(view.core.persistProgramDocumentRef.current({ handle: view.handle, session: view.target, name: "Draft" }, new AbortController().signal)).rejects.toThrow("channel unavailable");
  expect(view.core.writeLocalDocumentRef.current).not.toHaveBeenCalled(); expect(view.core.documentTransferFlightRef.current).toBe(false);
});
it("retirement of an attached owner waits for the exact captured archive on disk", async () => {
  const view = setupSave(); view.core.sessionRef.current = view.target;
  const pending = view.core.persistProgramDocumentRef.current({ handle: view.handle, session: view.target, name: "Captured.txt" }, new AbortController().signal);
  await vi.waitFor(() => expect(view.observer()).toBeDefined());
  view.observer().status(true); view.observer().archive(encodeDocumentArchiveBytes({ ...archive, parent_pack: [8] }));
  expect(view.core.documentTransferFlightRef.current).toBe(true);
  view.observer().archive(encodeDocumentArchiveBytes(archive)); await pending;
  expect(view.core.documentTransferFlightRef.current).toBe(false); expect(view.handle.exportAppMedia).not.toHaveBeenCalled(); expect(selection.downloads).toEqual([]);
});
it("new natural opening refuses a plugin already being retired", async () => {
  const view = setup(); view.core.pluginOpInFlightRef.current.add(view.handle.pluginId);
  await view.state.openNaturalFile(); expect(view.handle.createApp).not.toHaveBeenCalled(); expect(view.calls).not.toContain("focus");
});

it("retirement recovers an existing kept owner with current bytes before reattaching, and settles the reattachment before the instance may close", async () => {
  const view = setupSave(false); view.core.localCatalogRef.current.documents = [document];
  const currentArchive = { ...archive, parent_pack: [9], members: [{ ordinal: 0, reference: { artifact_id: "child", artifact_kind: "stdio.txt", standard: "utf-8", subset: "*" }, owner: { parent: { artifact_id: document.documentId, artifact_kind: "stdio.txt", standard: "utf-8", subset: "*" }, slot: "member", child_id: "child" }, envelope_pack: [7] }] };
  view.handle.readAppDocumentArchive.mockResolvedValue(currentArchive as never);
  await view.core.persistProgramDocumentRef.current({ handle: view.handle, session: view.target, name: "Recovered.txt" }, new AbortController().signal);
  const admission = view.core.writeLocalDocumentRef.current.mock.calls[0]![0] as { document: typeof document; archive: Uint8Array };
  expect(admission.document).toEqual(document); expect(admission.archive).toEqual(encodeDocumentArchiveBytes(currentArchive));
  expect(view.calls.filter(call => ["drain", "write", "attach", "catalog", "focus"].includes(call))).toEqual(["drain", "write", "attach", "drain"]);
  expect(view.handle.loadAppDocumentArchive).not.toHaveBeenCalled(); expect(selection.downloads).toEqual([]);
});
it("recovery refuses attachment if native content changes after the verified write", async () => {
  const view = setupSave(false); view.core.localCatalogRef.current.documents = [document];
  view.core.writeLocalDocumentRef.current.mockImplementation(async () => { view.handle.readAppDocumentArchive.mockResolvedValue({ ...archive, parent_pack: [9] }); return true; });
  await expect(view.core.persistProgramDocumentRef.current({ handle: view.handle, session: view.target, name: "Recovered.txt" }, new AbortController().signal)).rejects.toThrow("saved-revision-changed");
  expect(view.core.openDocumentRef.current).not.toHaveBeenCalled();
});

it("recovery refuses a kept record retired during its archive write", async () => {
  const view = setupSave(false); view.core.localCatalogRef.current.documents = [document];
  view.core.writeLocalDocumentRef.current.mockImplementation(async () => { view.core.localCatalogRef.current.documents = []; return true; });
  await expect(view.core.persistProgramDocumentRef.current({ handle: view.handle, session: view.target, name: "Recovered.txt" }, new AbortController().signal)).rejects.toThrow("persistence-owner-unavailable");
  expect(view.core.openDocumentRef.current).not.toHaveBeenCalled();
});

for (const outcome of fixture.segmentedSave) it("Save delivers staged native output " + outcome.id, async () => {
  const view = setupSave(true, true);
  const portId = "artifact:native", mediaType = "application/pdf", formatKind = "s.stdio.pdf@1.7";
  const identity = { app_instance_id: view.target.instanceId, parent_document_id: document.documentId, base_revision: 3n };
  const handle = { ...identity, operation_id: 8n, generation: 2n };
  const submitted = vi.fn(async () => handle);
  const segments = outcome.chunks.map(bytes => Uint8Array.from(bytes));
  Object.assign(view.handle, {
    readAppDocumentIdentity: async () => identity,
    submitMediaExport: submitted,
    pollMediaExport: async () => ({ handle, state: "complete", applied_progress: 2n, checkpoint_available: false, mime_type: mediaType, total_bytes: BigInt(segments.reduce((total, bytes) => total + bytes.length, 0)), detail: "", descriptor: [...encodePackValue({ portId, kindId: formatKind, mediaType: { class: "data", form: "document" }, wire: { kind: "binary", format_kind: formatKind } })] }),
    takeMediaExportChunk: async () => {
      const data = segments.shift();
      if (data !== undefined) return { handle, data, terminal: false };
      if (outcome.id === "edited") view.handle.readAppDocumentArchive.mockResolvedValue({ ...archive, parent_pack: [9] });
      return { handle, data: new Uint8Array(), terminal: true };
    },
    cancelMediaExport: vi.fn(),
  });
  const saving = view.state.saveNaturalFile();
  await vi.waitFor(() => expect(view.observer()).toBeDefined());
  expect(submitted).not.toHaveBeenCalled();
  view.observer().archive(encodeDocumentArchiveBytes(archive));
  await saving;
  expect(submitted).toHaveBeenCalledExactlyOnceWith(view.target.instanceId, portId, document.documentId, 3n);
  expect(view.handle.exportAppMedia).not.toHaveBeenCalled();
  expect(selection.segmented).toEqual(outcome.chunks);
  expect(selection.downloads).toHaveLength(outcome.delivered ? 1 : 0);
  expect(selection.aborted).toBe(!outcome.delivered);
  expect(view.calls.includes("shell.documentTransfer.file-saved")).toBe(outcome.delivered);
});
it("opens a dropped OS file through the same path as Open without asking the picker", async () => {
  const view = setup();
  selection.files = [];
  await view.state.openNaturalFile(new File(["hi"], "Dropped.txt"));
  expect(view.calls.filter(call => ["create", "import", "write", "attach", "catalog", "focus"].includes(call))).toEqual(["create", "import", "write", "attach", "catalog", "focus"]);
  expect((view.core.writeLocalDocumentRef.current.mock.calls[0]![0] as unknown as { document: typeof document }).document.name).toBe("Dropped.txt");
  expect(view.calls).toContain("shell.documentTransfer.file-opened");
  expect(view.core.documentTransferFlightRef.current).toBe(false);
});
it("Save keeps the file name the document was opened from", async () => {
  const view = setupSave();
  view.core.localCatalogRef.current.documents = [{ ...document, name: "Budget 2026.TXT" }];
  const saving = view.state.saveNaturalFile();
  await vi.waitFor(() => expect(view.observer()).toBeDefined());
  view.observer().archive(encodeDocumentArchiveBytes(archive));
  await saving;
  expect(selection.downloads).toEqual(["Budget 2026.TXT"]);
});
it("Save names a document that never came from a file by kind and time", async () => {
  const view = setupSave();
  const saving = view.state.saveNaturalFile();
  await vi.waitFor(() => expect(view.observer()).toBeDefined());
  view.observer().archive(encodeDocumentArchiveBytes(archive));
  await saving;
  expect(selection.downloads).toHaveLength(1);
  expect(selection.downloads[0]).toMatch(/^txt-\d{8}T\d{6}Z\.txt$/u);
});

const standaloneOpening = opened.cases.find(entry => entry.id === "standalone-adopts-the-opened-file")!;
it("a standalone program host adopts the opened file as its program after it is durable, so every window follows it", async () => {
  const view = setup(false, false);
  view.core.sessionRef.current = { ...view.core.sessionRef.current, pluginId: view.handle.pluginId, app: { id: document.surface.appId } };
  await view.state.openNaturalFile();
  expect(view.calls.filter(call => ["import", "write", "attach", "catalog", "focus", "adopt"].includes(call))).toEqual(["import", "write", "attach", "catalog", standaloneOpening.publication === "adopt-primary" ? "adopt" : "focus"]);
  expect(view.core.adoptPreparedPrimarySession).toHaveBeenCalledWith(document.surface.appId, expect.objectContaining({ handle: view.handle, instanceId: 7 }));
  expect(view.core.sessionRef.current.instanceId).toBe(7);
  expect(view.calls).toContain("shell.documentTransfer.file-opened");
  expect(view.handle.destroyApp).not.toHaveBeenCalled();
});
it("a standalone program host whose session switch refuses the opened file retires it and unlists it", async () => {
  const view = setup(false, false);
  view.core.adoptPreparedPrimarySession.mockImplementation(async () => { view.calls.push("adopt"); });
  vi.spyOn(console, "warn").mockImplementation(() => undefined);
  await view.state.openNaturalFile();
  expect(view.calls).toContain("adopt");
  expect(view.calls).toContain("close");
  expect(view.handle.destroyApp).toHaveBeenCalledWith(7);
  expect(view.calls).not.toContain("shell.documentTransfer.file-opened");
  expect(view.core.documentTransferFlightRef.current).toBe(false);
});
it("a standalone program host reopens a kept file as its program, and a kept file it already shows needs no switch", async () => {
  const view = setup(false, false); view.core.localCatalogRef.current.documents = [document];
  await view.state.openKeptFile(document, new AbortController().signal);
  expect(view.calls.filter(call => ["load", "attach", "focus", "adopt"].includes(call))).toEqual(["load", "attach", "adopt"]);
  expect(view.core.sessionRef.current.instanceId).toBe(7);
  view.core.openDocumentSessionsRef.current.set(document.documentId, { plugin: view.handle, session: { app: view.core.loadedPluginsRef.current[0]!.manifest.apps[0], instanceId: 7 } });
  view.core.adoptPreparedPrimarySession.mockClear(); view.core.readLocalDocumentRef.current.mockClear();
  await view.state.openKeptFile(document, new AbortController().signal);
  expect(view.core.adoptPreparedPrimarySession).not.toHaveBeenCalled(); expect(view.core.readLocalDocumentRef.current).not.toHaveBeenCalled();
  expect(view.calls.filter(call => call === "shell.documentTransfer.file-opened")).toHaveLength(2);
});
it("a standalone program host checkpoints the uncommitted edits of the program it replaces before the switch retires it", async () => {
  const view = setup(false, false);
  const primary = { ...view.core.sessionRef.current, pluginId: view.handle.pluginId, app: { id: document.surface.appId, controllerId: "pdf" }, instanceId: 1 };
  view.core.sessionRef.current = primary;
  view.core.historyStore.get = () => ({ [`${primary.pluginId}#1`]: { cursor: 2, entries: { a: { seq: 1, kind: "mutation", applied: true }, b: { seq: 2, kind: "mutation", applied: true } } } });
  await view.state.openNaturalFile();
  expect(view.calls.filter(call => ["catalog", "commitCheckpoint", "adopt"].includes(call))).toEqual(["catalog", "commitCheckpoint", "adopt"]);
  expect(view.core.onAction).toHaveBeenCalledWith(expect.objectContaining({ controllerId: "pdf", action: "commitCheckpoint" }));
  expect(view.core.sessionRef.current.instanceId).toBe(7);
});
