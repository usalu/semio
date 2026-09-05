// #region 🧲️Header
/** @emoji 🧪️ Focused segmented-download drain tests with fake producers and sinks. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { describe, expect, it } from "vitest";
import {
  createSegmentedDownloadSink,
  drainSegmentedMediaExport,
  MAX_SEGMENTED_DOWNLOAD_CHUNK_BYTES,
  parseSegmentedDownloadMarker,
  parseSegmentedDownloadOperationId,
  type SegmentedDownloadSink,
} from "../📤️SegmentedDownload/🟦️.ts";
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
    for (const invalid of [new Uint8Array(0), new Uint8Array(MAX_SEGMENTED_DOWNLOAD_CHUNK_BYTES + 1)]) {
      await expect(drainSegmentedMediaExport("x.bin", "application/octet-stream", "11", "semio-segmented-handle-v1:identity", async () => invalid, { sinkFactory: async () => fakeSink([], []) })).rejects.toThrow("segmented-download-chunk-limit");
    }
    const chunks = [new Uint8Array(2), new Uint8Array(2), undefined];
    await expect(drainSegmentedMediaExport("x.bin", "application/octet-stream", "12", "semio-segmented-handle-v1:identity", async () => chunks.shift(), { maximumBytes: 3, sinkFactory: async () => fakeSink([], []) })).rejects.toThrow("segmented-download-total-limit");
  });
});
//#endregion 🧵Drain
