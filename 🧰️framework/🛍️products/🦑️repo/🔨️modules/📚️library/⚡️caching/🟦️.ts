import { join } from "node:path";

/** ⚡️ The single repository cache root shared by every tool, agent and dev (`.🧬semio/🦑️repo/⚡️cache/…`). */
export function repoCacheDirectory(repoRoot: string, ...segments: string[]): string {
  return join(repoRoot, ".🧬semio", "🦑️repo", "⚡️cache", ...segments);
}
