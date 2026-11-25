import { spawnSync } from "node:child_process";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 📐️ The declared grace of a termination.
 * @see ./🧬️schema/🔣️.json */
export const TERMINATION_POLICY_V1 = schema.definitions.Policy.const;

/** 🌳️ Captures the first-party ownership state needed to terminate a live child. */
export interface OwnedChildHandle { readonly pid?: number; readonly exitCode: number | null; readonly signalCode: string | null }

interface OwnedProcessRow {
  pid: number;
  parent: number;
  group: number;
  depth: number;
}

/** 🔌️ The host facts of a termination: the platform and the synchronous process start, replaced by a test to run the Windows branch against a `taskkill` fixture on any host. */
export interface TerminationHostV1 { readonly platform?: string; readonly spawnSync?: typeof spawnSync; readonly record?: (row: Readonly<{ pid: number }>, signal: "SIGINT" | "SIGTERM" | "SIGKILL") => void }

/** 🪓️Terminates one owned subprocess tree, including descendant-created POSIX groups, before ancestry is lost. On Windows `taskkill /T /F` ends the tree (exit 128 = the process is already gone); any other failure ends at least the root. See https://learn.microsoft.com/windows-server/administration/windows-commands/taskkill */
export function terminateOwnedProcessTree(rootPid: number, host: TerminationHostV1 = {}): void {
  if (!Number.isSafeInteger(rootPid) || rootPid <= 0) return;
  if ((host.platform ?? process.platform) === "win32") {
    let status: number | null = null;
    try { status = (host.spawnSync ?? spawnSync)("taskkill", ["/pid", String(rootPid), "/T", "/F"], { stdio: "ignore", windowsHide: true }).status; } catch {}
    if (status !== 0 && status !== 128) try { process.kill(rootPid, "SIGKILL"); } catch {}
    return;
  }
  const owned = ownedRowsV1(rootPid);
  if (owned === undefined) {
    try { process.kill(-rootPid, "SIGKILL"); } catch {}
    try { host.record?.({ pid: rootPid }, "SIGKILL"); } catch {}
    try { process.kill(rootPid, "SIGKILL"); } catch {}
    return;
  }
  signalOwnedRowsV1(owned, "SIGKILL", host.record);
}

/** 🌳️ The processes of the owned tree of `rootPid` now, deepest first is up to the caller: pid, parent, group and depth; `undefined` when the process table cannot be read. */
function ownedRowsV1(rootPid: number): OwnedProcessRow[] | undefined {
  const snapshot = spawnSync("ps", ["-axo", "pid=,ppid=,pgid="], { encoding: "utf8", windowsHide: true });
  if (snapshot.status !== 0 || typeof snapshot.stdout !== "string") return undefined;
  const rows = snapshot.stdout.split("\n").flatMap((line): Omit<OwnedProcessRow, "depth">[] => {
    const fields = line.trim().split(/\s+/).map(Number);
    return fields.length === 3 && fields.every((field) => Number.isSafeInteger(field) && field > 0)
      ? [{ pid: fields[0]!, parent: fields[1]!, group: fields[2]! }]
      : [];
  });
  const depth = new Map<number, number>([[rootPid, 0]]);
  for (let changed = true; changed;) {
    changed = false;
    for (const row of rows) {
      if (depth.has(row.pid)) continue;
      const parentDepth = depth.get(row.parent);
      if (parentDepth === undefined) continue;
      depth.set(row.pid, parentDepth + 1);
      changed = true;
    }
  }
  const owned: OwnedProcessRow[] = rows.filter((row) => depth.has(row.pid)).map((row) => ({ ...row, depth: depth.get(row.pid)! }));
  if (!owned.some((row) => row.pid === rootPid)) owned.push({ pid: rootPid, parent: 0, group: rootPid, depth: 0 });
  return owned;
}

/** 📨️ Sends `signal` to the owned rows: the process groups the tree created first (deepest first), then each process, announcing each process through `record` before. */
function signalOwnedRowsV1(owned: OwnedProcessRow[], signal: "SIGINT" | "SIGTERM" | "SIGKILL", record?: TerminationHostV1["record"]): void {
  for (const row of owned) try { record?.(row, signal); } catch {}
  const ownedPids = new Set(owned.map((row) => row.pid));
  const groups = new Map<number, number>();
  for (const row of owned) {
    if (ownedPids.has(row.group)) groups.set(row.group, Math.max(groups.get(row.group) ?? 0, row.depth));
  }
  for (const [group] of [...groups].sort((left, right) => right[1] - left[1])) {
    try { process.kill(-group, signal); } catch {}
  }
  for (const row of owned.sort((left, right) => right.depth - left.depth)) {
    try { process.kill(row.pid, signal); } catch {}
  }
}

/** ⏹️ Terminates one still-running spawned child with its whole descendant tree — POSIX groups and `taskkill /T`
 * on Windows through [[terminateOwnedProcessTree]] — and never signals a pid its exited child no longer owns. */
export function terminateOwnedChildTree(child: OwnedChildHandle, host: TerminationHostV1 = {}): void {
  if (child.exitCode === null && child.signalCode === null && child.pid) terminateOwnedProcessTree(child.pid, host);
}


/** 🗣️ The record of one signal in English and German, `<English> / <German>`: the victim, the signal, the killing process, the command and the reason. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function ownedKillRecordLineV1(input: Readonly<{ pid: number; signal: "SIGINT" | "SIGTERM" | "SIGKILL"; killer: number; label: string; reason: string }>): string {
  const fill = (language: "en" | "de"): string => schema.definitions.Texts.const[language]["kill-record"].replace(/\{(\w+)\}/gu, (_all, name: string) => String((input as Record<string, string | number>)[name] ?? ""));
  return `${fill("en")} / ${fill("de")}`;
}

/** 🕊️ The signal and grace options, the wait, whether a pid lives, and the platform ports used by graceful termination. */
export interface GracefulTerminationV1 extends TerminationHostV1 { readonly initialSignal?: "SIGINT" | "SIGTERM"; readonly graceMilliseconds?: number; readonly pollMilliseconds?: number; readonly sleep?: (milliseconds: number) => Promise<void>; readonly alive?: (pid: number) => boolean }

/** 🤝️ Sends the initial signal (SIGTERM by default) to the whole owned tree before ancestry is lost. After the grace period it sends SIGKILL to remaining members, including descendants started during the wait. Windows and an unreadable process table fall back to [[terminateOwnedProcessTree]]. Resolves when the tree is gone or the kill was sent.
 * @see ./🧬️schema/🔣️.json */
export async function terminateOwnedProcessTreeGracefullyV1(rootPid: number, host: GracefulTerminationV1 = {}): Promise<void> {
  if (!Number.isSafeInteger(rootPid) || rootPid <= 0) return;
  const owned = (host.platform ?? process.platform) === "win32" ? undefined : ownedRowsV1(rootPid);
  if (owned === undefined) return terminateOwnedProcessTree(rootPid, host);
  const grace = host.graceMilliseconds ?? TERMINATION_POLICY_V1.graceMilliseconds, poll = host.pollMilliseconds ?? TERMINATION_POLICY_V1.pollMilliseconds;
  const sleep = host.sleep ?? ((milliseconds: number) => new Promise<void>((done) => setTimeout(done, milliseconds)));
  const lives = host.alive ?? ((pid: number): boolean => { try { process.kill(pid, 0); return true; } catch (error) { return (error as NodeJS.ErrnoException).code === "EPERM"; } });
  signalOwnedRowsV1(owned, host.initialSignal ?? "SIGTERM", host.record);
  for (let waited = 0; waited < grace && owned.some((row) => lives(row.pid)); waited += poll) await sleep(poll);
  const left = owned.filter((row) => lives(row.pid)), fresh = lives(rootPid) ? ownedRowsV1(rootPid) ?? [] : [];
  const rows = [...left, ...fresh.filter((row) => !left.some((known) => known.pid === row.pid))];
  if (rows.length > 0) signalOwnedRowsV1(rows, "SIGKILL", host.record);
}

/** ⏹️ [[terminateOwnedChildTree]] that asks first ([[terminateOwnedProcessTreeGracefullyV1]]). */
export function terminateOwnedChildTreeGracefullyV1(child: OwnedChildHandle, host: GracefulTerminationV1 = {}): Promise<void> {
  return child.exitCode === null && child.signalCode === null && child.pid ? terminateOwnedProcessTreeGracefullyV1(child.pid, host) : Promise.resolve();
}
