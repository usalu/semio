/** 🔡️ Text and dictionary helpers of the parser projection: Python-compatible JSON text (separators and escapes), schema-node truthiness and code-point ordering, so the projected sources stay byte-stable across implementations.
 * @see https://docs.python.org/3/library/json.html#json.dumps
 */

//#region 🔖️Dictionaries
/** 🧱️ A JSON object read from a schema document. */
export type Dict = { readonly [key: string]: unknown };

/** 🧱️ True for a JSON object (never an array, never `null`). */
export const isDict = (value: unknown): value is Dict => typeof value === "object" && value !== null && !Array.isArray(value);

/** 🧱️ The object itself, or an empty object for any other value. */
export const asDict = (value: unknown): Dict => (isDict(value) ? value : {});

/** 🧱️ True when the object states `key` itself. */
export const has = (node: Dict, key: string): boolean => Object.hasOwn(node, key);

/** 🧱️ Truthiness of a schema value: absent, `null`, `false`, `0`, `""`, `[]` and `{}` are empty. */
export function truthy(value: unknown): boolean {
  if (value === undefined || value === null || value === false || value === 0 || value === "") return false;
  if (Array.isArray(value)) return value.length > 0;
  if (isDict(value)) return Object.keys(value).length > 0;
  return true;
}

/** 🧱️ True when a key would be reordered by an object (an array index), so insertion order could not be trusted. */
const isIndexKey = (key: string): boolean => /^(?:0|[1-9][0-9]*)$/u.test(key) && Number(key) < 2 ** 32 - 1;

/** 🧱️ The entries of an object in document order, or `null` when an index-like key makes that order unrecoverable. */
export function orderedEntries(node: Dict): [string, unknown][] | null {
  const keys = Object.keys(node);
  return keys.some(isIndexKey) ? null : keys.map((key) => [key, node[key]]);
}
//#endregion 🔖️Dictionaries

//#region 🔖️Names
/** 🏷️ An export name: starts with an upper-case letter. */
export const EXPORT_NAME = /^[A-Z][A-Za-z0-9]*\n?$/u;

/** 🏷️ A GraphQL or TypeScript identifier. */
export const IDENTIFIER_NAME = /^[A-Za-z_][A-Za-z0-9_]*\n?$/u;

/** 🏷️ A bare PascalCase type name. */
export const BARE_TYPE_NAME = /^[A-Z][A-Za-z0-9]*$/u;

/** 🏷️ Escapes a literal for use inside a regular expression. */
export const escapeRegExp = (text: string): string => text.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");

/** 🏷️ `CsvRecordAdded` to `csv_record_added`. */
export const snake = (name: string): string => name.replace(/(?<!^)(?=[A-Z])/gu, "_").toLowerCase();

/** 🏷️ The key as it appears in an object literal: bare when it is an identifier, quoted otherwise. */
export const keyText = (field: string): string => (IDENTIFIER_NAME.test(field) ? field : jsonString(field, false));

/** 🔀️ Orders strings by code point, which is the order of their UTF-8 bytes. */
export const codePointOrder = (left: string, right: string): number => Buffer.compare(Buffer.from(left), Buffer.from(right));
//#endregion 🔖️Names

//#region 🔖️JsonText
/** 📝️ A JSON string literal; with `ensureAscii` every non-ASCII code unit becomes a lower-case `\uXXXX` escape. */
export function jsonString(value: string, ensureAscii: boolean): string {
  const text = JSON.stringify(value);
  return ensureAscii ? text.replace(/[^\x20-\x7e]/gu, (unit) => `\\u${unit.charCodeAt(0).toString(16).padStart(4, "0")}`) : text;
}

/** 📝️ A JSON value in Python's `json.dumps` layout (`", "` and `": "` separators). */
export function jsonValue(value: unknown, ensureAscii: boolean): string {
  if (typeof value === "string") return jsonString(value, ensureAscii);
  if (typeof value === "bigint") return value.toString();
  if (Array.isArray(value)) return `[${value.map((item) => jsonValue(item, ensureAscii)).join(", ")}]`;
  if (isDict(value)) return `{${Object.entries(value).map(([key, item]) => `${jsonString(key, ensureAscii)}: ${jsonValue(item, ensureAscii)}`).join(", ")}}`;
  return JSON.stringify(value) ?? "null";
}
//#endregion 🔖️JsonText
