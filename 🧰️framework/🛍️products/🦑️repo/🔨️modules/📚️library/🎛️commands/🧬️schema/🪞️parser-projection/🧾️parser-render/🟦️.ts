/** 🧾️ Renders one schema export as a typed `parse<Export>` function: the expression for every schema form, the callees it needs and the source it reads its declared types from.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts
 */
import { BARE_TYPE_NAME, EXPORT_NAME, asDict, has, isDict, jsonString, jsonValue, keyText, orderedEntries, truthy, type Dict } from "../🔡️json-text/🟦️.ts";
import { aliasMembers, aliasText, aliasVariants, declaredTypes, definesParser, importsParser, instantiatesGeneric, memberTypes, normalizeType, reexportedBinding, splitTop, unionMembers } from "../📖️declared-types/🟦️.ts";
import { dirname, relative, resolve } from "node:path";
import {createHash} from "node:crypto";
import { isAny, nullableOf } from "../📐️type-render/🟦️.ts";

//#region 🔖️Context
/** 🧭️ One schema document's TypeScript leaf and the exports it states, addressed by the document's `$id` and by its file (`file:<absolute path>`); `root` is the export a reference to the whole document names. */
export class Owner {
  constructor(
    readonly ts: string,
    readonly exports: ReadonlySet<string>,
    readonly root?: string,
    readonly shared = false,
    readonly load: () => Dict | undefined = () => undefined,
    readonly specifier?: string,
  ) {}
}

/** 📚️ The text of every source the projection reads, cached on first read so one run sees one consistent tree. */
export interface SourceReader {
  read(path: string): string;
  declarations(path: string): string;
}

/** 🔗️ A parser named by the file that declares it. */
export type ParserKey = string;

/** 🗝️ The key of parser `name` declared (or promised) in `owner`. */
export const parserKey = (owner: string, name: string): ParserKey => `${owner}\u0000${name}`;

/** 〽️ The owner and the name a parser key addresses. */
export function parserKeyParts(key: ParserKey): [string, string] {
  const at = key.indexOf("\u0000");
  return [key.slice(0, at), key.slice(at + 1)];
}

/** 🏫️ What one parser rendering needs from its surroundings, and what it learned about its callees. */
export class Context {
  readonly local = new Set<string>();
  self: string | undefined;

  constructor(
    readonly helper: string,
    readonly documentId: string | undefined,
    readonly index: ReadonlyMap<string, Owner>,
    readonly target: string,
    readonly siblings: readonly string[],
    readonly sources: SourceReader,
    readonly promised: ReadonlySet<ParserKey>,
    readonly closed = true,
    readonly document?: Dict,
    readonly reserved:ReadonlySet<string> = index.get(documentId??"")?.exports??new Set(),
    readonly resolving = new Set<string>(),
    readonly foreign = new Map<ParserKey,string>(),
    readonly redirect?: Owner,
    readonly receiver = target,
  ) {}

  /** 👁️ The context in which the nodes of another document are rendered: it reads that document, names its parsers as imports of the referring leaf, and shares the callees and the cycle guard of this one. */
  viewOf(owner: Owner, document: Dict | undefined): Context {
    return new Context(this.helper, typeof document?.$id === "string" ? document.$id : undefined, this.index, owner.ts, [], this.sources, this.promised, this.closed, document, this.reserved, this.resolving, this.foreign, owner, this.receiver);
  }

  get own(): string {
    return this.sources.read(this.target);
  }

  get declared(): string {
    return this.sources.declarations(this.target);
  }
}

/** ☎️ `parse<name>` as an in-scope call: this file first, then a sibling facet of the same module. */
function locateParser(name: string, context: Context): string | undefined {
  const own = context.own;
  if (definesParser(own, name) || importsParser(own, name) || context.promised.has(parserKey(context.target, name))) {
    context.local.add(name);
    return name;
  }
  for (const sibling of context.siblings) {
    if (sibling !== context.target && (definesParser(context.sources.read(sibling), name) || context.promised.has(parserKey(sibling, name)))) {
      context.foreign.set(parserKey(sibling, name), `parse${name}`);
      return name;
    }
  }
  return undefined;
}

/** 📞️ The identifier a call of `parse<name>` uses inside this file: the local binding when the file exports it under an alias, `parse<name>` otherwise. */
const calleeOf = (name: string, context: Context): string => reexportedBinding(context.own, `parse${name}`) ?? `parse${name}`;

/** 🔎️ The document a reference addresses: by `$id`, or, for a relative file reference, by the file it resolves to from the folder of the leaf that refers to it. */
export function findOwner(index: ReadonlyMap<string, Owner>, document: string, leaf: string): Owner | undefined {
  const found = index.get(document);
  if (found !== undefined || /^[A-Za-z][A-Za-z0-9+.-]*:/u.test(document)) return found;
  try {
    return index.get(`file:${resolve(dirname(leaf), decodeURIComponent(document))}`);
  } catch {
    return undefined;
  }
}

const ownerOf = (document: string, context: Context): Owner | undefined => findOwner(context.index, document, context.target);

/** 📍️ The node a JSON pointer addresses inside `document`, or `undefined`. */
export function pointed(pointer: string, document: Dict | undefined): Dict | undefined {
  let node: unknown = document;
  for (const segment of pointer.split("/").slice(1)) {
    let key: string;
    try {
      key = decodeURIComponent(segment).replace(/~1/gu, "/").replace(/~0/gu, "~");
    } catch {
      return undefined;
    }
    node = isDict(node) ? node[key] : Array.isArray(node) ? node[Number(key)] : undefined;
  }
  return isDict(node) ? node : undefined;
}


/** 🪪️ Keeps foreign parser origins distinct from names reserved by the receiving schema. */
function foreignParserBinding(owner:Owner,name:string,context:Context):string{
 const local=context.reserved.has(name)||context.promised.has(parserKey(context.receiver,name))||definesParser(context.sources.read(context.receiver),name);
 if(owner.ts===context.receiver||!local)return `parse${name}`;
 const origin=owner.specifier??relative(dirname(context.receiver),owner.ts).replaceAll("\\","/");
 return `parse${name}$${createHash("sha256").update(origin).digest("hex")}`;
}

function referenceCall(reference: string, access: string, at: string, context: Context, path: string, depth: number): string | undefined {
  const hash = reference.indexOf("#");
  const document = hash < 0 ? reference : reference.slice(0, hash);
  const pointer = hash < 0 ? "" : reference.slice(hash + 1);
  const same = document === "" || document === context.documentId;
  const owner = same ? context.redirect : ownerOf(document, context);
  if (!same && owner === undefined) return undefined;
  const target = pointer === "" ? owner?.root : pointer.split("/").at(-1)!;
  const signature = owner?.shared && target !== undefined && EXPORT_NAME.test(target) ? new RegExp(`export function parse${target}\\(value: unknown(, at\\b)?`, "u").exec(context.sources.read(owner.ts)) : null;
  const callable = owner === undefined || !owner.shared || (owner.specifier !== undefined && signature !== null);
  const exported = target !== undefined && EXPORT_NAME.test(target) && (owner === undefined || owner.exports.has(target)) && callable;
  if (pointer.startsWith("/") && !exported) {
    const view = owner === undefined || owner === context.redirect ? context : context.viewOf(owner, owner.load());
    const node = pointed(pointer, view.document);
    const key = `${view.target}${pointer}`;
    if (node === undefined || context.resolving.has(key)) return undefined;
    context.resolving.add(key);
    try {
      return parserExpression(node, access, path, view, depth);
    } finally {
      context.resolving.delete(key);
    }
  }
  if (target === undefined || !EXPORT_NAME.test(target)) return undefined;
  const arguments_ = owner?.shared && signature !== null && signature[1] === undefined ? access : `${access}, ${at}`;
  if (owner !== undefined) {
    if (!owner.exports.has(target)) return undefined;
    context.foreign.set(parserKey(owner.ts, target), foreignParserBinding(owner,target,context));
  } else context.local.add(target);
  return `${owner !== undefined ? context.foreign.get(parserKey(owner.ts,target))! : calleeOf(target, context)}(${arguments_})`;
}
//#endregion 🔖️Context

//#region 🔖️Fragments
/** 📏️ The schema's own bounds, passed to the guard so the parser refuses what the schema refuses. */
export function boundsText(node: Dict, keys: readonly string[]): string {
  const stated: Record<string, unknown> = {};
  for (const key of keys) if (has(node, key) && (typeof node[key] === "number" || typeof node[key] === "bigint" || typeof node[key] === "string")) stated[key] = node[key];
  return truthy(stated) ? `, ${jsonValue(stated, false)}` : "";
}

/** 🕳️ A schema node that states no structure (`{"type": "object"}` or an `array` without `items`); an object that is closed (`additionalProperties: false`) states its structure: nothing. */
const opaque = (node: Dict): boolean =>
  !has(node, "$ref") && ((node.type === "object" && !truthy(node.properties) && !isDict(node.additionalProperties) && node.additionalProperties !== false) || (node.type === "array" && !truthy(node.items)));

/** 🔒️ The property names a closed object states, as the array literal `<prefix>Closed` takes. */
const keyList = (keys: readonly string[]): string => `[${keys.map((key) => jsonString(key, false)).join(", ")}]`;

/** 📪️ `read` wrapped in the refusal of unknown properties exactly where the schema says `additionalProperties: false`; `undefined` when the schema also states pattern properties, which the refusal cannot honour. */
function closedRead(node: Dict, read: string, at: string, keys: readonly string[], context: Context): string | undefined {
  if (!context.closed || node.additionalProperties !== false) return read;
  return has(node, "patternProperties") ? undefined : `${context.helper}Closed(${read}, ${at}, ${keyList(keys)})`;
}

const exactItems = (count: number): string => `, {"minItems": ${count}, "maxItems": ${count}}`;

/** ⚓️ True when the schema states both the element schema and a fixed length of an array: it states more than a declared tuple alias, so the schema renders the parser. */
const fixedItems = (node: Dict): boolean => isDict(node.items) && truthy(node.items) && node.minItems === node.maxItems && typeof node.minItems === "number" && node.minItems >= 1 && node.minItems <= MAX_TUPLE;

/** 🔧️ The longest fixed-size array a parser reads as a tuple. */
const MAX_TUPLE = 16;

const itemBindings = (depth: number): [string, string] => (depth === 0 ? ["item", "index"] : [`item${depth}`, `index${depth}`]);

const stripQuotes = (text: string): string => text.replace(/^['"]+|['"]+$/gu, "");
//#endregion 🔖️Fragments

//#region 🔖️Declared
/** ⌨️ A parser for a field whose type is only known from the interface that declares it (scalars, tuples, arrays, named types). The root value of an export never resolves to a parser of its own name, so an opaque export whose declared type names the export cannot call itself or import over its own declaration; a field of that type still recurses. */
function typedExpression(rawText: string, access: string, path: string, context: Context, depth: number): string | undefined {
  const helper = context.helper;
  const at = `\`${path}\``;
  const text = normalizeType(rawText);
  if (text === "number") return `${helper}Number(${access}, ${at})`;
  if (text === "string") return `${helper}String(${access}, ${at})`;
  if (text === "boolean") return `${helper}Boolean(${access}, ${at})`;
  if (text.startsWith("[") && text.endsWith("]")) {
    const members = splitTop(text.slice(1, -1));
    const items = `items${depth}`;
    const elements = members.map((member, index) => typedExpression(member, `${items}[${index}]`, `${path}[${index}]`, context, depth + 1));
    if (members.length === 0 || elements.some((element) => element === undefined)) return undefined;
    return `((${items}) => [${elements.join(", ")}])(${helper}Array(${access}, ${at}${exactItems(members.length)}))`;
  }
  if (text.endsWith("[]")) {
    const [item, index] = depth ? [`item${depth}`, `index${depth}`] : ["item", "index"];
    const element = typedExpression(text.slice(0, -2), item, `${path}[\${${index}}]`, context, depth + 1);
    return element === undefined ? undefined : `${helper}Array(${access}, ${at}).map((${item}, ${index}) => ${element})`;
  }
  if (text === context.self && access === "value") return undefined;
  if (BARE_TYPE_NAME.test(text)) return locateParser(text, context) ? `${calleeOf(text, context)}(${access}, ${at})` : undefined;
  return undefined;
}
//#endregion 🔖️Declared

//#region 🔖️Objects
/** 🧷️ The schema's own `default` of a property, as source text, when it is a JSON literal a parser can state (`null` is an absent default only for a field whose declared type admits `null`). */
function defaultText(shape: Dict, nullable = false): string | undefined {
  const fallback = shape.default;
  if (!has(shape, "default")) return undefined;
  if (fallback === null) return nullable ? "null" : undefined;
  return typeof fallback === "string" || typeof fallback === "number" || typeof fallback === "boolean" || Array.isArray(fallback) || isDict(fallback) ? jsonValue(fallback, false) : undefined;
}

function objectFields(node: Dict, row: string, path: string, context: Context, depth: number, declared?: ReadonlyMap<string, boolean>, types?: ReadonlyMap<string, string>): string[] | undefined {
  const required = new Set<unknown>(Array.isArray(node.required) ? node.required : []);
  const properties = asDict(node.properties);
  const nullTyped = (field: string): boolean => {
    const type = types?.get(field);
    return type !== undefined && unionMembers(type).includes("null");
  };
  if (declared) for (const [field, optional] of declared) if (!optional && defaultText(asDict(properties[field]), nullTyped(field)) === undefined) required.add(field);
  const entries = orderedEntries(properties);
  if (entries === null) return undefined;
  const lines: string[] = [];
  for (const [field, shape] of entries) {
    const access = `${row}[${jsonString(field, true)}]`;
    const fieldType = types?.get(field);
    const inner = nullableOf(asDict(shape));
    const absent = !required.has(field) && inner !== undefined && fieldType !== undefined && !unionMembers(fieldType).includes("null");
    const read = parserExpression(absent ? inner : asDict(shape), access, `${path}.${field}`, context, depth, fieldType);
    if (read === undefined) return undefined;
    const expression = read === access && required.has(field) ? `(${access} !== undefined ? ${access} : ${context.helper}Reject(\`${path}.${field}\`, "required property is missing"))` : read;
    const fallback = !required.has(field) && declared?.get(field) === false ? defaultText(asDict(shape), nullTyped(field)) : undefined;
    lines.push(required.has(field) ? `${keyText(field)}: ${expression}` : `${keyText(field)}: ${access} ${absent ? "==" : "==="} ${absent ? "null" : "undefined"} ? ${fallback ?? "undefined"} : ${expression}`);
  }
  return lines;
}

/** 🔑️ The property every alternative pins to a distinct string `const`, which selects the alternative. */
function taggedDiscriminator(alternatives: readonly unknown[]): string | undefined {
  if (alternatives.length === 0 || !alternatives.every((alternative) => isDict(alternative) && isDict(alternative.properties))) return undefined;
  const first = alternatives[0] as Dict;
  for (const key of Object.keys(first.properties as Dict)) {
    const tags = alternatives.map((alternative) => asDict(asDict((alternative as Dict).properties)[key]).const);
    if (tags.every((tag) => typeof tag === "string") && new Set(tags).size === tags.length) return key;
  }
  return undefined;
}

function unionExpression(alternatives: readonly unknown[], access: string, path: string, context: Context, depth: number, declared?: string): string | undefined {
  const helper = context.helper;
  const at = `\`${path}\``;
  const row = `row${depth}`;
  const key = taggedDiscriminator(alternatives);
  if (key !== undefined) {
    const tags = alternatives.map((alternative) => asDict(asDict((alternative as Dict).properties)[key]).const as string);
    const cases: string[] = [];
    for (const [position, alternative] of (alternatives as Dict[]).entries()) {
      const tag = tags[position]!;
      const parts: string[] = [];
      for (const member of Array.isArray(alternative.allOf) ? alternative.allOf : []) {
        const call = isDict(member) ? parserExpression(member, row, path, context, depth + 1) : undefined;
        if (call === undefined) return undefined;
        parts.push(`...${call}`);
      }
      const entries = orderedEntries(alternative.properties as Dict);
      if (entries === null) return undefined;
      const rest: Dict = { ...alternative, properties: Object.fromEntries(entries.filter(([field]) => field !== key)) };
      const variant = (declared === undefined ? [] : aliasMembers(context.own, declared)).find((members) => stripQuotes(members.get(key) ?? "") === tag);
      const fields = truthy(rest.properties) ? objectFields(rest, row, path, context, depth + 1, undefined, variant) : [];
      if (fields === undefined) return undefined;
      const closed = context.closed && alternative.additionalProperties === false && parts.length > 0 ? undefined : closedRead(alternative, row, at, Object.keys(alternative.properties as Dict), context);
      if (closed === undefined) return undefined;
      cases.push(`case ${jsonString(tag, true)}: ${closed === row ? "" : `${closed}; `}return { ${[`${keyText(key)}: ${jsonString(tag, true)}`, ...parts, ...fields].join(", ")} };`);
    }
    const members = tags.map((tag) => jsonString(tag, true)).join(", ");
    return `((${row}) => { switch (${helper}Member(${row}[${jsonString(key, true)}], \`${path}.${key}\`, [${members}] as const)) { ${cases.join(" ")} } })(${helper}Object(${access}, ${at}))`;
  }
  if (alternatives.length > 0 && alternatives.every((alternative) => isDict(alternative) && alternative.type === "object" && isDict(alternative.properties) && Object.keys(alternative.properties).length === 1)) {
    const branches: string[] = [];
    for (const alternative of alternatives as Dict[]) {
      const [field, shape] = Object.entries(alternative.properties as Dict)[0]!;
      const variants = declared === undefined ? new Map<string, string>() : aliasVariants(context.own, declared);
      const expression = parserExpression(asDict(shape), `${row}[${jsonString(field, true)}]`, `${path}.${field}`, context, depth + 1, variants.get(field));
      if (expression === undefined) return undefined;
      const closed = closedRead(alternative, row, at, [field], context);
      if (closed === undefined) return undefined;
      branches.push(`if (${row}[${jsonString(field, true)}] !== undefined) ${closed === row ? "" : `{ ${closed}; `}return { ${keyText(field)}: ${expression} };${closed === row ? "" : " }"}`);
    }
    return `((${row}) => { ${branches.join(" ")} return ${helper}Reject(${at}, "value matches no variant"); })(${helper}Object(${access}, ${at}))`;
  }
  return alternativesExpression(alternatives as Dict[], access, path, context, depth);
}

type JsonClass = "string" | "number" | "boolean" | "null" | "array" | "object";

const CLASS_TESTS: Record<JsonClass, (value: string) => string> = {
  string: (value) => `typeof ${value} === "string"`,
  number: (value) => `typeof ${value} === "number"`,
  boolean: (value) => `typeof ${value} === "boolean"`,
  null: (value) => `${value} === null`,
  array: (value) => `Array.isArray(${value})`,
  object: (value) => `typeof ${value} === "object" && ${value} !== null && !Array.isArray(${value})`,
};

const classOfValue = (value: unknown): JsonClass | undefined =>
  value === null ? "null" : typeof value === "string" ? "string" : typeof value === "number" || typeof value === "bigint" ? "number" : typeof value === "boolean" ? "boolean" : Array.isArray(value) ? "array" : isDict(value) ? "object" : undefined;

/** 📘️ The one JSON class an alternative admits, when it states one (a reference states none). */
function jsonClass(node: Dict): JsonClass | undefined {
  if (has(node, "$ref")) return undefined;
  if (has(node, "const")) return classOfValue(node.const);
  if (Array.isArray(node.enum) && node.enum.length > 0) {
    const classes = new Set(node.enum.map(classOfValue));
    return classes.size === 1 ? [...classes][0] : undefined;
  }
  const kind = node.type;
  if (kind === "integer" || kind === "number") return "number";
  return kind === "string" || kind === "boolean" || kind === "null" || kind === "array" || kind === "object" ? kind : undefined;
}

/** 🆕️ A value that matches the first of several alternatives (`anyOf`, an untagged `oneOf`, a `type` list): each alternative whose JSON class fits is tried in order and its refusal is swallowed, and the value is refused when none accepts it. */
function alternativesExpression(alternatives: readonly Dict[], access: string, path: string, context: Context, depth: number): string | undefined {
  const helper = context.helper;
  const input = `input${depth}`;
  const branches: string[] = [];
  for (const alternative of alternatives) {
    if (!isDict(alternative)) return undefined;
    const expression = parserExpression(alternative, input, path, context, depth + 1);
    if (expression === undefined) return undefined;
    const attempt = `try { return ${expression}; } catch (error) { if (!(error instanceof ${helper}Refusal)) throw error; }`;
    const kind = jsonClass(alternative);
    branches.push(kind === undefined ? attempt : `if (${CLASS_TESTS[kind](input)}) { ${attempt} }`);
  }
  return alternatives.length === 0 ? undefined : `((${input}: unknown) => { ${branches.join(" ")} return ${helper}Reject(\`${path}\`, "value matches no variant"); })(${access})`;
}
//#endregion 🔖️Objects

//#region 🔖️Expressions
/** 🧬️ The parser expression of one schema node. `path` is the INNER text of a template literal (for example `${at}.edits`), so an array element extends it positionally without nesting one template literal in another; `depth` names the row, item and index bindings of nested levels so an inner level never shadows an outer one. A node that states no structure (or any value) whose declared type instantiates a generic of the codebase (`Box<Point>`) has no expression: an opaque read cannot produce that type, so the parser is refused instead of emitted with a type error or a cast. */
export function parserExpression(node: Dict, access: string, path: string, context: Context, depth = 0, declared?: string): string | undefined {
  const helper = context.helper;
  const at = `\`${path}\``;
  if (has(node, "$ref")) return typeof node.$ref === "string" ? referenceCall(node.$ref, access, at, context, path, depth) : undefined;
  if (declared !== undefined && (opaque(node) || isAny(node) || (node.type === "array" && !Array.isArray(node.items) && !fixedItems(node) && declared.startsWith("[") && declared.endsWith("]")))) {
    const typed = typedExpression(declared, access, path, context, depth);
    if (typed !== undefined) return typed;
    if (instantiatesGeneric(declared, (name) => aliasText(context.declared, name))) return undefined;
  }
  const nullable = nullableOf(node);
  if (nullable !== undefined) {
    const inner = declared === undefined ? undefined : declared.replace(/\s*\|\s*null\s*$/u, "");
    const expression = parserExpression(nullable, access, path, context, depth, inner);
    return expression === undefined ? undefined : `(${access} === null ? null : ${expression})`;
  }
  if (Array.isArray(node.enum) && node.enum.every((entry) => typeof entry === "string")) {
    return `${helper}Member(${access}, ${at}, [${node.enum.map((entry) => jsonString(entry, false)).join(", ")}] as const)`;
  }
  if (Array.isArray(node.enum) && node.enum.length > 0 && node.enum.every((entry) => typeof entry === "string" || typeof entry === "number" || typeof entry === "bigint" || typeof entry === "boolean")) {
    const members = node.enum.map((entry) => jsonValue(entry, false)).join(", ");
    const input = `input${depth}`;
    return `((${input}: unknown) => ([${members}] as readonly unknown[]).includes(${input}) ? (${input} as ${node.enum.map((entry) => jsonValue(entry, false)).join(" | ")}) : ${helper}Reject(${at}, "value is not one of ${members.replace(/["\\`$]/gu, "")}"))(${access})`;
  }
  if (has(node, "const") && (typeof node.const === "string" || typeof node.const === "number" || typeof node.const === "bigint" || typeof node.const === "boolean")) {
    return `${helper}Constant(${access}, ${at}, ${jsonValue(node.const, false)})`;
  }
  if (has(node, "const") && (Array.isArray(node.const) || isDict(node.const) || node.const === null)) return constantExpression(node.const, access, path, context, depth);
  for (const key of ["oneOf", "anyOf"]) {
    const alternatives = node[key];
    if (Array.isArray(alternatives)) return unionExpression(alternatives, access, path, context, depth, declared);
  }
  if (Array.isArray(node.allOf) && node.allOf.length === 1 && isDict(node.allOf[0]) && !has(node, "properties")) return parserExpression(node.allOf[0], access, path, context, depth);
  if (Array.isArray(node.allOf) && node.allOf.length > 1 && !has(node, "properties") && node.allOf.every((part) => isDict(part) && (has(part, "$ref") || part.type === "object"))) return mergedExpression(node.allOf as Dict[], access, path, context, depth);
  const kind = node.type;
  if (Array.isArray(kind)) return alternativesExpression(kind.map((entry) => ({ ...node, type: entry })), access, path, context, depth);
  if (kind === "null") return `(${access} === null ? null : ${helper}Reject(${at}, "value is not null"))`;
  if (kind === undefined && isAny(node)) return access;
  if (kind === "string") return `${helper}String(${access}, ${at}${boundsText(node, ["minLength", "maxLength", "pattern"])})`;
  if (kind === "boolean") return `${helper}Boolean(${access}, ${at})`;
  if (kind === "integer") return `${helper}Integer(${access}, ${at}${boundsText(node, ["minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum"])})`;
  if (kind === "number") return `${helper}Number(${access}, ${at}${boundsText(node, ["minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum"])})`;
  if (kind === "array") return arrayExpression(node, access, path, context, depth, declared);
  if (kind === "object") return objectExpression(node, access, path, context, depth, declared);
  return undefined;
}

/** 🔀️ An object that satisfies every part of an `allOf` of objects: each part reads the same row and the parts are merged, so a field a later part states wins. */
function mergedExpression(parts: readonly Dict[], access: string, path: string, context: Context, depth: number): string | undefined {
  const helper = context.helper;
  const at = `\`${path}\``;
  const row = `row${depth}`;
  const calls: string[] = [];
  for (const part of parts) {
    const call = parserExpression(part, row, path, context, depth + 1);
    if (call === undefined) return undefined;
    calls.push(`...${call}`);
  }
  return `((${row}) => ({ ${calls.join(", ")} }))(${helper}Object(${access}, ${at}))`;
}

/** 🗿️ A value equal to a constant that is no scalar: `null`, a tuple of constants or an object of constants (an object admits no other property). */
function constantExpression(value: unknown, access: string, path: string, context: Context, depth: number): string | undefined {
  const helper = context.helper;
  const at = `\`${path}\``;
  if (value === null) return `(${access} === null ? null : ${helper}Reject(${at}, "value is not null"))`;
  if (typeof value === "string" || typeof value === "number" || typeof value === "bigint" || typeof value === "boolean") return `${helper}Constant(${access}, ${at}, ${jsonValue(value, false)})`;
  if (Array.isArray(value)) {
    const names = `items${depth}`;
    const elements = value.map((member, index) => constantExpression(member, `${names}[${index}]`, `${path}[${index}]`, context, depth + 1));
    return elements.some((element) => element === undefined) ? undefined : `((${names}) => [${elements.join(", ")}] as const)(${helper}Array(${access}, ${at}${exactItems(value.length)}))`;
  }
  if (isDict(value)) {
    const row = `row${depth}`;
    const entries = orderedEntries(value);
    if (entries === null) return undefined;
    const fields = entries.map(([field, member]) => {
      const expression = constantExpression(member, `${row}[${jsonString(field, true)}]`, `${path}.${field}`, context, depth + 1);
      return expression === undefined ? undefined : `${keyText(field)}: ${expression}`;
    });
    if (fields.some((field) => field === undefined)) return undefined;
    return `((${row}) => ({ ${fields.join(", ")} }))(${helper}Closed(${helper}Object(${access}, ${at}), ${at}, ${keyList(entries.map(([field]) => field))}))`;
  }
  return undefined;
}

function arrayExpression(node: Dict, access: string, path: string, context: Context, depth: number, declared?: string): string | undefined {
  const helper = context.helper;
  const at = `\`${path}\``;
  const items = node.items;
  const limits = boundsText(node, ["minItems", "maxItems"]);
  if (Array.isArray(items)) {
    const names = `items${depth}`;
    const elements = items.map((member, index) => parserExpression(asDict(member), `${names}[${index}]`, `${path}[${index}]`, context, depth + 1));
    if (elements.some((element) => element === undefined)) return undefined;
    return `((${names}) => [${elements.join(", ")}])(${helper}Array(${access}, ${at}${exactItems(items.length)}))`;
  }
  const fixed = node.minItems === node.maxItems && typeof node.minItems === "number" && node.minItems >= 1 && node.minItems <= MAX_TUPLE ? node.minItems : undefined;
  if (isDict(items) && truthy(items) && fixed !== undefined) {
    const names = `items${depth}`;
    const elements = Array.from({ length: fixed }, (_, position) => parserExpression(items, `${names}[${position}]`, `${path}[${position}]`, context, depth + 1, declared !== undefined && declared.endsWith("[]") ? declared.slice(0, -2) : undefined));
    return elements.some((element) => element === undefined) ? undefined : `((${names}) => [${elements.join(", ")}])(${helper}Array(${access}, ${at}${exactItems(fixed)}))`;
  }
  if (isDict(items) && truthy(items)) {
    const [item, index] = itemBindings(depth);
    const element = parserExpression(items, item, `${path}[\${${index}}]`, context, depth + 1, declared !== undefined && declared.endsWith("[]") ? declared.slice(0, -2) : undefined);
    return element === undefined ? undefined : `${helper}Array(${access}, ${at}${limits}).map((${item}, ${index}) => ${element})`;
  }
  return `[...${helper}Array(${access}, ${at}${limits})]`;
}

function objectExpression(node: Dict, access: string, path: string, context: Context, depth: number, declared?: string): string | undefined {
  const helper = context.helper;
  const at = `\`${path}\``;
  if (isDict(node.properties) && truthy(node.properties)) {
    const row = `row${depth}`;
    const named = declared !== undefined && BARE_TYPE_NAME.test(declared) ? declared : undefined;
    const known = named ? declaredTypes(context.declared, named) : declared !== undefined && declared.startsWith("{") && declared.endsWith("}") ? memberTypes(declared.slice(1, -1)) : undefined;
    const fields = objectFields(node, row, path, context, depth + 1, undefined, known);
    const read = closedRead(node, `${helper}Object(${access}, ${at})`, at, Object.keys(node.properties), context);
    return fields === undefined || read === undefined ? undefined : `((${row}) => ({ ${fields.join(", ")} }))(${read})`;
  }
  const extra = node.additionalProperties;
  if (isDict(extra) && truthy(extra)) {
    const [key, value] = [`key${depth}`, `value${depth}`];
    const element = parserExpression(extra, value, `${path}.\${${key}}`, context, depth + 1);
    return element === undefined ? undefined : `Object.fromEntries(Object.entries(${helper}Object(${access}, ${at})).map(([${key}, ${value}]) => [${key}, ${element}]))`;
  }
  return closedRead(node, `${helper}Object(${access}, ${at})`, at, [], context);
}
//#endregion 🔖️Expressions

//#region 🔖️Parsers
/** 🖌️ The `export function parse<name>` source of one export, or `undefined` when the schema has a form no renderer covers. */
export function renderParser(name: string, definition: Dict, context: Context, declared?: ReadonlyMap<string, boolean>): string | undefined {
  context.self = name;
  const discriminated = Array.isArray(definition.oneOf) || Array.isArray(definition.anyOf);
  if (definition.type === "object" && isDict(definition.properties) && truthy(definition.properties) && !discriminated) {
    const lines = objectFields(definition, "row", "${at}", context, 0, declared, declaredTypes(context.declared, name));
    if (lines === undefined) return undefined;
    const body = lines.map((line) => `    ${line},`).join("\n");
    const read = closedRead(definition, `${context.helper}Object(value, at)`, "at", Object.keys(definition.properties), context);
    if (read === undefined) return undefined;
    return `export function parse${name}(value: unknown, at = "$"): ${name} {\n  const row = ${read};\n  return {\n${body}\n  };\n}\n`;
  }
  const alias = aliasText(context.declared, name);
  const aliased = alias !== undefined && alias.startsWith("[") && definition.type === "array" && !Array.isArray(definition.items) ? typedExpression(alias, "value", "${at}", context, 0) : undefined;
  const expression = aliased || parserExpression(definition, "value", "${at}", context, 0, name);
  return expression ? `export function parse${name}(value: unknown, at = "$"): ${name} {\n  return ${expression};\n}\n` : undefined;
}
//#endregion 🔖️Parsers
