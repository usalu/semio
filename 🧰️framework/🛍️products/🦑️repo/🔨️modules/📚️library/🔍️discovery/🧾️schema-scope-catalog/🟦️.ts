import { inspectSchemaValidationReadIndex, type SchemaReadObserver, type SchemaValidationReadIndex } from "../🔗️schema-reads/🟦️.ts";
import { createHash } from "node:crypto";
import { readFileSync, readdirSync } from "node:fs";
import { readdir as readdirAsync } from "node:fs/promises";
import { join, resolve } from "node:path";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";
import { isDiscoverySkipDirectory } from "../🪧️owner-markers/🟦️.ts";
import { type TaxonomyPathMatcher, createTaxonomyPathMatcher } from "../🎯️taxonomy-path-matching/🟦️.ts";
import { canonicalPrimaryFilenameForKind, loadCatalogTaxonomy, loadTaxonomy } from "../🔐️taxonomy-authority/🟦️.ts";
import { resolveSchemaFacetKind } from "../⚖️fixed-contract-resolution/🟦️.ts";

/** 🧾️ One machine-readable finding of the schema scope tooling; `code` is stable, `path` repository-relative. */
export interface SchemaScopeDiagnostic {
  readonly code: string;
  readonly path: string;
  readonly detail: string;
}

/** 🔗️ One `$ref` occurrence, already classified against the catalog addressing rules. */
export interface SchemaScopeReference {
  readonly documentPath: string;
  readonly pointer: string;
  readonly ref: string;
  readonly targetScopeId: string | null;
  readonly targetExportId: string | null;
  readonly local: boolean;
}

/** 📄️ One JSON Schema document inside a schema module. */
export interface SchemaScopeDocument {
  readonly path: string;
  readonly scopeId: string | null;
  /** 🧩️ Segments of the `$id` before its facet filename; a facet document repeats its module's scope path and only appends facet segments. */
  readonly scopePath: readonly string[] | null;
  readonly id: string | null;
  /** 🏷️ Filename of the `$id` without its `.json` suffix; the facet this document carries within its module. */
  readonly facetFilename: string | null;
  readonly dialect: string | null;
  readonly exports: readonly string[];
  /** 🗂️ Export id to the format ids its `x-semio-formats` annotation restricts it to; absent means every provided format. */
  readonly exportFormats: Readonly<Record<string, readonly string[]>>;
  readonly references: readonly SchemaScopeReference[];
}

/** 🧬️ One `🧬️schema` module on disk together with the owner level that admits it. */
export interface SchemaScopeModule {
  readonly modulePath: string;
  readonly ownerPath: string;
  readonly level: string | null;
  readonly facetKindId: string;
  readonly scopeId: string | null;
  readonly formats: Readonly<Record<string, string>>;
  readonly documents: readonly SchemaScopeDocument[];
  readonly hashes: Readonly<Record<string, string>>;
}

/** 🏷️ Where one export is declared: the module-relative file that carries it and the facet that file is. */
export interface SchemaCatalogExport {
  /** 🗂️ Module-relative path including every nested facet directory (`🔺️diff/📝️text/🔣️.json`); join with the scope `path`. */
  readonly file: string;
  /** 🏷️ Facet id derived from the declaring document's `$id` (`schema`, `snapshot`, `snapshot/text`, …). */
  readonly facet: string;
}

/** 📇️ The generated scope catalog: one row per scope id, derived only from declared `$id`s. */
export interface SchemaCatalogScope {
  readonly path: string;
  readonly level: string;
  readonly facetKind: string;
  /** 🗂️ Format id to module-relative leaf filename; join with `path` for the repository-relative source. */
  readonly formats: Readonly<Record<string, string>>;
  readonly exports: Readonly<Record<string, SchemaCatalogExport>>;
  readonly dependsOn: readonly string[];
  readonly hashes: Readonly<Record<string, string>>;
}

/** 📇️ Provenance header plus the deterministic scope table. */
export interface SchemaCatalog {
  readonly contractId: string;
  readonly catalogVersion: 1;
  readonly generator: string;
  readonly taxonomySchemaVersion: number;
  readonly jsonSchemaDialect: string;
  readonly scopes: Readonly<Record<string, SchemaCatalogScope>>;
}

/** 🧾️ Everything one repository walk produced: modules, catalog and every finding. */
export interface SchemaScopeInventory {
  readonly modules: readonly SchemaScopeModule[];
  readonly catalog: SchemaCatalog;
  readonly diagnostics: readonly SchemaScopeDiagnostic[];
  readonly placement: readonly SchemaScopeDiagnostic[];
}

function byteSort(left: string, right: string): number {
  return Buffer.from(left).compare(Buffer.from(right));
}

/**
 * 🧫️ Whether a repository-relative path lies inside a test or fixture COLLECTION.
 *
 * Only the authored fixture and test self patterns name a collection root; every ancestor of the path is tested against
 * them, so the `/**` siblings would only restate the walk. The one carve-out is `🔨️modules/<m>`: a module
 * member is a module whatever emoji its name starts with, and without it the repository test platform
 * (`🔨️modules/🧪️test`, which owns the test-protocol contract every host reads) would be classified as a
 * collection of its own examples. A collection never owns a contract scope — and, since WP2c, a schema
 * contract facet inside one is always a `schema-fixture-defines-schema` finding. A case may declare
 * ordinary schema-shaped parser input as `inertSchemaData`, which never authorizes a contract facet.
 */
export function schemaScopeCollectionPath(path: string, taxonomy: Taxonomy = loadTaxonomy(), matcher: TaxonomyPathMatcher = createTaxonomyPathMatcher()): boolean {
  const levels = taxonomy.schemaScopeOwnerLevels;
  const roots = levels.fixtureOwnerPathPatterns.filter((pattern) => !pattern.endsWith("/**"));
  const segments = path.split("/");
  for (let index = 0; index < segments.length; index += 1) {
    if (index > 0 && segments[index - 1] === levels.moduleMemberDirName) continue;
    const prefix = segments.slice(0, index + 1).join("/");
    if (roots.some((pattern) => matcher.matches(prefix, pattern))) return true;
  }
  return false;
}

/** 🧱️ A contract facet below a collection boundary cannot become inert example data. */
export function schemaCollectionContractPath(path: string, taxonomy: Taxonomy = loadTaxonomy(), matcher: TaxonomyPathMatcher = createTaxonomyPathMatcher()): boolean {
  const segments = path.split("/");
  return segments.some((segment, index) => (segment === taxonomy.schemaScopeOwnerLevels.facetDirName || segment === "🛂️schema") && schemaScopeCollectionPath(segments.slice(0, index).join("/"), taxonomy, matcher))
    || schemaScopeCollectionPath(path, taxonomy, matcher) && /^(?:(?:📐️|🔣️|🧬️)?schema\.json|.+\.schema\.json)$/u.test(segments.at(-1)!);
}

/** 🧠️ Recognizes JSON Schema grammar without requiring an optional dialect marker. */
function schemaRootAuthority(value: unknown): boolean {
  if (!value || typeof value !== "object" || Array.isArray(value)) return false;
  const node = value as Record<string, unknown>;
  if (typeof node.$schema === "string" && /^https?:\/\/json-schema\.org\//u.test(node.$schema)) return true;
  const object = (child: unknown): child is Record<string, unknown> => !!child && typeof child === "object" && !Array.isArray(child);
  const types = new Set(["null", "boolean", "object", "array", "number", "string", "integer"]);
  if (node.type !== undefined && !(typeof node.type === "string" ? types.has(node.type) : Array.isArray(node.type) && node.type.length > 0 && node.type.every(type => typeof type === "string" && types.has(type)))) return false;
  if (node.properties !== undefined && !(object(node.properties) && Object.values(node.properties).every(child => typeof child === "boolean" || object(child) && (Object.keys(child).length === 0 || schemaRootAuthority(child) || Object.hasOwn(child, "const") || Array.isArray(child.enum))))) return false;
  if (node.required !== undefined && !(Array.isArray(node.required) && node.required.every(key => typeof key === "string"))) return false;
  const constraints = ["enum", "const", "minLength", "maxLength", "pattern", "format", "minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum", "multipleOf", "items", "additionalItems", "minItems", "maxItems", "uniqueItems", "contains", "required", "additionalProperties", "patternProperties", "propertyNames", "minProperties", "maxProperties", "dependencies", "not", "if", "then", "else"];
  const annotations = ["type", "title", "description", "default", "examples", "readOnly", "writeOnly", "$id", "$comment", "$schema"];
  const propertiesAuthority = object(node.properties) && (Object.keys(node.properties).length > 0 || node.type !== undefined || Object.keys(node).every(key => annotations.includes(key) || constraints.includes(key) || ["properties", "$defs", "definitions", "allOf", "anyOf", "oneOf"].includes(key) || key.startsWith("x-")));
  if (typeof node.$ref === "string" || propertiesAuthority || node.type !== undefined && (constraints.some(key => Object.hasOwn(node, key)) || Object.keys(node).every(key => annotations.includes(key)))) return true;
  return ["allOf", "anyOf", "oneOf"].some(key => Array.isArray(node[key]) && node[key].length > 0 && node[key].every(child => typeof child === "boolean" || schemaRootAuthority(child)))
    || ["$defs", "definitions"].some(key => object(node[key]) && Object.values(node[key]).some(schemaRootAuthority));
}

/** 🚫️ Owner shapes that can never carry a schema module, regardless of the level patterns. */
export function schemaScopeOwnerExcluded(ownerPath: string, taxonomy: Taxonomy = loadTaxonomy(), matcher: TaxonomyPathMatcher = createTaxonomyPathMatcher()): boolean {
  return taxonomy.schemaScopeOwnerLevels.excludedOwnerPathPatterns.some((pattern) => matcher.matches(ownerPath, pattern)) || schemaScopeCollectionPath(ownerPath, taxonomy, matcher);
}

/** 🧭️ The declared owner level a schema module sits at, or `null` when no level admits it. */
export function schemaScopeOwnerLevel(ownerPath: string, taxonomy: Taxonomy = loadTaxonomy(), matcher: TaxonomyPathMatcher = createTaxonomyPathMatcher()): string | null {
  if (schemaScopeOwnerExcluded(ownerPath, taxonomy, matcher)) return null;
  for (const [levelId, level] of Object.entries(taxonomy.schemaScopeOwnerLevels.levels)) if (level.pathPatterns.some((pattern) => matcher.matches(ownerPath, pattern))) return levelId;
  return null;
}

/**
 * 🪞️ The owner directory a schema module belongs to: its parent, except when the module directory is
 * itself a declared owner level.
 *
 * `🧰️framework/🔨️modules/🧬️schema` is the framework module whose own name is the facet directory name, so
 * its parent is the module container `🧰️framework/🔨️modules`, which is deliberately not an owner level.
 * A module that already sits at an eligible level owns itself; the alternative — admitting the container —
 * would make every framework module's parent an owner.
 */
export function schemaScopeModuleOwnerPath(modulePath: string, taxonomy: Taxonomy = loadTaxonomy(), matcher: TaxonomyPathMatcher = createTaxonomyPathMatcher()): string {
  const parent = modulePath.slice(0, Math.max(0, modulePath.lastIndexOf("/")));
  if (schemaScopeOwnerLevel(parent, taxonomy, matcher)) return parent;
  return schemaScopeOwnerLevel(modulePath, taxonomy, matcher) ? modulePath : parent;
}

/** 🧩️ Splits one document `$id` into its scope path segments and its facet filename, or `null` when it is not addressable. */
export function schemaDocumentIdParts(documentId: string, taxonomy: Taxonomy = loadTaxonomy()): { readonly scopePath: readonly string[]; readonly facet: string } | null {
  const resolution = taxonomy.schemaExportResolution;
  if (!documentId.startsWith(resolution.idBase)) return null;
  const segments = documentId.slice(resolution.idBase.length).split("/");
  const facet = segments.pop();
  if (!facet || !new RegExp(resolution.idFacetFilenamePattern, "u").test(facet) || segments.length === 0) return null;
  return new RegExp(resolution.scopeIdPattern, "u").test(segments.join(resolution.scopeIdSeparator)) ? { scopePath: segments, facet } : null;
}

/** 🆔️ Derives the dotted scope id from one document `$id`; never from the path the file happens to sit at. */
export function schemaScopeIdFromDocumentId(documentId: string, taxonomy: Taxonomy = loadTaxonomy()): string | null {
  const parts = schemaDocumentIdParts(documentId, taxonomy);
  return parts ? parts.scopePath.join(taxonomy.schemaExportResolution.scopeIdSeparator) : null;
}

/** 🔗️ Parses one `schema://<scope id>/<ExportId>` binding into its resolution key halves. */
export function parseSchemaExportUri(uri: string, taxonomy: Taxonomy = loadTaxonomy()): { readonly scopeId: string; readonly exportId: string } | null {
  const match = new RegExp(taxonomy.schemaExportResolution.uriPattern, "u").exec(uri);
  const groups = match?.groups;
  return groups?.scope && groups.export ? { scopeId: groups.scope, exportId: groups.export } : null;
}

interface SchemaWalkResult {
  readonly directories: readonly string[];
  readonly files: readonly string[];
}

const GIT_SUBMODULE_PATHS = new Map<string, readonly string[]>();

const GIT_SUBMODULE_PATH_DECLARATION = /^[\t ]*path[\t ]*=[\t ]*(.+?)[\t ]*$/gmu;

/**
 * 🧩️ The repository-relative paths `.gitmodules` declares, memoised per repository root.
 *
 * A submodule is a foreign repository tracked as one gitlink: its files are not this repository's sources
 * and every law that walks the tree would double-count them. Read from the declaration file, never from a
 * `git` subprocess, so the walk stays a pure filesystem read; NFC-normalised because the declaration and
 * the directory entry can spell the same emoji path in different Unicode forms.
 * @see https://git-scm.com/docs/gitmodules
 */
export function gitSubmodulePaths(repoRoot: string): readonly string[] {
  const root = resolve(repoRoot);
  const memoised = GIT_SUBMODULE_PATHS.get(root);
  if (memoised) return memoised;
  let declared: string[] = [];
  try {
    declared = [...readFileSync(join(root, ".gitmodules"), "utf8").matchAll(GIT_SUBMODULE_PATH_DECLARATION)].map((match) => match[1]!.replaceAll("\\", "/").replace(/^\.\//u, "").replace(/\/+$/u, "").normalize("NFC")).filter((path) => path.length > 0);
  } catch {
    declared = [];
  }
  const paths = [...new Set(declared)].sort(byteSort);
  GIT_SUBMODULE_PATHS.set(root, paths);
  return paths;
}

function walkRepositoryTree(repoRoot: string, taxonomy: Taxonomy): SchemaWalkResult {
  const directories: string[] = [];
  const files: string[] = [];
  const opaque = [...Object.values(taxonomy.pathExclusions).map((exclusion) => exclusion.path.replaceAll("\\", "/").replace(/^\.\//u, "").replace(/\/+$/u, "")), ...gitSubmodulePaths(repoRoot)].map((prefix) => prefix.normalize("NFC"));
  const visit = (relDir: string): void => {
    let entries: import("node:fs").Dirent<string>[];
    try {
      entries = readdirSync(join(repoRoot, relDir), { withFileTypes: true });
    } catch {
      return;
    }
    for (const entry of entries.sort((left, right) => byteSort(left.name, right.name))) {
      if (entry.isSymbolicLink()) continue;
      const rel = relDir ? `${relDir}/${entry.name}` : entry.name;
      const key = rel.normalize("NFC");
      if (opaque.some((prefix) => key === prefix || key.startsWith(`${prefix}/`))) continue;
      if (entry.isDirectory()) {
        if (isDiscoverySkipDirectory(entry.name) || entry.name.startsWith(".")) continue;
        directories.push(rel);
        visit(rel);
      } else if (entry.isFile()) files.push(rel);
    }
  };
  visit("");
  return { directories, files };
}

/** 🔗️ Inspects the original taxonomy walk with explicit unavailable evidence, progress and cancellation. */
export async function inventorySchemaValidationReads(repoRoot: string, taxonomy: Taxonomy = loadCatalogTaxonomy(), observer: SchemaReadObserver = {}): Promise<SchemaValidationReadIndex> {
  const root = resolve(repoRoot), files: string[] = [], unavailablePaths: string[] = [], opaque = [...Object.values(taxonomy.pathExclusions).map(exclusion => exclusion.path.replaceAll("\\", "/").replace(/^\.\//u, "").replace(/\/+$/u, "")), ...gitSubmodulePaths(root)].map(prefix => prefix.normalize("NFC"));
  let completed = 0;
  observer.onProgress?.({ phase: "walk", completed, total: null, path: null });
  const visit = async (directory: string): Promise<void> => {
    observer.checkCancellation?.();
    let entries: import("node:fs").Dirent<string>[];
    try { entries = await readdirAsync(join(root, directory), { withFileTypes: true }); } catch { unavailablePaths.push(directory); return; }
    for (const entry of entries.sort((left, right) => byteSort(left.name, right.name))) {
      observer.checkCancellation?.();
      const path = directory ? `${directory}/${entry.name}` : entry.name, key = path.normalize("NFC");
      if (entry.isSymbolicLink() || opaque.some(prefix => key === prefix || key.startsWith(`${prefix}/`))) continue;
      if (entry.isDirectory() && (isDiscoverySkipDirectory(entry.name) || entry.name.startsWith("."))) continue;
      observer.onProgress?.({ phase: "walk", completed: ++completed, total: null, path });
      if (entry.isDirectory()) await visit(path); else if (entry.isFile()) files.push(path);
    }
  };
  await visit("");
  const fileSet = new Set(files), index = await inspectSchemaValidationReadIndex(files, path => {
    if (!fileSet.has(path)) return undefined;
    try { return readFileSync(join(root, path), "utf8"); } catch { return undefined; }
  }, observer);
  return { reports: index.reports, unavailablePaths: [...new Set([...unavailablePaths, ...index.unavailablePaths])].sort(byteSort) };
}

/** 🔎️ First index of `sorted` whose byte order is not below `value` (prefix ranges without a linear scan). */
function lowerBound(sorted: readonly string[], value: string): number {
  let low = 0;
  let high = sorted.length;
  while (low < high) {
    const middle = (low + high) >> 1;
    if (byteSort(sorted[middle]!, value) < 0) low = middle + 1;
    else high = middle;
  }
  return low;
}

function jsonRefOccurrences(value: unknown, pointer: string, out: { pointer: string; ref: string }[]): void {
  if (Array.isArray(value)) {
    value.forEach((item, index) => jsonRefOccurrences(item, `${pointer}/${index}`, out));
    return;
  }
  if (!value || typeof value !== "object") return;
  for (const [key, child] of Object.entries(value as Record<string, unknown>)) {
    if (key === "$ref" && typeof child === "string") out.push({ pointer: `${pointer}/$ref`, ref: child });
    else jsonRefOccurrences(child, `${pointer}/${key.replaceAll("~", "~0").replaceAll("/", "~1")}`, out);
  }
}

/** 🚫️ Every pointer at which the document declares `enum: []`, a keyword no instance can ever satisfy. */
function jsonEmptyEnumPointers(value: unknown, pointer: string, out: string[]): void {
  if (Array.isArray(value)) {
    value.forEach((item, index) => jsonEmptyEnumPointers(item, `${pointer}/${index}`, out));
    return;
  }
  if (!value || typeof value !== "object") return;
  for (const [key, child] of Object.entries(value as Record<string, unknown>)) {
    const next = `${pointer}/${key.replaceAll("~", "~0").replaceAll("/", "~1")}`;
    if (key === "enum" && Array.isArray(child) && child.length === 0) out.push(next);
    else jsonEmptyEnumPointers(child, next, out);
  }
}

/** 🎯️ Whether one JSON pointer addresses an existing member of the document; the empty pointer is the document. */
function jsonPointerResolves(root: unknown, pointer: string): boolean {
  if (pointer === "") return true;
  if (!pointer.startsWith("/")) return false;
  let current: unknown = root;
  for (const raw of pointer.slice(1).split("/")) {
    const key = raw.replaceAll("~1", "/").replaceAll("~0", "~");
    if (Array.isArray(current)) {
      const index = Number(key);
      if (!/^(?:0|[1-9][0-9]*)$/u.test(key) || index >= current.length) return false;
      current = current[index];
      continue;
    }
    if (!current || typeof current !== "object" || !Object.hasOwn(current as object, key)) return false;
    current = (current as Record<string, unknown>)[key];
  }
  return true;
}

/** 🔎️ Whether one non-JSON format leaf declares the named export, by the declaration keywords of that language. */
function schemaFormatDeclaresExport(formatId: string, source: string, exportId: string): boolean {
  const name = exportId.replaceAll(/[\\^$.*+?()[\]{}|]/gu, "\\$&");
  const keywords: Readonly<Record<string, readonly string[]>> = {
    "🦀️rust": ["struct", "enum", "type", "trait", "fn"],
    "🟦️typescript": ["interface", "type", "class", "enum", "const", "function"],
    "🔗️graphql": ["type", "input", "enum", "union", "scalar", "interface"],
    "🛰️protobuf": ["message", "enum", "service"],
  };
  const declared = keywords[formatId];
  if (!declared) return false;
  if (new RegExp(`\\b(?:${declared.join("|")})\\s+${name}\\b`, "u").test(source)) return true;
  if (formatId === "🦀️rust") return new RegExp(`\\bpub\\s+use\\b[^;]*\\b${name}\\b`, "u").test(source);
  if (formatId !== "🟦️typescript") return false;
  return new RegExp(`\\bparse${name}\\b`, "u").test(source) || new RegExp(`\\bexport\\b(?:\\s+type)?\\s*\\{[^}]*\\b${name}\\b[^}]*\\}`, "u").test(source);
}

/** 🧪️ Distinguishes embedded schema authority from schema-shaped input specimens. */
function schemaEmbeddedExampleAuthority(subject: unknown): boolean {
  if (!subject || typeof subject !== "object") return false;
  if (Array.isArray(subject)) return subject.some(schemaEmbeddedExampleAuthority);
  return Object.entries(subject as Record<string, unknown>).some(([key, value]) => {
    if (/^(?:input|inputs|value|values|payload|request|document|documents|files|expect)$/u.test(key) || /^expected(?:$|[A-Z])/u.test(key)) return false;
    if (/(?:^|[A-Z])schema$/iu.test(key) && value && typeof value === "object" && !Array.isArray(value)) {
      const schema = value as Record<string, unknown>;
      if (["$schema", "$ref", "type", "properties", "items", "allOf", "anyOf", "oneOf", "enum", "const"].some((keyword) => Object.hasOwn(schema, keyword))) return true;
    }
    return schemaEmbeddedExampleAuthority(value);
  });
}

/** 🔬️ Recognizes immutable supplied authorities paired with unperformed trial observations. */
function schemaLiteralTrialEnvelope(value: unknown): boolean {
  if (!value || typeof value !== "object" || Array.isArray(value)) return false;
  const envelope = value as Record<string, unknown>, axes: Readonly<Record<string, string>> = { work: "maximumItems", copy: "maximumCopyBytes", capacity: "maximumCapacityBytes", release: "maximumReleaseBytes", depth: "maximumDepth" };
  const authorities = Object.values(envelope).filter((child): child is Record<string, number> => !!child && typeof child === "object" && !Array.isArray(child) && Object.values(axes).every(axis => Number.isInteger((child as Record<string, unknown>)[axis]) && Number((child as Record<string, unknown>)[axis]) >= 0));
  if (!authorities.length) return false;
  return Object.entries(envelope).some(([key, rows]) => Array.isArray(rows) && rows.length > 0 && (rows.every(row => row && typeof row === "object" && !Array.isArray(row) && !Object.hasOwn(row, "actual") && ["input", "inputs", "operation", "setup", "grant"].some(field => Object.hasOwn(row, field)) && Object.keys(row).some(field => /^(?:expect$|expected(?:$|[A-Z]))/u.test(field))) || /(?:^refusals$|Refusals$)/u.test(key) && rows.length > 1 && authorities.some(authority => rows.every(row => row && typeof row === "object" && !Array.isArray(row) && Object.hasOwn(axes, row.axis) && Number.isInteger(row.value) && row.value >= 0 && row.value <= authority[axes[row.axis]!]! && typeof row.kind === "string"))));
}

/** 🧫️ Recognizes test corpus contracts from their examples and expectations independently of titles. */
function schemaTestCorpusDefinition(exportId: string, subject: unknown, document: Record<string, unknown>): boolean {
  const producedRecord = /(?:Report|Result|Progress|Outcome|Diagnostic|Coverage)(?:V\d+)?$/u.test(exportId);
  const namedCorpus = /(?:Fixture|Corpus|Cases|(?:^|[A-Z])Vectors?(?:File)?$)/iu.test(exportId);
  const visited = [new Set<unknown>(), new Set<unknown>()];
  const inspect = (value: unknown, testCases = false): boolean => {
    const seen = visited[Number(testCases)]!;
    if (!value || typeof value !== "object" || seen.has(value)) return false;
    seen.add(value);
    if (Array.isArray(value)) return value.some((child) => inspect(child, testCases));
    const node = value as Record<string, unknown>;
    if (schemaLiteralTrialEnvelope(node.const)) return true;
    if (Array.isArray(node.const) && node.const.some(row => row && typeof row === "object" && Object.keys(row).some(key => /^(?:expect$|expected(?:$|[A-Z]))/u.test(key)))) return true;
    if (node.const && typeof node.const === "object" && !Array.isArray(node.const)) {
      const envelope = node.const as Record<string, unknown>;
      if (Object.entries(envelope).some(([key, rows]) => ["cases", "vectors", "scenarios", "laws"].includes(key) && Array.isArray(rows)) || Array.isArray(envelope.prefixTurns) && ["maximumItems", "maximumCopyBytes", "maximumDepth", "maximumTurns"].every(key => typeof envelope[key] === "number")) return true;
      if (Array.isArray(envelope.tiles) && envelope.tiles.length && envelope.tiles.every(row => row && typeof row === "object" && typeof row.file === "string" && /^[0-9a-f]{64}$/u.test(row.sha256) && typeof row.bytes === "number" && Array.isArray(row.layers) && row.layers.every((layer: Record<string, unknown>) => layer && typeof layer === "object" && typeof layer.features === "number" && Array.isArray(layer.geometryTypes) && layer.geometryTypes.every(count => typeof count === "number")))) return true;
    }
    const reference = typeof node.$ref === "string" ? node.$ref.match(/^#\/(\$defs|definitions)\/([^/]+)$/u) : null;
    if (reference && inspect((document[reference[1]!] as Record<string, unknown> | undefined)?.[reference[2]!], testCases)) return true;
    const properties = node.properties && typeof node.properties === "object" ? node.properties as Record<string, unknown> : {};
    const expectations = Object.keys(properties).filter((key) => /^(?:expect$|expected(?:$|[A-Z]))/u.test(key));
    const trialInput = ["input", "inputs", "setup", "operation", "sourceFixture"].some(key => Object.hasOwn(properties, key));
    const measuredVerification = !trialInput && ["fixture", "role", "path", "expected", "actual", "ok", "missing"].every(key => {
      const child = properties[key];
      return child && typeof child === "object" && !Array.isArray(child) && (child as Record<string, unknown>).type === (["ok", "missing"].includes(key) ? "boolean" : "string") && !Object.hasOwn(child, "const") && !Object.hasOwn(child, "enum");
    });
    if ((!producedRecord && !measuredVerification || testCases && trialInput) && expectations.length && (testCases || namedCorpus || ["grants", "hostile", "hostiles", "sourceFixture"].some((key) => Object.hasOwn(properties, key)))) return true;
    if (!producedRecord && testCases && ["accepted", "valid"].some((key) => Object.hasOwn(properties, key))) return true;
    const marker = properties.schema && typeof properties.schema === "object" ? (properties.schema as Record<string, unknown>).const : undefined;
    if (typeof marker === "string" && /(?:fixture|corpus|test[-.]cases)/iu.test(marker)) return true;
    const fixedProperties = Object.entries(properties).filter(([, child]) => child && typeof child === "object" && Object.hasOwn(child, "const"));
    if (schemaLiteralTrialEnvelope(Object.fromEntries(fixedProperties.map(([key, child]) => [key, (child as Record<string, unknown>).const])))) return true;
    const fixed = new Map(fixedProperties), fields = (child: unknown): Record<string, unknown> => child && typeof child === "object" ? ((child as Record<string, unknown>).properties as Record<string, unknown> | undefined) ?? {} : {};
    const fixedNumbers = (keys: readonly string[]): boolean => keys.every(key => typeof (fixed.get(key) as Record<string, unknown> | undefined)?.const === "number");
    const items = (key: string): Record<string, unknown> => { const child = properties[key]; return child && typeof child === "object" ? ((child as Record<string, unknown>).items as Record<string, unknown> | undefined) ?? {} : {}; };
    const owns = (record: Record<string, unknown>, keys: readonly string[]): boolean => keys.every(key => Object.hasOwn(record, key));
    if (node.additionalProperties === false) {
      const ownerRows = properties.rows as Record<string, unknown> | undefined, ownerPair = items("rows");
      if (ownerRows?.type === "array" && typeof ownerRows.minItems === "number" && ownerRows.minItems >= 3 && ownerPair.type === "array" && Array.isArray(ownerPair.items) && ownerPair.items.length === 2 && ownerPair.items.every(item => item && typeof item === "object" && item.type === "string") && owns(fields(properties.relay), ["artifactRef", "role", "expectedOwner"])) return true;
      if (Array.isArray((fixed.get("ports") as Record<string, unknown> | undefined)?.const) && ["selection", "hover"].every(key => Array.isArray(((fixed.get(key) as Record<string, unknown> | undefined)?.const as Record<string, unknown> | undefined)?.ids)) && ["x", "y", "zoom"].every(key => typeof ((fixed.get("viewport") as Record<string, unknown> | undefined)?.const as Record<string, unknown> | undefined)?.[key] === "number") && Array.isArray((fixed.get("methods") as Record<string, unknown> | undefined)?.const)) return true;
      if ((properties.source as Record<string, unknown> | undefined)?.type === "string" && fixedNumbers(["maximumBytes", "maximumItems", "maximumCopyBytes", "maximumTurns"])) return true;
      if (owns(properties, ["source", "maximumItems", "maximumCopyBytes", "maximumDepth", "maximumTurns", "cancelUnits"]) && (properties.source as Record<string, unknown> | undefined)?.type === "string" && fixedNumbers(["maximumItems", "maximumCopyBytes", "maximumDepth", "maximumTurns"]) && Array.isArray((fixed.get("cancelUnits") as Record<string, unknown> | undefined)?.const)) return true;
      if (fixedNumbers(["maximumItems", "maximumCopyBytes", "maximumDepth", "maximumTurns"]) && Array.isArray((fixed.get("prefixTurns") as Record<string, unknown> | undefined)?.const)) return true;
      if (typeof (fixed.get("text") as Record<string, unknown> | undefined)?.const === "string" && fixedNumbers(["reservedCapacity", "maximumTurns"]) && ["unsupportedKind", "unsupportedFullGrantKind"].some(key => typeof (fixed.get(key) as Record<string, unknown> | undefined)?.const === "string") && (fixedNumbers(["maximumItems", "maximumCopyBytes", "maximumDepth"]) || fixedNumbers(["logicalPage"]))) return true;
      if (owns(properties, ["maximumSymbols", "maximumSymbolBytes", "constructorBytes", "valid", "invalid", "large"])) return true;
      if (owns(properties, ["lanes", "grants", "maximumItems", "coldBirthCredited", "terminalDropBytes"])) return true;
      if (owns(properties, ["copyBytes", "grants", "cancelStops", "elements"])) return true;
      if (owns(properties, ["targets", "groups", "dependencies"]) && fixed.has("targets") && fixed.has("dependencies")) return true;
      if (owns(properties, ["maximumBytes", "constructorAllocationBytes", "variants", "initializerOwners", "axes", "backingBytes", "terminalEmpty"])) return true;
      if (owns(properties, ["wideListItems", "cancelAfter", "maximumOwnedBytes", "maximumFrontierBytes", "unknownKeyBytes", "maximumErrorBytes", "rangeListItems"])) return true;
      if (owns(properties, ["fields", "maximumBytes", "tinyMaximumBytes", "cancelAt"]) && fixed.has("schema")) return true;
      if (["package", "module", "manifest", "library", "mount", "dependencies", "publicEntries", "forbiddenProviders"].every(key => fixed.has(key))) return true;
      const owners = properties.owners as Record<string, unknown> | undefined;
      if (owns(properties, ["owners", "routers", "artifactDefinitions", "inputs", "registration"]) && owners && typeof owners.minItems === "number" && owners.minItems > 0 && owners.minItems === owners.maxItems) return true;
      const revisions = properties.rows as Record<string, unknown> | undefined;
      if (revisions && typeof revisions.minItems === "number" && revisions.minItems > 0 && revisions.minItems === revisions.maxItems && owns(fields(items("rows")), ["path", "originalSha256", "currentSha256"])) return true;
      const contexts = fields(items("contexts")), phases = contexts.phases as Record<string, unknown> | undefined;
      if (Object.hasOwn(properties, "imports") && owns(contexts, ["id", "source", "phases"]) && phases && owns(fields(phases.items), ["file", "projects", "packages", "locked", "targets"])) return true;
      if (owns(properties, ["counts", "rows", "refused", "encodings"]) && ["owner", "table", "columns", "identity"].every(key => fixed.has(key))) return true;
      if (["renderId", "kind", "group", "handle", "label", "revision", "expectedBytes"].every(key => fixed.has(key))) return true;
      const emptySegments = fields(properties.emptySegments), capacityRefusal = emptySegments.accepted as Record<string, unknown> | undefined;
      if (owns(properties, ["byteGrants", "capacityGrant", "cancelAt", "cases", "emptySegments"]) && owns(fields(items("cases")), ["code", "message", "target"]) && Object.hasOwn(emptySegments, "count") && capacityRefusal?.const === false) return true;
      const unchanged = properties.sourceUnchanged as Record<string, unknown> | undefined, changed = properties.changedSourceAccepted as Record<string, unknown> | undefined;
      if (owns(properties, ["cancelAt", "sourceUnchanged", "changedSourceAccepted"]) && unchanged?.const === true && changed?.const === false) return true;
      if (owns(properties, ["bodyBytes", "text", "repeat", "originalUnchanged", "terminalEmpty"]) && fixed.has("bodyBytes") && ["originalUnchanged", "terminalEmpty"].every(key => (properties[key] as Record<string, unknown> | undefined)?.const === true)) return true;
      if (owns(properties, ["prefix", "prefixRepeat", "cancelAt", "comparisons", "objectEntries"]) && ["bodyBytes", "payloadChunkBytes", "treeDepth", "maximumCursorBytes"].every(key => fixed.has(key)) && owns(fields(items("comparisons")), ["left", "right", "ordering"])) return true;
      if (owns(properties, ["initial", "first", "latest", "remote", "unsubmitted", "commandsAfterEnterAndBlur", "commandsAfterNextBlur", "draftAfterFirstEcho", "conflictAfterFirstEcho", "dirtyAfterFinalEcho"])) return true;
      const refusals = fields(properties.refusals);
      if (owns(properties, ["fields", "copy", "frontier", "maximumBytes", "tinyMaximumBytes", "recursiveDepth", "refusals"]) && owns(fields(properties.copy), ["repeatCount", "cancelAt"]) && owns(fields(properties.frontier), ["fieldCount", "cancelAt"]) && ["ownership", "cancellation"].every(key => refusals[key] && typeof refusals[key] === "object" && Object.hasOwn(refusals[key]!, "const"))) return true;
    }
    const fixedTrials = fixedProperties.some(([key, child]) => /(?:Grants|Frontiers|Cuts|budgetCases|zeroGrants)$/u.test(key) && Array.isArray((child as Record<string, unknown>).const));
    const fixedRefusals = fixedProperties.some(([key, child]) => /(?:^refusals$|Refusals$)/u.test(key) && Array.isArray((child as Record<string, unknown>).const));
    const fixedAfterVerification = fixedProperties.some(([key, child]) => {
      const outcome = (child as Record<string, unknown>).const;
      return /^after[A-Z]/u.test(key) && outcome !== null && typeof outcome === "object" && !Array.isArray(outcome) && Object.values(outcome).some(value => typeof value === "boolean");
    });
    const fixedScalarSchedules = fixedProperties.filter(([, child]) => {
      const schedule = (child as Record<string, unknown>).const;
      return Array.isArray(schedule) && schedule.length > 0 && schedule.every(value => typeof value === "number");
    });
    const sampleTexts = Object.values(properties).some(child => child && typeof child === "object" && (child as Record<string, unknown>).type === "array" && ((child as Record<string, unknown>).items as Record<string, unknown> | undefined)?.type === "string");
    const fixedOwnershipMove = fixedProperties.some(([, child]) => {
      const move = (child as Record<string, unknown>).const as Record<string, unknown> | null;
      return move !== null && typeof move === "object" && !Array.isArray(move) && move.items === 1 && move.copyBytes === 0;
    });
    if (!producedRecord && (fixedRefusals && fixedAfterVerification || fixedTrials && (expectations.length > 0 || fixedProperties.some(([key]) => /(?:Survives|Allocations|Readable|Mutates|Unchanged|RequiresTerminal|AfterClose|BeforeFinalReader)/u.test(key))) || fixedRefusals && sampleTexts && fixedScalarSchedules.length >= 2 && fixedOwnershipMove)) return true;
    if (Object.entries(properties).some(([key, child]) => {
      const examples = child && typeof child === "object" ? (child as Record<string, unknown>).const : undefined;
      return Array.isArray(examples) && (["cases", "vectors", "scenarios", "laws"].includes(key) || examples.some((example) => example && typeof example === "object" && Object.keys(example).some((field) => /^(?:expect$|expected(?:$|[A-Z]))/u.test(field))));
    })) return true;
    if (Object.entries(properties).some(([key, child]) => inspect(child, testCases || /(?:^|[A-Z])(?:cases|vectors|scenarios|valid|invalid)$/iu.test(key)))) return true;
    return Object.entries(node).some(([key, child]) => !["properties", "$defs", "definitions"].includes(key) && inspect(child, testCases));
  };
  return inspect(subject);
}

function readSchemaDocument(repoRoot: string, path: string, taxonomy: Taxonomy, diagnostics: SchemaScopeDiagnostic[]): SchemaScopeDocument | null {
  let parsed: Record<string, unknown>;
  try {
    parsed = JSON.parse(readFileSync(join(repoRoot, path), "utf8")) as Record<string, unknown>;
  } catch (error) {
    diagnostics.push({ code: "schema-document-unparseable", path, detail: (error as Error).message });
    return null;
  }
  if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) {
    diagnostics.push({ code: "schema-document-not-object", path, detail: "A schema module leaf must be a JSON Schema object." });
    return null;
  }
  const resolution = taxonomy.schemaExportResolution;
  const id = typeof parsed.$id === "string" ? parsed.$id : null;
  const dialect = typeof parsed.$schema === "string" ? parsed.$schema : null;
  if (!id) diagnostics.push({ code: "schema-module-id-missing", path, detail: `Declare $id as ${resolution.idBase}<scope path>/<facet>.json.` });
  const parts = id ? schemaDocumentIdParts(id, taxonomy) : null;
  const scopeId = parts ? parts.scopePath.join(resolution.scopeIdSeparator) : null;
  if (id && !scopeId) diagnostics.push({ code: "schema-document-id-unaddressable", path, detail: `$id ${JSON.stringify(id)} does not resolve to a dotted scope id under ${resolution.idBase}.` });
  if (dialect !== taxonomy.schemaJsonDialect) diagnostics.push({ code: "schema-dialect-not-draft-07", path, detail: `$schema must be ${taxonomy.schemaJsonDialect}, got ${JSON.stringify(dialect)}.` });
  if (parsed["x-semio-mutationKinds"] !== undefined) diagnostics.push({ code: "schema-mutation-aggregate-kinds-redundant", path, detail: "x-semio-mutationKinds restates the oneOf $ref union and has no reader; the aggregate's $refs are the identity." });
  const exportPattern = new RegExp(resolution.exportIdPattern, "u");
  const formatKeys = Object.keys(taxonomy.schemaFormats).sort(byteSort);
  const exports: string[] = [];
  const exportFormats: Record<string, readonly string[]> = {};
  const restriction = (subject: unknown, exportId: string): void => {
    const declared = subject && typeof subject === "object" ? (subject as Record<string, unknown>)["x-semio-formats"] : undefined;
    if (declared === undefined) return;
    if (!Array.isArray(declared) || declared.length === 0 || declared.some((entry) => typeof entry !== "string" || !formatKeys.includes(entry))) {
      diagnostics.push({ code: "schema-export-formats-annotation-invalid", path, detail: `${exportId} declares x-semio-formats ${JSON.stringify(declared)}; restricted support is a non-empty subset of ${formatKeys.join(", ")}.` });
      return;
    }
    exportFormats[exportId] = [...new Set(declared as string[])].sort(byteSort);
  };
  const rootExport = typeof parsed[resolution.rootExportKeyword] === "string" ? String(parsed[resolution.rootExportKeyword]) : null;
  const rootIsObject = parsed.type === "object" || (parsed.type === undefined && typeof parsed.properties === "object" && parsed.properties !== null);
  const rootIsCorpus = schemaTestCorpusDefinition(rootExport ?? "", parsed, parsed);
  if (rootIsCorpus) diagnostics.push({ code: "schema-fixture-defines-schema", path, detail: `${rootExport ?? "The document root"} defines a test corpus contract. Examples use actual domain contracts and cannot own separate schemas.` });
  if (rootExport && rootIsObject && !rootIsCorpus) {
    if (exportPattern.test(rootExport)) {
      exports.push(rootExport);
      restriction(parsed, rootExport);
    } else diagnostics.push({ code: "schema-export-id-invalid", path, detail: `Root ${resolution.rootExportKeyword} ${JSON.stringify(rootExport)} is not a PascalCase export id.` });
  }
  const defs = parsed[resolution.exportsKeyword];
  const defRows = defs && typeof defs === "object" && !Array.isArray(defs) ? (defs as Record<string, unknown>) : {};
  const localDefinitions = parsed.definitions && typeof parsed.definitions === "object" && !Array.isArray(parsed.definitions) ? parsed.definitions as Record<string, unknown> : {};
  for (const [key, value] of Object.entries(localDefinitions)) if (schemaTestCorpusDefinition(key, value, parsed)) diagnostics.push({ code: "schema-fixture-defines-schema", path, detail: `${key} defines a test corpus contract. Examples use actual domain contracts and cannot own separate schemas.` });
  const defKeys = Object.keys(defRows);
  for (const key of defKeys) {
    if (schemaTestCorpusDefinition(key, defRows[key], parsed)) diagnostics.push({ code: "schema-fixture-defines-schema", path, detail: `${key} defines a test corpus contract. Examples use actual domain contracts and cannot own separate schemas.` });
    else if (!exportPattern.test(key)) diagnostics.push({ code: "schema-export-id-invalid", path, detail: `${resolution.exportsKeyword} key ${JSON.stringify(key)} is not a PascalCase export id.` });
    else if (!exports.includes(key)) {
      exports.push(key);
      restriction(defRows[key], key);
    }
  }
  const emptyEnums: string[] = [];
  jsonEmptyEnumPointers(parsed, "", emptyEnums);
  for (const pointer of emptyEnums) diagnostics.push({ code: "schema-enum-empty", path, detail: `${pointer} declares enum: [], which no instance can satisfy. State the intent instead: propertyNames: false or maxProperties: 0 for "no member is admitted", or delete the keyword.` });
  const occurrences: { pointer: string; ref: string }[] = [];
  jsonRefOccurrences(parsed, "", occurrences);
  const references: SchemaScopeReference[] = [];
  for (const { pointer, ref } of occurrences) {
    if (ref.startsWith("#")) {
      const local = ref.slice(1);
      const prefix = `/${resolution.exportsKeyword}/`;
      const exportId = local.startsWith(prefix) && !local.slice(prefix.length).includes("/") ? local.slice(prefix.length) : null;
      if (!jsonPointerResolves(parsed, local)) diagnostics.push({ code: "schema-ref-unresolved", path, detail: `${pointer} references ${JSON.stringify(ref)}, which this document does not declare.` });
      references.push({ documentPath: path, pointer, ref, targetScopeId: scopeId, targetExportId: exportId, local: true });
      continue;
    }
    if (!ref.startsWith(resolution.idBase)) {
      diagnostics.push({ code: "schema-ref-unresolved", path, detail: `${pointer} uses ${JSON.stringify(ref)}; a cross-scope reference names the target $id under ${resolution.idBase}.` });
      continue;
    }
    const [base, fragment = ""] = ref.split("#");
    const targetScopeId = schemaScopeIdFromDocumentId(base!, taxonomy);
    const prefix = `/${resolution.exportsKeyword}/`;
    const exportId = fragment.startsWith(prefix) ? fragment.slice(prefix.length) : null;
    if (!targetScopeId || (fragment && !exportId)) {
      diagnostics.push({ code: "schema-ref-unresolved", path, detail: `${pointer} uses ${JSON.stringify(ref)}; a cross-scope reference is <target $id>#${prefix}<ExportId>.` });
      continue;
    }
    references.push({ documentPath: path, pointer, ref, targetScopeId, targetExportId: exportId, local: false });
  }
  return { path, scopeId, scopePath: parts?.scopePath ?? null, id, facetFilename: parts ? parts.facet.replace(/\.json$/u, "") : null, dialect, exports, exportFormats, references };
}

/**
 * 🧬️ Walks every `🧬️schema` module, reads its documents and derives the scope catalog and every finding.
 *
 * A module's content is the set of walked leaves that are still readable when the walk reaches them, so a
 * file a concurrent writer removes mid-walk is absent from `hashes`, `documents` and the catalog row alike
 * rather than aborting the run or outliving its deletion (ledger row 109).
 */
export function inventorySchemaScopes(repoRoot: string, taxonomy: Taxonomy = loadCatalogTaxonomy()): SchemaScopeInventory {
  const root = resolve(repoRoot);
  const { directories, files } = walkRepositoryTree(root, taxonomy);
  const facetDir = taxonomy.schemaScopeOwnerLevels.facetDirName;
  const resolution = taxonomy.schemaExportResolution;
  const diagnostics: SchemaScopeDiagnostic[] = [];
  const matcher = createTaxonomyPathMatcher();
  const modulePaths = directories.filter((path) => path.endsWith(`/${facetDir}`) || path === facetDir).sort(byteSort);
  const modulePathSet = new Set(modulePaths);
  const fileSet = new Set(files);
  const sortedFiles = [...files].sort(byteSort);
  const filesUnder = (prefix: string): string[] => {
    const start = lowerBound(sortedFiles, `${prefix}/`);
    const found: string[] = [];
    for (let index = start; index < sortedFiles.length && sortedFiles[index]!.startsWith(`${prefix}/`); index += 1) found.push(sortedFiles[index]!);
    return found;
  };
  const formatLeaves = Object.entries(taxonomy.schemaFormats).map(([formatId, format]) => [formatId, canonicalPrimaryFilenameForKind(format.fileKindId, taxonomy)] as const);
  const ownerPaths = modulePaths.map((modulePath) => schemaScopeModuleOwnerPath(modulePath, taxonomy, matcher));
  const nestedOwners = new Set(ownerPaths);
  const facetChainDirs = new Set(taxonomy.schemaChildDirs);
  const ownedByNestedScope = (path: string, modulePath: string): boolean => {
    const segments = path.slice(modulePath.length + 1).split("/");
    for (let index = 1; index < segments.length; index += 1) if (nestedOwners.has(`${modulePath}/${segments.slice(0, index).join("/")}`)) return true;
    return false;
  };
  const canonicalLeaves = new Set(formatLeaves.map(([, leaf]) => leaf));
  const normativeFormatId = taxonomy.schemaFacetKinds![taxonomy.schemaDefaultFacetKind]!.normativeFormat;
  const normativeJsonLeaf = canonicalPrimaryFilenameForKind(taxonomy.schemaFormats[normativeFormatId]!.fileKindId, taxonomy);
  const isFacetLeaf = (path: string, modulePath: string, leaves: ReadonlySet<string>): boolean => {
    const segments = path.slice(modulePath.length + 1).split("/");
    return leaves.has(segments.at(-1)!) && segments.slice(0, -1).every((segment) => facetChainDirs.has(segment));
  };
  const descriptorLeafName = canonicalPrimaryFilenameForKind(taxonomy.mutationDescriptorFileKindId, taxonomy);
  const inertDeclarations = new Map<string, ReadonlySet<string>>();
  const inertSchemaData = (path: string): boolean => {
    if (schemaCollectionContractPath(path, taxonomy, matcher)) return false;
    const segments = path.split("/");
    for (let index = segments.length - 1; index > 0; index -= 1) {
      const caseFile = `${segments.slice(0, index).join("/")}/${descriptorLeafName}`;
      if (!fileSet.has(caseFile)) continue;
      let declared = inertDeclarations.get(caseFile);
      if (!declared) {
        let named: string[] = [];
        try {
          const value = (JSON.parse(readFileSync(join(root, caseFile), "utf8")) as Record<string, unknown>)[taxonomy.schemaScopeOwnerLevels.inertSchemaDataKeyword];
          if (Array.isArray(value)) named = value.filter((entry): entry is string => typeof entry === "string");
        } catch {
          named = [];
        }
        declared = new Set(named);
        inertDeclarations.set(caseFile, declared);
      }
      if (declared.has(segments.slice(index).join("/"))) return true;
    }
    return false;
  };
  const modules: SchemaScopeModule[] = [];
  const rootScopePaths = new Map<string, readonly string[] | null>();
  const rootDocuments = new Map<string, SchemaScopeDocument>();
  for (const [index, modulePath] of modulePaths.entries()) {
    const ownerPath = ownerPaths[index]!;
    if (schemaScopeCollectionPath(modulePath, taxonomy, matcher)) {
      continue;
    }
    const level = schemaScopeOwnerLevel(ownerPath, taxonomy, matcher);
    if (!level) diagnostics.push({ code: "schema-owner-ineligible", path: modulePath, detail: `${ownerPath || "."} is not a declared schemaScopeOwnerLevels level.` });
    const walked = filesUnder(modulePath).filter((path) => !ownedByNestedScope(path, modulePath) && !schemaScopeOwnerExcluded(path, taxonomy, matcher) && isFacetLeaf(path, modulePath, canonicalLeaves));
    const hashes: Record<string, string> = {};
    const owned: string[] = [];
    for (const path of walked) {
      let bytes: Buffer;
      try {
        bytes = readFileSync(join(root, path));
      } catch {
        continue;
      }
      owned.push(path);
      hashes[path.slice(modulePath.length + 1)] = createHash("sha256").update(bytes).digest("hex");
    }
    const facetKindId = resolveSchemaFacetKind(modulePath, taxonomy);
    const formats: Record<string, string> = {};
    for (const [formatId, leaf] of formatLeaves) if (fileSet.has(`${modulePath}/${leaf}`)) formats[formatId] = `${modulePath}/${leaf}`;
    const jsonLeaf = canonicalPrimaryFilenameForKind(taxonomy.schemaFormats[taxonomy.schemaFacetKinds![facetKindId]!.normativeFormat]!.fileKindId, taxonomy);
    const documents = jsonLeaf.endsWith(".json") ? owned.filter((path) => path.endsWith(`/${jsonLeaf}`)).map((path) => readSchemaDocument(root, path, taxonomy, diagnostics)).filter((document): document is SchemaScopeDocument => document !== null) : [];
    const rootDocument = documents.find((document) => document.path === `${modulePath}/${jsonLeaf}`) ?? null;
    const rootScopeId = rootDocument?.scopeId ?? null;
    const rootScopePath = rootDocument?.scopePath ?? null;
    rootScopePaths.set(modulePath, rootScopePath);
    if (rootDocument) rootDocuments.set(modulePath, rootDocument);
    if (documents.length > 0 && !rootScopeId) diagnostics.push({ code: "schema-module-id-missing", path: modulePath, detail: `${modulePath}/${jsonLeaf} must declare the $id that names this module's scope.` });
    const mutationsPrefix = `${taxonomy.schemaScopeOwnerLevels.mutationsFacetDirName}/`;
    if (rootScopeId && rootScopePath) for (const document of documents) {
      if (document === rootDocument || !document.scopePath) continue;
      const relative = document.path.slice(modulePath.length + 1);
      if (relative === `${mutationsPrefix}${jsonLeaf}` || relative.startsWith(mutationsPrefix + mutationsPrefix)) continue;
      if (document.scopePath.length >= rootScopePath.length && document.scopePath.slice(0, rootScopePath.length).join("/") === rootScopePath.join("/")) continue;
      diagnostics.push({ code: "schema-module-id-inconsistent", path: document.path, detail: `$id ${JSON.stringify(document.id)} must keep this module's scope path ${resolution.idBase}${rootScopePath.join("/")}/ and vary only the facet filename; a facet document never deepens the scope.` });
    }
    modules.push({ modulePath, ownerPath, level, facetKindId, scopeId: rootScopeId, formats, documents, hashes });
  }
  const admittedDocuments = new Set(modules.flatMap(module => module.documents.map(document => document.path)));
  for (const path of sortedFiles) {
    const collection = schemaScopeCollectionPath(path, taxonomy, matcher);
    if (collection && schemaCollectionContractPath(path, taxonomy, matcher)) {
      diagnostics.push({ code: "schema-fixture-defines-schema", path, detail: `${path} defines schema authority inside examples; contract identity cannot be declared inert.` });
      continue;
    }
    if (!path.endsWith(".json") || inertSchemaData(path) || admittedDocuments.has(path)) continue;
    let parsed: unknown;
    try { parsed = JSON.parse(readFileSync(join(root, path), "utf8")); } catch { continue; }
    if (collection) {
      if (schemaRootAuthority(parsed) || schemaEmbeddedExampleAuthority(parsed)) diagnostics.push({ code: "schema-fixture-defines-schema", path, detail: `${path} defines schema authority inside examples. Ordinary parser specimens require an explicit inert data declaration.` });
      continue;
    }
    if (!schemaRootAuthority(parsed) || !parsed || typeof parsed !== "object" || Array.isArray(parsed)) continue;
    const document = parsed as Record<string, unknown>;
    const rootExport = typeof document[resolution.rootExportKeyword] === "string" ? String(document[resolution.rootExportKeyword]) : "";
    if (schemaTestCorpusDefinition(rootExport, document, document)) diagnostics.push({ code: "schema-fixture-defines-schema", path, detail: "The document defines a test corpus contract outside the admitted schema facets." });
    for (const keyword of [resolution.exportsKeyword, "definitions"]) {
      const definitions = document[keyword];
      if (!definitions || typeof definitions !== "object" || Array.isArray(definitions)) continue;
      for (const [name, value] of Object.entries(definitions)) if (schemaTestCorpusDefinition(name, value, document)) diagnostics.push({ code: "schema-fixture-defines-schema", path, detail: `${name} defines a test corpus contract outside the admitted schema facets.` });
    }
  }
  const declaredSemanticKind = (leafOwnerPath: string): string | null => {
    try {
      const kind = (JSON.parse(readFileSync(join(root, leafOwnerPath, descriptorLeafName), "utf8")) as { semanticKind?: unknown }).semanticKind;
      return typeof kind === "string" && kind ? kind : null;
    } catch {
      return null;
    }
  };
  const mutationsDir = taxonomy.schemaScopeOwnerLevels.mutationsFacetDirName;
  const mutationRootModule = (leafOwnerPath: string): { readonly module: string; readonly depth: number } | null => {
    const segments = leafOwnerPath.split("/");
    for (let index = segments.length - 1; index >= 1; index -= 1) {
      if (segments[index] !== mutationsDir || segments[index - 1] !== facetDir) continue;
      return { module: segments.slice(0, index).join("/"), depth: segments.length - index - 1 };
    }
    return null;
  };
  const mutationLeafOwners: string[] = [];
  for (const path of sortedFiles) {
    if (path.slice(path.lastIndexOf("/") + 1) !== descriptorLeafName) continue;
    const ownerPath = path.slice(0, path.lastIndexOf("/"));
    const enclosing = mutationRootModule(ownerPath);
    if (!enclosing || enclosing.depth < 1 || enclosing.depth > 2 || schemaScopeCollectionPath(ownerPath, taxonomy, matcher)) continue;
    if (declaredSemanticKind(ownerPath) !== null) mutationLeafOwners.push(ownerPath);
  }
  for (const leafOwnerPath of mutationLeafOwners) {
    const enclosing = mutationRootModule(leafOwnerPath)!;
    const enclosingScopePath = rootScopePaths.get(enclosing.module) ?? null;
    const kind = declaredSemanticKind(leafOwnerPath);
    const expectedPath = [...(enclosingScopePath ?? ["<enclosing scope path>"]), resolution.mutationScopeSegment, kind ?? "<semanticKind>"];
    const expected = `${resolution.idBase}${[...expectedPath, resolution.mutationLeafFacetFilename].join("/")}`;
    const leafModulePath = `${leafOwnerPath}/${facetDir}`;
    const document = rootDocuments.get(leafModulePath) ?? null;
    if (!document) {
      diagnostics.push({ code: "schema-mutation-leaf-schema-absent", path: leafModulePath, detail: `The leaf is the authority for its own payload, so ${leafModulePath}/${normativeJsonLeaf} must exist and declare $id ${expected}.` });
      continue;
    }
    if (document.id === expected) continue;
    diagnostics.push({ code: "schema-mutation-leaf-id", path: document.path, detail: `$id ${JSON.stringify(document.id)} must be ${expected}: a mutation leaf is its own scope, not a facet of its enclosing module.` });
  }
  for (const module of modules) {
    const rootScopePath = rootScopePaths.get(module.modulePath) ?? null;
    if (!rootScopePath) continue;
    const aggregate = module.documents.find((document) => document.path === `${module.modulePath}/${mutationsDir}/${normativeJsonLeaf}`);
    if (!aggregate) continue;
    const expected = `${resolution.idBase}${[...rootScopePath, resolution.mutationAggregateFacetFilename].join("/")}`;
    if (aggregate.id === expected) continue;
    diagnostics.push({ code: "schema-mutation-aggregate-id", path: aggregate.path, detail: `$id ${JSON.stringify(aggregate.id)} must be ${expected}: an aggregate is the mutations facet of its own module and never deepens the scope.` });
  }
  const documentsById = new Map<string, { readonly scopeId: string | null; readonly path: string; readonly exports: ReadonlySet<string> }>();
  for (const module of modules) for (const document of module.documents) {
    if (!document.id) continue;
    const existing = documentsById.get(document.id);
    if (existing) {
      diagnostics.push({ code: "schema-document-id-duplicate", path: document.path, detail: `$id ${document.id} is already declared by ${existing.path}; a document $id is the resolution key and is unique repository-wide.` });
      continue;
    }
    documentsById.set(document.id, { scopeId: module.level ? module.scopeId : null, path: document.path, exports: new Set(document.exports) });
  }
  const byScope = new Map<string, SchemaScopeModule[]>();
  for (const module of modules) if (module.scopeId && module.level) byScope.set(module.scopeId, [...(byScope.get(module.scopeId) ?? []), module]);
  const scopes: Record<string, SchemaCatalogScope> = {};
  for (const [scopeId, claimants] of [...byScope.entries()].sort(([left], [right]) => byteSort(left, right))) {
    const owner = claimants[0]!;
    if (claimants.length > 1) for (const claimant of claimants) diagnostics.push({ code: "schema-scope-ambiguous", path: claimant.modulePath, detail: `Scope ${scopeId} is declared by ${claimant.modulePath}/${canonicalPrimaryFilenameForKind(taxonomy.schemaFormats[taxonomy.schemaFacetKinds![claimant.facetKindId]!.normativeFormat]!.fileKindId, taxonomy)} and by ${claimants.filter((other) => other !== claimant).map((other) => other.modulePath).join(", ")}; ${owner.modulePath} holds the catalog row.` });
    const ownerScopePath = rootScopePaths.get(owner.modulePath) ?? null;
    const exports: Record<string, SchemaCatalogExport> = {};
    for (const document of [...owner.documents].sort((left, right) => byteSort(left.path, right.path))) {
      const relative = document.path.slice(owner.modulePath.length + 1);
      const suffix = ownerScopePath && document.scopePath && document.scopePath.length >= ownerScopePath.length ? document.scopePath.slice(ownerScopePath.length) : [];
      const facet = document.facetFilename ? [...suffix, document.facetFilename].join("/") : relative.slice(0, Math.max(0, relative.lastIndexOf("/"))) || taxonomy.schemaDefaultFacetKind;
      for (const exportId of document.exports) {
        const existing = exports[exportId];
        if (existing) {
          if (existing.file !== relative) diagnostics.push({ code: "schema-export-id-duplicate", path: document.path, detail: `Export ${exportId} is already declared by ${owner.modulePath}/${existing.file}; one scope resolves each export id to exactly one file.` });
          continue;
        }
        exports[exportId] = { file: relative, facet };
      }
    }
    const dependsOn = [...new Set(owner.documents.flatMap((document) => document.references).filter((reference) => !reference.local).map((reference) => documentsById.get(reference.ref.split("#")[0]!)?.scopeId).filter((value): value is string => value !== null && value !== undefined && value !== scopeId))].sort(byteSort);
    const hashes = Object.fromEntries(Object.entries(owner.hashes).sort(([left], [right]) => byteSort(left, right)));
    const formats = Object.fromEntries(Object.entries(owner.formats).map(([formatId, path]) => [formatId, path.slice(owner.modulePath.length + 1)]));
    scopes[scopeId] = { path: owner.modulePath, level: owner.level!, facetKind: owner.facetKindId, formats, exports: Object.fromEntries(Object.entries(exports).sort(([left], [right]) => byteSort(left, right))), dependsOn, hashes };
  }
  for (const module of modules) for (const document of module.documents) for (const reference of document.references) {
    if (reference.local) continue;
    const target = documentsById.get(reference.ref.split("#")[0]!);
    if (!target) diagnostics.push({ code: "schema-ref-unresolved", path: document.path, detail: `${reference.pointer} references ${JSON.stringify(reference.ref)}; no catalogued document declares that $id.` });
    else if (reference.targetExportId && !target.exports.has(reference.targetExportId)) diagnostics.push({ code: "schema-ref-unresolved", path: document.path, detail: `${reference.pointer} references export ${JSON.stringify(reference.targetExportId)}, which ${target.path} does not declare.` });
    else if (target.scopeId === null) diagnostics.push({ code: "schema-cross-scope-dependency-uncataloged", path: document.path, detail: `${reference.pointer} resolves into ${target.path}, which no catalogued scope owns; a module compiles only against the scopes its dependsOn names.` });
    else if (module.scopeId && module.level && byScope.get(module.scopeId)?.[0] === module && target.scopeId !== module.scopeId && !(scopes[module.scopeId]?.dependsOn ?? []).includes(target.scopeId)) diagnostics.push({ code: "schema-cross-scope-dependency-forbidden", path: document.path, detail: `${reference.pointer} crosses into scope ${target.scopeId}, which ${module.scopeId} does not declare in dependsOn.` });
  }
  for (const [scopeId, scope] of Object.entries(scopes)) {
    const provided = Object.keys(scope.formats).filter((formatId) => formatId !== normativeFormatId);
    if (provided.length === 0) continue;
    const sources = new Map<string, string>();
    for (const formatId of provided) {
      try {
        sources.set(formatId, readFileSync(join(root, scope.path, scope.formats[formatId]!), "utf8"));
      } catch {
        sources.set(formatId, "");
      }
    }
    const restrictions = new Map<string, readonly string[]>();
    for (const module of byScope.get(scopeId) ?? []) for (const document of module.documents) for (const [exportId, formats] of Object.entries(document.exportFormats)) restrictions.set(exportId, formats);
    for (const [exportId, declaration] of Object.entries(scope.exports)) {
      const restricted = restrictions.get(exportId);
      for (const formatId of provided) {
        if (restricted && !restricted.includes(formatId)) continue;
        if (schemaFormatDeclaresExport(formatId, sources.get(formatId) ?? "", exportId)) continue;
        diagnostics.push({ code: "schema-export-incomplete", path: `${scope.path}/${scope.formats[formatId]!}`, detail: `${scopeId} declares export ${exportId} in ${declaration.file}; ${formatId} carries no declaration of it. Provide it, or annotate the export with x-semio-formats to declare restricted support honestly.` });
      }
    }
  }
  const placement: SchemaScopeDiagnostic[] = [];
  let reportedAncestor: string | null = null;
  for (const path of [...files, ...directories].sort(byteSort)) {
    if (Object.hasOwn(resolution.placementExceptions, path)) continue;
    if (modulePathSet.has(path) || inertSchemaData(path)) continue;
    if (reportedAncestor && path.startsWith(`${reportedAncestor}/`)) continue;
    const pattern = resolution.forbiddenPlacementPatterns.find((candidate) => matcher.matches(path, candidate));
    if (!pattern) continue;
    reportedAncestor = path;
    placement.push({ code: "schema-placement-forbidden-filename", path, detail: `${pattern} ceases to exist: move the contract into its owner's ${facetDir} module.` });
  }
  const catalog: SchemaCatalog = {
    contractId: resolution.catalogContractId,
    catalogVersion: 1,
    generator: resolution.generator,
    taxonomySchemaVersion: taxonomy.schemaVersion,
    jsonSchemaDialect: taxonomy.schemaJsonDialect,
    scopes,
  };
  return { modules, catalog, diagnostics: diagnostics.slice().sort((left, right) => byteSort(left.path, right.path) || byteSort(left.code, right.code) || byteSort(left.detail, right.detail)), placement };
}

/** 📇️ Deterministic catalog bytes; the generated file is exactly this rendering. */
export function renderSchemaCatalog(catalog: SchemaCatalog): string {
  return `${JSON.stringify(catalog, null, 2)}\n`;
}

/** 📓️ Markdown index of every catalogued scope, its exports and its cross-scope dependencies. */
export function renderSchemaCatalogDocument(catalog: SchemaCatalog): string {
  const lines = [
    "# 🧬️ Schema scope catalog",
    "",
    `Generated by \`${catalog.generator}\` — never hand-edited.`,
    "",
    `- contract: \`${catalog.contractId}\``,
    `- taxonomy schemaVersion: \`${catalog.taxonomySchemaVersion}\``,
    `- JSON Schema dialect: \`${catalog.jsonSchemaDialect}\``,
    `- scopes: \`${Object.keys(catalog.scopes).length}\``,
    "",
    "| Scope | Level | Module | Exports | Depends on |",
    "| --- | --- | --- | --- | --- |",
  ];
  for (const [scopeId, scope] of Object.entries(catalog.scopes)) {
    lines.push(`| \`${scopeId}\` | ${scope.level} | \`${scope.path}\` | ${Object.entries(scope.exports).map(([exportId, declaration]) => `\`${exportId}\` (\`${declaration.file}\`)`).join(", ") || "—"} | ${scope.dependsOn.map((value) => `\`${value}\``).join(", ") || "—"} |`);
  }
  lines.push("");
  return lines.join("\n");
}

/** 🦀️ One `(scope, export, format)` triple the Rust `schema_export_catalog_entries()` registry resolves to a non-empty leaf. */
export interface SchemaRustEntry {
  readonly scope: string;
  readonly export: string;
  readonly format: string;
}

/** 🦀️ The dump `schema verify` consumes; `contractId` is `schemaExportResolution.rustEntriesContractId`. */
export interface SchemaRustEntryDump {
  readonly contractId: string;
  readonly generator: string;
  readonly entries: readonly SchemaRustEntry[];
}

/**
 * 🦀️ Cross-checks the generated catalog against the Rust registry dump on the `(scope, export, format)` key.
 * Only scopes the dump declares are compared in both directions; catalogued scopes the dump never mentions are
 * reported as `rust-scope-unregistered` when `complete` is set, so the gate is usable while registration lands.
 */
export function schemaRustEntryDiagnostics(catalog: SchemaCatalog, dump: SchemaRustEntryDump, taxonomy: Taxonomy = loadCatalogTaxonomy(), complete = false): SchemaScopeDiagnostic[] {
  const resolution = taxonomy.schemaExportResolution;
  const path = resolution.catalogPath;
  const diagnostics: SchemaScopeDiagnostic[] = [];
  if (dump.contractId !== resolution.rustEntriesContractId) return [{ code: "rust-entries-contract-unknown", path, detail: `The dump declares contractId ${JSON.stringify(dump.contractId)}; ${resolution.rustEntriesContractId} is the declared shape.` }];
  const asciiByKey = resolution.formatIds;
  const known = new Set(Object.values(asciiByKey));
  const declared = new Set(dump.entries.map((entry) => entry.scope));
  const catalogTriples = new Set<string>();
  for (const [scopeId, scope] of Object.entries(catalog.scopes)) {
    if (!declared.has(scopeId)) continue;
    for (const exportId of Object.keys(scope.exports)) for (const formatKey of Object.keys(scope.formats)) if (asciiByKey[formatKey]) catalogTriples.add(`${scopeId}\0${exportId}\0${asciiByKey[formatKey]}`);
  }
  const rustTriples = new Set<string>();
  for (const entry of dump.entries) {
    const key = `${entry.scope}\0${entry.export}\0${entry.format}`;
    rustTriples.add(key);
    if (!known.has(entry.format)) diagnostics.push({ code: "rust-entry-format-unknown", path, detail: `${entry.scope}/${entry.export} registers format ${JSON.stringify(entry.format)}; the declared ids are ${[...known].sort().join(", ")}.` });
    else if (!catalog.scopes[entry.scope]) diagnostics.push({ code: "rust-scope-unknown", path, detail: `The registry declares scope ${entry.scope}, which the catalog does not contain.` });
    else if (!Object.hasOwn(catalog.scopes[entry.scope]!.exports, entry.export)) diagnostics.push({ code: "rust-export-unknown", path, detail: `The registry declares ${entry.scope}/${entry.export}, which ${catalog.scopes[entry.scope]!.path} does not export.` });
    else if (!catalogTriples.has(key)) diagnostics.push({ code: "rust-entry-format-absent", path, detail: `The registry resolves ${entry.scope}/${entry.export} in ${entry.format}; ${catalog.scopes[entry.scope]!.path} declares no leaf for that format.` });
  }
  for (const key of [...catalogTriples].sort(byteSort)) {
    if (rustTriples.has(key)) continue;
    const [scopeId, exportId, format] = key.split("\0") as [string, string, string];
    diagnostics.push({ code: "rust-entry-missing", path, detail: `The catalog declares ${scopeId}/${exportId} in ${format}; the registry of that scope does not register it.` });
  }
  if (complete) for (const scopeId of Object.keys(catalog.scopes).sort(byteSort)) {
    if (declared.has(scopeId)) continue;
    diagnostics.push({ code: "rust-scope-unregistered", path, detail: `${catalog.scopes[scopeId]!.path} declares scope ${scopeId}; no register_scope_schema_exports call publishes it.` });
  }
  return diagnostics.slice().sort((left, right) => byteSort(left.code, right.code) || byteSort(left.detail, right.detail));
}

/** 🧾️ One JSON line per finding, byte-stable across runs, for `schema check --report`. */
export function renderSchemaCheckReport(findings: readonly SchemaScopeDiagnostic[]): string {
  return findings.map((finding) => JSON.stringify(finding)).join("\n") + (findings.length > 0 ? "\n" : "");
}
