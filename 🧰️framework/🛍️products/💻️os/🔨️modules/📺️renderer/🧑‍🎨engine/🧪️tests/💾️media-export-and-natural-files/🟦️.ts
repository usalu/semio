/** 💾 Media export download and natural file codecs. */
import {
  DOWNLOAD_MEDIA_EXPORT_REVOKE_MS,
  downloadMediaExport,
  readBlobBytesBoundedV1,
  requestFileSelectionV1,
  mediaExportEncodingText
} from "../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import {
  naturalFileFormatV1,
  naturalFileNameV1,
  naturalMediaDescriptorMatchesV1,
  openNaturalFileOwnerV1
} from "../../🧱️elements/🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { OPEN_ARTIFACT_FILE_ACTION_ID, SAVE_ARTIFACT_FILE_ACTION_ID } from "@semio-tech/framework";
import naturalFileLifecycleFixture from "@semio-tech/framework-plugin/testing/fixtures/natural-file-lifecycle.json";
import { cleanup } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { type AppDefinition } from "@semio-tech/framework";
import "../🧰️engine-contract-helpers/🟦️.ts";

describe("framework renderer hosts", () => {
  afterEach(() => cleanup());

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
    const unsupported = { id: naturalFileLifecycleFixture.unsupported[0]!.appId, actions: [], io: { exportFormats: [], importFormats: [] }, windowKinds: [{ actions: [] }] } as unknown as AppDefinition;
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
});
