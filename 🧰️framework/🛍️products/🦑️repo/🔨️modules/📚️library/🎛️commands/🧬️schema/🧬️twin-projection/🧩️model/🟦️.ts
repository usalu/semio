/** 🧩️ The schema-first model of the twin projection: every schema module under a scope, addressed by `$id`, with the twins it provides. The JSON Schema document is the only source; no twin is read to produce another.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🧬️twin-projection/🟦️.ts
 */
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, join, relative, sep } from "node:path";

//#region 🔖️Documents
/** 🧱️ A JSON object read from a schema document. */
export type Node = { [key: string]: any };

/** 🍃️ The twin leaf file name of every projected language. */
export const TWIN_FILE = { typescript: "🟦️.ts", rust: "🦀️.rs", protobuf: "🛰️.proto", graphql: "🔗️.graphql" } as const;

/** 🗣️ A projected language. */
export type Language = keyof typeof TWIN_FILE;

/** 🧬️ A facet of a schema module: a projected language, or the layout of the JSON Schema document itself (`json`), which is embedded verbatim in the crates and counts against their source boundary. */
export type Facet = Language | "json";

/** 🗂️ Folders no schema module lives in. */
const SKIPPED = new Set(["node_modules", "target", "🗑️generated", "🧫️fixtures", "🧪️fixtures", "🧪️tests", "📦️packages", ".git"]);

/** 📄️ One schema module: its document, its folder and the twins it provides. */
export interface SchemaModule {
  readonly dir: string;
  readonly jsonPath: string;
  readonly schema: Node;
  readonly id: string;
  readonly twins: readonly Language[];
  readonly definitions: ReadonlyMap<string, Node>;
}

/** 🧭️ A repository-relative path with forward slashes. */
export const portable = (root: string, path: string): string => relative(root, path).split(sep).join("/");

/** 📚️ The `$defs` and `definitions` of a document in the order they are written. */
export function definitionsOf(schema: Node): Map<string, Node> {
  return new Map<string, Node>([...Object.entries<Node>(schema.definitions ?? {}), ...Object.entries<Node>(schema.$defs ?? {})]);
}

/** 🔎️ True for a schema document of a twin family: a draft-07 document that states a structure. */
const isSchema = (schema: unknown): schema is Node =>
  typeof schema === "object" && schema !== null && typeof (schema as Node).$schema === "string" && ("properties" in (schema as Node) || "$defs" in (schema as Node) || "definitions" in (schema as Node) || "oneOf" in (schema as Node) || "anyOf" in (schema as Node));

/** 🌳️ Reads every schema module below `root`, in UTF-8 path order, skipping folders no module lives in. */
export function loadModules(root: string): SchemaModule[] {
  const modules: SchemaModule[] = [];
  const walk = (dir: string): void => {
    const entries = readdirSync(dir, { withFileTypes: true }).sort((left, right) => Buffer.compare(Buffer.from(left.name), Buffer.from(right.name)));
    const jsonPath = join(dir, "🔣️.json");
    if (entries.some((entry) => entry.name === "🔣️.json" && entry.isFile())) {
      let schema: unknown;
      try {
        schema = JSON.parse(readFileSync(jsonPath, "utf8"));
      } catch {
        schema = undefined;
      }
      if (isSchema(schema)) {
        const twins = (Object.keys(TWIN_FILE) as Language[]).filter((language) => existsSync(join(dir, TWIN_FILE[language])));
        modules.push({ dir, jsonPath, schema, id: typeof schema.$id === "string" ? schema.$id : "", twins, definitions: definitionsOf(schema) });
      }
    }
    for (const entry of entries) if (entry.isDirectory() && !entry.name.startsWith(".") && !SKIPPED.has(entry.name)) walk(join(dir, entry.name));
  };
  walk(root);
  return modules;
}

/** 🗺️ The modules of a scope addressed by `$id`; the first module in path order owns a duplicate id. */
export function indexModules(modules: readonly SchemaModule[]): Map<string, SchemaModule> {
  const index = new Map<string, SchemaModule>();
  for (const module of modules) if (module.id !== "" && !index.has(module.id)) index.set(module.id, module);
  return index;
}
//#endregion 🔖️Documents

//#region 🔖️Serialization
const inline = (value: unknown): string => JSON.stringify(value);

/** 🗜️ The compact layout of a schema document: one line per top-level key and per definition. */
function compactLayout(document: Node): string {
  const rows = Object.entries(document).map(([key, value]) =>
    (key === "$defs" || key === "definitions") && typeof value === "object" && value !== null && !Array.isArray(value)
      ? `  ${inline(key)}:{\n${Object.entries(value).map(([name, node]) => `    ${inline(name)}:${inline(node)}`).join(",\n")}\n  }`
      : `  ${inline(key)}:${inline(value)}`,
  );
  return `{\n${rows.join(",\n")}\n}\n`;
}

/** 📏️ The most lines a schema document may take in the indented layout; a longer document is laid out compact (one line per definition). The embedded JSON competes with its four twins for the 4000 active lines and 262144 bytes of the repo source boundary, so the layout is a function of the document and not of an owner's habit. */
export const SCHEMA_INDENT_LINES = 1500;

/** 💾️ The canonical text of a schema document: compact when `original` already is compact or the indented layout would exceed `SCHEMA_INDENT_LINES`, else two-space indent. Pure and idempotent, so `generate` writes it and `check --strict` compares against it. */
export function serializeSchema(document: Node, original: string): string {
  const indented = `${JSON.stringify(document, null, 2)}\n`;
  const compact = original === compactLayout(JSON.parse(original));
  return compact || indented.split("\n").length > SCHEMA_INDENT_LINES ? compactLayout(document) : indented;
}
//#endregion 🔖️Serialization

//#region 🔖️Pointers
/** 📍️ The node a JSON Pointer names inside `schema` (RFC 6901 unescaping). */
export function pointed(schema: Node, pointer: string): Node | undefined {
  let node: Node | undefined = schema;
  for (const step of pointer.split("/").slice(1)) {
    node = node?.[step.replace(/~1/gu, "/").replace(/~0/gu, "~")];
    if (node === undefined) return undefined;
  }
  return node;
}

/** 🔗️ A `$ref` split into the owning document id (empty for a local ref) and the pointer inside it. */
export function splitRef(ref: string): { readonly id: string; readonly pointer: string } {
  const at = ref.indexOf("#");
  return at < 0 ? { id: ref, pointer: "" } : { id: ref.slice(0, at), pointer: ref.slice(at + 1) };
}

/** 🔑️ The definition key a pointer `/$defs/Name` or `/definitions/Name` names. */
export const definitionKey = (pointer: string): string | undefined => /^\/(?:\$defs|definitions)\/([^/]+)$/u.exec(pointer)?.[1];
//#endregion 🔖️Pointers

//#region 🔖️Annotations
/** 🏷️ The twin annotation a schema node states: `{ emoji, summary, name, doc }` under `x-semio-twin`. */
export const twinNote = (node: Node | undefined): Node => (typeof node?.["x-semio-twin"] === "object" && node["x-semio-twin"] !== null ? node["x-semio-twin"] : {});

/** 🍃️ The descriptor beside a mutation leaf payload: the sibling `🔣️.json` of the folder that holds the payload schema. */
export function leafDescriptor(module: SchemaModule): { readonly kind: string; readonly emoji: string; readonly variant: string } | undefined {
  if (!/\/🧬️mutations\/[^/]+\/🧬️schema$/u.test(module.dir.split(sep).join("/"))) return undefined;
  const path = join(dirname(module.dir), "🔣️.json");
  if (!existsSync(path)) return undefined;
  const descriptor = JSON.parse(readFileSync(path, "utf8")) as Node;
  return typeof descriptor.semanticKind === "string" ? { kind: descriptor.semanticKind, emoji: typeof descriptor.emoji === "string" ? descriptor.emoji : "", variant: typeof descriptor.aggregateVariant === "string" ? descriptor.aggregateVariant : "" } : undefined;
}
//#endregion 🔖️Annotations

/** 🪪️ The node without the properties whose names are no identifier (`$schema`, the document's own meta key): protobuf and GraphQL cannot name them. */
export function withoutMetaProperties(node: Node): Node {
  if (typeof node.properties !== "object" || node.properties === null) return node;
  return { ...node, properties: Object.fromEntries(Object.entries(node.properties).filter(([key]) => /^[A-Za-z_]/u.test(key))) };
}

/** 🏷️ The envelope tag of a leaf payload (`mutation: { const }`) is no payload field of the foreign twins. */
export function withoutTag(node: Node): Node {
  if (typeof node.properties !== "object" || node.properties === null) return node;
  return { ...node, properties: Object.fromEntries(Object.entries(node.properties).filter(([key, value]) => !(key === "mutation" && (value as Node).const !== undefined))), required: Array.isArray(node.required) ? node.required.filter((key: string) => key !== "mutation") : node.required };
}

/** 🔢️ The node with every integer that carries no `format` given the protobuf width its bounds pin (`uint32` within 0..2³²−1, `uint64` from 0, else `int64`). */
export function withIntegerFormats(node: Node): Node {
  if (Array.isArray(node)) return node.map(withIntegerFormats) as unknown as Node;
  if (typeof node !== "object" || node === null) return node;
  const copy: Node = Object.fromEntries(Object.entries(node).map(([key, value]) => [key, key === "properties" || key === "items" || key === "anyOf" || key === "oneOf" || key === "additionalProperties" ? (key === "properties" ? Object.fromEntries(Object.entries(value as Node).map(([name, member]) => [name, withIntegerFormats(member)])) : withIntegerFormats(value as Node)) : value]));
  const kinds: unknown[] = Array.isArray(copy.type) ? copy.type : [copy.type];
  if (kinds.includes("integer") && copy.format === undefined && typeof copy.minimum === "number" && copy.minimum >= 0) copy.format = typeof copy.maximum === "number" && copy.maximum <= 4294967295 ? "uint32" : "uint64";
  return copy;
}

/** 🧺️ A variant of a mutation aggregate: its tag and the record name of its payload. */
export interface AggregateMember {
  readonly variant: string;
  readonly type: string;
  readonly payload: SchemaModule;
}

/** 🧺️ The branches of an aggregate document (`oneOf` of leaf payload references, plain or one-property tagged), or `undefined` when the document is no aggregate. The payload name is the `title` of the referenced leaf schema. */
export function aggregateMembers(module: SchemaModule, index: ReadonlyMap<string, SchemaModule>): AggregateMember[] | undefined {
  const branches: Node[] | undefined = module.schema.oneOf ?? module.schema.anyOf;
  if (branches === undefined || branches.length === 0 || module.schema.properties !== undefined) return undefined;
  const members: AggregateMember[] = [];
  for (const branch of branches) {
    const tagged = typeof branch.$ref === "string" ? undefined : Object.keys(branch.properties ?? {});
    const variant = tagged?.length === 1 ? tagged[0]! : undefined;
    const ref = typeof branch.$ref === "string" ? branch.$ref : variant === undefined ? undefined : branch.properties[variant].$ref;
    if (typeof ref !== "string") return undefined;
    const { id, pointer } = splitRef(ref);
    const target = pointer === "" ? index.get(id) : undefined;
    if (target === undefined) return undefined;
    const title = typeof target.schema.title === "string" ? target.schema.title : variant;
    if (title === undefined) return undefined;
    members.push({ variant: variant ?? title, type: title, payload: target });
  }
  return members;
}

/** 🔗️ The node a `$ref` chain ends at, followed through the scope's `$id` index; the node itself when it is no reference or the chain cannot be followed. */
export function resolveNode(node: Node, module: SchemaModule, index: ReadonlyMap<string, SchemaModule>): Node {
  let current = node;
  let owner = module;
  for (let hops = 0; typeof current.$ref === "string" && hops < 12; hops += 1) {
    const { id, pointer } = splitRef(current.$ref);
    const next = id === "" ? owner : index.get(id);
    const target = next === undefined ? undefined : pointer === "" ? next.schema : pointed(next.schema, pointer);
    if (next === undefined || target === undefined) return current;
    owner = next;
    current = target;
  }
  return current;
}
