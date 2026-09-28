/** 🧮️ Law of the latency gate's language-agnostic scenarios `../../../🧫️fixtures/⏱️interaction-latency.json` against the
 * os-dev schema `InteractionLatencyScenariosV1` (third-party validator: jsonschema): every scenario is well-formed, a hover
 * scenario names the attribute its target paints the hover into, and every scenario is judged on its React commits per
 * input (ticket 26/09/23 F3: a writer key cost 3 whole-shell renders, a puzzle3d hover transition 20 commits). */
import { readFileSync } from "node:fs";
import { Validator } from "jsonschema";
import { describe, expect, it } from "vitest";
import { quantileV1, type LatencyScenarios } from "../🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../../../🧫️fixtures/⏱️interaction-latency.json", import.meta.url), "utf8")) as LatencyScenarios;
const schema = JSON.parse(readFileSync(new URL("../../../🧬️schema/🔣️.json", import.meta.url), "utf8")) as { $id: string };
const validator = new Validator();
validator.addSchema(schema as never, schema.$id);
const scenariosSchema = { $ref: `${schema.$id}#/$defs/InteractionLatencyScenariosV1` };

describe("interaction-latency scenarios", () => {
  it("satisfy InteractionLatencyScenariosV1 of the os-dev schema (jsonschema)", () => {
    expect(validator.validate(fixture, scenariosSchema as never).errors.map(String)).toEqual([]);
  });

  it("refuse a hover scenario without its hover attribute and an unknown gesture", () => {
    const hover = fixture.scenarios.find((scenario) => scenario.action === "hover")!;
    const { hoverAttribute: _hoverAttribute, ...withoutAttribute } = hover;
    expect(validator.validate({ ...fixture, scenarios: [withoutAttribute] }, scenariosSchema as never).valid).toBe(false);
    expect(validator.validate({ ...fixture, scenarios: [{ ...hover, action: "scroll" }] }, scenariosSchema as never).valid).toBe(false);
  });

  it("judge every scenario on its React commits per input", () => {
    expect(fixture.scenarios.filter((scenario) => scenario.maxCommitsPerInput === undefined).map((scenario) => scenario.id)).toEqual([]);
  });

  it("take the nearest-rank quantile", () => {
    expect(quantileV1([], 0.5)).toBeNull();
    expect(quantileV1([4, 1, 3, 2], 0.5)).toBe(3);
    expect(quantileV1([4, 1, 3, 2], 0.95)).toBe(4);
    expect(quantileV1([1.26], 1)).toBe(1.3);
  });
});
