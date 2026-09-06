/** 🔣️ `s.stdio.json@rfc8259` → remodeling snapshot — a spec-driven RFC 8259 reader.
 *
 *  Validation is total: an unknown key, a missing non-`#[serde(default)]` key, a wrong JSON type,
 *  an unknown enum lexeme or a mis-sized fixed array is a hard error naming the offending path.
 *  Nothing is coerced silently, so a fixture that drifts from the schema fails loudly here rather
 *  than surviving into an apply. */

import { REMODELING_SNAPSHOT_SPEC, camelOf, type RecordSpec, type RemodelingSnapshot, type ValueSpec } from "../../../../../../../🧬️schema/📸️snapshot/🟦️.ts";

/** 🚫 A decode refusal carrying the JSON pointer of the value that caused it. */
export class RemodelingDecodeError extends Error {
  constructor(
    readonly path: string,
    message: string,
  ) {
    super(`${path || "$"}: ${message}`);
    this.name = "RemodelingDecodeError";
  }
}

const fail = (path: string, message: string): never => {
  throw new RemodelingDecodeError(path, message);
};

const isPlainObject = (value: unknown): value is Record<string, unknown> => typeof value === "object" && value !== null && !Array.isArray(value);

const finiteNumber = (value: unknown, path: string): number => (typeof value === "number" && Number.isFinite(value) ? value : (fail(path, `expected a finite number, got ${JSON.stringify(value)}`) as never));

const wholeNumber = (value: unknown, path: string, signed: boolean): number => {
  const number = finiteNumber(value, path);
  if (!Number.isInteger(number)) fail(path, `expected an integer, got ${number}`);
  if (!signed && number < 0) fail(path, `expected an unsigned integer, got ${number}`);
  return number;
};

/** 🧩️ Decodes one value against its spec. */
export function decodeValue(value: unknown, spec: ValueSpec, path: string): unknown {
  switch (spec.k) {
    case "text":
      return typeof value === "string" ? value : fail(path, `expected a string, got ${JSON.stringify(value)}`);
    case "bool":
      return typeof value === "boolean" ? value : fail(path, `expected a boolean, got ${JSON.stringify(value)}`);
    case "uint":
      return wholeNumber(value, path, false);
    case "int":
      return wholeNumber(value, path, true);
    case "f64":
    case "f32":
      return finiteNumber(value, path);
    case "enum":
      return typeof value === "string" && spec.of.includes(value) ? value : fail(path, `expected one of ${spec.of.join(" | ")}, got ${JSON.stringify(value)}`);
    case "tuple": {
      if (!Array.isArray(value)) fail(path, `expected an array of ${spec.len} numbers, got ${JSON.stringify(value)}`);
      const items = value as unknown[];
      if (items.length !== spec.len) fail(path, `expected exactly ${spec.len} numbers, got ${items.length}`);
      return items.map((item, index) => finiteNumber(item, `${path}[${index}]`));
    }
    case "list":
      if (!Array.isArray(value)) fail(path, `expected an array, got ${JSON.stringify(value)}`);
      return (value as unknown[]).map((item, index) => decodeValue(item, spec.of, `${path}[${index}]`));
    case "map": {
      if (!isPlainObject(value)) fail(path, `expected an object, got ${JSON.stringify(value)}`);
      const out: Record<string, unknown> = {};
      for (const [key, entry] of Object.entries(value as Record<string, unknown>)) out[key] = decodeValue(entry, spec.of, `${path}.${key}`);
      return out;
    }
    case "rec":
      return decodeRecord(value, spec.of(), path);
    case "opt":
      return value === null || value === undefined ? null : decodeValue(value, spec.of, path);
  }
}

/** 🧱 Decodes one record, rejecting unknown and unfilled keys. */
export function decodeRecord(value: unknown, spec: RecordSpec, path: string): Record<string, unknown> {
  if (!isPlainObject(value)) fail(path, `expected a ${spec.title} object, got ${JSON.stringify(value)}`);
  const source = value as Record<string, unknown>;
  const known = new Set(spec.fields.map((field) => camelOf(field.name)));
  for (const key of Object.keys(source)) if (!known.has(key)) fail(`${path}.${key}`, `unknown key for ${spec.title} (known: ${[...known].join(", ")})`);
  const out: Record<string, unknown> = {};
  for (const field of spec.fields) {
    const key = camelOf(field.name);
    if (!(key in source)) {
      if (!spec.serdeDefault && !field.jsonOptional) fail(`${path}.${key}`, `missing required key for ${spec.title}`);
      out[key] = field.dflt();
      continue;
    }
    out[key] = decodeValue(source[key], field.spec, `${path}.${key}`);
  }
  return out;
}

/** 📸️ Decodes a parsed RFC 8259 document into a validated `RemodelingSnapshot`. */
export const decodeRemodelingSnapshot = (json: unknown): RemodelingSnapshot => decodeRecord(json, REMODELING_SNAPSHOT_SPEC, "") as unknown as RemodelingSnapshot;

/** 📄️ Decodes RFC 8259 text into a validated `RemodelingSnapshot`. */
export function remodelingSnapshotFromJsonText(text: string): RemodelingSnapshot {
  let parsed: unknown;
  try {
    parsed = JSON.parse(text);
  } catch (error) {
    throw new RemodelingDecodeError("", `not RFC 8259 text: ${(error as Error).message}`);
  }
  return decodeRemodelingSnapshot(parsed);
}
