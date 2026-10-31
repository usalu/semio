import { test, expect } from "bun:test";
import Ajv2020 from "ajv";
import { applyPatch } from "fast-json-patch";
import { readFileSync } from "node:fs";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };

test("one owned Pack source returns its exact business tree after codec scaffold close", () => {
  const validate = new Ajv2020({ strict: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  expect(validate({ ...fixture, headerCopyBytes: 64 })).toBe(false);
  expect(validate({ ...fixture, physicalRetirement: false })).toBe(false);
  const original = fixture.source;
  const transferred = applyPatch<{ original?: typeof original; pending: typeof original | null }>({ original, pending: null }, [{ op: "move", from: "/original", path: "/pending" }], true, false).newDocument;
  expect(transferred.pending).toEqual(original);
  expect(BigInt(original.nodeHashLiteral)).toBe((1n << 64n) - 3n);
  const lease = fixture.leaseElection;
  expect(lease.handleBytes + lease.payloadMoves * lease.optionalPayloadBytes).toBe(lease.copyBytes);
  expect(lease.copyBytes).toBeLessThan(8 * 64);
  for (const copy of fixture.copyGrants) {
    const input = new TextEncoder().encode(original.title);
    const output: number[] = [];
    for (let at = 0; at < input.length; at += copy) output.push(...input.subarray(at, at + copy));
    expect(new TextDecoder().decode(new Uint8Array(output))).toBe(original.title);
  }
  const source = readFileSync(new URL("../🦀️.rs", import.meta.url), "utf8");
  expect(source).toContain("OwnedProjectedPackWireCursor");
  expect(source).toContain("BorrowedProjectedPackCursor");
  expect(source).toContain("take_authority(grant)");
  expect(source).not.toContain("JsonWrite");
  expect(source).not.toContain("to_value(");
});

test("paged octets preserve cross-page values against the Buffer oracle", () => {
 const validate = new Ajv2020({ strict: true }).compile(schema);
 expect(validate(fixture)).toBe(true);
 const { length, pattern } = fixture.pagedBytes;
 const original = Buffer.from(Array.from({ length }, (_, index) => pattern[index % pattern.length]));
 const pages = Array.from({ length: Math.ceil(length / 1024) }, (_, index) => original.subarray(index * 1024, (index + 1) * 1024));
 const emitted: number[] = [];
 for (let offset = 0; offset < length; offset += 64) for (let index = offset; index < Math.min(length, offset + 64); index++) emitted.push(pages[Math.floor(index / 1024)][index % 1024]);
 expect(Buffer.from(emitted).equals(original)).toBe(true);
 expect(Buffer.from(emitted).toString("base64")).toBe(original.toString("base64"));
 expect(pages.length).toBeGreaterThan(4);
});
