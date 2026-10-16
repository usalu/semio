/** 📇️ The schema-document index the mutation derive reads: which production schema document answers each `$id` under every plugin and module directory. */

import Ajv from "ajv";
import { readFileSync, readdirSync, type Dirent } from "node:fs";
import { join, relative, sep } from "node:path";

const SCHEMA_INDEX_SCHEMA = "semio.dsl.mutation-schema-document-index/v1";
const SCHEMA_INDEX_EXCLUDED = new Set(["target", "node_modules", "dist", "🗑️generated", "📦️packages"]);
const SCHEMA_INDEX_COLLECTIONS = new Set(["🧫️fixtures", "🧪️fixtures", "🧫️examples", "🧪️examples", "🧪️tests"]);

/** 🧫️ An example collection keeps its name when that directory is itself a module. */
function schemaIndexExampleCollection(segments: readonly string[]): boolean {
  return segments.some((part, index) => SCHEMA_INDEX_COLLECTIONS.has(part) && segments[index - 1] !== "🔨️modules");
}

/** 🗂️ Directory entries in UTF-8 order, the order that makes a duplicate `$id` pick one path on every platform. */
function schemaIndexEntries(directory: string): Dirent[] {
  try {
    return readdirSync(directory, { withFileTypes: true }).sort((left, right) => Buffer.compare(Buffer.from(left.name), Buffer.from(right.name)));
  } catch {
    return [];
  }
}

/** 🧭️ Workspace-relative path with forward slashes. */
function schemaIndexRelative(repoRoot: string, path: string): string {
  const relativePath = relative(repoRoot, path);
  if (relativePath.startsWith("..") || relativePath === "") throw new Error(`schema index path escapes the workspace: ${path}`);
  return relativePath.split(sep).join("/").normalize("NFC");
}

/** 🗂️ `$id` to absolute path for the production schema documents under `root`, excluding example collections before they are read. */
export function schemaDocumentIndex(root: string): Map<string, string> {
  const segments = root.split(sep).filter((part) => part.length > 0);
  const index = new Map<string, string>();
  if (schemaIndexExampleCollection(segments)) return index;
  const walk = (directory: string, directorySegments: readonly string[], schema: boolean): void => {
    for (const entry of schemaIndexEntries(directory)) {
      if (entry.isSymbolicLink()) continue;
      const name = entry.name.normalize("NFC");
      const next = [...directorySegments, name];
      const path = join(directory, entry.name);
      if (entry.isDirectory()) {
        if (name.startsWith(".") || SCHEMA_INDEX_EXCLUDED.has(name) || schemaIndexExampleCollection(next)) continue;
        walk(path, next, schema || name === "🧬️schema");
      } else if (schema && entry.isFile() && name.endsWith(".json")) {
        let id: unknown;
        try { id = JSON.parse(readFileSync(path, "utf8")).$id; } catch { id = undefined; }
        if (typeof id !== "string" || id.length === 0) continue;
        if (!index.has(id)) index.set(id, path);
      }
    }
  };
  walk(root, segments, segments.includes("🧬️schema"));
  return index;
}

/** 🧭️ The search root the derive selects for a schema directory: its plugin, else its module, else the directory that owns the schema. */
function schemaSearchRoot(schemaDirectory: string): string | undefined {
  const segments = schemaDirectory.split("/");
  const under = (parent: string): string | undefined => {
    const index = segments.findLastIndex((segment, position) => position > 0 && segments[position - 1] === parent && segment.length > 0);
    return index >= 0 ? segments.slice(0, index + 1).join("/") : undefined;
  };
  const owner = segments.slice(0, -1).join("/");
  return under("🔌️plugins") ?? under("🔨️modules") ?? (owner.length > 0 ? owner : undefined);
}

/** 🧭️ Every plugin and module directory a leaf can select as its search root. */
function schemaIndexRoots(repoRoot: string): string[] {
  const roots: string[] = [];
  const walk = (directory: string, segments: readonly string[]): void => {
    for (const entry of schemaIndexEntries(directory)) {
      if (!entry.isDirectory() || entry.isSymbolicLink()) continue;
      const name = entry.name.normalize("NFC");
      const next = [...segments, name];
      if (name.startsWith(".") || SCHEMA_INDEX_EXCLUDED.has(name) || schemaIndexExampleCollection(next)) continue;
      const path = join(directory, entry.name);
      if (name === "🔌️plugins" || name === "🔨️modules") {
        for (const child of schemaIndexEntries(path)) {
          if (!child.isDirectory() || child.isSymbolicLink()) continue;
          const childName = child.name.normalize("NFC");
          const childSegments = [...next, childName];
          if (childName.startsWith(".") || SCHEMA_INDEX_EXCLUDED.has(childName) || schemaIndexExampleCollection(childSegments)) continue;
          roots.push(childSegments.join("/"));
        }
      }
      if (name === "🧬️schema" && schemaIndexEntries(path).some((child) => child.isDirectory() && !child.isSymbolicLink() && child.name.normalize("NFC") === "🧬️mutations")) {
        const owner = schemaSearchRoot(next.join("/"));
        if (owner !== undefined && !schemaIndexExampleCollection(owner.split("/"))) roots.push(owner);
      }
      walk(path, next);
    }
  };
  walk(repoRoot, []);
  return [...new Set(roots)].sort();
}

/** 🧬️ The schema-document index the mutation derive reads, validated against `MutationSchemaDocumentIndexV1`. */
export function mutationSchemaDocumentIndexProjection(repoRoot: string): string {
  const roots: Record<string, Record<string, string>> = {};
  for (const root of schemaIndexRoots(repoRoot)) {
    const documents: Record<string, string> = {};
    for (const [id, path] of [...schemaDocumentIndex(join(repoRoot, root))].sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0)) documents[id] = schemaIndexRelative(repoRoot, path);
    if (Object.keys(documents).length > 0) roots[root] = documents;
  }
  const projection = { schema: SCHEMA_INDEX_SCHEMA, roots };
  const document = JSON.parse(readFileSync(join(import.meta.dirname, "../🧬️schema/🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(document);
  const validate = ajv.getSchema(`${document.$id}#/$defs/MutationSchemaDocumentIndexV1`)!;
  if (!validate(projection)) throw new Error(`schema document index is outside its schema: ${JSON.stringify(validate.errors)}`);
  return `${JSON.stringify(projection, null, 2)}\n`;
}
