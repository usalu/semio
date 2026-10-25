/** 🩺️ The placement diagnostics of the schema contract layer: forbidden placements, module directories on disk, measurement disagreements, owner eligibility and fixture isolation. */
import { join } from "node:path";
import { testFilenameForKind, testTaxonomy } from "../../🗂️taxonomy/🟦️.ts";
import { forbiddenSchemaPlacements, isFixtureOwnedPath, matchesTaxonomyPathPattern, readSchemaCatalog, schemaDiagnostic, type SchemaDiagnostic, schemaJsonDialect, schemaModuleDirName } from "../🟦️.ts";
import { isDeclaredDialectDefinition, readJson, readSchemaDefinition, schemaScopeEligibility, schemaTreeFiles } from "../🏗️scope-eligibility-and-tree/🟦️.ts";

//#region 🧬️SchemaContracts
/**
 * 📍️ Placement: a schema definition lives in its owner's `🧬️schema/` module, under the canonical
 * filename of its format, and nowhere else.
 */
export function schemaPlacementDiagnostics(repoRoot: string, files: readonly string[] = schemaTreeFiles(repoRoot), inert: ReadonlySet<string> = inertSchemaDataPaths(repoRoot, files)): SchemaDiagnostic[] {
  const found: SchemaDiagnostic[] = [];
  const seenForbiddenDirs = new Set<string>();
  const { directories, files: forbiddenFiles, exceptions } = forbiddenSchemaPlacements(repoRoot);
  const dialect = schemaJsonDialect(repoRoot);
  const moduleDirName = schemaModuleDirName(repoRoot);
  for (const rel of files) {
    if (exceptions.has(rel)) continue;
    const segments = rel.split("/");
    const name = segments[segments.length - 1]!;
    const dirSegments = segments.slice(0, -1);
    const forbiddenDir = dirSegments.reduce<string | null>((carry, _segment, index) => {
      if (carry !== null) return carry;
      const prefix = dirSegments.slice(0, index + 1).join("/");
      return directories.some((pattern) => matchesTaxonomyPathPattern(pattern, prefix)) ? prefix : null;
    }, null);
    if (forbiddenDir !== null && !seenForbiddenDirs.has(forbiddenDir) && !exceptions.has(forbiddenDir)) {
      seenForbiddenDirs.add(forbiddenDir);
      found.push(schemaDiagnostic("schema-contracts-directory-forbidden", `${forbiddenDir} is a per-contract directory; the contract belongs in the owning scope's ${moduleDirName}/ module as a named export`, { path: forbiddenDir }));
    }
    if (!name.endsWith(".json") || inert.has(rel)) continue;
    if (forbiddenFiles.some((pattern) => matchesTaxonomyPathPattern(pattern, rel))) {
      found.push(schemaDiagnostic("schema-placement-forbidden-filename", `${rel} uses a retired schema placement; a schema module writes JSON Schema as ${moduleDirName}/🔣️.json`, { path: rel }));
      continue;
    }
    if (isFixtureOwnedPath(repoRoot, rel)) continue;
    const document = readSchemaDefinition(join(repoRoot, rel));
    if (document === undefined) continue;
    const moduleDir = schemaModuleDirOf(dirSegments, moduleDirName);
    if (moduleDir === null) {
      if (forbiddenDir === null) found.push(schemaDiagnostic("schema-placement-outside-module", `${rel} defines a schema outside any ${moduleDirName}/ module`, { path: rel }));
      continue;
    }
    if (!isDeclaredDialectDefinition(repoRoot, document)) found.push(schemaDiagnostic("schema-dialect-not-draft-07", `${rel} declares ${JSON.stringify((document as { $schema: string }).$schema)}; this repository speaks ${dialect}`, { path: rel }));
  }
  return found;
}

/** 🧬️ The `🧬️schema/` module a directory belongs to, or null when it is outside every module. */
function schemaModuleDirOf(dirSegments: readonly string[], moduleDirName: string): string | null {
  const at = dirSegments.lastIndexOf(moduleDirName);
  return at < 0 ? null : dirSegments.slice(0, at + 1).join("/");
}

/** 📁️ Every `🧬️schema/` module directory the FILE WALK reaches that carries a schema definition. */
export function schemaModuleDirectoriesOnDisk(repoRoot: string, files: readonly string[] = schemaTreeFiles(repoRoot)): ReadonlySet<string> {
  const moduleDirName = schemaModuleDirName(repoRoot);
  const found = new Set<string>();
  for (const rel of files) {
    const segments = rel.split("/");
    if (!segments[segments.length - 1]!.endsWith(".json")) continue;
    const moduleDir = schemaModuleDirOf(segments.slice(0, -1), moduleDirName);
    if (moduleDir === null || found.has(moduleDir)) continue;
    if (readSchemaDefinition(join(repoRoot, rel)) !== undefined) found.add(moduleDir);
  }
  return found;
}

/**
 * ⚖️ The GATE FLAG for the two-measurement problem (ledger row 150).
 *
 * This layer measures the tree twice and the two measurements are independent: PLACEMENT walks the
 * filesystem and asks where each definition sits, RESOLUTION reads the derived catalog and asks what
 * each declared scope owes. When they disagree — a module on disk no catalog row names, a catalog row
 * whose module the walk never produced a definition for — every number either half reports is a number
 * about a different tree, and a subtree can read "clean" from one side while the other has never
 * looked at it. That silence is what row 150 asks to be broken, so the disagreement itself is a
 * finding: `schema-catalog-stale`, the code whose remedy — regenerate the catalog — is exactly right
 * for it, and which `schema check` already raises from its own side.
 *
 * Under `--under` both sides are narrowed to the same subtree, so the comparison stays like-for-like.
 */
export function schemaMeasurementDisagreementDiagnostics(repoRoot: string, files: readonly string[] = schemaTreeFiles(repoRoot), under = ""): SchemaDiagnostic[] {
  const { catalog } = readSchemaCatalog(repoRoot);
  if (catalog === null) return [];
  const selected = (path: string): boolean => under.length === 0 || path === under || path.startsWith(`${under}/`);
  const onDisk = [...schemaModuleDirectoriesOnDisk(repoRoot, files)].filter((path) => selected(path) && !isFixtureOwnedPath(repoRoot, path));
  const catalogued = new Map<string, string>();
  for (const [id, scope] of Object.entries(catalog.scopes)) if (selected(scope.path) && !catalogued.has(scope.path)) catalogued.set(scope.path, id);
  const found: SchemaDiagnostic[] = [];
  for (const path of onDisk.sort()) if (!catalogued.has(path)) found.push(schemaDiagnostic("schema-catalog-stale", `${path} carries a schema definition and no catalog row names it, so export resolution never measures this module`, { path }));
  for (const [path, id] of [...catalogued].sort((left, right) => left[0].localeCompare(right[0]))) if (!onDisk.includes(path)) found.push(schemaDiagnostic("schema-catalog-stale", `the catalog declares scope ${JSON.stringify(id)} at ${path}, and the tree walk finds no schema definition there`, { scope: id, path }));
  return found;
}

/** 🏛️ Owner eligibility: every `🧬️schema/` module on disk sits on a declared scope owner level. */
export function schemaOwnerEligibilityDiagnostics(repoRoot: string, files: readonly string[] = schemaTreeFiles(repoRoot)): SchemaDiagnostic[] {
  const found: SchemaDiagnostic[] = [];
  const moduleDirName = schemaModuleDirName(repoRoot);
  const owners = new Set<string>();
  for (const rel of files) {
    const dirSegments = rel.split("/").slice(0, -1);
    const moduleDir = schemaModuleDirOf(dirSegments, moduleDirName);
    if (moduleDir === null) continue;
    owners.add(moduleDir.slice(0, moduleDir.length - moduleDirName.length - 1));
  }
  for (const owner of [...owners].sort()) {
    const verdict = schemaScopeEligibility(repoRoot, owner);
    if (!verdict.eligible) found.push(schemaDiagnostic("schema-owner-ineligible", `${owner} carries a ${moduleDirName}/ module and is not an eligible scope owner — ${verdict.reason}`, { path: owner }));
  }
  return found;
}

/** 🧫️ Fixture isolation: an example never defines the contract it is an example of. */
export function schemaFixtureIsolationDiagnostics(repoRoot: string, files: readonly string[] = schemaTreeFiles(repoRoot), inert: ReadonlySet<string> = inertSchemaDataPaths(repoRoot, files)): SchemaDiagnostic[] {
  const found: SchemaDiagnostic[] = [];
  for (const rel of files) {
    if (!isFixtureOwnedPath(repoRoot, rel)) continue;
    if (!schemaCollectionContractPath(repoRoot, rel) && (!rel.endsWith(".json") || inert.has(rel) || !fixtureSchemaAuthority(readJson(join(repoRoot, rel))))) continue;
    found.push(schemaDiagnostic("schema-fixture-defines-schema", `${rel} defines a schema inside a fixture or test collection; move the contract into the owning scope and bind the example to it with a schema:// target`, { path: rel }));
  }
  return found;
}

/** 🚧️ Contract facets below example collections cannot be declared as inert parser inputs. */
function schemaCollectionContractPath(repoRoot: string, path: string): boolean {
  const segments = path.split("/");
  return segments.some((segment, index) => (segment === schemaModuleDirName(repoRoot) || segment === "🛂️schema") && isFixtureOwnedPath(repoRoot, segments.slice(0, index).join("/")))
    || isFixtureOwnedPath(repoRoot, path) && /^(?:(?:📐️|🔣️|🧬️)?schema\.json|.+\.schema\.json)$/u.test(segments.at(-1)!);
}

/** 🧭️ Independent fixture authority recognition includes coherent marker-free schema roots. */
function fixtureSchemaAuthority(value: unknown): boolean {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return false;
  const node = value as Record<string, unknown>;
  if (typeof node.$schema === "string" && node.$schema.match(/^https?:\/\/json-schema\.org\//u)) return true;
  const map = (value: unknown): value is Record<string, unknown> => value !== null && typeof value === "object" && !Array.isArray(value);
  const schemaTypes = ["null", "boolean", "object", "array", "number", "string", "integer"];
  if ("type" in node && !(Array.isArray(node.type) ? node.type.length > 0 && node.type.every(value => schemaTypes.includes(value)) : typeof node.type === "string" && schemaTypes.includes(node.type))) return false;
  if ("properties" in node && (!map(node.properties) || Object.values(node.properties).some(value => typeof value !== "boolean" && (!map(value) || Object.keys(value).length > 0 && !fixtureSchemaAuthority(value) && !Object.hasOwn(value, "const") && !Array.isArray(value.enum))))) return false;
  if ("required" in node && (!Array.isArray(node.required) || node.required.some(value => typeof value !== "string"))) return false;
  const constraints = /^(?:enum|const|minLength|maxLength|pattern|format|minimum|maximum|exclusiveMinimum|exclusiveMaximum|multipleOf|items|additionalItems|minItems|maxItems|uniqueItems|contains|required|additionalProperties|patternProperties|propertyNames|minProperties|maxProperties|dependencies|not|if|then|else)$/u;
  const annotation = /^(?:type|title|description|default|examples|readOnly|writeOnly|\$id|\$comment|\$schema)$/u;
  if (typeof node.$ref === "string" || map(node.properties) || "type" in node && (Object.keys(node).some(key => constraints.test(key)) || Object.keys(node).every(key => annotation.test(key)))) return true;
  for (const keyword of ["allOf", "anyOf", "oneOf"]) {
    const rows = node[keyword];
    if (Array.isArray(rows) && rows.length && rows.every(value => typeof value === "boolean" || fixtureSchemaAuthority(value))) return true;
  }
  for (const keyword of ["$defs", "definitions"]) if (map(node[keyword]) && Object.values(node[keyword]).some(fixtureSchemaAuthority)) return true;
  return false;
}

/**
 * 🧾️ Schema-shaped documents a case DECLARES as inert data — a parser test's own corpus.
 *
 * The declaration is explicit and lives in the case's own data file, so "this JSON is a subject, not
 * a contract" is a statement someone made rather than a shape something happened to have.
 */
export function inertSchemaDataPaths(repoRoot: string, files: readonly string[]): Set<string> {
  const declared = new Set<string>();
  const caseFile = testFilenameForKind(testTaxonomy(repoRoot), testTaxonomy(repoRoot).testContributionFileKindId);
  for (const rel of files) {
    if (!rel.endsWith(`/${caseFile}`) || !isFixtureOwnedPath(repoRoot, rel)) continue;
    const document = readJson(join(repoRoot, rel));
    const names = (document as { inertSchemaData?: unknown } | undefined)?.inertSchemaData;
    if (!Array.isArray(names)) continue;
    const dir = rel.slice(0, rel.lastIndexOf("/"));
    for (const name of names) if (typeof name === "string" && !schemaCollectionContractPath(repoRoot, `${dir}/${name}`)) declared.add(`${dir}/${name}`);
  }
  return declared;
}
//#endregion 🧬️SchemaContracts
