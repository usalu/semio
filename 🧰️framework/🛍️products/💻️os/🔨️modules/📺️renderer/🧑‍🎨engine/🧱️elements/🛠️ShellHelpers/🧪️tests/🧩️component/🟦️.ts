// #region 🧲️Header
/** @emoji 🧪️ Focused segmented-download drain tests with fake producers and sinks. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { describe, expect, it } from "vitest";
import {
  createSegmentedDownloadSink,
  drainSegmentedMediaExport,
  SEGMENTED_DOWNLOAD_CONTRACT,
  SEGMENTED_DOWNLOAD_REFUSAL,
  parseSegmentedDownloadMarker,
  parseSegmentedDownloadOperationId,
  segmentedDownloadSinkFactory,
  type SegmentedDownloadSink,
} from "../../../📤️SegmentedDownload/🟦️.ts";
import { buildActionCategoryTree, type ResolvedActionDefinition } from "../../🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🧱️Fixtures
function fakeSink(events: string[], output: number[]): SegmentedDownloadSink {
  return {
    write: async (chunk) => {
      events.push(`write:${chunk.byteLength}`);
      output.push(...chunk);
    },
    close: async () => {
      events.push("close");
    },
    abort: async () => {
      events.push("abort");
    },
  };
}

function bytes(value: string): Uint8Array {
  return new TextEncoder().encode(value);
}
//#endregion 🧱️Fixtures

//#region 🪪Marker
describe("segmented download marker", () => {
  it("accepts only exact version-one markers and canonical u64 operation ids", () => {
    expect(parseSegmentedDownloadMarker("semio-segmented-handle-v1:identity")).toBe("identity");
    expect(parseSegmentedDownloadMarker("semio-segmented-handle-v1:base64")).toBe("base64");
    expect(() => parseSegmentedDownloadMarker("base64")).toThrow("segmented-download-marker-invalid");
    expect(parseSegmentedDownloadOperationId("42")).toBe(42n);
    expect(parseSegmentedDownloadOperationId("9007199254740993")).toBe(9_007_199_254_740_993n);
    expect(parseSegmentedDownloadOperationId("18446744073709551615")).toBe(18_446_744_073_709_551_615n);
    expect(() => parseSegmentedDownloadOperationId("042")).toThrow("segmented-download-operation-invalid");
    expect(() => parseSegmentedDownloadOperationId("0")).toThrow("segmented-download-operation-invalid");
    expect(() => parseSegmentedDownloadOperationId("18446744073709551616")).toThrow("segmented-download-operation-invalid");
  });
});
//#endregion 🪪Marker

//#region 🧵Drain
describe("segmented download drain", () => {
  it("fails closed when the browser exposes no real streaming file sink", async () => {
    await expect(createSegmentedDownloadSink("x.bin", "application/octet-stream")).rejects.toThrow("segmented-download-streaming-sink-unavailable");
  });

  it("awaits chunks sequentially and preserves identity ordering", async () => {
    const events: string[] = [];
    const output: number[] = [];
    const chunks = [bytes("ab"), bytes("cd"), undefined];
    let reads = 0;
    let concurrent = 0;
    let peakConcurrent = 0;
    await drainSegmentedMediaExport("x.svg", "image/svg+xml", "7", "semio-segmented-handle-v1:identity", async (operationId) => {
      expect(operationId).toBe(7n);
      reads += 1;
      concurrent += 1;
      peakConcurrent = Math.max(peakConcurrent, concurrent);
      await Promise.resolve();
      concurrent -= 1;
      events.push(`read:${reads}`);
      return chunks.shift();
    }, { sinkFactory: async () => fakeSink(events, output) });
    expect(peakConcurrent).toBe(1);
    expect(events).toEqual(["read:1", "write:2", "read:2", "write:2", "read:3", "close"]);
    expect(new TextDecoder().decode(Uint8Array.from(output))).toBe("abcd");
  });

  it("decodes base64 across producer boundaries without reordering", async () => {
    const output: number[] = [];
    const chunks = [bytes("SG"), bytes("VsbG8="), undefined];
    await drainSegmentedMediaExport("x.bin", "application/octet-stream", "8", "semio-segmented-handle-v1:base64", async () => chunks.shift(), { sinkFactory: async () => fakeSink([], output) });
    expect(new TextDecoder().decode(Uint8Array.from(output))).toBe("Hello");
  });

  it("aborts the sink when cancellation lands during an awaited producer read", async () => {
    const controller = new AbortController();
    const events: string[] = [];
    let reads = 0;
    await expect(drainSegmentedMediaExport("x.bin", "application/octet-stream", "9", "semio-segmented-handle-v1:identity", async () => {
      reads += 1;
      controller.abort(new Error("cancelled-by-test"));
      return bytes("ignored");
    }, { signal: controller.signal, sinkFactory: async () => fakeSink(events, []) })).rejects.toThrow("cancelled-by-test");
    expect(reads).toBe(1);
    expect(events).toEqual(["abort"]);
  });

  it("rejects an unknown-operation error before the required None terminator", async () => {
    const events: string[] = [];
    let reads = 0;
    await expect(drainSegmentedMediaExport("x.bin", "application/octet-stream", "10", "semio-segmented-handle-v1:identity", async () => {
      reads += 1;
      if (reads === 1) return bytes("ok");
      throw new Error("interactive-job.unknown-segmented-download");
    }, { sinkFactory: async () => fakeSink(events, []) })).rejects.toThrow("interactive-job.unknown-segmented-download");
    expect(events).toEqual(["write:2", "abort"]);
  });

  it("fails closed on per-chunk and total-cap overflow", async () => {
    for (const [invalid, code] of [[new Uint8Array(0), SEGMENTED_DOWNLOAD_REFUSAL.chunkEmpty], [new Uint8Array(SEGMENTED_DOWNLOAD_CONTRACT.chunkBytes + 1), SEGMENTED_DOWNLOAD_REFUSAL.chunkOverCap]] as const) {
      await expect(drainSegmentedMediaExport("x.bin", "application/octet-stream", "11", "semio-segmented-handle-v1:identity", async () => invalid, { sinkFactory: async () => fakeSink([], []) })).rejects.toThrow(code);
    }
    const chunks = [new Uint8Array(2), new Uint8Array(2), undefined];
    await expect(drainSegmentedMediaExport("x.bin", "application/octet-stream", "12", "semio-segmented-handle-v1:identity", async () => chunks.shift(), { maximumBytes: 3, sinkFactory: async () => fakeSink([], []) })).rejects.toThrow(SEGMENTED_DOWNLOAD_REFUSAL.totalOverCap);
  });
});
//#endregion 🧵Drain

//#region 🧺️AssembledSink
describe("segmented download assembled sink", () => {
  /** 🧺️ The shell's own sink factory must turn a chunked producer into ONE delivered file, without the
   * File System Access API: `createSegmentedDownloadSink` fails closed wherever `showSaveFilePicker` is
   * absent, which used to make an over-budget export silence (ticket 26/09/02, wave B38). */
  it("assembles every chunk in order and delivers the payload once", async () => {
    const payload = "x".repeat(SEGMENTED_DOWNLOAD_CONTRACT.chunkBytes) + "TAIL";
    const pages = [bytes(payload.slice(0, SEGMENTED_DOWNLOAD_CONTRACT.chunkBytes)), bytes(payload.slice(SEGMENTED_DOWNLOAD_CONTRACT.chunkBytes)), undefined];
    const delivered: { filename: string; mimeType: string; text: string }[] = [];
    await drainSegmentedMediaExport("nakagin-capsule-tower.json", "application/json", "7", "semio-segmented-handle-v1:identity", async () => pages.shift(), {
      sinkFactory: segmentedDownloadSinkFactory((filename, mimeType, assembled) => delivered.push({ filename, mimeType, text: new TextDecoder().decode(assembled) })),
    });
    expect(delivered).toEqual([{ filename: "nakagin-capsule-tower.json", mimeType: "application/json", text: payload }]);
  });

  /** 🧯️ A drain that aborts delivers NOTHING: a truncated payload must never reach the user as a file
   * that looks complete. */
  it("delivers nothing when the drain aborts", async () => {
    const delivered: string[] = [];
    const factory = segmentedDownloadSinkFactory((filename) => delivered.push(filename));
    await expect(
      drainSegmentedMediaExport("x.json", "application/json", "8", "semio-segmented-handle-v1:identity", async () => {
        throw new Error("interactive-job.unknown-segmented-download");
      }, { sinkFactory: factory }),
    ).rejects.toThrow("interactive-job.unknown-segmented-download");
    expect(delivered).toEqual([]);
  });
});
//#endregion 🧺️AssembledSink

//#region 🧰️ActionPaneGate
describe("actions pane utility gate", () => {
  const action = (id: string, kind: "history" | "mutation"): ResolvedActionDefinition =>
    ({ id, label: id, kind, inPalette: true, args: [] }) as unknown as ResolvedActionDefinition;
  const rows = (disabled: boolean) => {
    const executed: string[] = [];
    const sections = buildActionCategoryTree("2d-overview", "puzzle2d-play", [action("undo", "history"), action("deleteSelection", "mutation")], null, {}, disabled, () => {}, () => {}, () => {}, (descriptor) => executed.push(descriptor.action));
    const items = sections.flatMap((section) => section.items ?? []);
    return { executed, byId: new Map(items.map((item) => [item.id, item])) };
  };

  /** 🧯️ An armed utility gates the APP's verbs and nothing else. `mod+z` undoes while a brush is armed
   * (the chord is bound outside this pane), so a pane that refuses `#action.undo` at the same moment
   * contradicts its own keybinding — that contradiction made the row measure as inert with no fault
   * (2026-09-17 ◻️2d battery, `undo-action-row-changes-document`). */
  it("keeps framework-reserved rows pressable while an armed utility gates the app's own verbs", () => {
    const gated = rows(true);
    expect(gated.byId.get("action.deleteSelection")?.className).toContain("pointer-events-none");
    expect(gated.byId.get("action.undo")?.className).toBeUndefined();
    gated.byId.get("action.deleteSelection")?.onClick?.();
    gated.byId.get("action.undo")?.onClick?.();
    expect(gated.executed).toEqual(["undo"]);
  });

  /** ✅️ With no utility armed nothing is gated at all — the fix narrows the gate, it does not remove it. */
  it("presses every row when no utility is armed", () => {
    const open = rows(false);
    open.byId.get("action.deleteSelection")?.onClick?.();
    open.byId.get("action.undo")?.onClick?.();
    expect(open.executed).toEqual(["deleteSelection", "undo"]);
  });
});
//#endregion 🧰️ActionPaneGate
