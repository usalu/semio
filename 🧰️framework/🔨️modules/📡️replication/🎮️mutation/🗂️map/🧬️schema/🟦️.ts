/** 🗂️ Framework-owned map changes preserve value presence and sequential preconditions. */
import { parseDslValue, type DslValue } from "../../../../🌱️value/🧬️schema/🟦️.ts";

export type MapPresence = "any" | "present" | "absent" | "never";
export type MapEntryOperation<V> = { kind: "set"; value: V } | { kind: "remove" } | { kind: "reject" };
export interface MapEntryDelta<V> { key: string; precondition: MapPresence; operation: MapEntryOperation<V>; }
export interface MapDelta<V> { entries: MapEntryDelta<V>[]; }

/** 🚫️ Stable shared rejection of a base-incompatible map change. */
export class MapDeltaApplyError extends Error {
  constructor(readonly code: string, readonly key: string) { super(`${code}: ${key}`); }
}

function accepts(requirement: MapPresence, present: boolean): boolean {
  return requirement === "any" || requirement === (present ? "present" : "absent");
}

function compareKeys(left: string, right: string): number {
  for (let a = 0, b = 0; a < left.length && b < right.length;) {
    const x = left.codePointAt(a)!, y = right.codePointAt(b)!;
    if (x !== y) return x - y;
    a += x > 0xffff ? 2 : 1;
    b += y > 0xffff ? 2 : 1;
  }
  return left.length - right.length;
}

function exact(value: DslValue, fields: readonly string[]): Record<string, DslValue> {
  if (value === null || typeof value !== "object" || Array.isArray(value) || Object.keys(value).length !== fields.length || fields.some((key) => !Object.hasOwn(value, key))) throw new Error("map delta record has incorrect fields");
  return value;
}

/** 🪪️ Validates one shared wire delta and delegates each set payload to its owning schema. */
export function parseMapDelta<V = DslValue>(value: unknown, parseValue: (value: unknown) => V = parseDslValue as (value: unknown) => V): MapDelta<V> {
  const row = exact(parseDslValue(value), ["entries"]);
  if (!Array.isArray(row.entries)) throw new Error("map delta entries must be a list");
  const seen = new Set<string>();
  const entries = row.entries.map((value): MapEntryDelta<V> => {
    const entry = exact(value, ["key", "precondition", "operation"]);
    if (typeof entry.key !== "string" || seen.has(entry.key)) throw new Error("map delta keys must be unique strings");
    seen.add(entry.key);
    const precondition = entry.precondition;
    if (precondition !== "any" && precondition !== "present" && precondition !== "absent" && precondition !== "never") throw new Error("unknown map presence requirement");
    const raw = entry.operation!;
    if (raw === null || typeof raw !== "object" || Array.isArray(raw)) throw new Error("map operation must be a record");
    const operation = exact(raw, raw.kind === "set" ? ["kind", "value"] : ["kind"]);
    const kind = operation.kind;
    if (kind !== "set" && kind !== "remove" && kind !== "reject") throw new Error("unknown map operation");
    if ((precondition === "never") !== (kind === "reject")) throw new Error("unsatisfiable map changes must be explicit rejections");
    return { key: entry.key, precondition, operation: kind === "set" ? { kind, value: parseValue(operation.value) } : { kind } };
  });
  return { entries: entries.sort((a, b) => compareKeys(a.key, b.key)) };
}

/** 🧮️ Checks every original-base precondition before applying the first map change. */
export function applyMapDelta<V>(delta: MapDelta<V>, base: Readonly<Record<string, V>>): Record<string, V> {
  for (const entry of delta.entries) {
    if (entry.operation.kind === "reject" || !accepts(entry.precondition, Object.hasOwn(base, entry.key))) {
      throw new MapDeltaApplyError(entry.precondition === "never" || entry.operation.kind === "reject" ? "mutation.apply.unsatisfiable-precondition" : entry.precondition === "present" ? "mutation.apply.missing-target" : "mutation.apply.target-precondition", entry.key);
    }
  }
  const result = { ...base };
  for (const { key, operation } of delta.entries) {
    if (operation.kind === "set") Object.defineProperty(result, key, { value: operation.value, writable: true, enumerable: true, configurable: true });
    else delete result[key];
  }
  return result;
}

/** 🪢️ Composes partial entry transformations while retaining one entry per distinct key. */
export function composeMapDelta<V>(first: MapDelta<V>, second: MapDelta<V>): MapDelta<V> {
  const entries = new Map(first.entries.map((entry) => [entry.key, entry]));
  for (const next of second.entries) {
    const previous = entries.get(next.key);
    if (!previous) { entries.set(next.key, next); continue; }
    const compatible = previous.precondition !== "never" && previous.operation.kind !== "reject" && accepts(next.precondition, previous.operation.kind === "set");
    entries.set(next.key, compatible ? { ...next, precondition: previous.precondition } : { key: next.key, precondition: "never", operation: { kind: "reject" } });
  }
  return { entries: [...entries.values()].sort((a, b) => compareKeys(a.key, b.key)) };
}
