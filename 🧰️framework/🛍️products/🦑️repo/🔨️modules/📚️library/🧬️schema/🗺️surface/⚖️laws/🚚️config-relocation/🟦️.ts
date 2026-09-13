import type { BreachRecord } from "../../../../🟦️.ts";
import { POLICY_SKIP_DIRS, POLICY_SOURCE_OPERATIONS, policySourceDirectory, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { POLICY_APP_CONFIG_DIR, POLICY_APP_CONFIG_LEGACY_DIR, POLICY_APP_WASM_LEGACY_DIR } from "../../🧱️contract/🟦️.ts";

/** 🚚️ Reports legacy config and wasm directories without suppressing unavailable source evidence. */
export function policyAppSchemaConfigRelocationBreaches(repoRoot: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): BreachRecord[] {
  const breaches: BreachRecord[] = [],
    banned = new Set([POLICY_APP_CONFIG_LEGACY_DIR, POLICY_APP_WASM_LEGACY_DIR]),
    walk = (relDir: string): void => {
      const source = policySourceDirectory(repoRoot, relDir, operations);
      if (source.state === "missing") return;
      if (source.state !== "directory") {
        breaches.push({
          id: `app-schema-source-unreadable-${relDir}`,
          summary: `"${relDir}" is ${source.state}`,
          kind: "app-schema/source-unreadable",
          scope: relDir,
          priority: "high",
          reason: "Config relocation requires an admitted directory tree and never follows symbolic links.",
          solution: `Restore a readable regular directory at ${relDir}.`,
        });
        return;
      }
      for (const entry of source.entries) {
        if (!entry.isDirectory || entry.isSymbolicLink || POLICY_SKIP_DIRS.has(entry.name) || entry.name.startsWith(".")) continue;
        const childRel = `${relDir}/${entry.name}`;
        if (banned.has(entry.name)) {
          const kindLabel = entry.name === POLICY_APP_CONFIG_LEGACY_DIR ? "legacy abacus config" : "legacy spider-web wasm",
            replacement = entry.name === POLICY_APP_CONFIG_LEGACY_DIR ? POLICY_APP_CONFIG_DIR : "🌉️wasm";
          breaches.push({
            id: `app-schema-config-relocation-${childRel}`,
            summary: `"${childRel}" must move to ${replacement}`,
            kind: "app-schema/config-relocation",
            scope: childRel,
            priority: "high",
            reason: `${kindLabel} dirs are forbidden under ✏️s/🔌️plugins; consolidate onto the canonical emoji.`,
            solution: `Rename ${childRel}/ to use ${replacement}/ and update glue #[path] mounts.`,
          });
        }
        walk(childRel);
      }
    };
  walk("✏️s/🔌️plugins");
  return breaches;
}
