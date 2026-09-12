import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, utimesSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const MODULE_ROOT = join(dirname(fileURLToPath(import.meta.url)), "../..");

const PYTHON_PLAN_ORACLE = `import json,sys
areas=json.loads(sys.argv[1]); now=json.loads(sys.argv[2]); guard=json.loads(sys.argv[3])
out=[]
for area in areas:
 units=area['units']
 total=sum(u['bytes'] for u in units)
 cutoff=now-area['unusedAgeMs']
 age=[u for u in units if not u['lockHeld'] and u['recencyMs']<=cutoff]
 agePaths={u['path'] for u in age}
 remaining=[u for u in units if u['path'] not in agePaths]
 remainingBytes=sum(u['bytes'] for u in remaining)
 budget=[]
 guardedBytes=0
 if area['budgetBytes'] is not None and remainingBytes>area['budgetBytes']:
  guardCutoff=now-area.get('guardAgeMs',guard)
  for u in sorted(remaining,key=lambda u:(u['recencyMs'],u['path'])):
   if remainingBytes<=area['budgetBytes']: break
   if u['lockHeld'] or u['recencyMs']>guardCutoff:
    guardedBytes+=u['bytes']; continue
   budget.append(u); remainingBytes-=u['bytes']
 deleted=sum(u['bytes'] for u in age)+sum(u['bytes'] for u in budget)
 out.append({'name':area['name'],'totalBytes':total,'unitCount':len(units),'ageDeletions':[u['path'] for u in age],'budgetDeletions':[u['path'] for u in budget],'retainedBytes':total-deleted,'guardedOverBudgetBytes':guardedBytes})
print(json.dumps(out))
`;

const PYTHON_BYTES_ORACLE = `import os,stat,json,sys
total=0
for directory,dirs,files in os.walk(sys.argv[1],followlinks=False):
 dirs.sort();files.sort()
 for name in files:
  s=os.lstat(os.path.join(directory,name))
  if stat.S_ISREG(s.st_mode):total+=s.st_size
print(json.dumps(total))
`;

/** 🧪️ Verifies the pure age/budget/guard eviction planner against a hand-authored fixture and an independent Python oracle. */
async function testPlanCachePrune(): Promise<{ planCachePrune: typeof import("../../🧹️pruning/🟦️.ts").planCachePrune }> {
  const require = createRequire(import.meta.url);
  const fixtureRoot = join(MODULE_ROOT, "🧫️fixtures/cache-prune");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
  assert.equal(require("jsonschema").validate(fixture, JSON.parse(readFileSync(join(fixtureRoot, "🛂️schema/🔣️.json"), "utf8"))).valid, true);
  const pruning = await import("../../🧹️pruning/🟦️.ts");
  const plan = pruning.planCachePrune(fixture.areas, fixture.nowMs, fixture.guardAgeMs);
  const projected = Object.fromEntries(plan.areas.map((area) => [area.name, {
    totalBytes: area.totalBytes, unitCount: area.unitCount,
    ageDeletions: area.ageDeletions.map((unit) => unit.path).sort(),
    budgetDeletions: area.budgetDeletions.map((unit) => unit.path).sort(),
    retainedBytes: area.retainedBytes, guardedOverBudgetBytes: area.guardedOverBudgetBytes,
  }]));
  const expected = Object.fromEntries(Object.entries(fixture.expected).map(([name, row]: [string, any]) => [name, { ...row, ageDeletions: [...row.ageDeletions].sort(), budgetDeletions: [...row.budgetDeletions].sort() }]));
  assert.deepEqual(projected, expected);
  const python = Bun.which("python3") ?? Bun.which("python");
  assert.ok(python, "Native Python eviction oracle is required");
  const oracle = Bun.spawnSync([python, "-c", PYTHON_PLAN_ORACLE, JSON.stringify(fixture.areas), JSON.stringify(fixture.nowMs), JSON.stringify(fixture.guardAgeMs)], { stdout: "pipe", stderr: "pipe", timeout: 30000 });
  assert.equal(oracle.exitCode, 0, oracle.stderr.toString());
  const oracleProjected = Object.fromEntries((JSON.parse(oracle.stdout.toString()) as any[]).map(({ name, ...row }) => [name, { ...row, ageDeletions: [...row.ageDeletions].sort(), budgetDeletions: [...row.budgetDeletions].sort() }]));
  assert.deepEqual(projected, oracleProjected);
  assert.equal(pruning.formatBytes(0), "0 B");
  assert.equal(pruning.formatBytes(1536), "1.50 KiB");
  assert.equal(pruning.formatBytes(80 * 1024 ** 3), "80.00 GiB");
  console.log("[DEBUG] Cache eviction planner matches fixture and independent Python oracle for age, budget, guard (including per-area override) and lock protection PASS");
  return { planCachePrune: pruning.planCachePrune };
}

/** 🧪️ Verifies real-disk scanning, incremental session compaction, deletion and cancellation against a temp Cargo-shaped tree and a Python byte oracle. */
async function testCargoDiskScan(output: string): Promise<void> {
  const pruning = await import("../../🧹️pruning/🟦️.ts");
  const root = mkdtempSync(join(output, "cache-prune-"));
  const buildDir = join(root, "build"), targetDir = join(root, "target"), viteRoot = join(root, "vite"), agentsRoot = join(root, "agents");
  const now = Date.now(), old = new Date(now - 10 * 24 * 3600 * 1000), fresh = new Date(now);
  const put = (path: string, text: string, when: Date): void => {
    const file = join(root, path);
    mkdirSync(dirname(file), { recursive: true });
    writeFileSync(file, text);
    utimesSync(file, when, when);
  };
  put("build/debug/build/pkg-a/hash1/fingerprint/lib-abc.json", "{}", old);
  put("build/debug/build/pkg-a/hash1/out/generated.rs", "// generated\n", old);
  put("build/debug/incremental/crate-old-1a2b/s-htestold1-idold001-svhold/s-old.o", "obj", old);
  put("build/debug/incremental/crate-old-1a2b/s-htestold1-idold001.lock", "", old);
  put("build/debug/incremental/crate-fresh-3c4d/s-htestfrs1-idfrs001-svhfrs/s-new.o", "obj", fresh);
  put("build/debug/incremental/crate-fresh-3c4d/s-htestfrs1-idfrs001.lock", "", fresh);
  put("build/debug/incremental/crate-multi-5e6f/s-htestmu01-idmu0001-svhm1/s-old.o", "obj", old);
  put("build/debug/incremental/crate-multi-5e6f/s-htestmu01-idmu0001.lock", "", old);
  put("build/debug/incremental/crate-multi-5e6f/s-htestmu02-idmu0002-svhm2/s-new.o", "obj", fresh);
  put("build/debug/incremental/crate-multi-5e6f/s-htestmu02-idmu0002.lock", "", fresh);
  put("build/debug/incremental/crate-working-7g8h/s-htestwk01-idwk0001-svhwk/s-old.o", "obj", old);
  put("build/debug/incremental/crate-working-7g8h/s-htestwk01-idwk0001.lock", "", old);
  put("build/debug/incremental/crate-working-7g8h/s-htestwk02-idwk0002-working/s-active.o", "obj", old);
  put("build/debug/incremental/crate-working-7g8h/s-htestwk02-idwk0002.lock", "", old);
  put("target/debug/deps/libfoo-abcdef.rlib", "rlib", old);
  put("target/debug/examples/demo", "bin", fresh);
  put("target/debug/CACHEDIR.TAG", "Signature: 8a477f597d28d172789f06886806bc55", old);
  put("vite/consumer-a/deps/_meta.json", "{}", old);
  put("vite/consumer-b/deps/_meta.json", "{}", fresh);
  put("agents/stray-session/scratch.txt", "x", old);
  put("agents/resource-leases/00000000000000000000000000000000.sqlite", "lease", fresh);
  const signal = new AbortController().signal;
  const buildUnits = pruning.scanCargoBuildUnits(buildDir, signal);
  assert.deepEqual(buildUnits.map((unit) => unit.path), ["debug/build/pkg-a/hash1"], "Compiled-unit scanning no longer descends into incremental crate dirs");
  assert.equal(buildUnits[0].kind, "cargo-build");
  const incrementalScan = pruning.scanCargoIncrementalUnits(buildDir, signal);
  const byPath = Object.fromEntries(incrementalScan.units.map((unit) => [unit.path, unit]));
  assert.deepEqual(Object.keys(byPath).sort(), ["debug/incremental/crate-fresh-3c4d", "debug/incremental/crate-multi-5e6f", "debug/incremental/crate-old-1a2b", "debug/incremental/crate-working-7g8h"]);
  assert.equal(byPath["debug/incremental/crate-old-1a2b"].lockHeld, false);
  assert.equal(byPath["debug/incremental/crate-old-1a2b"].bytes, Buffer.byteLength("obj"), "A single finalized session is the crate dir's whole size");
  assert.equal(byPath["debug/incremental/crate-fresh-3c4d"].lockHeld, false);
  assert.equal(byPath["debug/incremental/crate-multi-5e6f"].bytes, Buffer.byteLength("obj"), "Only the newest finalized session counts toward the crate dir's LRU size");
  assert.equal(byPath["debug/incremental/crate-multi-5e6f"].recencyMs, fresh.getTime(), "Newest finalized session's recency, not the stale one's");
  assert.equal(byPath["debug/incremental/crate-working-7g8h"].lockHeld, true, "A -working session must mark the whole crate dir lockHeld, regardless of its own age");
  assert.equal(byPath["debug/incremental/crate-working-7g8h"].bytes, Buffer.byteLength("obj") * 2, "Finalized session plus the in-progress working session");
  assert.deepEqual(incrementalScan.staleSessions.map((unit) => unit.path), ["debug/incremental/crate-multi-5e6f/s-htestmu01-idmu0001-svhm1"], "Only crate-multi has more than one finalized session");
  assert.equal(incrementalScan.staleSessions[0].kind, "cargo-incremental-session");
  assert.equal(incrementalScan.staleSessions[0].lockHeld, false);
  const targetUnits = pruning.scanCargoTargetUnits(targetDir, signal);
  assert.deepEqual(targetUnits.map((unit) => unit.path).sort(), ["debug/deps/libfoo-abcdef.rlib", "debug/examples/demo"]);
  const viteUnits = pruning.scanDirectoryUnits(viteRoot, signal);
  assert.deepEqual(viteUnits.map((unit) => unit.path).sort(), ["consumer-a", "consumer-b"]);
  const agentUnits = pruning.scanDirectoryUnits(agentsRoot, signal, new Set(["resource-leases"]));
  assert.deepEqual(agentUnits.map((unit) => unit.path), ["stray-session"]);
  const python = Bun.which("python3") ?? Bun.which("python");
  assert.ok(python, "Native Python byte oracle is required");
  for (const [scanned, path] of [[buildUnits, join(buildDir, "debug/build")], [targetUnits, targetDir]] as const) {
    const oracle = Bun.spawnSync([python, "-c", PYTHON_BYTES_ORACLE, path], { stdout: "pipe", stderr: "pipe", timeout: 30000 });
    assert.equal(oracle.exitCode, 0, oracle.stderr.toString());
    const expectedBytes = JSON.parse(oracle.stdout.toString());
    const scannedBytes = scanned.reduce((sum: number, unit: { bytes: number }) => sum + unit.bytes, 0) + (scanned === targetUnits ? Buffer.byteLength("Signature: 8a477f597d28d172789f06886806bc55") : 0);
    assert.equal(scannedBytes, expectedBytes, `${path}: byte total must match Python os.walk oracle (sentinel files excluded from prune units, included in Python's raw walk)`);
  }
  const incrementalOracle = Bun.spawnSync([python, "-c", PYTHON_BYTES_ORACLE, join(buildDir, "debug/incremental")], { stdout: "pipe", stderr: "pipe", timeout: 30000 });
  assert.equal(incrementalOracle.exitCode, 0, incrementalOracle.stderr.toString());
  const incrementalRawBytes = JSON.parse(incrementalOracle.stdout.toString());
  const incrementalUnitsBytes = incrementalScan.units.reduce((sum, unit) => sum + unit.bytes, 0) + incrementalScan.staleSessions.reduce((sum, unit) => sum + unit.bytes, 0);
  assert.equal(incrementalUnitsBytes, incrementalRawBytes, "Compaction repartitions bytes between kept crate-dir units and stale sessions; it must never gain or lose bytes against a raw disk walk");
  const cancelled = new AbortController(); cancelled.abort(new Error("cancelled"));
  assert.throws(() => pruning.scanCargoBuildUnits(buildDir, cancelled.signal), /cancelled/);
  assert.throws(() => pruning.scanCargoIncrementalUnits(buildDir, cancelled.signal), /cancelled/);
  assert.throws(() => pruning.scanDirectoryUnits(viteRoot, cancelled.signal), /cancelled/);
  const nowMs = Date.now();
  const areas = [
    { name: "cargo", budgetBytes: null, unusedAgeMs: 5 * 24 * 3600 * 1000, units: [...buildUnits, ...targetUnits] },
    { name: "cargo-incremental", budgetBytes: null, unusedAgeMs: 5 * 24 * 3600 * 1000, units: incrementalScan.units },
    { name: "cargo-incremental-sessions", budgetBytes: null, unusedAgeMs: 0, units: incrementalScan.staleSessions },
    { name: "vite", budgetBytes: null, unusedAgeMs: 5 * 24 * 3600 * 1000, units: viteUnits },
    { name: "agents", budgetBytes: null, unusedAgeMs: 5 * 24 * 3600 * 1000, units: agentUnits },
  ];
  const plan = pruning.planCachePrune(areas, nowMs, 48 * 3600 * 1000);
  const cargoPlan = plan.areas.find((area) => area.name === "cargo")!;
  assert.deepEqual(cargoPlan.ageDeletions.map((unit) => unit.path).sort(), ["debug/build/pkg-a/hash1", "debug/deps/libfoo-abcdef.rlib"]);
  const incrementalPlan = plan.areas.find((area) => area.name === "cargo-incremental")!;
  assert.deepEqual(incrementalPlan.ageDeletions.map((unit) => unit.path), ["debug/incremental/crate-old-1a2b"], "crate-working-7g8h stays despite being just as old, because it is lockHeld");
  const staleSessionsPlan = plan.areas.find((area) => area.name === "cargo-incremental-sessions")!;
  assert.deepEqual(staleSessionsPlan.ageDeletions.map((unit) => unit.path), ["debug/incremental/crate-multi-5e6f/s-htestmu01-idmu0001-svhm1"], "Stale finalized sessions are reclaimed unconditionally, independent of the crate dir's own age");
  for (const unit of cargoPlan.ageDeletions) pruning.deleteUnit(unit.kind === "cargo-target-file" ? targetDir : buildDir, unit);
  for (const unit of [...incrementalPlan.ageDeletions, ...staleSessionsPlan.ageDeletions]) pruning.deleteUnit(buildDir, unit);
  for (const area of plan.areas.filter((area) => area.name === "vite" || area.name === "agents")) for (const unit of area.ageDeletions) pruning.deleteUnit(area.name === "vite" ? viteRoot : agentsRoot, unit);
  assert.equal(existsSync(join(buildDir, "debug/build/pkg-a/hash1")), false);
  assert.equal(existsSync(join(targetDir, "debug/deps/libfoo-abcdef.rlib")), false);
  assert.equal(existsSync(join(targetDir, "debug/deps")), false, "Emptied target-dir parent must be pruned");
  assert.equal(existsSync(join(targetDir, "debug/examples/demo")), true);
  assert.equal(existsSync(join(buildDir, "debug/incremental/crate-old-1a2b")), false);
  assert.equal(existsSync(join(buildDir, "debug/incremental/crate-fresh-3c4d")), true);
  assert.equal(existsSync(join(buildDir, "debug/incremental/crate-working-7g8h")), true, "Never delete a crate dir holding a -working session");
  assert.equal(existsSync(join(buildDir, "debug/incremental/crate-working-7g8h/s-htestwk02-idwk0002-working")), true);
  assert.equal(existsSync(join(buildDir, "debug/incremental/crate-multi-5e6f")), true, "Only the stale session inside is compacted; the crate dir itself survives");
  assert.equal(existsSync(join(buildDir, "debug/incremental/crate-multi-5e6f/s-htestmu01-idmu0001-svhm1")), false);
  assert.equal(existsSync(join(buildDir, "debug/incremental/crate-multi-5e6f/s-htestmu01-idmu0001.lock")), false, "The stale session's own lock file is removed alongside it");
  assert.equal(existsSync(join(buildDir, "debug/incremental/crate-multi-5e6f/s-htestmu02-idmu0002-svhm2")), true);
  assert.equal(existsSync(join(viteRoot, "consumer-a")), false);
  assert.equal(existsSync(join(viteRoot, "consumer-b")), true);
  assert.equal(existsSync(join(agentsRoot, "stray-session")), false);
  assert.equal(existsSync(join(agentsRoot, "resource-leases")), true, "Lease store must never be pruned as a stray scratch dir");
  rmSync(root, { recursive: true, force: true });
  console.log("[DEBUG] Real Cargo build/target/incremental/vite/agents scanning, Python byte oracle, cancellation, session compaction, deletion and empty-parent pruning PASS");
}

/** 🧹️ Verifies the shared cache pruner: pure eviction contract, real filesystem scanning and deletion. */
export async function testCachePrune(output: string): Promise<void> {
  await testPlanCachePrune();
  await testCargoDiskScan(output);
}
