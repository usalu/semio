import {captureOwnedProcess,type OwnedProcessCaptureOptions,type OwnedProcessCaptureResult} from "../../../📥️capture/🟦️.ts";
import { closeSync, existsSync, fstatSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, readSync, readdirSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { isAbsolute, join, resolve, sep } from "node:path";
import { createHash } from "node:crypto";
import { buildBudgetMs } from "../../../⏱️budget/🟦️.ts";

export type ExactCargoLawGroup = {
  package: string;
  target: { kind: "lib"; name?: string } | { kind: "test" | "bin"; name: string };
  laws: readonly string[];
  cargoArgs?: readonly string[];
};
export type ExactCargoLawStage = "build" | "list" | "native";
/** 🔌️ Owned process/fingerprint port permits deterministic hostile runner laws without compiling Cargo. */
export type ExactCargoLawPort = {
  probe: (command: string, args: string[], options: OwnedProcessCaptureOptions) => Promise<OwnedProcessCaptureResult>;
  fingerprint: (path: string) => { path: string; sha256: string };
};
export type ExactCargoLawOptions = {
  cwd: string;
  groups: readonly ExactCargoLawGroup[];
  manifestPaths: Readonly<Record<string, string>>;
  cargoTargetDir: string;
  cargoArgs?: readonly string[];
  env?: Readonly<Record<string, string | undefined>>;
  nativeEnv?: Readonly<Record<string, string | undefined>>;
  artifactDir?: string;
  buildBudgetMs?: number;
  listBudgetMs?: number;
  lawBudgetMs?: number;
  cancelled?: () => boolean;
  progress?: (event: { stage: ExactCargoLawStage; package: string; law?: string; artifactDir: string }) => void;
};
export type ExactCargoLawReceipt = {
  package: string;
  target: ExactCargoLawGroup["target"];
  executable: string;
  sha256: string;
  laws: readonly string[];
  assertions: number;
  artifactDir: string;
  cargoTargetDir: string;
};

export const EXACT_CARGO_ACTIVE_LEASE_DIRECTORY_PREFIX = ".exact-cargo-laws-active-";
export const EXACT_CARGO_ACTIVE_LEASE_MANIFEST = "lease.json";
export const EXACT_CARGO_ACTIVE_LEASE_MAX_AGE_MS = 120_000;

/** 🛡️ Recognizes only a fresh lease owned by a live exact-Cargo runner process. */
export function exactCargoGeneratedOutputHasLiveLease(root: string): boolean {
  const stack = [{ path: root, depth: 0 }];
  while (stack.length > 0) {
    const current = stack.pop()!;
    let names: string[];
    try {
      names = readdirSync(current.path);
    } catch {
      continue;
    }
    for (const name of names) {
      const path = join(current.path, name);
      let state;
      try {
        state = lstatSync(path);
      } catch {
        continue;
      }
      if (!state.isDirectory() || state.isSymbolicLink()) continue;
      if (name.startsWith(EXACT_CARGO_ACTIVE_LEASE_DIRECTORY_PREFIX)) {
        const manifestPath = join(path, EXACT_CARGO_ACTIVE_LEASE_MANIFEST);
        try {
          const manifestState = lstatSync(manifestPath);
          if (!manifestState.isFile() || manifestState.isSymbolicLink() || manifestState.size > 128 || Date.now() - manifestState.mtimeMs > EXACT_CARGO_ACTIVE_LEASE_MAX_AGE_MS || manifestState.mtimeMs - Date.now() > 5_000) continue;
          const manifest = JSON.parse(readFileSync(manifestPath, "utf8")) as { version?: unknown; pid?: unknown };
          if (manifest.version !== 1 || !Number.isSafeInteger(manifest.pid) || Number(manifest.pid) < 1) continue;
          try {
            process.kill(Number(manifest.pid), 0);
            return true;
          } catch (error) {
            if ((error as NodeJS.ErrnoException).code === "EPERM") return true;
          }
        } catch {
          continue;
        }
      } else if (current.depth < 4) stack.push({ path, depth: current.depth + 1 });
    }
  }
  return false;
}

/** 💓 Holds a fresh process-bound lease until the exact Cargo run reaches a terminal result. */
function beginExactCargoLease(artifactRoot: string): () => void {
  const leaseRoot = mkdtempSync(join(artifactRoot, EXACT_CARGO_ACTIVE_LEASE_DIRECTORY_PREFIX));
  const manifestPath = join(leaseRoot, EXACT_CARGO_ACTIVE_LEASE_MANIFEST);
  const heartbeat = (): void => {
    try {
      writeFileSync(manifestPath, JSON.stringify({ version: 1, pid: process.pid }), { mode: 0o600 });
    } catch {}
  };
  heartbeat();
  const timer = setInterval(heartbeat, 10_000);
  timer.unref?.();
  return () => {
    clearInterval(timer);
    rmSync(leaseRoot, { recursive: true, force: true });
  };
}

/** 🚫️ Preserves the precise failing stage and actual child status independently of assertion parsing. */
export class ExactCargoLawError extends Error {
  constructor(
    readonly stage: ExactCargoLawStage,
    readonly status: number | null,
    readonly signal: string | null,
    readonly artifactDir: string,
    detail: string,
  ) {
    super(`exact Cargo law ${stage} failed: status=${status} signal=${signal ?? "none"} artifacts=${artifactDir}; ${detail}`);
  }
}

/** 🔬️ Fingerprints one retained executable descriptor with bounded streaming and cancellation. */
export function exactExecutableFingerprint(path: string, control: Readonly<{ cancelled?: () => boolean; progress?: (completed: number, total: number) => void }> = {}): { path: string; sha256: string; byteLength: number } {
  const check = () => {
    if (control.cancelled?.()) throw new Error("Executable fingerprint cancelled");
  };
  check();
  if (!isAbsolute(path) || lstatSync(path).isSymbolicLink()) throw new Error("Executable must be one absolute regular file");
  const canonical = realpathSync(path);
  const descriptor = openSync(canonical, "r");
  try {
    const before = fstatSync(descriptor);
    const same = (other: typeof before) => other.isFile() && !other.isSymbolicLink() && other.dev === before.dev && other.ino === before.ino && other.size === before.size && other.mtimeMs === before.mtimeMs && other.ctimeMs === before.ctimeMs;
    if (!before.isFile() || !same(lstatSync(canonical)) || before.size <= 0 || before.size > 8 * 1024 ** 3 || (process.platform !== "win32" && (before.mode & 0o111) === 0)) throw new Error("Executable size or type denied");
    const digest = createHash("sha256");
    const buffer = Buffer.alloc(64 * 1024);
    let count = 0;
    while (true) {
      check();
      const length = readSync(descriptor, buffer);
      if (length === 0) break;
      digest.update(buffer.subarray(0, length));
      count += length;
      if (count > before.size) throw new Error("Executable changed while hashing");
      control.progress?.(count, before.size);
    }
    const after = fstatSync(descriptor);
    if (count !== before.size || !same(after) || !same(lstatSync(canonical))) throw new Error("Executable changed while hashing");
    return { path: canonical, sha256: digest.digest("hex"), byteLength: count };
  } finally {
    closeSync(descriptor);
  }
}

/** 🧪️ Compiles each explicit target once and executes only its hash-bound, exact-listed native laws. */
export async function runExactCargoLaws(options: ExactCargoLawOptions, port: ExactCargoLawPort = { probe: captureOwnedProcess, fingerprint: exactExecutableFingerprint }): Promise<readonly ExactCargoLawReceipt[]> {
  const configuredEnv = options.env ?? process.env;
  const artifactRoot = options.artifactDir ?? configuredEnv.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot || !isAbsolute(artifactRoot)) throw new Error("Exact Cargo laws require an absolute artifactDir or SEMIO_TEST_ARTIFACT_DIR");
  const cargoTargetDir = options.cargoTargetDir;
  if (!cargoTargetDir || !isAbsolute(cargoTargetDir)) throw new Error("Explicit absolute Cargo target directory required");
  const targetBoundary = process.platform === "win32" ? cargoTargetDir.toLowerCase() : cargoTargetDir;
  const sourceBoundary = process.platform === "win32" ? resolve(options.cwd).toLowerCase() : resolve(options.cwd);
  if (sourceBoundary === targetBoundary || sourceBoundary.startsWith(targetBoundary + sep)) throw new Error("Cargo target must not contain the source workspace");
  const env = { ...configuredEnv, CARGO_TARGET_DIR: cargoTargetDir };
  const nativeEnv: Record<string, string | undefined> = { ...env, ...options.nativeEnv, CARGO_TARGET_DIR: cargoTargetDir };
  delete nativeEnv.RUST_TEST_NOCAPTURE;
  if (!isAbsolute(options.cwd) || !options.groups.length || options.groups.length > 64) throw new Error("Exact Cargo laws require a bounded nonempty target list and absolute cwd");
  const groupKeys = options.groups.map((group) => JSON.stringify([group.package, group.target.kind, group.target.name ?? ""]));
  if (new Set(groupKeys).size !== groupKeys.length) throw new Error("Exact Cargo groups must combine laws for the same package/target");
  for (const group of options.groups) {
    if (!isAbsolute(options.manifestPaths[group.package] ?? "")) throw new Error("Explicit absolute Cargo manifest required for each package");
    if (!group.package || !group.laws.length || group.laws.length > 4096 || new Set(group.laws).size !== group.laws.length || group.laws.some((law) => !/^[A-Za-z_][A-Za-z0-9_:]*$/u.test(law)))
      throw new Error("Exact Cargo law identities must be nonempty and unique");
  }
  mkdirSync(artifactRoot, { recursive: true });
  const endLease = beginExactCargoLease(artifactRoot);
  try {
    const runRoot = mkdtempSync(join(artifactRoot, "exact-cargo-laws-"));
    const cancelled = options.cancelled ?? (() => false);
    const receipts: ExactCargoLawReceipt[] = [];
    const checkedBudget = (value: number, build: boolean): number => {
      if (!Number.isSafeInteger(value) || value < (build ? 0 : 1) || value > 24 * 60 * 60 * 1000) throw new Error("Exact Cargo budget must be finite and positive, or zero for builds");
      return value;
    };
    for (const [index, group] of options.groups.entries()) {
      const groupRoot = join(runRoot, String(index).padStart(2, "0"));
      mkdirSync(groupRoot);
      let stage: ExactCargoLawStage = "build";
      let last: OwnedProcessCaptureResult = { status: null, signal: null, stdout: "", stderr: "" };
      const fail = (detail: string): never => {
        throw new ExactCargoLawError(stage, last.status, last.signal, groupRoot, detail);
      };
      const checkpoint = (): void => {
        if (cancelled()) fail("cancelled");
      };
      const capture = async (next: ExactCargoLawStage, command: string, args: string[], budget: number, name: string): Promise<OwnedProcessCaptureResult> => {
        stage = next;
        checkpoint();
        options.progress?.({ stage, package: group.package, ...(next === "native" ? { law: args[0] } : {}), artifactDir: groupRoot });
        const stdoutPath = join(groupRoot, `${name}.stdout`);
        const stderrPath = join(groupRoot, `${name}.stderr`);
        last = await port.probe(command, args, { cwd: options.cwd, env: next === "build" ? env : nativeEnv, budgetMs: checkedBudget(budget, next === "build"), maxOutputBytes: next === "build" ? 256 * 1024 * 1024 : 8 * 1024 * 1024, stdoutPath, stderrPath, cancelled });
        if (!existsSync(stdoutPath)) writeFileSync(stdoutPath, last.stdout, { flag: "wx", mode: 0o600 });
        if (!existsSync(stderrPath)) writeFileSync(stderrPath, last.stderr, { flag: "wx", mode: 0o600 });
        writeFileSync(join(groupRoot, `${name}.json`), JSON.stringify({ command, args, cargoTargetDir, status: last.status, signal: last.signal, reason: last.reason ?? "exit" }), { flag: "wx", mode: 0o600 });
        checkpoint();
        return last;
      };
      const target = group.target.kind === "lib" ? ["--lib"] : [`--${group.target.kind}`, group.target.name];
      const cargoArgs = [...(options.cargoArgs ?? []), ...(group.cargoArgs ?? [])];
      if (
        cargoArgs.some(
          (arg) =>
            ["--", "--test", "--bin", "--lib", "-p", "--package", "--no-run", "--message-format", "--target-dir", "--manifest-path"].includes(arg) || ["--message-format=", "--target-dir=", "--manifest-path="].some((prefix) => arg.startsWith(prefix)),
        )
      )
        fail("Cargo target/control arguments are helper-owned");
      const built = await capture(
        "build",
        "cargo",
        ["test", "--manifest-path", options.manifestPaths[group.package]!, "-p", group.package, ...target, ...cargoArgs, "--no-run", "--message-format=json"],
        options.buildBudgetMs ?? buildBudgetMs(),
        "build",
      );
      const messages = built.stdout.split("\n").flatMap((line) => {
        try {
          return [JSON.parse(line)];
        } catch {
          return [];
        }
      });
      const errors = messages.filter((message) => message.reason === "compiler-message" && message.message?.level === "error").map((message) => message.message.rendered ?? message.message.message);
      if (built.status !== 0 || built.signal !== null || (built.reason && built.reason !== "exit")) fail(`${built.reason ?? "exit"}; ${(errors.length ? errors.slice(0, 3).join("\n") : built.stderr).slice(0, 6000)}`);
      const artifacts = messages.filter((message) => message.reason === "compiler-artifact" && message.profile?.test === true && typeof message.executable === "string");
      if (artifacts.length !== 1) fail(`expected one Cargo executable artifact, got ${artifacts.length}`);
      const artifact = artifacts[0];
      const packageId = String(artifact.package_id);
      const packageName = packageId.includes("#") ? packageId.slice(packageId.lastIndexOf("#") + 1).split("@")[0] : packageId.split(" ")[0];
      const kinds = artifact.target?.kind;
      if (
        packageName !== group.package ||
        !Array.isArray(kinds) ||
        !kinds.some((kind) => (group.target.kind === "lib" ? ["lib", "rlib", "dylib", "cdylib", "staticlib", "proc-macro"].includes(kind) : kind === group.target.kind)) ||
        (group.target.name && artifact.target?.name !== group.target.name) ||
        !isAbsolute(artifact.executable)
      )
        fail("Cargo executable package/target/path does not match the explicit group");
      const fingerprint = (): { path: string; sha256: string } => {
        try {
          return port.fingerprint(artifact.executable);
        } catch (error) {
          return fail(String(error));
        }
      };
      const initial = fingerprint();
      const verify = (): void => {
        checkpoint();
        const current = fingerprint();
        if (current.path !== initial.path || current.sha256 !== initial.sha256) fail("Cargo executable changed after its build receipt");
      };
      if (!isAbsolute(initial.path) || !/^[0-9a-f]{64}$/u.test(initial.sha256)) fail("Cargo executable fingerprint is invalid");
      writeFileSync(join(groupRoot, "executable.json"), JSON.stringify({ package: group.package, target: group.target, ...initial }), { flag: "wx", mode: 0o600 });
      verify();
      const listed = await capture("list", initial.path, ["--list"], options.listBudgetMs ?? 60_000, "list");
      verify();
      if (listed.status !== 0 || listed.signal !== null || (listed.reason && listed.reason !== "exit")) fail(`list ${listed.reason ?? "exit"}; ${listed.stderr.slice(0, 4000)}`);
      const discovered = listed.stdout
        .split(/\r?\n/u)
        .filter((line) => line.endsWith(": test"))
        .map((line) => line.slice(0, -6));
      const laws = group.laws.map((selector) => {
        const matches = discovered.filter((name) => name === selector || name.endsWith(`::${selector}`));
        if (matches.length !== 1) fail(`expected exactly one ${selector}, selected=${matches.length}`);
        return matches[0]!;
      });
      if (new Set(laws).size !== laws.length) fail("Law selectors resolve to the same native assertion");
      for (const [lawIndex, law] of laws.entries()) {
        verify();
        const result = await capture("native", initial.path, [law, "--exact", "--test-threads=1", "--show-output"], options.lawBudgetMs ?? 60_000, `law-${lawIndex}`);
        verify();
        const terminals = [...result.stdout.matchAll(/^test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;/gm)];
        if (
          result.status !== 0 ||
          result.signal !== null ||
          (result.reason && result.reason !== "exit") ||
          terminals.length !== 1 ||
          terminals[0]?.[1] !== "1" ||
          terminals[0]?.[2] !== "0" ||
          terminals[0]?.[3] !== "0" ||
          !result.stdout.split(/\r?\n/u).includes(`test ${law} ... ok`)
        )
          fail(`native assertion ${law} did not pass exactly once; ${(result.stdout + result.stderr).slice(-6000)}`);
      }
      const receipt = { package: group.package, target: group.target, executable: initial.path, sha256: initial.sha256, laws, assertions: laws.length, artifactDir: groupRoot, cargoTargetDir };
      writeFileSync(join(groupRoot, "receipt.json"), JSON.stringify(receipt), { flag: "wx", mode: 0o600 });
      receipts.push(receipt);
    }
    return receipts;
  } finally {
    endLease();
  }
}

