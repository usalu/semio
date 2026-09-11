import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { createRequire } from "node:module";
import { mkdtempSync, mkdirSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";

/** 🎯️ Proves a script-backed target hashes exactly its own import closure, with a safe fallback when its command names no script. */
export async function testPreciseCommandInputs(scratchRoot = process.cwd()): Promise<void> {
  const require = createRequire(import.meta.url);
  const pluginPath = join(process.cwd(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs");
  const { cacheInternals } = await import(pathToFileURL(pluginPath).href);
  const root = realpathSync(mkdtempSync(join(scratchRoot, "precise-command-inputs-")));
  try {
    mkdirSync(join(root, "entry"), { recursive: true });
    writeFileSync(join(root, "closure.ts"), "export const inClosure = 1;\n");
    writeFileSync(join(root, "unrelated.ts"), "export const notInClosure = 1;\n");
    writeFileSync(join(root, "entry/📜️script.ts"), 'import { inClosure } from "../closure.ts";\nexport function run() { return inClosure; }\n');

    const fallback = ["{workspaceRoot}/fallback-marker.ts"];
    const target = { options: { cwd: "entry", command: "bun ./📜️script.ts run" } };
    const inputs: string[] = cacheInternals.genericTargetCommandInputs(target, root, fallback);

    assert.ok(inputs.includes("{workspaceRoot}/entry/📜️script.ts"), "closure must include the target's own script");
    assert.ok(inputs.includes("{workspaceRoot}/closure.ts"), "closure must include a statically imported file");
    assert.ok(!inputs.includes("{workspaceRoot}/unrelated.ts"), "closure must exclude a sibling file the script never imports");
    assert.ok(!inputs.includes("{workspaceRoot}/fallback-marker.ts"), "a parseable command must not carry the broad fallback");

    const oracle = await require("esbuild").build({
      entryPoints: [join(root, "entry/📜️script.ts")],
      absWorkingDir: root,
      bundle: true,
      write: false,
      platform: "node",
      format: "esm",
      packages: "external",
      metafile: true,
      logLevel: "silent",
    });
    const oracleFiles = Object.keys(oracle.metafile.inputs).map((path) => `{workspaceRoot}/${relative(root, resolve(root, path)).split("\\").join("/")}`).sort();
    assert.deepEqual([...inputs].sort(), oracleFiles, "plugin closure must match the independent esbuild import oracle");

    const unparseable = { options: { cwd: ".", command: "echo hello" } };
    const fallbackResult = cacheInternals.genericTargetCommandInputs(unparseable, root, fallback);
    assert.deepEqual(fallbackResult, fallback, "a command naming no script must fall back to the previous broad contract, never under-hash");

    const hashOfListedBytes = () => createHash("sha256").update(inputs.map((path: string) => readFileSync(join(root, path.replace("{workspaceRoot}/", "")))).join("\0")).digest("hex");
    const hashBefore = hashOfListedBytes();
    writeFileSync(join(root, "closure.ts"), "export const inClosure = 2;\n");
    const hashAfter = hashOfListedBytes();
    assert.notEqual(hashBefore, hashAfter, "Nx hashes the bytes of every listed closure file, so editing closure.ts must change the task hash");
    writeFileSync(join(root, "unrelated.ts"), "export const notInClosure = 2;\n");
    const hashAfterUnrelatedEdit = hashOfListedBytes();
    assert.equal(hashAfter, hashAfterUnrelatedEdit, "unrelated.ts is outside the closure, so editing it must not change the task hash");

    console.log("[DEBUG] Precise command inputs: closure excludes unrelated files, matches esbuild oracle, changes hash only for closure edits, and falls back safely when unparseable PASS");
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
}

if (import.meta.main) {
  await testPreciseCommandInputs(process.env.SEMIO_TEST_SCRATCH_DIR ?? process.cwd());
}
