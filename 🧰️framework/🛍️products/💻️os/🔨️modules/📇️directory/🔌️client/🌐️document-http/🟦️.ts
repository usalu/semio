type Schema = boolean | Readonly<Record<string, unknown>>;
const KEYWORDS = new Set(["$schema", "$id", "$ref", "$defs", "definitions", "title", "description", "type", "const", "enum", "oneOf", "anyOf", "allOf", "not", "properties", "required", "additionalProperties", "items", "additionalItems", "minItems", "maxItems", "minLength", "maxLength", "minimum", "maximum", "pattern"]);

/** 🧬 Compiles a finite closed JSON-schema subset; unsupported declarations fail before transport. */
export function compileDocumentJsonSchemaV1(source: string): (value: unknown) => boolean {
  const root: unknown = JSON.parse(source);
  const schemas = new Set<Schema>();
  const resolve = (reference: unknown): Schema => {
    if (typeof reference !== "string" || !reference.startsWith("#/") || reference.length > 1024) throw new Error("document-schema.invalid-reference");
    let current = root;
    for (const field of reference.slice(2).split("/").map((part) => part.replace(/~1/gu, "/").replace(/~0/gu, "~"))) {
      if (current === null || typeof current !== "object" || !Object.hasOwn(current, field)) throw new Error("document-schema.missing-reference");
      current = (current as Readonly<Record<string, unknown>>)[field];
    }
    return admit(current);
  };
  const admit = (value: unknown): Schema => {
    if (typeof value === "boolean") return value;
    if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("document-schema.invalid");
    const schema = value as Readonly<Record<string, unknown>>;
    if (schemas.has(schema)) return schema;
    if (schemas.size >= 512 || Object.keys(schema).some((key) => !KEYWORDS.has(key))) throw new Error("document-schema.unsupported");
    schemas.add(schema);
    if (schema.type !== undefined && (typeof schema.type !== "string" || !["null", "boolean", "object", "array", "number", "integer", "string"].includes(schema.type))) throw new Error("document-schema.invalid-type");
    for (const name of ["minimum", "maximum", "minItems", "maxItems", "minLength", "maxLength"]) if (schema[name] !== undefined && (typeof schema[name] !== "number" || !Number.isFinite(schema[name]))) throw new Error("document-schema.invalid-bound");
    if (schema.pattern !== undefined) { if (typeof schema.pattern !== "string" || schema.pattern.length > 256) throw new Error("document-schema.invalid-pattern"); new RegExp(schema.pattern, "u"); }
    if (schema.enum !== undefined && (!Array.isArray(schema.enum) || schema.enum.length < 1 || schema.enum.length > 256)) throw new Error("document-schema.invalid-enum");
    if (schema.required !== undefined && (!Array.isArray(schema.required) || schema.required.some((name) => typeof name !== "string") || new Set(schema.required).size !== schema.required.length)) throw new Error("document-schema.invalid-required");
    for (const name of ["properties", "$defs", "definitions"]) if (schema[name] !== undefined) {
      const entries = schema[name];
      if (typeof entries !== "object" || entries === null || Array.isArray(entries)) throw new Error("document-schema.invalid-properties");
      for (const child of Object.values(entries)) admit(child);
    }
    for (const name of ["oneOf", "anyOf", "allOf"]) if (schema[name] !== undefined) {
      const children = schema[name];
      if (!Array.isArray(children) || children.length < 1 || children.length > 64) throw new Error("document-schema.invalid-composition");
      children.forEach(admit);
    }
    for (const name of ["not", "additionalProperties", "additionalItems"]) if (schema[name] !== undefined) admit(schema[name]);
    if (schema.items !== undefined) Array.isArray(schema.items) ? schema.items.forEach(admit) : admit(schema.items);
    if (schema.$ref !== undefined) resolve(schema.$ref);
    return schema;
  };
  const compiled = admit(root);
  const equal = (left: unknown, right: unknown): boolean => {
    if (left === right) return true;
    if (typeof left !== "object" || left === null || typeof right !== "object" || right === null || Array.isArray(left) !== Array.isArray(right)) return false;
    const a = Object.keys(left), b = Object.keys(right);
    return a.length === b.length && a.every((key) => Object.hasOwn(right, key) && equal(Reflect.get(left, key), Reflect.get(right, key)));
  };
  const matches = (schema: Schema, value: unknown, depth: number): boolean => {
    if (depth > 64) return false;
    if (typeof schema === "boolean") return schema;
    if (schema.$ref !== undefined) return matches(resolve(schema.$ref), value, depth + 1);
    const type = schema.type;
    if (type === "null" && value !== null || type === "boolean" && typeof value !== "boolean" || type === "string" && typeof value !== "string" || type === "number" && (typeof value !== "number" || !Number.isFinite(value)) || type === "integer" && (typeof value !== "number" || !Number.isSafeInteger(value)) || type === "array" && !Array.isArray(value) || type === "object" && (typeof value !== "object" || value === null || Array.isArray(value))) return false;
    if (Object.hasOwn(schema, "const") && !equal(schema.const, value) || schema.enum !== undefined && !(schema.enum as unknown[]).some((item) => equal(item, value))) return false;
    for (const name of ["oneOf", "anyOf", "allOf"] as const) if (schema[name] !== undefined) {
      const children = schema[name] as Schema[];
      const count = children.filter((child) => matches(child, value, depth + 1)).length;
      if (name === "oneOf" && count !== 1 || name === "anyOf" && count === 0 || name === "allOf" && count !== children.length) return false;
    }
    if (schema.not !== undefined && matches(schema.not as Schema, value, depth + 1)) return false;
    if (typeof value === "number" && (typeof schema.minimum === "number" && value < schema.minimum || typeof schema.maximum === "number" && value > schema.maximum)) return false;
    if (typeof value === "string") {
      const length = Array.from(value).length;
      if (typeof schema.minLength === "number" && length < schema.minLength || typeof schema.maxLength === "number" && length > schema.maxLength || typeof schema.pattern === "string" && !new RegExp(schema.pattern, "u").test(value)) return false;
    }
    if (Array.isArray(value)) {
      if (typeof schema.minItems === "number" && value.length < schema.minItems || typeof schema.maxItems === "number" && value.length > schema.maxItems) return false;
      if (Array.isArray(schema.items)) {
        if (!value.every((item, index) => matches((schema.items as Schema[])[index] ?? (schema.additionalItems as Schema | undefined) ?? true, item, depth + 1))) return false;
      } else if (schema.items !== undefined && !value.every((item) => matches(schema.items as Schema, item, depth + 1))) return false;
    } else if (typeof value === "object" && value !== null) {
      const fields = value as Readonly<Record<string, unknown>>;
      if (schema.required !== undefined && (schema.required as string[]).some((key) => !Object.hasOwn(fields, key))) return false;
      const properties = (schema.properties ?? {}) as Readonly<Record<string, Schema>>;
      if (!Object.keys(fields).every((key) => matches(Object.hasOwn(properties, key) ? properties[key]! : (schema.additionalProperties as Schema | undefined) ?? true, fields[key], depth + 1))) return false;
    }
    return true;
  };
  return (value) => matches(compiled, value, 0);
}
