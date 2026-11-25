/** 🧵️ The air flow worker is bundled apart from the page, with the worker plugins of the site alone: it reaches the library through its barrel, so the Node cast exporter must be replaced there too, or the release build stops at the first Node built-in.
 * @see ./🥒️.feature — the scenarios
 * @see ../../🏗️builder/🌐️vite/🟦️.ts — `staffageBrowserCastVitePlugin` and the `worker.plugins` of the site
 * @see ../../../🎯️targets/⚛️react/🔨️modules/🌊️flow/🧵️server/worker.ts — the worker entry */
import { join, resolve } from "node:path";
import { build } from "vite";
import { expect, it } from "vitest";
import { atTestLevel, testLevelBudgetMs } from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { repositoryCachePartitionRootDataV1 } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts";
import site from "../../🏗️builder/🌐️vite/🟦️.ts";

const staffage = resolve(import.meta.dirname, "../../..");
const cast = resolve(staffage, "🔨️modules/📤️export/🎭️cast/🟦️.ts");
const stub = resolve(staffage, "🌐️site/🏗️builder/🌐️vite/🌐️cast-stub.ts");
const worker = resolve(staffage, "🎯️targets/⚛️react/🔨️modules/🌊️flow/🧵️server/worker.ts");

async function configuration() {
  process.env.SEMIO_VITE_CACHE_DIRECTORY = join(repositoryCachePartitionRootDataV1(resolve(staffage, "../../.."), "products"), "vite-staffage-worker");
  return await site({ command: "build", mode: "release" });
}

it("replaces the Node cast exporter in the worker bundle as in the page bundle", async () => {
  const config = await configuration();
  const resolved = await Promise.all((config.worker?.plugins?.() ?? []).flat().map((plugin) => plugin.resolveId?.(cast)));
  expect(resolved).toContain(stub);
  expect(await Promise.all((config.plugins ?? []).flat().map((plugin) => "resolveId" in plugin && plugin.name === "staffage-browser-cast" ? plugin.resolveId?.(cast) : null))).toContain(stub);
});

atTestLevel(it, "long")("bundles the air flow worker without a Node built-in", async () => {
  const config = await configuration();
  const output = await build({ configFile: false, root: staffage, logLevel: "silent", plugins: config.worker?.plugins?.() as never, resolve: config.resolve as never, build: { write: false, minify: false, rollupOptions: { input: worker } } });
  const code = (Array.isArray(output) ? output : [output]).flatMap((result) => "output" in result ? result.output : []).map((chunk) => "code" in chunk ? chunk.code : "").join("\n");
  expect(code).toContain("postMessage");
  expect(code).not.toMatch(/__vite-browser-external|from\s*["']node:/u);
}, testLevelBudgetMs("long"));
