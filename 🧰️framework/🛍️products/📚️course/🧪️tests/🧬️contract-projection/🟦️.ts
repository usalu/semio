import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import contract from "../../🧬️schema/🔣️.json";
import vectors from "../../🧫️fixtures/🧬️contract-projection/🔣️.json";
import presentations from "../../🧫️fixtures/👥️presentation/🔣️.json";
import { projectContractDefinitions } from "../../🧬️schema/📽️projection/🟦️.ts";

describe("presentation contract projection", () => {
  for (const row of vectors.cases) it(row.id, () => {
    if (row.error) return expect(() => projectContractDefinitions(row.schema, row.root)).toThrow(row.error);
    const projected = projectContractDefinitions(row.schema, row.root);
    expect(Object.keys(projected.$defs)).toEqual(row.definitions);
    const original = new Ajv({ strict: false }).compile({ ...row.schema, $ref: projected.$ref });
    const compiled = new Ajv({ strict: false }).compile(projected);
    for (const value of row.valid ?? []) expect([original(value), compiled(value)]).toEqual([true, true]);
    for (const value of row.invalid ?? []) expect([original(value), compiled(value)]).toEqual([false, false]);
    expect(JSON.stringify(projected)).not.toContain("Authoring");
    if (row.id.startsWith("reachable")) expect(projected.$defs.Root).toMatchObject({ properties: { description: { $ref: "#/$defs/Label" }, payload: { const: { description: "Data", $ref: "Data" } } } });
  });

  it("matches the full normative schema on every public presentation vector", () => {
    const projected = projectContractDefinitions(contract, "PresencePresentation");
    const original = new Ajv({ strict: false }).compile({ ...contract, $ref: projected.$ref });
    const compiled = new Ajv({ strict: false }).compile(projected);
    for (const row of presentations) {
      const state = row.state as Record<string, unknown>;
      if (row.definition === "PresenceState" && Object.hasOwn(state, "view")) expect(compiled(state.view), row.id).toBe(original(state.view));
    }
    expect(Object.hasOwn(projected.$defs, "Command")).toBe(false);
    expect(Object.hasOwn(projected.$defs, "PresencePreferences")).toBe(true);
    expect(JSON.stringify(projected).length).toBeLessThan(JSON.stringify(contract).length / 2);
  });
});
