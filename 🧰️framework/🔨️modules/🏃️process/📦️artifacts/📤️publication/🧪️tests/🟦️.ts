import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { createRequire } from "node:module";
import { stageArtifacts } from "../🟦️.ts";
import { acquireResourceLease } from "../../../🔒️leases/🟦️.ts";

/** 📦️ Verifies portable publication, cancellation and complete bytes through independent Python. */
export async function testArtifactPublication(output: string): Promise<void> {
  const fixture = JSON.parse(readFileSync(resolve(import.meta.dirname, "../🧫️fixtures/🔣️.json"), "utf8"));
  assert.ok(new (createRequire(import.meta.url)("ajv").default)({strict:true}).validate(JSON.parse(readFileSync(resolve(import.meta.dirname,"../🧬️schema/🔣️.json"),"utf8")),fixture));
  const root = mkdtempSync(join(output, "artifact-publication-")), staging = join(root, "dist"), leaseDirectory = join(root, "leases");
  const files = new Map<string, string>();
  for (const [name, text] of Object.entries(fixture.files)) {
    const path = join(root, "input", name); mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, String(text)); files.set(name, path);
  }
  await assert.rejects(()=>Reflect.apply(stageArtifacts,undefined,[staging,fixture.owner,files]),/Explicit absolute publication lease/);
  await assert.rejects(()=>stageArtifacts(staging,fixture.owner,files,{leaseDirectory:"relative"}),/Explicit absolute publication lease/);
  const controller = new AbortController();
  const lease = await acquireResourceLease({ directory: leaseDirectory, resource: `artifact:${resolve(staging)}`, mode: "exclusive", signal: controller.signal });
  try {
    let published = false, waits = 0;
    const cancelled = new AbortController();
    const rejected = stageArtifacts(staging, fixture.owner, files, { leaseDirectory, signal: cancelled.signal, onWait: () => cancelled.abort(new Error(fixture.cancelReason)) });
    await assert.rejects(() => rejected, (error: Error) => error.name === "AbortError" || error.message === fixture.cancelReason);
    const pending = stageArtifacts(staging, fixture.owner, files, { leaseDirectory, onWait: () => { waits++; } }).then(() => { published = true; });
    await Bun.sleep(80);
    assert.equal(published, false, "Concurrent publication must wait for the active publisher");
    assert.ok(waits > 0, "Waiting must report progress");
    lease.release();
    await pending;
    const program = "import json,pathlib,sys\np=pathlib.Path(sys.argv[1]);print(json.dumps({str(f.relative_to(p)).replace('\\\\','/'):f.read_text() for f in p.rglob('*') if f.is_file() and f.name!='.nx-artifact.json'}))";
    const oracle = Bun.spawnSync([Bun.which("python3") ?? "python", "-c", program, staging], { stdout: "pipe", stderr: "pipe" });
    assert.equal(oracle.exitCode, 0, oracle.stderr.toString());
    assert.deepEqual(JSON.parse(oracle.stdout.toString()), fixture.files);
    await assert.rejects(() => stageArtifacts(staging, "foreign", files, { leaseDirectory }), /Unowned/);
    await assert.rejects(() => stageArtifacts(staging, fixture.owner, new Map([["../escape", files.values().next().value!]]), { leaseDirectory }), /Invalid artifact/);
    assert.deepEqual(readdirSync(staging).sort(), [".nx-artifact.json", "nested", "support.js"]);
  } finally { lease.release(); rmSync(root, { recursive: true, force: true }); }
}
