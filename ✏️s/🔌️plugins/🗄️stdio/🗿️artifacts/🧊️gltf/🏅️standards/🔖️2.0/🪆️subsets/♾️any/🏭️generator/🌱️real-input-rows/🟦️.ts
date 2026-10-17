/** 🧾️ glTF fixture generator: the real-input rows that edit the committed real GLB export, its container reader and writer and the output path of a row. */
import { basename, dirname, join, relative, resolve } from "node:path";
import type { Doc } from "../🏗️base-document/🟦️.ts";
import { create, createReindexed, del, remapSceneRefs } from "../✂️structural-edit/🟦️.ts";

//#region 🌱️Real-input rows
/** 🌱️ The committed real input of the artifact-root case (`../🧪️tests/🧊️mutate-gltf-2-0/🥒️.feature`): the 284 KB, 271-node
 *  metabolism export with node 1 nested under node 0, relative to this subset's `🔮️oracles` catalog. */
export const REAL_INPUT = "../🧪️tests/🧫️fixtures/🧊️mutate-gltf-2-0/🌳️base-with-nested-node/🧊️.glb";

type RealRow = { id: string; mutationId: string; family: string; notes: string; edit: (doc: Doc) => void };

/** 🔎️ The position of a value the row's edit requires to be present — a row whose precondition the real input does not
 *  meet is a generator error, never a silently different document. */
function positionOf(values: readonly unknown[] | undefined, value: unknown, what: string): number {
  const index = (values ?? []).indexOf(value);
  if (index < 0) throw new Error(`the real input has no ${what}`);
  return index;
}

/** 🌱️ One row per Examples row of the case's `@id-mutate` outline, same kind and params: each committed `➡️after.glb` is the
 *  real input with that edit applied to its JSON chunk by the generic operations every synthetic recipe uses, the BIN chunk
 *  carried byte for byte. The before is the committed real input itself, never rewritten here. */
export const REAL_ROWS: RealRow[] = [
  { id: "bind-node-child-metabolism-applied", mutationId: "bind-node-child", family: "structural", notes: "Links node 3 as node 2's first child — bind-node-child{parent:2,child:3,position:0}; node 3 keeps its scene-root entry, which the operation never touches.", edit: (doc) => {
    doc.nodes[2].children ??= [];
    create(doc, ["nodes", 2, "children"], 0, 3);
  } },
  { id: "unbind-node-child-metabolism-applied", mutationId: "unbind-node-child", family: "structural", notes: "Removes the real input's one nested link, node 1 from node 0's children — unbind-node-child{parent:0,child:1}.", edit: (doc) => del(doc, ["nodes", 0, "children"], positionOf(doc.nodes[0].children, 1, "child 1 under node 0")) },
  { id: "bind-scene-root-node-metabolism-applied", mutationId: "bind-scene-root-node", family: "structural", notes: "Adds node 1 (node 0's child) as scene 0's first root — bind-scene-root-node{scene:0,node:1,position:0}.", edit: (doc) => create(doc, ["scenes", 0, "nodes"], 0, 1) },
  { id: "unbind-scene-root-node-metabolism-applied", mutationId: "unbind-scene-root-node", family: "structural", notes: "Removes node 5 from scene 0's root list — unbind-scene-root-node{scene:0,node:5}.", edit: (doc) => del(doc, ["scenes", 0, "nodes"], positionOf(doc.scenes[0].nodes, 5, "root node 5 in scene 0")) },
  { id: "change-material-alpha-mode-metabolism-applied", mutationId: "change-material-alpha-mode", family: "material", notes: "Material 0's alphaMode: OPAQUE (absent) -> MASK — change-material-alpha-mode{material:0,alphaMode:MASK}.", edit: (doc) => {
    doc.materials[0].alphaMode = "MASK";
  } },
  { id: "change-material-double-sided-metabolism-applied", mutationId: "change-material-double-sided", family: "material", notes: "Material 0's doubleSided: false (absent) -> true — change-material-double-sided{material:0,doubleSided:true}.", edit: (doc) => {
    doc.materials[0].doubleSided = true;
  } },
  { id: "create-scene-metabolism-applied", mutationId: "create-scene", family: "structural", notes: "Inserts one empty scene at position 0 and bumps the default scene pointer 0 -> 1 so it still names the real scene — create-scene{position:0}.", edit: (doc) => createReindexed(doc, ["scenes"], 0, {}, (fn) => remapSceneRefs(doc, fn)) },
];

const GLB_MAGIC = 0x46546c67;
const GLB_JSON_CHUNK = 0x4e4f534a;
const GLB_BIN_CHUNK = 0x004e4942;

/** 📦️ Splits a glTF 2.0 binary container into its JSON document and its BIN chunk body (padding included). */
export function readGlb(bytes: Buffer): { doc: Doc; bin: Buffer | undefined } {
  if (bytes.length < 12 || bytes.readUInt32LE(0) !== GLB_MAGIC || bytes.readUInt32LE(4) !== 2 || bytes.readUInt32LE(8) !== bytes.length) throw new Error("not a glTF 2.0 GLB container");
  let doc: Doc;
  let bin: Buffer | undefined;
  for (let offset = 12; offset < bytes.length; ) {
    const length = bytes.readUInt32LE(offset);
    const type = bytes.readUInt32LE(offset + 4);
    const body = bytes.subarray(offset + 8, offset + 8 + length);
    if (type === GLB_JSON_CHUNK) doc = JSON.parse(body.toString("utf8"));
    else if (type === GLB_BIN_CHUNK) bin = body;
    offset += 8 + length;
  }
  if (doc === undefined) throw new Error("GLB container has no JSON chunk");
  return { doc, bin };
}

/** 📦️ Writes a glTF 2.0 binary container: the JSON chunk space-padded, the BIN chunk zero-padded, both to 4 bytes. */
export function writeGlb(doc: Doc, bin: Buffer | undefined): Buffer {
  const padded = (body: Buffer, fill: number): Buffer => Buffer.concat([body, Buffer.alloc((4 - (body.length % 4)) % 4, fill)]);
  const chunk = (type: number, body: Buffer): Buffer => {
    const head = Buffer.alloc(8);
    head.writeUInt32LE(body.length, 0);
    head.writeUInt32LE(type, 4);
    return Buffer.concat([head, body]);
  };
  const chunks = [chunk(GLB_JSON_CHUNK, padded(Buffer.from(JSON.stringify(doc), "utf8"), 0x20)), ...(bin === undefined ? [] : [chunk(GLB_BIN_CHUNK, padded(bin, 0))])];
  const header = Buffer.alloc(12);
  header.writeUInt32LE(GLB_MAGIC, 0);
  header.writeUInt32LE(2, 4);
  header.writeUInt32LE(12 + chunks.reduce((total, part) => total + part.length, 0), 8);
  return Buffer.concat([header, ...chunks]);
}

/** 🧭️ The one committed `➡️after.glb` of a real-input row, resolved from the current catalog: its before must be the real
 *  input itself and its after a `.glb` in the row's own directory beside it. */
export function realRowOutputPath(manifests: readonly { id: string; files: readonly { role: string; path: string }[] }[], id: string, catalogDir: string, outRoot?: string): string {
  if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(id)) throw new Error("Invalid glTF fixture identity.");
  const records = manifests.filter((record) => record.id === id);
  if (records.length !== 1 || records[0]!.files.length !== 2) throw new Error(`Expected one exact glTF fixture pair: ${id}`);
  const file = (role: string): string => {
    const files = records[0]!.files.filter((entry) => entry.role === role);
    if (files.length !== 1) throw new Error(`Expected one ${role} file for ${id}`);
    return files[0]!.path;
  };
  if (file("expected-before-glb") !== REAL_INPUT) throw new Error(`${id} must declare the real input ${REAL_INPUT} as its before`);
  const path = file("expected-after-glb");
  const target = resolve(catalogDir, path);
  const input = resolve(catalogDir, REAL_INPUT);
  if (relative(catalogDir, target).replaceAll("\\", "/") !== path || !path.endsWith("/➡️after.glb") || dirname(dirname(target)) !== dirname(dirname(input)) || dirname(target) === dirname(input)) throw new Error(`Fixture path escapes the case fixture folder: ${path}`);
  return outRoot === undefined ? target : join(outRoot, id, basename(target));
}
//#endregion 🌱️Real-input rows
