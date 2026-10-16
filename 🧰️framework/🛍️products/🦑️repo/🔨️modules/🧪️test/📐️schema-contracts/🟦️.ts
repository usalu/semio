/** 📐️ Schema contracts
 *
 * Schema catalog, export resolution, placement rules and the schema diagnostics of the test platform.
 */

//#region 🔌️Adapters
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { isExcludedTestPath, TAXONOMY_REL_PATH, testFilenameForKind, testTaxonomy } from "../🗂️taxonomy/🟦️.ts";
import { fileDigest } from "../🔢️digests/🟦️.ts";
import { SKIP_DIR_NAMES } from "../🧭️case-discovery/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧬️SchemaContracts
/**
 * 🧬️ The scope-owned schema contract layer: exactly one eligible owner per contract, named exports
 * resolved by `(scope id, export id, format id)` through the DERIVED CATALOG, and fixtures that bind
 * to an export instead of defining one.
 *
 * Every lookup here is explicit. There is no nearest-parent walk, no glob, no fixture-local fallback
 * and no repo-wide type scan: a name either resolves through the declared catalog or produces a
 * NAMED diagnostic. That is the whole discipline — a contract found by proximity is a contract
 * nobody declared, and it validates the wrong shape exactly as confidently as the right one.
 *
 * @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/SCOPE-OWNED-SCHEMA-CONTRACTS/📋️execution-contract.md
 */
export function schemaModuleDirName(repoRoot: string): string {
  const declared = (rawTaxonomy(repoRoot).schemaScopeOwnerLevels as { facetDirName?: unknown } | undefined)?.facetDirName;
  if (typeof declared !== "string" || declared.length === 0) throw new Error(`${TAXONOMY_REL_PATH} declares no \`schemaScopeOwnerLevels.facetDirName\``);
  return declared;
}

/**
 * 🧬️ The one JSON Schema dialect this repository speaks, as the taxonomy states it.
 *
 * Restating the dialect string here would make this file a second authority on it: the day the
 * taxonomy moves the tree to another draft, a constant compiled into the harness keeps reporting
 * the old one as correct.
 */
export function schemaJsonDialect(repoRoot: string): string {
  const declared = rawTaxonomy(repoRoot).schemaJsonDialect;
  if (typeof declared !== "string" || declared.length === 0) throw new Error(`${TAXONOMY_REL_PATH} declares no \`schemaJsonDialect\``);
  return declared;
}

/**
 * 🚫️ The placements the contract abolishes, as the taxonomy writes them.
 *
 * `schemaExportResolution.forbiddenPlacementPatterns` is the only list; the harness never restates a
 * retired filename (`*.schema.json`, `🧬️schema.json`) or a retired directory (`🧬️contracts/`).
 * `placementExceptions` maps a repository-relative path to the reason it survives.
 */
export function forbiddenSchemaPlacements(repoRoot: string): { directories: readonly string[]; files: readonly string[]; exceptions: ReadonlyMap<string, string> } {
  const declared = rawTaxonomy(repoRoot).schemaExportResolution as { forbiddenPlacementPatterns?: unknown; placementExceptions?: unknown } | undefined;
  const patterns = Array.isArray(declared?.forbiddenPlacementPatterns) ? (declared.forbiddenPlacementPatterns as readonly unknown[]).filter((entry): entry is string => typeof entry === "string") : [];
  // 🧭️A pattern the list also carries in `<pattern>/**` form names a forbidden DIRECTORY — the
  // descendant form is how the taxonomy says "and everything under it". Every other pattern names a
  // forbidden FILE. The split is read off the declaration; no directory or filename is named here.
  const set = new Set(patterns);
  const directories = patterns.filter((pattern) => set.has(`${pattern}/**`));
  const descendants = new Set(directories.map((pattern) => `${pattern}/**`));
  const files = patterns.filter((pattern) => !set.has(`${pattern}/**`) && !descendants.has(pattern));
  const rawExceptions = declared?.placementExceptions;
  const exceptions = new Map<string, string>();
  if (rawExceptions !== null && typeof rawExceptions === "object" && !Array.isArray(rawExceptions)) for (const [path, reason] of Object.entries(rawExceptions as Record<string, unknown>)) exceptions.set(path, typeof reason === "string" ? reason : "");
  return { directories, files, exceptions };
}

/**
 * 🏭️ The two implementations that emit the shared schema-contract vocabulary: this harness, and the
 * root script's `schema check`. A code names its emitters so the table can hold the codes only one
 * side can raise without either side pretending to raise the other's.
 */
export const SCHEMA_DIAGNOSTIC_EMITTERS = ["harness", "check"] as const;
export type SchemaDiagnosticEmitter = (typeof SCHEMA_DIAGNOSTIC_EMITTERS)[number];

/**
 * 🔠️ The single schema-contract diagnostic code table: every code EITHER implementation emits, each
 * with the one-line description a reader needs to act on the finding and the emitters that raise it.
 *
 * This constant IS the contract between the two implementations of the rule set (ledger rows 97, 125):
 * the harness emits from it and the root `schema check` imports it rather than restating a second
 * vocabulary. Import path:
 * `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`
 * → `SCHEMA_DIAGNOSTIC_CODE_TABLE` (descriptions + emitters), `SCHEMA_DIAGNOSTIC_CODES` (the ids, in
 * order), `SchemaDiagnosticCode` (the union), `SchemaHarnessDiagnosticCode` (the half this file may
 * raise). A code added here is added in one place only; the protocol schema's `SchemaDiagnosticCode`
 * enum is held equal to it by the invariants suite.
 *
 * `emitters` is a LIST, not a single tag: a rule implemented on both sides — the catalog states, the
 * placement and export rules `schema check` re-derives over the whole tree — is one code with two
 * emitters, and writing one of them would be a lie about the other.
 */
export const SCHEMA_DIAGNOSTIC_CODE_TABLE = {
  "schema-export-resolution-undeclared": { emitters: ["harness"], description: "the taxonomy declares no `schemaExportResolution`, so `schema://` addresses nothing" },
  "schema-catalog-missing": { emitters: ["harness", "check"], description: "the derived schema catalog the taxonomy points at does not exist" },
  "schema-catalog-malformed": { emitters: ["harness", "check"], description: "the catalog exists but is not the declared `{ scopes: { <id>: { path, formats, exports, dependsOn } } }` document" },
  "schema-catalog-stale": { emitters: ["harness", "check"], description: "the derived schema catalog does not match the schema modules on disk; regenerate it" },
  "schema-uri-malformed": { emitters: ["harness"], description: "a `schema://` uri is not `schema://<scope id>/<ExportId>`" },
  "schema-scope-unknown": { emitters: ["harness"], description: "a `schema://` uri names a scope id the catalog does not declare" },
  "schema-scope-ambiguous": { emitters: ["harness", "check"], description: "two catalog rows claim the same scope id" },
  "schema-export-unknown": { emitters: ["harness"], description: "a scope is addressed for an export it does not declare, or its carrier file declares no such `$defs` member" },
  "schema-format-unavailable": { emitters: ["harness"], description: "a scope implements no file for the format that was asked for, the normative one included" },
  "schema-file-missing": { emitters: ["harness"], description: "the catalog names a file for a scope, format or export and that file does not exist" },
  "schema-ref-unresolved": { emitters: ["harness", "check"], description: "a cross-document `$ref` addresses no export: a relative file path, an unknown `$id`, or a fragment other than `#/$defs/<ExportId>`" },
  "schema-ref-broken-internal": { emitters: ["harness"], description: "a module-internal JSON pointer (`#…`) lands on nothing in its own document" },
  "schema-cross-scope-dependency-forbidden": { emitters: ["harness", "check"], description: "a scope references another scope without declaring it in `dependsOn`" },
  "schema-placement-forbidden-filename": { emitters: ["harness", "check"], description: "a schema document carries a filename the taxonomy retired (`*.schema.json`, `🧬️schema.json`, …)" },
  "schema-placement-outside-module": { emitters: ["harness", "check"], description: "a schema document lives outside any `🧬️schema/` module of an eligible owner" },
  "schema-contracts-directory-forbidden": { emitters: ["harness"], description: "a schema document lives under the retired per-contract `🧬️contracts/<x>` shape" },
  "schema-owner-ineligible": { emitters: ["harness", "check"], description: "a `🧬️schema/` module hangs off a directory level the taxonomy does not admit as a scope owner" },
  "schema-export-incomplete": { emitters: ["harness", "check"], description: "an export is missing from a format its scope provides, or from a format its `x-semio-formats` declared" },
  "schema-export-parser-missing": { emitters: ["harness"], description: "a TypeScript export declares its type but no `parse<Export>()` entry point, so the format cannot decode it" },
  "schema-export-format-undeclared": { emitters: ["harness"], description: "a format declares the export while its `x-semio-formats` does not name that format, the normative one included" },
  "schema-export-formats-annotation-invalid": { emitters: ["harness", "check"], description: "`x-semio-formats` is not a non-empty array of taxonomy format ids" },
  "schema-export-id-duplicate": { emitters: ["check"], description: "one scope resolves the same export id to two carrier files, so `schema://<scope>/<Export>` names both" },
  "schema-export-id-invalid": { emitters: ["check"], description: "an export key is not a PascalCase export id, so no format can declare an entity by that name" },
  "schema-dialect-not-draft-07": { emitters: ["harness", "check"], description: "a schema document declares a `$schema` dialect other than the single one the taxonomy allows" },
  "schema-module-id-missing": { emitters: ["harness", "check"], description: "a schema module declares no `$id`, so nothing can `$ref` it across scopes" },
  "schema-document-id-duplicate": { emitters: ["check"], description: "two schema documents declare the same `$id`, so a cross-scope `$ref` to it addresses either of them" },
  "schema-document-id-unaddressable": { emitters: ["check"], description: "a schema document's `$id` is not the `https://json.schemas.assets.semio-tech.com/<scope path>/<facet>.json` form the catalog resolves" },
  "schema-module-id-inconsistent": { emitters: ["check"], description: "a facet document's `$id` deepens or leaves its module's scope path instead of varying only the facet filename" },
  "schema-document-unparseable": { emitters: ["check"], description: "a file in the canonical schema slot is not parseable JSON at all" },
  "schema-document-not-object": { emitters: ["check"], description: "a schema module leaf parses but is not a JSON Schema object" },
  "schema-enum-empty": { emitters: ["check"], description: "an `enum: []` admits no instance; state the intent with `propertyNames: false` / `maxProperties: 0` or delete the keyword" },
  "schema-cross-scope-dependency-uncataloged": { emitters: ["check"], description: "a `$ref` resolves into a document no catalogued scope owns, so no `dependsOn` entry could ever declare it" },
  "schema-mutation-leaf-id": { emitters: ["check"], description: "a mutation leaf's `$id` is not the `<root scope path>/mutation/<semanticKind>/schema.json` form its own scope derives" },
  "schema-mutation-leaf-schema-absent": { emitters: ["check"], description: "a described mutation leaf carries no `🧬️schema/🔣️.json`, so the declared authority for its payload does not exist" },
  "schema-mutation-aggregate-id": { emitters: ["check"], description: "a mutations aggregate's `$id` is not its own module's `mutations.json` facet, so the aggregate deepens the scope" },
  "schema-mutation-aggregate-kinds-redundant": { emitters: ["check"], description: "an aggregate carries `x-semio-mutationKinds`, which restates its `oneOf` `$ref` union and has no reader" },
  "schema-mutation-input-ui": { emitters: ["harness"], description: "a mutation leaf input has no resolvable UI descriptor: no label in every locale (x-semio-ui or the input-label glossary), an invalid x-semio-ui, or a widget its value cannot take" },
  "schema-mutation-payload-parity": { emitters: ["harness"], description: "a committed mutation fixture's leaf payload (cut out of the aggregate wire value as `payload_value()` does) fails or is not fully described by its leaf payload schema, sits in another wire layout, maps to no leaf, or the leaf schema holds an object node without members" },
  "schema-mutation-label": { emitters: ["harness"], description: "a history row could show something other than its leaf's label in every locale: a leaf label that is locale-invariant data, an empty locale, an operation's text line, a body the gate cannot read, or an app overriding the leaf label" },
  "schema-mutation-editability": { emitters: ["harness"], description: "a mutation aggregate the generic history editor cannot edit without a declared reason: a hand-written `impl Mutation` that neither forwards the payload accessors nor is a lane, fixture or uninhabited, a generic aggregate without an emitted payload law, or an aggregate variant without a leaf descriptor" },
  "schema-fault-notice": { emitters: ["harness"], description: "a fault a guest can refuse with could reach the person as a raw code or English text: a guest code no framework or app table labels in every locale, a code that cannot be an app notice code, an anonymous `Fault::from(text)`, an invalid, unreadable or stale `fault_notices` table, or a committed descriptor publishing other notices than the sources declare" },
  "schema-fixture-defines-schema": { emitters: ["harness", "check"], description: "a contract facet lives inside examples, or an undeclared schema definition masquerades as example data; inert parser inputs never authorize contract facets" },
  "schema-fixture-local-schema-fallback": { emitters: ["harness"], description: "a fixture resolves its contract from a fixture-local copy instead of the owning scope" },
  "schema-fixture-metadata-invalid": { emitters: ["harness"], description: "a schema-bound fixture's declaration is not the shape the test protocol states" },
  "schema-fixture-parse-failed": { emitters: ["harness"], description: "a schema-bound fixture's payload could not be parsed in the format it was bound to" },
  "schema-instance-invalid": { emitters: ["harness"], description: "an instance fails structural validation against the export it was bound to" },
} as const;

/** 🔠️ Every diagnostic in the shared vocabulary, in table order. A distinct code per distinguishable failure. */
export const SCHEMA_DIAGNOSTIC_CODES = Object.keys(SCHEMA_DIAGNOSTIC_CODE_TABLE) as readonly (keyof typeof SCHEMA_DIAGNOSTIC_CODE_TABLE)[];
export type SchemaDiagnosticCode = keyof typeof SCHEMA_DIAGNOSTIC_CODE_TABLE;

/** 🏭️ The codes one emitter raises, as a type. `schemaDiagnostic` takes the harness half, so a code
 * only `schema check` can raise is a compile error here rather than a finding no rule produces. */
export type SchemaDiagnosticCodeEmittedBy<E extends SchemaDiagnosticEmitter> = { [K in SchemaDiagnosticCode]: E extends (typeof SCHEMA_DIAGNOSTIC_CODE_TABLE)[K]["emitters"][number] ? K : never }[SchemaDiagnosticCode];
export type SchemaHarnessDiagnosticCode = SchemaDiagnosticCodeEmittedBy<"harness">;

/** 🏭️ The codes one emitter raises, at runtime, in table order. */
export function schemaDiagnosticCodesEmittedBy(emitter: SchemaDiagnosticEmitter): readonly SchemaDiagnosticCode[] {
  return SCHEMA_DIAGNOSTIC_CODES.filter((code) => (SCHEMA_DIAGNOSTIC_CODE_TABLE[code].emitters as readonly string[]).includes(emitter));
}

/** 🩺️ One finding, with everything needed to act on it and nothing inferred. */
export type SchemaDiagnostic = Readonly<{ code: SchemaHarnessDiagnosticCode; scope: string | null; export: string | null; format: string | null; path: string | null; detail: string }>;

function schemaDiagnostic(code: SchemaHarnessDiagnosticCode, detail: string, where: Partial<Omit<SchemaDiagnostic, "code" | "detail">> = {}): SchemaDiagnostic {
  return { code, detail, scope: where.scope ?? null, export: where.export ?? null, format: where.format ?? null, path: where.path ?? null };
}

/** 🔣️ The `schema://` scheme and the catalog it resolves through, as the taxonomy declares them. */
export type SchemaExportResolution = Readonly<{ uriScheme: string; catalogPath: string; uriPattern: string | null }>;

/** 📚️ One scope's row in the derived catalog. `path` is the module directory; `formats` values are relative to it. */
export type SchemaCatalogScope = Readonly<{ path: string; formats: Readonly<Record<string, string>>; exports: Readonly<Record<string, SchemaCatalogExport>>; dependsOn: readonly string[]; hashes?: Readonly<Record<string, string>> }>;

/**
 * 📚️ One export's row: the module-relative file that CARRIES it, and the facet that file belongs to.
 *
 * A scope's `formats` names the module root only, and 1 006 of the repository's exports are declared
 * in a facet child (`🔺️diff/`, `📸️snapshot/`, `💡️inferences/`, `🧬️mutations/`). Without a per-export
 * file, `schema://<scope>/<Export>` resolves to a document that does not declare the export — and the
 * only way to find the right one would be to search the four children, which is the nearest-parent
 * habit this layer exists to remove.
 */
export type SchemaCatalogExport = Readonly<{ file: string; facet: string }>;

/** 📚️ The derived catalog: the single place `(scope id, export id, format id)` is answered from. */
export type SchemaCatalog = Readonly<{ scopes: Readonly<Record<string, SchemaCatalogScope>> }>;

/** 🎯️ One resolved export — the file a consumer actually reads, pinned by digest. */
export type ResolvedSchemaExport = Readonly<{ uri: string; scope: string; export: string; format: string; path: string; digest: string }>;

const rawTaxonomyCache = new Map<string, Record<string, unknown>>();
const schemaCatalogCache = new Map<string, { catalog: SchemaCatalog | null; diagnostics: readonly SchemaDiagnostic[] }>();

/** ♻️ Drops the taxonomy/catalog memoisation. Synthetic-tree tests build a new repository per case. */
export function clearSchemaContractCache(): void {
  rawTaxonomyCache.clear();
  schemaCatalogCache.clear();
  submodulePathCache.clear();
}

export function rawTaxonomy(repoRoot: string): Record<string, unknown> {
  const cached = rawTaxonomyCache.get(repoRoot);
  if (cached !== undefined) return cached;
  const parsed = JSON.parse(readFileSync(join(repoRoot, TAXONOMY_REL_PATH), "utf8")) as Record<string, unknown>;
  rawTaxonomyCache.set(repoRoot, parsed);
  return parsed;
}

/**
 * 🔣️ Reads the `schema://` resolution declaration out of the taxonomy.
 *
 * The catalog's location is VOCABULARY, not a constant restated here: two places that both know
 * where the catalog lives are two places that can disagree about it. When the taxonomy declares
 * nothing, this refuses with a named diagnostic rather than defaulting to a path — a default would
 * make an undeclared mechanism look configured.
 */
export function schemaExportResolution(repoRoot: string): { resolution: SchemaExportResolution | null; diagnostics: SchemaDiagnostic[] } {
  const declared = rawTaxonomy(repoRoot).schemaExportResolution;
  if (declared === undefined || declared === null || typeof declared !== "object" || Array.isArray(declared)) {
    return { resolution: null, diagnostics: [schemaDiagnostic("schema-export-resolution-undeclared", `${TAXONOMY_REL_PATH} declares no \`schemaExportResolution\`; the \`schema://\` scheme has no catalog to resolve through`, { path: TAXONOMY_REL_PATH })] };
  }
  const { uriScheme, catalogPath, uriPattern } = declared as { uriScheme?: unknown; catalogPath?: unknown; uriPattern?: unknown };
  if (uriScheme !== "schema" || typeof catalogPath !== "string" || catalogPath.length === 0) {
    return { resolution: null, diagnostics: [schemaDiagnostic("schema-export-resolution-undeclared", `${TAXONOMY_REL_PATH} \`schemaExportResolution\` must declare \`uriScheme\` "schema" and a non-empty repository-relative \`catalogPath\``, { path: TAXONOMY_REL_PATH })] };
  }
  return { resolution: { uriScheme, catalogPath, uriPattern: typeof uriPattern === "string" ? uriPattern : null }, diagnostics: [] };
}

/** 🔣️ The normative format of the `🧬️data` facet — the one a bare `schema://` URI resolves to. */
export function normativeSchemaFormat(repoRoot: string): string {
  const kinds = rawTaxonomy(repoRoot).schemaFacetKinds as Record<string, { normativeFormat?: string }> | undefined;
  const normative = kinds?.["🧬️data"]?.normativeFormat;
  if (typeof normative !== "string" || normative.length === 0) throw new Error(`${TAXONOMY_REL_PATH} declares no normative format for the 🧬️data schema facet`);
  return normative;
}

/**
 * 📚️ Reads the derived catalog from the declared path.
 *
 * It is READ, never searched for: an absent catalog is reported as absent. Walking the tree to find
 * "a file that looks like the catalog" would reintroduce, one layer up, exactly the discovery
 * heuristic this whole layer exists to remove.
 */
export function readSchemaCatalog(repoRoot: string): { catalog: SchemaCatalog | null; diagnostics: readonly SchemaDiagnostic[] } {
  const cached = schemaCatalogCache.get(repoRoot);
  if (cached !== undefined) return cached;
  const result = loadSchemaCatalog(repoRoot);
  schemaCatalogCache.set(repoRoot, result);
  return result;
}

function loadSchemaCatalog(repoRoot: string): { catalog: SchemaCatalog | null; diagnostics: readonly SchemaDiagnostic[] } {
  const { resolution, diagnostics } = schemaExportResolution(repoRoot);
  if (resolution === null) return { catalog: null, diagnostics };
  const abs = join(repoRoot, resolution.catalogPath);
  if (!existsSync(abs)) return { catalog: null, diagnostics: [schemaDiagnostic("schema-catalog-missing", `the declared schema catalog ${resolution.catalogPath} does not exist; generate it with \`schema generate\` before resolving any schema:// reference`, { path: resolution.catalogPath })] };
  let text: string;
  try {
    text = readFileSync(abs, "utf8");
  } catch (error) {
    return { catalog: null, diagnostics: [schemaDiagnostic("schema-catalog-malformed", `the schema catalog could not be read: ${error instanceof Error ? error.message : String(error)}`, { path: resolution.catalogPath })] };
  }
  let parsed: unknown;
  try {
    parsed = JSON.parse(text);
  } catch (error) {
    return { catalog: null, diagnostics: [schemaDiagnostic("schema-catalog-malformed", `the schema catalog is not JSON: ${error instanceof Error ? error.message : String(error)}`, { path: resolution.catalogPath })] };
  }
  const scopes = (parsed as { scopes?: unknown }).scopes;
  if (scopes === undefined || scopes === null || typeof scopes !== "object" || Array.isArray(scopes)) {
    return { catalog: null, diagnostics: [schemaDiagnostic("schema-catalog-malformed", "the schema catalog carries no `scopes` object", { path: resolution.catalogPath })] };
  }
  const found: SchemaDiagnostic[] = [];
  const rows: Record<string, SchemaCatalogScope> = {};
  const byPath = new Map<string, string>();
  // 🪞️A scope id declared twice in the catalog TEXT collapses on parse, so the raw bytes are counted
  // ONCE, here: two owners claiming one id is an ambiguity nothing downstream could otherwise see.
  const declarations = new Map<string, number>();
  for (const match of text.matchAll(/"([^"\\]+)"\s*:\s*\{/gu)) declarations.set(match[1]!, (declarations.get(match[1]!) ?? 0) + 1);
  for (const [id, row] of Object.entries(scopes as Record<string, unknown>)) {
    const scope = row as Partial<SchemaCatalogScope>;
    if (typeof scope.path !== "string" || typeof scope.formats !== "object" || scope.formats === null || typeof scope.exports !== "object" || scope.exports === null || Array.isArray(scope.exports) || !Array.isArray(scope.dependsOn)) {
      found.push(schemaDiagnostic("schema-catalog-malformed", `catalog scope ${JSON.stringify(id)} must carry \`path\`, \`formats\`, an \`exports\` object of ExportId → { file, facet } and \`dependsOn\``, { scope: id, path: resolution.catalogPath }));
      continue;
    }
    // 🧭️A row that survives the shape check is the only kind anything downstream may read: a malformed
    // row let through would be re-reported once per export and once per format, turning one catalog
    // defect into thousands of derived ones.
    rows[id] = scope as SchemaCatalogScope;
    for (const [exported, row] of Object.entries(scope.exports)) {
      const file = (row as Partial<SchemaCatalogExport> | null)?.file;
      const facet = (row as Partial<SchemaCatalogExport> | null)?.facet;
      if (typeof file !== "string" || file.length === 0 || file.startsWith("/") || file.includes("..") || typeof facet !== "string" || facet.length === 0) found.push(schemaDiagnostic("schema-catalog-malformed", `scope ${JSON.stringify(id)} export ${JSON.stringify(exported)} must name a \`file\` relative to ${scope.path} and the \`facet\` that carries it`, { scope: id, export: exported, path: resolution.catalogPath }));
    }
    const count = declarations.get(id) ?? 1;
    if (count > 1) found.push(schemaDiagnostic("schema-scope-ambiguous", `the catalog declares the scope id ${JSON.stringify(id)} ${count} times`, { scope: id, path: resolution.catalogPath }));
    const priorId = byPath.get(scope.path);
    if (priorId !== undefined) found.push(schemaDiagnostic("schema-scope-ambiguous", `scope ids ${JSON.stringify(priorId)} and ${JSON.stringify(id)} both own ${scope.path}`, { scope: id, path: scope.path }));
    else byPath.set(scope.path, id);
    for (const [format, file] of Object.entries(scope.formats)) {
      if (typeof file !== "string" || file.length === 0 || file.startsWith("/") || file.includes("..")) found.push(schemaDiagnostic("schema-catalog-malformed", `scope ${JSON.stringify(id)} format ${format} must name a file relative to ${scope.path}`, { scope: id, format, path: resolution.catalogPath }));
    }
  }
  return { catalog: { scopes: rows }, diagnostics: found };
}

const SCHEMA_URI_RE = /^schema:\/\/([a-z0-9]+(?:-[a-z0-9]+)*(?:\.[a-z0-9]+(?:-[a-z0-9]+)*)*)\/([A-Z][A-Za-z0-9]*)(?:\?format=([a-z]+))?$/u;

/**
 * 🔗️ Splits a `schema://<scope id>/<ExportId>[?format=<format id>]` URI, or refuses it. Nothing else
 * is a schema URI. The optional format id is one of `schemaExportResolution.formatIds` and selects a
 * non-normative implementation of the same export, such as a scope's GraphQL SDL.
 *
 * The taxonomy owns the grammar (`schemaExportResolution.uriPattern`), so a repository that tightens
 * or widens the id shape does it in one place; the constant here is what an unconfigured tree gets.
 */
export function parseSchemaUri(uri: string, pattern: string | null = null): { scope: string; export: string; format?: string } | null {
  const match = uri.match(pattern === null ? SCHEMA_URI_RE : new RegExp(pattern, "u"));
  if (match === null) return null;
  const scope = match.groups?.scope ?? match[1];
  const exported = match.groups?.export ?? match[2];
  const format = match.groups?.format ?? match[3];
  return scope === undefined || exported === undefined ? null : { scope, export: exported, ...(format === undefined ? {} : { format }) };
}

/** 🔣️ The catalog format key (`🔗️graphql`) a URI format id (`graphql`) names, or `null` when the taxonomy declares no such format. */
export function schemaFormatKey(repoRoot: string, formatId: string): string | null {
  const formatIds = (rawTaxonomy(repoRoot).schemaExportResolution as { formatIds?: Record<string, string> } | undefined)?.formatIds ?? {};
  return Object.entries(formatIds).find(([, id]) => id === formatId)?.[0] ?? null;
}

/**
 * 🎯️ Resolves `schema://<scope>/<Export>` to the file one format of that export is written in.
 *
 * Each failure gets its OWN code — malformed URI, unknown scope, unknown export, unavailable format,
 * missing file — because "it did not resolve" is not actionable and every one of those five is.
 */
export function resolveSchemaExport(repoRoot: string, uri: string, format?: string): { resolved: ResolvedSchemaExport | null; diagnostics: readonly SchemaDiagnostic[] } {
  const { resolution, diagnostics: undeclared } = schemaExportResolution(repoRoot);
  const parsed = parseSchemaUri(uri, resolution?.uriPattern ?? null);
  if (parsed === null) return { resolved: null, diagnostics: [schemaDiagnostic("schema-uri-malformed", `${JSON.stringify(uri)} is not a schema://<scope id>/<ExportId> reference`, { path: null })] };
  if (resolution === null) return { resolved: null, diagnostics: undeclared };
  const { catalog, diagnostics } = readSchemaCatalog(repoRoot);
  if (catalog === null) return { resolved: null, diagnostics };
  const scope = catalog.scopes[parsed.scope];
  if (scope === undefined) return { resolved: null, diagnostics: [...diagnostics, schemaDiagnostic("schema-scope-unknown", `the catalog declares no scope ${JSON.stringify(parsed.scope)}`, { scope: parsed.scope, export: parsed.export })] };
  const exported = scope.exports[parsed.export];
  if (exported === undefined) {
    const names = Object.keys(scope.exports);
    return { resolved: null, diagnostics: [...diagnostics, schemaDiagnostic("schema-export-unknown", `scope ${JSON.stringify(parsed.scope)} exports ${names.length === 0 ? "nothing" : names.join(", ")} — not ${JSON.stringify(parsed.export)}`, { scope: parsed.scope, export: parsed.export })] };
  }
  const requested = format ?? (parsed.format === undefined ? normativeSchemaFormat(repoRoot) : schemaFormatKey(repoRoot, parsed.format));
  if (requested === null) return { resolved: null, diagnostics: [...diagnostics, schemaDiagnostic("schema-format-unavailable", `${JSON.stringify(uri)} names format ${JSON.stringify(parsed.format)}, which schemaExportResolution.formatIds does not declare`, { scope: parsed.scope, export: parsed.export, format: parsed.format ?? null })] };
  const chosen = requested;
  const relative = scope.formats[chosen];
  if (typeof relative !== "string") return { resolved: null, diagnostics: [...diagnostics, schemaDiagnostic("schema-format-unavailable", `scope ${JSON.stringify(parsed.scope)} declares no ${chosen} implementation`, { scope: parsed.scope, export: parsed.export, format: chosen })] };
  const file = schemaExportFile(scope, exported, chosen, normativeSchemaFormat(repoRoot));
  if (!existsSync(join(repoRoot, file))) return { resolved: null, diagnostics: [...diagnostics, schemaDiagnostic("schema-file-missing", `the catalog names ${file} for ${chosen}, and that file does not exist`, { scope: parsed.scope, export: parsed.export, format: chosen, path: file })] };
  // 🧫️A contract may never be answered out of a fixture or test tree. Resolution reaching one means
  // the catalog itself was built over an ineligible owner, and every consumer of it is reading an
  // example as if it were the contract.
  if (isFixtureOwnedPath(repoRoot, file)) return { resolved: null, diagnostics: [...diagnostics, schemaDiagnostic("schema-fixture-local-schema-fallback", `${file} lies under a fixture or test directory, so it cannot answer ${uri}`, { scope: parsed.scope, export: parsed.export, format: chosen, path: file })] };
  return { resolved: { uri, scope: parsed.scope, export: parsed.export, format: chosen, path: file, digest: fileDigest(join(repoRoot, file)) }, diagnostics };
}

/**
 * 🧫️ Whether a repository-relative path lives inside a test or fixture COLLECTION.
 *
 * The patterns are the taxonomy's (`schemaScopeOwnerLevels.fixtureOwnerPathPatterns`), so what counts as
 * a collection is declared once. The one thing decided here is the module-member carve-out, and its
 * directory name is the taxonomy's too (`schemaScopeOwnerLevels.moduleMemberDirName`): a module member is
 * a module whatever emoji its name starts with, and without the carve-out the repository test platform —
 * `🔨️modules/🧪️test`, which owns the test-protocol contract every host reads — would be classified as a
 * collection of its own examples.
 */
export function isFixtureOwnedPath(repoRoot: string, rel: string): boolean {
  // 🧭️Only the SELF patterns (`**/🧪️*`) name a collection ROOT; the `/**` siblings name its descendants
  // and are redundant here, because every ancestor of the path is tested. Keeping them would make
  // `🔨️modules/🧪️test/🧬️schema` a collection by virtue of an ancestor the carve-out had just excused.
  const declared = schemaScopeOwnerLevels(repoRoot);
  const patterns = (declared?.fixtureOwnerPathPatterns ?? []).filter((pattern) => !pattern.endsWith("/**"));
  const moduleMemberDirName = declared?.moduleMemberDirName;
  const segments = rel.split("/");
  for (let index = 0; index < segments.length; index += 1) {
    if (isModuleMemberSegment(segments, index, moduleMemberDirName)) continue;
    const prefix = segments.slice(0, index + 1).join("/");
    if (patterns.some((pattern) => matchesTaxonomyPathPattern(pattern, prefix))) return true;
  }
  return false;
}

/**
 * 🔨️ Whether the segment at `index` is a member of the taxonomy's module-member directory.
 *
 * This decides COLLECTION membership only — "does this path lie inside a test or fixture collection" —
 * never ownership, which `schemaScopeEligibility` reads from the taxonomy. A module member is a module
 * whatever emoji its name starts with; `🧪️tests/` and `./🧪️tests/🧫️fixtures` under it are collections. The
 * directory name is `schemaScopeOwnerLevels.moduleMemberDirName`; a taxonomy that declares none carves
 * nothing out, because a carve-out nobody declared is a rule nobody can read.
 */
function isModuleMemberSegment(segments: readonly string[], index: number, moduleMemberDirName: string | undefined): boolean {
  return moduleMemberDirName !== undefined && index > 0 && segments[index - 1] === moduleMemberDirName;
}

/** 🏛️ One eligibility verdict for a candidate scope owner directory. */
export type SchemaScopeEligibility = Readonly<{ path: string; eligible: boolean; level: string | null; reason: string }>;

/** 🏛️ The declared owner levels and exclusions, exactly as the taxonomy states them. */
export type SchemaScopeOwnerLevels = Readonly<{ facetDirName: string; levels: Readonly<Record<string, { pathPatterns: readonly string[]; reason: string }>>; excludedOwnerPathPatterns: readonly string[]; fixtureOwnerPathPatterns?: readonly string[]; moduleMemberDirName?: string }>;

export function schemaScopeOwnerLevels(repoRoot: string): SchemaScopeOwnerLevels | null {
  const declared = rawTaxonomy(repoRoot).schemaScopeOwnerLevels as SchemaScopeOwnerLevels | undefined;
  if (declared === undefined || typeof declared.levels !== "object" || !Array.isArray(declared.excludedOwnerPathPatterns)) return null;
  return declared;
}

/**
 * 🔎️ Matches one repository-relative path against a taxonomy path pattern.
 *
 * `**` spans any number of segments, `*` spans exactly one and never a separator. It is written here
 * rather than taken from a glob library because a runtime dependency on one is forbidden — and
 * because a pattern language nobody can read is a policy nobody can audit.
 */
export function matchesTaxonomyPathPattern(pattern: string, path: string): boolean {
  const escaped = pattern.replace(/[.+^${}()|[\]\\]/gu, "\\$&");
  const expression = escaped
    .split("**/")
    .join("\u0000")
    .replace(/\*\*/gu, "\u0001")
    .replace(/\*/gu, "[^/]*")
    .split("\u0000")
    .join("(?:.*/)?")
    .split("\u0001")
    .join(".*");
  return new RegExp(`^${expression}$`, "u").test(path);
}

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
  // 🧭️An INSTANCE names the schema that validates it — a relative path to a `🧬️schema/` module. A
  // DEFINITION names a meta-schema on json-schema.org. That one distinction is what keeps every
  // `$schema`-carrying config file out of these findings.
  return typeof dialect === "string" && /^https?:\/\/json-schema\.org\//u.test(dialect);
}

/** 🧬️ Whether a schema definition declares the one dialect the taxonomy names. */
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
 * 🧬️ Reads a file only far enough to decide whether it DEFINES a schema.
 *
 * A definition necessarily names a json-schema.org meta-schema, so the substring test settles the
 * overwhelming majority of a monorepo's JSON without parsing it. The parse still decides — the
 * prefilter can only produce candidates, never verdicts.
 */
function readSchemaDefinition(abs: string): unknown | undefined {
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

const submodulePathCache = new Map<string, ReadonlySet<string>>();

/**
 * 🧩️ The repository-relative paths `.gitmodules` declares as submodules, parsed once per root.
 *
 * A submodule is a FOREIGN repository: its files are authored, versioned and reviewed elsewhere, and
 * this workspace tracks one gitlink for the whole tree. Auditing them reports findings nobody here
 * can act on, against documents no owner in this tree could move. The declaration is read from
 * `.gitmodules` rather than from `git submodule status`, so the rule holds in a devcontainer, on a
 * bare checkout and with no `git` binary at all; paths are NFC-normalised because the file is written
 * by git while directory entries come from the filesystem, and macOS hands those back decomposed.
 */
export function submodulePaths(repoRoot: string): ReadonlySet<string> {
  const cached = submodulePathCache.get(repoRoot);
  if (cached !== undefined) return cached;
  let text = "";
  try {
    text = readFileSync(join(repoRoot, ".gitmodules"), "utf8");
  } catch {
    /* 🧭️A repository with no submodules declares none; an absent file is that statement. */
  }
  const declared = new Set<string>();
  for (const line of text.split(/\r?\n/u)) {
    const match = /^\s*path\s*=\s*(.+?)\s*$/u.exec(line);
    if (match !== null) declared.add(match[1]!.normalize("NFC"));
  }
  submodulePathCache.set(repoRoot, declared);
  return declared;
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
    // 🧭️The outermost forbidden ANCESTOR directory is the defect; naming the deeper one would report
    // the same violation at the wrong level.
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
      // 🧬️Inside a forbidden directory the directory finding already says the whole thing must go;
      // repeating it once per file inside would report one defect as many.
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

/** 🧬️ Every `🧬️schema/` module directory the FILE WALK reaches that carries a schema definition. */
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
  // 🧫️A module inside a `🧪️`/`🧫️` collection is one the catalog is RIGHT not to name, and
  // `schema-fixture-defines-schema` already reports it. Counting it here too would report one defect
  // twice and make the disagreement number a claim about the catalog that is not true.
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
    // 🧬️A mutation leaf's own `🧬️schema/` sits INSIDE its owner's `🧬️schema/🧬️mutations/` — the leaf
    // is the declared authority for its payload, so the leaf path is what is judged, not its ancestor.
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
function inertSchemaDataPaths(repoRoot: string, files: readonly string[]): Set<string> {
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

/**
 * 🎯️ The schema of one named export inside a module document.
 *
 * A module either lists its exports under `$defs`, or IS a single export and says so with the root
 * keyword the taxonomy declares (`schemaExportResolution.rootExportKeyword`, today `title`). Both are
 * declarations; neither is a guess, and 1 691 of the 460 catalogued scopes use the second form.
 */
export function moduleExportSchema(repoRoot: string, document: unknown, exportId: string): unknown | undefined {
  if (document === null || typeof document !== "object" || Array.isArray(document)) return undefined;
  const defs = ((document as { $defs?: unknown }).$defs ?? {}) as Record<string, unknown>;
  if (Object.hasOwn(defs, exportId)) return defs[exportId];
  const keyword = (rawTaxonomy(repoRoot).schemaExportResolution as { rootExportKeyword?: unknown } | undefined)?.rootExportKeyword;
  if (typeof keyword !== "string") return undefined;
  return (document as Record<string, unknown>)[keyword] === exportId ? document : undefined;
}

/** 📚️ The repository-relative file one scope keeps a format in, or null when it declares none. */
export function schemaScopeFile(scope: SchemaCatalogScope, format: string): string | null {
  const relative = scope.formats[format];
  return typeof relative === "string" && relative.length > 0 ? `${scope.path}/${relative}` : null;
}

/**
 * 🎯️ The repository-relative file one EXPORT of a scope is written in, for one format.
 *
 * The catalog names the export's normative file outright. Every other format is its sibling in the
 * same facet directory — the facet is what decides which document carries the export, and a format is
 * only the filename inside it. Nothing is searched for: both halves are declared.
 */
export function schemaExportFile(scope: SchemaCatalogScope, exported: SchemaCatalogExport, format: string, normativeFormat: string): string {
  if (format === normativeFormat) return `${scope.path}/${exported.file}`;
  const relative = scope.formats[format]!;
  const facetDir = exported.file.includes("/") ? exported.file.slice(0, exported.file.lastIndexOf("/")) : "";
  const filename = relative.includes("/") ? relative.slice(relative.lastIndexOf("/") + 1) : relative;
  return facetDir === "" ? `${scope.path}/${filename}` : `${scope.path}/${facetDir}/${filename}`;
}

/** 🔣️ The format ids the taxonomy declares, the only vocabulary an `x-semio-formats` annotation may use. */
export function declaredSchemaFormatIds(repoRoot: string): ReadonlySet<string> {
  const formats = rawTaxonomy(repoRoot).schemaFormats;
  if (formats === null || typeof formats !== "object" || Array.isArray(formats)) throw new Error(`${TAXONOMY_REL_PATH} declares no \`schemaFormats\``);
  return new Set(Object.keys(formats as Record<string, unknown>));
}

/** 🏷️ The keyword an export annotates its restricted format support with, per execution contract §B. */
export const SCHEMA_EXPORT_FORMATS_KEYWORD = "x-semio-formats";

/**
 * 🏷️ The formats one export claims, or `null` when it claims all of them.
 *
 * An export with no annotation must exist in EVERY format its scope provides. An annotated export
 * must exist in EXACTLY the formats it names — a law or contract export that is only ever validated
 * says so, and is then held to that statement in both directions: a format it declared and does not
 * implement is incomplete, and a format it implements without declaring makes the annotation a
 * comment rather than a contract.
 */
export function restrictedExportFormats(repoRoot: string, definition: unknown): { formats: readonly string[] | null; invalid: string | null } {
  if (definition === null || typeof definition !== "object" || Array.isArray(definition)) return { formats: null, invalid: null };
  const declared = (definition as Record<string, unknown>)[SCHEMA_EXPORT_FORMATS_KEYWORD];
  if (declared === undefined) return { formats: null, invalid: null };
  if (!Array.isArray(declared) || declared.length === 0) return { formats: null, invalid: `${SCHEMA_EXPORT_FORMATS_KEYWORD} must be a non-empty array of taxonomy format ids` };
  const vocabulary = declaredSchemaFormatIds(repoRoot);
  const unknown = declared.filter((entry) => typeof entry !== "string" || !vocabulary.has(entry));
  if (unknown.length > 0) return { formats: null, invalid: `${SCHEMA_EXPORT_FORMATS_KEYWORD} names ${unknown.map((entry) => JSON.stringify(entry)).join(", ")}, which the taxonomy does not declare as a schema format` };
  return { formats: declared as readonly string[], invalid: null };
}

/**
 * ✅️ Completeness: every export the JSON Schema module declares exists, under the same name, in
 * every format the module actually implements — or in exactly the formats it declared with
 * `x-semio-formats`.
 *
 * A format file that declares four of five exports is worse than an absent one: consumers of the
 * fifth get a link error at build time in one language and silence in another.
 *
 * "Exists in a format" is what that format's presence rule says it is (execution contract §A):
 * GraphQL admits any of `GRAPHQL_EXPORT_KEYWORDS`, and TypeScript requires the exported type AND
 * `parse<Export>()` — the missing entry point is `schema-export-parser-missing`, kept apart from
 * `schema-export-incomplete` so "the type is not there" and "the type is there and unusable" are
 * two counts an owner can act on separately.
 */
export function schemaExportCompletenessDiagnostics(repoRoot: string, scopeId: string, scope: SchemaCatalogScope): SchemaDiagnostic[] {
  const found: SchemaDiagnostic[] = [];
  const normativeFormat = normativeSchemaFormat(repoRoot);
  const jsonFile = schemaScopeFile(scope, normativeFormat);
  if (jsonFile === null) return [schemaDiagnostic("schema-format-unavailable", `scope ${JSON.stringify(scopeId)} declares no ${normativeFormat} implementation, so its exports have no normative definition`, { scope: scopeId })];
  const document = readJson(join(repoRoot, jsonFile));
  if (document === undefined) return [schemaDiagnostic("schema-catalog-malformed", `${jsonFile} is not readable JSON`, { scope: scopeId, path: jsonFile })];
  if (typeof (document as { $id?: unknown }).$id !== "string") found.push(schemaDiagnostic("schema-module-id-missing", `${jsonFile} declares no $id, so nothing can $ref it across scopes`, { scope: scopeId, path: jsonFile }));
  if (!isDeclaredDialectDefinition(repoRoot, document)) found.push(schemaDiagnostic("schema-dialect-not-draft-07", `${jsonFile} does not declare ${schemaJsonDialect(repoRoot)}`, { scope: scopeId, path: jsonFile }));
  const normative = normativeFormat;
  const declared = new Map<string, unknown>();
  for (const [exported, row] of Object.entries(scope.exports)) {
    // 📚️The export's OWN file answers, not the module root: a facet child carries its own `$defs`.
    const exportFile = schemaExportFile(scope, row, normative, normative);
    const carrier = exportFile === jsonFile ? document : readJson(join(repoRoot, exportFile));
    if (carrier === undefined) {
      found.push(schemaDiagnostic("schema-file-missing", `the catalog names ${exportFile} for export ${JSON.stringify(exported)}, and that file is not readable JSON`, { scope: scopeId, export: exported, path: exportFile }));
      continue;
    }
    const definition = moduleExportSchema(repoRoot, carrier, exported);
    if (definition === undefined) found.push(schemaDiagnostic("schema-export-unknown", `the catalog names ${exportFile} for export ${JSON.stringify(exported)}, and that document declares it neither in $defs nor as its root export`, { scope: scopeId, export: exported, path: exportFile }));
    else declared.set(exported, definition);
  }
  for (const [exported, row] of Object.entries(scope.exports)) {
    if (!/^[A-Z][A-Za-z0-9]*$/u.test(exported) || !declared.has(exported)) continue;
    const { formats: restricted, invalid } = restrictedExportFormats(repoRoot, declared.get(exported));
    if (invalid !== null) {
      found.push(schemaDiagnostic("schema-export-formats-annotation-invalid", `export ${JSON.stringify(exported)} ${invalid}`, { scope: scopeId, export: exported, path: schemaExportFile(scope, row, normative, normative) }));
      continue;
    }
    // 🏷️A declared format the scope does not implement at all cannot carry the export, so the
    // annotation is a promise the module never kept — reported against the format, not the file.
    if (restricted !== null) for (const format of restricted) if (!Object.hasOwn(scope.formats, format)) found.push(schemaDiagnostic("schema-export-incomplete", `export ${JSON.stringify(exported)} declares ${format} support, and scope ${JSON.stringify(scopeId)} implements no ${format} file`, { scope: scopeId, export: exported, format }));
    for (const format of Object.keys(scope.formats)) {
      const supported = restricted === null || restricted.includes(format);
      // 🧬️The normative format is where the export was READ from, so its presence is already
      // established; the only question left is whether the annotation admitted to it.
      if (format === normative) {
        if (!supported) found.push(schemaDiagnostic("schema-export-format-undeclared", `export ${JSON.stringify(exported)} is defined in ${format} and its ${SCHEMA_EXPORT_FORMATS_KEYWORD} does not name ${format}`, { scope: scopeId, export: exported, format, path: schemaExportFile(scope, row, normative, normative) }));
        continue;
      }
      const file = schemaExportFile(scope, row, format, normative);
      const abs = join(repoRoot, file);
      if (!existsSync(abs)) {
        if (supported) found.push(schemaDiagnostic("schema-file-missing", `the catalog names ${file} for ${format}, and that file does not exist`, { scope: scopeId, format, path: file }));
        continue;
      }
      const source = readFileSync(abs, "utf8");
      const present = declaresSchemaExport(format, source, exported);
      if (supported && !present) found.push(schemaDiagnostic("schema-export-incomplete", `${file} declares no ${format} entity named ${exported}`, { scope: scopeId, export: exported, format, path: file }));
      // 🚪️The entry point is asked for only where the entity exists: a format missing the export
      // entirely is one defect, already named, and reporting it twice would make the count a lie.
      if (supported && present && !declaresSchemaExportParser(format, source, exported)) found.push(schemaDiagnostic("schema-export-parser-missing", `${file} declares ${exported} and no parse${exported}(), so the export has a type and no entry point`, { scope: scopeId, export: exported, format, path: file }));
      if (!supported && present) found.push(schemaDiagnostic("schema-export-format-undeclared", `${file} declares a ${format} entity named ${exported}, and the export's ${SCHEMA_EXPORT_FORMATS_KEYWORD} restricts it to ${restricted!.join(", ")}`, { scope: scopeId, export: exported, format, path: file }));
    }
  }
  return found;
}

/** 🔗️ The GraphQL type-system keywords an export may be declared with (execution contract §A, row 98). */
export const GRAPHQL_EXPORT_KEYWORDS = ["type", "input", "enum", "interface", "union", "scalar"] as const;

/**
 * 🔎️ Whether one format's source declares a same-named entity for an export.
 *
 * GraphQL admits any of `GRAPHQL_EXPORT_KEYWORDS`: an export whose JSON Schema is an `enum` or a
 * `oneOf` has no honest `type` form, and forcing one would make the GraphQL document a lie about the
 * contract. TypeScript answers only for the TYPE half here; its `parse<Export>()` half is
 * `declaresSchemaExportParser`, so the two failures stay separately reportable. Rust admits a
 * definition OR a per-name `pub use` re-export (`declaresRustReExport`): a scope that owns the type
 * elsewhere and republishes it by name has declared it, and demanding a second definition would
 * mandate the restatement contract §A forbids.
 */
export function declaresSchemaExport(format: string, source: string, exported: string): boolean {
  const name = exported.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
  switch (format) {
    case "🛰️protobuf":
      return new RegExp(`^\\s*message\\s+${name}\\b`, "mu").test(source);
    case "🔗️graphql":
      return new RegExp(`^\\s*(?:${GRAPHQL_EXPORT_KEYWORDS.join("|")})\\s+${name}\\b`, "mu").test(source);
    case "🦀️rust":
      return new RegExp(`^\\s*pub\\s+(?:struct|enum|type)\\s+${name}\\b`, "mu").test(source) || declaresRustReExport(source, name);
    case "🟦️typescript":
      return new RegExp(`^\\s*export\\s+(?:interface|type|const|class)\\s+${name}\\b`, "mu").test(source);
    default:
      return false;
  }
}

/**
 * 🦀️ Whether a Rust source PUBLISHES one export by name through a re-export.
 *
 * Execution contract §A (ledger row 148): a Rust re-export declares an export when it names it, one
 * name per statement — `pub use <path>::<Export>;`, or `pub use <path>::<Other> as <Export>;`, which
 * publishes the same name through a rename. A GROUPED re-export (`pub use a::{X, Y};`) and a GLOB
 * (`pub use a::*;`) declare nothing here: neither one names `<Export>` at the module's surface, so a
 * reader of the module — and this rule is a reader — cannot tell from the statement whether the
 * export is published at all, which is exactly the ambiguity a declared contract exists to remove.
 * `pub(crate) use` is not `pub use`: a re-export the crate keeps to itself publishes nothing.
 *
 * @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/SCOPE-OWNED-SCHEMA-CONTRACTS/📋️execution-contract.md
 */
function declaresRustReExport(source: string, name: string): boolean {
  const path = "[A-Za-z_][A-Za-z0-9_]*(?:\\s*::\\s*[A-Za-z_][A-Za-z0-9_]*)*";
  return new RegExp(`^\\s*pub\\s+use\\s+(?![^;\\n]*[{*])${path}\\s*::\\s*(?:${name}|[A-Za-z_][A-Za-z0-9_]*\\s+as\\s+${name})\\s*;`, "mu").test(source);
}

/**
 * 🚪️ Whether one format's source carries the entry point that format's completeness ALSO requires.
 *
 * Only TypeScript has one: execution contract §A defines a TS export as "an exported
 * `interface`/`type` **and** `parse<Export>`". A type alone is erased at runtime, so a consumer
 * handed a `schema://` payload has no way to enter the contract — the declaration would be a
 * comment. Every other format's entity IS its entry point, so they answer `true` unconditionally.
 */
export function declaresSchemaExportParser(format: string, source: string, exported: string): boolean {
  if (format !== "🟦️typescript") return true;
  const name = exported.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
  return new RegExp(`^\\s*export\\s+(?:(?:async\\s+)?function|const|let|declare\\s+function)\\s+parse${name}\\b`, "mu").test(source);
}

/**
 * 🧬️ The `$id` a mutation leaf's schema module must carry, derived from its root module's `$id`.
 *
 * Grammar (execution contract §A, ledger row 78): `<root scope path>/mutation/<semanticKind>/schema.json`.
 * The SEMANTIC KIND keys the leaf, never its directory path — a two-segment leaf directory
 * (`🧬️mutations/<domain>/<verb>`, the glTF shape) is one leaf and gets one id, and deriving the id
 * from the path instead would collide the 229 stdio leaves onto 80 ids.
 */
export function mutationLeafSchemaId(repoRoot: string, rootModuleId: string, semanticKind: string): string {
  const declared = rawTaxonomy(repoRoot).schemaExportResolution as { mutationScopeSegment?: unknown; mutationLeafFacetFilename?: unknown } | undefined;
  const segment = declared?.mutationScopeSegment;
  const filename = declared?.mutationLeafFacetFilename;
  if (typeof segment !== "string" || segment.length === 0 || typeof filename !== "string" || filename.length === 0) throw new Error(`${TAXONOMY_REL_PATH} \`schemaExportResolution\` declares no \`mutationScopeSegment\`/\`mutationLeafFacetFilename\``);
  const at = rootModuleId.lastIndexOf("/");
  if (at < 0) throw new Error(`${JSON.stringify(rootModuleId)} is not a <scope path>/<facet>.json module id`);
  return `${rootModuleId.slice(0, at)}/${segment}/${semanticKind}/${filename}`;
}

/**
 * 📚️ Every normative document one scope owns: its module root and each export's carrier file.
 *
 * A scope is not one file. `formats` names the module root, while a facet child (`🔺️diff/`,
 * `📸️snapshot/`, `💡️inferences/`, `🧬️mutations/`) carries exports of the same scope, declares its
 * own `$id` and writes its own `$ref`s.
 */
export function scopeNormativeDocuments(scope: SchemaCatalogScope, normativeFormat: string): string[] {
  const documents = new Set<string>();
  const root = schemaScopeFile(scope, normativeFormat);
  if (root !== null) documents.add(root);
  for (const row of Object.values(scope.exports)) documents.add(schemaExportFile(scope, row, normativeFormat, normativeFormat));
  return [...documents].sort();
}

/** 🔗️ The member a JSON pointer addresses inside one document, or `undefined` when it addresses none. */
function jsonPointerTarget(document: unknown, pointer: string): unknown | undefined {
  if (pointer.length === 0) return document;
  if (!pointer.startsWith("/")) return undefined;
  let node: unknown = document;
  for (const raw of pointer.slice(1).split("/")) {
    const token = raw.replace(/~1/gu, "/").replace(/~0/gu, "~");
    if (Array.isArray(node)) {
      const index = Number(token);
      if (!Number.isInteger(index) || index < 0 || index >= node.length) return undefined;
      node = node[index];
      continue;
    }
    if (node === null || typeof node !== "object" || !Object.hasOwn(node as Record<string, unknown>, token)) return undefined;
    node = (node as Record<string, unknown>)[token];
  }
  return node;
}

/**
 * 🔗️ Judges one `$ref` written in one document of one scope.
 *
 * Three shapes are legal and they are legal for different reasons:
 *
 * 1. **Module-internal** (`#/definitions/<helper>`, `#/$defs/<Export>`, `#`) — a document addressing
 *    its own members. A helper is deliberately NOT an export, so requiring internal refs to be
 *    export-addressed would forbid the one place helpers are allowed to live. Only a pointer that
 *    lands on nothing is a finding here, and it is `schema-ref-broken-internal` — a defect of one
 *    document, owned by that document's partition, never the cross-document addressing refusal.
 * 2. **Cross-document, whole document** (`<target $id>`) — the aggregate-branch form: a mutation
 *    aggregate is a `$ref` union over leaves, and a leaf is its own scope whose entire document IS
 *    its payload contract.
 * 3. **Cross-document, one export** (`<target $id>#/$defs/<ExportId>`).
 *
 * Everything else — a relative file path, a cross-document pointer into another module's
 * `definitions` — is refused: a reference resolved by filesystem position or into a private helper
 * is a dependency nobody declared. (Execution contract §A, ledger row 77.)
 */
function schemaRefDiagnostics(scopeId: string, scope: SchemaCatalogScope, file: string, document: unknown, catalog: SchemaCatalog, scopeOfId: ReadonlyMap<string, string>): SchemaDiagnostic[] {
  const found: SchemaDiagnostic[] = [];
  for (const ref of jsonRefsIn(document)) {
    if (ref.startsWith("#")) {
      if (jsonPointerTarget(document, ref.slice(1)) === undefined) found.push(schemaDiagnostic("schema-ref-broken-internal", `${file} references ${ref}, and this document declares no such member`, { scope: scopeId, path: file }));
      continue;
    }
    const hash = ref.indexOf("#");
    const target = hash < 0 ? ref : ref.slice(0, hash);
    const fragment = hash < 0 ? "" : ref.slice(hash + 1);
    if (!/^https?:\/\//u.test(target)) {
      found.push(schemaDiagnostic("schema-ref-unresolved", `${file} references ${JSON.stringify(ref)}; a cross-document reference names the target module's $id, never a file path`, { scope: scopeId, path: file }));
      continue;
    }
    const targetScope = scopeOfId.get(target);
    if (targetScope === undefined) {
      found.push(schemaDiagnostic("schema-ref-unresolved", `${file} references ${target}, and no catalog scope declares that $id`, { scope: scopeId, path: file }));
      continue;
    }
    const addressed = fragment.length === 0 ? null : fragment.match(/^\/\$defs\/([A-Za-z0-9_]+)$/u);
    if (fragment.length > 0 && addressed === null) {
      found.push(schemaDiagnostic("schema-ref-unresolved", `${file} references ${JSON.stringify(ref)}; a cross-document reference addresses the whole module or one export, as <$id>#/$defs/<ExportId>`, { scope: scopeId, path: file }));
      continue;
    }
    const exported = addressed === null ? null : addressed[1]!;
    if (targetScope === scopeId) continue;
    if (exported !== null && !Object.hasOwn(catalog.scopes[targetScope]!.exports, exported)) {
      found.push(schemaDiagnostic("schema-export-unknown", `${file} references ${exported} from scope ${targetScope}, which does not export it`, { scope: scopeId, export: exported, path: file }));
      continue;
    }
    if (!scope.dependsOn.includes(targetScope)) found.push(schemaDiagnostic("schema-cross-scope-dependency-forbidden", `scope ${scopeId} references ${targetScope}${exported === null ? "" : `/${exported}`} without declaring ${targetScope} in dependsOn`, { scope: scopeId, export: exported, path: file }));
  }
  return found;
}

/**
 * 🔗️ Export resolution over the whole catalog: every `$ref` lands where it says it does, and a
 * cross-scope reference is only legal when the referring scope DECLARED that dependency.
 *
 * Every document of a scope is read, not only its module root: a facet child (`🔺️diff/`,
 * `🧬️mutations/`, …) carries exports of the same scope and writes its own refs.
 */
export function schemaResolutionDiagnostics(repoRoot: string, under = ""): SchemaDiagnostic[] {
  const { catalog, diagnostics } = readSchemaCatalog(repoRoot);
  if (catalog === null) return [...diagnostics];
  const found: SchemaDiagnostic[] = [...diagnostics];
  const selected = (scope: SchemaCatalogScope): boolean => under.length === 0 || scope.path === under || scope.path.startsWith(`${under}/`);
  const normative = normativeSchemaFormat(repoRoot);
  // 🪪️Every normative document of a scope declares its own `$id` — a facet child keeps the scope
  // path and varies only the facet filename — and a reference may address any of them. Indexing the
  // module root alone would report a legal `<facet $id>#/$defs/<Export>` as unresolved.
  const scopeOfId = new Map<string, string>();
  for (const [id, scope] of Object.entries(catalog.scopes)) {
    for (const file of scopeNormativeDocuments(scope, normative)) {
      const declaredId = (readJson(join(repoRoot, file)) as { $id?: unknown } | undefined)?.$id;
      if (typeof declaredId === "string" && !scopeOfId.has(declaredId)) scopeOfId.set(declaredId, id);
    }
  }
  for (const [id, scope] of Object.entries(catalog.scopes)) {
    // 📏️The `$id` index above is built over EVERY scope even under a filter: a subtree's refs
    // point out of it, and an index narrowed to the filter would report every one of them unresolved.
    if (!selected(scope)) continue;
    found.push(...schemaExportCompletenessDiagnostics(repoRoot, id, scope));
    for (const file of scopeNormativeDocuments(scope, normative)) {
      const document = readJson(join(repoRoot, file));
      if (document === undefined) continue;
      found.push(...schemaRefDiagnostics(id, scope, file, document, catalog, scopeOfId));
    }
  }
  return found;
}

/** 🔗️ Every `$ref` string anywhere in a JSON document. */
function jsonRefsIn(document: unknown): string[] {
  const refs: string[] = [];
  const visit = (node: unknown): void => {
    if (Array.isArray(node)) {
      for (const item of node) visit(item);
      return;
    }
    if (node === null || typeof node !== "object") return;
    for (const [key, value] of Object.entries(node)) {
      if (key === "$ref" && typeof value === "string") refs.push(value);
      else visit(value);
    }
  };
  visit(document);
  return refs;
}

/**
 * 🩺️ Every schema-contract invariant, over one subtree or the whole tree. One walk, one ordered list.
 *
 * `under` narrows BOTH measurements, never just the cheap one (ledger row 135): the placement walk is
 * rooted at the subtree and the catalog‑driven resolution half selects the scopes whose module path
 * lies inside it. Skipping resolution under a filter — what this did before — made every per‑partition
 * run report zero `schema-export-*` and `schema-ref-*` rows BY CONSTRUCTION, which reads as "this
 * subtree is clean" to everyone who does not know the gate declined to look.
 */
export function schemaContractDiagnostics(repoRoot: string, under = ""): SchemaDiagnostic[] {
  const files = schemaTreeFiles(repoRoot, under);
  const inert = inertSchemaDataPaths(repoRoot, files);
  return [...schemaPlacementDiagnostics(repoRoot, files, inert), ...schemaOwnerEligibilityDiagnostics(repoRoot, files), ...schemaFixtureIsolationDiagnostics(repoRoot, files, inert), ...schemaMeasurementDisagreementDiagnostics(repoRoot, files, under), ...schemaResolutionDiagnostics(repoRoot, under)];
}
//#endregion 🧬️SchemaContracts
