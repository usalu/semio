/** 🎼️ Extension utilities
 *
 * Logging, workspace and repo-binary resolution, CLI execution and repo-event parsing.
 */

// #region 🔌️Adapters
import { execFile } from "child_process";
import * as fs from "fs";
import * as path from "path";
import { promisify } from "util";
import * as vscode from "vscode";
import { type RepoEvent } from "../⚙️data-models/🟦️.ts";
import { UI_STRINGS } from "../🎞️constants/🟦️.ts";
import { cachedRepoBaseUrl, outputChannel } from "../🎩️globals/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🔌️Adapters
/**
 * execFileAsync holds the data fields for a execFileAsync record.
 **/
export const execFileAsync = promisify(execFile);
// #endregion 🔌️Adapters

// #region 🎼️Utilities
// Utilities provide logging, fixed read-only query execution and binary resolution.

/**
 * writeLog holds the data fields for a writeLog record.
 **/
function writeLog(level: string, args: any[]): void {
  const message = args.map((a) => (typeof a === "object" ? JSON.stringify(a, null, 2) : String(a))).join(" ");
  const prefix = level === "ERROR" ? "[ERROR] " : "";
  outputChannel.current?.appendLine(prefix + message);
  try {
    const logPath = path.join(getWorkspaceRoot() || "", "📋️activation.log");
    fs.appendFileSync(logPath, `[${level}] ${message}\n`);
  } catch (e) {}
}

/**
 * log holds the data fields for a log record.
 **/
export function log(...args: any[]): void {
  writeLog("LOG", args);
}

/**
 * logError holds the data fields for a logError record.
 **/
export function logError(...args: any[]): void {
  writeLog("ERROR", args);
}

/**
 * getWorkspaceRoot holds the data fields for a getWorkspaceRoot record.
 **/
export function getWorkspaceRoot(): string | undefined {
  return vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
}

/** 🗃️ The repo command line binary (`semio-repo`) the workspace built: `REPO_CLI_BIN` when set, else the debug deliverable below the shared Cargo `target-dir` of `.cargo/config.toml`. */
export function resolveRepoBinaryPath(root: string, env: NodeJS.ProcessEnv = process.env): string | undefined {
  const configured = env.REPO_CLI_BIN?.trim();
  const ext = process.platform === "win32" ? ".exe" : "";
  const candidate = configured ? path.resolve(root, configured) : path.join(root, ".🧬semio", "🦑️repo", "⚡️cache", "cargo", "target", "debug", `semio-repo${ext}`);
  return fs.existsSync(candidate) ? candidate : undefined;
}

/** getRepoBinaryPath holds the data fields for a getRepoBinaryPath record.
 **/
export function getRepoBinaryPath(): string | undefined {
  const root = getWorkspaceRoot();
  return root ? resolveRepoBinaryPath(root) : undefined;
}

/** getRepoCommand holds the data fields for a getRepoCommand record.
 **/
export function getRepoCommand(): string {
  const binaryPath = getRepoBinaryPath();
  return binaryPath ?? "";
}

/**
 * hasRepoAccess holds the data fields for a hasRepoAccess record.
 **/
export function hasRepoAccess(): boolean {
  return getRepoCommand() !== "";
}

/**
 * getUiString holds the data fields for a getUiString record.
 **/
function getUiString(key: keyof typeof UI_STRINGS.en): string {
  const language = vscode.env.language.split("-")[0];
  const bundle = UI_STRINGS[language as keyof typeof UI_STRINGS] ?? UI_STRINGS.en;
  return (bundle as any)[key];
}

/**
 * resolveCheckpointSha holds the data fields for a resolveCheckpointSha record.
 **/
function resolveCheckpointSha(checkpoint: string | { sha?: string } | undefined): string | undefined {
  if (!checkpoint) return undefined;
  if (typeof checkpoint === "string") return checkpoint;
  return checkpoint.sha;
}

/** getGitHubRepoBaseUrl holds the data fields for a getGitHubRepoBaseUrl record.
 **/
export function getGitHubRepoBaseUrl(): string | undefined {
  if (cachedRepoBaseUrl.current !== undefined) return cachedRepoBaseUrl.current;
  const root = getWorkspaceRoot();
  if (!root) return (cachedRepoBaseUrl.current = undefined);
  const packagePath = path.join(root, "package.json");
  if (!fs.existsSync(packagePath)) return (cachedRepoBaseUrl.current = undefined);
  const raw = fs.readFileSync(packagePath, "utf8");
  const parsed = JSON.parse(raw) as { repository?: { url?: string } | string };
  const repoUrl = typeof parsed.repository === "string" ? parsed.repository : parsed.repository?.url;
  if (!repoUrl) return (cachedRepoBaseUrl.current = undefined);
  let cleaned = repoUrl.replace(/^git\+/, "").replace(/\.git$/, "");
  if (cleaned.startsWith("git@")) {
    const match = cleaned.match(/^git@([^:]+):(.+)$/);
    if (match) cleaned = `https://${match[1]}/${match[2]}`;
  }
  cachedRepoBaseUrl.current = cleaned.startsWith("http://") || cleaned.startsWith("https://") ? cleaned : undefined;
  return cachedRepoBaseUrl.current;
}

/**
 * Parses raw CLI output into structured repo events.
 * Implementations MUST split output by newlines and parse each non-empty line as JSON.
 **/
export function parseRepoEvents(output: string): RepoEvent[] {
  const lines = output
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.length > 0);
  return lines.map((line) => JSON.parse(line) as RepoEvent);
}

/**
 * Extracts the final result payload from a sequence of repo events.
 * Implementations MUST throw on fatal errors and return the last meaningful result.
 **/
export function extractRepoResult(events: RepoEvent[]): Record<string, unknown> {
  const results: unknown[] = [];
  const controlKinds = new Set(["start", "progress", "log", "done"]);

  for (const event of events) {
    if (event.kind === "error" && event.error?.fatal) {
      throw new Error(event.error.message ?? "Repo command failed");
    }
    if (event.kind === "result") {
      results.push(event.result ?? event.data ?? null);
    } else if (!event.kind || !controlKinds.has(event.kind)) {
      results.push(event);
    }
  }

  if (results.length > 0 && results.some((r) => r && typeof r === "object" && "section" in r)) {
    const sections = results.map((r) => (r as any).section).filter((s) => s);
    if (sections.length > 0) {
      return { data: { sections } };
    }
  }

  let lastResult = results.length > 0 ? results[results.length - 1] : null;

  if (lastResult && typeof lastResult === "object" && !Array.isArray(lastResult)) {
    const res = lastResult as Record<string, unknown>;
    if ("data" in res || "errors" in res) {
      return res;
    }
    return { data: res };
  }
  return { data: lastResult };
}
// #endregion 🎼️Utilities
