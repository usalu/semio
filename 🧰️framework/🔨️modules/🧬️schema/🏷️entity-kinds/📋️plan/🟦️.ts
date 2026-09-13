/** 📋️ Exact three-language entity-kind output plan. */
import { join } from "node:path";
import type { EntityCatalogSource } from "../📥️source/🟦️.ts";
import { emitGo, emitRust, emitTypeScript } from "../📽️projection/🟦️.ts";

export type GeneratedTarget = { readonly path: string; readonly content: string };

export function generatedTargets(repoRoot: string, source: EntityCatalogSource): readonly GeneratedTarget[] {
  const schemaRoot = join(import.meta.dir, "..", "..");
  return [
    { path: join(schemaRoot, "🤖️generated", "🏷️entity-kinds", "🟦️.ts"), content: emitTypeScript(source) },
    { path: join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🏷️entity-kinds/🐹️.go"), content: emitGo(source) },
    { path: join(schemaRoot, "🤖️generated", "🏷️entity-kinds", "🦀️.rs"), content: emitRust(source) },
  ];
}
