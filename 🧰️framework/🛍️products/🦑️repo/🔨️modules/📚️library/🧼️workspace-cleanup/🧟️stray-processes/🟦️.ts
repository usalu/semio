import { spawnSync } from "node:child_process";
import { basename } from "node:path";

/** 🧟Dev-tool executables whose leftovers accumulate across agent/terminal sessions and are safe to reap. */
const DEV_TOOL_EXECUTABLE_NAMES = new Set([
  "bun", "zsh", "cargo", "node", "esbuild", "rustc", "vite", "deno", "go", "rust-analyzer", "gopls", "tsserver",
  "jest", "vitest", "playwright", "nx", "turbo", "webpack", "rollup", "tsc", "watchman", "cursor-agent", "wasm-pack",
  "mdbook", "gradle", "mvn", "java", "javac", "swift", "ruby", "perl", "make", "ninja", "cmake", "docker", "colima",
]);

/** 🔎Command-line markers for dev leftovers whose executable basename alone is ambiguous (`bash`, `python`, …). */
const DEV_COMMAND_MARKERS = [
  /node_modules\/\.bin\//,
  /\/\.cargo\/(?:registry|git)\//,
  /\.rustup\//,
  /📜️script\.ts/,
  /\/semio\//,
  /cursor-agent\b.*\bworker start\b/,
  /@esbuild\//,
  /\bvite\b.*--port\b/,
  /\bnx run\b/,
  /\bbun run\b/,
  /\bcargo (?:run|test|build|check|clippy)\b/,
  /\b(?:jest|vitest|playwright)\b/,
];

const IDE_HOST_MARKERS = /(?:Cursor Helper|Code Helper|Electron|Visual Studio Code)/i;

export interface ProcessRow {
  pid: number;
  ppid: number;
  stat: string;
  name: string;
  command: string;
}

export interface StrayProcessRemoval {
  pid: number;
  ppid: number;
  name: string;
  action: "reaped-zombie" | "killed-stray";
}

/** 🏷️Derives the executable basename from a `ps` command column (macOS often reports zombies as `<defunct>`). */
export function strayProcessExecutableName(command: string): string {
  const trimmed = command.trim();
  if (trimmed === "" || trimmed === "<defunct>") return trimmed;
  for (const name of DEV_TOOL_EXECUTABLE_NAMES) {
    if (new RegExp(`(?:/|^)${name}(?:\\s|$)`).test(trimmed)) return name;
  }
  const token = trimmed.split(/\s+/)[0] ?? "";
  return basename(token.replace(/^-/, ""));
}

/** 🧪Whether a live process row looks like an abandoned dev-session leftover. */
export function isDevLeftoverRow(row: Pick<ProcessRow, "name" | "command" | "stat">): boolean {
  if (row.stat.startsWith("Z")) return false;
  if (row.name !== "<defunct>" && DEV_TOOL_EXECUTABLE_NAMES.has(row.name)) return true;
  return DEV_COMMAND_MARKERS.some((marker) => marker.test(row.command));
}

function processRows(): ProcessRow[] {
  const snapshot = spawnSync("ps", ["-axo", "pid=,ppid=,stat=,command="], { encoding: "utf8", windowsHide: true });
  if (snapshot.status !== 0 || typeof snapshot.stdout !== "string") return [];
  return snapshot.stdout.split("\n").flatMap((line): ProcessRow[] => {
    const match = line.trim().match(/^(\d+)\s+(\d+)\s+(\S+)\s+(.+)$/);
    if (!match) return [];
    const [, pid, ppid, stat, command] = match;
    const name = strayProcessExecutableName(command!);
    return [{ pid: Number(pid), ppid: Number(ppid), stat: stat!, name, command: command! }];
  });
}

/** 🌳Walks `ppid` links from the current process to protect this script's own ancestry (shell, editor, agent) from being reaped. */
function ancestryPids(rows: readonly ProcessRow[], selfPid: number): Set<number> {
  const byPid = new Map(rows.map((row) => [row.pid, row]));
  const ancestry = new Set<number>();
  for (let pid: number | undefined = selfPid; pid !== undefined && pid > 1 && !ancestry.has(pid); pid = byPid.get(pid)?.ppid) ancestry.add(pid);
  return ancestry;
}

/** 🖥️Skips IDE-hosted trees (Cursor/VS Code/Electron) while still reaping abandoned agent dev servers. */
function ideHostedPids(rows: readonly ProcessRow[]): Set<number> {
  const byPid = new Map(rows.map((row) => [row.pid, row]));
  const hosted = new Set<number>();
  for (const row of rows) {
    for (let pid: number | undefined = row.pid; pid !== undefined && pid > 1; pid = byPid.get(pid)?.ppid) {
      const ancestor = byPid.get(pid);
      if (!ancestor) break;
      if (IDE_HOST_MARKERS.test(ancestor.command)) {
        hosted.add(row.pid);
        break;
      }
    }
  }
  return hosted;
}

function rowDevLeftoverLabel(row: ProcessRow, byPid: ReadonlyMap<number, ProcessRow>): string | null {
  if (isDevLeftoverRow(row)) return row.name !== "<defunct>" ? row.name : "dev-leftover";
  if (!row.stat.startsWith("Z")) return null;
  const parent = byPid.get(row.ppid);
  if (!parent) return null;
  if (isDevLeftoverRow(parent)) return strayProcessExecutableName(parent.command) || parent.name;
  return null;
}

/**
 * 🧟️Plans reaping of dev-tool leftovers from crashed or abandoned terminal/agent sessions. Zombies whose `comm` is
 * `<defunct>` match via their parent's executable name or command markers.
 */
export function planStrayProcessRemovals(rows: readonly ProcessRow[], selfPid: number): StrayProcessRemoval[] {
  const byPid = new Map(rows.map((row) => [row.pid, row]));
  const protectedPids = new Set([...ancestryPids(rows, selfPid), ...ideHostedPids(rows)]);
  const removals: StrayProcessRemoval[] = [];
  const killed = new Set<number>();
  const liveLeftovers = rows
    .filter((row) => isDevLeftoverRow(row) && !protectedPids.has(row.pid))
    .sort((a, b) => b.pid - a.pid);
  for (const row of rows) {
    if (protectedPids.has(row.pid)) continue;
    const label = rowDevLeftoverLabel(row, byPid);
    if (!label) continue;
    if (row.stat.startsWith("Z")) {
      const parent = byPid.get(row.ppid);
      if (row.ppid <= 1 || protectedPids.has(row.ppid) || killed.has(row.ppid) || !parent) continue;
      killed.add(row.ppid);
      removals.push({ pid: row.pid, ppid: row.ppid, name: label, action: "reaped-zombie" });
    }
  }
  for (const row of liveLeftovers) {
    if (killed.has(row.pid)) continue;
    killed.add(row.pid);
    removals.push({ pid: row.pid, ppid: row.ppid, name: row.name, action: "killed-stray" });
  }
  return removals;
}

function applyStrayProcessRemovals(rows: readonly ProcessRow[], selfPid: number, dry: boolean): StrayProcessRemoval[] {
  const plan = planStrayProcessRemovals(rows, selfPid);
  if (!dry) {
    for (const row of plan) {
      const target = row.action === "reaped-zombie" ? row.ppid : row.pid;
      try { process.kill(target, "SIGKILL"); } catch {}
    }
  }
  return plan;
}

/**
 * 🧟️Reaps dev leftovers from crashed or abandoned sessions: zombie (`Z` state) rows are reaped by killing their
 * still-live parent, and every other matching live row is killed directly. Never touches this script's own ancestry
 * or IDE-hosted trees.
 */
export function cleanKillStrayProcesses(dry: boolean): StrayProcessRemoval[] {
  if (process.platform === "win32") return [];
  const removals: StrayProcessRemoval[] = [];
  for (let pass = 0; pass < 32; pass++) {
    const batch = applyStrayProcessRemovals(processRows(), process.pid, dry);
    removals.push(...batch);
    if (batch.length === 0 || dry) break;
  }
  return removals;
}

/** 🔎Lists live dev leftovers that would be killed (for audits after `clean`). */
export function listDevLeftoverRows(rows: readonly ProcessRow[], selfPid: number): ProcessRow[] {
  const byPid = new Map(rows.map((row) => [row.pid, row]));
  const protectedPids = new Set([...ancestryPids(rows, selfPid), ...ideHostedPids(rows)]);
  return rows.filter((row) => isDevLeftoverRow(row) && !protectedPids.has(row.pid));
}

/** 🔎Counts zombie rows whose parent is a dev leftover — used to verify clean reaping. */
export function countStrayDevToolZombies(rows: readonly ProcessRow[]): number {
  const byPid = new Map(rows.map((row) => [row.pid, row]));
  return rows.filter((row) => row.stat.startsWith("Z") && rowDevLeftoverLabel(row, byPid) !== null).length;
}
