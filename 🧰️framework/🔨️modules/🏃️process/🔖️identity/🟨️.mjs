import {spawnSync} from "node:child_process";
import {createRequire} from "node:module";

/** 📐️ The tolerances of a process identity; the schema file is the single source.
 * @see ./🧬️schema/🔣️.json */
const schema = createRequire(import.meta.url)("./🧬️schema/🔣️.json");
export const PROCESS_IDENTITY_POLICY_V1 = Object.freeze({...schema.definitions.Policy.const});

const POSIX_COLUMNS = "lstart=";
const MONTHS = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

/** 🧮️ The creation time in ms of one `ps -o lstart=` text, `null` when it is not that shape. */
export function parsePosixProcessStartV1(text) {
  const found = /^\s*[A-Za-z]{3}\s+([A-Za-z]{3})\s+(\d{1,2})\s+(\d{2}):(\d{2}):(\d{2})\s+(\d{4})\s*$/u.exec(text);
  if (found === null) return null;
  const month = MONTHS.indexOf(found[1]);
  return month < 0 ? null : Date.UTC(Number(found[6]), month, Number(found[2]), Number(found[3]), Number(found[4]), Number(found[5]));
}

/** 🕰️ Reads the creation time of one process from the operating system (plain-JavaScript twin of `readProcessStartV1`). */
export function readProcessStartV1(pid, platform = process.platform) {
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
export function processIdentityOfV1(pid, read = readProcessStartV1, platform = process.platform) {
  const start = read(pid, platform);
  return typeof start === "number" ? {pid, startMs: start} : undefined;
}

/** 🧾️ A persisted holder record is not valid. */
export class ProcessIdentityRecordErrorV1 extends Error {
  constructor(where) {
    super(`${where}: a holder record needs pid and startMs (integers >= 1); a pid alone does not name a process / ein Halter-Eintrag braucht pid und startMs (ganze Zahlen >= 1), eine pid allein benennt keinen Prozess`);
    this.name = "ProcessIdentityRecordErrorV1";
  }
}

/** 🧾️ The identity a persisted holder record names (no `startMs` reads as 0: pid-only). */
export function requireProcessIdentityV1(value, where) {
  if (value === null || typeof value !== "object" || !Number.isSafeInteger(value.pid) || value.pid < 1 || (value.startMs !== undefined && (!Number.isSafeInteger(value.startMs) || value.startMs < 0))) throw new ProcessIdentityRecordErrorV1(where);
  return {pid: value.pid, startMs: value.startMs === undefined ? 0 : value.startMs};
}

/** 🧾️ The identity a transient record names, `undefined` when it names none. */
export function parseProcessIdentityV1(value) {
  try {
    return requireProcessIdentityV1(value, "parseProcessIdentityV1");
  } catch {
    return undefined;
  }
}

/** ⚖️ The pure verdict of a recorded identity against one read of its pid. */
export function processIdentityVerdictV1(recorded, seen, toleranceMs = PROCESS_IDENTITY_POLICY_V1.toleranceMilliseconds) {
  const identity = requireProcessIdentityV1(recorded, "processIdentityVerdictV1");
  if (seen === "gone") return "gone";
  if (seen === "unknown") return "unknown";
  if (identity.startMs === 0) return "same";
  return Math.abs(seen - identity.startMs) <= toleranceMs ? "same" : "reused";
}

/** 🔍️ Whether the recorded holder is gone or replaced; an unreadable table never frees. */
export function processIdentityDeadV1(recorded, read = readProcessStartV1, platform = process.platform, toleranceMs) {
  const verdict = processIdentityVerdictV1(recorded, read(recorded.pid, platform), toleranceMs);
  return verdict === "gone" || verdict === "reused";
}

/** 🫀️ Whether a pid exists in the process table; for a process this caller spawned itself or a persisted pid without creation time.
 * @see ./🟦️.ts */
export function processPidExistsV1(pid) {
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    return error?.code === "EPERM";
  }
}

/** 🫀️ Whether the holder a persisted record names by `pid` and an optional `startMs` still lives (twin of `processRecordAliveV1`). */
export function processRecordAliveV1(pid, startMs) {
  if (!Number.isSafeInteger(pid) || pid < 1) return false;
  return processHolderAliveV1({pid, startMs: Number.isSafeInteger(startMs) && startMs > 0 ? startMs : 0});
}

const aliveMemo = new Map();
const ALIVE_MEMO_MILLISECONDS = 2000;

/** 🫀️ Whether the holder named by a persisted record still lives (twin of `processHolderAliveV1`). */
export function processHolderAliveV1(holder, read = readProcessStartV1, platform = process.platform) {
  const identity = requireProcessIdentityV1(holder, "processHolderAliveV1");
  if (read === readProcessStartV1) {
    if (!processPidExistsV1(identity.pid)) return false;
    if (identity.startMs === 0) return true;
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

/** 🪪️ The identity of the process `pid` now; throws when it is gone or unreadable. */
export function processIdentityForPidV1(pid) {
  if (pid === process.pid) return ownProcessIdentityV1();
  const found = processIdentityOfV1(pid);
  if (found === undefined) throw new ProcessIdentityRecordErrorV1(`process ${pid} is gone or unreadable`);
  return found;
}

let own;

/** 🪪️ The identity of this process for the records it writes. */
export function ownProcessIdentityV1() {
  if (own !== undefined && own.pid === process.pid) return own;
  const read = readProcessStartV1(process.pid);
  own = {pid: process.pid, startMs: typeof read === "number" ? read : Math.round(Date.now() - process.uptime() * 1000)};
  return own;
}
