import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { buildCargoArtifacts } from "../../🟦️.ts";
import { publishedCargoArtifactDirectoryV1 } from "../../📤️published/🟦️.ts";

import { lawBuildOptionsV1, withIssuedLawStorageV1 } from "../🦀️cargo/🟦️.ts";
import{withBuildTestInvocationV1}from"../../../../🧪️testing/📥️invocation/🟦️.ts";

/** 🦀️ Compares a captured executable with Cargo while retaining shared intermediates and retiring private outputs. */
export async function testCargoArtifactPublication(output: string): Promise<void> {
  const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url), "utf8")).cargo;
  await withBuildTestInvocationV1(original => withIssuedLawStorageV1(original, new AbortController().signal, async (held, admission) => {
  const root = mkdtempSync(join(held.root, "p-"));
  try {
    mkdirSync(join(root, ".cargo"));
    writeFileSync(join(root, ".cargo/config.toml"), '[build]\ntarget-dir="shared-output"\nbuild-dir="build-cache"\n');
    writeFileSync(join(root, "Cargo.toml"), `[workspace]\n[profile.prototype]\ninherits="dev"\n[package]\nname=${JSON.stringify(fixture.name)}\nversion="0.0.0"\nedition="2021"\n[[bin]]\nname=${JSON.stringify(fixture.name)}\npath="main.rs"\n`);
    writeFileSync(join(root, "main.rs"), `fn main() { print!(${JSON.stringify(fixture.stdout)}); }\n`);
    const oracle = Bun.spawn(["cargo", "build", "--offline", "--target-dir", join(root, "oracle")], { cwd: root, stdout: "pipe", stderr: "pipe" });
    const [status, diagnostic] = await Promise.all([oracle.exited, new Response(oracle.stderr).text()]);
    assert.equal(status, 0, diagnostic);
    const executable = fixture.name + (process.platform === "win32" ? ".exe" : "");
    const expected = Bun.spawnSync([join(root, "oracle/debug", executable)], { stdout: "pipe", stderr: "pipe" });
    assert.equal(expected.exitCode, 0, expected.stderr.toString());
    assert.equal(expected.stdout.toString(), fixture.stdout);
    await buildCargoArtifacts(join(root,"Cargo.toml"), ["--bin", fixture.name], {version:1,mode:"prototype",cwd:root,buildDirectory:join(root,"build-cache"),leaseDirectory:output,captureDirectory:join(root,"dist"),budgetMs:10000},lawBuildOptionsV1(held, admission));
    assert.equal(existsSync(join(root, "shared-output/debug", executable)), false, "Captured builds must not contend for the workspace's uplifted artifacts");
    assert.ok(existsSync(join(root, "build-cache")), "Compiler intermediates must remain shared");
    const delivered = Bun.spawnSync([join(publishedCargoArtifactDirectoryV1(held.storage.backing.root, join(root, "Cargo.toml"), "prototype"), executable)], { stdout: "pipe", stderr: "pipe" });
    assert.equal(delivered.exitCode, 0, delivered.stderr.toString());
    assert.deepEqual(delivered.stdout, expected.stdout);
    assert.deepEqual(existsSync(join(root, "dist")) ? readdirSync(join(root, "dist")) : [], [], "Private Cargo outputs must retire after publication");
  } finally { rmSync(root, { recursive: true, force: true }); }
  }));
}

