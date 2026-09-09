import assert from "node:assert/strict";
import { readFileSync, mkdirSync, mkdtempSync, readdirSync, rmSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";

/** 🧪️ Verifies portable optimizer selection against a language-neutral archive contract. */
export async function testBinaryenToolchain(workspace: string, output?: string): Promise<void> {
  const root = join(import.meta.dir, "../.."), require = createRequire(import.meta.url);
  const manifest = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")), schema = JSON.parse(readFileSync(join(root, "🧬️schema/🔣️.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "./🔣️.json"), "utf8"));
  assert.equal(new (require("ajv"))().compile(schema)(manifest), true);
  const { binaryenDistribution, binaryenMembers, binaryenIdentity, prepareBinaryen, binaryenDirectory } = await import("../../📜️script.ts");
  assert.deepEqual(manifest.platforms.map((row: any) => `${row.platform}/${row.architecture}`).sort(), fixture.supported.sort());
  for (const key of fixture.supported) {
    const [platform, architecture] = key.split("/"), row = binaryenDistribution(platform, architecture);
    assert.equal(row.sha256, manifest.platforms.find((row: any) => row.platform === platform && row.architecture === architecture).sha256);
    assert.equal(binaryenIdentity(platform, architecture), `binaryen:130:${key}:${row.sha256}`);
  }
  for (const key of fixture.unsupported) { const [platform, architecture] = key.split("/"); assert.throws(() => binaryenDistribution(platform, architecture), /Unsupported/); }
  assert.deepEqual(binaryenMembers(fixture.members, "darwin"), fixture.selected);
  assert.deepEqual(binaryenMembers(fixture.members, "win32"), fixture.selected.map((path: string) => path === "bin/wasm-opt" ? path + ".exe" : path));
  for (const member of fixture.invalid) assert.throws(() => binaryenMembers([member], "darwin"), /Invalid/);
  const project = JSON.parse(readFileSync(join(workspace, "📋️project.json"), "utf8"));
  assert.equal(project.targets[fixture.target]?.cache, false);
  assert.deepEqual(project.targets[fixture.target].outputs, []);
  assert.equal(project.targets[fixture.target].options.command, `bun "${fixture.entry}" prepare`);
  assert.ok(project.targets["deps-wasm"].dependsOn.includes(fixture.target));
  const { cacheInternals } = await import("../../../../../../🟨️.mjs");
  const targets = { wasm: { cache: true, dependsOn: ["declarations"] } };
  assert.deepEqual(cacheInternals.withWasmTooling(targets, "runWasmPackWebBuild({})").wasm.dependsOn, ["declarations", `workspace:${fixture.target}`]);
  assert.deepEqual(cacheInternals.withWasmTooling(targets, "runOtherBuild({})"), targets);
  if (output) {
    mkdirSync(output, { recursive: true });
    const temporary = mkdtempSync(join(output, "binaryen-rejection-")), previous = globalThis.fetch;
    let requests = 0;
    try {
      const controller = new AbortController(); controller.abort();
      globalThis.fetch = (async () => { requests++; return new Response("invalid archive"); }) as typeof fetch;
      await assert.rejects(prepareBinaryen(temporary, controller.signal), /abort/i);
      assert.equal(requests, 0);
      await assert.rejects(prepareBinaryen(temporary, new AbortController().signal), /checksum/);
      assert.equal(requests, 1);
      assert.deepEqual(readdirSync(join(binaryenDirectory(temporary), "..")), []);
    } finally { globalThis.fetch = previous; rmSync(temporary, { recursive: true, force: true }); }
  }
  console.log("[DEBUG] Binaryen platform/archive contract and Ajv schema parity PASS");
}
