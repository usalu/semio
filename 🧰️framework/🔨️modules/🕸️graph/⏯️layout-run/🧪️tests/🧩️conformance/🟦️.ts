/** 🧩️ Independent Ajv conformance of actual layout graph and config payloads and hostile inputs; the ToolRun table retains its reason, stage, counter and checkpoint laws. */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🎞️layout-run.json";
import schema from "../../🧬️schema/🔣️.json";

const TOOL_RUN_RESERVED_REASON_FLOOR = 0xff00;
const FIELD_BYTES: Record<string, number> = { u32: 4, u64: 8, f64: 8 };
const table = (schema as any)["x-semio-toolRun"];
const law = fixture as any;

describe("schema oracle (ajv)", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addKeyword({ keyword: "x-semio-toolRun", metaSchema: true } as any);
  ajv.addSchema(schema);
  const validator = (name: string) => ajv.getSchema(`${(schema as any).$id}#/$defs/${name}`)!;

  test("actual graph and config payloads satisfy their domain contracts", () => {
    const config = validator("LayoutRunConfig"), graph = validator("LayoutRunGraph");
    expect(config(law.defaultConfig), JSON.stringify(config.errors)).toBe(true);
    for (const row of law.cases) {
      expect(config(row.config), JSON.stringify(config.errors)).toBe(true);
      if (row.graph) expect(graph(row.graph), JSON.stringify(graph.errors)).toBe(true);
    }
  });

  test("hostile config and malformed graph payloads are rejected", () => {
    const config = validator("LayoutRunConfig"), graph = validator("LayoutRunGraph");
    const first = law.cases[0].config;
    expect(config({ ...first, extra: true })).toBe(false);
    expect(config({ ...first, compactOps: 65537 })).toBe(false);
    expect(config({ ...first, repulsionFalloff: "cubic" })).toBe(false);
    expect(config({ ...first, velocityDamping: 1.5 })).toBe(false);
    expect(config((({ seed, ...rest }) => rest)(first))).toBe(false);
    expect(graph({ nodes: [], edges: [] })).toBe(true);
    expect(graph({ nodes: [{ entity: 1, origin: null, radius: 0, pinned: false, anchor: null }], edges: [] })).toBe(false);
    expect(graph({ nodes: [{ entity: 1, origin: { x: 0 }, radius: 1, pinned: false, anchor: null }], edges: [] })).toBe(false);
  });

  test("every case config is complete and the default config is a valid config", () => {
    const required: string[] = (schema as any).$defs.LayoutRunConfig.required;
    expect(Object.keys(law.defaultConfig).sort()).toEqual([...required].sort());
    for (const row of law.cases) expect(Object.keys(row.config).sort()).toEqual([...required].sort());
  });
});

describe("tool run table", () => {
  test("reason codes are dense, unique and below the reserved floor", () => {
    const codes = table.reasons.map((reason: any) => reason.code);
    expect(codes).toEqual(codes.map((_: number, index: number) => index));
    expect(Math.max(...codes)).toBeLessThan(TOOL_RUN_RESERVED_REASON_FLOOR);
    expect(new Set(table.reasons.map((reason: any) => reason.id)).size).toBe(codes.length);
    for (const reason of table.reasons) expect(["testing", "success", "warning", "danger"]).toContain(reason.verdict);
  });

  test("stage and counter indices are dense and ids unique", () => {
    for (const rows of [table.stages, table.counters]) {
      expect(rows.map((row: any) => row.index)).toEqual(rows.map((_: unknown, index: number) => index));
      expect(new Set(rows.map((row: any) => row.id)).size).toBe(rows.length);
    }
    expect(table.counters.length).toBeLessThanOrEqual(8);
  });

  test("checkpoint fields add up to the declared byte length", () => {
    expect(table.checkpoint.fields.reduce((sum: number, field: any) => sum + FIELD_BYTES[field.type], 0)).toBe(table.checkpoint.bytes);
    expect(table.checkpoint.fields[0].value).toBe(Number.parseInt("4C524331", 16));
  });

  test("every verdict prefix entry names a declared reason with its verdict", () => {
    const verdictOf = new Map(table.reasons.map((reason: any) => [reason.id, reason.verdict]));
    for (const row of law.cases) {
      for (const entry of row.expect.verdictPrefix) {
        const [, verdict, reason] = /^\d+:(\w+)\/(\w+)$/.exec(entry)!;
        expect(verdictOf.get(reason)).toBe(verdict);
      }
    }
  });
});
