import { expect, test } from "bun:test";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";
import { readViteConfigLoaderV1 } from "../../⚡️caching/🌐️vite/🎛️loader/🟦️.ts";

/** 🎛️ The os dev builder's configuration is an async factory that awaits opaque dynamic imports. Vite's `runner` loader closes its module runner before such a factory resumes, so every route that hands that builder to Vite has to ask for the `native` loader. */
const root = getWorkspaceRoot();
const dev = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev");
const routes = [join(dev, "🚚️distribution/📜️script.ts"), join(dev, "♻️activation/🌐️serve/🟦️.ts")];

test("an async factory with an opaque dynamic import loads natively and not through the module runner", async () => {
  const { loadConfigFromFile } = createRequire(join(root, "package.json"))("vite") as typeof import("vite");
  const directory = mkdtempSync(join(tmpdir(), "semio-vite-loader-"));
  try {
    writeFileSync(join(directory, "dep.mjs"), "export const value = 7;\n");
    const config = join(directory, "config.mjs");
    writeFileSync(config, 'export default async () => ({ define: { LOADED: String((await import(/* @vite-ignore */ ["./", "dep.mjs"].join(""))).value) } });\n');
    const environment = { command: "build", mode: "prototype" } as const;
    await expect(loadConfigFromFile(environment, config, directory, undefined, undefined, "runner")).rejects.toThrow("module runner has been closed");
    const loaded = await loadConfigFromFile(environment, config, directory, undefined, undefined, "native");
    expect(loaded?.config.define).toEqual({ LOADED: "7" });
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("the config loader is runner unless native is asked for, and nothing else is admitted", () => {
  expect(readViteConfigLoaderV1(undefined)).toBe("runner");
  expect(readViteConfigLoaderV1("runner")).toBe("runner");
  expect(readViteConfigLoaderV1("native")).toBe("native");
  expect(() => readViteConfigLoaderV1("bundle")).toThrow("runner or native");
});

test("every route that hands the os dev builder to Vite asks for the native loader", () => {
  for (const route of routes) {
    const line = readFileSync(route, "utf8").split("\n").find((candidate) => candidate.includes("config: contribution ?"));
    expect(line, route).toBeDefined();
    expect(line, route).toContain('configLoader: "native"');
  }
});
