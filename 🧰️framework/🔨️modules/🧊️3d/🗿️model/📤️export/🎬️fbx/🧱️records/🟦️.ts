/** 🧱️ Binary FBX 7.4 node records: typed properties, the record tree and its little-endian serialisation with header, sentinels and footer.
 *
 * @see https://code.blender.org/2013/08/fbx-binary-file-format-specification/
 */
import type { Operation } from "../../../🧬️schema/🟦️.ts";
import { Pace, silentOperation } from "../../../⏱️operation/🟦️.ts";
import { ByteWriter } from "../../🧱️bytes/🟦️.ts";
import type { Compressor } from "../🗜️deflate/🟦️.ts";

/** 🏷️ One typed FBX property: scalars `C Y I F D L`, raw `R`, string `S`, arrays `b i l f d`. */
export type Property =
  | { readonly type: "C"; readonly value: boolean }
  | { readonly type: "I" | "F" | "D" | "L"; readonly value: number }
  | { readonly type: "S"; readonly value: string }
  | { readonly type: "R"; readonly value: Uint8Array }
  | { readonly type: "i" | "l" | "f" | "d"; readonly value: readonly number[] };

/** 🌲️ One node record: a name, its properties and its children. */
export type FbxNode = { readonly name: string; readonly properties: readonly Property[]; readonly children: readonly FbxNode[] };

/** 🏗️ Builds a node record. */
export function node(name: string, properties: readonly Property[] = [], children: readonly FbxNode[] = []): FbxNode {
  return { name, properties, children };
}

export const int = (value: number): Property => ({ type: "I", value });
export const long = (value: number): Property => ({ type: "L", value });
export const double = (value: number): Property => ({ type: "D", value });
export const text = (value: string): Property => ({ type: "S", value });
export const flag = (value: boolean): Property => ({ type: "C", value });
export const raw = (value: Uint8Array): Property => ({ type: "R", value });
export const ints = (value: readonly number[]): Property => ({ type: "i", value });
export const longs = (value: readonly number[]): Property => ({ type: "l", value });
export const floats = (value: readonly number[]): Property => ({ type: "f", value });
export const doubles = (value: readonly number[]): Property => ({ type: "d", value });

/** 🔖️ A node holding exactly one property, the FBX idiom for a scalar field. */
export function field(name: string, property: Property): FbxNode {
  return node(name, [property]);
}

/** 📝️ One `P` entry of a `Properties70` block; numbers become doubles, except the integer-like types, booleans are 32-bit integers (assimp rejects any other binary type for them). */
export function property70(name: string, type: string, label: string, flags: string, ...values: readonly (number | string | boolean)[]): FbxNode {
  const integral = type === "int" || type === "enum" || type === "Integer";
  const typed = values.map((value): Property => (typeof value === "boolean" ? int(value ? 1 : 0) : typeof value === "string" ? text(value) : type === "KTime" ? long(value) : integral ? int(value) : double(value)));
  return node("P", [text(name), text(type), text(label), text(flags), ...typed]);
}

/** 🗃️ A `Properties70` block. */
export function properties70(...entries: readonly FbxNode[]): FbxNode {
  return node("Properties70", [], entries);
}

/** ⏱️ Ticks of an FBX time per second. */
export const TICKS_PER_SECOND = 46186158000;

/** ⏲️ A time in seconds as FBX ticks. */
export function ticks(seconds: number): number {
  return Math.round(seconds * TICKS_PER_SECOND);
}

const FILE_ID = Uint8Array.from([0x28, 0xb3, 0x2a, 0xeb, 0xb6, 0x24, 0xcc, 0xc2, 0xbf, 0xc8, 0xb0, 0x2a, 0xa9, 0x2b, 0xfc, 0xf1]);
const FOOTER_ID = Uint8Array.from([0xfa, 0xbc, 0xab, 0x09, 0xd0, 0xc8, 0xd4, 0x66, 0xb1, 0x76, 0xfb, 0x83, 0x1c, 0xf7, 0x26, 0x7e]);
const FOOTER_MAGIC = Uint8Array.from([0xf8, 0x5a, 0x8c, 0x6a, 0xde, 0xf5, 0xd9, 0x7e, 0xec, 0xe9, 0x0c, 0xe3, 0x75, 0x8f, 0x29, 0x0b]);
const HEADER = "Kaydara FBX Binary  \u0000\u001a\u0000";
const SENTINEL_BYTES = 13;
const COMPRESSION_THRESHOLD = 128;

/** 🆔️ The fixed 16-byte `FileId` of the header extension (the creation time it stands for is fixed too, so files never depend on the clock). */
export const FILE_ID_BYTES = FILE_ID;
/** 🕰️ The fixed creation time stamp written to every file. */
export const CREATION_TIME = "1970-01-01 10:00:00:000";

const encoder = new TextEncoder();

function arrayBytes(type: "i" | "l" | "f" | "d", values: readonly number[], pace: Pace): Uint8Array {
  const scratch = new ByteWriter(values.length * (type === "i" || type === "f" ? 4 : 8));
  for (const value of values) {
    pace.tick();
    if (type === "i") scratch.i32(value);
    else if (type === "l") scratch.i64(value);
    else if (type === "f") scratch.f32(value);
    else scratch.f64(value);
  }
  return scratch.written();
}

function writeProperty(out: ByteWriter, property: Property, compressor: Compressor, pace: Pace): void {
  out.u8(property.type.charCodeAt(0));
  switch (property.type) {
    case "C":
      out.u8(property.value ? 1 : 0);
      return;
    case "I":
      out.i32(property.value);
      return;
    case "F":
      out.f32(property.value);
      return;
    case "D":
      out.f64(property.value);
      return;
    case "L":
      out.i64(property.value);
      return;
    case "S": {
      const bytes = encoder.encode(property.value);
      out.u32(bytes.length);
      out.append(bytes);
      return;
    }
    case "R":
      out.u32(property.value.length);
      out.append(property.value);
      return;
    default: {
      const data = arrayBytes(property.type, property.value, pace);
      out.u32(property.value.length);
      const packed = data.length >= COMPRESSION_THRESHOLD ? compressor.zlib(data, pace.operation) : undefined;
      if (packed && packed.length < data.length) {
        out.u32(1);
        out.u32(packed.length);
        out.append(packed);
      } else {
        out.u32(0);
        out.u32(data.length);
        out.append(data);
      }
    }
  }
}

function writeNode(out: ByteWriter, record: FbxNode, compressor: Compressor, pace: Pace, check: () => void): void {
  check();
  const start = out.length;
  out.u32(0);
  out.u32(record.properties.length);
  out.u32(0);
  const name = encoder.encode(record.name);
  out.u8(name.length);
  out.append(name);
  const propertiesStart = out.length;
  for (const property of record.properties) writeProperty(out, property, compressor, pace);
  out.patchU32(start + 8, out.length - propertiesStart);
  if (record.children.length > 0) {
    for (const child of record.children) writeNode(out, child, compressor, pace, check);
    for (let index = 0; index < SENTINEL_BYTES; index++) out.u8(0);
  }
  out.patchU32(start, out.length);
}

/** 💾️ Serialises top-level records into a complete binary FBX 7.4 file; `check` runs between records so a caller can cancel and report progress, the optional `operation` is polled every few thousand array values and handed to the compressor. */
export function serialize(records: readonly FbxNode[], version: number, compressor: Compressor, check: (done: number, total: number) => void, operation: Operation = silentOperation): Uint8Array {
  const pace = new Pace(operation);
  const out = new ByteWriter();
  out.append(encoder.encode(HEADER));
  out.u32(version);
  records.forEach((record, index) => {
    check(index, records.length);
    writeNode(out, record, compressor, pace, () => check(index, records.length));
  });
  for (let index = 0; index < SENTINEL_BYTES; index++) out.u8(0);
  out.append(FOOTER_ID);
  for (let index = 0; index < 4; index++) out.u8(0);
  let padding = ((out.length + 15) & ~15) - out.length;
  if (padding === 0) padding = 16;
  for (let index = 0; index < padding; index++) out.u8(0);
  out.u32(version);
  for (let index = 0; index < 120; index++) out.u8(0);
  out.append(FOOTER_MAGIC);
  return out.finish();
}
