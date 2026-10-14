import {requireNativeStoragePathV1} from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts";
import {spawnSync} from "node:child_process";
import {mkdtempSync,writeFileSync,existsSync,readFileSync,rmSync} from "node:fs";
import {join,dirname} from "node:path";
import {tmpdir} from "node:os";
import {defaultBudgetMs,budgetTimeoutHint,type RunCmdOpts} from "../🟦️.ts";

export interface RunProbeResult {
  status: number | null;
  stdout: string;
  stderr: string;
  signal: NodeJS.Signals | null;
}

type CapturedSpawnOptions = Readonly<{
  cwd?: string;
  env?: NodeJS.ProcessEnv;
  input?: Buffer | string;
  timeout?: number;
  killSignal?: NodeJS.Signals | number;
}>;

type CapturedSpawnResult = Readonly<{
  status: number | null;
  stdout: Buffer;
  stderr: Buffer;
  signal: NodeJS.Signals | null;
  error?: Error;
}>;

const CAPTURE_WRAPPER_SOURCE = `
const { readFileSync, writeFileSync } = await import("node:fs");
const { spawnSync } = await import("node:child_process");
const spec = JSON.parse(readFileSync(process.env.SEMIO_PROCESS_CAPTURE_SPEC, "utf8"));
const input = spec.input === undefined ? undefined : Buffer.from(spec.input, "base64");
const result = spawnSync(spec.cmd, spec.args, { cwd: spec.cwd, env: spec.env, timeout: spec.timeout, killSignal: spec.killSignal, stdio: [input === undefined ? "ignore" : "pipe", "pipe", "pipe"], ...(input === undefined ? {} : { input }), maxBuffer: 536870912 });
writeFileSync(spec.stdoutPath, result.stdout ?? Buffer.alloc(0));
writeFileSync(spec.stderrPath, result.stderr ?? Buffer.alloc(0));
const sourceError = result.error;
const error = sourceError ? { message: sourceError.message, code: sourceError.code, errno: sourceError.errno, syscall: sourceError.syscall, path: sourceError.path, spawnargs: sourceError.spawnargs } : undefined;
writeFileSync(spec.metaPath, JSON.stringify({ status: result.status, signal: result.signal, error }));
`;

/** 📥️Captures subprocess bytes through a child-owned OS-file bridge so Bun test and native process hosts agree exactly. */
export function spawnCapturedSync(cmd: string, args: readonly string[], opts: CapturedSpawnOptions = {}): CapturedSpawnResult {
  const diagnostic=opts.env?.SEMIO_DIAGNOSTIC_DIRECTORY??process.env.SEMIO_DIAGNOSTIC_DIRECTORY;if(diagnostic)requireNativeStoragePathV1(dirname(diagnostic),diagnostic);const captureRoot=mkdtempSync(join(diagnostic??tmpdir(),"semio-process-capture-"));
  const stdoutPath = join(captureRoot, "stdout");
  const stderrPath = join(captureRoot, "stderr");
  const metaPath = join(captureRoot, "meta.json");
  const specPath = join(captureRoot, "spec.json");
  const env = Object.fromEntries(Object.entries(opts.env ?? process.env).filter((entry): entry is [string, string] => entry[1] !== undefined));
  writeFileSync(
    specPath,
    JSON.stringify({ cmd, args, cwd: opts.cwd, env, input: opts.input === undefined ? undefined : Buffer.from(opts.input).toString("base64"), timeout: opts.timeout, killSignal: opts.killSignal, stdoutPath, stderrPath, metaPath }),
  );
  try {
    const launcher = spawnSync(process.execPath, ["-e", CAPTURE_WRAPPER_SOURCE], {
      env: { ...process.env, SEMIO_PROCESS_CAPTURE_SPEC: specPath },
      timeout: opts.timeout ? opts.timeout + 5_000 : undefined,
      killSignal: opts.killSignal,
      stdio: "ignore",
    });
    if (!existsSync(metaPath)) return { status: launcher.status, stdout: Buffer.alloc(0), stderr: Buffer.alloc(0), signal: launcher.signal, error: launcher.error };
    const meta = JSON.parse(readFileSync(metaPath, "utf8")) as { status: number | null; signal: NodeJS.Signals | null; error?: NodeJS.ErrnoException };
    const error = meta.error ? Object.assign(new Error(meta.error.message), meta.error) : undefined;
    return { status: meta.status, stdout: readFileSync(stdoutPath), stderr: readFileSync(stderrPath), signal: meta.signal, error };
  } finally {
    rmSync(captureRoot, { recursive: true, force: true });
  }
}

/** 🔍️Budgeted capability probe with captured stdout/stderr — for `--version` checks that must not inherit stdio. */
export function runProbe(cmd: string, args: string[], opts: RunCmdOpts = {}): RunProbeResult {
  const budgetMs = opts.budgetMs ?? defaultBudgetMs(cmd);
  const result = spawnCapturedSync(cmd, args, {
    cwd: opts.cwd,
    env: opts.env ?? process.env,
    timeout: budgetMs,
    killSignal: "SIGKILL",
  });
  if (result.error) {
    if ((result.error as NodeJS.ErrnoException).code === "ETIMEDOUT") {
      console.error(`[budget] ${cmd} ${args.join(" ")} exceeded ${budgetMs}ms — killed. ${budgetTimeoutHint(cmd, opts.onTimeoutHint)}`);
    }
    throw result.error;
  }
  return {
    status: result.status,
    stdout: result.stdout.toString("utf8"),
    stderr: result.stderr.toString("utf8"),
    signal: result.signal,
  };
}

