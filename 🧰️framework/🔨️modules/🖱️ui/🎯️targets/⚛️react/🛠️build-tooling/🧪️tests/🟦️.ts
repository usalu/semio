/** 🧪️ `uiTailwindBuildPlugins({ sourceBase })` hands the Tailwind scan hook a config whose root is the narrowed base and every other key of the resolved Vite config unchanged. */
import { expect, mock, test } from "bun:test";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

type Case = { id: string; sourceBase: string | null; viteRoot: string; expectedRoot: string; expectedOtherKeysKept: string[] };
const fixture = JSON.parse(readFileSync(fileURLToPath(new URL("./🔣️.json", import.meta.url)), "utf8")) as { schemaVersion: number; cases: Case[] };
const received: Record<string, unknown>[] = [];

mock.module("@tailwindcss/vite", () => ({
  default: () => [{ name: "@tailwindcss/vite:scan", configResolved(config: Record<string, unknown>) { received.push(config); } }, { name: "@tailwindcss/vite:generate" }],
}));
const { uiTailwindBuildPlugins } = await import("../🟦️.ts");

test("the narrowed source base replaces only the root seen by the Tailwind scan hook", async () => {
  expect(fixture.schemaVersion).toBe(1);
  for (const row of fixture.cases) {
    received.length = 0;
    const plugins = row.sourceBase === null ? uiTailwindBuildPlugins() : uiTailwindBuildPlugins({ sourceBase: row.sourceBase });
    expect(plugins.map((plugin) => (plugin as { name?: string }).name), row.id).toEqual(["@tailwindcss/vite:scan", "@tailwindcss/vite:generate"]);
    const config = { root: row.viteRoot, build: { outDir: "dist" }, css: { devSourcemap: false } };
    await (plugins[0] as unknown as { configResolved(config: object): unknown }).configResolved(config);
    expect(received.length, row.id).toBe(1);
    expect(received[0]!.root, row.id).toBe(row.expectedRoot);
    for (const key of row.expectedOtherKeysKept) expect(received[0]![key], `${row.id} ${key}`).toBe(config[key as keyof typeof config]);
    expect(config.root, `${row.id} caller config untouched`).toBe(row.viteRoot);
  }
});
