import { describe, test, expect } from "vitest";
import { join } from "node:path";
import { readGeneratedCatalogProjection, projectedHostPluginFilter } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";
import { isHostPlaygroundFilter } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🟦️.ts";
describe("Specific installed host projection", () => { test("preserves every original installed host predicate comparison", () => {
    const live = readGeneratedCatalogProjection(join(import.meta.dirname, "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated"));
    const filters = [undefined, "", "not-a-plugin", ...live.playgrounds.flatMap((row) => [row.variant, ...row.aliases]), ...live.entries.map((entry) => entry.pluginId)];
    for (const filter of filters) expect(isHostPlaygroundFilter(filter, live.playgrounds, live.entries), `live ${filter}`).toBe(projectedHostPluginFilter(live, filter));
    expect(filters.filter((filter) => isHostPlaygroundFilter(filter, live.playgrounds, live.entries)).length).toBeGreaterThan(0);
}); });
