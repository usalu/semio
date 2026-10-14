/** 🧩️ Describes a non-null, non-array object whose member values are unknown. */
export type UnknownRecord = Record<string, unknown>;

/** 🛡️ Admits non-null objects without cloning or requiring plain JSON identity. */
export function requireRecord(value: unknown, name: string): UnknownRecord {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error(`${name} must be an object`);
  return value as UnknownRecord;
}

/** 🧵️ Preserves string-array identity and the language's occupied-slot semantics. */
export function requireStringArray(value: unknown, name: string): readonly string[] {
  if (!Array.isArray(value) || value.some(item => typeof item !== "string")) throw new Error(`${name} must be a string array`);
  return value;
}

/** 📝️ Requires a non-empty string without trimming or changing its spelling. */
export function requireString(value: unknown, name: string): string {
  if (typeof value !== "string" || value.length === 0) throw new Error(`${name} must be a non-empty string`);
  return value;
}

/** 🔒️ Narrows an unchanged string to an explicitly admitted literal. */
export function requireLiteral<T extends string>(value: unknown, name: string, allowed: readonly T[]): T {
  const text = requireString(value, name);
  if (!allowed.includes(text as T)) throw new Error(`${name} must be one of ${allowed.join(", ")}`);
  return text as T;
}

/** 🔑️ Requires exactly the declared enumerable own keys without changing either input. */
export function requireExactKeys(value: UnknownRecord, keys: readonly string[], name: string): void {
  const actual = Object.keys(value).sort(), expected = [...keys].sort();
  if (actual.length !== expected.length || actual.some((key, index) => key !== expected[index])) throw new Error(`${name} must contain exactly ${expected.join(", ")}`);
}

//#region 🔣️JsonSchemaSubset
function jsonSchemaSubsetObject(value: unknown): Record<string, unknown> | undefined {
  return typeof value === "object" && value !== null && !Array.isArray(value) ? value as Record<string, unknown> : undefined;
}


/** ⚖️ Compares JSON values with ordered arrays and unordered object members. */
export function jsonSchemaSubsetValueEquals(left: unknown, right: unknown): boolean {
  if (left === right) return true;
  if (typeof left !== "object" || left === null || typeof right !== "object" || right === null || Array.isArray(left) !== Array.isArray(right)) return false;
  if (Array.isArray(left)) return left.length === (right as unknown[]).length && left.every((member, index) => jsonSchemaSubsetValueEquals(member, (right as unknown[])[index]));
  const a = left as Record<string, unknown>, b = right as Record<string, unknown>, keys = Object.keys(a);
  return keys.length === Object.keys(b).length && keys.every(key => Object.hasOwn(b, key) && jsonSchemaSubsetValueEquals(a[key], b[key]));
}


function jsonSchemaSubsetValueKey(value: unknown): string {
  if (Array.isArray(value)) return `[${value.map(jsonSchemaSubsetValueKey).join(",")}]`;
  if (value !== null && typeof value === "object") return `{${Object.keys(value).sort().map(key => `${JSON.stringify(key)}:${jsonSchemaSubsetValueKey(Reflect.get(value, key))}`).join(",")}}`;
  return `${typeof value}:${JSON.stringify(value)}`;
}
function jsonSchemaSubsetDuplicates(values: readonly unknown[]): boolean {
  const buckets = new Map<string, unknown[]>();
  for (const value of values) {
    const key = jsonSchemaSubsetValueKey(value), bucket = buckets.get(key);
    if (bucket?.some(previous => jsonSchemaSubsetValueEquals(previous, value))) return true;
    if (bucket) bucket.push(value); else buckets.set(key, [value]);
  }
  return false;
}

function jsonSchemaSubsetTypeMatches(type: unknown, value: unknown): boolean {
  if (Array.isArray(type)) return type.some((candidate) => jsonSchemaSubsetTypeMatches(candidate, value));
  if (type === "object") return jsonSchemaSubsetObject(value) !== undefined;
  if (type === "array") return Array.isArray(value);
  if (type === "string") return typeof value === "string";
  if (type === "integer") return typeof value === "number" && Number.isInteger(value);
  if (type === "number") return typeof value === "number" && Number.isFinite(value);
  if (type === "boolean") return typeof value === "boolean";
  if (type === "null") return value === null;
  return true;
}


/** 🧭️ Resolution context: the document a local `#/…` pointer walks, plus every sibling document a cross-document `$ref` names by `$id`. */
interface JsonSchemaSubsetScope { readonly root: unknown; readonly documents: ReadonlyMap<string, unknown> }

/** 🔗️ Resolves `$ref` to its document and node: `#/…` or the root's own `$id` inside the root, `<$id>`/`<$id>#/…` into a sibling document. */
function jsonSchemaSubsetResolve(scope: JsonSchemaSubsetScope, reference: string): JsonSchemaSubsetScope & { readonly target: unknown } | undefined {
  const hash = reference.indexOf("#"), id = hash < 0 ? reference : reference.slice(0, hash), fragment = hash < 0 ? "" : reference.slice(hash + 1);
  const root = id === "" || jsonSchemaSubsetObject(scope.root)?.$id === id ? scope.root : scope.documents.get(id);
  if (root === undefined || (fragment !== "" && !fragment.startsWith("/"))) return undefined;
  const target = fragment.split("/").filter((segment) => segment.length > 0).reduce<unknown>((node, segment) => {
    const key = decodeURIComponent(segment.replace(/~1/g, "/").replace(/~0/g, "~"));
    return Array.isArray(node) ? (/^\d+$/.test(key) ? node[Number(key)] : undefined) : jsonSchemaSubsetObject(node)?.[key];
  }, root);
  return target === undefined ? undefined : { root, documents: scope.documents, target };
}

/** 📚️ Indexes sibling schema documents by their required, unique `$id`. */
function jsonSchemaSubsetDocuments(documents: readonly unknown[]): ReadonlyMap<string, unknown> {
  const indexed = new Map<string, unknown>();
  for (const document of documents) {
    const id = jsonSchemaSubsetObject(document)?.$id;
    if (typeof id !== "string") throw new Error("sibling schema document requires an `$id`");
    if (indexed.has(id)) throw new Error(`duplicate sibling schema \`$id\` ${id}`);
    indexed.set(id, document);
  }
  return indexed;
}


function jsonSchemaSubsetSnapshot(value: unknown, copies = new Map<object, unknown>()): unknown {
  if (typeof value !== "object" || value === null) return value;
  const prior = copies.get(value);
  if (prior !== undefined) return prior;
  const result: Record<string, unknown> | unknown[] = Array.isArray(value) ? new Array(value.length) : Object.create(null);
  copies.set(value, result);
  for (const key of Object.keys(value)) Reflect.set(result, key, jsonSchemaSubsetSnapshot(Reflect.get(value, key), copies));
  return result;
}

type JsonSchemaSubsetCheck = (value: unknown, path: string) => string[];

/** 🧱️ An explicitly prepared snapshot owns reusable schema decisions and observes each supplied value anew. */
export interface CompiledJsonSchemaSubset { validate(value: unknown): string[] }

/** ⚡️ Compiles references, regexes and property tables once without retaining mutable caller schema aliases. */
export function compileJsonSchemaSubset(schema: unknown, root: unknown = schema, documents: readonly unknown[] = []): CompiledJsonSchemaSubset {
  const snapshot = jsonSchemaSubsetSnapshot({ schema, root, documents }) as { schema: unknown; root: unknown; documents: readonly unknown[] };
  const scope: JsonSchemaSubsetScope = { root: snapshot.root, documents: jsonSchemaSubsetDocuments(snapshot.documents) };
  const plans = new Map<unknown, Map<unknown, JsonSchemaSubsetCheck>>();
  const prepare = (schema: unknown, scope: JsonSchemaSubsetScope): JsonSchemaSubsetCheck => {
    const contract = jsonSchemaSubsetObject(schema);
    if (!contract) return schema === true ? () => [] : (_value, path) => [`${path} schema must be an object`];
    let scoped = plans.get(scope.root);
    if (!scoped) { scoped = new Map(); plans.set(scope.root, scoped); }
    const prior = scoped.get(contract);
    if (prior) return prior;
    let ready: JsonSchemaSubsetCheck;
    const check: JsonSchemaSubsetCheck = (value, path) => ready(value, path);
    scoped.set(contract, check);
    const resolved = typeof contract.$ref === "string" ? jsonSchemaSubsetResolve(scope, contract.$ref) : undefined;
    const reference = resolved ? prepare(resolved.target, resolved) : undefined;
    const all = Array.isArray(contract.allOf) ? contract.allOf.map(member => prepare(member, scope)) : [];
    const any = Array.isArray(contract.anyOf) ? contract.anyOf.map(member => prepare(member, scope)) : undefined;
    const one = Array.isArray(contract.oneOf) ? contract.oneOf.map(member => prepare(member, scope)) : undefined;
    const enums = Array.isArray(contract.enum) ? contract.enum : undefined;
    const pattern = typeof contract.pattern === "string" ? new RegExp(contract.pattern, "u") : undefined;
    const items = "items" in contract ? prepare(contract.items, scope) : undefined;
    const contains = "contains" in contract ? prepare(contract.contains, scope) : undefined;
    const required = Array.isArray(contract.required) ? contract.required.filter((key): key is string => typeof key === "string") : [];
    const properties = jsonSchemaSubsetObject(contract.properties) ?? {};
    const entries = Object.entries(properties).map(([key, child]) => [key, prepare(child, scope)] as const);
    const patterns = Object.entries(jsonSchemaSubsetObject(contract.patternProperties) ?? {}).map(([pattern, child]) => [new RegExp(pattern, "u"), prepare(child, scope)] as const);
    const names = "propertyNames" in contract ? prepare(contract.propertyNames, scope) : undefined;
    const additional = jsonSchemaSubsetObject(contract.additionalProperties) ? prepare(contract.additionalProperties, scope) : undefined;
    ready = (value, path) => {
      const errors: string[] = [];
      if (typeof contract.$ref === "string") {
        if (reference) errors.push(...reference(value, path));
        else errors.push(`${path} references unresolvable ${contract.$ref}`);
      }
      if ("const" in contract && !jsonSchemaSubsetValueEquals(value, contract.const)) errors.push(`${path} must equal its const`);
      if (enums && !enums.some(candidate => jsonSchemaSubsetValueEquals(value, candidate))) errors.push(`${path} must be an allowed enum value`);
      for (const member of all) errors.push(...member(value, path));
      if (any && !any.some(member => member(value, path).length === 0)) errors.push(`${path} must match one anyOf branch`);
      if (one && one.filter(member => member(value, path).length === 0).length !== 1) errors.push(`${path} must match exactly one oneOf branch`);
      if (!jsonSchemaSubsetTypeMatches(contract.type, value)) { errors.push(`${path} must be ${String(contract.type)}`); return errors; }
      if (typeof value === "string") {
        const length = [...value].length;
        if (typeof contract.minLength === "number" && length < contract.minLength) errors.push(`${path} must contain at least ${contract.minLength} character(s)`);
        if (typeof contract.maxLength === "number" && length > contract.maxLength) errors.push(`${path} must contain at most ${contract.maxLength} character(s)`);
        if (pattern && !pattern.test(value)) errors.push(`${path} must match ${contract.pattern}`);
      }
      if (typeof value === "number" && typeof contract.minimum === "number" && value < contract.minimum) errors.push(`${path} must be at least ${contract.minimum}`);
      if (typeof value === "number" && typeof contract.maximum === "number" && value > contract.maximum) errors.push(`${path} must be at most ${contract.maximum}`);
      if (typeof value === "number" && typeof contract.exclusiveMinimum === "number" && value <= contract.exclusiveMinimum) errors.push(`${path} must be greater than ${contract.exclusiveMinimum}`);
      if (typeof value === "number" && typeof contract.exclusiveMaximum === "number" && value >= contract.exclusiveMaximum) errors.push(`${path} must be less than ${contract.exclusiveMaximum}`);
      if (Array.isArray(value)) {
        if (typeof contract.minItems === "number" && value.length < contract.minItems) errors.push(`${path} must contain at least ${contract.minItems} item(s)`);
        if (typeof contract.maxItems === "number" && value.length > contract.maxItems) errors.push(`${path} must contain at most ${contract.maxItems} item(s)`);
        if (contract.uniqueItems === true && jsonSchemaSubsetDuplicates(value)) errors.push(`${path} items must be unique`);
        if (items) for (let index = 0; index < value.length; index++) errors.push(...items(value[index], `${path}/${index}`));
        if (contains && !value.some((item, index) => contains(item, `${path}/${index}`).length === 0)) errors.push(`${path} must contain a matching item`);
      }
      const object = jsonSchemaSubsetObject(value);
      if (object) {
        const keys = Object.keys(object), size = keys.length;
        if (typeof contract.minProperties === "number" && size < contract.minProperties) errors.push(`${path} must contain at least ${contract.minProperties} property(s)`);
        if (typeof contract.maxProperties === "number" && size > contract.maxProperties) errors.push(`${path} must contain at most ${contract.maxProperties} property(s)`);
        for (const key of required) if (!(key in object)) errors.push(`${path}/${key} is required`);
        for (const [key, child] of entries) if (key in object) errors.push(...child(object[key], `${path}/${key}`));
        for (const key of keys) {
          if (names) errors.push(...names(key, `${path}/${key} (name)`));
          let matched = false;
          for (const [pattern, child] of patterns) if (pattern.test(key)) { matched = true; errors.push(...child(object[key], `${path}/${key}`)); }
          if (key in properties || matched) continue;
          if (contract.additionalProperties === false) errors.push(`${path}/${key} is not allowed`);
          else if (additional) errors.push(...additional(object[key], `${path}/${key}`));
        }
      }
      return errors;
    };
    return check;
  };
  const check = prepare(snapshot.schema, scope);
  return Object.freeze({ validate: (value: unknown) => check(value, "") });
}

/** 🧭️ Validates the owned Draft-07 subset from current schema input; prepared callers reuse explicit immutable snapshots. */
export function validateJsonSchemaSubset(schema: unknown, value: unknown, root: unknown = schema, documents: readonly unknown[] = []): string[] {
  return compileJsonSchemaSubset(schema, root, documents).validate(value);
}
