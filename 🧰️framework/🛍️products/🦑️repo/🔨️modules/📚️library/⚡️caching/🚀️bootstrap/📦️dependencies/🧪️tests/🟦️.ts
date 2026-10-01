/** 📦️ Portable locked-install and owner-requested lock refresh command laws. */
import { expect, test } from "bun:test";
import { readFileSync, mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { resolve, join } from "node:path";
import Ajv from "ajv";
import { build } from "esbuild";
import { prepareJavascriptDependencies, SyncScript, RefreshLockScript } from "../📜️script.ts";

const owner = resolve(import.meta.dir, "..");
const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8")) as { operations: { mode: "sync" | "lock"; arguments: string[] }[]; platforms: string[]; rejections: string[][] };
const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8"));

test("validates the closed portable dependency corpus before command execution", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...fixture, extra: true })).toBe(false);
  expect(validate({ ...fixture, operations: fixture.operations.map(operation => ({ ...operation, arguments: ["install", "--production"] })) })).toBe(false);
});

test("selects exact Bun arguments across all supported hosts and rejects installer overrides", async () => {
  for (const platform of fixture.platforms) for (const operation of fixture.operations) {
    const calls: unknown[] = [], signal = new AbortController().signal;
    await prepareJavascriptDependencies(operation.mode, platform, signal, async (args, cwd, actualSignal) => { calls.push({ args, cwd, identicalSignal: actualSignal === signal }); });
    expect(calls).toEqual([{ args: operation.arguments, cwd: platform, identicalSignal: true }]);
  }
  for (const args of fixture.rejections) for (const Constructor of [SyncScript, RefreshLockScript]) await expect(new Constructor(owner, owner).run(args)).rejects.toThrow("accepts no installer overrides");
  const aborted = new AbortController(); aborted.abort();
  let called = false;
  await expect(prepareJavascriptDependencies("lock", owner, aborted.signal, async () => { called = true; })).rejects.toThrow();
  expect(called).toBe(false);
  await expect(prepareJavascriptDependencies("lock", owner, new AbortController().signal, async () => { throw new Error("installer refused"); })).rejects.toThrow("installer refused");
});

test("an independent Node bundle executes the same authored command corpus", async () => {
  const artifact = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifact) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(artifact, { recursive: true });
  const output = mkdtempSync(join(artifact, "javascript-dependencies-")), module = join(output, "dependencies.mjs");
  await build({ entryPoints: [join(owner, "📜️script.ts")], outfile: module, platform: "node", format: "esm", bundle: true, packages: "external", logLevel: "silent" });
  const entry = join(output, "oracle.mjs");
  writeFileSync(entry, `import { prepareJavascriptDependencies } from ${JSON.stringify(module)}; const cases = ${JSON.stringify(fixture.operations)}; const calls = []; for (const operation of cases) await prepareJavascriptDependencies(operation.mode, "workspace", new AbortController().signal, async (args, cwd) => calls.push({args,cwd})); process.stdout.write(JSON.stringify(calls));`);
  const child = Bun.spawn(["node", entry], { stdout: "pipe", stderr: "pipe" });
  const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
  expect({ code, stderr }).toEqual({ code: 0, stderr: "" });
  expect(JSON.parse(stdout)).toEqual(fixture.operations.map(operation => ({ args: operation.arguments, cwd: "workspace" })));
});
