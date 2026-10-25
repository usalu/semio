import { type Dirent, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

const POLICY_DISCOVERY_SKIPPED_DIRS = new Set(["node_modules", ".git", ".nx", ".venv", "target", "dist", "build", "out", "__pycache__", "obj", "bin", "storybook-static", "🎫️tickets", "⚡️cache", "🤖️generated", "🗑️generated"]);

/** 🔒️ Filename an area uses to contribute allowlist entries to repository policy rules. */
export const POLICY_ALLOWLIST_FILENAME = "🔒️policy-allowlist.json";

/**
 * 🔀️ Merges one policy rule's allowlist from every area that contributes to it. A rule therefore
 * never names a path it exempts: the exemption lives with the code it describes, and deleting that
 * code deletes the exemption. Discovery is by convention, so a new area needs no code change.
 */
export function policyDiscoveredAllowlist(repoRoot: string, key: string): Set<string> {
  const merged = new Set<string>();
  const walk = (relDir: string): void => {
    let entries: Dirent[];
    try {
      entries = readdirSync(join(repoRoot, relDir || "."), { withFileTypes: true });
    } catch {
      return;
    }
    for (const entry of entries) {
      const childRel = relDir ? `${relDir}/${entry.name}` : entry.name;
      if (entry.isDirectory()) {
        if (!POLICY_DISCOVERY_SKIPPED_DIRS.has(entry.name)) walk(childRel);
        continue;
      }
      if (entry.name !== POLICY_ALLOWLIST_FILENAME) continue;
      try {
        const parsed = JSON.parse(readFileSync(join(repoRoot, childRel), "utf8")) as Record<string, unknown>;
        for (const value of (parsed[key] as string[] | undefined) ?? []) merged.add(value);
      } catch {
        /* an unreadable allowlist contributes nothing, which fails closed */
      }
    }
  };
  walk("");
  return merged;
}
