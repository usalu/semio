import { jsonDocumentDuplicateKeys } from "../../🧬️schema/🔣️json-document/🟦️.ts";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";
import { cargoProviderTomlParser } from "../🚚️cargo-provider/🟦️.ts";
import { taxonomyPathPatternMatches } from "../🎯️taxonomy-path-matching/🟦️.ts";

export type SchemaFormatSpec = { readonly fileKindId: string; readonly fieldCasing: string };

export type TaxonomyImplementationBreachId = "taxonomy/kind-only-basename" | "taxonomy/target-inside-package-boundary";

export interface TaxonomyImplementationFinding {
  readonly breachId: TaxonomyImplementationBreachId;
  readonly path: string;
  readonly fileKindId: string | null;
  readonly actualBasename: string | null;
  readonly expectedBasename: string | null;
  readonly exemptionAuthorityId: string | null;
}

export function implementationPackageLocation(path: string, taxonomy: Taxonomy): Readonly<{ packageRoot: string; ecosystemId: string }> | null {
  const parts = path.replaceAll("\\", "/").replace(/^\.\//u, "").split("/");
  const packageIndex = parts.indexOf(taxonomy.packagesDirName);
  const ecosystemId = parts[packageIndex + 1] ?? "";
  return packageIndex < 0 || !taxonomy.ecosystems[ecosystemId] ? null : { packageRoot: parts.slice(0, packageIndex + 2).join("/"), ecosystemId };
}

/** 🏗️ Resolves Cargo's active package-root build script from an actual manifest. */
export function cargoPackageRootBuildScriptPath(source: string): string | null {
  try {
    const parsed = cargoProviderTomlParser.parse(source) as { readonly package?: { readonly name?: unknown; readonly build?: unknown } };
    if (typeof parsed.package?.name !== "string" || !/^[A-Za-z0-9_-]+$/u.test(parsed.package.name)) return null;
    const configured = parsed.package.build;
    if (configured === undefined) return "build.rs";
    if (configured === false) return null;
    return configured === "build.rs" ? configured : null;
  } catch {
    return null;
  }
}

export interface TaxonomyImplementationCensusProgress {
  readonly phase: "walk" | "classify" | "complete";
  readonly pathsVisited: number;
  readonly pathsClassified: number;
  readonly findings: number;
  readonly vanishedDirectories: number;
}

export interface TaxonomyImplementationCensusOptions {
  readonly signal?: AbortSignal;
  readonly onProgress?: (progress: TaxonomyImplementationCensusProgress) => void;
  readonly yieldEvery?: number;
}

/** 📐️ Validates Draft-07 keyword shapes without imposing names on a schema's physical file. */
export function mutationPayloadSchemaDocumentProblems(value: unknown, dialect = "http://json-schema.org/draft-07/schema#"): string[] {
  const problems: string[] = [];
  const object = (item: unknown): item is Record<string, unknown> => item !== null && typeof item === "object" && !Array.isArray(item);
  const canonical = (item: unknown): string => Array.isArray(item) ? `[${item.map(canonical).join(",")}]` : object(item) ? `{${Object.keys(item).sort().map((key) => `${JSON.stringify(key)}:${canonical(item[key])}`).join(",")}}` : JSON.stringify(item) ?? String(item);
  const unique = (items: readonly unknown[]): boolean => new Set(items.map(canonical)).size === items.length;
  const strings = (item: unknown): boolean => Array.isArray(item) && item.every((part) => typeof part === "string") && unique(item);
  const regex = (pattern: string): boolean => { try { new RegExp(pattern, "u"); return true; } catch { return false; } };
  const visit = (item: unknown, path: string): void => {
    if (typeof item === "boolean") return;
    if (!object(item)) { problems.push(`${path} must be an object or boolean schema`); return; }
    for (const [key, child] of Object.entries(item)) {
      const at = `${path}/${key}`;
      if (["$id", "$schema", "$ref", "$comment", "title", "description", "format", "contentMediaType", "contentEncoding", "pattern"].includes(key) && typeof child !== "string") problems.push(`${at} must be a string`);
      if (["readOnly", "uniqueItems"].includes(key) && typeof child !== "boolean") problems.push(`${at} must be a boolean`);
      if (["maximum", "exclusiveMaximum", "minimum", "exclusiveMinimum", "multipleOf"].includes(key) && (typeof child !== "number" || !Number.isFinite(child) || key === "multipleOf" && child <= 0)) problems.push(`${at} must be a valid numeric bound`);
      if (["maxLength", "minLength", "maxItems", "minItems", "maxProperties", "minProperties"].includes(key) && (typeof child !== "number" || !Number.isInteger(child) || child < 0)) problems.push(`${at} must be a nonnegative integer`);
      if (key === "pattern" && typeof child === "string" && !regex(child)) problems.push(`${at} must be a regular expression`);
      if (key === "required" && !strings(child)) problems.push(`${at} must contain unique strings`);
      if (key === "examples" && !Array.isArray(child)) problems.push(`${at} must be an array`);
      if (key === "enum" && (!Array.isArray(child) || child.length === 0 || !unique(child))) problems.push(`${at} must contain unique values`);
      if (key === "type") {
        const types = Array.isArray(child) ? child : [child];
        if (types.length === 0 || !unique(types) || types.some((type) => typeof type !== "string" || !["array", "boolean", "integer", "null", "number", "object", "string"].includes(type))) problems.push(`${at} must name unique JSON types`);
      }
      if (["additionalItems", "contains", "additionalProperties", "propertyNames", "if", "then", "else", "not"].includes(key)) visit(child, at);
      if (["allOf", "anyOf", "oneOf"].includes(key) || key === "items" && Array.isArray(child)) {
        if (!Array.isArray(child) || child.length === 0) problems.push(`${at} must contain schemas`);
        else child.forEach((schema, index) => visit(schema, `${at}/${index}`));
      } else if (key === "items") visit(child, at);
      if (["definitions", "properties", "patternProperties", "dependencies"].includes(key)) {
        if (!object(child)) problems.push(`${at} must be an object`);
        else for (const [name, schema] of Object.entries(child)) {
          if (key === "patternProperties" && !regex(name)) problems.push(`${at}/${name} must be a regular expression`);
          if (key === "dependencies" && Array.isArray(schema)) { if (!strings(schema)) problems.push(`${at}/${name} must contain unique strings`); }
          else visit(schema, `${at}/${name}`);
        }
      }
    }
  };
  if (!object(value) || value.$schema !== dialect) problems.push("Payload must declare its registered JSON Schema dialect");
  visit(value, "");
  return problems;
}

/** 🛡️ Validates descriptor authority against admitted, non-linked nodes within one exact operation owner. */
export function mutationPayloadSchemaProblems(owner: string, pointer: unknown, node: (path: string) => Readonly<{ kind: string; content?: string; repositoryBoundary?: boolean }>, dialect = "http://json-schema.org/draft-07/schema#"): string[] {
  const local = (path: string): boolean => path.length > 0 && path === path.normalize("NFC") && !/^[\/]|[\\:#\u0000-\u001F\u007F\u2028\u2029]/u.test(path) && path.split("/").every((part) => part !== "" && part !== "." && part !== "..");
  if (!local(owner) || typeof pointer !== "string" || !local(pointer) || !pointer.endsWith(".json")) return ["Payload schema must be an exact owner-relative JSON path without traversal, fragments, or absolute coordinates"];
  const target = `${owner}/${pointer}`, parts = target.split("/");
  for (let index = 1; index < parts.length; index++) {
    const path = parts.slice(0, index).join("/"), observed = node(path);
    if (observed.kind !== "directory" || observed.repositoryBoundary) return [`Payload schema ancestor ${path} is not an admitted regular owner directory`];
  }
  const observed = node(target);
  if (observed.kind !== "file" || observed.repositoryBoundary || typeof observed.content !== "string") return [`Payload schema ${target} is not an admitted regular file`];
  try { const document = JSON.parse(observed.content); return [...jsonDocumentDuplicateKeys(observed.content), ...mutationPayloadSchemaDocumentProblems(document, dialect)]; }
  catch { return [`Payload schema ${target} is not valid JSON`]; }
}

export const fileKindIdsByFilename = new WeakMap<Taxonomy, Map<string, string | null>>();

export function longestExtensionFileKindId(filename: string, taxonomy: Taxonomy): string | null {
  const terminalCandidates = Object.entries(taxonomy.fileKinds).flatMap(([kindId, kind]) => kind.extensionChains.filter((extension) => filename.endsWith(extension)).map((extension) => ({ kindId, extension })));
  const longest = Math.max(0, ...terminalCandidates.map((candidate) => candidate.extension.length));
  const longestKindIds = [...new Set(terminalCandidates.filter((candidate) => candidate.extension.length === longest).map((candidate) => candidate.kindId))];
  return longestKindIds.length === 1 ? longestKindIds[0]! : null;
}

export const scopedFileKindIds = new WeakMap<Taxonomy, Map<string, string | null>>();

export function resolveScopedFileKindId(normalized: string, taxonomy: Taxonomy, context: Readonly<{ parentDirectoryKindId?: string }>): string | null {
  const filename = normalized.slice(normalized.lastIndexOf("/") + 1);
  const matches = Object.entries(taxonomy.scopedFileKinds).filter(([, spec]) => taxonomyPathPatternMatches(normalized, spec.pathPattern)
    && (spec.parentDirectoryKindId === undefined || spec.parentDirectoryKindId === context.parentDirectoryKindId)
    && new RegExp(spec.sourceFilenamePattern, "u").test(filename)
    && spec.extensionChains.some((extension) => filename.endsWith(extension)));
  return matches.length === 1 ? matches[0]![0] : null;
}
