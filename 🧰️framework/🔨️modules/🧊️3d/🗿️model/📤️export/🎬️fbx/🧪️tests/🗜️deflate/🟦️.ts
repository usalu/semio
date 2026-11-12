import { deflateSync, inflateSync } from "node:zlib";
import { unzlibSync } from "fflate";
import { describe, expect, it } from "vitest";
import { deflateZlib, inflateZlib } from "../../🗜️deflate/🟦️.ts";
import { fixture, sha256 } from "../🧰️support/🟦️.ts";

type Spec = { kind: "repeat"; pattern: number[]; count: number } | { kind: "lcg"; seed: number; length: number } | { kind: "float64-ramp"; count: number; step: number };
type Row = { name: string; spec: Spec; inputSha256: string; inputLength: number; zlibLength: number; zlibSha256: string };

function bytesOf(spec: Spec): Uint8Array {
  if (spec.kind === "repeat") return Uint8Array.from({ length: spec.pattern.length * spec.count }, (_, index) => spec.pattern[index % spec.pattern.length]);
  if (spec.kind === "lcg") {
    let state = spec.seed >>> 0;
    return Uint8Array.from({ length: spec.length }, () => ((state = (Math.imul(state, 1664525) + 1013904223) >>> 0), state >>> 24));
  }
  const view = new DataView(new ArrayBuffer(spec.count * 8));
  for (let index = 0; index < spec.count; index++) view.setFloat64(index * 8, index * spec.step, true);
  return new Uint8Array(view.buffer);
}

/** 🗜️ The owned zlib against two third-party implementations (node:zlib, fflate) and the language-agnostic vectors. */
describe("owned deflate", () => {
  const { rows } = fixture<{ rows: Row[] }>("deflate");
  for (const row of rows)
    it(`${row.name}: vector digest, third-party inflate, own inflate`, () => {
      const input = bytesOf(row.spec);
      expect(sha256(input)).toBe(row.inputSha256);
      const packed = deflateZlib(input);
      expect(packed.length).toBe(row.zlibLength);
      expect(sha256(packed)).toBe(row.zlibSha256);
      expect(Buffer.from(inflateSync(packed)).equals(Buffer.from(input))).toBe(true);
      expect(Buffer.from(unzlibSync(packed)).equals(Buffer.from(input))).toBe(true);
      expect(Buffer.from(inflateZlib(packed)).equals(Buffer.from(input))).toBe(true);
      expect(Buffer.from(inflateZlib(deflateSync(input))).equals(Buffer.from(input))).toBe(true);
      if (row.spec.kind !== "lcg" && input.length > 64) expect(packed.length).toBeLessThan(input.length);
    });

  it("uses fixed and dynamic Huffman blocks and never needs a stored fallback", () => {
    const types = new Set(rows.map((row) => (deflateZlib(bytesOf(row.spec))[2] >> 1) & 3));
    expect(types.has(1)).toBe(true);
    expect(types.has(2)).toBe(true);
    expect(types.has(0)).toBe(false);
  });

  it("round-trips pseudo-random inputs of many sizes through third-party inflate", () => {
    for (let length = 0; length < 70000; length = length < 300 ? length + 1 : Math.floor(length * 1.9)) {
      const input = bytesOf({ kind: "lcg", seed: length + 1, length });
      const mixed = Uint8Array.from(input, (byte, index) => (index % 7 < 4 ? byte & 3 : byte));
      for (const sample of [input, mixed]) {
        expect(Buffer.from(inflateSync(deflateZlib(sample))).equals(Buffer.from(sample))).toBe(true);
        expect(Buffer.from(inflateZlib(deflateZlib(sample))).equals(Buffer.from(sample))).toBe(true);
      }
    }
  });
});
