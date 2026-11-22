import fileOpenLifecycleFixture from "./file-open.json";
import originalMediaCases from "./original-media.json";
import { Blob as NodeBlob } from "node:buffer";
import { pickOriginalMedia } from "../../🧱️elements/🔌️PluginRuntime/🚚️handle-turn-drive/original-media-pick.ts";
import type { OriginalMediaFrame } from "../../🧱️elements/🔌️PluginRuntime/🚚️handle-turn-drive/original-media-sender.ts";
/** 💾 Media export download and natural file codecs. */
import {
  DOWNLOAD_MEDIA_EXPORT_REVOKE_MS,
  downloadMediaExport,
  readBlobBytesBoundedV1,
  readSelectedFilesBoundedV1,
  requestFileSelectionV1,
  requestFileOpen,
  mediaExportEncodingText
} from "../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import {
  naturalFileFormatV1,
  naturalFileNameV1,
  naturalMediaDescriptorMatchesV1,
  naturalFileMediaInputV1,
  openNaturalFileOwnerV1
} from "../../🧱️elements/🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { OPEN_ARTIFACT_FILE_ACTION_ID, SAVE_ARTIFACT_FILE_ACTION_ID } from "@semio-tech/framework";
import naturalFileLifecycleFixture from "@semio-tech/framework-plugin/testing/fixtures/natural-file-lifecycle.json";
import Ajv from "ajv";
import lifecycleSchema from "../../../../🔌️plugin/🧪️tests/🧫️fixtures/📄️natural-file-lifecycle/📄️lifecycle.schema.json";
import { encodeAppCommand } from "@semio-tech/framework/app-channel";
import { createTurnPump } from "../../🧱️elements/🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts";
import type { WireTurnResult } from "../../🧱️elements/🔌️PluginRuntime/🪝️wire-codec/🟦️.ts";
import { cleanup } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { type AppDefinition } from "@semio-tech/framework";
import "../🧰️engine-contract-helpers/🟦️.ts";

describe("framework renderer hosts", () => {
  afterEach(() => cleanup());

  it.each(originalMediaCases)("$name", async ({ mode, frames, progress, finished, result }) => {
    const body = Uint8Array.from({ length: 4097 }, (_, index) => index % 251);
    const oracle = new NodeBlob([body]);
    const request = {
      kind: 1, maximumFileBytes: body.byteLength,
      loan: { operationId: "1".repeat(16), generation: "2".repeat(16), instanceId: 1, documentId: "note", baseRevision: "3".repeat(64) },
    };
    const source = {
      name: "note.png", type: "image/png", size: body.byteLength,
      slice: (start: number, end: number) => ({ arrayBuffer: async () => body.slice(start, end).buffer }),
    };
    const controller = new AbortController();
    const abort = new Error("fixture cancelled");
    const retirement = new Error("fixture retirement failed");
    const observedFrames: OriginalMediaFrame["kind"][] = [];
    const observedProgress: number[] = [];
    const finish = vi.fn();
    let current = true;

    const picked = pickOriginalMedia(request, {
      current: () => true,
      async cross(frame, bytes, owned) {
        expect(owned()).toBe(true);
        expect(frame.loan).toEqual(request.loan);
        observedFrames.push(frame.kind);
        if (frame.kind === "page") {
          const start = frame.index * 4096;
          expect(bytes).toEqual(new Uint8Array(await oracle.slice(start, start + frame.byteLength).arrayBuffer()));
        }
        if (frame.kind === "cancel" && mode === "failed-retirement") throw retirement;
      },
    }, async () => {
      if (mode === "stale-selection") {
        current = false;
        throw new Error("fixture selection owner retired");
      }
      return mode === "empty-selection" ? [] : [source];
    }, () => current, () => ({
      signal: controller.signal,
      progress(completed, total) {
        expect(total).toBe(body.byteLength);
        observedProgress.push(completed);
        if (mode === "abort" || mode === "failed-retirement") controller.abort(abort);
      },
      finish,
    }));

    if (mode === "failed-retirement") {
      await expect(picked).rejects.toMatchObject({ message: result, errors: [abort, retirement] });
    } else {
      await expect(picked).resolves.toBe(result);
    }
    expect(observedFrames).toEqual(frames);
    expect(observedProgress).toEqual(progress);
    expect(finish).toHaveBeenCalledTimes(finished);
  });

  it("forwards cancellation into the admitted import and awaits owner retirement", async () => {
    const fixture = naturalFileLifecycleFixture.openCancellation;
    expect(new Ajv().compile(lifecycleSchema)(naturalFileLifecycleFixture)).toBe(true);
    const { promise: admitted, resolve: admit } = Promise.withResolvers<void>();
    const { promise: closed, resolve: finishRetirement } = Promise.withResolvers<void>();
    const controller = new AbortController();
    const instance = naturalFileLifecycleFixture.lifecycle.openedInstanceId;
    const order: string[] = [];
    const opened = openNaturalFileOwnerV1({
      signal: controller.signal,
      create: async () => { order.push("open"); return instance; },
      importBytes: (id, signal) => {
        expect(id).toBe(instance);
        admit();
        expect(signal).toBe(controller.signal);
        order.push("submit");
        return new Promise((_, reject) => signal.addEventListener("abort", () => {
          order.push("cancel");
          reject(signal.reason);
        }, { once: true }));
      },
      retire: async (id) => {
        expect(id).toBe(instance);
        order.push("retire");
        await closed;
        order.push("closed");
      },
    });
    const result = expect(opened).rejects.toBe(fixture.reason);
    await admitted;
    controller.abort(fixture.reason);
    await Promise.resolve();
    expect(order).toEqual(fixture.expectedOrder.slice(0, -1));
    finishRetirement();
    await result;
    expect(order).toEqual(fixture.expectedOrder);
  });

  it("acknowledges the current media crossing before abort and runs the queued command for the same actor", async () => {
    const fixture = naturalFileLifecycleFixture.openCancellation;
    const instance = naturalFileLifecycleFixture.lifecycle.openedInstanceId;
    const controller = new AbortController();
    const actorId = "natural-file-cancel-law#" + instance;
    const { promise: admitted, resolve: admit } = Promise.withResolvers<void>();
    const { promise: receipt, resolve: acknowledge } = Promise.withResolvers<WireTurnResult>();
    const turn = (ingress: string): WireTurnResult => ({ uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" }, commandIngress: { tag: ingress } });
    const observed: string[] = [];
    const acknowledgement = { kind: "message", payload: { source: { tag: "shell", val: instance }, payload: [1] } };
    let acknowledgedBeforeAbort = false;
    const publish = vi.fn((outcome: unknown) => {
      if (typeof outcome === "object" && outcome !== null && "error" in outcome) acknowledgedBeforeAbort = observed.includes("acknowledge");
    });
    const pump = createTurnPump(
      {
        pluginId: "natural-file-cancel-law",
        shardClient: { captureActorActivation: () => ({ assertActive: () => {} }) },
        state: { eventSeq: 0 },
        closingInstances: new Set(),
        actorIdByInstance: new Map([[instance, actorId]]),
        documentBindings: new Map(),
        turnOutcomes: { push: publish },
      } as unknown as Parameters<typeof createTurnPump>[0],
      {
        requireActorId: () => actorId,
        submitTurn: async (_actor, events, options) => {
          const seq = options?.commandPage?.cursor.seq;
          if (seq === undefined && events.length > 0) {
            expect(events).toEqual([acknowledgement]);
            observed.push("acknowledge");
            return turn("idle");
          }
          observed.push(seq === 1n ? "media-in" : seq === 2n ? "read-document" : "continuation");
          if (seq === 1n) { admit(); return receipt; }
          return turn("command-complete");
        },
      } as Parameters<typeof createTurnPump>[1],
      {
        acceptUiPatches: async (_instanceId, received) => ({
          acknowledgements: received.commandIngress?.tag === "page-accepted" ? [acknowledgement] : [],
          turns: [],
        }),
      } as Parameters<typeof createTurnPump>[2],
      {
        routeHostEffects: () => [],
        flushReservedJobDrives: async () => {},
        commitReservedToolSpawnsWhileSerialized: async () => {},
      } as Parameters<typeof createTurnPump>[3],
    );
    const media = encodeAppCommand({ MediaIn: { seq: 1, port: "artifact:native", descriptor: [], data: naturalFileLifecycleFixture.transport.octets } });
    const read = encodeAppCommand({ ReadDocument: { seq: 2 } });
    const importing = pump.runQueuedTurn(instance, [media], undefined, controller.signal);
    await admitted;
    const reading = pump.runQueuedTurn(instance, [read], undefined, controller.signal);
    controller.abort(fixture.reason);
    acknowledge({ ...turn("page-accepted"), status: { tag: "more-work" } });
    await Promise.all([importing, reading]);
    expect(publish.mock.calls).toEqual([[{ instanceId: instance, error: fixture.reason }], [{ instanceId: instance, frames: [] }]]);
    expect(observed).toEqual(fixture.expectedCrossings);
    expect(acknowledgedBeforeAbort).toBe(true);
  });

  it("preserves the import failure when retiring the fresh owner also fails", async () => {
    const fixture = naturalFileLifecycleFixture.openFailure;
    const primary = new Error(fixture.import);
    const retirement = new Error(fixture.retirement);
    const retire = vi.fn<(instance: number) => Promise<void>>().mockRejectedValue(retirement);
    const instance = naturalFileLifecycleFixture.lifecycle.openedInstanceId;
    await expect(openNaturalFileOwnerV1({
      signal: new AbortController().signal,
      create: async () => instance,
      importBytes: async () => { throw primary; },
      retire,
    })).rejects.toMatchObject({cause: primary, errors: [primary, retirement]});
    expect(retire).toHaveBeenCalledExactlyOnceWith(instance);
  });

  it("treats a non-string media-export encoding as a one-shot download, not a segmented marker", () => {
    expect(mediaExportEncodingText("utf-8")).toBe("utf-8");
    expect(mediaExportEncodingText({ some: "utf-8" })).toBeUndefined();
    expect(mediaExportEncodingText(undefined)).toBeUndefined();
  });

  it("appends a download anchor and keeps the object URL alive past the click turn", () => {
    const revoked: string[] = [];
    const create = URL.createObjectURL;
    const revoke = URL.revokeObjectURL;
    URL.createObjectURL = () => "blob:export-snapshot";
    URL.revokeObjectURL = (url) => {
      revoked.push(String(url));
    };
    try {
      downloadMediaExport("puzzle-3d.json", "application/json", "{}", "utf-8");
      expect(DOWNLOAD_MEDIA_EXPORT_REVOKE_MS).toBeGreaterThanOrEqual(1000);
      expect(document.querySelector("a[download='puzzle-3d.json']")).not.toBeNull();
      expect(revoked).toEqual([]);
    } finally {
      URL.createObjectURL = create;
      URL.revokeObjectURL = revoke;
    }
  });

  it("refuses a picked PDF outside its encoded command extent before reading", async () => {
    const law = naturalFileLifecycleFixture.wireIngress;
    const input = naturalFileMediaInputV1(law.formatKind);
    expect(input.maximumBytes).toBe(law.browserPayloadBytes);
    const slice = vi.fn();
    const oversized = { size: input.maximumBytes + 1, slice } as unknown as Blob;
    await expect(readBlobBytesBoundedV1(oversized, {
      signal: new AbortController().signal, maximumBytes: input.maximumBytes, chunkBytes: 4096,
    })).rejects.toThrow();
    expect(slice).not.toHaveBeenCalled();
  });

  it("mounts exact natural codecs and opens bytes into an isolated owner", async () => {
    const definition = (row: (typeof naturalFileLifecycleFixture.formats)[number]): AppDefinition => {
      const args = [
        { id: "formatKind", default: row.formatKind },
        { id: "extension", default: row.extension },
        { id: "mediaType", default: row.mediaType },
        { id: "binary", default: row.binary },
      ];
      return {
        id: row.appId,
        actions: [],
        io: { exportFormats: [row.formatKind], importFormats: [row.formatKind] },
        windowKinds: [{ actions: [{ id: SAVE_ARTIFACT_FILE_ACTION_ID, args }, { id: OPEN_ARTIFACT_FILE_ACTION_ID, args }] }],
      } as unknown as AppDefinition;
    };
    for (const row of naturalFileLifecycleFixture.formats) {
      const format = { formatKind: row.formatKind, extension: row.extension, mediaType: row.mediaType, binary: row.binary };
      expect(naturalFileFormatV1(definition(row))).toEqual(format);
      expect(naturalFileNameV1(row.appId, row.extension, new Date("2026-10-03T12:34:56.000Z"))).toMatch(new RegExp(`20261003T123456Z\\${row.extension}$`, "u"));
      expect(naturalMediaDescriptorMatchesV1({ portId: "artifact:native", kindId: row.formatKind, wire: { kind: "binary", format_kind: row.formatKind } }, format)).toBe(true);
      expect(naturalMediaDescriptorMatchesV1({ portId: "artifact:native", kindId: row.formatKind, wire: { Binary: { format_kind: row.formatKind } } }, format)).toBe(false);
    }
    const unsupported = { id: "s.fixture.unsupported@1/*#editor", actions: [], io: { exportFormats: [], importFormats: [] }, windowKinds: [{ actions: [] }] } as unknown as AppDefinition;
    expect(naturalFileFormatV1(unsupported)).toBeNull();
    const matched = definition(naturalFileLifecycleFixture.formats[0]!);
    const mismatched = { ...matched, io: { ...matched.io, importFormats: [] } } as AppDefinition;
    expect(naturalFileFormatV1(mismatched)).toBeNull();

    const currentHistory = ["edit"];
    const openedHistory: string[] = [];
    const retired: number[] = [];
    const signal = new AbortController();
    const opened = await openNaturalFileOwnerV1({
      signal: signal.signal,
      create: async () => naturalFileLifecycleFixture.lifecycle.openedInstanceId,
      importBytes: async (instanceId) => {
        expect(instanceId).toBe(naturalFileLifecycleFixture.lifecycle.openedInstanceId);
      },
      retire: async (instanceId) => {
        retired.push(instanceId);
      },
    });
    expect({
      saveInstanceId: naturalFileLifecycleFixture.lifecycle.currentInstanceId,
      preservedInstanceId: naturalFileLifecycleFixture.lifecycle.currentInstanceId,
      openedInstanceId: opened,
      openedHistoryEntries: openedHistory.length,
    }).toEqual(naturalFileLifecycleFixture.lifecycle.expected);
    expect(currentHistory).toEqual(["edit"]);
    expect(retired).toEqual([]);

    const cancelled = new AbortController();
    const cancelledRetirements: number[] = [];
    await expect(openNaturalFileOwnerV1({
      signal: cancelled.signal,
      create: async () => naturalFileLifecycleFixture.lifecycle.openedInstanceId,
      importBytes: async () => cancelled.abort(),
      retire: async (instanceId) => {
        cancelledRetirements.push(instanceId);
      },
    })).rejects.toBeDefined();
    expect(cancelledRetirements).toEqual([naturalFileLifecycleFixture.lifecycle.openedInstanceId]);

    const read = naturalFileLifecycleFixture.browserRead;
    const source = new Blob([Uint8Array.from(read.octets)]);
    const progress: number[] = [];
    const loaded = await readBlobBytesBoundedV1(source, {
      signal: new AbortController().signal,
      maximumBytes: read.maximumBytes,
      chunkBytes: read.chunkBytes,
      progress: (completed) => progress.push(completed),
    });
    expect([...loaded]).toEqual(read.octets);
    expect(progress).toEqual(read.expectedProgress);
    await expect(readBlobBytesBoundedV1(source, {
      signal: new AbortController().signal,
      maximumBytes: read.octets.length - 1,
      chunkBytes: read.chunkBytes,
      progress: () => {},
    })).rejects.toThrow("natural-file.read-limit");

    const interrupted = new AbortController();
    await expect(readBlobBytesBoundedV1(source, {
      signal: interrupted.signal,
      maximumBytes: read.maximumBytes,
      chunkBytes: read.chunkBytes,
      progress: (completed) => {
        if (completed === read.cancelAfterBytes) interrupted.abort("fixture cancellation");
      },
    })).rejects.toBe("fixture cancellation");
    const retried = await readBlobBytesBoundedV1(source, {
      signal: new AbortController().signal,
      maximumBytes: read.maximumBytes,
      chunkBytes: read.chunkBytes,
      progress: () => {},
    });
    expect([...retried]).toEqual(read.octets);

    const failedBlob = { size: 1, slice: () => ({ arrayBuffer: async () => { throw new Error("fixture read fault"); } }) } as unknown as Blob;
    await expect(readBlobBytesBoundedV1(failedBlob, {
      signal: new AbortController().signal,
      maximumBytes: read.maximumBytes,
      chunkBytes: read.chunkBytes,
      progress: () => {},
    })).rejects.toThrow("natural-file.read-failed");

    const picker = document.createElement("input");
    const createElement = vi.spyOn(document, "createElement").mockReturnValueOnce(picker);
    const click = vi.spyOn(picker, "click").mockImplementation(() => {});
    const remove = vi.spyOn(picker, "remove");
    const selection = requestFileSelectionV1(".csv,text/csv");
    picker.dispatchEvent(new Event("cancel"));
    expect(await selection).toEqual([]);
    expect(picker.onchange).toBeNull();
    expect(picker.oncancel).toBeNull();
    expect(click).toHaveBeenCalledOnce();
    expect(remove).toHaveBeenCalledOnce();
    createElement.mockRestore();
    click.mockRestore();
    remove.mockRestore();

    const retryPicker = document.createElement("input");
    const retryFile = new File([Uint8Array.from(read.octets)], "retry.csv", { type: "text/csv" });
    Object.defineProperty(retryPicker, "files", { configurable: true, value: [retryFile] });
    const retryCreateElement = vi.spyOn(document, "createElement").mockReturnValueOnce(retryPicker);
    const retryClick = vi.spyOn(retryPicker, "click").mockImplementation(() => {});
    const retrySelection = requestFileSelectionV1(".csv,text/csv");
    retryPicker.dispatchEvent(new Event("change"));
    expect(await retrySelection).toEqual([retryFile]);
    retryCreateElement.mockRestore();
    retryClick.mockRestore();
  });

  it("settles and retires the custom Lowpoly picker on every empty terminal path", async () => {
    for (const terminal of fileOpenLifecycleFixture.terminalEvents) {
      const picker = document.createElement("input");
      const createElement = vi.spyOn(document, "createElement").mockReturnValueOnce(picker);
      const click = vi.spyOn(picker, "click").mockImplementation(() => {
        if (terminal === "clickFailure") throw new Error("fixture picker unavailable");
      });
      const remove = vi.spyOn(picker, "remove");
      try {
        const opened = requestFileOpen(fileOpenLifecycleFixture.accept, fileOpenLifecycleFixture.readAs, fileOpenLifecycleFixture.multiple);
        if (terminal !== "clickFailure") picker.dispatchEvent(new Event(terminal === "empty" ? "change" : "cancel"));
        const observed = await Promise.race([
          opened.then(files => ({ kind: "settled", files })),
          new Promise(resolve => setTimeout(() => resolve({ kind: "pending" }), 0)),
        ]);
        expect(observed).toEqual({ kind: "settled", files: [] });
        expect(picker.onchange).toBeNull();
        expect(picker.oncancel).toBeNull();
        expect(remove).toHaveBeenCalledOnce();
        picker.dispatchEvent(new Event("cancel"));
        expect(remove).toHaveBeenCalledOnce();
      } finally {
        createElement.mockRestore();
        click.mockRestore();
        remove.mockRestore();
      }
    }
  });

  it("keeps selected custom files ordered with exact names and dataURLs", async () => {
    const picker = document.createElement("input");
    const files = fileOpenLifecycleFixture.files.map(file => new File([file.text], file.name, { type: file.type }));
    Object.defineProperty(picker, "files", { configurable: true, value: files });
    const createElement = vi.spyOn(document, "createElement").mockReturnValueOnce(picker);
    const click = vi.spyOn(picker, "click").mockImplementation(() => {});
    const remove = vi.spyOn(picker, "remove");
    try {
      const opened = requestFileOpen(fileOpenLifecycleFixture.accept, fileOpenLifecycleFixture.readAs, fileOpenLifecycleFixture.multiple);
      expect(picker.accept).toBe(fileOpenLifecycleFixture.accept);
      expect(picker.multiple).toBe(fileOpenLifecycleFixture.multiple);
      picker.dispatchEvent(new Event("change"));
      expect(await opened).toEqual(fileOpenLifecycleFixture.files.map(({ name, contents }) => ({ name, contents })));
      expect(picker.onchange).toBeNull();
      expect(picker.oncancel).toBeNull();
      expect(remove).toHaveBeenCalledOnce();
    } finally {
      createElement.mockRestore();
      click.mockRestore();
      remove.mockRestore();
    }
  });

  it("checks every file size before reading and emits the exact admitted bytes", async () => {
    const bound = fileOpenLifecycleFixture.bound;
    const read = vi.spyOn(Blob.prototype, "slice");
    const control = { signal: new AbortController().signal, progress: vi.fn() };
    const within = new File([Uint8Array.from(bound.octets)], bound.name, { type: "application/x-fixture" });
    const above = new File([new Uint8Array(bound.maximumBytes + 1)], bound.oversized);
    try {
      await expect(readSelectedFilesBoundedV1([within, above], "dataUrl", bound.maximumBytes, control)).rejects.toMatchObject({
        fileName: bound.oversized, size: bound.maximumBytes + 1, maximumBytes: bound.maximumBytes,
      });
      expect(read).not.toHaveBeenCalled();
      const accepted = await readSelectedFilesBoundedV1([within], "dataUrl", bound.maximumBytes, control);
      expect(accepted).toEqual([{ name: bound.name, contents: `data:application/octet-stream;base64,${Buffer.from(bound.octets).toString("base64")}` }]);
      expect(control.progress).toHaveBeenLastCalledWith(bound.octets.length, bound.octets.length);
      const abort = new AbortController();
      abort.abort();
      await expect(readSelectedFilesBoundedV1([within], "dataUrl", bound.maximumBytes, { ...control, signal: abort.signal })).rejects.toMatchObject({ name: "AbortError" });
      expect(read).toHaveBeenCalledTimes(1);
    } finally {
      read.mockRestore();
    }
  });
});
