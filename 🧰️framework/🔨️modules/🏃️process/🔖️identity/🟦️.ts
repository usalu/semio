import {spawnSync} from "node:child_process";
import schema from "./🧬️schema/🔣️.json" with {type: "json"};

/** 📐️ The tolerances of a process identity.
 * @see ./🧬️schema/🔣️.json */
export const PROCESS_IDENTITY_POLICY_V1 = schema.definitions.Policy.const;

/** 🔖️ A process named by pid and creation time in milliseconds since the epoch (0: the creation time is not known and only the pid counts). */
export type ProcessIdentityV1 = Readonly<{pid: number; startMs: number}>;

/** ⚖️ The verdict on a recorded identity: the same process, gone, another process with the reused pid, or not readable now. */
export type ProcessIdentityVerdictV1 = "same" | "gone" | "reused" | "unknown";

/** 👁️ What one read of a pid found: its creation time, that it does not exist, or that the table could not be read. */
export type ProcessStartReadV1 = number | "gone" | "unknown";

/** 🔌️ Replaces the operating system read in a test. */
export type ProcessStartReaderV1 = (pid: number, platform: NodeJS.Platform) => ProcessStartReadV1;

const POSIX_COLUMNS = "lstart=";

/** 🧮️ The creation time in ms of one `ps -o lstart=` text (`Wed Nov 25 17:25:10 2026`, read in UTC), `null` when it is not that shape. */
export function parsePosixProcessStartV1(text: string): number | null {
  const found = /^\s*[A-Za-z]{3}\s+([A-Za-z]{3})\s+(\d{1,2})\s+(\d{2}):(\d{2}):(\d{2})\s+(\d{4})\s*$/u.exec(text);
  if (found === null) return null;
  const month = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"].indexOf(found[1]!);
  return month < 0 ? null : Date.UTC(Number(found[6]), month, Number(found[2]), Number(found[3]), Number(found[4]), Number(found[5]));
}

/** 🕰️ Reads the creation time of one process from the operating system: Windows asks `Win32_Process.CreationDate` (the creation FILETIME), macOS and Linux ask `ps -o lstart=` with the C locale in UTC.
 * @see https://learn.microsoft.com/windows/win32/cimwin32prov/win32-process */
export function readProcessStartV1(pid: number, platform: NodeJS.Platform = process.platform): ProcessStartReadV1 {
  const id = Math.trunc(pid);
  if (!Number.isInteger(id) || id < 1) return "gone";
  const timeout = PROCESS_IDENTITY_POLICY_V1.readTimeoutMilliseconds;
  try {
    if (platform === "win32") {
      const run = spawnSync("powershell.exe", ["-NoProfile", "-NonInteractive", "-Command", `$p = Get-CimInstance Win32_Process -Filter 'ProcessId=${id}'; if ($null -eq $p -or $null -eq $p.CreationDate) { 'gone' } else { ([DateTimeOffset]$p.CreationDate).ToUnixTimeMilliseconds() }`], {encoding: "utf8", windowsHide: true, timeout});
      if (run.status !== 0) return "unknown";
      const text = String(run.stdout).trim();
      if (text === "gone") return "gone";
      const value = Number(text);
      return Number.isFinite(value) && value > 0 ? value : "unknown";
    }
    const run = spawnSync("ps", ["-p", String(id), "-o", POSIX_COLUMNS], {encoding: "utf8", timeout, env: {...process.env, LC_ALL: "C", TZ: "UTC0"}});
    if (run.error !== undefined || run.status === null) return "unknown";
    const row = String(run.stdout).split("\n").map(parsePosixProcessStartV1).find(value => value !== null);
    if (row !== undefined && row !== null) return row;
    return run.status === 1 && String(run.stderr).trim() === "" ? "gone" : "unknown";
  } catch {
    return "unknown";
  }
}

/** 🪪️ The identity of a live process now, `undefined` when it is gone or cannot be read. */
export function processIdentityOfV1(pid: number, read: ProcessStartReaderV1 = readProcessStartV1, platform: NodeJS.Platform = process.platform): ProcessIdentityV1 | undefined {
  const start = read(pid, platform);
  return typeof start === "number" ? {pid, startMs: start} : undefined;
}

/** 🧾️ A persisted record of a holder is not valid: the process has no creation time (a pid alone never names a process). */
export class ProcessIdentityRecordErrorV1 extends Error {
  constructor(where: string) {
    super(`${where}: a holder record needs pid and startMs (integers >= 1); a pid alone does not name a process / ein Halter-Eintrag braucht pid und startMs (ganze Zahlen >= 1), eine pid allein benennt keinen Prozess`);
    this.name = "ProcessIdentityRecordErrorV1";
  }
}

/** 🧾️ The identity a persisted holder record names: a hard error when `pid` is not a positive integer or `startMs` is present and not a non-negative integer (0: start unknown). A record that has no `startMs` at all was written before the creation time was recorded (live machines hold such records once): it reads as `startMs` 0 (start unknown, only the pid counts) and the holder writes `startMs` with its next write; no writer ever produces it.
 * @see ./🧬️schema/🔣️.json */
export function requireProcessIdentityV1(value: unknown, where: string): ProcessIdentityV1 {
  const row = value as {pid?: unknown; startMs?: unknown} | null | undefined;
  if (row === null || typeof row !== "object" || !Number.isSafeInteger(row.pid) || (row.pid as number) < 1 || (row.startMs !== undefined && (!Number.isSafeInteger(row.startMs) || (row.startMs as number) < 0))) throw new ProcessIdentityRecordErrorV1(where);
  return {pid: row.pid as number, startMs: row.startMs === undefined ? 0 : row.startMs as number};
}

/** 🧾️ The identity a transient record names, `undefined` when it has no creation time: for claim files that age out when they are unreadable (a durable record uses `requireProcessIdentityV1`). */
export function parseProcessIdentityV1(value: unknown): ProcessIdentityV1 | undefined {
  try {
    return requireProcessIdentityV1(value, "parseProcessIdentityV1");
  } catch {
    return undefined;
  }
}

/** ⚖️ The pure verdict of a recorded identity against what one read of its pid found: a pid that exists with a creation time further than the tolerance from the recorded one is another process (a reused pid) and its holder is dead. A record without a creation time (`startMs` 0, written before it was recorded) is judged by its pid alone.
 * @see ./🧪️tests/🟦️.ts */
export function processIdentityVerdictV1(recorded: Readonly<{pid: number; startMs: number}>, seen: ProcessStartReadV1, toleranceMs: number = PROCESS_IDENTITY_POLICY_V1.toleranceMilliseconds): ProcessIdentityVerdictV1 {
  const identity = requireProcessIdentityV1(recorded, "processIdentityVerdictV1");
  if (seen === "gone") return "gone";
  if (seen === "unknown") return "unknown";
  if (identity.startMs === 0) return "same";
  return Math.abs(seen - identity.startMs) <= toleranceMs ? "same" : "reused";
}

/** 🔍️ Whether the holder named by `recorded` is dead for the purpose of freeing its resource: gone or another process now; an unreadable table never frees. */
export function processIdentityDeadV1(recorded: Readonly<{pid: number; startMs: number}>, read: ProcessStartReaderV1 = readProcessStartV1, platform: NodeJS.Platform = process.platform, toleranceMs?: number): boolean {
  const verdict = processIdentityVerdictV1(recorded, read(recorded.pid, platform), toleranceMs);
  return verdict === "gone" || verdict === "reused";
}

/** 📌️ The identity a process spawned by `spawnedMs` has, pinned from its first observation: the table row of `pid` whose creation time lies between the tolerance before the spawn and `spawnGraceMilliseconds` after it (the clock is read before the operating system creates the process, and a loaded machine creates it late), `undefined` when the row is outside that window and so cannot be the process that was spawned. */
export function pinSpawnedProcessV1(spawned: Readonly<{pid: number; spawnedMs: number}>, rowStartMs: number | undefined, policy: Readonly<{toleranceMilliseconds: number; spawnGraceMilliseconds: number}> = PROCESS_IDENTITY_POLICY_V1): ProcessIdentityV1 | undefined {
  if (rowStartMs === undefined || !Number.isFinite(rowStartMs)) return undefined;
  return rowStartMs >= spawned.spawnedMs - policy.toleranceMilliseconds && rowStartMs <= spawned.spawnedMs + policy.spawnGraceMilliseconds ? {pid: spawned.pid, startMs: Math.round(rowStartMs)} : undefined;
}

/** 🫀️ Whether a pid exists in the process table, zombies included; a permission refusal still means it exists. The single raw `kill(pid, 0)` of the repository: a pid alone is only enough for a process this caller spawned itself in this run, every persisted holder is judged by `processHolderAliveV1`.
 * @see ./🧪️tests/🔒️law/🟦️.ts */
export function processPidExistsV1(pid: number): boolean {
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    return (error as NodeJS.ErrnoException).code === "EPERM";
  }
}

/** 🫀️ Whether the holder named by a persisted record still lives: its pid exists and the process now at the pid was created within the tolerance of the recorded `startMs`; a pid reused by another process is dead, an unreadable table counts as alive (never frees a resource). Throws `ProcessIdentityRecordErrorV1` for a record without a creation time. */
export function processHolderAliveV1(holder: Readonly<{pid: number; startMs: number}>, read: ProcessStartReaderV1 = readProcessStartV1, platform: NodeJS.Platform = process.platform): boolean {
  const identity = requireProcessIdentityV1(holder, "processHolderAliveV1");
  if (read === readProcessStartV1) {
    if (!processPidExistsV1(identity.pid)) return false;
    const key = `${identity.pid}:${identity.startMs}`, now = Date.now(), hit = aliveMemo.get(key);
    if (hit !== undefined && now - hit < ALIVE_MEMO_MILLISECONDS) return true;
    const alive = !processIdentityDeadV1(identity, read, platform);
    if (alive) {
      if (aliveMemo.size > 512) aliveMemo.clear();
      aliveMemo.set(key, now);
    } else aliveMemo.delete(key);
    return alive;
  }
  return !processIdentityDeadV1(identity, read, platform);
}

/** 🫀️ Whether the holder a persisted record names by `pid` and an optional `startMs` still lives: a record written before the creation time was recorded (no `startMs`) is judged by its pid alone, a pid that is not a positive integer is dead, never an error. The form for readers of records that may predate `startMs` (live daemons keep writing the old shape until their next start). */
export function processRecordAliveV1(pid: unknown, startMs?: unknown): boolean {
  if (!Number.isSafeInteger(pid) || (pid as number) < 1) return false;
  return processHolderAliveV1({pid: pid as number, startMs: Number.isSafeInteger(startMs) && (startMs as number) > 0 ? (startMs as number) : 0});
}

const aliveMemo = new Map<string, number>();
const ALIVE_MEMO_MILLISECONDS = 2000;

/** 🪪️ The identity of the process `pid` now, for a holder record written on behalf of that process (this one or a spawned child): this process answers from its cached identity, any other is read from the process table; throws when it is gone or unreadable. */
export function processIdentityForPidV1(pid: number): ProcessIdentityV1 {
  if (pid === process.pid) return ownProcessIdentityV1();
  const found = processIdentityOfV1(pid);
  if (found === undefined) throw new ProcessIdentityRecordErrorV1(`process ${pid} is gone or unreadable`);
  return found;
}

let own: ProcessIdentityV1 | undefined;

/** 🪪️ The identity of this process for the records it writes: the creation time the operating system reports, else the instant this runtime started (`process.uptime()`), read once. */
export function ownProcessIdentityV1(): ProcessIdentityV1 {
  if (own !== undefined && own.pid === process.pid) return own;
  const read = readProcessStartV1(process.pid);
  const startMs = typeof read === "number" ? read : Math.round(Date.now() - process.uptime() * 1000);
  own = {pid: process.pid, startMs};
  return own;
}
