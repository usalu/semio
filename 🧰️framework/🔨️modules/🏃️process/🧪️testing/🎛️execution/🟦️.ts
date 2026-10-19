import { spawn, spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, realpathSync, rmSync } from "node:fs";
import { join, resolve, sep } from "node:path";
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

const scratchRoots = new Map<string, string>();
let exitGuard = false;

/** 🧲️ Lists the attached macOS disk images through the system tool, deepest backing file first. */
export function attachedDiskImagesV1(): readonly Readonly<{ image: string; target: string }>[] {
  if (process.platform !== "darwin") return [];
  const info = spawnSync("/usr/bin/hdiutil", ["info", "-plist"], { encoding: "utf8" });
  if (info.status !== 0) return [];
  const json = spawnSync("/usr/bin/plutil", ["-convert", "json", "-o", "-", "-"], { input: info.stdout, encoding: "utf8" });
  if (json.status !== 0) return [];
  const rows = (JSON.parse(json.stdout) as { images?: { "image-path"?: string; "system-entities"?: { "mount-point"?: string; "dev-entry"?: string }[] }[] }).images ?? [];
  return rows.flatMap((row) => {
    const entities = row["system-entities"] ?? [], target = entities.find((entity) => entity["mount-point"])?.["mount-point"] ?? entities[0]?.["dev-entry"];
    if (!row["image-path"] || !target) return [];
    let image = row["image-path"];
    try { image = realpathSync(image); } catch {}
    return [{ image, target }];
  }).sort((left, right) => right.image.length - left.image.length);
}

/** 🧲️ Detaches every disk image whose backing file lives under the root, nested images first. */
export function detachDiskImagesUnderV1(root: string): readonly string[] {
  let real = resolve(root);
  try { real = realpathSync(real); } catch {}
  const prefix = real + sep, detached: string[] = [];
  for (const { image, target } of attachedDiskImagesV1()) {
    if (!image.startsWith(prefix)) continue;
    const plain = spawnSync("/usr/bin/hdiutil", ["detach", target], { encoding: "utf8" });
    if (plain.status === 0 || spawnSync("/usr/bin/hdiutil", ["detach", "-force", target], { encoding: "utf8" }).status === 0) detached.push(image);
  }
  return detached;
}

/** 🗑️ Detaches every disk image under a scratch root created by this module, then removes the tree. */
export function disposeTestScratchRootV1(root: string): void {
  const real = scratchRoots.get(resolve(root));
  if (!real) throw Error("Scratch root created by createTestScratchRootV1 required");
  try {
    detachDiskImagesUnderV1(real);
    rmSync(real, { recursive: true, force: true });
  } finally {
    scratchRoots.delete(resolve(root));
  }
}

/** 🧹️ Creates a unique scratch root that is detached and removed on disposal and when the process exits. */
export function createTestScratchRootV1(parent: string, prefix: string): string {
  mkdirSync(parent, { recursive: true });
  const root = mkdtempSync(join(parent, prefix)), real = realpathSync(root);
  if (!exitGuard) process.on("exit", () => { for (const path of [...scratchRoots.keys()]) try { disposeTestScratchRootV1(path); } catch (error) { console.error(`[leak] scratch root ${path} not released: ${String(error)}`); } });
  exitGuard = true;
  scratchRoots.set(resolve(root), real);
  return root;
}
