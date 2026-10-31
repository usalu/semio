import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { minimatch } from "minimatch";
import { schemaScopeOwnerLevel } from "../🟦️.ts";
import { loadCatalogTaxonomy } from "../../🔐️taxonomy-authority/🟦️.ts";

const corpus = JSON.parse(readFileSync(new URL("./📥️.json", import.meta.url), "utf8")) as { cases: { id: string; path: string; level: string | null }[] };
const taxonomy = loadCatalogTaxonomy();
const owners = taxonomy.schemaScopeOwnerLevels;
for (const row of corpus.cases) test("schema state lane ownership: " + row.id, () => {
  const match = (pattern: string) => minimatch(row.path, pattern, { dot: true });
  const excluded = [...owners.excludedOwnerPathPatterns, ...owners.fixtureOwnerPathPatterns].some(match);
  const independent = excluded ? null : Object.entries(owners.levels).find(([, level]) => level.pathPatterns.some(match))?.[0] ?? null;
  expect(independent).toBe(row.level);
  expect(schemaScopeOwnerLevel(row.path, taxonomy)).toBe(row.level);
});

test("all declared state lane kinds own schema children", () => {
  for (const lane of ["🎚️config", "👥️presence", "🫧️transient"]) {
    expect(taxonomy.surfaceChildDirs).toContain(lane);
    expect(taxonomy.modeChildDirs).toContain(lane);
    expect(taxonomy.windowChildDirs).toContain(lane);
  }
  for (const key of ["configChildDirs", "presenceChildDirs", "transientChildDirs"] as const) expect(taxonomy[key]).toContain("🧬️schema");
});
