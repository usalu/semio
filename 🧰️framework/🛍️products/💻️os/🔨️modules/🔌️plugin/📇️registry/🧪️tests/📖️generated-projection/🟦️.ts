import { mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterAll, describe, expect, test } from "vitest";
import { isHostPlaygroundFilter } from "../../🟦️.ts";
import { filterProjectedPluginRegistry, projectedHostPluginFilter, readGeneratedCatalogProjection, resolveRegistryPluginIdsForFilter, type PluginRegistryEntry, type PlaygroundEntry } from "../../📜️script.ts";

const vector = JSON.parse(readFileSync(join(import.meta.dirname, "../../🧫️fixtures", "📖️generated-projection.json"), "utf8")) as {
  readonly entries: PluginRegistryEntry[];
  readonly playgrounds: PlaygroundEntry[];
  readonly expectations: { readonly filter: string | null; readonly host: boolean; readonly pluginIds: string[] }[];
};
const generatedDir = mkdtempSync(join(tmpdir(), "semio-generated-projection-"));
mkdirSync(generatedDir, { recursive: true });
writeFileSync(join(generatedDir, "🔌️plugins.json"), `${JSON.stringify(vector.entries, null, 2)}\n`);
writeFileSync(join(generatedDir, "🎠️playgrounds.json"), `${JSON.stringify(vector.playgrounds, null, 2)}\n`);
afterAll(() => rmSync(generatedDir, { recursive: true, force: true }));

/** 🔮️ Independent fixed-point closure includes contributions consumed at every dependency depth. */
function closureOracle(entries: readonly PluginRegistryEntry[], playgrounds: readonly PlaygroundEntry[], filter: string): Set<string> {
  const target = playgrounds.find((row) => row.variant === filter || row.aliases.includes(filter))?.pluginId ?? filter;
  const byId = new Map(entries.map((entry) => [entry.pluginId, entry]));
  const ids = new Set(byId.has(target) ? [target] : []);
  for (let changed = true; changed; ) {
    changed = false;
    for (const id of [...ids]) {
      const row = byId.get(id)!;
      const dependencies = [...row.dependsOn, ...entries.filter(entry => row.host || entry.contributes.some(topic => row.consumes.includes(topic))).map(entry => entry.pluginId)];
      for (const dep of dependencies) if (!ids.has(dep)) { ids.add(dep); changed = true; }
    }
  }
  return ids;
}

describe("generated catalog projection", () => {
  test("session filtering preserves recursively consumed contributions", () => {
    const fixture = JSON.parse(readFileSync(join(import.meta.dirname, "../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/runtime-components/🔣️.json"), "utf8"));
    for (const row of fixture.cases.filter((row: { roots: string[] }) => row.roots.length === 1)) {
      expect(resolveRegistryPluginIdsForFilter(row.roots[0], fixture.components, []), row.name).toEqual(row.expected);
    }
  });

  test("reads the projected rows byte-for-byte and sorts entries like the generator", () => {
    const projection = readGeneratedCatalogProjection(generatedDir);
    expect(projection.entries.map((entry) => entry.pluginId)).toEqual(vector.entries.map((entry) => entry.pluginId));
    expect(projection.playgrounds.map((row) => row.variant)).toEqual(vector.playgrounds.map((row) => row.variant));
    const sorted = [...vector.entries].map((entry) => entry.pluginId).sort((a, b) => a.localeCompare(b));
    expect(filterProjectedPluginRegistry(projection).map((entry) => entry.pluginId)).toEqual(sorted);
  });

  test("filter semantics match the language-agnostic expectation vector and the fixpoint oracle", () => {
    const projection = readGeneratedCatalogProjection(generatedDir);
    for (const expectation of vector.expectations) {
      const filter = expectation.filter ?? undefined;
      expect(projectedHostPluginFilter(projection, filter), `host ${expectation.filter}`).toBe(expectation.host);
      const ids = filterProjectedPluginRegistry(projection, filter).map((entry) => entry.pluginId);
      expect(ids, `filter ${expectation.filter}`).toEqual(expectation.pluginIds);
      if (filter && !expectation.host) {
        expect(new Set(resolveRegistryPluginIdsForFilter(filter, vector.entries, vector.playgrounds))).toEqual(closureOracle(vector.entries, vector.playgrounds, filter));
        const known = new Set(vector.entries.map((entry) => entry.pluginId));
        expect(new Set(ids)).toEqual(new Set([...closureOracle(vector.entries, vector.playgrounds, filter)].filter((id) => known.has(id))));
      }
    }
  });

  test("the generated-module host predicate is semantically identical to the projection one", () => {
    const projection = readGeneratedCatalogProjection(generatedDir);
    for (const expectation of vector.expectations) {
      const filter = expectation.filter ?? undefined;
      expect(isHostPlaygroundFilter(filter, vector.playgrounds, vector.entries), `vector host ${expectation.filter}`).toBe(expectation.host);
      expect(isHostPlaygroundFilter(filter, vector.playgrounds, vector.entries)).toBe(projectedHostPluginFilter(projection, filter));
    }
    const live = readGeneratedCatalogProjection(join(import.meta.dirname, "../../🤖️generated"));
    const filters = [undefined, "", "not-a-plugin", ...live.playgrounds.flatMap((row) => [row.variant, ...row.aliases]), ...live.entries.map((entry) => entry.pluginId)];
    for (const filter of filters) expect(isHostPlaygroundFilter(filter), `live ${filter}`).toBe(projectedHostPluginFilter(live, filter));
    expect(filters.filter((filter) => isHostPlaygroundFilter(filter)).length).toBeGreaterThan(0);
  });

  test("rejects a projection that is not a row array", () => {
    const broken = mkdtempSync(join(tmpdir(), "semio-generated-projection-broken-"));
    writeFileSync(join(broken, "🔌️plugins.json"), "{}\n");
    writeFileSync(join(broken, "🎠️playgrounds.json"), "[]\n");
    expect(() => readGeneratedCatalogProjection(broken)).toThrow(/not a projected row array/u);
    rmSync(broken, { recursive: true, force: true });
  });
});
