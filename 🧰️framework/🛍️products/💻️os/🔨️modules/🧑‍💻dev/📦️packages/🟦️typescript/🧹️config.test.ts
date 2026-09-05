/**
 * 🧹️ Vite config hygiene. Pinned to the node environment: at `long` the suite default is jsdom, whose
 * `TextEncoder` fails esbuild JS API's `instanceof Uint8Array` invariant, and the graph bundles below are
 * pure Node work with no DOM.
 *
 * @vitest-environment node
 */
import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { build, type Plugin } from "esbuild";
import { describe, expect, it } from "vitest";
import { stripExecutableShebang } from "./🧪️tests/🧹️executable-source/🟦️.ts";

describe("executable source transformation", () => {
  it.each([
    ["#!/usr/bin/env bun\nexport const value = 1;\n", "export const value = 1;\n"],
    ["#!/usr/bin/env bun\r\nexport const value = 1;\r\n", "export const value = 1;\r\n"],
    ["export const value = '#!/usr/bin/env bun';\n", "export const value = '#!/usr/bin/env bun';\n"],
  ])("removes only a leading Bun shebang before Vite injects imports", (source, expected) => {
    expect(stripExecutableShebang(source)).toBe(expected);
  });
});

const packageDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(packageDir, "../../../../../../..");
const contract = JSON.parse(readFileSync(join(packageDir, "🧫️fixtures/⚙️config-graph.json"), "utf8")) as {
  readonly entry: string;
  readonly deny: readonly string[];
  readonly require: readonly string[];
  readonly maxModules: number;
  readonly maxSourceBytes: number;
};
const entryPath = join(repoRoot, contract.entry);

/** @emoji 📦️ Vite's own `externalize-deps` rule for `--configLoader bundle` (`vite/dist/node/chunks/config.js`):
 * every bare specifier is external, so a config's real bundle graph is its relative-import closure. */
const externalizeDeps: Plugin = {
  name: "externalize-deps",
  setup(builder) {
    builder.onResolve({ filter: /^[^.#].*/ }, ({ path, kind }) => (kind === "entry-point" || path.startsWith("/") ? null : { external: true }));
  },
};

const repoRelative = (path: string): string => relative(repoRoot, resolve(repoRoot, path)).replaceAll("\\", "/");

async function esbuildConfigGraph(): Promise<{ readonly modules: readonly string[]; readonly sourceBytes: number }> {
  const result = await build({ entryPoints: [entryPath], bundle: true, write: false, metafile: true, platform: "node", format: "esm", logLevel: "silent", absWorkingDir: repoRoot, plugins: [externalizeDeps] });
  return { modules: Object.keys(result.metafile.inputs).map(repoRelative).sort(), sourceBytes: Object.values(result.metafile.inputs).reduce((total, input) => total + input.bytes, 0) };
}

/** @emoji 🔮️ Independent oracle: Bun's own bundler resolves the same entry under the same
 * externalize-every-package rule, and its sourcemap `sources` array is the module set it actually
 * read — a second implementation of the graph this contract bounds, not a second read of esbuild's. */
function bunConfigGraph(): readonly string[] {
  const bun = process.execPath.endsWith("bun") ? process.execPath : "bun";
  const outDir = mkdtempSync(join(tmpdir(), "semio-config-graph-"));
  try {
    const status = spawnSync(bun, ["build", entryPath, "--target=bun", "--packages=external", "--sourcemap=external", "--outdir", outDir], { cwd: repoRoot, encoding: "utf8" });
    if (status.status !== 0) throw new Error(`bun build failed: ${status.stderr || status.stdout}`);
    const sourcemap = readdirSync(outDir).find((name) => name.endsWith(".map"));
    if (!sourcemap) throw new Error("bun build emitted no sourcemap to read the module graph from");
    return (JSON.parse(readFileSync(join(outDir, sourcemap), "utf8")) as { sources: string[] }).sources.map(repoRelative).sort();
  } finally {
    rmSync(outDir, { recursive: true, force: true });
  }
}

describe("vite config module graph", () => {
  it("keeps every denied module out of the bundled config graph", async () => {
    const { modules } = await esbuildConfigGraph();
    expect(modules).toContain(contract.entry);
    for (const denied of contract.deny) expect(modules, `${denied} is reachable from ⚙️vite.config.ts — Vite parses and watches it on every boot`).not.toContain(denied);
  });

  it("keeps every required module in the bundled config graph", async () => {
    const { modules } = await esbuildConfigGraph();
    for (const required of contract.require) expect(modules).toContain(required);
  });

  it("stays inside the declared module and source-byte bounds", async () => {
    const { modules, sourceBytes } = await esbuildConfigGraph();
    expect(modules.length).toBeLessThanOrEqual(contract.maxModules);
    expect(sourceBytes).toBeLessThanOrEqual(contract.maxSourceBytes);
    console.log(`[DEBUG] vite config graph: ${modules.length} modules, ${sourceBytes} source bytes`);
  });

  it("agrees with Bun's independent bundler on the resolved TypeScript module set", async () => {
    const { modules } = await esbuildConfigGraph();
    const oracle = bunConfigGraph();
    expect(oracle).toEqual(modules.filter((module) => /\.[cm]?[jt]sx?$/u.test(module)));
    for (const denied of contract.deny) expect(oracle).not.toContain(denied);
  });
});
