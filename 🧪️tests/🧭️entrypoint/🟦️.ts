import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { pathToFileURL } from "node:url";
import { build } from "esbuild";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };

const root = resolve(import.meta.dir, "../.."), entry = resolve(root, fixture.entry);

test("workspace help starts without loading unrelated command or test owners", () => {
  for (const argument of fixture.helpArguments) {
    const result = spawnSync(process.execPath, [entry, argument], { cwd: root, encoding: "utf8", timeout: 15_000 });
    expect(result.error).toBeUndefined();
    expect(result.status, result.stderr).toBe(0);
    expect(result.stdout.trim()).toBe(fixture.expectedHelp);
  }
});

test("root repo route creates the same original build caller as the dedicated entry", () => {
  for (const args of fixture.backendHelpArguments) {
    const result = spawnSync(process.execPath, [entry, ...args], { cwd: root, encoding: "utf8", timeout: 15_000 });
    expect(result.error).toBeUndefined();
    expect(result.status, result.stderr).toBe(0);
    expect(result.stdout).toContain("bun repo [--tool=nx|bazel]");
  }
}, 40_000);

test("developer command registration loads without its unrelated application gateway", () => {
  for (const module of fixture.isolatedCommandModules) {
    const source = `await import(${JSON.stringify(pathToFileURL(resolve(root, module)).href)})`;
    const result = spawnSync(process.execPath, ["-e", source], { cwd: root, encoding: "utf8", timeout: 15_000 });
    expect(result.error).toBeUndefined();
    expect(result.status, result.stderr).toBe(0);
  }
}, 30_000);

test("independent bundler confirms eager startup uses only platform and routing owners", async () => {
  const compiled = await build({
    absWorkingDir: root, entryPoints: [fixture.entry], bundle: true, write: false, platform: "node", format: "esm", logLevel: "silent",
    plugins: [{ name: "command-boundary", setup(bundler) {
      bundler.onResolve({ filter: /.*/ }, args => {
        if (args.kind === "dynamic-import") return { path: args.path, external: true };
        if (args.importer === entry && !fixture.eagerOwners.some(prefix => args.path.startsWith(prefix))) throw Error(`Eager command dependency: ${args.path}`);
      });
    } }],
  });
  const source = readFileSync(entry, "utf8");
  for (const command of fixture.commands) expect(source.includes(`.registerLazy("${command}"`) || command === "scale-fixture").toBe(true);
  const result = spawnSync(process.execPath, ["-", "--help"], { cwd: root, encoding: "utf8", input: compiled.outputFiles[0]!.text, timeout: 15_000 });
  expect(result.status, result.stderr).toBe(0);
  expect(result.stdout.trim()).toBe(fixture.expectedHelp);
}, 30_000);
