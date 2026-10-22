import { spawn, spawnSync, type ChildProcess, type StdioOptions } from "node:child_process";
import { constants } from "node:os";
import { Readable } from "node:stream";
import { runtimeMessageV1 } from "../../🗣️messages/🟦️.ts";
import type { RuntimeByteStreamV1, RuntimeChildV1, RuntimeSpawnOptionsV1, RuntimeSpawnSyncResultV1, RuntimeStdioV1 } from "../../📋️contract/🟦️.ts";

function environment(source: RuntimeSpawnOptionsV1["env"]): NodeJS.ProcessEnv {
  if (source === undefined) return process.env;
  const result: NodeJS.ProcessEnv = {};
  for (const [name, value] of Object.entries(source)) if (value !== undefined) result[name] = value;
  return result;
}

function wire(value: RuntimeSpawnOptionsV1["stdin"], fallback: RuntimeStdioV1): "pipe" | "inherit" | "ignore" {
  return value === undefined ? fallback : typeof value === "string" && (value === "pipe" || value === "inherit" || value === "ignore") ? value : "pipe";
}

function empty(): RuntimeByteStreamV1 { return new ReadableStream<Uint8Array>({ start: controller => controller.close() }) as unknown as RuntimeByteStreamV1; }
function webStream(stream: Readable | null): RuntimeByteStreamV1 { return stream === null ? empty() : (Readable.toWeb(stream) as unknown as RuntimeByteStreamV1); }
function exitNumber(code: number | null, signal: NodeJS.Signals | null): number { return code ?? 128 + (signal === null ? 0 : (constants.signals[signal] ?? 0)); }

/** 🚀️ Starts one child process over `node:child_process` with web streams; defaults are stdin ignored, stdout piped, stderr inherited. */
export function spawnV1(command: readonly string[], options: RuntimeSpawnOptionsV1 = {}): RuntimeChildV1 {
  if (command.length === 0) throw new TypeError(runtimeMessageV1("commandRequired", {}));
  const input = typeof options.stdin === "string" && !["pipe", "inherit", "ignore"].includes(options.stdin) || options.stdin instanceof Uint8Array ? options.stdin : undefined;
  const stdio: StdioOptions = [wire(options.stdin, "ignore"), wire(options.stdout, "pipe"), wire(options.stderr, "inherit")];
  const child: ChildProcess = spawn(command[0]!, command.slice(1), { cwd: options.cwd, env: environment(options.env), stdio, detached: options.detached, signal: options.signal, windowsHide: true });
  if (input !== undefined) child.stdin?.end(input);
  let timer: ReturnType<typeof setTimeout> | undefined;
  if (options.timeout !== undefined) timer = setTimeout(() => child.kill("SIGTERM"), options.timeout);
  let exitCode: number | null = null;
  const exited = new Promise<number>((accept, reject) => {
    child.once("error", error => { if (timer !== undefined) clearTimeout(timer); reject(error); });
    child.once("close", (code, signal) => { if (timer !== undefined) clearTimeout(timer); exitCode = exitNumber(code, signal); accept(exitCode); });
  });
  exited.catch(() => undefined);
  return {
    pid: child.pid ?? -1,
    exited,
    stdout: webStream(child.stdout),
    stderr: webStream(child.stderr),
    get exitCode() { return exitCode; },
    kill: signal => { child.kill(signal as NodeJS.Signals | number | undefined); },
  };
}

/** 🚀️ Runs one child to completion over `node:child_process`; output is unbounded and defaults pipe both streams. */
export function spawnSyncV1(command: readonly string[], options: RuntimeSpawnOptionsV1 = {}): RuntimeSpawnSyncResultV1 {
  if (command.length === 0) throw new TypeError(runtimeMessageV1("commandRequired", {}));
  const input = typeof options.stdin === "string" && !["pipe", "inherit", "ignore"].includes(options.stdin) || options.stdin instanceof Uint8Array ? options.stdin : undefined;
  const result = spawnSync(command[0]!, command.slice(1), { cwd: options.cwd, env: environment(options.env), stdio: [input === undefined ? wire(options.stdin, "ignore") : "pipe", wire(options.stdout, "pipe"), wire(options.stderr, "pipe")], input, timeout: options.timeout, maxBuffer: Infinity, windowsHide: true });
  if (result.error !== undefined && (result.error as NodeJS.ErrnoException).code === "ENOENT") throw result.error;
  return { exitCode: result.status, signalCode: result.signal, success: result.status === 0, stdout: result.stdout ?? Buffer.alloc(0), stderr: result.stderr ?? Buffer.alloc(0) };
}
