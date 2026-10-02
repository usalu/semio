import { spawn } from "node:child_process";
import { mkdtempSync, readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { test, expect } from "bun:test";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import { captureOwnedProcess } from "../🟦️.ts";

const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!artifactRoot) throw Error("SEMIO_TEST_ARTIFACT_DIR is required");
type Vector = typeof fixture.cases[number];

/** 🔬️ Observes the same portable process contract through independent Node built-in pipes. */
function oracle(row: Vector, cwd: string): Promise<{ reason: string; status: number | null; stdout: string; stderr: string }> {
  return new Promise(accept => {
    const child = spawn(row.id === "spawn-error" ? join(cwd, "absent-process") : "node", ["-e", row.source], { cwd, env: process.env, stdio: ["ignore", "pipe", "pipe"] });
    let reason = "exit", stdout = Buffer.alloc(0), stderr = Buffer.alloc(0), count = 0;
    const stop = (cause: string) => { if (reason === "exit") { reason = cause; child.kill("SIGKILL"); } };
    const append = (bytes: Buffer, error: boolean) => {
      const retained = bytes.subarray(0, Math.max(0, row.maximumBytes - count));
      if (error) stderr = Buffer.concat([stderr, retained]); else stdout = Buffer.concat([stdout, retained]);
      count += bytes.length;
      if (count > row.maximumBytes) stop("output-limit");
    };
    child.stdout.on("data", bytes => append(bytes, false));
    child.stderr.on("data", bytes => append(bytes, true));
    child.on("error", error => { reason = "spawn-error"; append(Buffer.from(error.message), true); });
    const timeout = row.budgetMs ? setTimeout(() => stop("timeout"), row.budgetMs) : undefined;
    const cancellation = row.cancelAfterMs !== null ? setTimeout(() => stop("cancelled"), row.cancelAfterMs) : undefined;
    child.on("close", status => { clearTimeout(timeout); clearTimeout(cancellation); accept({ reason, status, stdout: stdout.toString("utf8"), stderr: stderr.toString("utf8") }); });
  });
}

test("owned process vectors have strict independent schema admission", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  expect(validate({ ...fixture, extra: true })).toBe(false);
  expect(new Set(fixture.cases.map(row => row.id)).size).toBe(6);
});

for (const row of fixture.cases) test(`owned process capture matches independent Node ${row.id} evidence`, async () => {
  const root = mkdtempSync(join(artifactRoot, "owned-capture-vector-"));
  const independent = await oracle(row, root);
  expect(independent.reason).toBe(row.reason);
  if (row.id !== "spawn-error") expect(independent.status).toBe(row.status);
  if (row.stdout !== null) expect(independent.stdout).toBe(row.stdout);
  if (row.stderr !== null) expect(independent.stderr).toBe(row.stderr);
  const started = Date.now();
  const options = { cwd: root, env: process.env, budgetMs: row.budgetMs, maxOutputBytes: row.maximumBytes, stdoutPath: join(root, "stdout"), stderrPath: join(root, "stderr"), cancelled: () => row.cancelAfterMs !== null && Date.now() - started >= row.cancelAfterMs };
  const result = await captureOwnedProcess(row.id === "spawn-error" ? join(root, "absent-process") : "node", ["-e", row.source], options);
  expect(result.reason).toBe(independent.reason);
  if (row.id !== "spawn-error") expect(result.status).toBe(independent.status);
  if (row.stdout !== null) expect(result.stdout).toBe(independent.stdout);
  if (row.stderr !== null) expect(result.stderr).toBe(independent.stderr);
  if (row.id === "output-limit") expect(result.stdout).toBe(independent.stdout);
  if (row.id === "spawn-error") expect(result.stderr).toContain("ENOENT");
  expect(Buffer.byteLength(result.stdout) + Buffer.byteLength(result.stderr)).toBeLessThanOrEqual(row.maximumBytes);
  expect(readFileSync(options.stdoutPath, "utf8")).toBe(result.stdout);
  expect(readFileSync(options.stderrPath, "utf8")).toBe(result.stderr);
  expect(Date.now() - started).toBeLessThan(5000);
});

test("owned process capture refuses cancellation before child execution", async () => {
  const root = mkdtempSync(join(artifactRoot, "owned-capture-precheck-")), marker = join(root, "spawned");
  const result = await captureOwnedProcess("node", ["-e", "require('node:fs').writeFileSync(process.argv[1],'spawned')", marker], { cwd: root, env: process.env, budgetMs: 0, maxOutputBytes: 1024, stdoutPath: join(root, "stdout"), stderrPath: join(root, "stderr"), cancelled: () => true });
  expect(result).toEqual({ status: null, signal: null, reason: "cancelled", stdout: "", stderr: "" });
  expect(existsSync(marker)).toBe(false);
});

for (const mode of ["exit", "timeout", "cancelled", "output-limit"] as const) {
  test(`exact Cargo process capture retains actual ${mode} evidence`, async () => {
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
    const root = mkdtempSync(join(artifactRoot, "exact-process-fixture-"));
    const started = Date.now();
    const source = mode === "exit" ? "process.stdout.write('stdout');process.stderr.write('stderr');process.exitCode=7"
      : mode === "output-limit" ? "process.stdout.write('x'.repeat(16384));setInterval(()=>{},1000)"
      : "process.stdout.write('ready');setInterval(()=>{},1000)";
    const options = { cwd: root, env: process.env, budgetMs: mode === "timeout" ? 250 : 0, maxOutputBytes: 1024, stdoutPath: join(root, "stdout"), stderrPath: join(root, "stderr"), cancelled: () => mode === "cancelled" && Date.now() - started >= 150 };
    const result = await captureOwnedProcess(process.execPath, ["-e", source], options);
    expect(result.reason).toBe(mode === "exit" ? "exit" : mode);
    expect(Buffer.byteLength(result.stdout) + Buffer.byteLength(result.stderr)).toBeLessThanOrEqual(1024);
    expect(readFileSync(options.stdoutPath, "utf8")).toBe(result.stdout);
    expect(readFileSync(options.stderrPath, "utf8")).toBe(result.stderr);
    if (mode === "exit") {
      expect(result.status).toBe(7);
      expect(result.stdout).toBe("stdout");
      expect(result.stderr).toBe("stderr");
    }
    expect(Date.now() - started).toBeLessThan(5000);
  });
}
