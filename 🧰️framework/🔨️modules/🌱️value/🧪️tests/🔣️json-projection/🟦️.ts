import assert from "node:assert/strict";
import Ajv from "ajv";
import schema from "../../🧬️schema/🔣️.json" with { type: "json" };
import fixture from "./🔣️.json" with { type: "json" };
import { parseDslValue } from "../../🧬️schema/🟦️.ts";

/** 🌱️ Validates the shared JSON projection against Ajv without accepting hidden runtime state. */
export function testSharedDynamicValueOracle(): void {
  const validate = new Ajv({ strict: true }).compile(schema);
  for (const value of fixture.values) {
    assert.equal(validate(value), true, JSON.stringify(validate.errors));
    assert.deepEqual(parseDslValue(value), value);
    assert.deepEqual(JSON.parse(JSON.stringify(parseDslValue(value))), value);
  }
  const cycle: Record<string, unknown> = {};
  cycle.self = cycle;
  let reads = 0;
  const accessor = Object.defineProperty({}, "value", { enumerable: true, get: () => { reads++; return 1; } });
  const constructions: Record<string, unknown> = {
    "not-a-number": NaN,
    infinity: Infinity,
    undefined,
    cycle,
    "sparse-array": new Array(1),
    "symbol-field": { [Symbol("hidden")]: 1 },
    "hidden-field": Object.defineProperty({}, "hidden", { value: 1 }),
    "accessor-field": accessor,
    "array-extra-field": Object.assign([], { hidden: 1 }),
  };
  for (const name of fixture.rejectedConstructions) assert.throws(() => parseDslValue(constructions[name]), undefined, name);
  assert.equal(reads, 0, "validation must not execute accessor values");
  const child = { value: 1 }, shared = { left: child, right: child };
  assert.deepEqual(parseDslValue(shared), shared);
  console.log("[DEBUG] shared dynamic value projection matched nine Ajv JSON vectors and rejected all nine non-JSON constructions");
}
