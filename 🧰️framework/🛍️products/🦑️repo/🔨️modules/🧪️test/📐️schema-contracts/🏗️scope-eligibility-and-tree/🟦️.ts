/** 🏗️ Which scopes may own a schema: fixture-owned paths, the owner levels and their eligibility, the declared dialect, schema definition reading and the schema tree on disk. */
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { isExcludedTestPath, TAXONOMY_REL_PATH } from "../../🗂️taxonomy/🟦️.ts";
import { SKIP_DIR_NAMES } from "../../🧭️case-discovery/🟦️.ts";
import { matchesTaxonomyPathPattern, schemaJsonDialect, type SchemaScopeEligibility, schemaScopeOwnerLevels, submodulePaths } from "../🟦️.ts";

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
  if (document === null || typeof document !== "object" || Array.isArray(document)) return false;
  const dialect = (document as { $schema?: unknown }).$schema;
  return typeof dialect === "string" && /^https?:\/\/json-schema\.org\//u.test(dialect);
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
  if (!text.includes("json-schema.org")) return undefined;
  let parsed: unknown;
  try {
    parsed = JSON.parse(text);
  } catch {
    return undefined;
  }
  return isJsonSchemaDefinition(parsed) ? parsed : undefined;
}

/**
 * 🗂️ Every file under a subtree, repository-relative, skipping the taxonomy's exclusions.
 *
 * Three kinds of directory are never a schema authority and are cut at the directory, not per file:
 * dot-directories (tooling state — `.nx/cache` alone carries six generated copies of one manifest
 * schema), the `SKIP_DIR_NAMES` build and dependency outputs (`dist/` among them, so the gitignored
 * `dist/🧬️schema/📜️artifact-definition.json` copies under each artifact's TypeScript package cannot
 * double a law that walks the artifact tree — ledger row 153), and declared git submodules (row 133).
 */
export function schemaTreeFiles(repoRoot: string, under = ""): string[] {
  const submodules = submodulePaths(repoRoot);
  const files: string[] = [];
  const walk = (absDir: string, relDir: string): void => {
    let entries: import("node:fs").Dirent[];
    try {
      entries = readdirSync(absDir, { withFileTypes: true });
    } catch {
      return;
    }
    for (const entry of entries) {
      if (entry.isSymbolicLink()) continue;
      const rel = relDir.length === 0 ? entry.name : `${relDir}/${entry.name}`;
      if (entry.isDirectory()) {
        if (entry.name.startsWith(".") || SKIP_DIR_NAMES.has(entry.name) || submodules.has(rel.normalize("NFC")) || isExcludedTestPath(repoRoot, rel)) continue;
        walk(join(absDir, entry.name), rel);
        continue;
      }
      if (entry.isFile()) files.push(rel);
    }
  };
  walk(join(repoRoot, under), under);
  return files.sort();
}
//#endregion 🧬️SchemaContracts
