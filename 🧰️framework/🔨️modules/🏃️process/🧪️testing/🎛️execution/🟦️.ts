import { spawn } from "node:child_process";
import { terminateOwnedProcessTree } from "../../🪓️termination/🟦️.ts";

/** 🎛️ Supplies a test command's complete caller-owned process policy. */
export type TestCommandOptions = Readonly<{
  cwd: string;
  budgetMs: number;
  env?: Readonly<Record<string, string | undefined>>;
  signal?: AbortSignal;
  onTimeoutHint?: string;
  throwOnFailure?: boolean;
  captureStdout?: Readonly<{ limitBytes: number; onChunk: (bytes: Uint8Array) => void }>;
}>;

/** ⏱️ Executes one explicitly bounded test process without selecting a tool or repository policy. */
export async function runBudgetedTestCommand(command: string, args: readonly string[], options: TestCommandOptions): Promise<void> {
  if (!Number.isSafeInteger(options.budgetMs) || options.budgetMs < 0) throw Error("Invalid test command budget");
  if (options.captureStdout && (!Number.isSafeInteger(options.captureStdout.limitBytes) || options.captureStdout.limitBytes < 1)) throw Error("Invalid captured output byte limit");
  if (!options.cwd) throw Error("Missing test command working directory");
  if (options.signal?.aborted) throw Error(`${command} cancelled`);
  const child = spawn(command, [...args], { stdio: options.captureStdout ? ["inherit", "pipe", "inherit"] : "inherit", cwd: options.cwd, env: options.env ?? process.env, detached: process.platform !== "win32", windowsHide: true });
  let reason: "timeout" | "cancelled" | "interrupted" | "output-limit" | undefined;
  const stop = (next: typeof reason): void => {
    reason ??= next;
    if (child.pid && child.exitCode === null && child.signalCode === null) terminateOwnedProcessTree(child.pid);
  };
  let bytes=0;
  child.stdout?.on("data", (chunk: Buffer) => { bytes += chunk.byteLength; if(bytes > options.captureStdout!.limitBytes) stop("output-limit"); else options.captureStdout!.onChunk(chunk); });
  const abort = (): void => stop("cancelled"), interrupt = (): void => stop("interrupted");
  options.signal?.addEventListener("abort", abort, { once: true });
  if (options.signal?.aborted) abort();
  process.once("SIGINT", interrupt);
  process.once("SIGTERM", interrupt);
  const timer = options.budgetMs > 0 ? setTimeout(() => stop("timeout"), options.budgetMs) : undefined;
  try {
    const { code, signal } = await new Promise<{ code: number | null; signal: string | null }>((accept, reject) => {
      child.once("error", reject);
      child.once("close", (code, signal) => accept({ code, signal }));
    });
    const failure = reason === "timeout" ? `${command} exceeded its ${options.budgetMs}ms budget` : reason ? `${command} ${reason}` : signal ? `${command} was killed by ${signal}` : code !== 0 ? `${command} exited with status ${code}` : undefined;
    if (!failure) return;
    if (reason === "timeout") console.error(`[budget] ${command} ${args.join(" ")} exceeded ${options.budgetMs}ms — killed. ${options.onTimeoutHint ?? "Trim it, or assign it to a higher level (quick/long/exhaustive)."}`);
    if (options.throwOnFailure) throw Error(failure);
    process.exit(reason || signal ? 1 : code ?? 1);
  } finally {
    if (timer) clearTimeout(timer);
    options.signal?.removeEventListener("abort", abort);
    process.off("SIGINT", interrupt);
    process.off("SIGTERM", interrupt);
  }
}
