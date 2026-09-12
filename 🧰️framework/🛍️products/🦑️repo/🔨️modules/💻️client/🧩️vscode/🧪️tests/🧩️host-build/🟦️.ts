import assert from "node:assert/strict";
import { mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import { extensionBuildConfig } from "../../🏗️builder/🟦️.ts";

/** 🧩️ Compares CommonJS host bundles with esbuild and native Node while excluding framework test registration. */
export async function testExtensionHostBuild(generated: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🧩️host-build/🔣️.json"), "utf8"));
  const root = mkdtempSync(join(generated, "extension-host-build-")), config = extensionBuildConfig(root, "entry.ts", "out", "extension.cjs", false);
  assert.deepEqual(config.define, fixture.define);
  for (const [id, expected] of fixture.externals) assert.equal(config.build.rollupOptions.external(id), expected, id);
  writeFileSync(join(root, "entry.ts"), fixture.source); writeFileSync(join(root, "library.ts"), fixture.library);
  const { build } = await import("vite");
  await build({ ...config, logLevel: "warn" });
  await require("esbuild").build({ entryPoints: [join(root, "entry.ts")], bundle: true, platform: "node", target: "node22", format: "cjs", define: fixture.define, outfile: join(root, "oracle.cjs") });
  for (const path of [join(root, "out/extension.cjs"), join(root, "oracle.cjs")]) {
    const result = spawnSync("node", ["-e", "console.log(JSON.stringify(require(process.argv[1]).result))", path], { cwd: root, encoding: "utf8", timeout: 10000 });
    assert.equal(result.status, 0, result.stderr);
    assert.deepEqual(JSON.parse(result.stdout), fixture.expected);
    assert.ok(!readFileSync(path, "utf8").includes("unavailable-vitest-suite"));
  }
  console.log("[DEBUG] Extension CommonJS host build excludes Vitest registration, preserves Node modules and matches esbuild/native Node PASS");
}
