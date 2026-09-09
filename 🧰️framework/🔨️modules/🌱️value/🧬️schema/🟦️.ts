/** 🌱️ JSON projection of the framework-owned dynamic value tree. */
export type DslValue = null | boolean | number | string | DslValue[] | { [key: string]: DslValue };

/** 🌱️ Validates a finite JSON tree without using the native call stack for nested values. */
export function parseDslValue(value: unknown): DslValue {
  const active = new Set<object>();
  const stack: { value: unknown; close?: object }[] = [{ value }];
  while (stack.length) {
    const frame = stack.pop()!;
    if (frame.close) { active.delete(frame.close); continue; }
    const current = frame.value;
    if (current === null || typeof current === "string" || typeof current === "boolean") continue;
    if (typeof current === "number" && Number.isFinite(current)) continue;
    if (typeof current !== "object" || current === null || active.has(current)) throw new Error("dynamic value is not a finite JSON tree");
    const array = Array.isArray(current);
    if (!array && Object.getPrototypeOf(current) !== Object.prototype && Object.getPrototypeOf(current) !== null) throw new Error("dynamic object must contain JSON fields");
    const fields = Object.getOwnPropertyDescriptors(current), keys = Reflect.ownKeys(fields);
    if (array && keys.length !== fields.length!.value + 1) throw new Error("dynamic array must contain only contiguous JSON entries");
    active.add(current);
    stack.push({ value: null, close: current });
    for (const key of keys) {
      if (typeof key !== "string") throw new Error("dynamic object fields must have string names");
      if (array && key === "length") continue;
      const field = fields[key]!;
      if (!field.enumerable || !("value" in field)) throw new Error("dynamic object must contain enumerable data fields");
      if (array && (!/^(?:0|[1-9][0-9]*)$/.test(key) || Number(key) >= fields.length!.value)) throw new Error("dynamic array has a non-index field");
      stack.push({ value: field.value });
    }
  }
  return value as DslValue;
}
