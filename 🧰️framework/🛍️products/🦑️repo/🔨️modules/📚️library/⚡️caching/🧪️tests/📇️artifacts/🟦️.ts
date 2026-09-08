import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync, linkSync, symlinkSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { createRequire } from "node:module";

/** 📇️ Verifies artifact ownership and retained-byte accounting against JSON Schema and native Python. */
export async function testArtifactRegistry(workspace: string, output: string): Promise<void> {
  const require = createRequire(join(workspace, "package.json")), validate = require("jsonschema").validate;
  const fixtureRoot = resolve(import.meta.dirname, "../../🧫️fixtures/artifact-registry"), fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
  assert.equal(validate(fixture, JSON.parse(readFileSync(join(fixtureRoot, "🛂️schema/🔣️.json"), "utf8"))).valid, true);
  const { createArtifactRegistry, measureArtifactRegistry } = await import("../../📦️artifacts/📇️registry/🟦️.ts");
  const registry = createArtifactRegistry(fixture.declarations);
  assert.equal(validate(registry, JSON.parse(readFileSync(resolve(import.meta.dirname, "../../📦️artifacts/📇️registry/🧬️schema/🔣️.json"), "utf8"))).valid, true);
  assert.deepEqual(registry.findings, []);
  assert.deepEqual(registry.entries.find(entry => entry.owner === "app:build")?.consumers, ["app:test"]);
  for (const path of fixture.invalid) assert.ok(createArtifactRegistry([{ owner: "bad:build", path }]).findings.some(finding => finding.rule === "CACHE-04"), path);
  for (const other of fixture.overlaps) assert.ok(createArtifactRegistry([...fixture.declarations, other]).findings.some(finding => finding.rule === "CACHE-03"), other.path);
  assert.deepEqual(createArtifactRegistry([...fixture.declarations].reverse()), registry);
  assert.equal(createArtifactRegistry([...fixture.declarations, fixture.declarations[0]]).entries.length, registry.entries.length);
  const root = mkdtempSync(join(output, "artifact-registry-")), workspaceRoot = join(root, "workspace"), outside = join(root, "outside");
  mkdirSync(workspaceRoot); mkdirSync(outside); writeFileSync(join(outside, "user-data"), "must not be counted");
  for (const file of fixture.files) { const path = join(workspaceRoot, file.path); mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, file.text); }
  linkSync(join(workspaceRoot, fixture.hardlink.source), join(workspaceRoot, fixture.hardlink.path));
  symlinkSync(outside, join(workspaceRoot, fixture.symlink.path), process.platform === "win32" ? "junction" : "dir");
  const progress: number[] = [];
  const measurement = await measureArtifactRegistry(workspaceRoot, registry, { signal: new AbortController().signal, onProgress: event => progress.push(event.files) });
  assert.equal(measurement.complete, true); assert.equal(measurement.totals.files, fixture.files.length);
  assert.equal(measurement.totals.symlinks, 1); assert.equal(measurement.totals.hardlinks, 1);
  assert.equal(measurement.totals.apparent, fixture.files.reduce((sum: number, file: { text: string }) => sum + Buffer.byteLength(file.text), 0));
  assert.ok(progress.length > 0); assert.equal(progress.at(-1), fixture.files.length);
  const oracleProgram = "import os,stat,json,sys\nseen=set();total={'apparent':0,'allocated':0,'files':0}\nfor directory,dirs,files in os.walk(sys.argv[1],followlinks=False):\n dirs.sort();files.sort()\n for name in files:\n  s=os.lstat(os.path.join(directory,name));key=(s.st_dev,s.st_ino)\n  if not stat.S_ISREG(s.st_mode) or key in seen:continue\n  seen.add(key);total['apparent']+=s.st_size;total['allocated']+=getattr(s,'st_blocks',(s.st_size+511)//512)*512;total['files']+=1\nprint(json.dumps(total))";
  const python = Bun.which("python3") ?? Bun.which("python"); assert.ok(python, "Native Python filesystem oracle is required");
  const oracle = Bun.spawnSync([python, "-c", oracleProgram, workspaceRoot], { stdout: "pipe", stderr: "pipe", timeout: 30000 });
  assert.equal(oracle.exitCode, 0, oracle.stderr.toString());
  const expected = JSON.parse(oracle.stdout.toString());
  if (process.platform === "win32") expected.allocated = null;
  assert.deepEqual({ apparent: measurement.totals.apparent, allocated: measurement.totals.allocated, files: measurement.totals.files }, expected);
  const cancelled = AbortSignal.abort(new Error("cancelled accounting"));
  await assert.rejects(measureArtifactRegistry(workspaceRoot, registry, { signal: cancelled }), /cancelled accounting/);
  await assert.rejects(measureArtifactRegistry(workspaceRoot, registry, { signal: new AbortController().signal, onProgress: () => { throw new Error("progress rejected"); } }), /progress rejected/);
  const linked = createArtifactRegistry([{ owner: "linked:build", path: "redirect/dist" }]);
  symlinkSync(outside, join(workspaceRoot, "redirect"), process.platform === "win32" ? "junction" : "dir");
  const rejected = await measureArtifactRegistry(workspaceRoot, linked, { signal: new AbortController().signal });
  assert.equal(rejected.complete, false); assert.ok(rejected.errors.some(error => error.path === "redirect/dist"));
  const linkedRoot = await measureArtifactRegistry(workspaceRoot, createArtifactRegistry([{ owner: "linked:build", path: "redirect" }]), { signal: new AbortController().signal });
  assert.equal(linkedRoot.complete, false); assert.ok(linkedRoot.errors.some(error => error.path === "redirect"));
  const cancelling = new AbortController();
  await assert.rejects(measureArtifactRegistry(workspaceRoot, registry, { signal: cancelling.signal, onProgress: () => cancelling.abort(new Error("cancel during accounting")) }), /cancel during accounting/);
  writeFileSync(join(root, "registry.json"), JSON.stringify(registry, null, 2) + "\n");
  writeFileSync(join(root, "measurement.json"), JSON.stringify(measurement, null, 2) + "\n");
  console.log("[DEBUG] Artifact registry paths, overlaps, links, cancellation and native Python byte accounting PASS");
}
