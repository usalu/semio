/** 🖥️ The proctor under test: the release build of `proctor serve` in production mode on a throw-away loopback port over a throw-away data directory, its memory and its roster count. */
import { execFile } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { createServer } from "node:net";
import { launchProctor } from "../../../🏗️bootstrap/🟦️.ts";
import { RESERVED_PORTS } from "../📊️settings-and-measurements/🟦️.ts";
import { pause } from "../🚚️transport/🟦️.ts";

//#region 🖥️Proctor
/** 🔌️ A free loopback port no dev server of this repository uses. */
export async function freePort(): Promise<number> {
  for (;;) {
    const port = await new Promise<number>((found, refused) => {
      const probe = createServer();
      probe.once("error", refused);
      probe.listen(0, "127.0.0.1", () => {
        const address = probe.address();
        probe.close(() => found(typeof address === "object" && address !== null ? address.port : 0));
      });
    });
    if (port !== 0 && !RESERVED_PORTS.has(port)) return port;
  }
}

/** 🖨️ What `command` prints, without holding this thread while it runs (a held thread would be counted as latency
 * of the hall); empty when it cannot run. */
function printed(command: string, args: readonly string[]): Promise<string> {
  return new Promise((done) => execFile(command, [...args], { encoding: "utf8", windowsHide: true }, (_error, stdout) => done(stdout ?? "")));
}

/** 🧠️ The resident memory of process `pid` in MiB, or `undefined` where this platform cannot say. */
export async function residentMebibytes(pid: number): Promise<number | undefined> {
  if (process.platform === "linux") {
    const status = existsSync(`/proc/${pid}/status`) ? readFileSync(`/proc/${pid}/status`, "utf8") : "";
    const kibibytes = /^VmRSS:\s+(\d+) kB$/mu.exec(status)?.[1];
    return kibibytes === undefined ? undefined : Number(kibibytes) / 1024;
  }
  if (process.platform === "win32") {
    const listed = await printed("tasklist", ["/FI", `PID eq ${pid}`, "/FO", "CSV", "/NH"]);
    const kibibytes = /"([\d.,'   ]+) K"\s*$/mu.exec(listed.trim())?.[1]?.replace(/\D/gu, "");
    return kibibytes === undefined || kibibytes === "" ? undefined : Number(kibibytes) / 1024;
  }
  const kibibytes = (await printed("ps", ["-o", "rss=", "-p", String(pid)])).trim();
  return kibibytes === "" ? undefined : Number(kibibytes) / 1024;
}

/** ⚙️ The processor time process `pid` has used so far in seconds, or `undefined` where this platform cannot say. */
export async function processorSeconds(pid: number): Promise<number | undefined> {
  if (process.platform === "win32") {
    const seconds = (await printed("powershell", ["-NoProfile", "-NonInteractive", "-Command", `(Get-Process -Id ${pid}).TotalProcessorTime.TotalMilliseconds`])).trim().replace(",", ".");
    return seconds === "" || Number.isNaN(Number(seconds)) ? undefined : Number(seconds) / 1000;
  }
  const time = (await printed("ps", ["-o", "time=", "-p", String(pid)])).trim();
  const parts = /^(?:(\d+)-)?(?:(\d+):)?(\d+):(\d+(?:\.\d+)?)$/u.exec(time);
  return parts === null ? undefined : Number(parts[1] ?? 0) * 86_400 + Number(parts[2] ?? 0) * 3_600 + Number(parts[3]) * 60 + Number(parts[4]);
}

/** 🪪️ The roster of a stopped proctor as an operator counts it: what `prune` would remove at once (every registration
 * nobody submitted a run under) and what would stay. */
export interface Roster {
  /** 🧹️ Registrations nobody played under: what a prune removes. */
  readonly idle: number;
  /** 🛟️ Learners that submitted a run, and the registrations they hold. */
  readonly played: number;
  readonly registrations: number;
}

/** 🪣️ Count the roster of the stopped proctor of `env` with its own `prune --older-than 0s --dry-run`, which changes
 * nothing; `undefined` when the count cannot be read. */
export async function rosterOf(executable: string, repoRoot: string, env: NodeJS.ProcessEnv, log: string): Promise<Roster | undefined> {
  await launchProctor(executable, repoRoot, ["prune", "--older-than", "0s", "--dry-run"], env, log).exited;
  const counted = existsSync(log) ? readFileSync(log, "utf8") : "";
  const idle = /^would prune (\d+) registrations /mu.exec(counted)?.[1];
  const staying = /^ {2}staying: (\d+) learners \((\d+) of them submitted a run\), \d+ handles, (\d+) registrations$/mu.exec(counted);
  if (idle === undefined || staying === null || Number(staying[1]) !== Number(staying[2])) return undefined;
  return { idle: Number(idle), played: Number(staying[2]), registrations: Number(staying[3]) };
}

export async function ready(base: string, timeoutMs: number): Promise<boolean> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    try {
      const answer = await fetch(`${base}/instance`, { headers: { "x-forwarded-proto": "https" }, signal: AbortSignal.timeout(2_000) });
      if (answer.ok) return true;
    } catch {}
    await pause(200);
  }
  return false;
}
//#endregion 🖥️Proctor
