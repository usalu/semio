import { strict as assert } from "node:assert";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import { admitMutationInventoryProviderV1, selectMutationInventoryProviderV1 } from "../🟦️.ts";

/** 🧪️ Proves provider admission and explicit ownership against an independent schema oracle. */
export function runMutationInventoryProviderChecksV1(): number {
  const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const schema = read("../🧬️schema/🔣️.json"), vectors = read("../🧫️fixtures/🔣️.json");
  const oracle = new Ajv({ strict: true }).compile(schema);
  for (const vector of vectors.cases) {
    assert.equal(Boolean(oracle(vector.value)), vector.valid);
    if (vector.valid) assert.deepEqual(admitMutationInventoryProviderV1(vector.value), vector.value);
    else assert.throws(() => admitMutationInventoryProviderV1(vector.value));
  }
  const providers = [{ script: resolve("outer/runner"), roots: [resolve("lower/alpha")] }, { script: resolve("second/runner"), roots: [resolve("lower/bravo")] }];
  assert.equal(selectMutationInventoryProviderV1(providers, resolve("lower/alpha/nested")), providers[0]);
  assert.equal(selectMutationInventoryProviderV1(providers, resolve("lower/bravo")), providers[1]);
  assert.equal(selectMutationInventoryProviderV1(providers, resolve("lower/alpha-imposter")), undefined);
  assert.throws(() => selectMutationInventoryProviderV1([...providers, providers[0]!], resolve("lower/alpha")));
  return vectors.cases.length + 4;
}
