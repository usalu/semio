// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

/**
 * 🟦️ Independent TypeScript implementation of the `stdio.semio.mesh` carrier and its eighteen-verb
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
 * - the DSL body from `../../🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/📝️text/📸️snapshot/📖️.grammar.semio`
 *   (`document = artifact-mark schema-line meshes-line materials-line textures-line`, the six
 *   `topology` letters, `option-hex`, and plain bracketed number lists for the geometry buffers);
 * - the pack body from the shared record-document codec (`🧰️semio/🚪️io/💾️binary/🎒️record-document/🟦️.ts`), driven by the committed record spec
 *   `…/🧬️schema/📸️snapshot/🗺️record-spec/🔣️.json`: the SPK container, the fixed-Huffman deflate, the BLAKE3 content hash and the schema-hashed record body
 *   are specified by `🧰️framework/🔨️modules/🎒️pack` and re-implemented there, not here. The committed `🎒️.pack.semio` files were written by the Rust
 *   record pack, so `parsePack`/`packBytes` reproducing them is a cross-language byte agreement;
 * - the eighteen verbs, their argument lists and their JSON wire form from
 *   `…/🚪️io/📝️text/🧬️mutations/📖️.grammar.semio`, the committed proto and JSON schema
 *   mirrors, and the committed per-kind `(before, mutation, after)` specification vectors.
 *
 * Nothing here imports, links, wraps or transliterates the Rust subject. Where the two disagree the
 * disagreement is a finding, not something to tune away.
 */

// #region 🔌️Adapters
import * as THREE from "three";
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "@semio-tech/framework/test/adapter";
import { sha256Hex } from "@semio-tech/framework-hash";
import { decodePack, encodePack, loadRecordSpec, type RecordSpecDocument } from "../../../../../../🚪️io/💾️binary/🎒️record-document/🟦️.ts";
import recordSpecDocument from "../../🧬️schema/📸️snapshot/🗺️record-spec/🔣️.json";

/** #⃣ Stable content digest of a text or byte string: the leading 32 hex digits of its sha256. */
const digest = (input: string | Uint8Array): string => sha256Hex(typeof input === "string" ? new TextEncoder().encode(input) : input).slice(0, 32);
// #endregion 🔌️Adapters

// #region 🧬️Model
type Point3 = { x: number; y: number; z: number };
type Uv = { u: number; v: number };
type Rgba = { r: number; g: number; b: number; a: number };
type Primitive = { id: string; topology: string; positions: Point3[]; normals: Point3[]; uvs: Uv[]; colors: Rgba[]; indices: number[]; materialId: string | null };
type Mesh = { id: string; primitives: Primitive[] };
type Material = { id: string; baseColor: Rgba; metallic: number; roughness: number; baseColorTexture: string | null; metallicRoughnessTexture: string | null; normalTexture: string | null; occlusionTexture: string | null; emissiveTexture: string | null; };
type Texture = { id: string; mime: string; bytes: number[] };
type Snapshot = { schema: string; meshes: Mesh[]; materials: Material[]; textures: Texture[] };
type Mutation = Record<string, Record<string, unknown>>;

const DSL_PREAMBLE = "semio stdio.semio.mesh.dsl v1";

/** 🔺️ `topology = "P" | "L" | "S" | "T" | "X" | "F"`, the six declared topologies in the grammar's own order. */
const TOPOLOGY_ORDER = ["points", "lines", "lineStrip", "triangles", "triangleStrip", "triangleFan"] as const;
const TOPOLOGY_LETTER: Record<string, string> = { points: "P", lines: "L", lineStrip: "S", triangles: "T", triangleStrip: "X", triangleFan: "F" };
const LETTER_TOPOLOGY: Record<string, string> = Object.fromEntries(Object.entries(TOPOLOGY_LETTER).map(([kind, letter]) => [letter, kind]));

const ARTIFACT_DSL = "shared://🔺️mutate-semio-mesh/🗣️.dsl.semio";
const ARTIFACT_PACK = "shared://🔺️mutate-semio-mesh/🎒️.pack.semio";
// #endregion 🧬️Model

// #region 🔡️Leaves
/** 🔡️ The grammar's built-in `hex` macro in the writing direction, for a string leaf. */
function hexOfText(text: string): string {
  return Array.from(new TextEncoder().encode(text))
    .map((byte) => byte.toString(16).padStart(2, "0"))
    .join("");
}

/** 🪄️ The grammar's `hex` macro in the reading direction, for a string leaf. */
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

/** 🍃️ One `f64` leaf, printed the way the reference printer prints it: the shortest decimal that
 * reads back as the same double, with no exponent and no trailing `.0`. */
function printF64(value: number): string {
  return plainDecimal(String(value));
}

/** 🧱️ One `f32` leaf. `SemioRgba` and a material's `metallic`/`roughness` are single precision, so
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
  if (parts.length !== 9) throw new Error(`a material carries nine leaves, found ${parts.length}`);
  return { id: textOfHex(parts[0]!), baseColor: parseRgba(parts[1]!), metallic: Math.fround(Number(parts[2])), roughness: Math.fround(Number(parts[3])), baseColorTexture: parseOptionHex(parts[4]!, "texture"), metallicRoughnessTexture: parseOptionHex(parts[5]!, "texture"), normalTexture: parseOptionHex(parts[6]!, "texture"), occlusionTexture: parseOptionHex(parts[7]!, "texture"), emissiveTexture: parseOptionHex(parts[8]!, "texture") };
}

function printMaterial(material: Material): string {
  return `[${hexOfText(material.id)},${printRgba(material.baseColor)},${printF32(material.metallic)},${printF32(material.roughness)},${printOptionHex(material.baseColorTexture ?? null)},${printOptionHex(material.metallicRoughnessTexture ?? null)},${printOptionHex(material.normalTexture ?? null)},${printOptionHex(material.occlusionTexture ?? null)},${printOptionHex(material.emissiveTexture ?? null)}]`;
}

function parseTexture(text: string): Texture {
  const parts = itemsOf(text, "a texture");
  if (parts.length !== 3) throw new Error(`a texture carries three leaves, found ${parts.length}`);
  return { id: textOfHex(parts[0]!), mime: textOfHex(parts[1]!), bytes: Array.from(bytesOfHex(parts[2]!)) };
}

function printTexture(texture: Texture): string {
  return `[${hexOfText(texture.id)},${hexOfText(texture.mime)},${hexOfBytes(texture.bytes)}]`;
}

/** 📄️ `document = artifact-mark schema-line meshes-line materials-line textures-line`. */
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

/** 🌀️ The committed DSL grammar in the writing direction, line for line in its declared order. */
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
const RECORD_SPEC = loadRecordSpec(recordSpecDocument as unknown as RecordSpecDocument);

/** 🎒️ A `.pack.semio` file as the JSON snapshot its record document carries, decoded by the shared record-document codec from the committed record spec. */
export function parsePack(input: Uint8Array): Snapshot {
  return decodePack(RECORD_SPEC, input) as unknown as Snapshot;
}

/** 🔍️ The writing direction of `parsePack`, inside the shared binary envelope. */
export function packBytes(document: Snapshot): Uint8Array {
  return encodePack(RECORD_SPEC, document);
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

/** 🔑️ An id-keyed pool's `create` arm: replace the member with this id in place, or insert it at `at` (append when absent). */
function upsert<T extends { id: string }>(pool: T[], entry: T, at?: number): void {
  const found = pool.findIndex((member) => member.id === entry.id);
  if (found !== -1) pool[found] = entry;
  else if (at === undefined || at >= pool.length) pool.push(entry);
  else pool.splice(at, 0, entry);
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
      upsert(result.meshes, clone(argument.mesh as Mesh), argument.at as number | undefined);
      return result;
    case "DeleteMesh":
      meshOf(result, argument.id as string, "delete-mesh");
      result.meshes = result.meshes.filter((mesh) => mesh.id !== argument.id);
      return result;
    case "CreatePrimitive":
      upsert(meshOf(result, argument.mesh_id as string, "create-primitive").primitives, clone(argument.primitive as Primitive), argument.at as number | undefined);
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
      upsert(result.materials, { ...material, id: material.id, baseColor: { r: Math.fround(material.baseColor.r), g: Math.fround(material.baseColor.g), b: Math.fround(material.baseColor.b), a: Math.fround(material.baseColor.a) }, metallic: Math.fround(material.metallic), roughness: Math.fround(material.roughness), baseColorTexture: material.baseColorTexture ?? null, metallicRoughnessTexture: material.metallicRoughnessTexture ?? null, normalTexture: material.normalTexture ?? null, occlusionTexture: material.occlusionTexture ?? null, emissiveTexture: material.emissiveTexture ?? null }, argument.at as number | undefined);
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
      upsert(result.textures, clone(argument.texture as Texture), argument.at as number | undefined);
      return result;
    case "DeleteTexture":
      textureOf(result, argument.id as string, "delete-texture");
      if(result.materials.some(material=>[material.baseColorTexture,material.metallicRoughnessTexture,material.normalTexture,material.occlusionTexture,material.emissiveTexture].includes(argument.id as string)))throw Error("mutation.target-referenced");
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
      throw new Error(`${verb} is not one of this subset's eighteen declared verbs`);
  }
}

/**
 * ↩️ The verb's own inverse against the document it is about to be applied to, as the ORDERED
 * sequence of verbs that restores it.
 *
 * Every `create-` verb carries an optional `at`, so undoing the removal of a member is one verb: the
 * create that re-declares it at its original index.
 */
export function inverseMutation(document: Snapshot, mutation: Mutation): Mutation[] {
  const [verb, argument] = verbOf(mutation);
  const restorePool = <T extends { id: string }>(pool: T[], removedId: string, create: (entry: T, at: number) => Mutation): Mutation[] => {
    const at = pool.findIndex((member) => member.id === removedId);
    if (at === -1) throw new Error(`${kebab(verb)} addresses ${removedId}, which the document does not carry`);
    return [create(clone(pool[at]), at)];
  };
  switch (verb) {
    case "CreateMesh": {
      const previous = document.meshes.find((mesh) => mesh.id === (argument.mesh as Mesh).id);
      return previous === undefined ? [{ DeleteMesh: { id: (argument.mesh as Mesh).id } }] : [{ CreateMesh: { mesh: clone(previous) } }];
    }
    case "DeleteMesh":
      return restorePool(document.meshes, argument.id as string, (mesh, at) => ({ CreateMesh: { mesh, at } }));
    case "CreatePrimitive": {
      const mesh = meshOf(document, argument.mesh_id as string, "create-primitive");
      const previous = mesh.primitives.find((primitive) => primitive.id === (argument.primitive as Primitive).id);
      return previous === undefined
        ? [{ DeletePrimitive: { mesh_id: mesh.id, primitive_id: (argument.primitive as Primitive).id } }]
        : [{ CreatePrimitive: { mesh_id: mesh.id, primitive: clone(previous) } }];
    }
    case "DeletePrimitive": {
      const mesh = meshOf(document, argument.mesh_id as string, "delete-primitive");
      return restorePool(mesh.primitives, argument.primitive_id as string, (primitive, at) => ({ CreatePrimitive: { mesh_id: mesh.id, primitive, at } }));
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
      return restorePool(document.materials, argument.id as string, (material, at) => ({ CreateMaterial: { material, at } }));
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
      return restorePool(document.textures, argument.id as string, (texture, at) => ({ CreateTexture: { texture, at } }));
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
      throw new Error(`${verb} is not one of this subset's eighteen declared verbs`);
  }
}
// #endregion 🧬️Mutations

// #region 🔺️Three
/**
 * 🔄️ What the third-party 3D engine states about the primitives this document carries.
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
    materials: document.materials.map((material) => ({ ...material, id: material.id, baseColor: rgba(material.baseColor), metallic: Number(printF32(material.metallic)), roughness: Number(printF32(material.roughness)) })),
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

function snapshotJson(ctx: AdapterContext, uri: string): unknown {
  return JSON.parse(new TextDecoder().decode(ctx.inputBytes(uri)));
}

function artifact(ctx: AdapterContext): Snapshot {
  return parseDsl(new TextDecoder().decode(ctx.inputBytes(ARTIFACT_DSL)));
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
  const mutation = snapshotJson(ctx, stepUris(ctx, "shared://🔺️mutate-semio-mesh/").find((uri) => uri.endsWith("/🦠️mutation/🔣️.json"))!) as Mutation;
  const applied = applyMutation(document, mutation);
  return { projection: { document: projectionOf(applied), geometry: threeReport(applied) } };
}

function inverse(ctx: AdapterContext): AdapterOutcome {
  const document = artifact(ctx);
  const mutation = snapshotJson(ctx, stepUris(ctx, "shared://🔺️mutate-semio-mesh/").find((uri) => uri.endsWith("/🦠️mutation/🔣️.json"))!) as Mutation;
  const undo = inverseMutation(document, mutation);
  const mutated = applyMutation(document, mutation);
  let restored = mutated;
  for (const step of undo) restored = applyMutation(restored, step);
  if (JSON.stringify(restored) !== JSON.stringify(document)) throw assertion(`${ctx.scenario.id}: undoing the mutation did not restore the model`);
  return { projection: { mutated: projectionOf(mutated), restored: projectionOf(restored) } };
}

function specVector(ctx: AdapterContext): AdapterOutcome {
  const uris = stepUris(ctx, "shared://🧬️mutations/");
  const before = snapshotJson(ctx, uris[0]!) as Snapshot;
  const mutation = snapshotJson(ctx, uris[1]!) as Mutation;
  const expected = snapshotJson(ctx, uris[2]!) as Snapshot;
  const applied = applyMutation(before, mutation);
  if (JSON.stringify(applied) !== JSON.stringify(expected)) throw assertion(`${ctx.scenario.id}: the applied snapshot is not the committed after-snapshot`);
  return { projection: projectionOf(applied) };
}

function identityRoundTrip(ctx: AdapterContext): AdapterOutcome {
  const dslBytes = ctx.inputBytes(ARTIFACT_DSL);
  const parsed = parseDsl(new TextDecoder().decode(dslBytes));
  const printed = new TextEncoder().encode(printDsl(parsed));
  if (digest(printed) !== digest(dslBytes)) throw assertion("identity-round-trip: re-printing the parsed model did not reproduce the committed DSL file");
  const packBuffer = ctx.inputBytes(ARTIFACT_PACK);
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
/** 🧭️ Handlers are registered under the Scenario Outline base ids, which the host resolves for every Examples row, and plain scenarios under their own ids. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    mutate: { oracle: mutate },
    inverse: { oracle: inverse },
    "spec-vector": { oracle: specVector },
    "identity-round-trip": { oracle: identityRoundTrip },
  },
});
// #endregion 🧭️Adapter
