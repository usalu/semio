import assert from "node:assert/strict";
import nativeFs, { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { open } from "node:fs/promises";
import { mock } from "bun:test";
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
    assert.equal(fixture.cancellation.point, "identical-eof");
    const handle = await open(files.values().next().value!, "r"), prototype = Object.getPrototypeOf(handle), read = prototype.read;
    await handle.close();
    const eofCancellation = new AbortController();
    let eofReads = 0;
    prototype.read = async function(this: unknown, ...args: unknown[]) {
      const result = await read.apply(this, args);
      if (!result.bytesRead && ++eofReads === files.size * 2) eofCancellation.abort(new Error(fixture.cancelReason));
      return result;
    };
    try {
      await assert.rejects(() => stageArtifacts(staging, fixture.owner, files, { leaseDirectory, signal: eofCancellation.signal }), error => error === eofCancellation.signal.reason);
      assert.equal(eofCancellation.signal.aborted, true);
      assert.equal(eofReads, files.size * 2);
      assert.equal(fixture.cancellation.outcome, "rejected");
      assert.equal(fixture.cancellation.unchanged, true);
      const preserved = Bun.spawnSync([Bun.which("python3") ?? "python", "-c", program, staging], { stdout: "pipe", stderr: "pipe" });
      assert.equal(preserved.exitCode, 0, preserved.stderr.toString());
      assert.deepEqual(JSON.parse(preserved.stdout.toString()), fixture.files);
    } finally { prototype.read = read; }
    await assert.rejects(() => stageArtifacts(staging, "foreign", files, { leaseDirectory }), /Unowned/);
    await assert.rejects(() => stageArtifacts(staging, fixture.owner, new Map([["../escape", files.values().next().value!]]), { leaseDirectory }), /Invalid artifact/);
    assert.deepEqual(readdirSync(staging).sort(), [".nx-artifact.json", "nested", "support.js"]);
    const native = { ...nativeFs };
    let attempts = 0;
    const emulateRename = (code: string, failures: number): void => {
      attempts = 0;
      mock.module("node:fs", () => ({ ...native, renameSync: (source: string, destination: string) => {
        if (destination === staging && source.startsWith(staging + ".stage-") && !source.endsWith(".previous") && ++attempts <= failures) throw Object.assign(new Error("Contended artifact rename"), { code });
        native.renameSync(source, destination);
      } }));
    };
    const verifyBytes = (expected: Record<string, string>): void => {
      const result = Bun.spawnSync([Bun.which("python3") ?? "python", "-c", program, staging], { stdout: "pipe", stderr: "pipe" });
      assert.equal(result.exitCode, 0, result.stderr.toString());
      assert.deepEqual(JSON.parse(result.stdout.toString()), expected);
    };
    try {
      for (const [name, text] of Object.entries(fixture.replacement.files)) writeFileSync(files.get(name)!, String(text));
      emulateRename(fixture.replacement.transient.code, fixture.replacement.transient.failures);
      const retryElapsed: number[] = [];
      await stageArtifacts(staging, fixture.owner, files, { leaseDirectory, onWait: progress => { assert.equal(progress.resource, `artifact:${resolve(staging)}`); assert.equal(progress.mode, "exclusive"); retryElapsed.push(progress.elapsedMs); } });
      assert.equal(attempts, fixture.replacement.transient.failures + 1);
      assert.equal(retryElapsed.length, fixture.replacement.transient.failures);
      assert.ok(retryElapsed.every((elapsed, index) => elapsed >= 0 && (!index || elapsed >= retryElapsed[index - 1]!)));
      assert.equal(fixture.replacement.transient.outcome, "published");
      verifyBytes(fixture.replacement.files);
      for (const [name, text] of Object.entries(fixture.files)) writeFileSync(files.get(name)!, String(text));
      emulateRename(fixture.replacement.permanent.code, Infinity);
      await assert.rejects(() => stageArtifacts(staging, fixture.owner, files, { leaseDirectory }), (error: NodeJS.ErrnoException) => error.code === fixture.replacement.permanent.code);
      assert.equal(attempts, 1);
      assert.equal(fixture.replacement.permanent.unchanged, true);
      verifyBytes(fixture.replacement.files);
      emulateRename(fixture.replacement.persistent.code, Infinity);
      const retryStarted = Date.now();
      await assert.rejects(() => stageArtifacts(staging, fixture.owner, files, { leaseDirectory }), (error: NodeJS.ErrnoException) => error.code === fixture.replacement.persistent.code);
      assert.ok(Date.now() - retryStarted >= fixture.replacement.persistent.timeoutMs);
      assert.ok(attempts > 1);
      assert.equal(fixture.replacement.persistent.unchanged, true);
      verifyBytes(fixture.replacement.files);
      emulateRename(fixture.replacement.cancellation.code, Infinity);
      const retryCancellation = new AbortController();
      await assert.rejects(() => stageArtifacts(staging, fixture.owner, files, { leaseDirectory, signal: retryCancellation.signal, onWait: () => retryCancellation.abort(new Error(fixture.cancelReason)) }), error => error === retryCancellation.signal.reason);
      assert.equal(attempts, 1);
      assert.equal(fixture.replacement.cancellation.unchanged, true);
      verifyBytes(fixture.replacement.files);
      assert.equal(fixture.replacement.initial.absentBefore, true);
      rmSync(staging, { recursive: true, force: true });
      emulateRename(fixture.replacement.initial.code, fixture.replacement.initial.failures);
      await stageArtifacts(staging, fixture.owner, files, { leaseDirectory });
      assert.equal(attempts, fixture.replacement.initial.failures + 1);
      assert.equal(fixture.replacement.initial.outcome, "published");
      verifyBytes(fixture.files);
      assert.deepEqual(readdirSync(root).filter(name => name.startsWith("dist.stage-")), []);
    } finally { mock.module("node:fs", () => native); }

  } finally { lease.release(); rmSync(root, { recursive: true, force: true }); }
}
