/** 📤️ The named schema exports of a scope: the export files, the declared formats, the completeness diagnostics and the declaration parsers of every format. */
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { TAXONOMY_REL_PATH } from "../../🗂️taxonomy/🟦️.ts";
import { normativeSchemaFormat, rawTaxonomy, type SchemaCatalogScope, schemaDiagnostic, type SchemaDiagnostic, schemaExportFile, schemaJsonDialect, schemaScopeFile } from "../🟦️.ts";
import { isDeclaredDialectDefinition, readJson } from "../🏗️scope-eligibility-and-tree/🟦️.ts";

//#region 🧬️SchemaContracts
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

/** 🔣️ The format ids the taxonomy declares, the only vocabulary an `x-semio-formats` annotation may use. */
export function declaredSchemaFormatIds(repoRoot: string): ReadonlySet<string> {
  const formats = rawTaxonomy(repoRoot).schemaFormats;
  if (formats === null || typeof formats !== "object" || Array.isArray(formats)) throw new Error(`${TAXONOMY_REL_PATH} declares no \`schemaFormats\``);
  return new Set(Object.keys(formats as Record<string, unknown>));
}

/** 🏷️ The keyword an export annotates its restricted format support with, per execution contract §B. */
export const SCHEMA_EXPORT_FORMATS_KEYWORD = "x-semio-formats";

/**
 * ⛔️ The formats one export claims, or `null` when it claims all of them.
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
    if (restricted !== null) for (const format of restricted) if (!Object.hasOwn(scope.formats, format)) found.push(schemaDiagnostic("schema-export-incomplete", `export ${JSON.stringify(exported)} declares ${format} support, and scope ${JSON.stringify(scopeId)} implements no ${format} file`, { scope: scopeId, export: exported, format }));
    for (const format of Object.keys(scope.formats)) {
      const supported = restricted === null || restricted.includes(format);
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
//#endregion 🧬️SchemaContracts
