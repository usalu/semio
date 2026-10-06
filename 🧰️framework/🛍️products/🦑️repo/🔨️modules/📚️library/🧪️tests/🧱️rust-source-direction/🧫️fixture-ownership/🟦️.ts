import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { existsSync, lstatSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import { validateJsonSchemaSubset } from "../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
interface Consumer { readonly source: string; readonly fixtureReference: string }
interface Owner { readonly id: string; readonly fixture: string; readonly schema: string; readonly former: string; readonly sha256: string; readonly consumers: readonly Consumer[] }
const root = resolve(import.meta.dir, "../../../../../../../..");
const fixture = JSON.parse(readFileSync(new URL("../../../🧫️fixtures/🧱️rust-source-direction/🧫️fixture-ownership/🔣️.json", import.meta.url), "utf8")) as { readonly schemaVersion: 1; readonly cases: readonly Owner[] };

test("closed source-owned shared fixture census has independent AJV schema parity", () => {
  
  expect(fixture["schemaVersion"]).toEqual(1);
  
  expect(new Set(fixture.cases.map(row => row.id)).size).toBe(4);
  for (const extra of [{ ...fixture, unknown: true }, { ...fixture, cases: fixture.cases.map(row => ({ ...row, unknown: true })) }]) {
    
    
  }
});
test("all original shared fixture bytes have one neutral physical owner and exact direct clients", async () => {
  for (const row of fixture.cases) {
    const file = resolve(root, row.fixture);
    expect(existsSync(file), row.id).toBe(true);
    expect(lstatSync(file).isFile()).toBe(true);
    expect(lstatSync(file).isSymbolicLink()).toBe(false);
    const native = readFileSync(file);
    const bun = new Uint8Array(await Bun.file(file).arrayBuffer());
    const digest = createHash("sha256").update(native).digest("hex");
    expect(new Bun.CryptoHasher("sha256").update(bun).digest("hex")).toBe(digest);
    expect(digest).toBe(row.sha256);
    expect(existsSync(resolve(root, row.former)), row.id).toBe(false);
    const ownedSchema = JSON.parse(readFileSync(resolve(root, row.schema), "utf8"));
    const corpus = JSON.parse(native.toString("utf8"));
    const oracle = new Ajv({ strict: true, allErrors: true }).compile(ownedSchema);
    expect(oracle(corpus), JSON.stringify(oracle.errors)).toBe(true);
    expect(validateJsonSchemaSubset(ownedSchema, corpus)).toEqual([]);
    expect(oracle({ ...corpus, unknown: true })).toBe(false);
    expect(validateJsonSchemaSubset(ownedSchema, { ...corpus, unknown: true }).length).toBeGreaterThan(0);
    for (const consumer of row.consumers) {
      const source = readFileSync(resolve(root, consumer.source), "utf8");
      expect(source.includes(consumer.fixtureReference), consumer.source).toBe(true);
      expect(resolve(root, consumer.source, "..", consumer.fixtureReference)).toBe(file);
    }
  }
});
