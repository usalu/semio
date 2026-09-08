import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";

/** 🧪️ Verifies portable optimizer selection against a language-neutral archive contract. */
export async function testBinaryenToolchain(): Promise<void> {
  const root = join(import.meta.dir, "../../../🚀️bootstrap/🛠️tools/🕸️wasm"), require = createRequire(import.meta.url);
  const manifest = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")), schema = JSON.parse(readFileSync(join(root, "🧬️schema/🔣️.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️toolchain.json"), "utf8"));
  assert.equal(new (require("ajv"))().compile(schema)(manifest), true);
  const { binaryenDistribution, binaryenMembers, binaryenIdentity } = await import("../../../🚀️bootstrap/🛠️tools/🕸️wasm/📜️script.ts");
  assert.deepEqual(manifest.platforms.map((row: any) => `${row.platform}/${row.architecture}`).sort(), fixture.supported.sort());
  for (const key of fixture.supported) {
    const [platform, architecture] = key.split("/"), row = binaryenDistribution(platform, architecture);
    assert.equal(row.sha256, manifest.platforms.find((row: any) => row.platform === platform && row.architecture === architecture).sha256);
    assert.equal(binaryenIdentity(platform, architecture), `binaryen:130:${key}:${row.sha256}`);
  }
  for (const key of fixture.unsupported) assert.throws(() => binaryenDistribution(...key.split("/")), /Unsupported/);
  assert.deepEqual(binaryenMembers(fixture.members, "darwin"), fixture.selected);
  assert.deepEqual(binaryenMembers(fixture.members, "win32"), fixture.selected.map((path: string) => path === "bin/wasm-opt" ? path + ".exe" : path));
  for (const member of fixture.invalid) assert.throws(() => binaryenMembers([member], "darwin"), /Invalid/);
  console.log("[DEBUG] Binaryen platform/archive contract and Ajv schema parity PASS");
}
