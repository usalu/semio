import { canonicalJson } from "../../🧹️normalization/🟦️.ts";

//#region 🔣️JsonSchemaSubset
export function jsonSchemaSubsetObject(value: unknown): Record<string, unknown> | undefined {
  return typeof value === "object" && value !== null && !Array.isArray(value) ? value as Record<string, unknown> : undefined;
}


export function jsonSchemaSubsetValueEquals(left: unknown, right: unknown): boolean {
  return canonicalJson(left) === canonicalJson(right);
}


export function jsonSchemaSubsetTypeMatches(type: unknown, value: unknown): boolean {
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


/** 🧭️ Resolves a document-local `#/…` JSON pointer against the root schema; any other reference is unresolvable here. */
export function jsonSchemaSubsetResolve(root: unknown, reference: string): unknown {
  if (!reference.startsWith("#")) return undefined;
  return reference.slice(1).split("/").filter((segment) => segment.length > 0).reduce<unknown>((node, segment) => jsonSchemaSubsetObject(node)?.[decodeURIComponent(segment.replace(/~1/g, "/").replace(/~0/g, "~"))], root);
}


export function jsonSchemaSubsetErrors(schema: unknown, value: unknown, path: string, root: unknown = schema): string[] {
  const contract = jsonSchemaSubsetObject(schema);
  if (!contract) return schema === true ? [] : [`${path} schema must be an object`];
  const errors: string[] = [];
  if (typeof contract.$ref === "string") {
    const target = jsonSchemaSubsetResolve(root, contract.$ref);
    if (target === undefined) errors.push(`${path} references unresolvable ${contract.$ref}`);
    else errors.push(...jsonSchemaSubsetErrors(target, value, path, root));
  }
  if ("const" in contract && !jsonSchemaSubsetValueEquals(value, contract.const)) errors.push(`${path} must equal its const`);
  const enumValues = Array.isArray(contract.enum) ? contract.enum : undefined;
  if (enumValues && !enumValues.some((candidate) => jsonSchemaSubsetValueEquals(value, candidate))) errors.push(`${path} must be an allowed enum value`);
  if (Array.isArray(contract.allOf)) for (const member of contract.allOf) errors.push(...jsonSchemaSubsetErrors(member, value, path, root));
  const alternatives = Array.isArray(contract.anyOf) ? contract.anyOf : undefined;
  if (alternatives && !alternatives.some((alternative) => jsonSchemaSubsetErrors(alternative, value, path, root).length === 0)) errors.push(`${path} must match one anyOf branch`);
  if (Array.isArray(contract.oneOf) && contract.oneOf.filter((alternative) => jsonSchemaSubsetErrors(alternative, value, path, root).length === 0).length !== 1) errors.push(`${path} must match exactly one oneOf branch`);
  if (!jsonSchemaSubsetTypeMatches(contract.type, value)) {
    errors.push(`${path} must be ${String(contract.type)}`);
    return errors;
  }
  if (typeof value === "string") {
    if (typeof contract.minLength === "number" && value.length < contract.minLength) errors.push(`${path} must contain at least ${contract.minLength} character(s)`);
    if (typeof contract.pattern === "string" && !new RegExp(contract.pattern, "u").test(value)) errors.push(`${path} must match ${contract.pattern}`);
  }
  if (typeof value === "number" && typeof contract.minimum === "number" && value < contract.minimum) errors.push(`${path} must be at least ${contract.minimum}`);
  if (typeof value === "number" && typeof contract.maximum === "number" && value > contract.maximum) errors.push(`${path} must be at most ${contract.maximum}`);
  if (Array.isArray(value)) {
    if (typeof contract.minItems === "number" && value.length < contract.minItems) errors.push(`${path} must contain at least ${contract.minItems} item(s)`);
    if (typeof contract.maxItems === "number" && value.length > contract.maxItems) errors.push(`${path} must contain at most ${contract.maxItems} item(s)`);
    if (contract.uniqueItems === true && new Set(value.map((item) => canonicalJson(item))).size !== value.length) errors.push(`${path} items must be unique`);
    if ("items" in contract) for (let index = 0; index < value.length; index++) errors.push(...jsonSchemaSubsetErrors(contract.items, value[index], `${path}/${index}`, root));
    if ("contains" in contract && !value.some((item, index) => jsonSchemaSubsetErrors(contract.contains, item, `${path}/${index}`, root).length === 0)) errors.push(`${path} must contain a matching item`);
  }
  const object = jsonSchemaSubsetObject(value);
  if (object) {
    const required = Array.isArray(contract.required) ? contract.required.filter((key): key is string => typeof key === "string") : [];
    for (const key of required) if (!(key in object)) errors.push(`${path}/${key} is required`);
    const properties = jsonSchemaSubsetObject(contract.properties) ?? {};
    const patterns = Object.entries(jsonSchemaSubsetObject(contract.patternProperties) ?? {}).map(([pattern, childSchema]) => ({ pattern: new RegExp(pattern, "u"), childSchema }));
    for (const [key, childSchema] of Object.entries(properties)) if (key in object) errors.push(...jsonSchemaSubsetErrors(childSchema, object[key], `${path}/${key}`, root));
    for (const key of Object.keys(object)) {
      if ("propertyNames" in contract) errors.push(...jsonSchemaSubsetErrors(contract.propertyNames, key, `${path}/${key} (name)`, root));
      const matched = patterns.filter(({ pattern }) => pattern.test(key));
      for (const { childSchema } of matched) errors.push(...jsonSchemaSubsetErrors(childSchema, object[key], `${path}/${key}`, root));
      if (key in properties || matched.length > 0) continue;
      if (contract.additionalProperties === false) errors.push(`${path}/${key} is not allowed`);
      else if (jsonSchemaSubsetObject(contract.additionalProperties)) errors.push(...jsonSchemaSubsetErrors(contract.additionalProperties, object[key], `${path}/${key}`, root));
    }
  }
  return errors;
}


/** 🧭️ Validates the repository-owned Draft-07 subset without an external runtime dependency; `root` resolves document-local `$ref`s. */
export function validateJsonSchemaSubset(schema: unknown, value: unknown, root: unknown = schema): string[] {
  return jsonSchemaSubsetErrors(schema, value, "", root);
}
