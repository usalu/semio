import { afterAll, beforeAll, expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { spawn, spawnSync } from "node:child_process";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { runCanonicalGoTests, runRepositoryTestCommand } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { findWorkspaceRoot } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";

const cli = join(import.meta.dir, "../../../💻️client/⌨️cli");
const repo = findWorkspaceRoot(import.meta.dir);
const fixture = JSON.parse(readFileSync(join(cli, "🧫️fixtures/🚦️test-dispatch/🔣️.json"), "utf8"));
const script = join(import.meta.dir, "../../📦️packages/🟦️typescript/📜️script.ts");

let compiledRoot: string;
let probeBinary: string;
let probeRoot: string;
const setupCancellation = new AbortController();
beforeAll(async () => {
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir();
  mkdirSync(artifacts, { recursive: true });
  compiledRoot = mkdtempSync(join(artifacts, "go-dispatch-compiled-"));
  probeRoot = mkdtempSync(join(artifacts, "go-cancellation-"));
  mkdirSync(join(probeRoot, "temporary"));
  probeBinary = join(compiledRoot, process.platform === "win32" ? "dispatch.test.exe" : "dispatch.test");
  console.log("[DEBUG] preparing canonical Go cancellation executable with a separate compiler budget");
  try {
    await runCanonicalGoTests(cli, ["-c", "-o", probeBinary], { env: { ...process.env, GOWORK: join(repo, "go.work"), SEMIO_GO_OVERLAY_OWNER: compiledRoot }, budgetMs: 60_000, packages: ["."], signal: setupCancellation.signal });

  } catch (error) {
    rmSync(compiledRoot, { recursive: true, force: true });
    rmSync(probeRoot, { recursive: true, force: true });
    throw error;
  }
  expect(existsSync(probeBinary)).toBe(true);
  console.log("[DEBUG] canonical Go cancellation executable prepared");
}, 65_000);
beforeAll(async () => {
  console.log("[DEBUG] preparing the exact nested Go cancellation package under its stable owned path");
    await runRepositoryTestCommand(probeBinary, ["-test.v", "-test.count=1", "-test.run=^TestCanonicalGoTestDispatcherNestedBudgetPreparation$"], {
      cwd: cli, env: { ...process.env, GOWORK: join(repo, "go.work"), SEMIO_GO_CANCELLATION_ROOT: probeRoot, TMPDIR: join(probeRoot, "temporary"), TMP: join(probeRoot, "temporary"), TEMP: join(probeRoot, "temporary") }, budgetMs: 60_000, signal: setupCancellation.signal, throwOnFailure: true,
    });
  console.log("[DEBUG] both Go compiler phases completed before the 12000ms timed probe");
}, 65_000);
afterAll(() => { setupCancellation.abort(); if (compiledRoot) rmSync(compiledRoot, { recursive: true, force: true }); if (probeRoot) rmSync(probeRoot, { recursive: true, force: true }); });

type ProcessRow = { pid: number; parent: number; group: number };

function processTable(): ProcessRow[] {
  const result = spawnSync("ps", ["-axo", "pid=,ppid=,pgid="], { encoding: "utf8" });
  if (result.status !== 0) throw new Error(result.stderr || "ps failed");
  return result.stdout.split("\n").flatMap((line) => {
    const fields = line.trim().split(/\s+/).map(Number);
    return fields.length === 3 && fields.every(Number.isSafeInteger) ? [{ pid: fields[0]!, parent: fields[1]!, group: fields[2]! }] : [];
  });
}

function ownedProcesses(root: number): ProcessRow[] {
  const rows = processTable(), owned = new Set([root]), result: ProcessRow[] = [];
  for (let changed = true; changed;) {
    changed = false;
    for (const row of rows) {
      if (owned.has(row.pid) || !owned.has(row.parent)) continue;
      owned.add(row.pid);
      result.push(row);
      changed = true;
    }
  }
  const owner = rows.find((row) => row.pid === root);
  return owner ? [...result, owner] : result;
}

function cleanupOwnedProcesses(rows: readonly ProcessRow[]): void {
  const pids = new Set(rows.map((row) => row.pid));
  for (const group of new Set(rows.filter((row) => pids.has(row.group)).map((row) => row.group))) {
    try { process.kill(-group, "SIGKILL"); } catch {}
  }
  for (const { pid } of [...rows].reverse()) {
    try { process.kill(pid, "SIGKILL"); } catch {}
  }
}

async function waitForPath(path: string, milliseconds: number): Promise<void> {
  const deadline = Date.now() + milliseconds;
  while (Date.now() < deadline) {
    if (existsSync(path)) return;
    await Bun.sleep(20);
  }
  throw new Error(`Timed out waiting for ${path}`);
}

async function expectProcessesGone(rows: readonly ProcessRow[]): Promise<void> {
  const deadline = Date.now() + 3_000;
  while (Date.now() < deadline) {
    const live = new Set(processTable().map((row) => row.pid));
    if (rows.every((row) => !live.has(row.pid))) return;
    await Bun.sleep(20);
  }
  const live = new Set(processTable().map((row) => row.pid));
  throw new Error(`Owned descendants survived: ${rows.filter((row) => live.has(row.pid)).map((row) => row.pid).join(", ")}`);
}

test("canonical Go preparation respects caller cancellation before compiler launch", async () => {
  const cancellation = new AbortController();
  cancellation.abort();
  await expect(runCanonicalGoTests(cli, ["-c", "-o", join(compiledRoot, "cancelled.test")], { env: { ...process.env, SEMIO_GO_OVERLAY_OWNER: compiledRoot }, packages: ["."], budgetMs: 60_000, signal: cancellation.signal })).rejects.toThrow("cancelled");
  expect(existsSync(join(compiledRoot, "cancelled.test"))).toBe(false);
});

test("public Go test dispatch uses canonical compiler inputs", async () => {
  await runCanonicalGoTests(cli, ["-count=1", "-v", "-run", "^TestCanonicalGoTestDispatcher(?:Cancellation|SpawnError)?$"], {
    env: { ...process.env, GOWORK: join(repo, "go.work") },
    budgetMs: 60_000,
  });
}, 65_000);

test("registered Go dispatch budget owns every nested process and overlay", async () => {
  if (process.platform === "win32") return;
  const artifactParent = process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir();
  mkdirSync(artifactParent, { recursive: true });
  const root = probeRoot;
  const temporary = join(root, "temporary");
  const output = { stdout: "", stderr: "" };
  const child = spawn(process.execPath, [script, "go-test", "binary", cli, probeBinary, "TestCanonicalGoTestDispatcherNestedBudgetProbe"], {
    cwd: repo,
    env: {
      ...process.env,
      SEMIO_TEST_ARTIFACT_DIR: process.env.SEMIO_TEST_ARTIFACT_DIR ?? compiledRoot,
      GOWORK: join(repo, "go.work"),
      SEMIO_GO_CANCELLATION_ROOT: root,
      SEMIO_TEST_BUDGET_MS: "12000",
      TMPDIR: temporary,
      TMP: temporary,
      TEMP: temporary,
    },
    stdio: ["ignore", "pipe", "pipe"],
  });
  child.stdout!.setEncoding("utf8");
  child.stderr!.setEncoding("utf8");
  child.stdout!.on("data", (chunk: string) => { output.stdout += chunk; });
  child.stderr!.on("data", (chunk: string) => { output.stderr += chunk; });
  const completion = new Promise<{ code: number | null; signal: NodeJS.Signals | null }>((resolve, reject) => {
    child.once("error", reject);
    child.once("close", (code, signal) => resolve({ code, signal }));
  });
  let observed: ProcessRow[] = [];
  try {
    await waitForPath(join(root, "cancellation", fixture.cancellation.ready), 20_000);
    observed = ownedProcesses(child.pid!);
    expect(observed.length).toBeGreaterThanOrEqual(4);
    expect(new Set(observed.map((row) => row.group)).size).toBeGreaterThanOrEqual(2);
    const ownedPids = new Set(observed.map((row) => row.pid));
    const depth = (row: ProcessRow): number => {
      let current = row, value = 0;
      while (ownedPids.has(current.parent)) {
        current = observed.find((candidate) => candidate.pid === current.parent)!;
        value++;
      }
      return value;
    };
    const nestedGroup = observed.filter((row) => row.pid === row.group).sort((left, right) => depth(right) - depth(left))[0];
    expect(nestedGroup).toBeDefined();
    process.kill(-nestedGroup!.group, "SIGSTOP");
    const started = Date.now();
    const outcome = await Promise.race([
      completion,
      Bun.sleep(17_000).then(() => { throw new Error("Nested Go budget did not return within its bound"); }),
    ]);
    expect(Date.now() - started).toBeLessThan(17_000);
    expect(outcome.code).not.toBe(0);
    expect(output.stderr).toContain("exceeded 12000ms");
    await expectProcessesGone(observed);
    const marker = join(root, "cancellation", fixture.cancellation.marker);
    const before = readFileSync(marker, "utf8");
    await Bun.sleep(200);
    expect(readFileSync(marker, "utf8")).toBe(before);
    expect(readdirSync(temporary).filter((name) => name.startsWith("semio-go-tests-"))).toEqual([]);
  } catch (error) {
    throw new Error(String(error) + "\n" + output.stderr + "\n" + output.stdout);
  } finally {
    cleanupOwnedProcesses([...observed, ...ownedProcesses(child.pid!)]);
    rmSync(root, { recursive: true, force: true });
  }
}, 40_000);
