/** 🧭️ The workspace lookups of the two-client document e2e: the repository root and the hub root found by walking up from this module, and the emoji-tolerant child lookup. */
import { existsSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

export function findWorkspaceRoot(start: string): string {
  let current = start;
  for (let depth = 0; depth < 32; depth++) {
    if (readdirSync(current).some((n) => n.endsWith("hub") && existsSync(join(current, n, "📦️packages")))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error("repo root not found");
}

export function pick(parent: string, pred: (n: string) => boolean): string {
  const name = readdirSync(parent).find((n) => n !== "node_modules" && pred(n));
  if (!name) throw new Error(`missing child in ${parent}`);
  return join(parent, name);
}

export const repoRoot = findWorkspaceRoot(dirname(fileURLToPath(import.meta.url)));
export const hubRoot = pick(repoRoot, (n) => n.endsWith("hub") && !n.startsWith("."));
