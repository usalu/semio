/** 🦀️ Renders the Rust structs of a schema module from its JSON Schema and splices them into the module's `🦀️.rs`, leaving every other item (impls, enums, mounts) untouched: the payload struct of a mutation leaf (the leaf template) and every record struct a schema annotates with `x-semio-rust.derive` (root, snapshot, diff and shared records).
 *
 * Extension keywords: `x-semio-rust` on a record (`doc`, `derive`, `attributes`, `name`) and on a property (`doc`, `attributes`, `type`, `name`); `x-semio-dsl` on a record (`keyword`, `id`, `layout`) and on a property (`unit`, `table`, `statements`, `block`) rendered as `#[dsl(..)]`; `x-semio-state` on a property rendered as `#[state(..)]`. A property states a Rust type that is not derivable (a crate path, an enum) with `x-semio-rust: { "type": "…" }`.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🧬️twin-projection/🟦️.ts
 */
import { sep } from "node:path";
import { definitionKey, splitRef, type Node, type SchemaModule } from "../🧩️model/🟦️.ts";

//#region 🔖️Templates
/** 🧱️ The attribute lines every leaf payload struct carries, then the derive-free `pub struct`. */
const LEAF_ATTRIBUTES: readonly string[] = [
  "#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]",
  "#[canonical_json(owner = semio_framework_pack_json)]",
  '#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]',
  "#[mutation_leaf(contract = ::protocol)]",
  '#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]',
  '#[value(rename_all = "camelCase")]',
];

/** 🐍️ A wire name as the Rust field name (`newMEdY` to `new_m_ed_y`: every capital opens a word). */
export const snake = (name: string): string => name.replace(/([A-Z])/gu, "_$1").toLowerCase();

/** 🧭️ What a type derivation needs to follow a reference. */
export interface RustScope {
  readonly module: SchemaModule;
  readonly index: ReadonlyMap<string, SchemaModule>;
}
//#endregion 🔖️Templates

//#region 🔖️Types
/** 🔢️ The Rust integer type that carries the bounds of an integer schema. */
function integerType(node: Node): string {
  const { minimum, maximum } = node;
  if (typeof minimum === "number" && minimum >= 0) {
    if (typeof maximum !== "number") return "usize";
    return maximum <= 255 ? "u8" : maximum <= 65535 ? "u16" : maximum <= 4294967295 ? "u32" : "u64";
  }
  return "i64";
}

/** 🦀️ The Rust name of a record: the annotated name, else its definition key (the title for a root). */
export function rustName(module: SchemaModule, key: string | undefined): string {
  const node: Node | undefined = key === undefined ? module.schema : module.definitions.get(key);
  const named = node?.["x-semio-rust"]?.name;
  return typeof named === "string" ? named : (key ?? String(module.schema.title));
}

/** 🔗️ The Rust name a `$ref` names (a record of this or a sibling module), or `undefined` when it cannot be followed. */
function referenceName(ref: string, scope: RustScope | undefined): string | undefined {
  if (scope === undefined) return undefined;
  const { id, pointer } = splitRef(ref);
  const owner = id === "" ? scope.module : scope.index.get(id);
  const key = definitionKey(pointer);
  return owner === undefined || key === undefined || !owner.definitions.has(key) ? undefined : rustName(owner, key);
}

/** 🕳️ True when a property admits `null`: a type list with `null` or an `anyOf|oneOf` with a null branch. */
export function admitsNull(node: Node): boolean {
  const kinds: unknown[] = Array.isArray(node.type) ? node.type : [node.type];
  const branches: Node[] = node.anyOf ?? node.oneOf ?? [];
  return kinds.includes("null") || branches.some((branch) => branch.type === "null");
}

/** 🧬️ The Rust type of a property without its `Option` wrapper: the annotated path, else what the schema pins. */
function baseType(node: Node, scope: RustScope | undefined): string | undefined {
  const annotated = node["x-semio-rust"]?.type;
  if (typeof annotated === "string") return annotated;
  if (typeof node.$ref === "string") return referenceName(node.$ref, scope);
  const branches: Node[] | undefined = node.anyOf ?? node.oneOf;
  if (branches !== undefined) {
    const rest = branches.filter((branch) => branch.type !== "null");
    return rest.length === 1 ? baseType(rest[0]!, scope) : undefined;
  }
  const types: string[] = Array.isArray(node.type) ? node.type : [node.type];
  const kind = types.find((type) => type !== "null");
  if (kind === "string") return node.enum === undefined ? "String" : undefined;
  if (kind === "number") return node.format === "float" ? "f32" : "f64";
  if (kind === "integer") return integerType(node);
  if (kind === "boolean") return "bool";
  if (kind === "array") {
    const item = node.items === undefined || Array.isArray(node.items) ? undefined : baseType(node.items, scope);
    return item === undefined ? undefined : `Vec<${admitsNull(node.items) ? `Option<${item}>` : item}>`;
  }
  if (kind === "object" && node.properties === undefined && typeof node.additionalProperties === "object") {
    const value = baseType(node.additionalProperties, scope);
    return value === undefined ? undefined : `BTreeMap<String, ${value}>`;
  }
  return undefined;
}

/** 🗂️ The Rust type of a property: the base type, `Option`-wrapped when the property is optional or nullable. */
export function rustType(node: Node, required: boolean, scope?: RustScope): string | undefined {
  const base = baseType(node, scope);
  if (base === undefined) return undefined;
  return required && !admitsNull(node) ? base : `Option<${base}>`;
}
//#endregion 🔖️Types

//#region 🔖️Rendering
/** 🏷️ One `#[dsl(..)]` line from a keyword object (`{ unit: "m" }`, `{ table: true }`), or `undefined` when the object states nothing. */
function dslAttribute(dsl: Node | undefined): string | undefined {
  if (typeof dsl !== "object" || dsl === null) return undefined;
  const parts = Object.entries(dsl).flatMap(([key, value]) => (value === true ? [key] : typeof value === "string" ? [`${key} = ${JSON.stringify(value)}`] : []));
  return parts.length === 0 ? undefined : `#[dsl(${parts.join(", ")})]`;
}

const docLines = (note: Node | undefined, indent: string): string[] => (Array.isArray(note?.doc) ? note.doc.map((line: string) => `${indent}///${line === "" ? "" : ` ${line}`}`) : []);

const stringList = (value: unknown): string[] => (Array.isArray(value) ? value.filter((entry): entry is string => typeof entry === "string") : []);

/** 📝️ The properties of a record as `pub name: type,` field blocks, or `undefined` when a property has no derivable Rust type. */
function renderFields(node: Node, scope: RustScope, skip: (key: string, member: Node) => boolean, record?: string): string[] | undefined {
  const required = new Set<string>(node.required ?? []);
  const lines: string[] = [];
  for (const [key, member] of Object.entries<Node>(node.properties ?? {})) {
    if (skip(key, member)) continue;
    const type = rustType(member, required.has(key), scope);
    if (type === undefined && record !== undefined) throw new Error(`[schema twins] ${record}.${key} has no derivable Rust type (an inline enum or an unresolved reference): state it with "x-semio-rust": { "type": "<Rust type>" }.`);
    if (type === undefined) return undefined;
    const note = member["x-semio-rust"];
    const state = typeof member["x-semio-state"] === "string" ? [`#[state(${member["x-semio-state"]})]`] : [];
    const attributes = [dslAttribute(member["x-semio-dsl"]), ...state, ...stringList(note?.attributes)].filter((line): line is string => line !== undefined);
    lines.push(...docLines(note, "    "), ...attributes.map((line) => `    ${line}`), `    pub ${typeof note?.name === "string" ? note.name : snake(key)}: ${type},`);
  }
  return lines;
}

/** 📝️ The generated struct of a leaf payload (the leaf template), or `undefined` when a property has no derivable Rust type. */
export function renderLeafStruct(module: SchemaModule, index: ReadonlyMap<string, SchemaModule>): string | undefined {
  const fields = renderFields(module.schema, { module, index }, (key, member) => key === "mutation" && member.const !== undefined);
  return fields === undefined ? undefined : [...LEAF_ATTRIBUTES, `pub struct ${module.schema.title} {`, ...fields, "}"].join("\n");
}

/** 📸️ A snapshot module (or a folder below it): its records are document state and never derive `Default`, unlike diff patches and inference roots whose empty value means "no change". */
const SNAPSHOT_MODULE = /\/📸️snapshot(?:\/|$)/u;

/** 📝️ The generated struct of an annotated record (`key` undefined for the document root), or `undefined` when the record is not annotated with a derive list or a property has no derivable Rust type. */
export function renderRecordStruct(module: SchemaModule, index: ReadonlyMap<string, SchemaModule>, key: string | undefined): string | undefined {
  const node: Node | undefined = key === undefined ? module.schema : module.definitions.get(key);
  const note = node?.["x-semio-rust"];
  const derive = stringList(note?.derive);
  if (node === undefined || derive.length === 0) return undefined;
  if (derive.includes("Default") && SNAPSHOT_MODULE.test(module.dir.split(sep).join("/"))) throw new Error(`[schema twins] ${module.jsonPath}: snapshot record ${rustName(module, key)} derives Default; snapshot entities are constructed explicitly (NO-DEFAULTS).`);
  const fields = renderFields(node, { module, index }, () => false, `${module.jsonPath} ${rustName(module, key)}`);
  if (fields === undefined) return undefined;
  const schema = typeof note?.artifactSchema === "string" ? `#[artifact_schema(id = ${JSON.stringify(note.artifactSchema)})]` : undefined;
  const header = [...docLines(note, ""), `#[derive(${derive.join(", ")})]`, ...stringList(note?.attributes), dslAttribute(node["x-semio-dsl"]), schema].filter((line): line is string => line !== undefined);
  return [...header, `pub struct ${rustName(module, key)} {`, ...fields, "}"].join("\n");
}

const structPattern = (name: string): RegExp => new RegExp(`(?:^[ \\t]*(?:///[^\\n]*|#\\[[^\\n]*\\])\\n)*^pub struct ${name} \\{\\n[\\s\\S]*?^\\}`, "mu");

/** 🪡️ Replaces the doc lines, attributes and body of `pub struct <name>` in a Rust source with the generated struct; `undefined` when the source declares no such struct. */
export function spliceStruct(source: string, name: string, generated: string): string | undefined {
  const pattern = structPattern(name);
  return pattern.test(source) ? source.replace(pattern, () => generated) : undefined;
}
/** ➕️ Puts a generated struct into a Rust source that declares none of that name: after the struct of `after` when the source declares it, else at the end of the file. */
export function insertStruct(source: string, after: string | undefined, generated: string): string {
  const previous = after === undefined ? null : structPattern(after).exec(source);
  if (previous === null) return `${source.replace(/\n*$/u, "")}\n\n${generated}\n`;
  const end = previous.index + previous[0].length;
  return `${source.slice(0, end)}\n\n${generated}${source.slice(end)}`;
}
//#endregion 🔖️Rendering

//#region 🔖️Harvest
/** 🧾️ One field of a committed struct. */
interface ParsedField {
  readonly docs: string[];
  readonly attributes: string[];
  readonly name: string;
  readonly type: string;
}

/** 🧾️ A committed struct taken apart. */
interface ParsedStruct {
  readonly docs: string[];
  readonly attributes: string[];
  readonly fields: ParsedField[];
}

/** 🔬️ A committed `pub struct <name>` as doc lines, attribute lines and `pub field: type,` fields, or `undefined` when it holds anything else (comments, blank lines, multi-line attributes or types). */
export function parseStruct(source: string, name: string): ParsedStruct | undefined {
  const found = structPattern(name).exec(source);
  if (found === null) return undefined;
  const lines = found[0].split("\n");
  const header: string[] = [];
  let at = 0;
  for (; at < lines.length && !lines[at]!.startsWith("pub struct "); at += 1) header.push(lines[at]!);
  if (!lines[at]!.endsWith(" {") || lines.at(-1) !== "}") return undefined;
  const parsed: ParsedStruct = { docs: header.filter((line) => line.startsWith("///")).map((line) => line.replace(/^\/\/\/ ?/u, "")), attributes: header.filter((line) => line.startsWith("#[")), fields: [] };
  if (parsed.docs.length + parsed.attributes.length !== header.length) return undefined;
  let docs: string[] = [];
  let attributes: string[] = [];
  for (const line of lines.slice(at + 1, -1)) {
    const text = line.slice(4);
    if (!line.startsWith("    ") || text.startsWith(" ")) return undefined;
    if (text.startsWith("///")) docs.push(text.replace(/^\/\/\/ ?/u, ""));
    else if (text.startsWith("#[")) attributes.push(text);
    else {
      const field = /^pub (\w+): (.+),$/u.exec(text);
      if (field === null) return undefined;
      parsed.fields.push({ docs, attributes, name: field[1]!, type: field[2]! });
      docs = [];
      attributes = [];
    }
  }
  return docs.length + attributes.length === 0 ? parsed : undefined;
}

/** 🔎️ The `key = "value"` and bare-flag pairs of a `#[dsl(..)]` attribute as a keyword object, or `undefined` when it states anything else. */
function parseDsl(attribute: string): Node | undefined {
  const body = /^#\[dsl\((.*)\)\]$/u.exec(attribute)?.[1];
  if (body === undefined) return undefined;
  const result: Node = {};
  for (const part of body.split(/, (?=\w+(?: =|,|$))/u)) {
    const pair = /^(\w+)(?: = "([^"]*)")?$/u.exec(part);
    if (pair === null) return undefined;
    result[pair[1]!] = pair[2] ?? true;
  }
  return result;
}

const compact = (text: string): string => text.replace(/\s+/gu, " ").trim();

/** 📏️ A struct reading that ignores attribute order (the derive stays first) and whitespace. */
function reading(struct: ParsedStruct): string {
  const sorted = (attributes: readonly string[]): string[] => [...attributes].sort();
  const derive = struct.attributes.filter((line) => line.startsWith("#[derive("));
  const rest = struct.attributes.filter((line) => !line.startsWith("#[derive("));
  return JSON.stringify({ docs: struct.docs, derive: derive.map(compact), attributes: sorted(rest).map(compact), fields: struct.fields.map((field) => ({ docs: field.docs, attributes: sorted(field.attributes).map(compact), name: field.name, type: compact(field.type) })) });
}

/** 🏷️ The `x-semio-rust`, `x-semio-dsl` and `x-semio-state` annotations that make a record's derived struct equal its committed struct, per property by position. The leaf template carries no annotation of its own: `leaf` limits the harvest to the property names and types. */
interface Harvest {
  readonly record: Node;
  readonly properties: Map<string, Node>;
}

/** 🌾️ Reads the annotations of a committed record struct, or `undefined` when the struct cannot be reproduced from a schema (a field count that differs from the property count, a hand-written layout, an attribute the vocabulary does not carry). The derive list must be a single `#[derive(..)]`. */
function harvest(node: Node, source: string, name: string, scope: RustScope): Harvest | undefined {
  const parsed = parseStruct(source, name);
  const keys = Object.keys(node.properties ?? {});
  if (parsed === undefined || parsed.fields.length !== keys.length) return undefined;
  const derive = parsed.attributes.filter((line) => line.startsWith("#[derive("));
  const deriveBody = derive.length === 1 ? /^#\[derive\((.*)\)\]$/u.exec(derive[0]!)?.[1] : undefined;
  if (deriveBody === undefined) return undefined;
  const record: Node = { doc: parsed.docs, derive: deriveBody.split(", ") };
  const dsl: Node = {};
  const attributes: string[] = [];
  for (const line of parsed.attributes) {
    if (line.startsWith("#[derive(")) continue;
    const parsedDsl = parseDsl(line);
    const schema = /^#\[artifact_schema\(id = "([^"]*)"\)\]$/u.exec(line)?.[1];
    if (parsedDsl !== undefined) Object.assign(dsl, parsedDsl);
    else if (schema !== undefined) record.artifactSchema = schema;
    else attributes.push(line);
  }
  if (attributes.length > 0) record.attributes = attributes;
  const required = new Set<string>(node.required ?? []);
  const properties = new Map<string, Node>();
  let misaligned = false;
  keys.forEach((key, at) => {
    const field = parsed.fields[at]!;
    const member = node.properties[key] as Node;
    const note: Node = {};
    if (field.docs.length > 0) note.doc = field.docs;
    if (snake(key) !== field.name) note.name = field.name;
    if (snake(key).replaceAll("_", "") !== field.name.replaceAll("_", "")) misaligned = true;
    const derived = rustType({ ...member, "x-semio-rust": undefined }, required.has(key), scope);
    if (derived === undefined || compact(derived) !== compact(field.type)) {
      const optional = !(required.has(key) && !admitsNull(member));
      note.type = optional ? field.type.replace(/^Option<(.*)>$/u, "$1") : field.type;
    }
    const memberDsl: Node = {};
    let state: string | undefined;
    const rest: string[] = [];
    for (const line of field.attributes) {
      const stated = /^#\[state\((\w+)\)\]$/u.exec(line);
      const parsedDsl = parseDsl(line);
      if (stated !== null) state = stated[1];
      else if (parsedDsl !== undefined) Object.assign(memberDsl, parsedDsl);
      else rest.push(line);
    }
    if (rest.length > 0) note.attributes = rest;
    const out: Node = {};
    if (Object.keys(note).length > 0) out["x-semio-rust"] = note;
    if (Object.keys(memberDsl).length > 0) out["x-semio-dsl"] = memberDsl;
    if (state !== undefined) out["x-semio-state"] = state;
    properties.set(key, out);
  });
  if (Object.keys(dsl).length > 0) record.dsl = dsl;
  return misaligned ? undefined : { record, properties };
}

/** 🏷️ Applies a harvest to a copy of a record node. */
function annotated(node: Node, found: Harvest): Node {
  const { dsl, ...note } = found.record;
  const properties = Object.fromEntries(
    Object.entries<Node>(node.properties ?? {}).map(([key, member]) => {
      const extra = found.properties.get(key) ?? {};
      const { "x-semio-state": state, ...rest } = extra;
      return [key, { ...member, ...(state === undefined || typeof member["x-semio-state"] === "string" ? {} : { "x-semio-state": state }), ...rest, ...(rest["x-semio-rust"] === undefined ? {} : { "x-semio-rust": { ...member["x-semio-rust"], ...rest["x-semio-rust"] } }) }];
    }),
  );
  return { ...node, "x-semio-rust": { ...node["x-semio-rust"], ...note }, ...(dsl === undefined ? {} : { "x-semio-dsl": dsl }), properties };
}

/** 🏷️ The annotated copy of every record of a module whose committed struct in `source` the schema can reproduce, as `[key | undefined for the root, node]`; a record that already carries its Rust annotations is the source and is left alone, and a record whose struct cannot be reproduced stays unannotated and untouched. */
export function annotateRecords(module: SchemaModule, index: ReadonlyMap<string, SchemaModule>, source: string, claimed: Set<string>): { readonly key: string | undefined; readonly node: Node }[] {
  const scope: RustScope = { module, index };
  const candidates: { key: string | undefined; node: Node }[] = [];
  if (module.schema.properties !== undefined) candidates.push({ key: undefined, node: module.schema });
  for (const [key, node] of module.definitions) if (typeof node === "object" && node.properties !== undefined && (node.type === "object" || node.type === undefined)) candidates.push({ key, node });
  const result: { key: string | undefined; node: Node }[] = [];
  for (const { key, node } of candidates) {
    const name = rustName(module, key);
    if (claimed.has(name)) continue;
    if (node["x-semio-rust"]?.derive !== undefined) {
      claimed.add(name);
      continue;
    }
    const found = harvest(node, source, name, scope);
    const parsed = found === undefined ? undefined : parseStruct(source, name);
    if (found === undefined || parsed === undefined) continue;
    const copy = annotated(node, found);
    const probe = key === undefined ? { ...module, schema: copy } : { ...module, schema: { ...module.schema, $defs: { ...module.schema.$defs, [key]: copy } }, definitions: new Map(module.definitions).set(key, copy) };
    const text = renderRecordStruct(probe, index, key);
    const again = text === undefined ? undefined : parseStruct(text, name);
    if (again === undefined || reading(again) !== reading(parsed)) continue;
    claimed.add(name);
    result.push({ key, node: copy });
  }
  return result;
}

/** 🧾️ The `pub <name>: <type>,` fields of a committed struct, in order. */
export function structFields(source: string, title: string): { readonly name: string; readonly type: string }[] | undefined {
  const body = new RegExp(`^pub struct ${title} \\{\\n([\\s\\S]*?)^\\}`, "mu").exec(source)?.[1];
  if (body === undefined) return undefined;
  return [...body.matchAll(/^ {4}pub (\w+): (.*),$/gmu)].map((match) => ({ name: match[1]!, type: match[2]! }));
}

/** 🏷️ The `x-semio-rust` annotations a leaf payload schema needs so that its derived struct equals the committed one: per property, the Rust name and type where the derivation differs. Properties are matched by position. */
export function annotationsFor(module: SchemaModule, index: ReadonlyMap<string, SchemaModule>, source: string): Map<string, { name?: string; type?: string }> | undefined {
  const fields = structFields(source, String(module.schema.title));
  const required = new Set<string>(module.schema.required ?? []);
  const keys = Object.keys(module.schema.properties ?? {}).filter((key) => !(key === "mutation" && module.schema.properties[key].const !== undefined));
  if (fields === undefined || fields.length !== keys.length) return undefined;
  const notes = new Map<string, { name?: string; type?: string }>();
  keys.forEach((key, at) => {
    const node = { ...module.schema.properties[key], "x-semio-rust": undefined } as Node;
    const derived = rustType(node, required.has(key), { module, index });
    const note: { name?: string; type?: string } = {};
    if (snake(key) !== fields[at]!.name) note.name = fields[at]!.name;
    if (derived !== fields[at]!.type) note.type = fields[at]!.type.replace(/^Option<(.*)>$/u, (whole, inner: string) => (required.has(key) && !admitsNull(node) ? whole : inner));
    if (note.name !== undefined || note.type !== undefined) notes.set(key, note);
  });
  return notes;
}
//#endregion 🔖️Harvest
