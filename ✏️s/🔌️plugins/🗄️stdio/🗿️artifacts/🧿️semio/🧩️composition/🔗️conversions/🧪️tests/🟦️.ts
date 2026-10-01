import { strict as assert } from "node:assert";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { selectSemioConversionTargetsV1 } from "../🟦️.ts";

/** 🧪️ Proves present-target selection against an independent schema implementation. */
export function runSemioConversionContributionChecks(): number {
  const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const corpus = read("../🧫️fixtures/🔣️.json"), schema = read("../🧬️schema/🔣️.json");
  const valid = new Ajv({ strict: true }).compile(schema);
  assert(valid(corpus.links));
  for (const vector of corpus.cases) {
    const selected = selectSemioConversionTargetsV1(corpus.links, new Set<string>(vector.present));
    const oracle = corpus.links.groups.flatMap((group: { targets: { package: string; feature: string }[] }) => group.targets.filter((target) => vector.present.includes(target.package))).map((target: { feature: string }) => target.feature);
    assert.deepEqual(selected.map((target) => target.feature), oracle);
    assert.deepEqual(oracle, vector.expected);
  }
  assert(!valid({ ...corpus.links, unknown: true }));
  assert.throws(() => selectSemioConversionTargetsV1({ ...corpus.links, unknown: true }, new Set()));
  return corpus.cases.length + 2;
}
