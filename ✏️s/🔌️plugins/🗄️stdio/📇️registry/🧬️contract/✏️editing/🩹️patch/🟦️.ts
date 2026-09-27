import { SnapshotEditError, snapshotEditSource, snapshotFromEditSource, type SnapshotEditEvent, type SnapshotValue } from "../🟦️.js";

export type SnapshotPatchEdit = { operation: "set" | "insert"; value: SnapshotValue } | { operation: "remove" };
export interface SnapshotValuePatch { path: string[]; edit: SnapshotPatchEdit }
export interface SnapshotPatch { edits: SnapshotValuePatch[] }
export const SNAPSHOT_PATCH_MAX_BYTES = 1_048_576;

const pointer = (path: string[]): string => path.map((segment) => "/" + segment.replaceAll("~", "~0").replaceAll("/", "~1")).join("");
const fail = (path: string[], message: string): never => { throw new SnapshotEditError("snapshot-edit.path-invalid", pointer(path), message); };
const dictionary = (value: SnapshotValue): value is Record<string, SnapshotValue> => value !== null && typeof value === "object" && !Array.isArray(value);

function decodePointer(path: string): string[] {
  if (path === "") return [];
  if (!path.startsWith("/")) return fail([], "an RFC 6901 pointer must start with '/'");
  return path.slice(1).split("/").map((segment) => {
    if (/~(?![01])/u.test(segment)) return fail([], "invalid RFC 6901 escape");
    return segment.replaceAll("~1", "/").replaceAll("~0", "~");
  });
}

function index(segment: string, length: number, insert: boolean, path: string[]): number {
  if (insert && segment === "-") return length;
  const value = Number(segment);
  if (!/^(0|[1-9][0-9]*)$/u.test(segment) || !Number.isSafeInteger(value) || (insert ? value > length : value >= length)) return fail(path, "invalid array index");
  return value;
}

function at(snapshot: SnapshotValue, path: string[]): SnapshotValue {
  let value = snapshot;
  for (const segment of path) {
    if (Array.isArray(value)) value = value[index(segment, value.length, false, path)];
    else if (dictionary(value) && Object.hasOwn(value, segment)) value = value[segment];
    else return fail(path, "the addressed field does not exist");
  }
  return value;
}

function normalizePath(snapshot: SnapshotValue, patch: SnapshotValuePatch): string[] {
  const path = patch.path.slice();
  if (patch.edit.operation === "insert" && path.at(-1) === "-") {
    const parent = at(snapshot, path.slice(0, -1));
    if (Array.isArray(parent)) path[path.length - 1] = String(parent.length);
  }
  return path;
}

function validatePatch(patch: SnapshotPatch): void {
  if (patch.edits.length > 2 || patch.edits.some((row) => row.path.length > 128 || row.path.some((segment) => typeof segment !== "string"))) {
    throw new SnapshotEditError("snapshot-edit.patch-limit", "", "a snapshot patch permits two edits and 128 path segments");
  }
  const source = snapshotEditSource(patch as unknown as SnapshotValue);
  if (new TextEncoder().encode(source).length > SNAPSHOT_PATCH_MAX_BYTES) throw new SnapshotEditError("snapshot-edit.patch-limit", "", "snapshot patch exceeds the native publication item limit");
}

function applyOne(snapshot: SnapshotValue, row: SnapshotValuePatch): SnapshotValue {
  const path = normalizePath(snapshot, row);
  const descend = (value: SnapshotValue, depth: number): SnapshotValue => {
    if (depth === path.length) {
      if (row.edit.operation !== "set") return fail(path, "only replacement can address the document root");
      return structuredClone(row.edit.value);
    }
    const key = path[depth];
    const leaf = depth + 1 === path.length;
    if (Array.isArray(value)) {
      const position = index(key, value.length, leaf && row.edit.operation === "insert", path);
      const next = value.slice();
      if (!leaf) next[position] = descend(value[position], depth + 1);
      else if (row.edit.operation === "insert") next.splice(position, 0, structuredClone(row.edit.value));
      else if (row.edit.operation === "remove") next.splice(position, 1);
      else next[position] = structuredClone(row.edit.value);
      return next;
    }
    if (!dictionary(value)) return fail(path, "the addressed parent is a scalar");
    const exists = Object.hasOwn(value, key);
    if ((!leaf || row.edit.operation !== "insert") && !exists) return fail(path, "the addressed field does not exist");
    if (leaf && row.edit.operation === "insert" && exists) return fail(path, "the object key already exists");
    const next = { ...value };
    if (leaf && row.edit.operation === "remove") delete next[key];
    else Object.defineProperty(next, key, { value: leaf ? structuredClone((row.edit as { value: SnapshotValue }).value) : descend(value[key], depth + 1), enumerable: true, writable: true, configurable: true });
    return next;
  };
  return descend(snapshot, 0);
}

/** 🧭️ Translates an edit into compact path operations without cloning unrelated payloads. */
export function prepareSnapshotPatch(snapshot: SnapshotValue, event: SnapshotEditEvent): SnapshotPatch {
  let edits: SnapshotValuePatch[];
  switch (event.operation) {
    case "setValue": edits = [{ path: decodePointer(event.path), edit: { operation: "set", value: event.value } }]; break;
    case "insertValue": edits = [{ path: decodePointer(event.path), edit: { operation: "insert", value: event.value } }]; break;
    case "removeValue": edits = [{ path: decodePointer(event.path), edit: { operation: "remove" } }]; break;
    case "replaceSource": edits = [{ path: [], edit: { operation: "set", value: snapshotFromEditSource(event.source) } }]; break;
    case "moveValue": {
      const from = decodePointer(event.from), path = decodePointer(event.path);
      if (from.length === 0 || path.length === 0 || (path.length > from.length && from.every((segment, i) => path[i] === segment))) return fail(path, "move cannot address the root or a descendant of its source");
      const value = at(snapshot, from);
      if (event.from === event.path) edits = [];
      else {
        const remove: SnapshotValuePatch = { path: from, edit: { operation: "remove" } };
        const insert: SnapshotValuePatch = { path, edit: { operation: "insert", value } };
        edits = [remove, { ...insert, path: normalizePath(applyOne(snapshot, remove), insert) }];
      }
      break;
    }
    case "renameKey": {
      const path = decodePointer(event.path), parent = path.slice(0, -1);
      if (path.length === 0 || !dictionary(at(snapshot, parent))) return fail(path, "only an object key can be renamed");
      const value = at(snapshot, path);
      edits = path.at(-1) === event.key ? [] : [{ path, edit: { operation: "remove" } }, { path: [...parent, event.key], edit: { operation: "insert", value } }];
      break;
    }
  }
  if (edits.length === 1) edits[0] = { ...edits[0], path: normalizePath(snapshot, edits[0]) };
  const patch = { edits };
  validatePatch(patch);
  return patch;
}

/** 🛡️ Applies an atomic structural patch while preserving every untouched subtree. */
export function applySnapshotPatch(snapshot: SnapshotValue, patch: SnapshotPatch): SnapshotValue {
  validatePatch(patch);
  return patch.edits.reduce(applyOne, snapshot);
}

/** ↩️ Captures exact inverse values from the current publication base. */
export function inverseSnapshotPatch(snapshot: SnapshotValue, patch: SnapshotPatch): SnapshotPatch {
  validatePatch(patch);
  const edits: SnapshotValuePatch[] = [];
  let current = snapshot;
  for (const row of patch.edits) {
    const path = normalizePath(current, row);
    const edit: SnapshotPatchEdit = row.edit.operation === "insert" ? { operation: "remove" } : { operation: row.edit.operation === "set" ? "set" : "insert", value: at(current, path) };
    current = applyOne(current, row);
    edits.unshift({ path, edit });
  }
  const inverse = { edits };
  validatePatch(inverse);
  return inverse;
}
