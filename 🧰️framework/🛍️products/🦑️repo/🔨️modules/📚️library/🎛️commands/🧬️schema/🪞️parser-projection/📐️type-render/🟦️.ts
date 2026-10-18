/** 📐️ Renders one schema export as a GraphQL type, a protobuf message or a TypeScript type, derived from the normative JSON Schema alone.
 *
 * A nested object becomes a named declaration `<Parent><Field>` in GraphQL and protobuf. A form the target language cannot express (a union, a map, a tuple, any value) carries its JSON: the GraphQL scalar `JSON`, the protobuf field `bytes` (the JSON text of the value); the JSON Schema keeps stating the shape. A name a declaration would take that the leaf or the document already uses is never taken: that nested value falls back to `JSON` or `bytes`.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts
 */
import { EXPORT_NAME, IDENTIFIER_NAME, asDict, has, isDict, jsonString, jsonValue, orderedEntries, snake, truthy, type Dict } from "../🔡️json-text/🟦️.ts";

//#region 🔖️Shapes
/** 🔎️ The one element schema of an array; a positional (tuple) `items` list has none. */
export const itemSchema = (node: Dict): Dict => (isDict(node.items) ? node.items : {});

/** 🔎️ The export a `$ref` names, if its last segment is an export name. */
const referenced = (node: Dict): string | undefined => {
  const target = String(node.$ref).split("/").at(-1)!;
  return EXPORT_NAME.test(target) ? target : undefined;
};

const isNull = (node: unknown): boolean => isDict(node) && node.type === "null";

/** 🔎️ The single non-null alternative of `type: [T, "null"]` or `anyOf|oneOf: [T, {"type": "null"}]`. */
export function nullableOf(node: Dict): Dict | undefined {
  const kinds = node.type;
  if (Array.isArray(kinds) && kinds.includes("null")) {
    const rest = kinds.filter((kind) => kind !== "null");
    return rest.length === 1 ? { ...Object.fromEntries(Object.entries(node).filter(([key]) => key !== "type")), type: rest[0] } : undefined;
  }
  for (const key of ["anyOf", "oneOf"]) {
    const alternatives = node[key];
    if (Array.isArray(alternatives) && alternatives.length === 2 && alternatives.filter(isNull).length === 1) {
      const other = alternatives.find((alternative) => !isNull(alternative));
      return isDict(other) ? other : undefined;
    }
  }
  return undefined;
}

const stringEnum = (node: Dict): readonly string[] | undefined => (Array.isArray(node.enum) && node.enum.every((entry) => typeof entry === "string") ? node.enum : undefined);

const isScalar = (value: unknown): value is string | number | bigint | boolean => typeof value === "string" || typeof value === "number" || typeof value === "bigint" || typeof value === "boolean";

const objectEntries = (node: Dict): [string, unknown][] | undefined => {
  if ((node.type !== "object" && has(node, "type")) || !isDict(node.properties) || !truthy(node.properties)) return undefined;
  return orderedEntries(node.properties) ?? undefined;
};

const ANNOTATIONS = new Set(["description", "title", "default", "examples", "$comment", "deprecated", "readOnly", "writeOnly", "$id", "$schema"]);

/** 🔎️ A node that states no structure, only annotations: any JSON value. */
export const isAny = (node: Dict): boolean => Object.keys(node).every((key) => ANNOTATIONS.has(key) || key.startsWith("x-"));

/** 🏷️ A field name as a GraphQL or protobuf identifier: a character outside `[A-Za-z0-9_]` becomes `_` and a leading digit gets a `_` in front (`2d` to `_2d`); `undefined` when nothing is left. */
const identifierOf = (field: string): string | undefined => {
  const name = field.replace(/[^A-Za-z0-9_]/gu, "_");
  return name === "" ? undefined : /^[0-9]/u.test(name) ? `_${name}` : name;
};

/** 🏷️ `foo_bar` and `fooBar` to `FooBar`. */
const pascal = (field: string): string =>
  field
    .split(/[^A-Za-z0-9]+/u)
    .filter((part) => part !== "")
    .map((part) => part.slice(0, 1).toUpperCase() + part.slice(1))
    .join("");

type ScalarKind = "string" | "integer" | "number" | "boolean";

const memberKind = (value: unknown): ScalarKind | undefined =>
  typeof value === "string" ? "string" : typeof value === "boolean" ? "boolean" : typeof value === "bigint" ? "integer" : typeof value === "number" ? (Number.isInteger(value) ? "integer" : "number") : undefined;

/** 🔎️ The scalar kind an `enum` or `const` pins its values to, or `undefined` for none or a mixture. */
function pinnedKind(node: Dict): ScalarKind | undefined {
  const members = has(node, "const") ? [node.const] : Array.isArray(node.enum) ? node.enum : undefined;
  if (members === undefined || members.length === 0) return undefined;
  const kinds = new Set(members.map(memberKind));
  if (kinds.has(undefined)) return undefined;
  if (kinds.size === 1) return [...kinds][0];
  return [...kinds].every((kind) => kind === "integer" || kind === "number") ? "number" : undefined;
}

/** 🔎️ The kind of a node: what its `enum` or `const` pins, else its `type`, else `object` for a node that states properties. */
function kindOf(node: Dict): string | undefined {
  return pinnedKind(node) ?? (typeof node.type === "string" ? node.type : !has(node, "type") && objectEntries(node) !== undefined ? "object" : undefined);
}
//#endregion 🔖️Shapes

//#region 🔖️Auxiliaries
/** 🏷️ True when a name is already declared in the leaf (or is an export of the document) and must not be taken by an auxiliary declaration. */
export type Taken = (name: string) => boolean;

/** 📦️ The named declarations one rendering adds in front of its export (`<Parent><Field>` for a nested object, the `JSON` scalar), and the names it has claimed. */
class Auxiliaries {
  private readonly claimed = new Set<string>();
  private readonly texts = new Map<string, string>();

  constructor(private readonly taken: Taken, root: string) {
    this.claimed.add(root);
  }

  claim(name: string): boolean {
    if (this.claimed.has(name) || this.taken(name)) return false;
    this.claimed.add(name);
    return true;
  }

  put(name: string, text: string): void {
    this.texts.set(name, text);
  }

  json(): void {
    if (this.claim("JSON")) this.put("JSON", "scalar JSON\n");
  }

  all(): string[] {
    return [...this.texts.values()];
  }
}
//#endregion 🔖️Auxiliaries

//#region 🔖️GraphQL
interface GraphqlType {
  readonly text: string;
  readonly nullable: boolean;
}

const GRAPHQL_SCALARS = { string: "String", boolean: "Boolean", integer: "Int", number: "Float" } as const;

/** 🔗️ A `type <name>` of a nested object, or `undefined` when a property has no GraphQL name. */
function graphqlObject(name: string, node: Dict, aux: Auxiliaries): string | undefined {
  const entries = objectEntries(node);
  if (entries === undefined) return undefined;
  const required = new Set(Array.isArray(node.required) ? node.required : []);
  const lines: string[] = [];
  const names = new Set<string>();
  for (const [field, shape] of entries) {
    const label = identifierOf(field);
    if (label === undefined || names.has(label)) return undefined;
    names.add(label);
    const rendered = graphqlType(asDict(shape), name, field, aux);
    lines.push(`  ${label}: ${rendered.text}${required.has(field) && !rendered.nullable ? "!" : ""}`);
  }
  return `type ${name} {\n${lines.join("\n")}\n}\n`;
}

/** 🔗️ The GraphQL type of one schema node and whether it admits `null`. Every node has one: a nested object becomes the type `<Parent><Field>`, an enum or constant its scalar, and a form GraphQL cannot express (a map, a union, any value, a tuple) the scalar `JSON`. */
function graphqlType(node: Dict, parent: string, field: string, aux: Auxiliaries): GraphqlType {
  const json = (): GraphqlType => {
    aux.json();
    return { text: "JSON", nullable: false };
  };
  if (has(node, "$ref")) {
    const target = referenced(node);
    return target === undefined ? json() : { text: target, nullable: false };
  }
  const present = nullableOf(node);
  if (present) return { ...graphqlType(present, parent, field, aux), nullable: true };
  const kind = kindOf(node);
  if (kind === "string" || kind === "boolean" || kind === "integer" || kind === "number") return { text: GRAPHQL_SCALARS[kind], nullable: false };
  if (kind === "array") {
    if (!isDict(node.items)) return json();
    const inner = graphqlType(node.items, parent, field, aux);
    return { text: `[${inner.text}${inner.nullable ? "" : "!"}]`, nullable: false };
  }
  if (kind === "object" && objectEntries(node) !== undefined) {
    const name = `${parent}${pascal(field)}`;
    if (pascal(field) === "" || !aux.claim(name)) return json();
    const text = graphqlObject(name, node, aux);
    if (text === undefined) return json();
    aux.put(name, text);
    return { text: name, nullable: false };
  }
  return json();
}

/** 🔗️ The GraphQL declarations of one export: an enum, an object type (with the types of its nested objects in front) or a scalar. `undefined` only when a property has no GraphQL name. */
export function renderGraphql(name: string, definition: Dict, taken: Taken = () => false): string | undefined {
  const members = stringEnum(definition);
  if (members) {
    if (members.every((entry) => IDENTIFIER_NAME.test(entry))) return `enum ${name} {\n${members.map((entry) => `  ${entry}`).join("\n")}\n}\n`;
    return `# 🔤️ ${name} is a string domain whose members are not GraphQL names; the JSON Schema enumerates them.\nscalar ${name}\n`;
  }
  const aux = new Auxiliaries(taken, name);
  const entries = objectEntries(definition);
  if (entries !== undefined) {
    const text = graphqlObject(name, definition, aux);
    return text === undefined ? undefined : [...aux.all(), text].join("\n");
  }
  if (isDict(definition.properties) && truthy(definition.properties)) return undefined;
  return `scalar ${name}\n`;
}
//#endregion 🔖️GraphQL

//#region 🔖️Protobuf
interface ProtoType {
  readonly text: string;
  readonly repeated: boolean;
  readonly map: boolean;
}

const BYTES: ProtoType = { text: "bytes", repeated: false, map: false };

const single = (text: string): ProtoType => ({ text, repeated: false, map: false });

/** 🛰️ A `message <name>` of a nested object, or `undefined` when a property has no protobuf name. */
function protoMessage(name: string, node: Dict, aux: Auxiliaries): string | undefined {
  const entries = objectEntries(node);
  if (entries === undefined) return undefined;
  const required = new Set(Array.isArray(node.required) ? node.required : []);
  const lines: string[] = [];
  const names = new Set<string>();
  let index = 1;
  for (const [field, shape] of entries) {
    const label = identifierOf(snake(field));
    if (label === undefined || names.has(label)) return undefined;
    names.add(label);
    const rendered = protoType(asDict(shape), name, field, aux);
    const prefix = rendered.repeated ? "repeated " : rendered.map ? "" : required.has(field) && nullableOf(asDict(shape)) === undefined ? "" : "optional ";
    lines.push(`  ${prefix}${rendered.text} ${label} = ${index};`);
    index += 1;
  }
  return `message ${name} {\n${lines.join("\n")}\n}\n`;
}

/** 🛰️ The protobuf type of one schema node. Every node has one: a nested object becomes the message `<Parent><Field>`, a typed map `map<string, V>`, an array `repeated`, and a form protobuf cannot express (a union, any value, a tuple, a nullable array, an array of arrays) `bytes`, the JSON text of the value. */
function protoType(node: Dict, parent: string, field: string, aux: Auxiliaries): ProtoType {
  if (has(node, "$ref")) {
    const target = referenced(node);
    return target === undefined ? BYTES : single(target);
  }
  const present = nullableOf(node);
  if (present) {
    const inner = protoType(present, parent, field, aux);
    return inner.repeated || inner.map ? BYTES : inner;
  }
  const kind = kindOf(node);
  const declared = node.format;
  if (kind === "string") return single("string");
  if (kind === "boolean") return single("bool");
  if (kind === "integer") return single(["uint32", "uint64", "int32", "int64"].includes(declared as string) ? (declared as string) : "int64");
  if (kind === "number") return single(["double", "float"].includes(declared as string) ? (declared as string) : "double");
  if (kind === "array") {
    if (!isDict(node.items) || nullableOf(node.items) !== undefined) return BYTES;
    const inner = protoType(node.items, parent, field, aux);
    return inner.repeated || inner.map ? BYTES : { text: inner.text, repeated: true, map: false };
  }
  if (kind === "object") {
    if (objectEntries(node) !== undefined) {
      const name = `${parent}${pascal(field)}`;
      if (pascal(field) === "" || !aux.claim(name)) return BYTES;
      const text = protoMessage(name, node, aux);
      if (text === undefined) return BYTES;
      aux.put(name, text);
      return single(name);
    }
    const values = node.additionalProperties;
    if (isDict(values) && !isAny(values) && nullableOf(values) === undefined) {
      const inner = protoType(values, parent, field, aux);
      return inner.repeated || inner.map ? BYTES : { text: `map<string, ${inner.text}>`, repeated: false, map: true };
    }
  }
  return BYTES;
}

/** 🛰️ The protobuf messages of one export: the message itself (with the messages of its nested objects in front) or, for a value that is no object, a message whose one field carries the value. `undefined` only when a property has no protobuf name. */
export function renderProto(name: string, definition: Dict, taken: Taken = () => false): string | undefined {
  const aux = new Auxiliaries(taken, name);
  const entries = objectEntries(definition);
  if (entries !== undefined) {
    const text = protoMessage(name, definition, aux);
    return text === undefined ? undefined : [...aux.all(), text].join("\n");
  }
  if (isDict(definition.properties) && truthy(definition.properties)) return undefined;
  if (stringEnum(definition)) return `message ${name} {\n  string value = 1;\n}\n`;
  const rendered = protoType(definition, name, "value", aux);
  const field = rendered.map ? `${rendered.text} entries = 1;` : `${rendered.repeated ? "repeated " : ""}${rendered.text} value = 1;`;
  return [...aux.all(), `message ${name} {\n  ${field}\n}\n`].join("\n");
}
//#endregion 🔖️Protobuf

//#region 🔖️TypeScript
/** 🔎️ True when `text` has `operator` outside every bracket and string literal. */
function splitsAtTop(text: string, operator: string): boolean {
  let depth = 0;
  let quote: string | undefined;
  for (let at = 0; at < text.length; at += 1) {
    const char = text[at]!;
    if (quote !== undefined) {
      if (char === "\\") at += 1;
      else if (char === quote) quote = undefined;
    } else if (char === '"' || char === "'" || char === "`") quote = char;
    else if ("([{<".includes(char)) depth += 1;
    else if (")]}>".includes(char) && !(char === ">" && text[at - 1] === "=")) depth -= 1;
    else if (depth === 0 && text.startsWith(operator, at)) return true;
  }
  return false;
}

const group = (text: string): string => (text.startsWith("readonly ") || splitsAtTop(text, " | ") || splitsAtTop(text, " & ") ? `(${text})` : text);

const propertyName = (field: string): string => (IDENTIFIER_NAME.test(field) ? field : jsonString(field, false));

/** 🧭️ How the TypeScript type of a reference outside the plain export names is found: the type text, or the node to render in the scope of the document that states it. */
export type TypeResolution = string | { readonly node: Dict; readonly scope: TypeScope };

/** 🧭️ The references one document's types can follow (a whole document, a pointer into this or another document); without a scope only a reference to an export name has a type. */
export interface TypeScope {
  readonly type: (reference: string) => TypeResolution | undefined;
}

let following = 0;

/** 🧭️ How many references one type follows in place before it is refused as cyclic. */
const MAX_FOLLOWED = 12;

function tsFields(node: Dict, scope?: TypeScope): string[] | undefined {
  const entries = objectEntries(node);
  if (entries === undefined) return undefined;
  const required = new Set(Array.isArray(node.required) ? node.required : []);
  const lines: string[] = [];
  for (const [field, shape] of entries) {
    const rendered = tsType(asDict(shape), scope);
    if (rendered === undefined) return undefined;
    lines.push(`readonly ${propertyName(field)}${required.has(field) ? "" : "?"}: ${rendered}`);
  }
  return lines;
}

/** 🟦️ The TypeScript literal type of a constant: a scalar, `null`, a tuple or an object type. */
function constType(value: unknown): string | undefined {
  if (value === null) return "null";
  if (isScalar(value)) return jsonValue(value, false);
  if (Array.isArray(value)) {
    const parts = value.map(constType);
    return parts.every((part) => part !== undefined) ? `readonly [${parts.join(", ")}]` : undefined;
  }
  if (isDict(value)) {
    const parts = Object.entries(value).map(([key, member]) => {
      const rendered = constType(member);
      return rendered === undefined ? undefined : `readonly ${propertyName(key)}: ${rendered}`;
    });
    return parts.every((part) => part !== undefined) ? `{ ${parts.join("; ")} }` : undefined;
  }
  return undefined;
}

/** 🟦️ The TypeScript type text of one schema node, or `undefined` when it has no plain form: a reference nothing resolves, or a nested object with a property that has no plain form. A union is the union of its alternatives, `allOf` an intersection, a positional `items` list a tuple, a nested object an object type, a constant its literal type, and a node that states no structure `unknown`. */
export function tsType(node: Dict, scope?: TypeScope): string | undefined {
  if (has(node, "$ref")) {
    if (scope === undefined || typeof node.$ref !== "string") return referenced(node);
    const resolved = scope.type(node.$ref);
    if (resolved === undefined) return undefined;
    if (typeof resolved === "string") return resolved;
    if (following >= MAX_FOLLOWED) return undefined;
    following += 1;
    try {
      return tsType(resolved.node, resolved.scope);
    } finally {
      following -= 1;
    }
  }
  const nullable = nullableOf(node);
  if (nullable) {
    const inner = tsType(nullable, scope);
    return inner ? `${inner} | null` : undefined;
  }
  const members = stringEnum(node);
  if (members) return members.map((entry) => jsonString(entry, false)).join(" | ");
  if (has(node, "const")) return constType(node.const);
  if (Array.isArray(node.enum) && node.enum.length > 0 && node.enum.every(isScalar)) return node.enum.map((entry) => jsonValue(entry, false)).join(" | ");
  const kind = node.type;
  if (kind === undefined) {
    for (const key of ["oneOf", "anyOf"]) {
      const alternatives = node[key];
      if (Array.isArray(alternatives) && alternatives.length > 0) {
        const parts = alternatives.map((alternative) => (isDict(alternative) ? tsType(alternative, scope) : undefined));
        return parts.every((part) => part !== undefined) ? parts.join(" | ") : undefined;
      }
    }
    if (Array.isArray(node.allOf) && node.allOf.length > 0) {
      const parts = node.allOf.map((part) => (isDict(part) ? tsType(part, scope) : undefined));
      return parts.every((part) => part !== undefined) ? parts.map((part) => group(part!)).join(" & ") : undefined;
    }
    if (isDict(node.properties) && truthy(node.properties)) return tsType({ ...node, type: "object" }, scope);
    return isAny(node) ? "unknown" : undefined;
  }
  if (Array.isArray(kind)) {
    const parts = kind.map((entry) => (typeof entry === "string" ? tsType({ ...node, type: entry }, scope) : undefined));
    return parts.every((part) => part !== undefined) ? parts.join(" | ") : undefined;
  }
  if (kind === "null") return "null";
  if (kind === "string") return "string";
  if (kind === "boolean") return "boolean";
  if (kind === "integer" || kind === "number") return "number";
  if (kind === "array") {
    if (Array.isArray(node.items)) {
      const parts = node.items.map((member) => (isDict(member) ? tsType(member, scope) : undefined));
      return parts.every((part) => part !== undefined) ? `readonly [${parts.join(", ")}]` : undefined;
    }
    const inner = tsType(itemSchema(node), scope);
    return inner ? `readonly ${group(inner)}[]` : undefined;
  }
  if (kind === "object") {
    if (!truthy(node.properties)) {
      const values = node.additionalProperties;
      const inner = isDict(values) && !isAny(values) ? tsType(values, scope) : undefined;
      return inner ? `Readonly<Record<string, ${inner}>>` : "Readonly<Record<string, unknown>>";
    }
    const fields = tsFields(node, scope);
    return fields === undefined ? undefined : `{ ${fields.join("; ")} }`;
  }
  return undefined;
}

/** 🟦️ An exported TypeScript interface or alias for one export, or `undefined` when the schema has no plain form. */
export function renderTsType(name: string, definition: Dict, scope?: TypeScope): string | undefined {
  const fields = tsFields(definition, scope);
  if (fields !== undefined) return `export interface ${name} {\n${fields.map((line) => `  ${line};`).join("\n")}\n}\n`;
  const inline = tsType(definition, scope);
  return inline ? `export type ${name} = ${inline};\n` : undefined;
}
//#endregion 🔖️TypeScript
