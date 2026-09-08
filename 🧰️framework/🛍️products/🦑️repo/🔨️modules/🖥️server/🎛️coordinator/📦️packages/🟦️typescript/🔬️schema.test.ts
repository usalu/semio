//#region 🔌️Adapters
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { COORDINATOR_PARSERS, COORDINATOR_SCHEMA_ID, parseG3EventEnvelope, parseG3EventLogContract } from "../../🧬️schema/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧫️Fixtures
const ownerRoot = join(import.meta.dirname, "../..");
const schemaPath = join(ownerRoot, "🧬️schema/🔣️.json");
const document = JSON.parse(readFileSync(schemaPath, "utf-8")) as { $id: string; $defs: Record<string, Record<string, unknown>> };
const cases = JSON.parse(readFileSync(join(ownerRoot, "🧫️fixtures/📨️rest-cases.json"), "utf-8")) as {
  schema: string;
  accepted: { export: string; value: unknown; parsed: unknown }[];
  rejected: { export: string; value: unknown; reason: string }[];
};
const eventLog = readFileSync(join(ownerRoot, "🧫️fixtures/📜️g3-event-log.jsonl"), "utf-8");

const AjvConstructor = createRequire(import.meta.url)("ajv") as new (options: Record<string, unknown>) => {
  addSchema(schema: unknown): void;
  compile(schema: unknown): (data: unknown) => boolean;
};

function oracle(exportId: string): (data: unknown) => boolean {
  const ajv = new AjvConstructor({ strict: false, useDefaults: true, allErrors: true });
  ajv.addSchema(document);
  return ajv.compile({ $ref: `${document.$id}#/$defs/${exportId}` });
}

function contractConstants(): Record<string, unknown> {
  const properties = document.$defs.G3EventLogContract.properties as Record<string, { const: unknown }>;
  return Object.fromEntries(Object.entries(properties).map(([name, shape]) => [name, shape.const]));
}
//#endregion 🧫️Fixtures

//#region 🧪️Module
describe("coordinator schema module", () => {
  it("declares the canonical draft-07 identity the parsers implement", () => {
    expect(document.$schema).toBe("http://json-schema.org/draft-07/schema#");
    expect(document.$id).toBe(COORDINATOR_SCHEMA_ID);
    expect(cases.schema).toBe(COORDINATOR_SCHEMA_ID);
  });

  it("exposes exactly one parser per schema export", () => {
    expect(Object.keys(COORDINATOR_PARSERS).sort()).toEqual(Object.keys(document.$defs).sort());
  });
});
//#endregion 🧪️Module

//#region 🧪️Oracle
describe("coordinator wire contracts against an independent draft-07 validator", () => {
  it.each(cases.accepted.map((entry, index) => [index, entry] as const))("accepts %i %o", (_index, entry) => {
    const parsed = COORDINATOR_PARSERS[entry.export](entry.value);
    expect(parsed).toEqual({ success: true, data: entry.parsed });
    const data = structuredClone(entry.value);
    expect(oracle(entry.export)(data)).toBe(true);
    expect(data).toEqual(entry.parsed);
  });

  it.each(cases.rejected.map((entry, index) => [index, entry] as const))("rejects %i %o", (_index, entry) => {
    const parsed = COORDINATOR_PARSERS[entry.export](entry.value);
    expect(parsed.success).toBe(false);
    if (!parsed.success) expect(parsed.error.message).toContain(entry.reason);
    expect(oracle(entry.export)(structuredClone(entry.value))).toBe(false);
  });
});
//#endregion 🧪️Oracle

//#region 🧪️EventLog
describe("language-neutral event log", () => {
  it("keeps the log contract inside the owner module", () => {
    expect(parseG3EventLogContract(contractConstants())).toEqual({ success: true, data: contractConstants() });
    expect(oracle("G3EventLogContract")(contractConstants())).toBe(true);
  });

  it("validates every golden record against the envelope export", () => {
    const lines = eventLog.split("\n").filter((line) => line.length > 0);
    expect(lines.length).toBeGreaterThan(0);
    expect(eventLog).not.toContain("\r");
    const validate = oracle("G3EventEnvelope");
    for (const line of lines) {
      const record = JSON.parse(line);
      expect(parseG3EventEnvelope(record).success).toBe(true);
      expect(validate(record)).toBe(true);
    }
  });
});
//#endregion 🧪️EventLog
