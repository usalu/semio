/** 📖️ Owned reader of binary FBX 7.x and the language-agnostic structural digests of an encoding: node-record outline, object counts by type and the connection list resolved to names.
 *
 * It exists for the tests and for tools that inspect exports; the digests are what the fixtures of every implementation language compare.
 *
 * @see https://code.blender.org/2013/08/fbx-binary-file-format-specification/
 */
import type { Operation } from "../../../🧬️schema/🟦️.ts";
import { Pace, silentOperation } from "../../../⏱️operation/🟦️.ts";
import { inflateZlib } from "../🗜️deflate/🟦️.ts";

/** 🏷️ One decoded property: its type letter and value (`bigint` ids become numbers, raw bytes stay bytes). */
export type ReadProperty = { readonly type: string; readonly value: number | string | boolean | Uint8Array | readonly number[] };
/** 🌲️ One decoded node record. */
export type ReadNode = { readonly name: string; readonly properties: readonly ReadProperty[]; readonly children: readonly ReadNode[] };
/** 📄️ A decoded file. */
export type ReadFile = { readonly version: number; readonly nodes: readonly ReadNode[]; readonly footerStart: number };

const MAGIC = "Kaydara FBX Binary  \u0000\u001a\u0000";
const decoder = new TextDecoder();

/** 🔓️ Decodes a binary FBX file (versions below 7500); throws on a malformed file and `CancelledError` when the optional `operation` is cancelled (polled per record and every few thousand array values). */
export function readFbx(bytes: Uint8Array, operation: Operation = silentOperation): ReadFile {
  const pace = new Pace(operation);
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  if (decoder.decode(bytes.subarray(0, MAGIC.length)) !== MAGIC) throw new Error("not a binary FBX file");
  const version = view.getUint32(23, true);
  let offset = 27;
  const readProperty = (): ReadProperty => {
    const type = String.fromCharCode(bytes[offset++]);
    switch (type) {
      case "C":
        return { type, value: (bytes[offset++] & 1) === 1 };
      case "Y":
        offset += 2;
        return { type, value: view.getInt16(offset - 2, true) };
      case "I":
        offset += 4;
        return { type, value: view.getInt32(offset - 4, true) };
      case "F":
        offset += 4;
        return { type, value: view.getFloat32(offset - 4, true) };
      case "D":
        offset += 8;
        return { type, value: view.getFloat64(offset - 8, true) };
      case "L":
        offset += 8;
        return { type, value: Number(view.getBigInt64(offset - 8, true)) };
      case "S":
      case "R": {
        const length = view.getUint32(offset, true);
        offset += 4 + length;
        const slice = bytes.slice(offset - length, offset);
        return { type, value: type === "S" ? decoder.decode(slice) : slice };
      }
      case "b":
      case "i":
      case "l":
      case "f":
      case "d": {
        const count = view.getUint32(offset, true);
        const encoding = view.getUint32(offset + 4, true);
        const stored = view.getUint32(offset + 8, true);
        offset += 12;
        const packed = bytes.subarray(offset, offset + stored);
        offset += stored;
        const data = encoding === 1 ? inflateZlib(packed, operation) : packed;
        const source = new DataView(data.buffer, data.byteOffset, data.byteLength);
        const width = type === "b" ? 1 : type === "i" || type === "f" ? 4 : 8;
        if (data.length !== count * width) throw new Error("array length mismatch");
        const values = Array.from({ length: count }, (_, index) => (pace.tick(), type === "b" ? source.getUint8(index) : type === "i" ? source.getInt32(index * 4, true) : type === "l" ? Number(source.getBigInt64(index * 8, true)) : type === "f" ? source.getFloat32(index * 4, true) : source.getFloat64(index * 8, true)));
        return { type, value: values };
      }
      default:
        throw new Error(`unknown property type ${type}`);
    }
  };
  const readNode = (): ReadNode | undefined => {
    pace.tick(16);
    const end = view.getUint32(offset, true);
    const count = view.getUint32(offset + 4, true);
    const nameLength = bytes[offset + 12];
    if (end === 0) {
      offset += 13;
      return undefined;
    }
    const name = decoder.decode(bytes.subarray(offset + 13, offset + 13 + nameLength));
    offset += 13 + nameLength;
    const properties = Array.from({ length: count }, readProperty);
    const children: ReadNode[] = [];
    while (offset < end) {
      const child = readNode();
      if (child) children.push(child);
    }
    if (offset !== end) throw new Error(`node ${name} ends at ${offset}, expected ${end}`);
    return { name, properties, children };
  };
  const nodes: ReadNode[] = [];
  for (;;) {
    const probe = view.getUint32(offset, true);
    if (probe === 0) break;
    const record = readNode();
    if (record) nodes.push(record);
  }
  return { version, nodes, footerStart: offset + 13 };
}

/** 👶️ The first child named `name`. */
export function child(parent: ReadNode | undefined, name: string): ReadNode | undefined {
  return parent?.children.find((item) => item.name === name);
}

/** 🧒️ The children named `name`. */
export function childrenNamed(parent: ReadNode | undefined, name: string): ReadNode[] {
  return parent?.children.filter((item) => item.name === name) ?? [];
}

/** 🔑️ The first property value of the child `name` (the FBX idiom for a scalar field). */
export function fieldValue(parent: ReadNode | undefined, name: string): ReadProperty["value"] | undefined {
  return child(parent, name)?.properties[0]?.value;
}

function signature(properties: readonly ReadProperty[]): string {
  return properties.map((property) => (Array.isArray(property.value) ? `${property.type}*${property.value.length}` : property.type === "R" ? `R*${(property.value as Uint8Array).length}` : property.type)).join(",");
}

/** 🧾️ The record tree as indented lines `Name(types)`; array properties show their length (`d*24`). */
export function outline(file: ReadFile): string[] {
  const lines: string[] = [];
  const walk = (record: ReadNode, depth: number): void => {
    lines.push(`${"  ".repeat(depth)}${record.name}(${signature(record.properties)})`);
    record.children.forEach((item) => walk(item, depth + 1));
  };
  file.nodes.forEach((record) => walk(record, 0));
  return lines;
}

function label(record: ReadNode): string {
  const [name, kind] = [record.properties[1]?.value, record.properties[2]?.value];
  const [base, klass] = typeof name === "string" ? name.split("\u0000\u0001") : ["", ""];
  return `${record.name}:${base}${klass && klass !== record.name ? `<${klass}>` : ""}${typeof kind === "string" && kind !== "" ? `[${kind}]` : ""}`;
}

/** 🧮️ Object counts by `Objects` child name. */
export function objectCounts(file: ReadFile): Record<string, number> {
  const counts: Record<string, number> = {};
  for (const record of file.nodes.find((item) => item.name === "Objects")?.children ?? []) counts[record.name] = (counts[record.name] ?? 0) + 1;
  return Object.fromEntries(Object.entries(counts).sort(([a], [b]) => a.localeCompare(b)));
}

/** 🔗️ The connections as `Kind:name[subtype] -> Kind:name[subtype] (OO|OP property)` lines in file order; the root is `Root`. */
export function connectionList(file: ReadFile): string[] {
  const objects = file.nodes.find((item) => item.name === "Objects")?.children ?? [];
  const names = new Map<number, string>(objects.map((record) => [record.properties[0].value as number, label(record)]));
  names.set(0, "Root");
  return (file.nodes.find((item) => item.name === "Connections")?.children ?? []).map((record) => {
    const [kind, from, to, property] = record.properties.map((item) => item.value);
    return `${names.get(from as number) ?? `?${from}`} -> ${names.get(to as number) ?? `?${to}`} (${kind}${property === undefined ? "" : ` ${property}`})`;
  });
}
