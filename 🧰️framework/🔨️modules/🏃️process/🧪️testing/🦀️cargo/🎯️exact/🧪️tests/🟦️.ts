import { ExactCargoLawError, exactExecutableFingerprint, type ExactCargoLawPort } from "../🟦️.ts";
import { test, expect } from "bun:test";
import { readFileSync, mkdtempSync, readdirSync, renameSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { createHash } from "node:crypto";

import { runExactCargoLaws } from "../🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));

test("exact executable fingerprint retains identity, exposes progress and refuses cancellation or path replacement", async () => {
  const root = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!, "executable-fingerprint-"));
  const path = join(root, "fixture");
  const bytes = Buffer.from(fixture.executableBytesHex, "hex");
  writeFileSync(path, bytes, { mode: 0o700 });
  const progress: number[][] = [];
  const receipt = exactExecutableFingerprint(path, { progress: (completed, total) => progress.push([completed, total]) });
  expect(receipt).toEqual({ path, sha256: fixture.executableSha256, byteLength: bytes.byteLength });
  expect(receipt.sha256).toBe(Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex"));
  expect(progress.at(-1)).toEqual([bytes.byteLength, bytes.byteLength]);
  expect(() => exactExecutableFingerprint(path, { cancelled: () => true })).toThrow("cancelled");
  let cancelled = false;
  expect(() => exactExecutableFingerprint(path, { cancelled: () => cancelled, progress: () => { cancelled = true; } })).toThrow("cancelled");
  let replaced = false;
  expect(() => exactExecutableFingerprint(path, { progress() {
    if (replaced) return;
    renameSync(path, path + ".retained");
    writeFileSync(path, bytes, { mode: 0o700 });
    replaced = true;
  } })).toThrow("changed while hashing");
  expect(replaced).toBe(true);
});

test("exact Cargo law fixture has independent strict schema and dual SHA-256 identity", async () => {
  const bytes = Buffer.from(fixture.executableBytesHex, "hex");
  expect(createHash("sha256").update(bytes).digest("hex")).toBe(fixture.executableSha256);
  expect(Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex")).toBe(fixture.executableSha256);
  expect(new Set(fixture.cases.map((row: any) => row.id)).size).toBe(20);
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
        if (command === "cargo") expect(options.budgetMs).toBe(0);
        else expect(options.budgetMs).toBeGreaterThan(0);
        expect(options.maxOutputBytes).toBeGreaterThan(0);
        expect(options.stdoutPath.startsWith(root)).toBe(true);
        expect(options.env.CARGO_TARGET_DIR).toBe(join(artifactRoot,fixture.compilerStorageDirectory));
        expect(options.env.SEMIO_STAGE_ENV_LAW).toBe(fixture.stageEnvironment.sharedValue);
        expect(options.env.RUST_MIN_STACK).toBe(command === "cargo" ? fixture.stageEnvironment.buildStack : fixture.stageEnvironment.nativeStack);
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
        const result = row.mutation === "native-output-override" && options.env.RUST_TEST_NOCAPTURE === "1" ? `${fixture.capturedOutput}\nok` : ignored ? "ignored" : "ok";
        return { status: row.mutation === "native-exit" ? 101 : 0, signal: null, stdout: `test ${args[0]} ... ${result}\n${captured}test result: ok. ${passed} passed; 0 failed; ${ignored} ignored; 0 measured; 0 filtered out; finished in 0.00s\n`, stderr: "" };
      },
    };
    let assertions = 0;
    let outcome = "denied";
    try {
      const env = { ...process.env, RUST_MIN_STACK: fixture.stageEnvironment.buildStack, SEMIO_STAGE_ENV_LAW: fixture.stageEnvironment.sharedValue, CARGO_TARGET_DIR: row.mutation === "source-cargo-target" ? root : undefined, ...(row.mutation === "native-output-override" ? { RUST_TEST_NOCAPTURE: "1" } : {}) };
      const receipts = await runExactCargoLaws({ cwd: root, artifactDir: root, manifestPaths:{[fixture.package]:join(root,"Cargo.toml")}, cargoTargetDir:row.mutation === "source-cargo-target" ? root : join(artifactRoot,fixture.compilerStorageDirectory), env, nativeEnv: { RUST_MIN_STACK: fixture.stageEnvironment.nativeStack, ...(row.mutation === "native-output-override" ? { RUST_TEST_NOCAPTURE: "1" } : {}) }, groups: [{ package: fixture.package, target: fixture.target, laws: fixture.laws }], cancelled: () => cancelled }, port);
      assertions = receipts.reduce((sum, receipt) => sum + receipt.assertions, 0);
      expect(receipts[0]?.laws).toEqual(fixture.laws);
      expect(receipts[0]?.sha256).toBe(fixture.executableSha256);
      if (row.mutation === "debug-output") expect(readFileSync(join(receipts[0]!.artifactDir, "law-0.stdout"), "utf8")).toContain(fixture.capturedOutput);
      outcome = "passed";
    } catch (error) {
      if (row.mutation === "source-cargo-target") expect(String(error)).toContain("Cargo target must not contain the source workspace");
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


test("exact Cargo law counts match the portable schema before any compiler admission", async () => {
  for (const vector of fixture.lawLimitVectors) {
    const laws = Array.from({ length: vector.count }, (_, index) => `corpus::law_${index}`);
    const root = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!, "law-count-"));
    let built = false;
    const port: ExactCargoLawPort = {
      fingerprint() { throw new Error("Denied build has no executable"); },
      async probe(command) {
        expect(command).toBe("cargo");
        built = true;
        return { status: 101, signal: null, stdout: "", stderr: "controlled admission witness" };
      },
    };
    try {
      await runExactCargoLaws({ cwd: root, artifactDir: root, manifestPaths:{[fixture.package]:join(root,"Cargo.toml")}, cargoTargetDir:join(process.env.SEMIO_TEST_ARTIFACT_DIR!,fixture.compilerStorageDirectory), groups: [{ package: fixture.package, target: fixture.target, laws }] }, port);
      throw new Error("Controlled compiler refusal must terminate");
    } catch (error) {
      if (vector.accepted) expect(error).toBeInstanceOf(ExactCargoLawError);
      else expect(String(error)).toContain("Exact Cargo law identities must be nonempty and unique");
    }
    expect(built).toBe(vector.accepted);
  }
});
