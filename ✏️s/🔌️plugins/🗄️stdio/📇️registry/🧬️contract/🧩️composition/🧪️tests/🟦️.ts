import { strict as assert } from "node:assert";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { selectCompositionContributionsV1 } from "../🟦️.ts";

/** 🧪️ Compares neutral contribution admission and removal with an independent schema oracle. */
export function runCompositionContributionChecks(): number {
  const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const corpus = read("../🧫️fixtures/🔣️.json");
  const ajv = new Ajv({ strict: true });
  const validRow = ajv.compile(read("../🧬️schema/🔣️.json"));
  const unique = ajv.compile({ type: "array", uniqueItems: true });
  for (const vector of corpus.cases) {
    const oracle = vector.contributions.every((row: unknown) => validRow(row)) && ["artifact", "package", "order"].every((key) => unique(vector.contributions.map((row: Record<string, unknown>) => row[key])));
    let selected: string[] | undefined;
    try { selected = selectCompositionContributionsV1(vector.contributions, vector.selection).map((row) => row.artifact); } catch {}
    assert.equal(selected !== undefined, oracle, `${vector.id}: independent admission`);
    assert.equal(selected !== undefined, !vector.error, vector.id);
    if (selected) {
      const expected = vector.contributions.filter((row: { selections: string[] }) => row.selections.includes(vector.selection)).sort((a: { order: number }, b: { order: number }) => a.order - b.order).map((row: { artifact: string }) => row.artifact);
      assert.deepEqual(selected, expected, `${vector.id}: independent selection`);
      assert.deepEqual(selected, vector.expected, vector.id);
    }
  }
  return corpus.cases.length;
}
