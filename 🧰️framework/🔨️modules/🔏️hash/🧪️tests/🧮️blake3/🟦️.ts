/** 🧮️ Law of the first-party BLAKE3 twin (`../../🟦️.ts`) against vectors of the BLAKE3 team's own implementation
 * (`../../🧫️fixtures/🧮️blake3/🔣️.json`: the dev-only `blake3` crate 1.8.7, input byte i = i % 251):
 * the 32-byte hash and the 131-byte extended output at every chunk and tree edge length, hashed in one update
 * and in uneven, unaligned update runs. */
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { Blake3Hasher, blake3Hex } from "../../🟦️.ts";

type Blake3VectorsV1 = Readonly<{ schema: "semio.hash.blake3-vectors/v1"; source: string; vectors: readonly Readonly<{ length: number; hash: string; xof131: string }>[] }>;

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🧮️blake3/🔣️.json", import.meta.url), "utf8")) as Blake3VectorsV1;
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🧮️blake3/🔣️.json", import.meta.url), "utf8"));
  const input = (length: number): Uint8Array => Uint8Array.from({ length }, (_, index) => index % 251);
  const hex = (bytes: Uint8Array): string => Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");

  describe("🔏️ BLAKE3 twin", () => {
    it("admits the closed independent vector corpus", () => {
      const validate = new Ajv({ strict: true }).compile(schema);
      expect(validate(fixture)).toBe(true);
      expect(validate({ ...fixture, unknown: true })).toBe(false);
      expect(validate({ ...fixture, vectors: fixture.vectors.slice(1) })).toBe(false);
      expect(new Set(fixture.vectors.map((vector) => vector.length)).size).toBe(fixture.vectors.length);
    });

    it("carries the chunk and tree edges", () => {
      expect(fixture.schema).toBe("semio.hash.blake3-vectors/v1");
      expect(fixture.vectors.map((vector) => vector.length)).toEqual(expect.arrayContaining([0, 1, 64, 65, 1024, 1025, 2048, 2049, 4097, 8193, 1048577]));
    });

    it.each(fixture.vectors.map((vector) => [vector.length, vector] as const))("hashes %i bytes like the BLAKE3 team's implementation", (_length, vector) => {
      const bytes = input(vector.length);
      expect(blake3Hex(bytes)).toBe(vector.hash);
      const hasher = new Blake3Hasher();
      hasher.update(bytes);
      expect(hex(hasher.digest(131))).toBe(vector.xof131);
    });

    it.each(fixture.vectors.map((vector) => [vector.length, vector] as const))("hashes %i bytes fed in uneven, unaligned runs", (_length, vector) => {
      const padded = new Uint8Array(vector.length + 3);
      padded.set(input(vector.length), 3);
      const bytes = padded.subarray(3);
      const hasher = new Blake3Hasher();
      for (let offset = 0, step = 1; offset < bytes.length; offset += step, step = (step * 7 + 5) % 1500 || 1) hasher.update(bytes.subarray(offset, Math.min(bytes.length, offset + step)));
      expect(hex(hasher.digest())).toBe(vector.hash);
    });
  });
}
