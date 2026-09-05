import { test, expect } from "bun:test";
import { existsSync, mkdirSync, readFileSync, mkdtempSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { createHash } from "node:crypto";
import Ajv2020 from "ajv/dist/2020.js";
import { ExactCargoLawError, runExactCargoLawProcess, runExactCargoLaws, type ExactCargoLawPort } from "../../📦️packages/🟦️typescript/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("./🧪️fixture/🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("./🧬️schema.json", import.meta.url), "utf8"));

for (const mode of ["exit", "timeout", "cancelled", "output-limit"] as const) {
  test(`exact Cargo process capture retains actual ${mode} evidence`, async () => {
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
    const root = mkdtempSync(join(artifactRoot, "exact-process-fixture-"));
    const started = Date.now();
    const source = mode === "exit" ? "process.stdout.write('stdout');process.stderr.write('stderr');process.exitCode=7"
      : mode === "output-limit" ? "process.stdout.write('x'.repeat(16384));setInterval(()=>{},1000)"
      : "process.stdout.write('ready');setInterval(()=>{},1000)";
    const options = { cwd: root, env: process.env, budgetMs: mode === "timeout" ? 250 : 3000, maxOutputBytes: 1024, stdoutPath: join(root, "stdout"), stderrPath: join(root, "stderr"), cancelled: () => mode === "cancelled" && Date.now() - started >= 150 };
    const result = await runExactCargoLawProcess(process.execPath, ["-e", source], options);
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

test("exact Cargo law fixture has independent strict schema and dual SHA-256 identity", async () => {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  expect(validate({ ...fixture, extra: true })).toBe(false);
  const bytes = Buffer.from(fixture.executableBytesHex, "hex");
  expect(createHash("sha256").update(bytes).digest("hex")).toBe(fixture.executableSha256);
  expect(Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex")).toBe(fixture.executableSha256);
  expect(new Set(fixture.cases.map((row: any) => row.id)).size).toBe(19);
  expect(validate({ ...fixture, nativeArguments: ["--exact", "--test-threads=1", "--nocapture"] })).toBe(false);
});

test("active exact Cargo lease protects ticket evidence from workspace cleanup", async () => {
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
  const workspace = mkdtempSync(join(artifactRoot, "exact-cargo-clean-fixture-"));
  const ticket = join(workspace, ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️09", "☀️05", "ACTIVE-CARGO");
  const generated = join(ticket, "🗑️generated");
  const lease = join(generated, "run", `${fixture.activeLease.directoryPrefix}fixture`);
  try {
    mkdirSync(lease, { recursive: true });
    writeFileSync(join(ticket, "🎫️ticket.json"), '{"status":"closed"}');
    writeFileSync(join(lease, fixture.activeLease.manifestName), JSON.stringify({ version: fixture.activeLease.version, pid: process.pid }));
    const { CleanScript } = await import("../../../../../../../📜️script.ts");
    new CleanScript(workspace, workspace).run([]);
    expect(existsSync(generated)).toBe(true);
    rmSync(lease, { recursive: true });
    new CleanScript(workspace, workspace).run([]);
    expect(existsSync(generated)).toBe(false);
  } finally { rmSync(workspace, { recursive: true, force: true }); }
});

for (const row of fixture.cases) {
  test(`exact Cargo law runner: ${row.id}`, async () => {
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR must name the active ticket generated directory");
    const root = mkdtempSync(join(artifactRoot, "exact-cargo-fixture-"));
    const executable = resolve(root, process.platform === "win32" ? "fixture.exe" : "fixture");
    let builds = 0;
    let fingerprints = 0;
    let cancelled = row.mutation === "cancel-before";
    const calls: Array<{ command: string; args: string[] }> = [];
    const port: ExactCargoLawPort = {
      fingerprint(path) {
        expect(path).toBe(executable);
        fingerprints++;
        const changed = row.mutation === "hash-after-list" && fingerprints >= 3 || row.mutation === "hash-after-law" && fingerprints >= 5;
        return { path, sha256: changed ? "11".repeat(32) : fixture.executableSha256 };
      },
      async probe(command, args, options) {
        const leases = readdirSync(root).filter(name => name.startsWith(fixture.activeLease.directoryPrefix));
        expect(leases).toHaveLength(1);
        const lease = JSON.parse(readFileSync(join(root, leases[0], fixture.activeLease.manifestName), "utf8"));
        expect(lease).toEqual({ version: fixture.activeLease.version, pid: process.pid });
        calls.push({ command, args });
        expect(options.budgetMs).toBeGreaterThan(0);
        expect(options.maxOutputBytes).toBeGreaterThan(0);
        expect(options.stdoutPath.startsWith(root)).toBe(true);
        expect(options.env.CARGO_TARGET_DIR).toBe(join(root, "cargo-target"));
        if (command === "cargo") {
          builds++;
          expect(args.filter(arg => arg === "--no-run")).toHaveLength(1);
          expect(args).toContain("--message-format=json");
          expect(args).not.toContain("--list");
          const artifact = { reason: "compiler-artifact", package_id: "path+file:///fixture#fixture-package@0.1.0", target: { name: "fixture_laws", kind: ["test"] }, profile: { test: true }, executable };
          if (row.mutation === "wrong-target") artifact.target.name = "other";
          if (row.mutation === "relative-executable") artifact.executable = "relative";
          const artifacts = row.mutation === "missing-artifact" ? [] : row.mutation === "duplicate-artifact" ? [artifact, artifact] : [artifact];
          if (row.mutation === "cancel-after-build") cancelled = true;
          return { status: row.mutation === "build-exit" ? 101 : 0, signal: null, stdout: artifacts.map(item => JSON.stringify(item)).join("\n"), stderr: "fixture build diagnostic" };
        }
        expect(command).toBe(executable);
        if (args[0] === "--list") {
          const laws = row.mutation === "missing-law" ? ["first_law"] : row.mutation === "duplicate-law" ? [...fixture.laws, "first_law"] : fixture.laws;
          return { status: 0, signal: null, stdout: laws.map((law: string) => `${law}: test`).join("\n"), stderr: "" };
        }
        expect(args.slice(1)).toEqual(fixture.nativeArguments);
        const passed = row.mutation === "zero-pass" ? 0 : row.mutation === "two-pass" ? 2 : 1;
        const ignored = row.mutation === "ignored-law" ? 1 : 0;
        const captured = row.mutation === "debug-output" ? `\nsuccesses:\n---- ${args[0]} stdout ----\n${fixture.capturedOutput}\n\nsuccesses:\n    ${args[0]}\n\n` : row.mutation === "terminal-spoof" ? "\ntest result: ok. 1 passed; 0 failed; 0 ignored;\n" : "";
        return { status: row.mutation === "native-exit" ? 101 : 0, signal: null, stdout: `test ${args[0]} ... ${ignored ? "ignored" : "ok"}\n${captured}test result: ok. ${passed} passed; 0 failed; ${ignored} ignored; 0 measured; 0 filtered out; finished in 0.00s\n`, stderr: "" };
      },
    };
    let assertions = 0;
    let outcome = "denied";
    try {
      const env = { ...process.env, CARGO_TARGET_DIR: row.mutation === "outside-cargo-target" ? resolve(root.slice(0, root.lastIndexOf("🗑️generated")), "outside-target") : undefined };
      const receipts = await runExactCargoLaws({ cwd: root, artifactDir: root, env, groups: [{ package: fixture.package, target: fixture.target, laws: fixture.laws }], cancelled: () => cancelled }, port);
      assertions = receipts.reduce((sum, receipt) => sum + receipt.assertions, 0);
      expect(receipts[0]?.laws).toEqual(fixture.laws);
      expect(receipts[0]?.sha256).toBe(fixture.executableSha256);
      if (row.mutation === "debug-output") expect(readFileSync(join(receipts[0]!.artifactDir, "law-0.stdout"), "utf8")).toContain(fixture.capturedOutput);
      outcome = "passed";
    } catch (error) {
      if (row.mutation === "outside-cargo-target") expect(String(error)).toContain("absolute ticket-generated Cargo target");
      else expect(error).toBeInstanceOf(ExactCargoLawError);
      if (row.mutation === "build-exit" || row.mutation === "native-exit") expect((error as ExactCargoLawError).status).toBe(101);
    }
    expect(outcome).toBe(row.expected);
    expect(builds).toBe(row.builds);
    expect(assertions).toBe(row.assertions);
    expect(calls.filter(call => call.command === "cargo")).toHaveLength(row.builds);
    expect(readdirSync(root).filter(name => name.startsWith(fixture.activeLease.directoryPrefix))).toEqual([]);
  });
}
