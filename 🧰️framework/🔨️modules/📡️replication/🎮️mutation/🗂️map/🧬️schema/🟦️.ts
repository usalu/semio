/** 🗂️ Framework-owned keyed map changes preserve value presence and sequential preconditions. */
import { parseDslValue, type DslValue } from "@semio-tech/value-ts/schema";

export type MapPresence = "any" | "present" | "absent" | "never";
export type MapEntryOperation<V> = { kind: "set"; value: V } | { kind: "remove" } | { kind: "reject" };
export interface MapEntryDelta<V> { precondition: MapPresence; operation: MapEntryOperation<V>; }
export interface MapDelta<V> { entries: Record<string, MapEntryDelta<V>>; }

/** 🚫️ Stable shared rejection of a base-incompatible map change. */
export class MapDeltaApplyError extends Error {
  constructor(readonly code: string, readonly key: string) { super(`${code}: ${key}`); }
}
function accepts(requirement: MapPresence, present: boolean): boolean { return requirement === "any" || requirement === (present ? "present" : "absent"); }
function compareKeys(left: string, right: string): number {
  for (let a = 0, b = 0; a < left.length && b < right.length;) {
    const x = left.codePointAt(a)!, y = right.codePointAt(b)!;
    if (x !== y) return x - y;
    a += x > 0xffff ? 2 : 1;
    b += y > 0xffff ? 2 : 1;
  }
  return left.length - right.length;
}
function record(value: unknown): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("map delta value must be a record");
  return value as Record<string, unknown>;
}
function exact(value: unknown, fields: readonly string[]): Record<string, unknown> {
  const row = record(value);
  if (Object.keys(row).length !== fields.length || fields.some(key => !Object.hasOwn(row, key))) throw new Error("map delta record has incorrect fields");
  return row;
}

/** 🪪️ Validates shared keyed wire entries and delegates each set payload to its owning schema. */
export function parseMapDelta<V = DslValue>(value: unknown, parseValue: (value: unknown) => V = parseDslValue as (value: unknown) => V): MapDelta<V> {
  const row = exact(value, ["entries"]);
  const entries = Object.entries(record(row.entries)).sort(([a], [b]) => compareKeys(a, b)).map(([key, value]): [string, MapEntryDelta<V>] => {
    const entry = exact(value, ["precondition", "operation"]);
    const precondition = entry.precondition;
    if (precondition !== "any" && precondition !== "present" && precondition !== "absent" && precondition !== "never") throw new Error("unknown map presence requirement");
    const raw = record(entry.operation);
    const operation = exact(raw, raw.kind === "set" ? ["kind", "value"] : ["kind"]);
    const kind = operation.kind;
    if (kind !== "set" && kind !== "remove" && kind !== "reject") throw new Error("unknown map operation");
    if ((precondition === "never") !== (kind === "reject")) throw new Error("unsatisfiable map changes must be explicit rejections");
    return [key, { precondition, operation: kind === "set" ? { kind, value: parseValue(operation.value) } : { kind } }];
  });
  return { entries: Object.fromEntries(entries) };
}

/** 🧮️ Checks every original-base precondition before applying the first map change. */
export function applyMapDelta<V>(delta: MapDelta<V>, base: Readonly<Record<string, V>>): Record<string, V> {
  for (const [key, entry] of Object.entries(delta.entries)) {
    if (entry.operation.kind === "reject" || !accepts(entry.precondition, Object.hasOwn(base, key))) throw new MapDeltaApplyError(entry.precondition === "never" || entry.operation.kind === "reject" ? "mutation.apply.unsatisfiable-precondition" : entry.precondition === "present" ? "mutation.apply.missing-target" : "mutation.apply.target-precondition", key);
  }
  const result = { ...base };
  for (const [key, { operation }] of Object.entries(delta.entries)) {
    if (operation.kind === "set") Object.defineProperty(result, key, { value: operation.value, writable: true, enumerable: true, configurable: true });
    else delete result[key];
  }
  return result;
}

/** 🪢️ Composes partial entry transformations while retaining one entry per distinct key. */
export function composeMapDelta<V>(first: MapDelta<V>, second: MapDelta<V>): MapDelta<V> {
  const entries = new Map(Object.entries(first.entries));
  for (const [key, next] of Object.entries(second.entries)) {
    const previous = entries.get(key);
    if (!previous) { entries.set(key, next); continue; }
    const compatible = previous.precondition !== "never" && previous.operation.kind !== "reject" && accepts(next.precondition, previous.operation.kind === "set");
    entries.set(key, compatible ? { ...next, precondition: previous.precondition } : { precondition: "never", operation: { kind: "reject" } });
  }
  return { entries: Object.fromEntries([...entries].sort(([a], [b]) => compareKeys(a, b))) };
}

/** ↩️ Reverses only authored map keys while preserving the original refusal domain. */
export function inverseMapDelta<V>(delta: MapDelta<V>, base: Readonly<Record<string, V>>): MapDelta<V> {
  return { entries: Object.fromEntries(Object.entries(delta.entries).flatMap(([key, entry]): [string, MapEntryDelta<V>][] => {
    const present = Object.hasOwn(base, key);
    if (entry.operation.kind === "reject" || !accepts(entry.precondition, present)) return [[key, { precondition: "never", operation: { kind: "reject" } }]];
    if (present) return [[key, { precondition: "any", operation: { kind: "set", value: base[key] } }]];
    return entry.operation.kind === "set" ? [[key, { precondition: "present", operation: { kind: "remove" } }]] : [];
  })) };
}
