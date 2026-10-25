/** 🧬️ Schema documents
 *
 * Reads the catalogued schema documents of the repository, resolves the uncatalogued components they reference, and the JSON-pointer and schema-kind helpers every mutation gate shares.
 */

import { readSchemaCatalog } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

//#region 📚️SchemaDocuments
/** 📄️ The JSON object at the repository-relative `path`, or `null` when it is absent or not a JSON object. */
export function readJsonObject(repoRoot: string, path: string): Record<string, unknown> | null {
  try {
    const value: unknown = JSON.parse(readFileSync(join(repoRoot, path), "utf8"));
    return value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : null;
  } catch {
    return null;
  }
}

/** 📚️ Every schema document under a catalogued scope directory, keyed by its `$id` — the cross-document `$ref` universe of the mutation lints. */
export function catalogSchemaDocuments(repoRoot: string): Map<string, Record<string, unknown>> {
  const { catalog } = readSchemaCatalog(repoRoot);
  const documents = new Map<string, Record<string, unknown>>();
  const scopePaths = new Set(Object.values(catalog?.scopes ?? {}).map((scope) => scope.path));
  const index = (directory: string): void => {
    for (const entry of readdirSync(join(repoRoot, directory), { withFileTypes: true })) {
      const path = `${directory}/${entry.name}`;
      if (entry.isDirectory() && !scopePaths.has(path)) index(path);
      else if (entry.isFile() && entry.name.endsWith(".json")) {
        const document = readJsonObject(repoRoot, path);
        if (typeof document?.$id === "string") documents.set(document.$id, document);
      }
    }
  };
  for (const path of scopePaths) if (existsSync(join(repoRoot, path))) index(path);
  return documents;
}
//#endregion 📚️SchemaDocuments

/**
 * 🧭️ Adds to `documents` every schema an already loaded document `$ref`s by an absolute `$id` the catalog does not hold — the
 * uncatalogued component schemas of fixture and test aggregates (`🧫️fixtures/…/🧬️schema/🔣️.json`) — found by `$id` among the
 * `🧬️schema` JSON files of the mutation tree, transitively. The files are read only when a reference is missing.
 */
export function resolveReferencedSchemaDocuments(repoRoot: string, documents: Map<string, Record<string, unknown>>, schemaFiles: readonly string[]): void {
  let index: Map<string, Record<string, unknown>> | null = null;
  const pending = [...documents.values()];
  const references = (node: unknown, found: Set<string>): Set<string> => {
    if (Array.isArray(node)) for (const item of node) references(item, found);
    else if (isRecord(node)) for (const [key, value] of Object.entries(node)) key === "$ref" && typeof value === "string" && /^https?:\/\//u.test(value) ? found.add(value.split("#")[0]!) : references(value, found);
    return found;
  };
  while (pending.length > 0) {
    for (const id of references(pending.pop(), new Set())) {
      if (documents.has(id)) continue;
      index ??= new Map(schemaFiles.flatMap((path) => {
        const document = readJsonObject(repoRoot, path);
        return typeof document?.$id === "string" ? [[document.$id, document] as const] : [];
      }));
      const document = index.get(id);
      if (document === undefined) continue;
      documents.set(id, document);
      pending.push(document);
    }
  }
}

//#region ⚖️MutationPayloadParity
export type SchemaNode = Readonly<{ document: string; pointer: string; node: Record<string, unknown> }>;

export const isRecord = (value: unknown): value is Record<string, unknown> => value !== null && typeof value === "object" && !Array.isArray(value);
export const escapePointer = (segment: string): string => segment.replaceAll("~", "~0").replaceAll("/", "~1");
export const unescapePointer = (segment: string): string => segment.replaceAll("~1", "/").replaceAll("~0", "~");
export const schemaKind = (name: string): string => name.replace(/^[^\p{L}\p{N}]+/u, "");
export const canonicalJson = (value: unknown): string => JSON.stringify(value, (_, entry: unknown) => (isRecord(entry) ? Object.fromEntries(Object.entries(entry).sort(([left], [right]) => left.localeCompare(right))) : entry));
//#endregion ⚖️MutationPayloadParity
