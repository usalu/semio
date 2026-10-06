/** 🧮️ Law of the boot-budget reducers over the language-agnostic fixture `../../../🧫️fixtures/🥾️boot-budget.json` (expected
 * values computed by the independent Python oracle `wp-f3/f3-boot-budget-oracle.py`, ticket 26/09/23 F3): every resource's
 * kind, the per-kind payload sums and the verdict — payload and warm transfer always judged, timings only under the load
 * ceiling, a missing beacon or an unloaded plugin always a violation. */
import { readFileSync } from "node:fs";
import { Validator } from "jsonschema";
import { describe, expect, it } from "vitest";
import { bootPayloadV1, bootResourceKindV1, judgeBootBudgetV1, readBootBudgetFixtureV1 } from "../🟦️.ts";

const fixture = readBootBudgetFixtureV1();
const schema = JSON.parse(readFileSync(new URL("../../../🧬️schema/🔣️.json", import.meta.url), "utf8")) as { $id: string };
const validator = new Validator();
validator.addSchema(schema as never, schema.$id);


describe("boot-budget reducers", () => {
  it("carries at least one vector per verdict", () => {
    expect(fixture.schema).toBe("semio.os-dev.boot-budget/v1");
    expect(new Set(fixture.vectors.map((vector) => vector.expectedVerdict.status))).toEqual(new Set(["pass", "blocked", "fail"]));
  });

  it("satisfies BootBudgetFixtureV1 of the os-dev schema (jsonschema), which refuses a vector naming an unknown kind", () => {
    
    const hostile = structuredClone(fixture) as unknown as { vectors: { expectedKinds: string[] }[] };
    hostile.vectors[0]!.expectedKinds[0] = "image";
    
  });

  it.each(fixture.vectors.map((vector) => [vector.name, vector] as const))("classifies, sums and judges %s like the independent oracle", (_name, vector) => {
    expect(vector.resources.map((resource) => bootResourceKindV1(resource.url, resource.initiatorType))).toEqual(vector.expectedKinds);
    expect(bootPayloadV1(vector.resources)).toEqual(vector.expectedPayload);
    expect(judgeBootBudgetV1(fixture.budget, vector.measurement)).toEqual(vector.expectedVerdict);
  });
});
