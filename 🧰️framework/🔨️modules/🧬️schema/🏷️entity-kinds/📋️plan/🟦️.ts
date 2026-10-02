/** 📋️ Exact neutral two-language entity-kind output plan. */
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import type { EntityCatalogSource } from "../📥️source/🟦️.ts";
import { emitRust, emitTypeScript } from "../📽️projection/🟦️.ts";

export type GeneratedTarget = { readonly path: string; readonly content: string };

export function generatedTargets(source: EntityCatalogSource): readonly GeneratedTarget[] {
  const schemaRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
  return [
    { path: join(schemaRoot, "🤖️generated", "🏷️entity-kinds", "🟦️.ts"), content: emitTypeScript(source) },
    { path: join(schemaRoot, "🤖️generated", "🏷️entity-kinds", "🦀️.rs"), content: emitRust(source) },
  ];
}
