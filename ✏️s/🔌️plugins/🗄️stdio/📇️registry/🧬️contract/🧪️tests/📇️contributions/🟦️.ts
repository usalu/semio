import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

/** 🧫 Checks portable contribution vectors and neutral registry ownership against Ajv. */
export function runContributionChecks(): number {
  const load = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const fixtures = load("../../🧫️fixtures/📇️contributions/🔣️.json");
  const ajv = new Ajv({ strict: true });
  let checks = 0;
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
  const constraint = ajv.compile(load("../../🧫️fixtures/📇️contributions/🧾️schema-input.json"));
  for (const row of fixtures.validationCases) {
    const source = load("../../🧫️fixtures/📇️contributions/alpha.json");
    source.representations[0].mimes = row.mimes;
    const representation = source.runtime_capabilities.find((item: { category: string }) => item.category === "representation");
    representation.claims = [...row.mimes.map((value: string) => ({ namespace: "mime", value })), ...source.representations[0].extensions.map((value: string) => ({ namespace: "extension", value }))];
    representation.descriptor = `runtime-capability:representation:${representation.claims.map((claim: { namespace: string; value: string }) => `${claim.namespace}:${claim.value}`).join("|")}`;
    assert.equal(row.constraint ? constraint(source) : true, row.accepted);
    checks++;
  }
  const contract = readFileSync(new URL("../../🦀️.rs", import.meta.url), "utf8");
  assert.doesNotMatch(contract, /source\.artifact\s*==\s*"[^"]+"/u, "general contribution contract contains artifact-owned policy");
  checks++;
  const registry = readFileSync(new URL("../../../🦀️.rs", import.meta.url), "utf8");
  assert.doesNotMatch(registry, /semio_s_artifact_stdio_|selected_contributions|expected_artifact_count|include_str!/u);
  checks++;
  return checks;
}
