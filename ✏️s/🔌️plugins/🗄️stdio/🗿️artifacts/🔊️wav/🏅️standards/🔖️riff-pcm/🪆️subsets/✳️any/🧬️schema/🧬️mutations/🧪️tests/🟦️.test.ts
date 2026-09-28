import { describe, expect, it } from "bun:test";
import type { WavMutation, WavSnapshot } from "../🟦️.ts";
import type { WavMutation as BinaryWavMutation } from "../💾️binary/🟦️.ts";
import type { WavMutation as TextWavMutation } from "../📝️text/🟦️.ts";
import { parseWavSnapshot } from "../../📸️snapshot/🟦️.ts";
import { parseWavDiff } from "../../🔺️diff/🟦️.ts";

type Equal<Left, Right> = (<Value>() => Value extends Left ? 1 : 2) extends <Value>() => Value extends Right ? 1 : 2 ? true : false;

const facets: readonly [Equal<WavMutation, TextWavMutation>, Equal<WavMutation, BinaryWavMutation>] = [true, true];
const boundaries = await Bun.file(new URL("../../../🧫️fixtures/🧭️serialization-boundaries/🔣️.json", import.meta.url)).json() as Readonly<{
  maximumFmtExtensionBytes: number;
  overflowFmtExtensionBytes: number;
  validFourcc: string;
  invalidFourcc: string;
  validOddFmtPadByte: number;
  validOddDataPadByte: number;
  validOddOtherPadByte: number;
  invalidEvenPadByte: number;
}>;
const fmt = { audioFormat: 1, channels: 1, sampleRate: 16_000, byteRate: 32_000, blockAlign: 2, bitsPerSample: 16 };
const data = { kind: "pcm16", value: [0, 2_000, -2_000, 0] } as const;
const snapshot: WavSnapshot = {
  schema: "stdio.wav",
  fmt: { ...fmt, ext: [9] },
  data: { kind: "raw", value: [0, 1, 2] },
  fmtPadByte: boundaries.validOddFmtPadByte,
  dataPadByte: boundaries.validOddDataPadByte,
  otherChunks: [{ fourcc: boundaries.validFourcc, data: [4, 0, 0], padByte: boundaries.validOddOtherPadByte }],
  chunkOrder: [{ kind: "format" }, { kind: "other", value: 0 }, { kind: "samples" }],
};
const operations = [
  { mutation: "setSnapshot", snapshot },
  { mutation: "patchSnapshot", patch: { edits: [{ path: ["fmt", "sampleRate"], edit: { operation: "set", value: 22_050 } }] } },
  { mutation: "setFmt", fmt },
  { mutation: "setData", data },
  { mutation: "patchData", index: 1, removeCount: 2, data: { kind: "pcm16", value: [3_000] }, moveTo: 0 },
  { mutation: "setOtherChunks", chunks: [{ fourcc: "LIST", data: [1, 2, 3, 4] }] },
] satisfies readonly WavMutation[];
const tags = ["setSnapshot", "patchSnapshot", "setFmt", "setData", "patchData", "setOtherChunks"] as const satisfies readonly WavMutation["mutation"][];
const fields: Readonly<Record<WavMutation["mutation"], readonly string[]>> = {
  setSnapshot: ["mutation", "snapshot"], patchSnapshot: ["mutation", "patch"], setFmt: ["mutation", "fmt"], setData: ["mutation", "data"], patchData: ["mutation", "index", "removeCount", "data", "moveTo"], setOtherChunks: ["mutation", "chunks"],
};

describe("WAV mutation TypeScript facets", () => {
  it("preserves the native tagged union across aggregate, text, and binary facets", () => {
    expect(facets).toEqual([true, true]);
    expect(operations.map(({ mutation }) => mutation)).toEqual(Array.from(tags));
    for (const operation of operations) expect(Object.keys(operation)).toEqual(Array.from(fields[operation.mutation]));
    expect(JSON.parse(JSON.stringify(operations))).toEqual(operations);
  });

  it("parses ordered chunks and their alignment bytes across snapshot and diff facets", () => {
    expect(parseWavSnapshot(snapshot)).toEqual(snapshot);
    expect(parseWavDiff({ fmtPadByte: boundaries.validOddFmtPadByte, dataPadByte: boundaries.validOddDataPadByte, otherChunks: snapshot.otherChunks, chunkOrder: snapshot.chunkOrder })).toEqual({
      fmtPadByte: boundaries.validOddFmtPadByte,
      dataPadByte: boundaries.validOddDataPadByte,
      otherChunks: snapshot.otherChunks,
      chunkOrder: snapshot.chunkOrder,
    });
  });

  it("rejects RIFF states that cannot save and reopen exactly", () => {
    expect(() => parseWavSnapshot({ ...snapshot, fmt, fmtPadByte: boundaries.invalidEvenPadByte })).toThrow(/fmtPadByte/u);
    expect(() => parseWavSnapshot({ ...snapshot, data, dataPadByte: boundaries.invalidEvenPadByte })).toThrow(/dataPadByte/u);
    expect(() => parseWavSnapshot({ ...snapshot, otherChunks: [{ fourcc: boundaries.validFourcc, data: [1, 2], padByte: boundaries.invalidEvenPadByte }] })).toThrow(/padByte/u);
    expect(() => parseWavSnapshot({ ...snapshot, otherChunks: [{ fourcc: boundaries.invalidFourcc, data: [1] }] })).toThrow(/fourcc/u);
    expect(() => parseWavSnapshot({ ...snapshot, fmt: { ...fmt, ext: Array.from({ length: boundaries.overflowFmtExtensionBytes }, () => 0) } })).toThrow(new RegExp(String(boundaries.maximumFmtExtensionBytes), "u"));
  });
});
