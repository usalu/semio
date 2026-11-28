import { spawn, type ChildProcess } from "node:child_process";
import { fileURLToPath } from "node:url";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 📐️ The grace and the log folder of the reaper.
 * @see ./🧬️schema/🔣️.json */
export const REAPER_POLICY_V1 = schema.definitions.Policy.const;

/** 📨️ One line of the pipe between a parent and its sentinel: register a child by pid and the time it was spawned, unregister it, or stop its registered tree now (a stop that left the registration in place: the sentinel ends the whole tree, on Windows through the job object of the child). */
export type ReaperLineV1 = Readonly<{ kind: "add"; pid: number; spawnedMs: number }> | Readonly<{ kind: "remove" | "stop"; pid: number }>;

/** ✍️ The text of one line, newline included. A pure function. */
export function reaperLineTextV1(line: ReaperLineV1): string {
  return line.kind === "add" ? `add ${line.pid} ${line.spawnedMs}\n` : `${line.kind} ${line.pid}\n`;
}

/** 👁️ The line a text spells, `undefined` for anything else (a sentinel never acts on a line it does not understand). A pure function. */
export function parseReaperLineV1(text: string): ReaperLineV1 | undefined {
  const found = /^(add|remove|stop) (\d{1,10})(?: (\d{1,16}))?$/u.exec(text.trim());
  if (found === null) return undefined;
  const pid = Number(found[2]);
  if (!Number.isSafeInteger(pid) || pid < 1) return undefined;
  if (found[1] !== "add") return found[3] === undefined ? { kind: found[1] as "remove" | "stop", pid } : undefined;
  return found[3] === undefined ? undefined : { kind: "add", pid, spawnedMs: Number(found[3]) };
}

/** 🔌️ How the sentinel is started: the program, its arguments and the spawn, replaced by a test. */
export interface ReaperHostV1 { readonly spawn?: typeof spawn; readonly program?: string; readonly script?: string; readonly parent?: number }

interface Sentinel { readonly child: ChildProcess; readonly parent: number }
let sentinel: Sentinel | undefined;

function sentinelFor(host: ReaperHostV1): Sentinel | undefined {
  const parent = host.parent ?? process.pid;
  if (sentinel !== undefined && sentinel.parent === parent && sentinel.child.exitCode === null && sentinel.child.signalCode === null && sentinel.child.stdin?.writable === true) return sentinel;
  try {
    const child = (host.spawn ?? spawn)(host.program ?? process.execPath, [host.script ?? fileURLToPath(new URL("./📜️script.ts", import.meta.url)), String(parent)], { detached: true, stdio: ["pipe", "ignore", "ignore"], windowsHide: true });
    child.on("error", () => { if (sentinel?.child === child) sentinel = undefined; });
    child.once("exit", () => { if (sentinel?.child === child) sentinel = undefined; });
    child.stdin?.on("error", () => { if (sentinel?.child === child) sentinel = undefined; });
    child.unref();
    (child.stdin as unknown as { unref?: () => void } | null)?.unref?.();
    sentinel = { child, parent };
    return sentinel;
  } catch {
    sentinel = undefined;
    return undefined;
  }
}

/** 🛑️ Asks the sentinel of this process to stop the registered tree of `pid` now. It starts no sentinel: a child that was never registered has none to stop. */
export function stopRegisteredV1(pid: number | undefined, host: ReaperHostV1 = {}): void {
  if (pid === undefined || !Number.isSafeInteger(pid) || pid < 1) return;
  const live = sentinel;
  if (live === undefined || live.parent !== (host.parent ?? process.pid) || live.child.exitCode !== null || live.child.signalCode !== null) return;
  try { live.child.stdin?.write(reaperLineTextV1({ kind: "stop", pid })); } catch {}
}

/** 🧹️ Registers one owned child with the sentinel of this process, which stops its process tree when this process ends in any way, SIGKILL included. Returns the function that unregisters it (call it when the child ended); a sentinel that cannot be started leaves the child as it was and never throws. */
export function reapWithParentV1(pid: number | undefined, host: ReaperHostV1 = {}): () => void {
  if (pid === undefined || !Number.isSafeInteger(pid) || pid < 1) return () => {};
  const send = (line: ReaperLineV1): void => { try { sentinelFor(host)?.child.stdin?.write(reaperLineTextV1(line)); } catch {} };
  send({ kind: "add", pid, spawnedMs: Date.now() });
  let open = true;
  return () => { if (open) { open = false; send({ kind: "remove", pid }); } };
}
