import { test, expect } from "bun:test";
import Ajv2020 from "ajv/dist/2020.js";
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
