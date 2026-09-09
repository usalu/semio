import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

/** 🚀️ Verifies the image's pinned runtime acquisition before application dependency synchronization. */
export function testContainerRuntimeBootstrap(workspace: string): void {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🚀️runtime-bootstrap/🔣️.json"), "utf8"));
  const dockerfile = readFileSync(join(workspace, ".devcontainer/Dockerfile"), "utf8"), configSource = readFileSync(join(workspace, ".devcontainer/devcontainer.json"), "utf8");
  assert.deepEqual(Bun.JSONC.parse(configSource), require("jsonc-parser").parse(configSource));
  assert.deepEqual(Bun.JSONC.parse(configSource).postCreateCommand, fixture.postCreateCommand);
  for (const path of fixture.retiredScripts) assert.equal(existsSync(join(workspace, path)), false, `Retired lifecycle bypass: ${path}`);
  const target = JSON.parse(readFileSync(join(workspace, "📋️project.json"), "utf8")).targets[fixture.postCreateCommand[3].split(":")[1]];
  assert.equal(target.cache, false); assert.deepEqual(target.outputs, []);
  assert.deepEqual(target.dependsOn ?? [], []);
  assert.ok(target.options.command.endsWith('⚡️caching/🚀️bootstrap/📦️dependencies/📜️script.ts" sync'));
  assert.ok(!Object.keys(Bun.JSONC.parse(configSource).features).some(key => /\/nx:/.test(key)), "Nx must come from the repository's locked bootstrap");
  assert.equal(JSON.parse(readFileSync(join(workspace, "package.json"), "utf8")).packageManager, `bun@${fixture.bun}`);
  assert.ok(dockerfile.includes(`ARG BUN_VERSION=${fixture.bun}\n`));
  assert.ok(dockerfile.includes(`ARG NODE_VERSION=${fixture.node}\n`));
  const runtime = dockerfile.match(/^RUN set -eu; \\\n[\s\S]*?(?=\n\n)/m)?.[0];
  assert.ok(runtime, "Runtime acquisition must be one checked image layer");
  const selector = runtime.replaceAll("\\\n", "\n").match(/case "\$runtime_arch" in[\s\S]*?esac;/)?.[0];
  assert.ok(selector, "Image must reject unsupported runtime architectures");
  const rows = [...selector.matchAll(/^\s*(amd64|arm64)\) bun_archive="([^"]+)"; bun_sha="([a-f0-9]{64})"; node_archive="([^"]+)"; node_sha="([a-f0-9]{64})";;/gm)].map(([, architecture, bunArchive, bunSha256, nodeArchive, nodeSha256]) => ({ architecture, bunArchive, bunSha256, nodeArchive, nodeSha256 }));
  assert.deepEqual(require("lodash/sortBy")(rows, "architecture"), require("lodash/sortBy")(fixture.platforms, "architecture"));
  for (const tool of ["bun", "node"]) {
    assert.ok(runtime.indexOf(`"$${tool}_sha`) < runtime.indexOf(tool === "bun" ? "unzip -p" : "tar -xJf"), `${tool} verification must precede extraction`);
    assert.ok(runtime.includes(`test "$(${tool} --version)" = "${tool === "node" ? "v" : ""}$${tool.toUpperCase()}_VERSION"`));
  }
  assert.equal((runtime.match(/sha256sum -c -/g) ?? []).length, 2);
  assert.equal((runtime.match(/--max-time 300/g) ?? []).length, 2);
  assert.ok(runtime.includes("https://github.com/oven-sh/bun/releases/download/bun-v$BUN_VERSION/"));
  assert.ok(runtime.includes("https://nodejs.org/dist/v$NODE_VERSION/"));
  assert.ok(!/bun install|npm|curl[^\n]*\|\s*(?:ba)?sh/.test(runtime));
  if (process.platform !== "win32") {
    const command = `${selector}\nprintf '%s|%s|%s|%s' "$bun_archive" "$bun_sha" "$node_archive" "$node_sha"`;
    for (const row of fixture.platforms) {
      const result = spawnSync("sh", ["-ec", command], { env: { ...process.env, runtime_arch: row.architecture }, encoding: "utf8", timeout: 5000 });
      assert.equal(result.status, 0, result.stderr);
      assert.equal(result.stdout, [row.bunArchive, row.bunSha256, row.nodeArchive, row.nodeSha256].join("|"));
    }
    for (const architecture of fixture.rejectedArchitectures) assert.notEqual(spawnSync("sh", ["-ec", command], { env: { ...process.env, runtime_arch: architecture }, encoding: "utf8", timeout: 5000 }).status, 0);
  }
  console.log("[DEBUG] Container runtime pins, checksum-before-extraction, repository Nx ownership and JSONC/lodash/native shell oracles PASS");
}
