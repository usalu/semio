// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

/**
 * 🟦️ Independent TypeScript implementation of the `stdio.semio.mesh` carrier and its seventeen-verb
 * mutation vocabulary — the differential ORACLE this case is measured against.
 *
 * Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. Two independent producers meet in this file, and each
 * answers the half it genuinely speaks.
 *
 * **three.js (r185) is a real third party and it speaks the GEOMETRY.** Every scenario that produces
 * primitives builds a real `THREE.BufferGeometry` from them — positions in a `Float64Array` so
 * nothing is quantised on the way in — and lets three.js state what it is: the attribute counts, the
 * bounding box `Box3` computes from the position attribute, and the flat vertex stream
 * `BufferGeometry.toNonIndexed()` produces by walking the index buffer. The Rust subject computes
 * those same facts by hand from the same primitives, so a projection matches only when a 3D engine
 * that has never seen this repository and this repository's own codec agree about the actual mesh.
 * What three.js does NOT do is read `.dsl.semio` or hold an opinion about a mutation verb, and that
 * boundary is named here rather than blurred.
 *
 * **This module is the second IMPLEMENTATION, for the half no third party speaks.** The carrier and
 * the vocabulary are semio's own, so they were written here from the committed specification
 * documents alone:
 * - the envelope — `semio <schema>.dsl v<version>` for text, `0x89 'S' 'E' 'M' 0D 0A 1A 0A` plus a
 *   little-endian u32 token length and the token for binary — from
 *   `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s envelope section;
 * - the DSL body from `../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
 *   (`document = artifact-mark schema-line meshes-line materials-line textures-line`, the six
 *   `topology` letters, `option-hex`, and plain bracketed number lists for the geometry buffers);
 * - the pack frame from `…/📸️snapshot/💾️binary/📡️.protocol.semio` and its Kaitai mirror,
 *   which names what the opaque tail holds: *"per-mesh id + varint primitive count, per-primitive
 *   id/topology-tag/positions/normals/uvs/colors(real f64/f32 LE buffers)/indices(u32 LE)/
 *   material_id-option, per-material id/baseColor/metallic/roughness, per-texture id/mime/raw-bytes"*.
 *   The exact field order was DERIVED from that description together with the committed
 *   `📚️examples/🧊️cube` bytes, whose DSL twin pins every field against a readable spelling, and the
 *   derivation is pinned by re-encoding that committed file byte for byte;
 * - the seventeen verbs, their argument lists and their JSON wire form from
 *   `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`, the committed proto and JSON schema
 *   mirrors, and the committed per-kind `(before, mutation, after)` specification vectors.
 *
 * Nothing here imports, links, wraps or transliterates the Rust subject. Where the two disagree the
 * disagreement is a finding, not something to tune away.
 */

// #region 🔌️Adapters
import * as THREE from "three";
import { defineTestAdapter, digest, type AdapterContext, type AdapterOutcome } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧬️Model
type Point3 = { x: number; y: number; z: number };
type Uv = { u: number; v: number };
type Rgba = { r: number; g: number; b: number; a: number };
type Primitive = { id: string; topology: string; positions: Point3[]; normals: Point3[]; uvs: Uv[]; colors: Rgba[]; indices: number[]; materialId: string | null };
type Mesh = { id: string; primitives: Primitive[] };
type Material = { id: string; baseColor: Rgba; metallic: number; roughness: number };
type Texture = { id: string; mime: string; bytes: number[] };
type Snapshot = { schema: string; meshes: Mesh[]; materials: Material[]; textures: Texture[] };
type Mutation = Record<string, Record<string, unknown>>;

const BINARY_MAGIC = new Uint8Array([0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]);
const DSL_PREAMBLE = "semio stdio.semio.mesh.dsl v1";
const PACK_TOKEN = "stdio.semio.mesh.pack v1";
const PACK_FORMAT = 1;

/** 🔺️ `topology = "P" | "L" | "S" | "T" | "X" | "F"`, in the enum order the pack tag byte indexes —
 * the committed cube's `T` primitive carries `0x03`, the fourth ordinal. */
const TOPOLOGY_ORDER = ["points", "lines", "lineStrip", "triangles", "triangleStrip", "triangleFan"] as const;
const TOPOLOGY_LETTER: Record<string, string> = { points: "P", lines: "L", lineStrip: "S", triangles: "T", triangleStrip: "X", triangleFan: "F" };
const LETTER_TOPOLOGY: Record<string, string> = Object.fromEntries(Object.entries(TOPOLOGY_LETTER).map(([kind, letter]) => [letter, kind]));

const KINDS = [
  "create-mesh",
  "delete-mesh",
  "create-primitive",
  "delete-primitive",
  "set-primitive-topology",
  "replace-primitive-geometry",
  "set-primitive-material",
  "create-material",
  "delete-material",
  "change-material-base-color",
  "change-material-metallic",
  "change-material-roughness",
  "create-texture",
  "delete-texture",
  "change-texture-mime",
  "replace-texture-bytes",
  "move-vertex",
] as const;

const ARTIFACT_DSL = "local://🗣️.dsl.semio";
const ARTIFACT_PACK = "local://🎒️.pack.semio";
// #endregion 🧬️Model

// #region 🔡️Leaves
/** 🔡️ The grammar's built-in `hex` macro in the writing direction, for a string leaf. */
function hexOfText(text: string): string {
  return Array.from(new TextEncoder().encode(text))
    .map((byte) => byte.toString(16).padStart(2, "0"))
    .join("");
}

/** 🔡️ The grammar's `hex` macro in the reading direction, for a string leaf. */
function textOfHex(hexed: string): string {
  return new TextDecoder("utf-8", { fatal: true }).decode(bytesOfHex(hexed));
}

function hexOfBytes(bytes: number[]): string {
  return bytes.map((byte) => byte.toString(16).padStart(2, "0")).join("");
}

function bytesOfHex(hexed: string): Uint8Array {
  const out = new Uint8Array(hexed.length / 2);
  for (let at = 0; at < out.length; at += 1) out[at] = Number.parseInt(hexed.slice(at * 2, at * 2 + 2), 16);
  return out;
}

/** 🔢️ Expands JavaScript's exponent form into the plain decimal the grammar's `number` token
 * admits. `INT | FLOAT` has no exponent alternative, and the reference printer never emits one. */
function plainDecimal(text: string): string {
  const match = /^(-?)(\d+)(?:\.(\d+))?[eE]([+-]?\d+)$/.exec(text);
  if (match === null) return text;
  const [, sign, whole, fraction = "", exponent] = match;
  const digits = whole! + fraction;
  const point = whole!.length + Number(exponent);
  if (point <= 0) return `${sign}0.${"0".repeat(-point)}${digits}`;
  if (point >= digits.length) return `${sign}${digits}${"0".repeat(point - digits.length)}`;
  return `${sign}${digits.slice(0, point)}.${digits.slice(point)}`;
}

/** 🔢️ One `f64` leaf, printed the way the reference printer prints it: the shortest decimal that
 * reads back as the same double, with no exponent and no trailing `.0`. */
function printF64(value: number): string {
  return plainDecimal(String(value));
}

/** 🔢️ One `f32` leaf. `SemioRgba` and a material's `metallic`/`roughness` are single precision, so
 * the shortest decimal that round-trips is the shortest that survives `Math.fround` — printing the
 * widened double instead would spell `0.1` as `0.10000000149011612`. */
function printF32(value: number): string {
  for (let precision = 1; precision <= 9; precision += 1) {
    const candidate = Number(value.toPrecision(precision));
    if (Math.fround(candidate) === value) return plainDecimal(String(candidate));
  }
  return plainDecimal(String(value));
}
// #endregion 🔡️Leaves

// #region 📜️TextPrimitives
/** 📜️ Splits a list body on the commas that sit at bracket depth zero. */
function splitTopLevel(text: string): string[] {
  const parts: string[] = [];
  let depth = 0;
  let current = "";
  for (const character of text) {
    if (character === "[") depth += 1;
    else if (character === "]") depth -= 1;
    if (character === "," && depth === 0) {
      parts.push(current);
      current = "";
      continue;
    }
    current += character;
  }
  if (current !== "" || parts.length > 0) parts.push(current);
  return parts;
}

function stripBrackets(text: string, what: string): string {
  if (text.length < 2 || !text.startsWith("[") || !text.endsWith("]")) throw new Error(`${what} must be a bracketed group, found ${text.slice(0, 60)}`);
  return text.slice(1, -1);
}

function itemsOf(text: string, what: string): string[] {
  return splitTopLevel(stripBrackets(text, what));
}

function parseOptionHex(text: string, what: string): string | null {
  const parts = itemsOf(text, what);
  if (parts.length === 1 && parts[0] === "0") return null;
  if (parts.length === 2 && parts[0] === "1") return textOfHex(parts[1]!);
  throw new Error(`${what} is not a well-formed option-hex: ${text.slice(0, 60)}`);
}

function printOptionHex(value: string | null): string {
  return value === null ? "[0]" : `[1,${hexOfText(value)}]`;
}

function parsePoint3(text: string): Point3 {
  const parts = itemsOf(text, "a point3");
  if (parts.length !== 3) throw new Error(`a point3 carries three leaves, found ${parts.length}`);
  return { x: Number(parts[0]), y: Number(parts[1]), z: Number(parts[2]) };
}

function printPoint3(point: Point3): string {
  return `[${printF64(point.x)},${printF64(point.y)},${printF64(point.z)}]`;
}

function parseUv(text: string): Uv {
  const parts = itemsOf(text, "a uv");
  if (parts.length !== 2) throw new Error(`a uv carries two leaves, found ${parts.length}`);
  return { u: Number(parts[0]), v: Number(parts[1]) };
}

function printUv(uv: Uv): string {
  return `[${printF64(uv.u)},${printF64(uv.v)}]`;
}

function parseRgba(text: string): Rgba {
  const parts = itemsOf(text, "an rgba");
  if (parts.length !== 4) throw new Error(`an rgba carries four leaves, found ${parts.length}`);
  return { r: Math.fround(Number(parts[0])), g: Math.fround(Number(parts[1])), b: Math.fround(Number(parts[2])), a: Math.fround(Number(parts[3])) };
}

function printRgba(colour: Rgba): string {
  return `[${printF32(colour.r)},${printF32(colour.g)},${printF32(colour.b)},${printF32(colour.a)}]`;
}
// #endregion 📜️TextPrimitives

// #region 📜️Dsl
function stripPreamble(text: string): string {
  const at = text.indexOf("\n");
  const line = at === -1 ? text : text.slice(0, at);
  if (line !== DSL_PREAMBLE) throw new Error(`the text envelope preamble is ${line}, expected ${DSL_PREAMBLE}`);
  return at === -1 ? "" : text.slice(at + 1);
}

function readField(body: string, name: string): [string, string] {
  const at = body.indexOf("\n");
  const line = at === -1 ? body : body.slice(0, at);
  const rest = at === -1 ? "" : body.slice(at + 1);
  if (!line.startsWith(`${name}=`)) throw new Error(`expected a ${name} line, found ${line.slice(0, 60)}`);
  return [line.slice(name.length + 1), rest];
}

function parsePrimitive(text: string): Primitive {
  const parts = itemsOf(text, "a primitive");
  if (parts.length !== 8) throw new Error(`a primitive carries eight leaves, found ${parts.length}`);
  const topology = LETTER_TOPOLOGY[parts[1]!];
  if (topology === undefined) throw new Error(`${parts[1]} is not one of the six topology letters`);
  const indices = itemsOf(parts[6]!, "indices");
  return {
    id: textOfHex(parts[0]!),
    topology,
    positions: itemsOf(parts[2]!, "positions").map(parsePoint3),
    normals: itemsOf(parts[3]!, "normals").map(parsePoint3),
    uvs: itemsOf(parts[4]!, "uvs").map(parseUv),
    colors: itemsOf(parts[5]!, "colors").map(parseRgba),
    indices: indices.map((entry) => Number(entry)),
    materialId: parseOptionHex(parts[7]!, "materialId"),
  };
}

function printPrimitive(primitive: Primitive): string {
  return `[${hexOfText(primitive.id)},${TOPOLOGY_LETTER[primitive.topology]},[${primitive.positions.map(printPoint3).join(",")}],[${primitive.normals.map(printPoint3).join(",")}],[${primitive.uvs.map(printUv).join(",")}],[${primitive.colors.map(printRgba).join(",")}],[${primitive.indices.join(",")}],${printOptionHex(primitive.materialId)}]`;
}

function parseMesh(text: string): Mesh {
  const parts = itemsOf(text, "a mesh");
  if (parts.length !== 2) throw new Error(`a mesh carries an id and a primitive list, found ${parts.length} leaves`);
  return { id: textOfHex(parts[0]!), primitives: itemsOf(parts[1]!, "primitives").map(parsePrimitive) };
}

function printMesh(mesh: Mesh): string {
  return `[${hexOfText(mesh.id)},[${mesh.primitives.map(printPrimitive).join(",")}]]`;
}

function parseMaterial(text: string): Material {
  const parts = itemsOf(text, "a material");
  if (parts.length !== 4) throw new Error(`a material carries four leaves, found ${parts.length}`);
  return { id: textOfHex(parts[0]!), baseColor: parseRgba(parts[1]!), metallic: Math.fround(Number(parts[2])), roughness: Math.fround(Number(parts[3])) };
}

function printMaterial(material: Material): string {
  return `[${hexOfText(material.id)},${printRgba(material.baseColor)},${printF32(material.metallic)},${printF32(material.roughness)}]`;
}

function parseTexture(text: string): Texture {
  const parts = itemsOf(text, "a texture");
  if (parts.length !== 3) throw new Error(`a texture carries three leaves, found ${parts.length}`);
  return { id: textOfHex(parts[0]!), mime: textOfHex(parts[1]!), bytes: Array.from(bytesOfHex(parts[2]!)) };
}

function printTexture(texture: Texture): string {
  return `[${hexOfText(texture.id)},${hexOfText(texture.mime)},${hexOfBytes(texture.bytes)}]`;
}

/** 📜️ `document = artifact-mark schema-line meshes-line materials-line textures-line`. */
export function parseDsl(text: string): Snapshot {
  let body = stripPreamble(text);
  let schemaHex: string;
  let meshes: string;
  let materials: string;
  let textures: string;
  [schemaHex, body] = readField(body, "schema");
  [meshes, body] = readField(body, "meshes");
  [materials, body] = readField(body, "materials");
  [textures, body] = readField(body, "textures");
  if (body !== "") throw new Error(`the document carries trailing content after its textures line: ${body.slice(0, 60)}`);
  return {
    schema: textOfHex(schemaHex),
    meshes: itemsOf(meshes, "meshes").map(parseMesh),
    materials: itemsOf(materials, "materials").map(parseMaterial),
    textures: itemsOf(textures, "textures").map(parseTexture),
  };
}

/** 📜️ The committed DSL grammar in the writing direction, line for line in its declared order. */
export function printDsl(document: Snapshot): string {
  return [
    DSL_PREAMBLE,
    `schema=${hexOfText(document.schema)}`,
    `meshes=[${document.meshes.map(printMesh).join(",")}]`,
    `materials=[${document.materials.map(printMaterial).join(",")}]`,
    `textures=[${document.textures.map(printTexture).join(",")}]`,
  ].join("\n");
}
// #endregion 📜️Dsl

// #region 🎒️Pack
class Reader {
  constructor(
    private readonly data: Uint8Array,
    private at = 0,
  ) {}

  get offset(): number {
    return this.at;
  }

  get done(): boolean {
    return this.at === this.data.length;
  }

  byte(): number {
    const value = this.data[this.at];
    if (value === undefined) throw new Error("the pack frame ends inside a record");
    this.at += 1;
    return value;
  }

  varint(): number {
    let value = 0;
    let shift = 0;
    for (;;) {
      const byte = this.byte();
      value += (byte & 0x7f) * 2 ** shift;
      if ((byte & 0x80) === 0) return value;
      shift += 7;
    }
  }

  bytes(length: number): Uint8Array {
    const slice = this.data.slice(this.at, this.at + length);
    if (slice.length !== length) throw new Error("the pack frame ends inside a length-prefixed run");
    this.at += length;
    return slice;
  }

  blob(): Uint8Array {
    return this.bytes(this.varint());
  }

  text(): string {
    return new TextDecoder("utf-8", { fatal: true }).decode(this.blob());
  }

  f64(): number {
    const view = new DataView(this.bytes(8).buffer);
    return view.getFloat64(0, true);
  }

  f32(): number {
    const view = new DataView(this.bytes(4).buffer);
    return view.getFloat32(0, true);
  }

  u32(): number {
    const view = new DataView(this.bytes(4).buffer);
    return view.getUint32(0, true);
  }
}

class Writer {
  private readonly chunks: number[] = [];

  byte(value: number): void {
    this.chunks.push(value & 0xff);
  }

  varint(value: number): void {
    let rest = value;
    for (;;) {
      const byte = rest % 128;
      rest = Math.floor(rest / 128);
      if (rest > 0) {
        this.byte(byte | 0x80);
        continue;
      }
      this.byte(byte);
      return;
    }
  }

  raw(bytes: Uint8Array | number[]): void {
    for (const byte of bytes) this.chunks.push(byte & 0xff);
  }

  blob(bytes: Uint8Array | number[]): void {
    const payload = bytes instanceof Uint8Array ? bytes : Uint8Array.from(bytes);
    this.varint(payload.length);
    this.raw(payload);
  }

  text(value: string): void {
    this.blob(new TextEncoder().encode(value));
  }

  f64(value: number): void {
    const buffer = new ArrayBuffer(8);
    new DataView(buffer).setFloat64(0, value, true);
    this.raw(new Uint8Array(buffer));
  }

  f32(value: number): void {
    const buffer = new ArrayBuffer(4);
    new DataView(buffer).setFloat32(0, value, true);
    this.raw(new Uint8Array(buffer));
  }

  u32(value: number): void {
    const buffer = new ArrayBuffer(4);
    new DataView(buffer).setUint32(0, value, true);
    this.raw(new Uint8Array(buffer));
  }

  finish(): Uint8Array {
    return Uint8Array.from(this.chunks);
  }
}

/** 🎒️ The committed binary envelope and the pack frame the Kaitai mirror describes. */
export function parsePack(input: Uint8Array): Snapshot {
  // 🧷️ A host hands these bytes over as a Node `Buffer`, whose `slice` is a VIEW into a shared pool
  // rather than a copy — reading a `DataView` off `.buffer` would then read from the pool's start.
  // One copy up front makes every offset below mean what it says.
  const data = Uint8Array.from(input);
  for (let at = 0; at < BINARY_MAGIC.length; at += 1) if (data[at] !== BINARY_MAGIC[at]) throw new Error("the binary envelope magic is not the semio magic");
  const tokenLength = new DataView(data.slice(8, 12).buffer).getUint32(0, true);
  const token = new TextDecoder().decode(data.slice(12, 12 + tokenLength));
  if (token !== PACK_TOKEN) throw new Error(`the binary envelope token is ${token}, expected ${PACK_TOKEN}`);
  const reader = new Reader(data, 12 + tokenLength);
  const format = reader.byte();
  if (format !== PACK_FORMAT) throw new Error(`the pack format byte is ${format}, expected ${PACK_FORMAT}`);
  const schema = reader.text();
  const meshes: Mesh[] = [];
  for (let mesh = reader.varint(); mesh > 0; mesh -= 1) {
    const id = reader.text();
    const primitives: Primitive[] = [];
    for (let count = reader.varint(); count > 0; count -= 1) {
      const primitiveId = reader.text();
      const tag = reader.byte();
      const topology = TOPOLOGY_ORDER[tag];
      if (topology === undefined) throw new Error(`the pack topology tag ${tag} is outside the declared enumeration`);
      const positions: Point3[] = [];
      for (let entry = reader.varint(); entry > 0; entry -= 1) positions.push({ x: reader.f64(), y: reader.f64(), z: reader.f64() });
      const normals: Point3[] = [];
      for (let entry = reader.varint(); entry > 0; entry -= 1) normals.push({ x: reader.f64(), y: reader.f64(), z: reader.f64() });
      const uvs: Uv[] = [];
      for (let entry = reader.varint(); entry > 0; entry -= 1) uvs.push({ u: reader.f64(), v: reader.f64() });
      const colors: Rgba[] = [];
      for (let entry = reader.varint(); entry > 0; entry -= 1) colors.push({ r: reader.f32(), g: reader.f32(), b: reader.f32(), a: reader.f32() });
      const indices: number[] = [];
      for (let entry = reader.varint(); entry > 0; entry -= 1) indices.push(reader.u32());
      const present = reader.byte();
      if (present !== 0 && present !== 1) throw new Error(`the material-id presence byte is ${present}, expected 0 or 1`);
      primitives.push({ id: primitiveId, topology, positions, normals, uvs, colors, indices, materialId: present === 1 ? reader.text() : null });
    }
    meshes.push({ id, primitives });
  }
  const materials: Material[] = [];
  for (let count = reader.varint(); count > 0; count -= 1) materials.push({ id: reader.text(), baseColor: { r: reader.f32(), g: reader.f32(), b: reader.f32(), a: reader.f32() }, metallic: reader.f32(), roughness: reader.f32() });
  const textures: Texture[] = [];
  for (let count = reader.varint(); count > 0; count -= 1) textures.push({ id: reader.text(), mime: reader.text(), bytes: Array.from(reader.blob()) });
  if (!reader.done) throw new Error(`the pack frame ends ${data.length - reader.offset} bytes before its envelope does`);
  return { schema, meshes, materials, textures };
}

/** 🎒️ The pack frame in the writing direction, inside the shared binary envelope. */
export function packBytes(document: Snapshot): Uint8Array {
  const body = new Writer();
  body.byte(PACK_FORMAT);
  body.text(document.schema);
  body.varint(document.meshes.length);
  for (const mesh of document.meshes) {
    body.text(mesh.id);
    body.varint(mesh.primitives.length);
    for (const primitive of mesh.primitives) {
      body.text(primitive.id);
      body.byte(TOPOLOGY_ORDER.indexOf(primitive.topology as (typeof TOPOLOGY_ORDER)[number]));
      body.varint(primitive.positions.length);
      for (const point of primitive.positions) {
        body.f64(point.x);
        body.f64(point.y);
        body.f64(point.z);
      }
      body.varint(primitive.normals.length);
      for (const point of primitive.normals) {
        body.f64(point.x);
        body.f64(point.y);
        body.f64(point.z);
      }
      body.varint(primitive.uvs.length);
      for (const uv of primitive.uvs) {
        body.f64(uv.u);
        body.f64(uv.v);
      }
      body.varint(primitive.colors.length);
      for (const colour of primitive.colors) {
        body.f32(colour.r);
        body.f32(colour.g);
        body.f32(colour.b);
        body.f32(colour.a);
      }
      body.varint(primitive.indices.length);
      for (const index of primitive.indices) body.u32(index);
      if (primitive.materialId === null) body.byte(0);
      else {
        body.byte(1);
        body.text(primitive.materialId);
      }
    }
  }
  body.varint(document.materials.length);
  for (const material of document.materials) {
    body.text(material.id);
    body.f32(material.baseColor.r);
    body.f32(material.baseColor.g);
    body.f32(material.baseColor.b);
    body.f32(material.baseColor.a);
    body.f32(material.metallic);
    body.f32(material.roughness);
  }
  body.varint(document.textures.length);
  for (const texture of document.textures) {
    body.text(texture.id);
    body.text(texture.mime);
    body.blob(texture.bytes);
  }
  const envelope = new Writer();
  const token = new TextEncoder().encode(PACK_TOKEN);
  envelope.raw(BINARY_MAGIC);
  envelope.u32(token.length);
  envelope.raw(token);
  envelope.raw(body.finish());
  return envelope.finish();
}
// #endregion 🎒️Pack

// #region 🧬️Mutations
function clone<T>(value: T): T {
  return JSON.parse(JSON.stringify(value)) as T;
}

/** 🏷️ The externally tagged wire form the committed vectors use: one key, the variant's own name in
 * `PascalCase`, whose value carries the verb's `snake_case` arguments. */
function verbOf(mutation: Mutation): [string, Record<string, unknown>] {
  const keys = Object.keys(mutation);
  if (keys.length !== 1) throw new Error(`a mutation payload carries exactly one variant key, found ${keys.length}`);
  return [keys[0]!, mutation[keys[0]!] as Record<string, unknown>];
}

function kebab(verb: string): string {
  return verb.replace(/([a-z0-9])([A-Z])/g, "$1-$2").toLowerCase();
}

function meshOf(document: Snapshot, id: string, verb: string): Mesh {
  const found = document.meshes.find((mesh) => mesh.id === id);
  if (found === undefined) throw new Error(`${verb} addresses the mesh ${id}, which the document does not carry`);
  return found;
}

function primitiveOf(document: Snapshot, meshId: string, primitiveId: string, verb: string): Primitive {
  const found = meshOf(document, meshId, verb).primitives.find((primitive) => primitive.id === primitiveId);
  if (found === undefined) throw new Error(`${verb} addresses the primitive ${primitiveId}, which ${meshId} does not carry`);
  return found;
}

function materialOf(document: Snapshot, id: string, verb: string): Material {
  const found = document.materials.find((material) => material.id === id);
  if (found === undefined) throw new Error(`${verb} addresses the material ${id}, which the document does not carry`);
  return found;
}

function textureOf(document: Snapshot, id: string, verb: string): Texture {
  const found = document.textures.find((texture) => texture.id === id);
  if (found === undefined) throw new Error(`${verb} addresses the texture ${id}, which the document does not carry`);
  return found;
}

/** 🔑️ An id-keyed pool's `create` arm: replace the member with this id in place, or append it. */
function upsert<T extends { id: string }>(pool: T[], entry: T): void {
  const at = pool.findIndex((member) => member.id === entry.id);
  if (at === -1) pool.push(entry);
  else pool[at] = entry;
}

/**
 * ▶️ One verb applied to a document, returning the resulting document.
 *
 * Each arm is the behaviour its committed `(before, mutation, after)` specification vector states:
 * the four `create-` verbs append an unknown id to their own pool and replace a known one in place,
 * the four `delete-` verbs remove by id from that pool alone, the primitive verbs address one
 * primitive inside one mesh by the two ids together, and `move-vertex` reaches into one position
 * array by index and leaves every parallel attribute array it is not addressing untouched.
 */
export function applyMutation(document: Snapshot, mutation: Mutation): Snapshot {
  const [verb, argument] = verbOf(mutation);
  const result = clone(document);
  switch (verb) {
    case "CreateMesh":
      upsert(result.meshes, clone(argument.mesh as Mesh));
      return result;
    case "DeleteMesh":
      meshOf(result, argument.id as string, "delete-mesh");
      result.meshes = result.meshes.filter((mesh) => mesh.id !== argument.id);
      return result;
    case "CreatePrimitive":
      upsert(meshOf(result, argument.mesh_id as string, "create-primitive").primitives, clone(argument.primitive as Primitive));
      return result;
    case "DeletePrimitive": {
      const mesh = meshOf(result, argument.mesh_id as string, "delete-primitive");
      primitiveOf(result, argument.mesh_id as string, argument.primitive_id as string, "delete-primitive");
      mesh.primitives = mesh.primitives.filter((primitive) => primitive.id !== argument.primitive_id);
      return result;
    }
    case "SetPrimitiveTopology": {
      const topology = argument.topology as string;
      if (!TOPOLOGY_ORDER.includes(topology as (typeof TOPOLOGY_ORDER)[number])) throw new Error(`${topology} is not one of the six declared topologies`);
      primitiveOf(result, argument.mesh_id as string, argument.primitive_id as string, "set-primitive-topology").topology = topology;
      return result;
    }
    case "ReplacePrimitiveGeometry": {
      const primitive = primitiveOf(result, argument.mesh_id as string, argument.primitive_id as string, "replace-primitive-geometry");
      primitive.positions = clone(argument.positions as Point3[]);
      primitive.normals = clone(argument.normals as Point3[]);
      primitive.uvs = clone(argument.uvs as Uv[]);
      primitive.colors = (clone(argument.colors as Rgba[]) ?? []).map((colour) => ({ r: Math.fround(colour.r), g: Math.fround(colour.g), b: Math.fround(colour.b), a: Math.fround(colour.a) }));
      primitive.indices = clone(argument.indices as number[]);
      return result;
    }
    case "SetPrimitiveMaterial":
      primitiveOf(result, argument.mesh_id as string, argument.primitive_id as string, "set-primitive-material").materialId = (argument.material_id as string | null) ?? null;
      return result;
    case "CreateMaterial": {
      const material = clone(argument.material as Material);
      upsert(result.materials, { id: material.id, baseColor: { r: Math.fround(material.baseColor.r), g: Math.fround(material.baseColor.g), b: Math.fround(material.baseColor.b), a: Math.fround(material.baseColor.a) }, metallic: Math.fround(material.metallic), roughness: Math.fround(material.roughness) });
      return result;
    }
    case "DeleteMaterial":
      materialOf(result, argument.id as string, "delete-material");
      result.materials = result.materials.filter((material) => material.id !== argument.id);
      return result;
    case "ChangeMaterialBaseColor": {
      const colour = argument.new_base_color as Rgba;
      materialOf(result, argument.id as string, "change-material-base-color").baseColor = { r: Math.fround(colour.r), g: Math.fround(colour.g), b: Math.fround(colour.b), a: Math.fround(colour.a) };
      return result;
    }
    case "ChangeMaterialMetallic":
      materialOf(result, argument.id as string, "change-material-metallic").metallic = Math.fround(argument.new_metallic as number);
      return result;
    case "ChangeMaterialRoughness":
      materialOf(result, argument.id as string, "change-material-roughness").roughness = Math.fround(argument.new_roughness as number);
      return result;
    case "CreateTexture":
      upsert(result.textures, clone(argument.texture as Texture));
      return result;
    case "DeleteTexture":
      textureOf(result, argument.id as string, "delete-texture");
      result.textures = result.textures.filter((texture) => texture.id !== argument.id);
      return result;
    case "ChangeTextureMime":
      textureOf(result, argument.id as string, "change-texture-mime").mime = argument.new_mime as string;
      return result;
    case "ReplaceTextureBytes":
      textureOf(result, argument.id as string, "replace-texture-bytes").bytes = clone(argument.new_bytes as number[]);
      return result;
    case "MoveVertex": {
      const primitive = primitiveOf(result, argument.mesh_id as string, argument.primitive_id as string, "move-vertex");
      const index = argument.vertex_index as number;
      if (index < 0 || index >= primitive.positions.length) throw new Error(`move-vertex addresses vertex ${index} of a ${primitive.positions.length}-vertex primitive`);
      primitive.positions[index] = clone(argument.new_point as Point3);
      return result;
    }
    default:
      throw new Error(`${verb} is not one of this subset's seventeen declared verbs`);
  }
}

/**
 * ↩️ The verb's own inverse against the document it is about to be applied to, as the ORDERED
 * sequence of verbs that restores it.
 *
 * Every `create-` pool is append-only and carries no index, so undoing the removal of a member that
 * was not last cannot be one verb: the sequence lifts the whole tail off and re-declares it in
 * order, the same repair `mutate-obj-3-0`'s `restore_face_at` records for its own membership lists.
 */
export function inverseMutation(document: Snapshot, mutation: Mutation): Mutation[] {
  const [verb, argument] = verbOf(mutation);
  const restorePool = <T extends { id: string }>(pool: T[], removedId: string, create: (entry: T) => Mutation, remove: (id: string) => Mutation): Mutation[] => {
    const at = pool.findIndex((member) => member.id === removedId);
    if (at === -1) throw new Error(`${kebab(verb)} addresses ${removedId}, which the document does not carry`);
    const tail = pool.slice(at);
    return [...tail.slice(1).map((member) => remove(member.id)), ...tail.map((member) => create(clone(member)))];
  };
  switch (verb) {
    case "CreateMesh": {
      const previous = document.meshes.find((mesh) => mesh.id === (argument.mesh as Mesh).id);
      return previous === undefined ? [{ DeleteMesh: { id: (argument.mesh as Mesh).id } }] : [{ CreateMesh: { mesh: clone(previous) } }];
    }
    case "DeleteMesh":
      return restorePool(
        document.meshes,
        argument.id as string,
        (mesh) => ({ CreateMesh: { mesh } }),
        (id) => ({ DeleteMesh: { id } }),
      );
    case "CreatePrimitive": {
      const mesh = meshOf(document, argument.mesh_id as string, "create-primitive");
      const previous = mesh.primitives.find((primitive) => primitive.id === (argument.primitive as Primitive).id);
      return previous === undefined
        ? [{ DeletePrimitive: { mesh_id: mesh.id, primitive_id: (argument.primitive as Primitive).id } }]
        : [{ CreatePrimitive: { mesh_id: mesh.id, primitive: clone(previous) } }];
    }
    case "DeletePrimitive": {
      const mesh = meshOf(document, argument.mesh_id as string, "delete-primitive");
      return restorePool(
        mesh.primitives,
        argument.primitive_id as string,
        (primitive) => ({ CreatePrimitive: { mesh_id: mesh.id, primitive } }),
        (id) => ({ DeletePrimitive: { mesh_id: mesh.id, primitive_id: id } }),
      );
    }
    case "SetPrimitiveTopology": {
      const primitive = primitiveOf(document, argument.mesh_id as string, argument.primitive_id as string, "set-primitive-topology");
      return [{ SetPrimitiveTopology: { mesh_id: argument.mesh_id, primitive_id: argument.primitive_id, topology: primitive.topology } }];
    }
    case "ReplacePrimitiveGeometry": {
      const primitive = primitiveOf(document, argument.mesh_id as string, argument.primitive_id as string, "replace-primitive-geometry");
      return [
        {
          ReplacePrimitiveGeometry: {
            mesh_id: argument.mesh_id,
            primitive_id: argument.primitive_id,
            positions: clone(primitive.positions),
            normals: clone(primitive.normals),
            uvs: clone(primitive.uvs),
            colors: clone(primitive.colors),
            indices: clone(primitive.indices),
          },
        },
      ];
    }
    case "SetPrimitiveMaterial": {
      const primitive = primitiveOf(document, argument.mesh_id as string, argument.primitive_id as string, "set-primitive-material");
      return [{ SetPrimitiveMaterial: { mesh_id: argument.mesh_id, primitive_id: argument.primitive_id, material_id: primitive.materialId } }];
    }
    case "CreateMaterial": {
      const previous = document.materials.find((material) => material.id === (argument.material as Material).id);
      return previous === undefined ? [{ DeleteMaterial: { id: (argument.material as Material).id } }] : [{ CreateMaterial: { material: clone(previous) } }];
    }
    case "DeleteMaterial":
      return restorePool(
        document.materials,
        argument.id as string,
        (material) => ({ CreateMaterial: { material } }),
        (id) => ({ DeleteMaterial: { id } }),
      );
    case "ChangeMaterialBaseColor":
      return [{ ChangeMaterialBaseColor: { id: argument.id, new_base_color: clone(materialOf(document, argument.id as string, "change-material-base-color").baseColor) } }];
    case "ChangeMaterialMetallic":
      return [{ ChangeMaterialMetallic: { id: argument.id, new_metallic: materialOf(document, argument.id as string, "change-material-metallic").metallic } }];
    case "ChangeMaterialRoughness":
      return [{ ChangeMaterialRoughness: { id: argument.id, new_roughness: materialOf(document, argument.id as string, "change-material-roughness").roughness } }];
    case "CreateTexture": {
      const previous = document.textures.find((texture) => texture.id === (argument.texture as Texture).id);
      return previous === undefined ? [{ DeleteTexture: { id: (argument.texture as Texture).id } }] : [{ CreateTexture: { texture: clone(previous) } }];
    }
    case "DeleteTexture":
      return restorePool(
        document.textures,
        argument.id as string,
        (texture) => ({ CreateTexture: { texture } }),
        (id) => ({ DeleteTexture: { id } }),
      );
    case "ChangeTextureMime":
      return [{ ChangeTextureMime: { id: argument.id, new_mime: textureOf(document, argument.id as string, "change-texture-mime").mime } }];
    case "ReplaceTextureBytes":
      return [{ ReplaceTextureBytes: { id: argument.id, new_bytes: clone(textureOf(document, argument.id as string, "replace-texture-bytes").bytes) } }];
    case "MoveVertex": {
      const primitive = primitiveOf(document, argument.mesh_id as string, argument.primitive_id as string, "move-vertex");
      const index = argument.vertex_index as number;
      const point = primitive.positions[index];
      if (point === undefined) throw new Error(`move-vertex addresses vertex ${index} of a ${primitive.positions.length}-vertex primitive`);
      return [{ MoveVertex: { mesh_id: argument.mesh_id, primitive_id: argument.primitive_id, vertex_index: index, new_point: clone(point) } }];
    }
    default:
      throw new Error(`${verb} is not one of this subset's seventeen declared verbs`);
  }
}
// #endregion 🧬️Mutations

// #region 🔺️Three
/**
 * 🔺️ What the third-party 3D engine states about the primitives this document carries.
 *
 * Every primitive becomes a real `THREE.BufferGeometry` — positions in a `Float64Array`, so nothing
 * is quantised on the way in — and three.js reports its attribute counts, the bounding box `Box3`
 * computes from the position attribute, and the flat vertex stream `toNonIndexed()` produces by
 * walking the index buffer. An index that points outside the position array cannot be expanded at
 * all, which is how a broken index buffer fails here rather than passing quietly.
 */
export function threeReport(document: Snapshot): unknown {
  const primitives: unknown[] = [];
  for (const mesh of document.meshes) {
    for (const primitive of mesh.primitives) {
      const geometry = new THREE.BufferGeometry();
      const positions = new Float64Array(primitive.positions.flatMap((point) => [point.x, point.y, point.z]));
      geometry.setAttribute("position", new THREE.BufferAttribute(positions, 3));
      if (primitive.normals.length > 0) geometry.setAttribute("normal", new THREE.BufferAttribute(new Float64Array(primitive.normals.flatMap((point) => [point.x, point.y, point.z])), 3));
      if (primitive.uvs.length > 0) geometry.setAttribute("uv", new THREE.BufferAttribute(new Float64Array(primitive.uvs.flatMap((uv) => [uv.u, uv.v])), 2));
      if (primitive.colors.length > 0) geometry.setAttribute("color", new THREE.BufferAttribute(new Float32Array(primitive.colors.flatMap((colour) => [colour.r, colour.g, colour.b, colour.a])), 4));
      const addressable = primitive.indices.length > 0 && primitive.indices.every((index) => index < primitive.positions.length);
      if (addressable) geometry.setIndex(new THREE.BufferAttribute(Uint32Array.from(primitive.indices), 1));
      geometry.computeBoundingBox();
      const box = primitive.positions.length === 0 || geometry.boundingBox === null ? null : { min: [geometry.boundingBox.min.x, geometry.boundingBox.min.y, geometry.boundingBox.min.z], max: [geometry.boundingBox.max.x, geometry.boundingBox.max.y, geometry.boundingBox.max.z] };
      const expanded = addressable ? Array.from(geometry.toNonIndexed().attributes.position!.array as Float64Array) : null;
      primitives.push({
        meshId: mesh.id,
        primitiveId: primitive.id,
        topology: primitive.topology,
        counts: { positions: primitive.positions.length, normals: primitive.normals.length, uvs: primitive.uvs.length, colors: primitive.colors.length, indices: primitive.indices.length },
        addressable,
        boundingBox: box,
        nonIndexedPositions: expanded,
      });
    }
  }
  return { library: "three.js", primitives };
}

/**
 * 🎯️ The projection every scenario compares under `ordered-json-v1` — the snapshot's own structural
 * JSON shape.
 *
 * `SemioRgba`'s four channels and a material's `metallic`/`roughness` are SINGLE precision, and the
 * reference's JSON wire form spells such a leaf with the shortest decimal that round-trips as an
 * `f32` (`0.1`), not with the widened double a JavaScript number would print
 * (`0.10000000149011612`) — which is exactly how the committed specification vectors spell them.
 * Every single-precision leaf therefore goes out through `printF32` and back, so the two languages
 * compare the same number rather than the same bit pattern printed two ways.
 */
export function projectionOf(document: Snapshot): Snapshot {
  const rgba = (colour: Rgba): Rgba => ({ r: Number(printF32(colour.r)), g: Number(printF32(colour.g)), b: Number(printF32(colour.b)), a: Number(printF32(colour.a)) });
  return {
    schema: document.schema,
    meshes: document.meshes.map((mesh) => ({ id: mesh.id, primitives: mesh.primitives.map((primitive) => ({ ...clone(primitive), colors: primitive.colors.map(rgba) })) })),
    materials: document.materials.map((material) => ({ id: material.id, baseColor: rgba(material.baseColor), metallic: Number(printF32(material.metallic)), roughness: Number(printF32(material.roughness)) })),
    textures: clone(document.textures),
  };
}
// #endregion 🔺️Three

// #region 🧫️Scenario input
function stepUris(ctx: AdapterContext, scheme: string): string[] {
  const found: string[] = [];
  for (const step of ctx.scenario.steps) {
    const cells = [step.text, ...(step.dataTable ?? []).flat()];
    for (const cell of cells) for (const token of cell.split(/\s+/)) if (token.startsWith(scheme)) found.push(token);
  }
  return found;
}

function docString(ctx: AdapterContext): string {
  const found = ctx.scenario.steps.find((step) => step.docString !== undefined)?.docString;
  if (found === undefined) throw new Error(`${ctx.scenario.id} declares no doc string`);
  return found;
}

function fixtureJson(ctx: AdapterContext, uri: string): unknown {
  return JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(uri)));
}

function artifact(ctx: AdapterContext): Snapshot {
  return parseDsl(new TextDecoder().decode(ctx.fixtureBytes(ARTIFACT_DSL)));
}

function assertion(message: string): Error {
  const error = new Error(message);
  error.name = "AssertionError";
  return error;
}
// #endregion 🧫️Scenario input

// #region 🎯️Handlers
function mutate(ctx: AdapterContext): AdapterOutcome {
  const document = artifact(ctx);
  const mutation = fixtureJson(ctx, stepUris(ctx, "local://🦠️")[0]!) as Mutation;
  const applied = applyMutation(document, mutation);
  return { projection: { document: projectionOf(applied), geometry: threeReport(applied) } };
}

function inverse(ctx: AdapterContext): AdapterOutcome {
  const document = artifact(ctx);
  const mutation = fixtureJson(ctx, stepUris(ctx, "local://🦠️")[0]!) as Mutation;
  const undo = inverseMutation(document, mutation);
  const mutated = applyMutation(document, mutation);
  let restored = mutated;
  for (const step of undo) restored = applyMutation(restored, step);
  if (JSON.stringify(restored) !== JSON.stringify(document)) throw assertion(`${ctx.scenario.id}: undoing the mutation did not restore the model`);
  return { projection: { mutated: projectionOf(mutated), restored: projectionOf(restored) } };
}

function specVector(ctx: AdapterContext): AdapterOutcome {
  const uris = stepUris(ctx, "asset://");
  const before = fixtureJson(ctx, uris[0]!) as Snapshot;
  const mutation = fixtureJson(ctx, uris[1]!) as Mutation;
  const expected = fixtureJson(ctx, uris[2]!) as Snapshot;
  const applied = applyMutation(before, mutation);
  if (JSON.stringify(applied) !== JSON.stringify(expected)) throw assertion(`${ctx.scenario.id}: the applied snapshot is not the committed after-snapshot`);
  return { projection: projectionOf(applied) };
}

function identityRoundTrip(ctx: AdapterContext): AdapterOutcome {
  const dslBytes = ctx.fixtureBytes(ARTIFACT_DSL);
  const parsed = parseDsl(new TextDecoder().decode(dslBytes));
  const printed = new TextEncoder().encode(printDsl(parsed));
  if (digest(printed) !== digest(dslBytes)) throw assertion("identity-round-trip: re-printing the parsed model did not reproduce the committed DSL file");
  const packBuffer = ctx.fixtureBytes(ARTIFACT_PACK);
  const unpacked = parsePack(packBuffer);
  if (JSON.stringify(unpacked) !== JSON.stringify(parsed)) throw assertion("identity-round-trip: the committed binary twin decodes to a different model than the committed text artifact");
  const repacked = packBytes(parsed);
  if (digest(repacked) !== digest(packBuffer)) throw assertion("identity-round-trip: re-encoding the parsed model did not reproduce the committed pack file");
  return {
    projection: {
      document: projectionOf(parsed),
      geometry: threeReport(parsed),
      dslDigest: digest(printed),
      packDigest: digest(repacked),
      dslLength: printed.length,
      packLength: repacked.length,
    },
  };
}
// #endregion 🎯️Handlers

// #region 🧭️Adapter
/** 🧭️ Registration by FULL expanded scenario id, mirroring the feature's `Examples` tables. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    ...Object.fromEntries(KINDS.flatMap((kind) => [[`mutate-${kind}`, { oracle: mutate }], [`inverse-${kind}`, { oracle: inverse }], [`spec-vector-${kind}`, { oracle: specVector }]])),
    "identity-round-trip": { oracle: identityRoundTrip },
  },
});
// #endregion 🧭️Adapter
