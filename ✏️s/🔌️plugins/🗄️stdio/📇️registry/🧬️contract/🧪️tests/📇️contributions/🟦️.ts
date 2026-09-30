import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

/** 🧫 Checks portable contribution vectors and neutral registry ownership against Ajv. */
export function runContributionChecks(): number {
  const load = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const fixtures = load("../../🧫️fixtures/📇️contributions/🔣️.json");
  const schema = load("../../../🧬️schema/📇️contributions/🔣️.json");
  const ajv = new Ajv({ strict: true });
  const validate = ajv.compile(schema);
  assert.equal(validate(fixtures), true, JSON.stringify(validate.errors));
  assert.equal(validate({ ...fixtures, unexpected: true }), false);
  let checks = 2;
  for (const row of fixtures.cases) {
    let selected: string[] = [...row.selected];
    let accepted = true;
    for (const removed of row.remove) {
      const candidate = selected.filter((owner) => owner !== removed);
      const available = new Set(candidate.map((owner) => `s.stdio.${owner}`));
      const valid = candidate.every((owner) => load(`../../🧫️fixtures/📇️contributions/${owner}.json`).dependencies.every((dependency: string) => available.has(dependency)));
      accepted &&= valid;
      if (valid) selected = candidate;
    }
    assert.equal(accepted, row.accepted);
    assert.deepEqual(selected, row.remaining);
    checks += 2;
  }
  for (const row of fixtures.registrationCases) {
    const left = load(`../../🧫️fixtures/📇️contributions/${row.first}.json`);
    const right = load(`../../🧫️fixtures/📇️contributions/${row.second}.json`);
    const disjoint = (key: string) => left.representations[0][key].every((claim: string) => !right.representations[0][key].includes(claim));
    assert.equal(left.id !== right.id && left.directory !== right.directory && disjoint("mimes") && disjoint("extensions"), row.accepted);
    checks++;
  }
  for (const row of fixtures.receiptCases) {
    assert.equal(row.authoredFactory === false, row.accepted);
    checks++;
  }
  const registry = readFileSync(new URL("../../../🦀️.rs", import.meta.url), "utf8");
  assert.doesNotMatch(registry, /semio_s_artifact_stdio_|selected_contributions|expected_artifact_count|include_str!/u);
  checks++;
  const root = load("../../../🧬️schema/🔣️.json");
  const projection = new Ajv({ strict: false }).compile({ ...root, $ref: "#/$defs/NativeCatalogSurfaceCommitment" });
  const payload = { schema: "semio.stdio.artifact-catalog/v1", pluginId: "stdio", packageId: "semio:stdio", packageVersion: "0.1.0", definitions: [], codecs: [] };
  assert.equal(projection(payload), true, JSON.stringify(projection.errors));
  checks++;
  return checks;
}
