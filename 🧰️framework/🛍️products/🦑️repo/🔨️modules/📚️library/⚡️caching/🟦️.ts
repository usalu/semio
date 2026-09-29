import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

let generatedDirectories: ReadonlySet<string> | undefined;

/** ⚡️ The single repository cache root shared by every tool, agent and dev (`.🧬semio/🦑️repo/⚡️cache/…`). */
export function repoCacheDirectory(repoRoot: string, ...segments: string[]): string {
  return join(repoRoot, ".🧬semio", "🦑️repo", "⚡️cache", ...segments);
}

/** 📁️ True when one directory name is a policy-declared generated directory (`dist`, `🗑️generated`, …). */
export function isGeneratedDirectoryName(name: string): boolean {
  generatedDirectories ??= new Set(JSON.parse(readFileSync(fileURLToPath(new URL("./🔣️policy.json", import.meta.url)), "utf8")).generatedDirectories);
  return generatedDirectories.has(name);
}

/** 🗑️ True when a path lies inside a policy-declared generated directory: disposable output that never feeds source inputs. */
export function isGeneratedPath(path: string): boolean {
  return path.split(/[\\/]/u).some(isGeneratedDirectoryName);
}
