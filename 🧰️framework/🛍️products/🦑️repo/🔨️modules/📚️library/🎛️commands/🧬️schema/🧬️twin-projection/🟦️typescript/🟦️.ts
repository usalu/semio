/** 🟦️ Renders the TypeScript twin of a schema module: one interface or alias per record and its `parse<Record>` wire reader, derived from the JSON Schema alone.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🧬️twin-projection/🟦️.ts
 */
import { dirname, relative } from "node:path";
import { definitionKey, leafDescriptor, pointed, splitRef, twinNote, TWIN_FILE, type Node, type SchemaModule } from "../🧩️model/🟦️.ts";
import { facetOf, readerName, type Dialect } from "../🧭️dialect/🟦️.ts";

//#region 🔖️Shapes
/** 🧱️ A TypeScript type spelling with the reader expression that admits it. */
interface Shape {
  readonly type: string;
  readonly read: string;
}

/** 🧾️ The names a twin borrows from the wire contract and from sibling twins. */
class Uses {
  readonly contract = new Set<string>();
  readonly imports = new Map<string, Set<string>>();

  wire(name: string): string {
    this.contract.add(name);
    return name;
  }

  bring(twin: string, name: string): void {
    const names = this.imports.get(twin) ?? new Set<string>();
    names.add(name);
    this.imports.set(twin, names);
  }
}

/** 🧭️ What a shape needs to resolve a reference: the module it is read in, the scope's `$id` index and the dialect. */
interface Scope {
  readonly origin: SchemaModule;
  readonly module: SchemaModule;
  readonly index: ReadonlyMap<string, SchemaModule>;
  readonly dialect: Dialect;
}

const group = (type: string): string => (/[|&]/u.test(type) ? `(${type})` : type);

/** 🏷️ The exported root record name of a module. */
export const rootName = (module: SchemaModule): string => (typeof module.schema.title === "string" ? module.schema.title : "Record");

/** 🔤️ The TypeScript name a definition of a module takes: the annotated name, else the key, prefixed by the family when the schema withholds the record from TypeScript. */
export function recordName(module: SchemaModule, key: string, dialect: Dialect): string {
  const node = module.definitions.get(key);
  const named = twinNote(node).name;
  if (typeof named === "string") return named;
  const formats = node?.["x-semio-formats"];
  const withheld = Array.isArray(formats) && !formats.includes("🟦️typescript");
  return withheld ? `${dialect.recordPrefixPattern.exec(rootName(module))?.[0] ?? ""}${key}` : key;
}

/** 🗂️ The twin file a module provides. */
export const twinPath = (module: SchemaModule): string => `${module.dir}/${TWIN_FILE.typescript}`;

const wireOf = (uses: Uses, dialect: Dialect, kind: string): string => uses.wire(readerName(dialect, kind));

const jsonShape = (uses: Uses, dialect: Dialect): Shape => ({ type: dialect.jsonType, read: wireOf(uses, dialect, "Json") });

const literal = (values: readonly unknown[], uses: Uses, dialect: Dialect): Shape => ({
  type: values.map((value) => JSON.stringify(value)).join(" | "),
  read: `${wireOf(uses, dialect, "Literal")}(${values.map((value) => JSON.stringify(value)).join(", ")})`,
});

function reference(ref: string, scope: Scope, uses: Uses, context: string): Shape {
  const { id, pointer } = splitRef(ref);
  const owner = id === "" ? scope.module : scope.index.get(id);
  if (owner === undefined) throw new Error(`[schema twins] ${context}: unresolved reference ${ref}`);
  const key = definitionKey(pointer);
  if (pointer !== "" && key === undefined) {
    const target = pointed(owner.schema, pointer);
    if (target === undefined) throw new Error(`[schema twins] ${context}: unresolved pointer ${ref}`);
    return shape(target, { ...scope, module: owner }, uses, context);
  }
  const name = key === undefined ? rootName(owner) : recordName(owner, key, scope.dialect);
  if (owner !== scope.origin) {
    uses.bring(twinPath(owner), name);
    return { type: name, read: `parse${name}` };
  }
  return { type: name, read: `${wireOf(uses, scope.dialect, "Ref")}(() => parse${name})` };
}

function shape(node: Node, scope: Scope, uses: Uses, context: string, named?: string): Shape {
  const { dialect } = scope;
  if (typeof node.$ref === "string") return reference(node.$ref, scope, uses, context);
  const branches = (node.oneOf ?? node.anyOf) as Node[] | undefined;
  if (branches !== undefined) {
    const nulls = branches.filter((branch) => branch.type === "null");
    const rest = branches.filter((branch) => branch.type !== "null");
    if (nulls.length === 1 && rest.length === 1) {
      const inner = shape(rest[0]!, scope, uses, context);
      return { type: `${group(inner.type)} | null`, read: `${wireOf(uses, dialect, "Nullable")}(${inner.read})` };
    }
    return union(branches, scope, uses, context, named);
  }
  if (node.const !== undefined) return literal([node.const], uses, dialect);
  if (Array.isArray(node.enum)) return literal(node.enum, uses, dialect);
  const types: string[] = Array.isArray(node.type) ? node.type : node.type !== undefined ? [node.type] : node.properties !== undefined ? ["object"] : [];
  if (types.length === 2 && types.includes("null")) {
    const inner = shape({ ...node, type: types.find((type) => type !== "null") }, scope, uses, context);
    return { type: `${group(inner.type)} | null`, read: `${wireOf(uses, dialect, "Nullable")}(${inner.read})` };
  }
  if (types.length > 1 && !types.includes("null")) {
    const parts = types.map((type) => shape({ ...node, type }, scope, uses, context));
    return { type: parts.map((part) => group(part.type)).join(" | "), read: `${wireOf(uses, dialect, "Any")}<[${parts.map((part) => part.type).join(", ")}]>(${parts.map((part) => part.read).join(", ")})` };
  }
  if (types.length !== 1) return jsonShape(uses, dialect);
  switch (types[0]) {
    case "null":
      return literal([null], uses, dialect);
    case "string":
      return { type: "string", read: wireOf(uses, dialect, "String") };
    case "boolean":
      return { type: "boolean", read: wireOf(uses, dialect, "Boolean") };
    case "number":
    case "integer": {
      const reader = wireOf(uses, dialect, types[0] === "number" ? "Number" : "Integer");
      const bounds = Object.fromEntries(["minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum"].filter((key) => typeof node[key] === "number").map((key) => [key, node[key]]));
      return { type: "number", read: Object.keys(bounds).length > 0 ? `${wireOf(uses, dialect, "Range")}(${reader}, ${JSON.stringify(bounds)})` : reader };
    }
    case "array": {
      const item = node.items === undefined ? jsonShape(uses, dialect) : shape(node.items, scope, uses, `${context}[]`);
      const bounds = node.minItems !== undefined || node.maxItems !== undefined ? `, ${node.minItems ?? 0}, ${node.maxItems ?? "Number.POSITIVE_INFINITY"}` : "";
      return { type: `${group(item.type)}[]`, read: `${wireOf(uses, dialect, "Array")}(${item.read}${bounds})` };
    }
    case "object": {
      if (node.properties === undefined) {
        const value = node.additionalProperties !== undefined && typeof node.additionalProperties === "object" ? shape(node.additionalProperties, scope, uses, `${context}{}`) : jsonShape(uses, dialect);
        return { type: `{ [key: string]: ${value.type} }`, read: `${wireOf(uses, dialect, "Map")}(${value.read})` };
      }
      const object = members(node, scope, uses, context);
      const inline = `{ ${object.fields.map((field) => field.text).join(" ")} }`;
      return { type: inline, read: `${wireOf(uses, dialect, "Object")}<${named ?? inline}>({ ${object.reads} }${node.additionalProperties === false ? "" : ", false"})` };
    }
    default:
      throw new Error(`[schema twins] ${context}: unsupported type ${types[0]}`);
  }
}

/** 🔀️ A union: an externally tagged variant map, an aggregate of leaf payloads internally tagged by their `mutation` literal, or an untagged choice. */
function union(branches: readonly Node[], scope: Scope, uses: Uses, context: string, named?: string): Shape {
  const { dialect } = scope;
  const external = branches.every((branch) => branch.properties !== undefined && Object.keys(branch.properties).length === 1 && branch.properties[Object.keys(branch.properties)[0]!].$ref !== undefined);
  if (external) {
    const rows = branches.map((branch) => {
      const [variant] = Object.keys(branch.properties) as [string];
      return { variant, payload: reference(branch.properties[variant].$ref, scope, uses, context) };
    });
    return {
      type: `\n${rows.map((row) => `  | { ${row.variant}: ${row.payload.type} }`).join("\n")}`,
      read: `${wireOf(uses, dialect, "External")}<${named ?? "unknown"}>({\n${rows.map((row) => `  ${row.variant}: ${row.payload.read},`).join("\n")}\n})`,
    };
  }
  const refs = branches.every((branch) => typeof branch.$ref === "string");
  const tagOf = (branch: Node): string | undefined => {
    const declared = scope.index.get(splitRef(branch.$ref).id)?.schema.properties?.mutation?.const;
    return typeof declared === "string" ? declared : undefined;
  };
  const tags = refs ? branches.map(tagOf) : [];
  if (refs && tags.every((tag) => tag !== undefined)) {
    const payloads = branches.map((branch) => reference(branch.$ref, scope, uses, context));
    return {
      type: `\n${payloads.map((payload) => `  | ${payload.type}`).join("\n")}`,
      read: `${wireOf(uses, dialect, "Tagged")}<${named ?? "unknown"}, "mutation">("mutation", {\n${payloads.map((payload, at) => `  ${tags[at]}: ${payload.read},`).join("\n")}\n})`,
    };
  }
  const parts = branches.map((branch, at) => shape(branch, scope, uses, `${context}|${at}`));
  return { type: parts.map((part) => group(part.type)).join(" | "), read: `${wireOf(uses, dialect, "Any")}<[${parts.map((part) => part.type).join(", ")}]>(${parts.map((part) => part.read).join(", ")})` };
}
//#endregion 🔖️Shapes

//#region 🔖️Members
interface Field {
  readonly text: string;
  readonly line: string;
}

/** 📝️ The doc comment of a field: the annotated emoji and the state lane, or nothing. */
function fieldDoc(member: Node): string | undefined {
  const emoji = twinNote(member).emoji;
  const state = member["x-semio-state"];
  const parts = [typeof emoji === "string" ? emoji : undefined, typeof state === "string" ? `@state ${state}` : undefined].filter((part) => part !== undefined);
  return parts.length > 0 ? `/** ${parts.join(" ")} */` : undefined;
}

function members(node: Node, scope: Scope, uses: Uses, context: string): { readonly fields: readonly Field[]; readonly reads: string } {
  const { dialect } = scope;
  const required = new Set<string>(node.required ?? []);
  const entries = Object.entries<Node>(node.properties ?? {}).map(([key, member]) => {
    const inner = shape(member, scope, uses, `${context}.${key}`);
    const optional = !required.has(key) || / \| null$/u.test(inner.type);
    const property = /^[A-Za-z_][A-Za-z0-9_]*$/u.test(key) ? key : JSON.stringify(key);
    return { key: property, optional, inner, doc: fieldDoc(member) };
  });
  return {
    fields: entries.map((entry) => ({ text: `${entry.key}${entry.optional ? "?" : ""}: ${entry.inner.type};`, line: `${entry.doc === undefined ? "" : `  ${entry.doc}\n`}  ${entry.key}${entry.optional ? "?" : ""}: ${entry.inner.type};` })),
    reads: entries.map((entry) => `${entry.key}: ${wireOf(uses, dialect, entry.optional ? "Optional" : "Required")}(${entry.inner.read})`).join(", "),
  };
}
//#endregion 🔖️Members

//#region 🔖️Declarations
interface Declaration {
  readonly type: string;
  readonly parser: string;
}

function declare(name: string, node: Node, scope: Scope, uses: Uses): Declaration {
  const { dialect } = scope;
  const readerType = `${dialect.readerType}<${name}>`;
  if (node.properties !== undefined && (node.type === "object" || node.type === undefined)) {
    const object = members(node, scope, uses, name);
    return {
      type: object.fields.length > 0 ? `export interface ${name} {\n${object.fields.map((field) => field.line).join("\n")}\n}` : `export type ${name} = Record<string, never>;`,
      parser: `export const parse${name}: ${readerType} = ${wireOf(uses, dialect, "Object")}<${name}>({ ${object.reads} }${node.additionalProperties === false ? "" : ", false"});`,
    };
  }
  const inner = shape(node, scope, uses, name, name);
  return { type: `export type ${name} =${inner.type.startsWith("\n") ? "" : " "}${inner.type};`, parser: `export const parse${name}: ${readerType} = ${inner.read};` };
}

const byCodePoint = (left: string, right: string): number => Buffer.compare(Buffer.from(left), Buffer.from(right));

const specifier = (from: string, to: string): string => {
  const path = relative(dirname(from), to);
  return path.startsWith(".") ? path : `./${path}`;
};

/** 📝️ The comment that opens a generated twin: facet emoji, record, and the sentence that says what the twin is. */
function header(module: SchemaModule, dialect: Dialect): string[] {
  const note = twinNote(module.schema);
  const root = rootName(module);
  const leaf = leafDescriptor(module);
  const style = dialect.facets.get(facetOf(module));
  const emoji = typeof note.emoji === "string" ? note.emoji : leaf !== undefined ? leaf.emoji : (style?.emoji ?? "");
  const subject = leaf !== undefined ? `\`${leaf.kind}\`` : `\`${root}\``;
  const summary = typeof note.summary === "string" ? note.summary : leaf !== undefined ? `${dialect.leaf.summary} \`${root}\`` : (style?.summary ?? "");
  return [`/** ${emoji} ${subject} wire twin: ${summary}, exactly as \`./🦀️.rs\` writes it. Generated from \`./🔣️.json\``, ` * by \`${dialect.generatedBy}\`; readers judge structure, the schema's bounds stay Ajv's.`, " * @see ./🔣️.json */"];
}

/** 📝️ The generated TypeScript text of one module. */
export function renderTypescript(module: SchemaModule, index: ReadonlyMap<string, SchemaModule>, dialect: Dialect): string {
  const scope: Scope = { origin: module, module, index, dialect };
  const uses = new Uses();
  uses.contract.add(`type ${dialect.readerType}`);
  const declarations = [declare(rootName(module), module.schema, scope, uses), ...[...module.definitions].map(([key, node]) => declare(recordName(module, key, dialect), node, scope, uses))];
  const contract = [...uses.contract].sort(byCodePoint);
  const imports = [...uses.imports.entries()]
    .map(([path, names]) => ({ from: specifier(twinPath(module), path), names: [...names].sort(byCodePoint).flatMap((name) => [`type ${name}`, `parse${name}`]) }))
    .sort((left, right) => byCodePoint(left.from, right.from));
  return [
    ...header(module, dialect),
    `import { ${contract.join(", ")} } from "${dialect.contractFile === undefined ? dialect.contractModule : specifier(twinPath(module), dialect.contractFile)}";`,
    ...imports.map((entry) => `import { ${entry.names.join(", ")} } from "${entry.from}";`),
    "",
    ...declarations.flatMap((entry) => [entry.type, ""]),
    ...declarations.map((entry) => entry.parser),
    "",
  ].join("\n");
}
//#endregion 🔖️Declarations
