/** 🛂️ The JSON Schema subset validator behind every typed PDF snapshot parser, with its registry of schema documents. */

import schema from "../🔣️.json" with { type: "json" };
import { parseBinary64 } from "@semio-tech/framework/value/ieee754";

export type Schema = Record<string, unknown>;

export class SchemaRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

export const documents = new Map<string, Schema>();

export const registerSchemaDocument = (schema: Schema): void => {
  documents.set(String(schema["$id"]), schema);
};

export const resolveRef = (ref: string, own: Schema): { node: Schema; document: Schema } => {
  const [documentId, pointer] = ref.split("#");
  const document = documentId === "" ? own : documents.get(documentId);
  if (!document) throw new SchemaRefusal("$ref", `unknown schema document ${documentId}`);
  let node: unknown = document;
  for (const step of (pointer ?? "").split("/").filter((s) => s.length > 0)) node = (node as Record<string, unknown>)[step];
  if (!node) throw new SchemaRefusal("$ref", `unresolved pointer ${ref}`);
  return { node: node as Schema, document };
};

export const matches = (schema: Schema, value: unknown, own: Schema, at: string, errors: string[]): boolean => {
  if (schema["x-semio-primitive"] === "binary64" || schema["$ref"] === "https://json.schemas.assets.semio-tech.com/framework/value/schema.json#/$defs/Binary64Transport" || schema["$ref"] === "https://json.schemas.assets.semio-tech.com/framework/value/schema.json#/$defs/Binary64") { try { parseBinary64(value); return true; } catch { return (errors.push(`${at}: expected owned binary64`), false); } }
  if (schema["x-semio-primitive"] === "i64" || schema["x-semio-primitive"] === "u64") return typeof value === "bigint" && value >= (schema["x-semio-primitive"] === "i64" ? -9223372036854775808n : 0n) && value <= (schema["x-semio-primitive"] === "i64" ? 9223372036854775807n : 18446744073709551615n) || (errors.push(`${at}: expected owned integer word`), false);
  if (typeof schema["$ref"] === "string") { const resolved = resolveRef(schema["$ref"] as string, own); return matches(resolved.node, value, resolved.document, at, errors); }
  if (schema["const"] !== undefined) return value === schema["const"] || (errors.push(`${at}: expected ${JSON.stringify(schema["const"])}`), false);
  if (Array.isArray(schema["enum"])) return (schema["enum"] as unknown[]).includes(value) || (errors.push(`${at}: not one of ${(schema["enum"] as unknown[]).join(", ")}`), false);
  if (Array.isArray(schema["anyOf"])) return (schema["anyOf"] as Schema[]).some((s) => matches(s, value, own, at, [])) || (errors.push(`${at}: matches no alternative`), false);
  if (Array.isArray(schema["oneOf"])) return (schema["oneOf"] as Schema[]).filter((s) => matches(s, value, own, at, [])).length === 1 || (errors.push(`${at}: matches no single alternative`), false);
  if (Array.isArray(schema["allOf"])) return (schema["allOf"] as Schema[]).every((s) => matches(s, value, own, at, errors));
  const types = Array.isArray(schema["type"]) ? (schema["type"] as string[]) : typeof schema["type"] === "string" ? [schema["type"] as string] : [];
  const kind = value === null ? "null" : Array.isArray(value) ? "array" : typeof value === "number" ? (Number.isInteger(value) ? "integer" : "number") : typeof value;
  if (types.length > 0 && !types.includes(kind) && !(kind === "integer" && types.includes("number"))) return (errors.push(`${at}: expected ${types.join("|")}, found ${kind}`), false);
  if (kind === "integer" || kind === "number") {
    if (typeof schema["minimum"] === "number" && (value as number) < (schema["minimum"] as number)) return (errors.push(`${at}: below ${schema["minimum"]}`), false);
    if (typeof schema["maximum"] === "number" && (value as number) > (schema["maximum"] as number)) return (errors.push(`${at}: above ${schema["maximum"]}`), false);
  }
  if (kind === "array") {
    const items = value as unknown[];
    if (typeof schema["minItems"] === "number" && items.length < (schema["minItems"] as number)) return (errors.push(`${at}: fewer than ${schema["minItems"]} items`), false);
    if (typeof schema["maxItems"] === "number" && items.length > (schema["maxItems"] as number)) return (errors.push(`${at}: more than ${schema["maxItems"]} items`), false);
    if (Array.isArray(schema["items"])) return items.every((item, index) => matches((schema["items"] as Schema[])[index] ?? {}, item, own, `${at}[${index}]`, errors));
    if (schema["items"]) return items.every((item, index) => matches(schema["items"] as Schema, item, own, `${at}[${index}]`, errors));
  }
  if (kind === "object") {
    const row = value as Record<string, unknown>;
    for (const key of (schema["required"] as string[] | undefined) ?? []) if (row[key] === undefined) return (errors.push(`${at}.${key}: missing`), false);
    const properties = (schema["properties"] as Record<string, Schema> | undefined) ?? {};
    if(schema["additionalProperties"]===false && Object.keys(row).some(key=>!(key in properties)))return (errors.push(`${at}: unexpected field`),false);
    for (const [key, sub] of Object.entries(properties)) if (row[key] !== undefined && !matches(sub, row[key], own, `${at}.${key}`, errors)) return false;
  }
  return true;
};

export const validateAgainst = <T,>(schema: Schema, pointer: string, value: unknown): T => {
  const node = pointer === "" ? schema : resolveRef(`#${pointer}`, schema).node;
  const errors: string[] = [];
  if (!matches(node, value, schema, "$", errors)) throw new SchemaRefusal("$", errors[0] ?? "invalid");
  return value as T;
};
