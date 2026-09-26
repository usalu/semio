/** 🏷️ Build freshness of a RUNNING hub: is the process answering on the hub's port built from the source tree as it is now?
 *
 * The hub's staging verbs (`os-hub:build-dev`, `build-dev-postgres`, `build`) stage a `semio.cargo.binary-sources/v1`
 * record beside the executable (`os-hub.sources.json`: every source file Cargo compiled it from — its dep-info over the
 * whole dependency closure — and when that build started). This check resolves the listening process, its executable
 * (the operating system's own view: `lsof` on macOS, `/proc/<pid>/exe` on Linux, `Get-Process` on Windows) and the record
 * beside it, and applies Cargo's own freshness rule: fresh when every recorded source still exists and none was modified
 * after the build started, and the executable was not replaced after the process started. It names the first changed
 * source and counts all of them. A hub whose executable carries no record (a copy made without its record, or a build
 * made outside the staging verbs) is reported as unverifiable, never as fresh.
 *
 * Acceptance ledger 1.3 ("the hub a dev would open today carries the current tree, no stale binary").
 */
import { spawnSync } from "node:child_process";
import { existsSync, readFileSync, readlinkSync, statSync } from "node:fs";
import { dirname, join } from "node:path";
import { cargoBinarySourcesFreshnessV1, parseCargoBinarySourcesV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { HUB_BINARY_SOURCES_FILE, listeningProcessId } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";

/** 📊️ What the check found. */
export type BuildFreshnessReport = {
  hub: string;
  runId: string | null;
  pid: number | null;
  executable: string | null;
  processStartedAtMs: number | null;
  executableModifiedAtMs: number | null;
  record: string | null;
  builtAtMs: number | null;
  sources: number;
  changed: string[];
  verdict: "fresh" | "stale" | "unverifiable";
  reason: Readonly<{ en: string; de: string }>;
};

/** 🔎️ The executable path and start time of `pid`. */
export function processExecutable(pid: number): { executable: string; startedAtMs: number | null } {
  if (process.platform === "linux") {
    const executable = readlinkSync(`/proc/${pid}/exe`);
    const started = spawnSync("ps", ["-o", "lstart=", "-p", String(pid)], { encoding: "utf8" }).stdout.trim();
    return { executable, startedAtMs: started ? Date.parse(started) : null };
  }
  if (process.platform === "win32") {
    const answer = spawnSync("powershell", ["-NoProfile", "-Command", `$p = Get-Process -Id ${pid}; "$($p.Path)|$($p.StartTime.ToUniversalTime().ToString('o'))"`], { encoding: "utf8" });
    const [executable = "", started = ""] = answer.stdout.trim().split("|");
    return { executable, startedAtMs: started ? Date.parse(started) : null };
  }
  const escaped = (spawnSync("lsof", ["-a", "-p", String(pid), "-d", "txt", "-Fn"], { encoding: "utf8" }).stdout.split("\n").find((line) => line.startsWith("n")) ?? "").slice(1);
  const executable = Buffer.from(escaped.replace(/\\x([0-9a-f]{2})/giu, (_, hex: string) => String.fromCharCode(Number.parseInt(hex, 16))), "latin1").toString("utf8");
  const started = spawnSync("ps", ["-o", "lstart=", "-p", String(pid)], { encoding: "utf8" }).stdout.trim();
  return { executable, startedAtMs: started ? Date.parse(started) : null };
}

const modifiedAtMs = (path: string): number | null => {
  try {
    return statSync(path).mtimeMs;
  } catch {
    return null;
  }
};

/** 🏷️ Runs the check against the hub answering at `hub` (or the executable named by `executable`). */
export async function checkHubBuildFreshness(hub: string, executableOverride: string | null): Promise<BuildFreshnessReport> {
  const report: BuildFreshnessReport = { hub, runId: null, pid: null, executable: executableOverride, processStartedAtMs: null, executableModifiedAtMs: null, record: null, builtAtMs: null, sources: 0, changed: [], verdict: "unverifiable", reason: { en: "", de: "" } };
  const readiness = await fetch(`${hub}/readyz`, { signal: AbortSignal.timeout(10_000) }).then((response) => response.json() as Promise<{ runId?: string }>).catch(() => null);
  report.runId = readiness?.runId ?? null;
  if (executableOverride === null) {
    report.pid = listeningProcessId(Number(new URL(hub).port));
    const found = processExecutable(report.pid);
    report.executable = found.executable;
    report.processStartedAtMs = found.startedAtMs;
  }
  if (!report.executable || !existsSync(report.executable)) {
    report.reason = { en: `the hub's executable ${report.executable ?? "<unknown>"} is not on this machine`, de: `die ausführbare Datei des Hubs ${report.executable ?? "<unbekannt>"} liegt nicht auf dieser Maschine` };
    return report;
  }
  report.executableModifiedAtMs = modifiedAtMs(report.executable);
  report.record = join(dirname(report.executable), HUB_BINARY_SOURCES_FILE);
  if (!existsSync(report.record)) {
    report.reason = { en: `no ${HUB_BINARY_SOURCES_FILE} beside ${report.executable} (built or copied without its sources record)`, de: `kein ${HUB_BINARY_SOURCES_FILE} neben ${report.executable} (ohne Quellennachweis gebaut oder kopiert)` };
    return report;
  }
  const record = parseCargoBinarySourcesV1(readFileSync(report.record, "utf8"));
  if (record === null) {
    report.reason = { en: `${report.record} is not a semio.cargo.binary-sources/v1 record`, de: `${report.record} ist kein semio.cargo.binary-sources/v1-Nachweis` };
    return report;
  }
  report.builtAtMs = record.builtAtMs;
  report.sources = record.sources.length;
  report.changed = record.sources.filter((source) => {
    const modified = modifiedAtMs(source);
    return modified === null || modified > record.builtAtMs;
  });
  const verdict = cargoBinarySourcesFreshnessV1(record, modifiedAtMs);
  const replaced = report.processStartedAtMs !== null && report.executableModifiedAtMs !== null && report.executableModifiedAtMs > report.processStartedAtMs + 1_000;
  if (replaced) {
    report.verdict = "stale";
    const replacedAt = `${new Date(report.executableModifiedAtMs!).toISOString()} > ${new Date(report.processStartedAtMs!).toISOString()}`;
    report.reason = { en: `the executable was replaced after the process started (${replacedAt})`, de: `die ausführbare Datei wurde nach dem Prozessstart ersetzt (${replacedAt})` };
  } else if (verdict.fresh) {
    report.verdict = "fresh";
    report.reason = { en: `all ${report.sources} recorded sources unchanged since the build started ${new Date(record.builtAtMs).toISOString()}`, de: `alle ${report.sources} erfassten Quellen seit Buildbeginn ${new Date(record.builtAtMs).toISOString()} unverändert` };
  } else {
    report.verdict = "stale";
    report.reason = { en: `${report.changed.length}/${report.sources} recorded sources changed or vanished since the build started ${new Date(record.builtAtMs).toISOString()}; first: ${verdict.changed}`, de: `${report.changed.length}/${report.sources} erfasste Quellen seit Buildbeginn ${new Date(record.builtAtMs).toISOString()} geändert oder verschwunden; zuerst: ${verdict.changed}` };
  }
  return report;
}
