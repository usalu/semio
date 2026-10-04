/** 🔢️ Checks exact unsigned magnitudes and strict absolute-source boundaries independently. */
import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { createRequire } from "node:module";
import Ajv from "ajv/dist/2020.js";

type Outcome = {valueDecimal: string; consumed: number} | {kind: "truncated" | "malformed"; offset: number};
type Row = {id: string; bodyHex: string; trailingHex: string; reads: number; outcome: Outcome};
type Corpus = {version: 1; prefixHex: string; offset: number; primitive: string; cases: Row[]};
const fixturePath = resolve(import.meta.dir, "🧫️fixtures/🔣️.json");
const schemaPath = resolve(import.meta.dir, "🧬️schema/🔣️.json");
const oracle = createRequire(import.meta.url)("@webassemblyjs/leb128") as {decodeUInt64(bytes: Uint8Array, offset: number): {value: {toString(): string}; nextIndex: number}};

test("the closed absolute unsigned corpus preserves all full-width boundaries", () => {
  expect(existsSync(fixturePath), "closed absolute unsigned varint corpus").toBe(true);
  expect(existsSync(schemaPath), "closed absolute unsigned varint schema").toBe(true);
  const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Corpus;
  const validate = new Ajv({strict: true}).compile(JSON.parse(readFileSync(schemaPath, "utf8")));
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(new Set(fixture.cases.map(row => row.id)).size).toBe(fixture.cases.length);
  const magnitudes = new Set<string>();
  const truncations = new Set<number>();
  for (const row of fixture.cases) {
    const body = Buffer.from(row.bodyHex, "hex"), trailing = Buffer.from(row.trailingHex, "hex");
    const bytes = Buffer.concat([Buffer.from(fixture.prefixHex, "hex"), body, trailing]);
    if ("valueDecimal" in row.outcome) {
      const observed = oracle.decodeUInt64(bytes, fixture.offset);
      expect(observed.value.toString(), row.id).toBe(row.outcome.valueDecimal);
      expect(observed.nextIndex, row.id).toBe(fixture.offset + row.outcome.consumed);
      expect(bytes.subarray(observed.nextIndex).toString("hex"), row.id).toBe(row.trailingHex);
      expect(row.reads, row.id).toBe(row.outcome.consumed);
      const magnitude = BigInt(observed.value.toString());
      expect(magnitude >= 0n && magnitude <= 18446744073709551615n).toBe(true);
      const fixed = Buffer.alloc(8); fixed.writeBigUInt64LE(magnitude);
      expect(new DataView(fixed.buffer, fixed.byteOffset, fixed.byteLength).getBigUint64(0, true)).toBe(magnitude);
      magnitudes.add(magnitude.toString());
    } else if (row.outcome.kind === "truncated") {
      expect(trailing.length, row.id).toBe(0);
      expect(row.reads, row.id).toBe(body.length + 1);
      expect(row.outcome.offset, row.id).toBe(bytes.length);
      expect([...body].every(byte => (byte & 128) !== 0), row.id).toBe(true);
      truncations.add(body.length);
    } else {
      expect(body.length, row.id).toBe(10);
      expect(row.reads, row.id).toBe(10);
      expect(body[9]! > 1, row.id).toBe(true);
      expect(row.outcome.offset, row.id).toBe((body[9]! & 128) !== 0 ? fixture.offset : 0);
    }
  }
  for (let width = 7n; width <= 63n; width += 7n) {
    expect(magnitudes.has(((1n << width) - 1n).toString())).toBe(true);
    expect(magnitudes.has((1n << width).toString())).toBe(true);
  }
  expect(magnitudes.has("0")).toBe(true);
  expect(magnitudes.has("1")).toBe(true);
  expect(magnitudes.has("18446744073709551615")).toBe(true);
  expect([...truncations].sort((a,b) => a-b)).toEqual([0,1,2,3,4,5,6,7,8,9]);
  expect(fixture.cases.some(row => row.bodyHex === "8000" && "valueDecimal" in row.outcome && row.outcome.valueDecimal === "0")).toBe(true);
  expect(validate({...fixture, uncheckedNumber: 18446744073709551615})).toBe(false);
  expect(validate({...fixture, cases: [{...fixture.cases[0], outcome: {valueDecimal: 0, consumed: 1}}]})).toBe(false);
});

test("the third-party permissive malformed policy stays separate from the strict first-party contract", () => {
  expect(oracle.decodeUInt64(Uint8Array.of(128), 0).nextIndex).toBe(2);
  expect(oracle.decodeUInt64(Uint8Array.from([255,255,255,255,255,255,255,255,255,2]), 0).value.toString()).toBe("9223372036854775807");
});
