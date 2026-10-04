import { SnapshotEditError, snapshotEditSource, snapshotFromEditSource, type SnapshotEditEvent, type SnapshotValue } from "../🟦️.js";

/** 🩹️ One path-scoped snapshot edit — the TypeScript twin of the Rust `SnapshotPatch` (`../🦀️.rs` region `🩹️Patch`). */
export type SnapshotPatch =
  | { operation: "set"; path: string; value: SnapshotValue }
  | { operation: "insert"; path: string; value: SnapshotValue; index?: number }
  | { operation: "remove"; path: string }
  | { operation: "move"; from: string; path: string; index?: number }
  | { operation: "rename"; path: string; key: string }
  | { operation: "splice"; path: string; offset: number; remove: number; value: SnapshotValue; continued?: boolean };
export const SNAPSHOT_PATCH_MAX_BYTES = 1_048_576;
export const SNAPSHOT_PATCH_MAX_SEGMENTS = 128;
export const SNAPSHOT_PATCH_MAX_INVERSE_PARTS = 128;
const MAX_INDEX = 9_007_199_254_740_991;

type StepEdit = { operation: "set" | "insert"; value: SnapshotValue } | { operation: "insertAt"; value: SnapshotValue; index: number } | { operation: "remove" } | { operation: "splice"; offset: number; remove: number; value: SnapshotValue };
interface Step { path: string[]; edit: StepEdit }

const pointer = (path: string[]): string => path.map((segment) => "/" + segment.replaceAll("~", "~0").replaceAll("/", "~1")).join("");
const fail = (code: string, path: string[], message: string): never => { throw new SnapshotEditError(code, pointer(path), message); };
const dictionary = (value: SnapshotValue): value is Record<string, SnapshotValue> => value !== null && typeof value === "object" && !Array.isArray(value);
const encoder = new TextEncoder();
const byteLength = (text: string): number => encoder.encode(text).length;
/** 🧾️ The compact canonical JSON the Rust codec writes (`bigint` as its decimal digits). */
const canonical = (value: SnapshotValue): string => typeof value === "bigint" ? value.toString() : value === null || typeof value !== "object" ? JSON.stringify(value)
  : Array.isArray(value) ? `[${value.map(canonical).join(",")}]` : `{${Object.entries(value).map(([key, item]) => `${JSON.stringify(key)}:${canonical(item)}`).join(",")}}`;

function decodePointer(path: string): string[] {
  if (path === "") return [];
  if (!path.startsWith("/")) return fail("snapshot-edit.invalid-pointer", [], "an RFC 6901 pointer must start with '/'");
  return path.slice(1).split("/").map((segment) => {
    if (/~(?![01])/u.test(segment)) return fail("snapshot-edit.invalid-pointer", [], "invalid RFC 6901 escape");
    return segment.replaceAll("~1", "/").replaceAll("~0", "~");
  });
}

function index(segment: string, length: number, insert: boolean, path: string[]): number {
  if (insert && segment === "-") return length;
  const value = Number(segment);
  if (!/^(0|[1-9][0-9]*)$/u.test(segment) || !Number.isSafeInteger(value)) return fail("snapshot-edit.invalid-index", path, "invalid array index");
  if (insert ? value > length : value >= length) return fail("snapshot-edit.index-out-of-bounds", path, "array index out of bounds");
  return value;
}

function at(snapshot: SnapshotValue, path: string[]): SnapshotValue {
  let value = snapshot;
  for (const segment of path) {
    if (Array.isArray(value)) value = value[index(segment, value.length, false, path)];
    else if (dictionary(value) && Object.hasOwn(value, segment)) value = value[segment];
    else return fail("snapshot-edit.path-missing", path, "the addressed field does not exist");
  }
  return value;
}

function normalize(snapshot: SnapshotValue, path: string[]): string[] {
  const next = path.slice();
  if (next.at(-1) === "-") {
    const parent = at(snapshot, next.slice(0, -1));
    if (Array.isArray(parent)) next[next.length - 1] = String(parent.length);
  }
  return next;
}

function memberIndex(snapshot: SnapshotValue, path: string[]): number | undefined {
  if (path.length === 0) return undefined;
  const parent = at(snapshot, path.slice(0, -1));
  if (!dictionary(parent)) return undefined;
  const position = Object.keys(parent).indexOf(path.at(-1)!);
  return position < 0 ? fail("snapshot-edit.path-missing", path, "the addressed object key does not exist") : position;
}

function validatePatch(patch: SnapshotPatch, budget = SNAPSHOT_PATCH_MAX_BYTES): void {
  const pointers = patch.operation === "move" ? [patch.from, patch.path] : [patch.path];
  for (const path of pointers) if (decodePointer(path).length > SNAPSHOT_PATCH_MAX_SEGMENTS) throw new SnapshotEditError("snapshot-edit.patch-limit", path, "a snapshot patch pointer permits 128 segments");
  const positions = patch.operation === "splice" ? [patch.offset, patch.remove] : (patch.operation === "insert" || patch.operation === "move") && patch.index !== undefined ? [patch.index] : [];
  if (positions.some((position) => !Number.isSafeInteger(position) || position < 0 || position > MAX_INDEX)) throw new SnapshotEditError("snapshot-edit.invalid-index", patch.path, "a snapshot patch position exceeds the portable integer range");
  if ("value" in patch) snapshotEditSource(patch.value);
  if (byteLength(canonical(patch as unknown as SnapshotValue)) > Math.min(budget, SNAPSHOT_PATCH_MAX_BYTES)) throw new SnapshotEditError("snapshot-edit.patch-limit", "", "snapshot patch exceeds the native publication item limit");
}

const PATCH_MEMBERS: Readonly<Record<SnapshotPatch["operation"], { required: readonly string[]; optional: readonly string[] }>> = {
  set: { required: ["path", "value"], optional: [] },
  insert: { required: ["path", "value"], optional: ["index"] },
  remove: { required: ["path"], optional: [] },
  move: { required: ["from", "path"], optional: ["index"] },
  rename: { required: ["path", "key"], optional: [] },
  splice: { required: ["path", "offset", "remove", "value"], optional: ["continued"] },
};

/** 🧾️ Reads one wire `SnapshotPatch` exactly as the Rust `deny_unknown_fields` codec admits it — the closed member set of its
 * operation, string pointers and key, an optional non-negative safe-integer `index` — then the same pointer, index and size
 * bounds the operations enforce. */
export function parseSnapshotPatch(value: unknown): SnapshotPatch {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new SnapshotEditError("snapshot-edit.invalid-patch", "", "a snapshot patch is an object");
  const record = value as Record<string, unknown>;
  const members = PATCH_MEMBERS[record.operation as SnapshotPatch["operation"]];
  if (typeof record.operation !== "string" || members === undefined) throw new SnapshotEditError("snapshot-edit.invalid-patch", "", "unknown snapshot patch operation");
  for (const key of Object.keys(record)) if (key !== "operation" && !members.required.includes(key) && !members.optional.includes(key)) throw new SnapshotEditError("snapshot-edit.invalid-patch", "", `unknown snapshot patch member ${key}`);
  for (const key of members.required) if (!Object.hasOwn(record, key)) throw new SnapshotEditError("snapshot-edit.invalid-patch", "", `missing snapshot patch member ${key}`);
  for (const key of ["path", "from", "key"]) if (Object.hasOwn(record, key) && typeof record[key] !== "string") throw new SnapshotEditError("snapshot-edit.invalid-patch", "", `snapshot patch member ${key} is a string`);
  for (const key of ["index", "offset", "remove"]) if (Object.hasOwn(record, key) && (typeof record[key] !== "number" || !Number.isSafeInteger(record[key]) || (record[key] as number) < 0)) throw new SnapshotEditError("snapshot-edit.invalid-index", String(record.path ?? ""), `snapshot patch ${key} is a non-negative integer`);
  if (Object.hasOwn(record, "continued") && typeof record.continued !== "boolean") throw new SnapshotEditError("snapshot-edit.invalid-patch", "", "snapshot patch member continued is a boolean");
  if (record.operation === "splice" && (record.value === null || (typeof record.value !== "object" && typeof record.value !== "string"))) throw new SnapshotEditError("snapshot-edit.splice-mismatch", String(record.path), "a splice inserts array items, text or object members");
  const patch = record as unknown as SnapshotPatch;
  validatePatch(patch);
  return patch;
}

function steps(snapshot: SnapshotValue, patch: SnapshotPatch): Step[] {
  switch (patch.operation) {
    case "set": return [{ path: decodePointer(patch.path), edit: { operation: "set", value: patch.value } }];
    case "insert": return [{ path: decodePointer(patch.path), edit: patch.index === undefined ? { operation: "insert", value: patch.value } : { operation: "insertAt", value: patch.value, index: patch.index } }];
    case "remove": return [{ path: decodePointer(patch.path), edit: { operation: "remove" } }];
    case "move": {
      const from = decodePointer(patch.from), path = decodePointer(patch.path);
      if (from.length === 0 || path.length === 0 || (path.length > from.length && from.every((segment, i) => path[i] === segment))) return fail("snapshot-edit.invalid-move", path, "a move cannot address the root or a descendant of its source");
      if (pointer(from) === pointer(path)) return [];
      const value = at(snapshot, from);
      return [{ path: from, edit: { operation: "remove" } }, { path, edit: patch.index === undefined ? { operation: "insert", value } : { operation: "insertAt", value, index: patch.index } }];
    }
    case "rename": {
      const path = decodePointer(patch.path), parent = path.slice(0, -1);
      if (path.length === 0) return fail("snapshot-edit.root-operation", path, "cannot rename the document root");
      if (!dictionary(at(snapshot, parent))) return fail("snapshot-edit.not-object", path, "only an object key can be renamed");
      if (path.at(-1) === patch.key) return [];
      return [{ path, edit: { operation: "remove" } }, { path: [...parent, patch.key], edit: { operation: "insertAt", value: at(snapshot, path), index: memberIndex(snapshot, path)! } }];
    }
    case "splice": return [{ path: decodePointer(patch.path), edit: { operation: "splice", offset: patch.offset, remove: patch.remove, value: patch.value } }];
  }
}

function spliceRange(offset: number, remove: number, length: number, path: string[]): [number, number] {
  return offset + remove <= length ? [offset, offset + remove] : fail("snapshot-edit.index-out-of-bounds", path, "the splice range exceeds the container");
}

function textBytes(text: string, start: number, end: number, path: string[]): Uint8Array {
  const bytes = encoder.encode(text);
  const boundary = (at: number) => at === bytes.length || (bytes[at]! & 0xc0) !== 0x80;
  if (end > bytes.length || !boundary(start) || !boundary(end)) return fail("snapshot-edit.char-boundary", path, "a text splice must address UTF-8 character boundaries");
  return bytes;
}

const decoder = new TextDecoder("utf-8", { fatal: true });

/** ✂️ The units a splice of `remove` at `offset` takes out of `container`, as a value of its kind. */
function spliceRemoved(container: SnapshotValue, offset: number, remove: number, path: string[]): SnapshotValue {
  if (Array.isArray(container)) return container.slice(...spliceRange(offset, remove, container.length, path));
  if (typeof container === "string") {
    const [start, end] = spliceRange(offset, remove, byteLength(container), path);
    return decoder.decode(textBytes(container, start, end, path).slice(start, end));
  }
  if (dictionary(container)) return Object.fromEntries(Object.entries(container).slice(...spliceRange(offset, remove, Object.keys(container).length, path)));
  return fail("snapshot-edit.not-container", path, "a splice addresses an array, text or an object");
}

function spliceUnits(container: SnapshotValue, value: SnapshotValue, path: string[]): number {
  if (Array.isArray(container) && Array.isArray(value)) return value.length;
  if (typeof container === "string" && typeof value === "string") return byteLength(value);
  if (dictionary(container) && dictionary(value)) return Object.keys(value).length;
  return fail("snapshot-edit.splice-mismatch", path, "the splice value is not of the addressed container's kind");
}

function applySplice(container: SnapshotValue, offset: number, remove: number, value: SnapshotValue, path: string[]): SnapshotValue {
  spliceUnits(container, value, path);
  if (Array.isArray(container)) {
    const [start, end] = spliceRange(offset, remove, container.length, path);
    return [...container.slice(0, start), ...structuredClone(value as SnapshotValue[]), ...container.slice(end)];
  }
  if (typeof container === "string") {
    const [start, end] = spliceRange(offset, remove, byteLength(container), path);
    const bytes = textBytes(container, start, end, path);
    return decoder.decode(new Uint8Array([...bytes.slice(0, start), ...encoder.encode(value as string), ...bytes.slice(end)]));
  }
  const entries = Object.entries(container as Record<string, SnapshotValue>);
  const [start, end] = spliceRange(offset, remove, entries.length, path);
  const removed = new Set(entries.slice(start, end).map(([key]) => key));
  for (const key of Object.keys(value as object)) if (!removed.has(key) && entries.some(([existing]) => existing === key)) return fail("snapshot-edit.key-exists", [...path, key], "the object key already exists");
  return Object.fromEntries([...entries.slice(0, start), ...Object.entries(structuredClone(value as Record<string, SnapshotValue>)), ...entries.slice(end)]);
}

function applyStep(snapshot: SnapshotValue, step: Step): SnapshotValue {
  const path = step.edit.operation === "insert" ? normalize(snapshot, step.path) : step.path;
  if (step.edit.operation === "splice") {
    const { offset, remove, value } = step.edit;
    return applyStep(snapshot, { path, edit: { operation: "set", value: applySplice(at(snapshot, path), offset, remove, value, path) } });
  }
  const descend = (value: SnapshotValue, depth: number): SnapshotValue => {
    if (depth === path.length) {
      if (step.edit.operation !== "set") return fail("snapshot-edit.root-operation", path, "only replacement can address the document root");
      return structuredClone(step.edit.value);
    }
    const key = path[depth];
    const leaf = depth + 1 === path.length;
    if (Array.isArray(value)) {
      if (leaf && step.edit.operation === "insertAt") return fail("snapshot-edit.not-object", path, "positioned insertion requires an object parent");
      const position = index(key, value.length, leaf && step.edit.operation === "insert", path);
      const next = value.slice();
      if (!leaf) next[position] = descend(value[position], depth + 1);
      else if (step.edit.operation === "insert") next.splice(position, 0, structuredClone(step.edit.value));
      else if (step.edit.operation === "remove") next.splice(position, 1);
      else next[position] = structuredClone((step.edit as { value: SnapshotValue }).value);
      return next;
    }
    if (!dictionary(value)) return fail("snapshot-edit.not-container", path, "the addressed parent is a scalar");
    const exists = Object.hasOwn(value, key);
    const insert = step.edit.operation === "insert" || step.edit.operation === "insertAt";
    if ((!leaf || !insert) && !exists) return fail("snapshot-edit.path-missing", path, "the addressed field does not exist");
    if (leaf && insert && exists) return fail("snapshot-edit.key-exists", path, "the object key already exists");
    if (leaf && step.edit.operation === "insertAt") {
      const entries = Object.entries(value);
      if (step.edit.index > entries.length) return fail("snapshot-edit.index-out-of-bounds", path, "object insertion index is out of range");
      entries.splice(step.edit.index, 0, [key, structuredClone(step.edit.value)]);
      return Object.fromEntries(entries);
    }
    const next = { ...value };
    if (leaf && step.edit.operation === "remove") delete next[key];
    else Object.defineProperty(next, key, { value: leaf ? structuredClone((step.edit as { value: SnapshotValue }).value) : descend(value[key], depth + 1), enumerable: true, writable: true, configurable: true });
    return next;
  };
  return descend(snapshot, 0);
}

/** 🧭️ Translates one user edit into ONE canonical path operation without cloning unrelated payloads. */
export function prepareSnapshotPatch(snapshot: SnapshotValue, event: SnapshotEditEvent): SnapshotPatch {
  const canonical = (path: string): string => pointer(decodePointer(path));
  let patch: SnapshotPatch;
  switch (event.operation) {
    case "setValue": patch = { operation: "set", path: canonical(event.path), value: event.value }; break;
    case "insertValue": patch = { operation: "insert", path: pointer(normalize(snapshot, decodePointer(event.path))), value: event.value }; break;
    case "removeValue": patch = { operation: "remove", path: canonical(event.path) }; break;
    case "replaceSource": patch = { operation: "set", path: "", value: snapshotFromEditSource(event.source) }; break;
    case "moveValue": {
      const from = decodePointer(event.from), path = decodePointer(event.path);
      const descendant = path.length > from.length && from.every((segment, i) => path[i] === segment);
      const intermediate = from.length > 0 && pointer(from) !== pointer(path) && !descendant ? applyStep(snapshot, { path: from, edit: { operation: "remove" } }) : snapshot;
      patch = { operation: "move", from: pointer(from), path: pointer(normalize(intermediate, path)) };
      break;
    }
    case "renameKey": patch = { operation: "rename", path: canonical(event.path), key: event.key }; break;
  }
  validatePatch(patch);
  steps(snapshot, patch);
  return patch;
}

/** 🛡️ Applies one path operation while preserving every untouched subtree; a refusal leaves the original untouched. */
export function applySnapshotPatch(snapshot: SnapshotValue, patch: SnapshotPatch): SnapshotValue {
  validatePatch(patch);
  return steps(snapshot, patch).reduce(applyStep, snapshot);
}

/** ↩️ Captures the exact inverse from the current publication base as ONE operation (refused beyond the patch budget). */
export function inverseSnapshotPatch(snapshot: SnapshotValue, patch: SnapshotPatch): SnapshotPatch {
  validatePatch(patch);
  const inverse = exactInverse(snapshot, patch);
  validatePatch(inverse);
  return inverse;
}

/** 🧩️ The exact inverse in parts of at most `budget` canonical JSON bytes — the TypeScript twin of the Rust
 * `inverse_snapshot_patches_within` (same shells, same greedy packing, same parts). */
export function inverseSnapshotPatches(snapshot: SnapshotValue, patch: SnapshotPatch, budget = SNAPSHOT_PATCH_MAX_BYTES): SnapshotPatch[] {
  validatePatch(patch);
  const inverse = exactInverse(snapshot, patch);
  try {
    validatePatch(inverse, budget);
    return [inverse];
  } catch {
    const planner = new InversePlanner(Math.min(budget, SNAPSHOT_PATCH_MAX_BYTES));
    planner.split(snapshot, patch, inverse);
    const parts = planner.parts.map((part, position) => {
      if (part.operation !== "splice" || position < planner.parts.length - 1) return part;
      const last = { ...part };
      delete last.continued;
      return last;
    });
    for (const part of parts) validatePatch(part, planner.budget);
    return parts;
  }
}

function exactInverse(snapshot: SnapshotValue, patch: SnapshotPatch): SnapshotPatch {
  let inverse: SnapshotPatch;
  switch (patch.operation) {
    case "set": inverse = { operation: "set", path: patch.path, value: at(snapshot, decodePointer(patch.path)) }; break;
    case "insert": inverse = { operation: "remove", path: pointer(normalize(snapshot, decodePointer(patch.path))) }; break;
    case "remove": {
      const path = decodePointer(patch.path);
      const position = memberIndex(snapshot, path);
      inverse = { operation: "insert", path: patch.path, value: at(snapshot, path), ...(position === undefined ? {} : { index: position }) };
      break;
    }
    case "move": {
      const from = decodePointer(patch.from), path = decodePointer(patch.path);
      if (pointer(from) === pointer(path)) { inverse = patch; break; }
      const intermediate = applyStep(snapshot, { path: from, edit: { operation: "remove" } });
      const position = memberIndex(snapshot, from);
      inverse = { operation: "move", from: pointer(normalize(intermediate, path)), path: patch.from, ...(position === undefined ? {} : { index: position }) };
      break;
    }
    case "rename": {
      const path = decodePointer(patch.path);
      inverse = path.length === 0 || path.at(-1) === patch.key ? patch : { operation: "rename", path: pointer([...path.slice(0, -1), patch.key]), key: path.at(-1)! };
      break;
    }
    case "splice": {
      const path = decodePointer(patch.path);
      const container = at(snapshot, path);
      inverse = { operation: "splice", path: patch.path, offset: patch.offset, remove: spliceUnits(container, patch.value, path), value: spliceRemoved(container, patch.offset, patch.remove, path) };
      break;
    }
  }
  return inverse;
}

const textLength = (text: string): number => byteLength(JSON.stringify(text));
const jsonLength = (value: SnapshotValue): number => byteLength(canonical(value));
const minimalLength = (value: SnapshotValue): number => typeof value === "string" || Array.isArray(value) ? 2
  : dictionary(value) ? 2 + Object.entries(value).reduce((sum, [key, member]) => sum + textLength(key) + 1 + minimalLength(member), 0) + Math.max(0, Object.keys(value).length - 1) : jsonLength(value);

type Shell = { kind: "whole" } | { kind: "emptied" } | { kind: "object"; members: Shell[]; placed: number };
const WHOLE: Shell = { kind: "whole" };
const EMPTIED: Shell = { kind: "emptied" };

/** 🐚️ The stand-in of `value` within `room` bytes — the twin of the Rust `shell`. */
function shell(value: SnapshotValue, room: number): [SnapshotValue, Shell] {
  if (jsonLength(value) <= room) return [value, WHOLE];
  if (typeof value === "string") return ["", EMPTIED];
  if (Array.isArray(value)) return [[], EMPTIED];
  if (!dictionary(value)) return [value, WHOLE];
  const entries = Object.entries(value);
  const entry = (position: number, key: string): number => textLength(key) + 1 + (position > 0 ? 1 : 0);
  let rest = entries.reduce((sum, [key, member], position) => sum + entry(position, key) + minimalLength(member), 0);
  let used = 2;
  const placed: [string, SnapshotValue][] = [];
  const members: Shell[] = [];
  for (const [position, [key, member]] of entries.entries()) {
    const minimal = minimalLength(member);
    rest -= entry(position, key) + minimal;
    const available = room - (used + entry(position, key) + rest);
    if (available < minimal) break;
    const [standIn, memberShell] = shell(member, available);
    used += entry(position, key) + jsonLength(standIn);
    placed.push([key, standIn]);
    members.push(memberShell);
  }
  return [Object.fromEntries(placed), { kind: "object", members, placed: placed.length }];
}

type Units = { kind: "items"; items: SnapshotValue[] } | { kind: "text"; text: string } | { kind: "members"; members: [string, SnapshotValue][] };
const unitsOf = (value: SnapshotValue): Units | undefined => Array.isArray(value) ? { kind: "items", items: value } : typeof value === "string" ? { kind: "text", text: value } : dictionary(value) ? { kind: "members", members: Object.entries(value) } : undefined;
const unitsLength = (units: Units): number => units.kind === "items" ? units.items.length : units.kind === "text" ? byteLength(units.text) : units.members.length;

class InversePlanner {
  readonly parts: SnapshotPatch[] = [];
  constructor(readonly budget: number) {}

  private limit(): never { throw new SnapshotEditError("snapshot-edit.inverse-limit", "", "the exact inverse exceeds its bounded number of parts"); }

  private push(part: SnapshotPatch): void {
    if (this.parts.length >= SNAPSHOT_PATCH_MAX_INVERSE_PARTS) this.limit();
    this.parts.push(part);
  }

  private room(path: string, offset: number, remove: number): number {
    const room = this.budget - (byteLength(canonical({ operation: "splice", path, offset, remove, value: null, continued: true })) - 4);
    return room < 0 ? this.limit() : room;
  }

  split(snapshot: SnapshotValue, patch: SnapshotPatch, inverse: SnapshotPatch): void {
    switch (inverse.operation) {
      case "set": {
        const segments = decodePointer(inverse.path);
        if (segments.length === 0) return patch.operation === "set" ? this.root(patch.value, inverse.value) : this.limit();
        const parent = segments.slice(0, -1);
        const position = dictionary(at(snapshot, parent)) ? memberIndex(snapshot, segments)! : index(segments.at(-1)!, Number.MAX_SAFE_INTEGER, false, segments);
        return this.place(snapshot, parent, segments.at(-1)!, position, 1, inverse.value);
      }
      case "insert": {
        const segments = decodePointer(inverse.path);
        if (segments.length === 0) return this.limit();
        const position = inverse.index ?? index(segments.at(-1)!, Number.MAX_SAFE_INTEGER, false, segments);
        return this.place(snapshot, segments.slice(0, -1), segments.at(-1)!, position, 0, inverse.value);
      }
      case "splice": return this.append(decodePointer(inverse.path), inverse.offset, inverse.remove, unitsOf(inverse.value) ?? this.limit());
      default: throw new SnapshotEditError("snapshot-edit.patch-limit", "", "a pointer-only inverse exceeds the patch budget");
    }
  }

  private place(snapshot: SnapshotValue, parent: string[], key: string, position: number, remove: number, value: SnapshotValue): void {
    const object = dictionary(at(snapshot, parent));
    const room = this.room(pointer(parent), position, remove) - (object ? 2 + textLength(key) + 1 : 2);
    if (room < 0) this.limit();
    const [standIn, placed] = shell(value, room);
    this.push({ operation: "splice", path: pointer(parent), offset: position, remove, value: object ? { [key]: standIn } : [standIn], continued: true });
    this.fill([...parent, key], value, placed);
  }

  private root(current: SnapshotValue, value: SnapshotValue): void {
    const kind = (item: SnapshotValue) => Array.isArray(item) ? "array" : dictionary(item) ? "object" : typeof item;
    const [present, previous] = [unitsOf(current), unitsOf(value)];
    if (present !== undefined && previous !== undefined && kind(current) === kind(value)) {
      if (previous.kind === "members") {
        const [standIn, placed] = shell(value, this.room("", 0, unitsLength(present)));
        this.push({ operation: "splice", path: "", offset: 0, remove: unitsLength(present), value: standIn, continued: true });
        return this.fillObject([], previous.members, placed);
      }
      return this.append([], 0, unitsLength(present), previous);
    }
    const room = this.budget - (byteLength(canonical({ operation: "set", path: "", value: null })) - 4);
    if (room < 0) this.limit();
    const [standIn, placed] = shell(value, room);
    this.push({ operation: "set", path: "", value: standIn });
    this.fill([], value, placed);
  }

  private fill(path: string[], value: SnapshotValue, placed: Shell): void {
    if (placed.kind === "whole") return;
    if (placed.kind === "emptied") return this.append(path, 0, 0, unitsOf(value) ?? this.limit());
    if (!dictionary(value)) this.limit();
    this.fillObject(path, Object.entries(value), placed);
  }

  private fillObject(path: string[], members: [string, SnapshotValue][], placed: Shell): void {
    if (placed.kind !== "object") return;
    placed.members.forEach((memberShell, position) => this.fill([...path, members[position]![0]], members[position]![1], memberShell));
    if (placed.placed < members.length) this.append(path, placed.placed, 0, { kind: "members", members: members.slice(placed.placed) });
  }

  private append(path: string[], offset: number, remove: number, units: Units): void {
    const text = pointer(path);
    const characters = units.kind === "text" ? Array.from(units.text) : [];
    const length = unitsLength(units);
    let [start, next, character] = [offset, 0, 0];
    while (next < length || remove > 0) {
      const room = this.room(text, start, remove) - 2;
      if (room < 0) this.limit();
      let used = 0;
      const first = next;
      const refills: [string[], SnapshotValue, Shell][] = [];
      let value: SnapshotValue;
      if (units.kind === "text") {
        let chunk = "";
        while (character < characters.length) {
          const size = textLength(characters[character]!) - 2;
          if (used + size > room) break;
          used += size;
          chunk += characters[character]!;
          next += byteLength(characters[character]!);
          character += 1;
        }
        value = chunk;
      } else if (units.kind === "items") {
        const batch: SnapshotValue[] = [];
        while (next < units.items.length) {
          const [standIn, placed] = shell(units.items[next]!, room);
          const size = (batch.length > 0 ? 1 : 0) + jsonLength(standIn);
          if (batch.length > 0 && used + size > room) break;
          used += size;
          refills.push([[...path, String(start + batch.length)], units.items[next]!, placed]);
          batch.push(standIn);
          next += 1;
        }
        value = batch;
      } else {
        const batch: [string, SnapshotValue][] = [];
        while (next < units.members.length) {
          const [key, member] = units.members[next]!;
          const [standIn, placed] = shell(member, Math.max(0, room - (textLength(key) + 1)));
          const size = (batch.length > 0 ? 1 : 0) + textLength(key) + 1 + jsonLength(standIn);
          if (batch.length > 0 && used + size > room) break;
          used += size;
          refills.push([[...path, key], member, placed]);
          batch.push([key, standIn]);
          next += 1;
        }
        value = Object.fromEntries(batch);
      }
      if (next === first && remove === 0) throw new SnapshotEditError("snapshot-edit.patch-limit", text, "a unit of the inverse cannot be split within the patch budget");
      this.push({ operation: "splice", path: text, offset: start, remove, value, continued: true });
      start += next - first;
      remove = 0;
      for (const [refillPath, member, placed] of refills) this.fill(refillPath, member, placed);
    }
  }
}

/** 🧭️ One place in a schema document: the document's `$id` and the JSON Pointer of the node inside it. */
export interface SnapshotSchemaLocation { document: string; pointer: string }
type Schema = { [key: string]: unknown };

const escape = (segment: string): string => segment.replaceAll("~", "~0").replaceAll("/", "~1");
const reference = (location: SnapshotSchemaLocation): string => location.pointer === "" ? location.document : `${location.document}#${location.pointer}`;

function node(document: unknown, fragment: string): unknown {
  let current = document;
  for (const raw of fragment.split("/").slice(1)) {
    const segment = raw.replaceAll("~1", "/").replaceAll("~0", "~");
    if (Array.isArray(current)) current = current[Number(segment)];
    else if (current !== null && typeof current === "object" && Object.hasOwn(current, segment)) current = (current as Schema)[segment];
    else return undefined;
  }
  return current;
}

function followed(location: SnapshotSchemaLocation, resolve: (id: string) => unknown): [SnapshotSchemaLocation, unknown] | undefined {
  let current = location;
  for (let hop = 0; hop < 32; hop += 1) {
    const schema = node(resolve(current.document), current.pointer);
    if (schema === undefined) return undefined;
    const ref = (schema as Schema)?.$ref;
    if (typeof ref !== "string") return [current, schema];
    const [id, fragment = ""] = ref.split("#");
    current = { document: id === "" ? current.document : id, pointer: fragment };
  }
  return undefined;
}

function memberLocations(location: SnapshotSchemaLocation, segment: string, resolve: (id: string) => unknown, depth: number): SnapshotSchemaLocation[] {
  const resolved = depth < 32 ? followed(location, resolve) : undefined;
  if (resolved === undefined) return [];
  const [at, schema] = resolved as [SnapshotSchemaLocation, Schema];
  const child = (suffix: string): SnapshotSchemaLocation => ({ document: at.document, pointer: `${at.pointer}/${suffix}` });
  const found: SnapshotSchemaLocation[] = [];
  for (const union of ["oneOf", "anyOf", "allOf"]) {
    const members = schema[union];
    if (Array.isArray(members)) members.forEach((_, position) => found.push(...memberLocations(child(`${union}/${position}`), segment, resolve, depth + 1)));
  }
  const properties = schema.properties as Schema | undefined;
  const isIndex = segment === "-" || /^[0-9]+$/u.test(segment);
  if (properties !== undefined && Object.hasOwn(properties, segment)) found.push(child(`properties/${escape(segment)}`));
  else if (isIndex && schema.items !== undefined) {
    if (Array.isArray(schema.items)) {
      const position = Number(segment);
      if (segment !== "-" && position < schema.items.length) found.push(child(`items/${position}`));
      else if (schema.additionalItems !== null && typeof schema.additionalItems === "object") found.push(child("additionalItems"));
    } else if (schema.items !== null && typeof schema.items === "object") found.push(child("items"));
  } else if (schema.additionalProperties !== null && typeof schema.additionalProperties === "object") found.push(child("additionalProperties"));
  return found;
}

/** 🧭️ The location of the sub-schema of `document` that describes the instance at `segments`, from the schema alone — the
 * TypeScript twin of `snapshot_schema_location`. `undefined` when a document does not resolve or the location is ambiguous. */
export function snapshotSchemaLocation(document: string, segments: string[], resolve: (id: string) => unknown): SnapshotSchemaLocation | undefined {
  let location: SnapshotSchemaLocation = { document, pointer: "" };
  for (const segment of segments) {
    const candidates = memberLocations(location, segment, resolve, 0);
    const first = candidates[0] === undefined ? undefined : followed(candidates[0], resolve);
    if (first === undefined) return undefined;
    for (const candidate of candidates.slice(1)) {
      const other = followed(candidate, resolve);
      if (other === undefined) return undefined;
      if (reference(other[0]) !== reference(first[0]) && JSON.stringify(other[1]) !== JSON.stringify(first[1])) return undefined;
    }
    location = candidates[0]!;
  }
  return location;
}

/** 🔗️ The `$ref` of the sub-schema that types the value `patch` writes, or `undefined` for an operation that writes none or a
 * location that does not resolve. */
export function snapshotPatchValueReference(snapshotSchema: string, patch: SnapshotPatch, resolve: (id: string) => unknown): string | undefined {
  if (patch.operation !== "set" && patch.operation !== "insert") return undefined;
  const location = snapshotSchemaLocation(snapshotSchema, decodePointer(patch.path), resolve);
  return location === undefined ? undefined : reference(location);
}
