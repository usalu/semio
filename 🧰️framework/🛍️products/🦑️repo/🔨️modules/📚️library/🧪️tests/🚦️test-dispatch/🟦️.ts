import { expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { spawn, spawnSync } from "node:child_process";
import { tmpdir } from "node:os";
import { join } from "node:path";
import Ajv from "ajv";
import { findRepoRoot, runCanonicalGoTests } from "../../📦️packages/🟦️typescript/🟦️.ts";

const cli = join(import.meta.dir, "../../../💻️client/⌨️cli");
const repo = findRepoRoot(import.meta.dir);
const fixture = JSON.parse(readFileSync(join(cli, "🧫️fixtures/🚦️test-dispatch/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(cli, "🧬️schema/🚦️test-dispatch/🔣️.json"), "utf8"));
const script = join(import.meta.dir, "../../📦️packages/🟦️typescript/📜️script.ts");

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

test("public Go test dispatch uses canonical compiler inputs", async () => {
  const validate = new Ajv({ strict: false }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  await runCanonicalGoTests(cli, ["-count=1", "-v", "-run", "^TestCanonicalGoTestDispatcher(?:Cancellation|SpawnError)?$"], {
    env: { ...process.env, GOWORK: join(repo, "go.work") },
    budgetMs: 60_000,
  });
}, 65_000);

test("registered Go dispatch budget owns every nested process and overlay", async () => {
  if (process.platform === "win32") return;
  const artifactParent = process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir();
  mkdirSync(artifactParent, { recursive: true });
  const root = mkdtempSync(join(artifactParent, "go-cancellation-"));
  const temporary = join(root, "temporary");
  mkdirSync(temporary);
  const output = { stdout: "", stderr: "" };
  const child = spawn(process.execPath, [script, "go-test", cli, "-", "-count=1", "-v", "-run", "^TestCanonicalGoTestDispatcherNestedBudgetProbe$"], {
    cwd: repo,
    env: {
      ...process.env,
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
    console.log(`[DEBUG] nested budget returned in ${Date.now() - started}ms after observing ${observed.length} descendants across ${new Set(observed.map((row) => row.group)).size} groups`);
  } finally {
    cleanupOwnedProcesses(observed);
    rmSync(root, { recursive: true, force: true });
  }
}, 40_000);
