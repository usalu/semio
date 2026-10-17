/** 🧾️ Renders one schema export as a typed `parse<Export>` function: the expression for every schema form, the callees it needs and the source it reads its declared types from.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts
 */
import { BARE_TYPE_NAME, EXPORT_NAME, asDict, has, isDict, jsonString, jsonValue, keyText, orderedEntries, truthy, type Dict } from "../🔡️json-text/🟦️.ts";
import { aliasMembers, aliasText, aliasVariants, declaredTypes, definesParser, importsParser, memberTypes, normalizeType, reexportedBinding, splitTop, unionMembers } from "../📖️declared-types/🟦️.ts";
import { nullableOf } from "../📐️type-render/🟦️.ts";

//#region 🔖️Context
/** 🧭️ One schema document's TypeScript leaf and the exports it states, addressed by the document's `$id`. */
export class Owner {
  constructor(readonly ts: string, readonly exports: ReadonlySet<string>) {}
}

/** 📚️ The text of every source the projection reads, cached on first read so one run sees one consistent tree. */
export interface SourceReader {
  read(path: string): string;
  declarations(path: string): string;
}

/** 🔗️ A parser named by the file that declares it. */
export type ParserKey = string;

/** 🔗️ The key of parser `name` declared (or promised) in `owner`. */
export const parserKey = (owner: string, name: string): ParserKey => `${owner}\u0000${name}`;

/** 🔗️ The owner and the name a parser key addresses. */
export function parserKeyParts(key: ParserKey): [string, string] {
  const at = key.indexOf("\u0000");
  return [key.slice(0, at), key.slice(at + 1)];
}

/** 🧭️ What one parser rendering needs from its surroundings, and what it learned about its callees. */
export class Context {
  readonly local = new Set<string>();
  readonly foreign = new Set<ParserKey>();
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
  ) {}

  get own(): string {
    return this.sources.read(this.target);
  }

  get declared(): string {
    return this.sources.declarations(this.target);
  }
}

/** 🔎️ `parse<name>` as an in-scope call: this file first, then a sibling facet of the same module. */
function locateParser(name: string, context: Context): string | undefined {
  const own = context.own;
  if (definesParser(own, name) || importsParser(own, name) || context.promised.has(parserKey(context.target, name))) {
    context.local.add(name);
    return name;
  }
  for (const sibling of context.siblings) {
    if (sibling !== context.target && (definesParser(context.sources.read(sibling), name) || context.promised.has(parserKey(sibling, name)))) {
      context.foreign.add(parserKey(sibling, name));
      return name;
    }
  }
  return undefined;
}

/** 🔎️ The identifier a call of `parse<name>` uses inside this file: the local binding when the file exports it under an alias, `parse<name>` otherwise. */
const calleeOf = (name: string, context: Context): string => reexportedBinding(context.own, `parse${name}`) ?? `parse${name}`;

function referenceCall(reference: string, access: string, at: string, context: Context): string | undefined {
  const hash = reference.indexOf("#");
  const document = hash < 0 ? reference : reference.slice(0, hash);
  const target = hash < 0 ? "" : reference.slice(reference.indexOf("#") + 1).split("/").at(-1)!;
  if (!EXPORT_NAME.test(target)) return undefined;
  if (document && document !== context.documentId) {
    const owner = context.index.get(document);
    if (owner === undefined || !owner.exports.has(target)) return undefined;
    context.foreign.add(parserKey(owner.ts, target));
  } else context.local.add(target);
  return `${document && document !== context.documentId ? `parse${target}` : calleeOf(target, context)}(${access}, ${at})`;
}
//#endregion 🔖️Context

//#region 🔖️Fragments
/** 📏️ The schema's own bounds, passed to the guard so the parser refuses what the schema refuses. */
export function boundsText(node: Dict, keys: readonly string[]): string {
  const stated: Record<string, unknown> = {};
  for (const key of keys) if (has(node, key) && (typeof node[key] === "number" || typeof node[key] === "bigint" || typeof node[key] === "string")) stated[key] = node[key];
  return truthy(stated) ? `, ${jsonValue(stated, false)}` : "";
}

/** 🔎️ A schema node that states no structure (`{"type": "object"}` or an `array` without `items`); an object that is closed (`additionalProperties: false`) states its structure: nothing. */
const opaque = (node: Dict): boolean =>
  !has(node, "$ref") && ((node.type === "object" && !truthy(node.properties) && !isDict(node.additionalProperties) && node.additionalProperties !== false) || (node.type === "array" && !truthy(node.items)));

/** 🔒️ The property names a closed object states, as the array literal `<prefix>Closed` takes. */
const keyList = (keys: readonly string[]): string => `[${keys.map((key) => jsonString(key, false)).join(", ")}]`;

/** 🔒️ `read` wrapped in the refusal of unknown properties exactly where the schema says `additionalProperties: false`; `undefined` when the schema also states pattern properties, which the refusal cannot honour. */
function closedRead(node: Dict, read: string, at: string, keys: readonly string[], context: Context): string | undefined {
  if (!context.closed || node.additionalProperties !== false) return read;
  return has(node, "patternProperties") ? undefined : `${context.helper}Closed(${read}, ${at}, ${keyList(keys)})`;
}

const exactItems = (count: number): string => `, {"minItems": ${count}, "maxItems": ${count}}`;

/** 📏️ The longest fixed-size array a parser reads as a tuple. */
const MAX_TUPLE = 16;

const itemBindings = (depth: number): [string, string] => (depth === 0 ? ["item", "index"] : [`item${depth}`, `index${depth}`]);

const stripQuotes = (text: string): string => text.replace(/^['"]+|['"]+$/gu, "");
//#endregion 🔖️Fragments

//#region 🔖️Declared
/** 🧬️ A parser for a field whose type is only known from the interface that declares it (scalars, tuples, arrays, named types). The root value of an export never resolves to a parser of its own name, so an opaque export whose declared type names the export cannot call itself or import over its own declaration; a field of that type still recurses. */
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
/** 🧷️ The schema's own `default` of a property, as source text, when it is a JSON literal a parser can state (`null` is a value, not an absent default). */
function defaultText(shape: Dict): string | undefined {
  const fallback = shape.default;
  if (!has(shape, "default") || fallback === null) return undefined;
  return typeof fallback === "string" || typeof fallback === "number" || typeof fallback === "boolean" || Array.isArray(fallback) || isDict(fallback) ? jsonValue(fallback, false) : undefined;
}

function objectFields(node: Dict, row: string, path: string, context: Context, depth: number, declared?: ReadonlyMap<string, boolean>, types?: ReadonlyMap<string, string>): string[] | undefined {
  const required = new Set<unknown>(Array.isArray(node.required) ? node.required : []);
  const properties = asDict(node.properties);
  if (declared) for (const [field, optional] of declared) if (!optional && defaultText(asDict(properties[field])) === undefined) required.add(field);
  const entries = orderedEntries(properties);
  if (entries === null) return undefined;
  const lines: string[] = [];
  for (const [field, shape] of entries) {
    const access = `${row}[${jsonString(field, true)}]`;
    const fieldType = types?.get(field);
    const inner = nullableOf(asDict(shape));
    const absent = !required.has(field) && inner !== undefined && fieldType !== undefined && !unionMembers(fieldType).includes("null");
    const expression = parserExpression(absent ? inner : asDict(shape), access, `${path}.${field}`, context, depth, fieldType);
    if (expression === undefined) return undefined;
    const fallback = !required.has(field) && declared?.get(field) === false ? defaultText(asDict(shape)) : undefined;
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
  return undefined;
}
//#endregion 🔖️Objects

//#region 🔖️Expressions
/** 🧬️ The parser expression of one schema node. `path` is the INNER text of a template literal (for example `${at}.edits`), so an array element extends it positionally without nesting one template literal in another; `depth` names the row, item and index bindings of nested levels so an inner level never shadows an outer one. */
export function parserExpression(node: Dict, access: string, path: string, context: Context, depth = 0, declared?: string): string | undefined {
  const helper = context.helper;
  const at = `\`${path}\``;
  if (has(node, "$ref")) return typeof node.$ref === "string" ? referenceCall(node.$ref, access, at, context) : undefined;
  if (declared !== undefined && (opaque(node) || (node.type === "array" && !Array.isArray(node.items) && declared.startsWith("[") && declared.endsWith("]")))) {
    const typed = typedExpression(declared, access, path, context, depth);
    if (typed !== undefined) return typed;
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
  if (has(node, "const") && (typeof node.const === "string" || typeof node.const === "number" || typeof node.const === "bigint" || typeof node.const === "boolean")) {
    return `${helper}Constant(${access}, ${at}, ${jsonValue(node.const, false)})`;
  }
  for (const key of ["oneOf", "anyOf"]) {
    const alternatives = node[key];
    if (Array.isArray(alternatives)) return unionExpression(alternatives, access, path, context, depth, declared);
  }
  if (Array.isArray(node.allOf) && node.allOf.length === 1 && isDict(node.allOf[0]) && !has(node, "properties")) return parserExpression(node.allOf[0], access, path, context, depth);
  const kind = node.type;
  if (kind === "string") return `${helper}String(${access}, ${at}${boundsText(node, ["minLength", "maxLength", "pattern"])})`;
  if (kind === "boolean") return `${helper}Boolean(${access}, ${at})`;
  if (kind === "integer") return `${helper}Integer(${access}, ${at}${boundsText(node, ["minimum", "maximum"])})`;
  if (kind === "number") return `${helper}Number(${access}, ${at}${boundsText(node, ["minimum", "maximum"])})`;
  if (kind === "array") return arrayExpression(node, access, path, context, depth, declared);
  if (kind === "object") return objectExpression(node, access, path, context, depth, declared);
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
/** 🧾️ The `export function parse<name>` source of one export, or `undefined` when the schema has a form no renderer covers. */
export function renderParser(name: string, definition: Dict, context: Context, declared?: ReadonlyMap<string, boolean>): string | undefined {
  context.self = name;
  if (definition.type === "object" && isDict(definition.properties) && truthy(definition.properties)) {
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
