/** ✏️ Language-neutral stdio snapshot edit protocol, TypeScript implementation. */
export type SnapshotValue = null | boolean | number | bigint | string | SnapshotValue[] | { [key: string]: SnapshotValue };

export type SnapshotEditEvent =
  | { operation: "setValue"; path: string; value: SnapshotValue }
  | { operation: "insertValue"; path: string; value: SnapshotValue }
  | { operation: "removeValue"; path: string }
  | { operation: "moveValue"; from: string; path: string }
  | { operation: "renameKey"; path: string; key: string }
  | { operation: "replaceSource"; source: string };

export class SnapshotEditError extends Error {
  constructor(readonly code: string, readonly path: string, message: string) {
    super(path ? `${message} at ${path}` : message);
  }
}

export interface SnapshotEditCodec<T extends SnapshotValue> {
  validate(value: SnapshotValue): T;
}

const define = (object: { [key: string]: SnapshotValue }, key: string, value: SnapshotValue): void => { Object.defineProperty(object, key, { value, enumerable: true, writable: true, configurable: true }); };

function validateStructural(value: SnapshotValue, path = "", depth = 0, budget = { nodes: 65_536 }): void {
  if (depth > 128) throw new SnapshotEditError("snapshot-edit.depth-exceeded", path, "snapshot nesting exceeds 128 levels");
  if (budget.nodes-- === 0) throw new SnapshotEditError("snapshot-edit.node-limit", path, "snapshot exceeds 65536 nodes");
  if (typeof value === "bigint" && (value < -9223372036854775808n || value > 18446744073709551615n)) throw new SnapshotEditError("snapshot-edit.integer-range", path, "snapshot integers must fit signed or unsigned 64-bit values");
  if (typeof value === "number" && !Number.isFinite(value)) throw new SnapshotEditError("snapshot-edit.non-finite-number", path, "snapshot numbers must be finite");
  if (Array.isArray(value)) value.forEach((item, index) => validateStructural(item, `${path}/${index}`, depth + 1, budget));
  else if (value !== null && typeof value === "object") Object.entries(value).forEach(([key, item]) => validateStructural(item, `${path}/${key.replaceAll("~", "~0").replaceAll("/", "~1")}`, depth + 1, budget));
}

function pointer(path: string): string[] {
  if (path === "") return [];
  if (!path.startsWith("/")) throw new SnapshotEditError("snapshot-edit.invalid-pointer", path, "an RFC 6901 pointer must be empty or start with '/'");
  return path.slice(1).split("/").map((raw) => {
    let value = "";
    for (let index = 0; index < raw.length; index += 1) {
      if (raw[index] !== "~") value += raw[index];
      else if (raw[index + 1] === "0") { value += "~"; index += 1; }
      else if (raw[index + 1] === "1") { value += "/"; index += 1; }
      else throw new SnapshotEditError("snapshot-edit.invalid-pointer", path, "an RFC 6901 escape must be '~0' or '~1'");
    }
    return value;
  });
}

function index(segment: string, length: number, path: string, insert: boolean): number {
  if (insert && segment === "-") return length;
  if (!/^(0|[1-9][0-9]*)$/.test(segment)) throw new SnapshotEditError("snapshot-edit.invalid-index", path, `'${segment}' is not a canonical array index`);
  const value = Number(segment);
  if (!Number.isSafeInteger(value) || (insert ? value > length : value >= length)) throw new SnapshotEditError("snapshot-edit.index-out-of-bounds", path, `array index ${segment} exceeds length ${length}`);
  return value;
}

function at(root: SnapshotValue, segments: string[], path: string): SnapshotValue {
  let value = root;
  for (const segment of segments) {
    if (Array.isArray(value)) value = value[index(segment, value.length, path, false)];
    else if (value !== null && typeof value === "object") {
      if (!Object.hasOwn(value, segment)) throw new SnapshotEditError("snapshot-edit.path-missing", path, `object key '${segment}' does not exist`);
      value = value[segment];
    } else throw new SnapshotEditError("snapshot-edit.not-container", path, `path segment '${segment}' has a scalar parent`);
  }
  return value;
}

function parent(root: SnapshotValue, path: string): [SnapshotValue, string] {
  const segments = pointer(path);
  const key = segments.pop();
  if (key === undefined) throw new SnapshotEditError("snapshot-edit.root-operation", path, "this operation cannot address the document root");
  return [at(root, segments, path), key];
}

function remove(root: SnapshotValue, path: string): SnapshotValue {
  const [container, key] = parent(root, path);
  if (Array.isArray(container)) return container.splice(index(key, container.length, path, false), 1)[0];
  if (container !== null && typeof container === "object") {
    if (!Object.hasOwn(container, key)) throw new SnapshotEditError("snapshot-edit.path-missing", path, `object key '${key}' does not exist`);
    const value = container[key];
    delete container[key];
    return value;
  }
  throw new SnapshotEditError("snapshot-edit.not-container", path, "the addressed parent is a scalar");
}

function insert(root: SnapshotValue, path: string, value: SnapshotValue): void {
  const [container, key] = parent(root, path);
  if (Array.isArray(container)) container.splice(index(key, container.length, path, true), 0, value);
  else if (container !== null && typeof container === "object") {
    if (Object.hasOwn(container, key)) throw new SnapshotEditError("snapshot-edit.key-exists", path, `object key '${key}' already exists`);
    define(container, key, value);
  } else throw new SnapshotEditError("snapshot-edit.not-container", path, "the addressed parent is a scalar");
}

function valuesEquivalent(left: SnapshotValue, right: SnapshotValue): boolean {
  if (left === right) return true;
  if (Array.isArray(left)) return Array.isArray(right) && left.length === right.length && left.every((item, index) => valuesEquivalent(item, right[index]));
  if (left === null || right === null || typeof left !== "object" || typeof right !== "object" || Array.isArray(right)) return false;
  const keys = Object.keys(left);
  return keys.length === Object.keys(right).length && keys.every((key) => Object.hasOwn(right, key) && valuesEquivalent(left[key], right[key]));
}

function parseSnapshotSource(source: string): SnapshotValue {
  JSON.parse(source);
  const tokens = source.match(/"(?:\\[\s\S]|[^"\\])*"|[{}\[\],:]|[^{}\[\],:\s]+/gu) ?? [];
  let cursor = 0;
  const parse = (depth: number): SnapshotValue => {
    if (depth > 128) throw new SnapshotEditError("snapshot-edit.depth-exceeded", "", "snapshot nesting exceeds 128 levels");
    const token = tokens[cursor++];
    if (token === "{") {
      const object: { [key: string]: SnapshotValue } = {};
      while (tokens[cursor] !== "}") {
        const key = JSON.parse(tokens[cursor++]) as string;
        if (Object.hasOwn(object, key)) throw new SnapshotEditError("snapshot-edit.ambiguous-object", "", "source repeats object key '" + key + "'");
        cursor += 1;
        define(object, key, parse(depth + 1));
        if (tokens[cursor] !== ",") break;
        cursor += 1;
      }
      cursor += 1;
      return object;
    }
    if (token === "[") {
      const array: SnapshotValue[] = [];
      while (tokens[cursor] !== "]") {
        array.push(parse(depth + 1));
        if (tokens[cursor] !== ",") break;
        cursor += 1;
      }
      cursor += 1;
      return array;
    }
    if (/^-?(0|[1-9][0-9]*)$/u.test(token)) {
      const integer = BigInt(token);
      if (integer < -9223372036854775808n || integer > 18446744073709551615n) throw new SnapshotEditError("snapshot-edit.integer-range", "", "snapshot integers must fit signed or unsigned 64-bit values");
      return integer < BigInt(Number.MIN_SAFE_INTEGER) || integer > BigInt(Number.MAX_SAFE_INTEGER) ? integer : Number(token);
    }
    return JSON.parse(token) as SnapshotValue;
  };
  return parse(0);
}

function printSnapshotSource(value: SnapshotValue, depth: number): string {
  if (typeof value === "bigint") return value.toString();
  if (value === null || typeof value !== "object") return JSON.stringify(value);
  const array = Array.isArray(value);
  const entries = array ? value.map((item) => printSnapshotSource(item, depth + 1)) : Object.entries(value).map(([key, item]) => JSON.stringify(key) + ": " + printSnapshotSource(item, depth + 1));
  const [open, close] = array ? ["[", "]"] : ["{", "}"];
  return entries.length === 0 ? open + close : open + "\n" + "  ".repeat(depth + 1) + entries.join(",\n" + "  ".repeat(depth + 1)) + "\n" + "  ".repeat(depth) + close;
}

/** 🧾️ Prints complete snapshot JSON without a native file encoding round trip. */
export function snapshotEditSource(snapshot: SnapshotValue): string {
  validateStructural(snapshot);
  return printSnapshotSource(snapshot, 0);
}

/** 🔬️ Parses complete snapshot JSON and rejects ambiguous or normalized fields. */
export function snapshotFromEditSource<T extends SnapshotValue>(source: string, codec?: SnapshotEditCodec<T>): T {
  let parsed: SnapshotValue;
  try {
    parsed = parseSnapshotSource(source);
  } catch (error) {
    if (error instanceof SnapshotEditError) throw error;
    throw new SnapshotEditError("snapshot-edit.invalid-source", "", error instanceof Error ? error.message : String(error));
  }
  validateStructural(parsed);
  const decoded = codec ? codec.validate(structuredClone(parsed)) : parsed as T;
  if (!valuesEquivalent(parsed, decoded)) throw new SnapshotEditError("snapshot-edit.lossy-conversion", "", "the typed snapshot would normalize or discard part of the source");
  return decoded;
}

export function applySnapshotEdit<T extends SnapshotValue>(snapshot: T, event: SnapshotEditEvent, codec?: SnapshotEditCodec<T>): T {
  if (event.operation === "replaceSource") {
    return snapshotFromEditSource(event.source, codec);
  }
  let next: SnapshotValue = structuredClone(snapshot);
  if (event.operation === "setValue") {
    const segments = pointer(event.path);
    if (segments.length === 0) next = structuredClone(event.value);
    else {
      const key = segments.pop()!;
      const container = at(next, segments, event.path);
      if (Array.isArray(container)) container[index(key, container.length, event.path, false)] = structuredClone(event.value);
      else if (container !== null && typeof container === "object" && Object.hasOwn(container, key)) define(container, key, structuredClone(event.value));
      else throw new SnapshotEditError("snapshot-edit.path-missing", event.path, `object key '${key}' does not exist`);
    }
  } else if (event.operation === "insertValue") insert(next, event.path, structuredClone(event.value));
  else if (event.operation === "removeValue") remove(next, event.path);
  else if (event.operation === "moveValue") {
    const from = pointer(event.from), to = pointer(event.path);
    if (from.length === 0 || to.length === 0) throw new SnapshotEditError("snapshot-edit.root-operation", event.path, "move cannot address the document root");
    if (to.length > from.length && from.every((segment, index) => to[index] === segment)) throw new SnapshotEditError("snapshot-edit.descendant-move", event.path, "a value cannot move into its own descendant");
    if (event.from !== event.path) insert(next, event.path, remove(next, event.from));
  } else {
    const [container, key] = parent(next, event.path);
    if (container === null || Array.isArray(container) || typeof container !== "object") throw new SnapshotEditError("snapshot-edit.not-object", event.path, "only an object key can be renamed");
    if (!Object.hasOwn(container, key)) throw new SnapshotEditError("snapshot-edit.path-missing", event.path, `object key '${key}' does not exist`);
    if (key !== event.key && Object.hasOwn(container, event.key)) throw new SnapshotEditError("snapshot-edit.key-exists", event.path, `object key '${event.key}' already exists`);
    const value = container[key]; delete container[key]; define(container, event.key, value);
  }
  validateStructural(next);
  const decoded = codec ? codec.validate(structuredClone(next)) : next as T;
  if (!valuesEquivalent(next, decoded)) throw new SnapshotEditError("snapshot-edit.lossy-conversion", event.path, "the typed snapshot would normalize or discard part of the edit");
  return decoded;
}
