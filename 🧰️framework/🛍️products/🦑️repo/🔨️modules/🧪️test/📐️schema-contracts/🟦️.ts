/** 📐️ Schema contracts
 *
 * Schema catalog, export resolution, placement rules and the schema diagnostics of the test platform: the contract vocabulary, the taxonomy and catalog readers, the export resolution and the owner levels live here.
 *
 * @see ./🏗️scope-eligibility-and-tree/🟦️.ts — scope eligibility, definitions and the schema tree
 * @see ./🩺️placement-diagnostics/🟦️.ts — placement, measurement, eligibility and isolation diagnostics
 * @see ./📤️export-completeness/🟦️.ts — named exports and their declaration parsers
 * @see ./🔗️reference-diagnostics/🟦️.ts — reference resolution and the whole-tree contract diagnostics
 */
//#region 🔌️Adapters
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { TAXONOMY_REL_PATH } from "../🗂️taxonomy/🟦️.ts";
import { fileDigest } from "../🔢️digests/🟦️.ts";

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

export function schemaDiagnostic(code: SchemaHarnessDiagnosticCode, detail: string, where: Partial<Omit<SchemaDiagnostic, "code" | "detail">> = {}): SchemaDiagnostic {
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
  const declarations = new Map<string, number>();
  for (const match of text.matchAll(/"([^"\\]+)"\s*:\s*\{/gu)) declarations.set(match[1]!, (declarations.get(match[1]!) ?? 0) + 1);
  for (const [id, row] of Object.entries(scopes as Record<string, unknown>)) {
    const scope = row as Partial<SchemaCatalogScope>;
    if (typeof scope.path !== "string" || typeof scope.formats !== "object" || scope.formats === null || typeof scope.exports !== "object" || scope.exports === null || Array.isArray(scope.exports) || !Array.isArray(scope.dependsOn)) {
      found.push(schemaDiagnostic("schema-catalog-malformed", `catalog scope ${JSON.stringify(id)} must carry \`path\`, \`formats\`, an \`exports\` object of ExportId → { file, facet } and \`dependsOn\``, { scope: id, path: resolution.catalogPath }));
      continue;
    }
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
  } catch {}
  const declared = new Set<string>();
  for (const line of text.split(/\r?\n/u)) {
    const match = /^\s*path\s*=\s*(.+?)\s*$/u.exec(line);
    if (match !== null) declared.add(match[1]!.normalize("NFC"));
  }
  submodulePathCache.set(repoRoot, declared);
  return declared;
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
//#endregion 🧬️SchemaContracts
