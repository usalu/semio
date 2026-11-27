/** 🏗️ Which scopes may own a schema: fixture-owned paths, the owner levels and their eligibility, the declared dialect, schema definition reading and the schema tree on disk. */
import { readFileSync } from "node:fs";
import { TAXONOMY_REL_PATH, testTaxonomy } from "../../🗂️taxonomy/🟦️.ts";
import { isSchemaDefinitionDocument, parseSchemaDefinitionText, walkSchemaTree } from "../../../📚️library/🔍️discovery/🧾️schema-scope-catalog/🟦️.ts";
import { matchesTaxonomyPathPattern, schemaJsonDialect, type SchemaScopeEligibility, schemaScopeOwnerLevels } from "../🟦️.ts";

//#region 🧬️SchemaContracts
/**
 * 🏛️ Whether a directory may own a `🧬️schema/` module.
 *
 * Eligibility is POSITIONAL and it is DECLARED: the levels and the exclusions come from
 * `schemaScopeOwnerLevels` in the taxonomy, never from a table restated here. A second table would be
 * a second opinion, and the whole point of this layer is that there is exactly one.
 */
export function schemaScopeEligibility(repoRoot: string, ownerPath: string): SchemaScopeEligibility {
  const path = ownerPath.replace(/\/+$/u, "");
  const declared = schemaScopeOwnerLevels(repoRoot);
  if (declared === null) return { path, eligible: false, level: null, reason: `${TAXONOMY_REL_PATH} declares no schemaScopeOwnerLevels` };
  const excluded = declared.excludedOwnerPathPatterns.find((pattern) => matchesTaxonomyPathPattern(pattern, path));
  if (excluded !== undefined) return { path, eligible: false, level: null, reason: `excluded by ${excluded}: a directory matching it holds examples, publications or presentation, never authority` };
  for (const [level, spec] of Object.entries(declared.levels)) {
    if (spec.pathPatterns.some((pattern) => matchesTaxonomyPathPattern(pattern, path))) return { path, eligible: true, level, reason: `${level}: ${spec.reason}` };
  }
  return { path, eligible: false, level: null, reason: "no declared scope owner level matches this path" };
}

/** 🧬️ Whether a parsed JSON document DEFINES a schema, as opposed to being validated by one. */
export function isJsonSchemaDefinition(document: unknown): boolean {
  return isSchemaDefinitionDocument(document);
}

/** 📑️ Whether a schema definition declares the one dialect the taxonomy names. */
export function isDeclaredDialectDefinition(repoRoot: string, document: unknown): boolean {
  return isJsonSchemaDefinition(document) && (document as { $schema: string }).$schema === schemaJsonDialect(repoRoot);
}

export function readJson(abs: string): unknown | undefined {
  try {
    return JSON.parse(readFileSync(abs, "utf8"));
  } catch {
    return undefined;
  }
}

/**
 * 📖️ Reads a file only far enough to decide whether it DEFINES a schema.
 *
 * A definition necessarily names a json-schema.org meta-schema, so the substring test settles the
 * overwhelming majority of a monorepo's JSON without parsing it. The parse still decides — the
 * prefilter can only produce candidates, never verdicts.
 */
export function readSchemaDefinition(abs: string): unknown | undefined {
  let text: string;
  try {
    text = readFileSync(abs, "utf8");
  } catch {
    return undefined;
  }
  return parseSchemaDefinitionText(text);
}

/**
 * 🗂️ Every file under a subtree, repository-relative, skipping the taxonomy's exclusions: the generator's own walk, `walkSchemaTree`, so the tree this layer measures is the tree the catalog is generated from.
 *
 * Three kinds of directory are never a schema authority and are cut at the directory, not per file:
 * dot-directories (tooling state — `.nx/cache` alone carries six generated copies of one manifest
 * schema), the `SKIP_DIR_NAMES` build and dependency outputs (`dist/` among them, so the gitignored
 * `dist/🧬️schema/📜️artifact-definition.json` copies under each artifact's TypeScript package cannot
 * double a law that walks the artifact tree — ledger row 153), and declared git submodules (row 133).
 */
export function schemaTreeFiles(repoRoot: string, under = ""): string[] {
  return [...walkSchemaTree(repoRoot, testTaxonomy(repoRoot), under).files].sort();
}
//#endregion 🧬️SchemaContracts
