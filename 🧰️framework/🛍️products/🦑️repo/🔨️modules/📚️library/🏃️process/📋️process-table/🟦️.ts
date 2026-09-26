import { spawnSync } from "node:child_process";

/** 📋️ One row of the machine's process table; `stat` is the POSIX state column and empty on Windows, which has no zombies. */
export interface ProcessTableRow {
  readonly pid: number;
  readonly parent: number;
  readonly stat: string;
  readonly command: string;
}

/** 🧰️ How a snapshot reaches the platform tool; laws inject canned output to drive either branch on any host. */
export type ProcessTableRunner = (command: string, args: readonly string[]) => { readonly status: number | null; readonly stdout: string };

/** 🐧️ `ps` columns every POSIX `ps` (BSD, procps, busybox with `-o`) prints: pid, parent, state, full command line. */
export const POSIX_PROCESS_TABLE_ARGUMENTS = ["-axo", "pid=,ppid=,stat=,command="] as const;

/** 🪟️ `Get-CimInstance` projection as compressed JSON, written as UTF-8 so command lines with emoji paths survive the console code page. */
export const WINDOWS_PROCESS_TABLE_ARGUMENTS = [
  "-NoLogo",
  "-NoProfile",
  "-NonInteractive",
  "-Command",
  "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; Get-CimInstance Win32_Process | Select-Object ProcessId,ParentProcessId,Name,CommandLine | ConvertTo-Json -Compress",
] as const;

/** 🔢️ Parses `ps -axo pid=,ppid=,stat=,command=` output (any line ending). */
export function parsePosixProcessTable(stdout: string): ProcessTableRow[] {
  return stdout.split(/\r?\n/).flatMap((line): ProcessTableRow[] => {
    const match = line.trim().match(/^(\d+)\s+(\d+)\s+(\S+)\s+(.+)$/);
    return match ? [{ pid: Number(match[1]), parent: Number(match[2]), stat: match[3]!, command: match[4]! }] : [];
  });
}

/** 🔣️ Parses the `Get-CimInstance Win32_Process` JSON projection: one object or an array; a process without a readable `CommandLine` falls back to its image `Name`. */
export function parseWindowsProcessTable(stdout: string): ProcessTableRow[] {
  const text = stdout.replace(/^﻿/, "").trim();
  if (!text) return [];
  return [JSON.parse(text)].flat().flatMap((row: { ProcessId?: unknown; ParentProcessId?: unknown; Name?: unknown; CommandLine?: unknown } | null): ProcessTableRow[] => {
    if (!row || !Number.isSafeInteger(row.ProcessId) || !Number.isSafeInteger(row.ParentProcessId)) return [];
    const command = typeof row.CommandLine === "string" && row.CommandLine.trim() ? row.CommandLine.trim() : typeof row.Name === "string" ? row.Name : "";
    return [{ pid: row.ProcessId as number, parent: row.ParentProcessId as number, stat: "", command }];
  });
}

/** 🖥️ The real runner: no shell, hidden console window, UTF-8 capture. */
const nativeRunner: ProcessTableRunner = (command, args) => {
  const result = spawnSync(command, [...args], { encoding: "utf8", windowsHide: true, timeout: 15_000, maxBuffer: 64 * 1024 * 1024 });
  return { status: result.error ? null : result.status, stdout: result.stdout ?? "" };
};

/** 📸️ The process table on `platform`, or `undefined` when its tool is unavailable: callers must then refuse to act, never read it as "nothing runs". */
export function processTableSnapshot(platform: NodeJS.Platform = process.platform, run: ProcessTableRunner = nativeRunner): ProcessTableRow[] | undefined {
  const windows = platform === "win32";
  const result = windows ? run("powershell.exe", WINDOWS_PROCESS_TABLE_ARGUMENTS) : run("ps", POSIX_PROCESS_TABLE_ARGUMENTS);
  if (result.status !== 0) return undefined;
  try {
    return windows ? parseWindowsProcessTable(result.stdout) : parsePosixProcessTable(result.stdout);
  } catch {
    return undefined;
  }
}
