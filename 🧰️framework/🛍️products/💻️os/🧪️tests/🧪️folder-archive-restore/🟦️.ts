/** 🗃️ LAW: a document attached to a local folder survives a reload (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING follow-ups
 * 3 and 4, e2e finding 7). The device remembers the folder in its local-only `os.config.local-folders` facet; after the reload
 * the same document identity is offered its folder back, and the reconnect restores the head and the history rows. Attach → edit → reload → re-attach walks the real worker actor over a fake folder: every edit the program
 * publishes is admitted (a local document has no hub cold pair to wait for — the Rust actor never asked for one), the
 * program's archive write is the folder's content, the watch reading that write back is the document's own echo and replaces
 * nothing, and after the reload the one read-back is restored through `restoreDocumentArchiveV1`, which hands the fresh
 * program the saved head and re-reads its history rows before any surface paints again. A stale restore hydrates nothing. */
import { encodeDocumentBackboneEnvelopeBatchExact } from "@semio-tech/framework-replication";
import { decodeDocumentArchiveBytes, encodeBackboneMessage, encodeDocumentArchiveBytes, encodePackValue, type ArtifactActorConfig, type BackboneWorkerRequest, type BackboneWorkerResponse, type DocumentArchivePack } from "../../🟦️.ts";
import { restoreDocumentArchiveV1 } from "../../🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import { commitLocalFoldersConfigMutationV1, localFolderReconnectOfferV1, readLocalFolderBindingsV1, readLocalFolderEventsV1 } from "../../🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📎️local-folders/🟦️.tsx";
import { attachLocalFolder, detachLocalFolder, LOCAL_FOLDERS_CONFIG_SCHEMA } from "../../🎚️config/🧬️schema/🧬️mutations/🟦️.ts";
import { createMemoryStoragePort, OsShellConfig } from "@semio-tech/framework";

type Point = readonly [number, number];
type Folder = Map<string, Uint8Array>;

/** 🧩️ A program that owns one head and its history rows, exported and loaded whole as a document archive. */
class FolderProgram {
  positions: Record<string, Point> = { A: [86, 11], B: [120, 11] };
  rows: string[] = ["Set Active Example"];
  #edits = 0;

  constructor(readonly documentId: string, readonly actor: string) {}

  drag(id: string, to: Point): Uint8Array {
    this.#edits += 1;
    this.positions = { ...this.positions, [id]: to };
    this.rows = [...this.rows, `Drag ${id} to (${to[0]}, ${to[1]})`];
    return encodeBackboneMessage({
      kind: "mutations",
      envelopes: encodeDocumentBackboneEnvelopeBatchExact([{
        mutation_id: `${this.actor}-drag-${this.#edits}`,
        document_id: this.documentId,
        actor: this.actor,
        dependencies: [],
        observed: null,
        target: [],
        diff: { schema: "demo/v1", payload: encodePackValue({ id, to: [...to] }) },
        inverse: { schema: "demo/v1", payload: encodePackValue(null) },
        timestamp: { actor: 1n, physical_ms: BigInt(1_000 + this.#edits), logical: 0n },
        transaction: null,
        verb: null,
      }]),
    });
  }

  archive(): DocumentArchivePack {
    return { parent_pack: Array.from(new TextEncoder().encode(JSON.stringify({ positions: this.positions, rows: this.rows }))), parent_spr: [], members: [] };
  }

  load(archive: DocumentArchivePack): void {
    const state = JSON.parse(new TextDecoder().decode(Uint8Array.from(archive.parent_pack))) as { positions: Record<string, Point>; rows: string[] };
    this.positions = state.positions;
    this.rows = state.rows;
  }
}

export async function registerFolderArchiveRestoreTests(
  vitest: NonNullable<ImportMeta["vitest"]>,
  worker: {
    readonly testSeams: { workerPostTestSink: ((message: BackboneWorkerResponse) => void) | null };
    readonly artifactState: (documentId: string, spaceId?: string) => { readonly pendingMutations: readonly unknown[]; readonly revalidateFolder: () => Promise<void> } | undefined;
    readonly closeArtifact: (documentId: string, spaceId: string | undefined, clientInstanceId: string) => void;
    readonly handleTsRequest: (request: BackboneWorkerRequest) => void;
    readonly installStreamMuxEndpoint: (endpoint: null) => void;
  },
): Promise<void> {
  const { describe, it, expect } = vitest;
  const documentId = "puzzle-folder-restore";
  const binding = { kind: "folder", dataClass: "persistedLocalOnly", path: "/tmp/folder-archive-restore" } as const;
  const config = (clientInstanceId: string): ArtifactActorConfig & { readonly kind: "open"; readonly clientInstanceId: string } => ({ kind: "open", clientInstanceId, documentId, schema: "demo/v1", bindings: [binding], watchExternal: false, actor: "actor-1" });
  const settle = async (done: () => boolean): Promise<void> => {
    for (let turn = 0; turn < 200 && !done(); turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
    expect(done()).toBe(true);
  };
  const folderFetch = (folder: Folder, reads: { count: number }) => async (url: string, init?: RequestInit) => {
    if (init?.method === "PUT") {
      folder.set(url, new Uint8Array(init.body as Uint8Array));
      return new Response(null, { status: 200 });
    }
    reads.count += 1;
    const bytes = folder.get(url);
    return bytes === undefined ? new Response(null, { status: 204 }) : new Response(bytes.slice(), { status: 200 });
  };
  const events = (posted: readonly BackboneWorkerResponse[], kind: string) => posted.flatMap((message) => (message.kind === "event" && message.event.kind === kind ? [message.event] : []));

  describe("folder archive restore", () => {
    it("attach, edit, reload and re-attach restores the head and the history rows, with no refused edit and no own-write echo", async () => {
      const folder: Folder = new Map();
      const reads = { count: 0 };
      const posted: BackboneWorkerResponse[] = [];
      const originalFetch = globalThis.fetch;
      (globalThis as unknown as { fetch: unknown }).fetch = folderFetch(folder, reads);
      worker.installStreamMuxEndpoint(null);
      worker.testSeams.workerPostTestSink = (message) => posted.push(message);
      const first = crypto.randomUUID();
      const second = crypto.randomUUID();
      try {
        worker.handleTsRequest(config(first));
        await settle(() => reads.count === 1);
        const before = new FolderProgram(documentId, "actor-1");
        worker.handleTsRequest({ kind: "send", documentId, clientInstanceId: first, message: { kind: "documentBackbone", message: before.drag("A", [206, 81]) } });
        await settle(() => (worker.artifactState(documentId)?.pendingMutations.length ?? 0) === 1);
        worker.handleTsRequest({ kind: "send", documentId, clientInstanceId: first, message: { kind: "localDocumentArchive", archive: Array.from(encodeDocumentArchiveBytes(before.archive())) } });
        await settle(() => folder.size === 1 && worker.artifactState(documentId)?.pendingMutations.length === 0);
        await worker.artifactState(documentId)!.revalidateFolder();
        expect(reads.count).toBe(2);
        expect(events(posted, "commandOutcome")).toEqual([]);
        expect(events(posted, "documentArchiveReplaced")).toEqual([]);
        worker.closeArtifact(documentId, undefined, first);

        posted.length = 0;
        const after = new FolderProgram(documentId, "actor-1");
        expect(after.positions.A).toEqual([86, 11]);
        worker.handleTsRequest(config(second));
        await settle(() => events(posted, "documentArchiveReplaced").length === 1);
        const replaced = events(posted, "documentArchiveReplaced")[0] as { readonly archive: readonly number[] };
        const painted: { positions: Record<string, Point> | null; rows: readonly string[] | null } = { positions: null, rows: null };
        const restored = await restoreDocumentArchiveV1(decodeDocumentArchiveBytes(Uint8Array.from(replaced.archive)), () => true, {
          load: async (archive) => { after.load(archive); return true; },
          history: async () => { painted.rows = [...after.rows]; },
          refresh: async () => { painted.positions = { ...after.positions }; },
        });
        expect(restored).toBe(true);
        expect(after.positions.A).toEqual([206, 81]);
        expect(painted).toEqual({ positions: { A: [206, 81], B: [120, 11] }, rows: ["Set Active Example", "Drag A to (206, 81)"] });
        worker.handleTsRequest({ kind: "send", documentId, clientInstanceId: second, message: { kind: "documentBackbone", message: after.drag("B", [140, 30]) } });
        await settle(() => (worker.artifactState(documentId)?.pendingMutations.length ?? 0) === 1);
        expect(events(posted, "commandOutcome")).toEqual([]);
      } finally {
        worker.closeArtifact(documentId, undefined, second);
        worker.closeArtifact(documentId, undefined, first);
        worker.testSeams.workerPostTestSink = null;
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
      }
    });

    it("a remembered folder binding survives the reload and its reconnect restores the head and the history rows", async () => {
      const folder: Folder = new Map();
      const reads = { count: 0 };
      const posted: BackboneWorkerResponse[] = [];
      const device = createMemoryStoragePort();
      const originalFetch = globalThis.fetch;
      (globalThis as unknown as { fetch: unknown }).fetch = folderFetch(folder, reads);
      worker.installStreamMuxEndpoint(null);
      worker.testSeams.workerPostTestSink = (message) => posted.push(message);
      const first = crypto.randomUUID();
      const second = crypto.randomUUID();
      const identity = { documentId, pluginId: "puzzle2d", appId: "s.puzzle.puzzle2d@1/*#editor" };
      try {
        worker.handleTsRequest(config(first));
        await settle(() => reads.count === 1);
        commitLocalFoldersConfigMutationV1(device, attachLocalFolder({ ...identity, folder: { kind: "path", path: binding.path } }));
        const before = new FolderProgram(documentId, "actor-1");
        worker.handleTsRequest({ kind: "send", documentId, clientInstanceId: first, message: { kind: "documentBackbone", message: before.drag("A", [206, 81]) } });
        worker.handleTsRequest({ kind: "send", documentId, clientInstanceId: first, message: { kind: "localDocumentArchive", archive: Array.from(encodeDocumentArchiveBytes(before.archive())) } });
        await settle(() => folder.size === 1);
        worker.closeArtifact(documentId, undefined, first);

        posted.length = 0;
        const after = new FolderProgram(documentId, "actor-1");
        const offer = localFolderReconnectOfferV1(readLocalFolderBindingsV1(device), identity, new Set());
        expect(offer).toEqual({ ...identity, folder: { kind: "path", path: binding.path } });
        expect(localFolderReconnectOfferV1(readLocalFolderBindingsV1(device), { ...identity, appId: "s.puzzle.puzzle2d@1/*#viewer" }, new Set()), "another program holding the same id is offered nothing").toBeNull();
        expect(localFolderReconnectOfferV1(readLocalFolderBindingsV1(device), identity, new Set([documentId])), "an attached document is offered nothing").toBeNull();
        worker.handleTsRequest({ ...config(second), bindings: [{ kind: "folder", dataClass: "persistedLocalOnly", path: offer!.folder.path }] });
        await settle(() => events(posted, "documentArchiveReplaced").length === 1);
        const replaced = events(posted, "documentArchiveReplaced")[0] as { readonly archive: readonly number[] };
        const painted: { positions: Record<string, Point> | null; rows: readonly string[] | null } = { positions: null, rows: null };
        expect(await restoreDocumentArchiveV1(decodeDocumentArchiveBytes(Uint8Array.from(replaced.archive)), () => true, {
          load: async (archive) => { after.load(archive); return true; },
          history: async () => { painted.rows = [...after.rows]; },
          refresh: async () => { painted.positions = { ...after.positions }; },
        })).toBe(true);
        expect(painted).toEqual({ positions: { A: [206, 81], B: [120, 11] }, rows: ["Set Active Example", "Drag A to (206, 81)"] });
      } finally {
        worker.closeArtifact(documentId, undefined, second);
        worker.closeArtifact(documentId, undefined, first);
        worker.testSeams.workerPostTestSink = null;
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
      }
    });

    it("a detached folder is forgotten across the reload, and a binding is written to this device's local-only facet alone", () => {
      const device = createMemoryStoragePort();
      const identity = { documentId, pluginId: "puzzle2d", appId: "s.puzzle.puzzle2d@1/*#editor" };
      const attached = commitLocalFoldersConfigMutationV1(device, attachLocalFolder({ ...identity, folder: { kind: "path", path: binding.path } }));
      expect(attached.bindings).toHaveLength(1);
      expect(Object.keys(new OsShellConfig(device).getSnapshot().preferences)).toEqual([LOCAL_FOLDERS_CONFIG_SCHEMA]);
      expect(commitLocalFoldersConfigMutationV1(device, attachLocalFolder({ ...identity, folder: { kind: "path", path: binding.path } }))).toEqual(attached);
      expect(readLocalFolderEventsV1(device), "an attachment that changes nothing is not recorded").toHaveLength(1);
      commitLocalFoldersConfigMutationV1(device, detachLocalFolder(documentId));
      expect(readLocalFolderBindingsV1(device)).toEqual({ bindings: [] });
      expect(localFolderReconnectOfferV1(readLocalFolderBindingsV1(device), identity, new Set())).toBeNull();
      expect(readLocalFolderEventsV1(device).map((event) => event.mutation)).toEqual(["attachLocalFolder", "detachLocalFolder"]);
      new OsShellConfig(device).setPreference(LOCAL_FOLDERS_CONFIG_SCHEMA, JSON.stringify({ version: 1, events: [{ mutation: "attachLocalFolder", ...identity, folder: { kind: "handle", handleId: "h" } }] }));
      expect(readLocalFolderBindingsV1(device), "a log that is not the facet's reattaches nothing").toEqual({ bindings: [] });
    });

    it("a superseded or stale restore neither re-reads history nor refreshes a surface", async () => {
      const calls: string[] = [];
      const ports = { history: async () => { calls.push("history"); }, refresh: async () => { calls.push("refresh"); } };
      expect(await restoreDocumentArchiveV1("archive", () => true, { ...ports, load: async () => false })).toBe(false);
      expect(await restoreDocumentArchiveV1("archive", () => false, { ...ports, load: async () => true })).toBe(false);
      expect(calls).toEqual([]);
      let live = true;
      expect(await restoreDocumentArchiveV1("archive", () => live, { ...ports, load: async () => true, refresh: async () => { calls.push("refresh"); live = false; } })).toBe(false);
      expect(calls).toEqual(["history", "refresh"]);
    });
  });
}
