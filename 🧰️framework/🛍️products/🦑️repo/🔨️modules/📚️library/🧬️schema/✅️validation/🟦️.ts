import { canonicalJson } from "../../🧹️normalization/🟦️.ts";

//#region 🔣️JsonSchemaSubset
export function jsonSchemaSubsetObject(value: unknown): Record<string, unknown> | undefined {
  return typeof value === "object" && value !== null && !Array.isArray(value) ? value as Record<string, unknown> : undefined;
}


export function jsonSchemaSubsetValueEquals(left: unknown, right: unknown): boolean {
  return canonicalJson(left) === canonicalJson(right);
}


export function jsonSchemaSubsetTypeMatches(type: unknown, value: unknown): boolean {
  if (type === "object") return jsonSchemaSubsetObject(value) !== undefined;
  if (type === "array") return Array.isArray(value);
  if (type === "string") return typeof value === "string";
  if (type === "integer") return typeof value === "number" && Number.isInteger(value);
  if (type === "number") return typeof value === "number" && Number.isFinite(value);
  if (type === "boolean") return typeof value === "boolean";
  if (type === "null") return value === null;
  return true;
}


export function jsonSchemaSubsetErrors(schema: unknown, value: unknown, path: string): string[] {
  const contract = jsonSchemaSubsetObject(schema);
  if (!contract) return [`${path} schema must be an object`];
  const errors: string[] = [];
  if ("const" in contract && !jsonSchemaSubsetValueEquals(value, contract.const)) errors.push(`${path} must equal its const`);
  const enumValues = Array.isArray(contract.enum) ? contract.enum : undefined;
  if (enumValues && !enumValues.some((candidate) => jsonSchemaSubsetValueEquals(value, candidate))) errors.push(`${path} must be an allowed enum value`);
  const alternatives = Array.isArray(contract.anyOf) ? contract.anyOf : undefined;
  if (alternatives && !alternatives.some((alternative) => jsonSchemaSubsetErrors(alternative, value, path).length === 0)) errors.push(`${path} must match one anyOf branch`);
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
    if ("items" in contract) for (let index = 0; index < value.length; index++) errors.push(...jsonSchemaSubsetErrors(contract.items, value[index], `${path}/${index}`));
    if ("contains" in contract && !value.some((item, index) => jsonSchemaSubsetErrors(contract.contains, item, `${path}/${index}`).length === 0)) errors.push(`${path} must contain a matching item`);
  }
  const object = jsonSchemaSubsetObject(value);
  if (object) {
    const required = Array.isArray(contract.required) ? contract.required.filter((key): key is string => typeof key === "string") : [];
    for (const key of required) if (!(key in object)) errors.push(`${path}/${key} is required`);
    const properties = jsonSchemaSubsetObject(contract.properties) ?? {};
    for (const [key, childSchema] of Object.entries(properties)) if (key in object) errors.push(...jsonSchemaSubsetErrors(childSchema, object[key], `${path}/${key}`));
    if (contract.additionalProperties === false) for (const key of Object.keys(object)) if (!(key in properties)) errors.push(`${path}/${key} is not allowed`);
  }
  return errors;
}


/** 🧭️ Validates the repository-owned Draft-07 subset without an external runtime dependency. */
export function validateJsonSchemaSubset(schema: unknown, value: unknown): string[] {
  return jsonSchemaSubsetErrors(schema, value, "");
}
