/** 🧾️ Admits a record containing only declared enumerable data fields. */
export function parseSchemaRecord(value: unknown, keys: readonly string[], at = "$"): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: object required`);
  const prototype = Object.getPrototypeOf(value);
  if (prototype !== Object.prototype && prototype !== null) throw new Error(`${at}: plain record required`);
  const fields = Object.getOwnPropertyDescriptors(value);
  for (const key of Reflect.ownKeys(fields)) {
    if (typeof key !== "string" || !keys.includes(key)) throw new Error(`${at}: unknown field`);
    const field = fields[key]!;
    if (!field.enumerable || !("value" in field)) throw new Error(`${at}.${key}: enumerable data field required`);
  }
  return value as Record<string, unknown>;
}
