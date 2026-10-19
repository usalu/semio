/** 🧰️ The shared taxonomy v7 parse helpers: the expiry reader, the opaque path exclusion table and the projection types the parse phases use. */
import { requireString } from "@semio-tech/schema-validator-ts";

export function fixedExpiry(value: unknown, name: string): string | null {
  if (value === null) return null;
  const expires = requireString(value, name);
  if (!/^\d{4}-\d{2}-\d{2}$/u.test(expires)) throw new Error(`Taxonomy v7 ${name} must be null or YYYY-MM-DD`);
  return expires;
}

/** 🚧️ The exact opaque subtrees `🔣️taxonomy.json` may declare, in order: two user-owned scratch
 * trees plus the one tracked nested-repository gitlink (`git ls-files -s` mode `160000`), which must be
 * filtered lexically here or `inventoryTaxonomyWithSourceParentPruning` refuses to classify anything at
 * all. Mirrors `🔍️discovery/👥️mutation-owner-rules/🟦️.ts`'s `OPAQUE_PATH_EXCLUSIONS`; the two files are separate bundles with
 * no shared import path for a three-row table. The gitlink is named, never its parent `♻️mit-bestand/`,
 * whose `📋️bericht`/`🖼️asset` subtrees are generator-contract outputs and may not sit inside an opaque
 * subtree. */
export const TAXONOMY_OPAQUE_PATH_EXCLUSIONS: readonly (readonly [string, string])[] = [
  ["compose", "compose/"],
  ["temp-compose", "temp/compose/"],
  ["mit-bestand-recherche", "♻️mit-bestand/🔎️recherche/"],
];
