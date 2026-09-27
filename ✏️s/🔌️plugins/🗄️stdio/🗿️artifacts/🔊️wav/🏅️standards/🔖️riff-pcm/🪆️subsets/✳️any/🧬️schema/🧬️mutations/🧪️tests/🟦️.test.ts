import { describe, expect, it } from "bun:test";
import type { WavMutation, WavSnapshot } from "../🟦️.ts";
import type { WavMutation as BinaryWavMutation } from "../💾️binary/🟦️.ts";
import type { WavMutation as TextWavMutation } from "../📝️text/🟦️.ts";

type Equal<Left, Right> = (<Value>() => Value extends Left ? 1 : 2) extends <Value>() => Value extends Right ? 1 : 2 ? true : false;

const facets: readonly [Equal<WavMutation, TextWavMutation>, Equal<WavMutation, BinaryWavMutation>] = [true, true];
const fmt = { audioFormat: 1, channels: 1, sampleRate: 16_000, byteRate: 32_000, blockAlign: 2, bitsPerSample: 16 };
const data = { kind: "pcm16", value: [0, 2_000, -2_000, 0] } as const;
const snapshot: WavSnapshot = { schema: "stdio.wav", fmt, data, otherChunks: [{ fourcc: "fact", data: [4, 0, 0, 0] }] };
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
});
