import { expect, test } from "bun:test";
import Ajv from "ajv";
import schema from "../../🧬️schema/🔣️.json";
import fixture from "../../🧫️fixtures/🌐️geometry-lifetime/🔣️.json";

test("durable geometry inputs and canonical identities agree with the independent schema oracle", () => {
  const ajv = new Ajv({ strict: true });
  
  
  const importSource = ajv.compile(schema.$defs.GeometryImportSource);
  for (const source of fixture.imports) {
    expect(importSource(source)).toBe(true);
    expect(importSource({ ...source, handles: ["a".repeat(64)] })).toBe(false);
    expect(importSource({ ...source, data: "" })).toBe(false);
  }
  const identity = ajv.compile({ type: "string", pattern: "^[0-9a-f]{64}$" });
  for (const row of fixture.handles) expect(identity(row.value)).toBe(row.valid);
  
  
  
  
  
  
});
