/** 🎬️ Owned binary FBX 7.4 encoder of the neutral 3D model kernel `semio.model/v1`; zero runtime dependencies, deterministic bytes (fixed creation time, ids derived from content, owned zlib).
 *
 * The neutral frame (right-handed, +Y up, +Z ahead, metres) is written unchanged with `UnitScaleFactor` 100 (FBX counts centimetres). Nodes become `Model`s (`LimbNode` for joints, `Mesh`, `Null`), meshes `Geometry` with normals and colours by polygon vertex and
 * materials by polygon, skins `Deformer` Skin with one Cluster per joint plus a bind `Pose`, the chosen animation `AnimationStack`/`Layer`/`CurveNode`/`Curve` for node TRS and material colours, mechanism joints limit properties. A node that is a skin joint and also holds rigid triangles is a `LimbNode` with a child `Mesh` model `<id>-geometry` (a loader turns a skin joint into a bone and would drop geometry attached to it).
 * FBX geometry has no named sub-parts: the solid provenance of a mesh's triangle primitives is kept in the mesh `Model`'s user property `SolidRanges` as `id:firstPolygon:polygonCount` entries joined by `;` (polygons of the written geometry, in order), with the notice `solids-as-property`.
 * `lines` primitives are skipped with a notice: FBX has no line primitive that loaders agree on (three's `FBXLoader` would triangulate a two-vertex polygon into NaNs).
 *
 * @see https://code.blender.org/2013/08/fbx-binary-file-format-specification/
 * @see https://help.autodesk.com/view/FBX/2017/ENU/
 */
import type { Animation, Channel, Material, Mesh, ModelDocument, Node, Operation, Primitive, Skin, Slug, Variant } from "../../🧬️schema/🟦️.ts";
import { CancelledError, Pace } from "../../⏱️operation/🟦️.ts";
import { compose, invert4, multiply4, qNormalize, worldMatrices } from "../../🧮️math/🟦️.ts";
import { colourCurves, rotationCurves, vectorCurves } from "./🎞️animation/🟦️.ts";
import type { Curves } from "./🎞️animation/🟦️.ts";
import { ownedCompressor } from "./🗜️deflate/🟦️.ts";
import type { Compressor } from "./🗜️deflate/🟦️.ts";
import { COLOUR_PROPERTY, materialChildren, srgbEncode } from "./🎨️material/🟦️.ts";
import { mechanismOf } from "./🦴️mechanism/🟦️.ts";
import type { Notice } from "./🦴️mechanism/🟦️.ts";
import { eulerDegrees } from "./📐️euler/🟦️.ts";
import { CREATION_TIME, FILE_ID_BYTES, double, doubles, field, flag, floats, int, ints, long, longs, node, properties70, property70, raw, serialize, text, ticks } from "./🧱️records/🟦️.ts";
import type { FbxNode } from "./🧱️records/🟦️.ts";
import * as portable from "../../🧮️math/🎯️portable/🟦️.ts";

export type { Compressor } from "./🗜️deflate/🟦️.ts";
export { ownedCompressor } from "./🗜️deflate/🟦️.ts";
export type { Notice } from "./🦴️mechanism/🟦️.ts";

/** ⚙️ What one encoding includes: an optional clip, an optional variant look, the only format. */
export type FbxOptions = { readonly animation?: Slug; readonly variant?: Slug; readonly format: "binary" };
/** 📦️ The bytes of one encoding and everything that could not be expressed in FBX. */
export type FbxResult = { readonly bytes: Uint8Array; readonly notices: readonly Notice[] };

/** 🛑️ Thrown when the operation was cancelled; no bytes are produced. A `CancelledError` like every other kernel cancellation. */
export class FbxCancelledError extends CancelledError {
  constructor() {
    super();
    this.message = "encodeFbx cancelled";
    this.name = "FbxCancelledError";
  }
}

/** 🚫️ Thrown for a document or option FBX cannot encode; `code` is stable, `message` English, `german` the German text. */
export class FbxEncodeError extends Error {
  constructor(readonly code: string, message: string, readonly german: string) {
    super(message);
    this.name = "FbxEncodeError";
  }
}

const VERSION = 7400;
const CREATOR = "semio.model fbx/v1";
const KEY_ATTRIBUTE_MAGIC = 9.419963346924634e-30;
const AXIS_NAMES = ["X", "Y", "Z"] as const;
const GROUP = { NodeAttribute: 0, Geometry: 1, Model: 2, Material: 3, Deformer: 4, Pose: 5, AnimationStack: 6, AnimationLayer: 7, AnimationCurveNode: 8, AnimationCurve: 9 } as const;

type ObjectKind = keyof typeof GROUP;

const named = (name: string, kind: string): string => `${name}\u0000\u0001${kind}`;

class Ids {
  private readonly byKey = new Map<string, number>();
  private readonly taken = new Set<number>();
  of(key: string): number {
    const known = this.byKey.get(key);
    if (known !== undefined) return known;
    let high = 0x811c9dc5;
    let low = 0x9747b28c;
    for (let index = 0; index < key.length; index++) {
      high = Math.imul(high ^ key.charCodeAt(index), 16777619) >>> 0;
      low = Math.imul(low ^ key.charCodeAt(index), 2246822519) >>> 0;
      low = (low ^ (low >>> 13)) >>> 0;
    }
    let id = (high & 0xfffff) * 4294967296 + low;
    while (id === 0 || this.taken.has(id)) id++;
    this.taken.add(id);
    this.byKey.set(key, id);
    return id;
  }
}

type Built = { readonly id: number; readonly slots: readonly Slug[] };

function fail(code: string, en: string, de: string): never {
  throw new FbxEncodeError(code, en, de);
}

function sceneInfo(document: ModelDocument): FbxNode {
  return node("SceneInfo", [text(named("GlobalInfo", "SceneInfo")), text("UserData")], [
    field("Type", text("UserData")),
    field("Version", int(100)),
    node("MetaData", [], [field("Version", int(100)), field("Title", text(document.title.en)), field("Subject", text(document.title.de)), field("Author", text("")), field("Keywords", text(document.id)), field("Revision", text("")), field("Comment", text(document.provenance?.source ?? ""))]),
    properties70(property70("DocumentUrl", "KString", "Url", "", ""), property70("SrcDocumentUrl", "KString", "Url", "", "")),
  ]);
}

function globalSettings(): FbxNode {
  return node("GlobalSettings", [], [
    field("Version", int(1000)),
    properties70(
      property70("UpAxis", "int", "Integer", "", 1),
      property70("UpAxisSign", "int", "Integer", "", 1),
      property70("FrontAxis", "int", "Integer", "", 2),
      property70("FrontAxisSign", "int", "Integer", "", 1),
      property70("CoordAxis", "int", "Integer", "", 0),
      property70("CoordAxisSign", "int", "Integer", "", 1),
      property70("OriginalUpAxis", "int", "Integer", "", 1),
      property70("OriginalUpAxisSign", "int", "Integer", "", 1),
      property70("UnitScaleFactor", "double", "Number", "", 100),
      property70("OriginalUnitScaleFactor", "double", "Number", "", 100),
      property70("AmbientColor", "ColorRGB", "Color", "", 0, 0, 0),
      property70("DefaultCamera", "KString", "", "", "Producer Perspective"),
      property70("TimeMode", "enum", "", "", 0),
    ),
  ]);
}

function flatNormal(positions: readonly number[], a: number, b: number, c: number): [number, number, number] {
  const ux = positions[b * 3] - positions[a * 3];
  const uy = positions[b * 3 + 1] - positions[a * 3 + 1];
  const uz = positions[b * 3 + 2] - positions[a * 3 + 2];
  const vx = positions[c * 3] - positions[a * 3];
  const vy = positions[c * 3 + 1] - positions[a * 3 + 1];
  const vz = positions[c * 3 + 2] - positions[a * 3 + 2];
  const x = uy * vz - uz * vy;
  const y = uz * vx - ux * vz;
  const z = ux * vy - uy * vx;
  const length = portable.hypot(x, y, z) || 1;
  return [x / length, y / length, z / length];
}

/** 🧬️ The solid provenance of triangle primitives as `id:firstPolygon:polygonCount;...` over the polygons of the merged geometry; empty when no primitive carries it. */
const solidRangesOf = (triangles: readonly Primitive[]): string => {
  const out: string[] = [];
  let polygon = 0;
  for (const primitive of triangles) {
    for (const range of primitive.solids ?? []) out.push(`${range.id}:${polygon + range.first / 3}:${range.count / 3}`);
    polygon += primitive.indices.length / 3;
  }
  return out.join(";");
};

/** 🎥️ Encodes a resolved document; returns the bytes and the notices of everything FBX could not express. */
export function encodeFbxDetailed(document: ModelDocument, options: FbxOptions, operation: Operation, compressor: Compressor = ownedCompressor): FbxResult {
  const guard = new Pace(operation);
  const check = (): void => {
    try {
      guard.check();
    } catch {
      throw new FbxCancelledError();
    }
  };
  const tick = (units = 1): void => {
    try {
      guard.tick(units);
    } catch {
      throw new FbxCancelledError();
    }
  };
  check();
  operation.report(0);
  if (options.format !== "binary") fail("unsupported-format", `Format ${String(options.format)} is not supported; only binary FBX 7.4 is written.`, `Das Format ${String(options.format)} wird nicht unterstützt; nur binäres FBX 7.4 wird geschrieben.`);
  if (document.solids.length > 0) fail("procedural-document", "The document is procedural (it carries solids); tessellate it before encoding FBX.", "Das Dokument ist prozedural (es enthält Solids); vor der FBX-Kodierung tessellieren.");
  const animation: Animation | undefined = options.animation === undefined ? undefined : document.animations.find((item) => item.id === options.animation);
  if (options.animation !== undefined && !animation) fail("unknown-animation", `Animation ${options.animation} does not exist.`, `Die Animation ${options.animation} existiert nicht.`);
  const variant: Variant | undefined = options.variant === undefined ? undefined : document.variants.find((item) => item.id === options.variant);
  if (options.variant !== undefined && !variant) fail("unknown-variant", `Variant ${options.variant} does not exist.`, `Die Variante ${options.variant} existiert nicht.`);

  const notices: Notice[] = [];
  const ids = new Ids();
  const objects: { group: number; record: FbxNode }[] = [];
  const connections: FbxNode[] = [];
  const addObject = (kind: ObjectKind, record: FbxNode): void => {
    objects.push({ group: GROUP[kind], record });
  };
  const oo = (child: number, parent: number): void => {
    connections.push(node("C", [text("OO"), long(child), long(parent)]));
  };
  const op = (child: number, parent: number, property: string): void => {
    connections.push(node("C", [text("OP"), long(child), long(parent), text(property)]));
  };

  const nodesById = new Map<string, Node>(document.nodes.map((item) => [item.id, item]));
  const meshesById = new Map<string, Mesh>(document.meshes.map((item) => [item.id, item]));
  const skinsById = new Map<string, Skin>(document.skins.map((item) => [item.id, item]));
  const materialsById = new Map<string, Material>(document.materials.map((item) => [item.id, item]));
  const position = new Map<string, number>(document.nodes.map((item, index) => [item.id, index]));
  for (const [index, item] of document.nodes.entries()) {
    const parent = item.parent === null ? undefined : position.get(item.parent);
    if (item.parent !== null && (parent === undefined || parent >= index)) fail("node-order", `node ${item.id} precedes its parent ${item.parent}`, `Der Knoten ${item.id} steht vor seinem Elternknoten ${item.parent}; Knoten müssen ihren Eltern folgen.`);
  }
  const world = worldMatrices(document.nodes, (item) => compose(item.translation, qNormalize(item.rotation), item.scale));
  const jointNodes = new Set<string>(document.skins.flatMap((skin) => skin.joints));
  const resolveMaterial = (id: string): string => variant?.bindings[id] ?? id;
  const materialIds = new Map<string, number>();
  const materialObject = (id: string): number => {
    const known = materialIds.get(id);
    if (known !== undefined) return known;
    const material = materialsById.get(id);
    if (!material) return fail("unknown-material", `Material ${id} does not exist.`, `Das Material ${id} existiert nicht.`);
    const fbxId = ids.of(`Material:${id}`);
    materialIds.set(id, fbxId);
    addObject("Material", node("Material", [long(fbxId), text(named(id, "Material")), text("")], materialChildren(material)));
    if (material.unlit) notices.push({ code: "unlit-approximated", subject: id, message: { en: `Material ${id} is unlit; FBX has no unlit shading, it is written as Lambert with its base colour as diffuse.`, de: `Das Material ${id} ist unbeleuchtet; FBX kennt kein unbeleuchtetes Shading, es wird als Lambert mit der Grundfarbe als Diffus geschrieben.` } });
    return fbxId;
  };

  const preRotations = new Map<string, readonly number[]>();
  const modelIds = new Map<string, number>();
  const geometryCache = new Map<string, Built>();
  const poseMatrices = new Map<number, readonly number[]>();
  const totalNodes = Math.max(1, document.nodes.length);

  const buildGeometry = (item: Node, mesh: Mesh, triangles: readonly Primitive[], skin: Skin | undefined): Built => {
    const key = `${mesh.id}|${skin ? item.id : ""}`;
    const cached = geometryCache.get(key);
    if (cached && !skin) return cached;
    const positions: number[] = [];
    const polygons: number[] = [];
    const normals: number[] = [];
    const materials: number[] = [];
    const slots: Slug[] = [];
    const colours: number[] = [];
    const colourIndex: number[] = [];
    const clusters = skin ? skin.joints.map(() => ({ indexes: [] as number[], weights: [] as number[] })) : [];
    let anyColour = false;
    for (const primitive of triangles) {
      check();
      const base = positions.length / 3;
      const count = primitive.positions.length / 3;
      let malformed = !Number.isInteger(count) || primitive.indices.length % 3 !== 0;
      for (let at = 0; !malformed && at < primitive.indices.length; at++) {
        tick();
        const index = primitive.indices[at]!;
        malformed = !Number.isInteger(index) || index < 0 || index >= count;
      }
      if (malformed) fail("invalid-primitive", `Mesh ${mesh.id} has a malformed triangle primitive.`, `Das Netz ${mesh.id} hat ein fehlerhaftes Dreiecks-Primitiv.`);
      for (const value of primitive.positions) {
        tick();
        positions.push(value);
      }
      const resolved = resolveMaterial(primitive.material);
      let slot = slots.indexOf(resolved);
      if (slot < 0) {
        slot = slots.length;
        slots.push(resolved);
        materialObject(resolved);
      }
      for (let vertex = 0; vertex < count; vertex++) {
        tick(4);
        if (primitive.colors) {
          anyColour = true;
          colours.push(srgbEncode(primitive.colors[vertex * 4]), srgbEncode(primitive.colors[vertex * 4 + 1]), srgbEncode(primitive.colors[vertex * 4 + 2]), primitive.colors[vertex * 4 + 3]);
        } else colours.push(1, 1, 1, 1);
        if (skin && primitive.joints && primitive.weights)
          for (let influence = 0; influence < 4; influence++) {
            const weight = primitive.weights[vertex * 4 + influence];
            const joint = primitive.joints[vertex * 4 + influence];
            if (weight > 0 && clusters[joint]) {
              clusters[joint].indexes.push(base + vertex);
              clusters[joint].weights.push(weight);
            }
          }
      }
      for (let triangle = 0; triangle < primitive.indices.length; triangle += 3) {
        tick(4);
        const corner = [primitive.indices[triangle], primitive.indices[triangle + 1], primitive.indices[triangle + 2]];
        polygons.push(base + corner[0], base + corner[1], -(base + corner[2]) - 1);
        const flat = primitive.normals ? undefined : flatNormal(primitive.positions, corner[0], corner[1], corner[2]);
        for (const vertex of corner) {
          if (flat) normals.push(...flat);
          else normals.push(primitive.normals![vertex * 3], primitive.normals![vertex * 3 + 1], primitive.normals![vertex * 3 + 2]);
          colourIndex.push(base + vertex);
        }
        materials.push(slot);
      }
    }
    const id = ids.of(`Geometry:${key}`);
    const layers: FbxNode[] = [
      node("LayerElementNormal", [int(0)], [field("Version", int(102)), field("Name", text("")), field("MappingInformationType", text("ByPolygonVertex")), field("ReferenceInformationType", text("Direct")), node("Normals", [doubles(normals)])]),
      ...(anyColour ? [node("LayerElementColor", [int(0)], [field("Version", int(101)), field("Name", text("colorSet")), field("MappingInformationType", text("ByPolygonVertex")), field("ReferenceInformationType", text("IndexToDirect")), node("Colors", [doubles(colours)]), node("ColorIndex", [ints(colourIndex)])])] : []),
      node("LayerElementMaterial", [int(0)], [field("Version", int(101)), field("Name", text("")), field("MappingInformationType", text("ByPolygon")), field("ReferenceInformationType", text("IndexToDirect")), node("Materials", [ints(materials)])]),
    ];
    const layerElements = layers.map((layer) => node("LayerElement", [], [field("Type", text(layer.name)), field("TypedIndex", int(0))]));
    addObject("Geometry", node("Geometry", [long(id), text(named(mesh.id, "Geometry")), text("Mesh")], [field("GeometryVersion", int(124)), node("Vertices", [doubles(positions)]), node("PolygonVertexIndex", [ints(polygons)]), ...layers, node("Layer", [int(0)], [field("Version", int(100)), ...layerElements])]));
    const built = { id, slots };
    if (!skin) geometryCache.set(key, built);
    if (skin) {
      const meshWorld = world.get(item.id)!;
      const skinId = ids.of(`Skin:${item.id}`);
      addObject("Deformer", node("Deformer", [long(skinId), text(named(skin.id, "Deformer")), text("Skin")], [field("Version", int(101)), field("Link_DeformAcuracy", double(50))]));
      oo(skinId, id);
      poseMatrices.set(modelIds.get(item.id)!, meshWorld);
      skin.joints.forEach((jointId, index) => {
        const joint = nodesById.get(jointId);
        if (!joint) fail("unknown-joint", `Skin ${skin.id} names the unknown node ${jointId}.`, `Der Skin ${skin.id} nennt den unbekannten Knoten ${jointId}.`);
        const link = multiply4(meshWorld, invert4(skin.inverseBind[index]));
        const clusterId = ids.of(`Cluster:${item.id}:${jointId}`);
        addObject("Deformer", node("Deformer", [long(clusterId), text(named(jointId, "SubDeformer")), text("Cluster")], [field("Version", int(100)), node("UserData", [text(""), text("")]), node("Indexes", [ints(clusters[index].indexes)]), node("Weights", [doubles(clusters[index].weights)]), node("Transform", [doubles(meshWorld)]), node("TransformLink", [doubles(link)])]));
        oo(modelIds.get(jointId)!, clusterId);
        oo(clusterId, skinId);
        if (!poseMatrices.has(modelIds.get(jointId)!)) poseMatrices.set(modelIds.get(jointId)!, link);
      });
    }
    return built;
  };

  document.nodes.forEach((item) => modelIds.set(item.id, ids.of(`Model:${item.id}`)));
  document.nodes.forEach((item, position) => {
    check();
    operation.report(0.05 + (0.35 * position) / totalNodes);
    const id = modelIds.get(item.id)!;
    const mesh = item.mesh === undefined ? undefined : meshesById.get(item.mesh);
    if (item.mesh !== undefined && !mesh) fail("unknown-mesh", `Node ${item.id} names the unknown mesh ${item.mesh}.`, `Der Knoten ${item.id} nennt das unbekannte Netz ${item.mesh}.`);
    const triangles = mesh ? mesh.primitives.filter((primitive) => primitive.mode === "triangles") : [];
    const lines = mesh ? mesh.primitives.length - triangles.length : 0;
    if (lines > 0) notices.push({ code: "lines-skipped", subject: mesh!.id, message: { en: `Mesh ${mesh!.id} has ${lines} line primitive(s); FBX has no portable line primitive, they were not written.`, de: `Das Netz ${mesh!.id} hat ${lines} Linien-Primitiv(e); FBX kennt kein portables Linien-Primitiv, sie wurden nicht geschrieben.` } });
    const isJoint = jointNodes.has(item.id) || item.joint !== undefined;
    const split = triangles.length > 0 && jointNodes.has(item.id) && item.skin === undefined;
    const kind = triangles.length > 0 && !split ? "Mesh" : isJoint ? "LimbNode" : "Null";
    const solidRanges = solidRangesOf(triangles);
    if (solidRanges !== "") notices.push({ code: "solids-as-property", subject: mesh!.id, message: { en: `Mesh ${mesh!.id} has solid provenance; FBX geometry has no named parts, the ranges are written as the user property SolidRanges (id:firstPolygon:polygonCount;...).`, de: `Das Netz ${mesh!.id} hat Solid-Herkunft; FBX-Geometrie kennt keine benannten Teile, die Bereiche stehen in der Benutzereigenschaft SolidRanges (id:ersterPolygon:Polygonzahl;...).` } });
    const mechanism = mechanismOf(item);
    notices.push(...mechanism.notices);
    const rotation = item.rotation;
    if (mechanism.preRotation) preRotations.set(item.id, qNormalize(rotation));
    const euler = eulerDegrees(rotation);
    const entries = [
      property70("InheritType", "enum", "", "", 1),
      property70("RotationOrder", "enum", "", "", 0),
      property70("DefaultAttributeIndex", "int", "Integer", "", 0),
      property70("Lcl Translation", "Lcl Translation", "", "A", ...item.translation),
      property70("Lcl Rotation", "Lcl Rotation", "", "A", ...(mechanism.preRotation ? [0, 0, 0] : euler)),
      property70("Lcl Scaling", "Lcl Scaling", "", "A", ...item.scale),
      ...(mechanism.preRotation ? [property70("PreRotation", "Vector3D", "Vector", "", ...euler)] : []),
      ...mechanism.properties,
      ...(solidRanges === "" || split ? [] : [property70("SolidRanges", "KString", "", "U", solidRanges)]),
    ];
    const doubleSided = triangles.some((primitive) => materialsById.get(resolveMaterial(primitive.material))?.doubleSided);
    addObject("Model", node("Model", [long(id), text(named(item.id, "Model")), text(kind)], [field("Version", int(232)), properties70(...entries), field("Shading", flag(true)), field("Culling", text(doubleSided || kind !== "Mesh" ? "CullingOff" : "CullingOnCCW"))]));
    const parentId = item.parent === null ? 0 : modelIds.get(item.parent);
    if (parentId === undefined) fail("unknown-parent", `Node ${item.id} names the unknown parent ${item.parent}.`, `Der Knoten ${item.id} nennt den unbekannten Elternknoten ${item.parent}.`);
    oo(id, parentId);
    if (kind !== "Mesh") {
      const attributeId = ids.of(`NodeAttribute:${item.id}`);
      addObject("NodeAttribute", node("NodeAttribute", [long(attributeId), text(named(item.id, "NodeAttribute")), text(kind)], [...(kind === "LimbNode" ? [properties70(property70("Size", "double", "Number", "", 1))] : []), field("TypeFlags", text(kind === "LimbNode" ? "Skeleton" : "Null"))]));
      oo(attributeId, id);
      if (split) {
        const meshModelId = ids.of(`Model:${item.id}:geometry`);
        addObject("Model", node("Model", [long(meshModelId), text(named(`${item.id}-geometry`, "Model")), text("Mesh")], [field("Version", int(232)), properties70(property70("InheritType", "enum", "", "", 1), property70("RotationOrder", "enum", "", "", 0), property70("DefaultAttributeIndex", "int", "Integer", "", 0), ...(solidRanges === "" ? [] : [property70("SolidRanges", "KString", "", "U", solidRanges)])), field("Shading", flag(true)), field("Culling", text(doubleSided ? "CullingOff" : "CullingOnCCW"))]));
        oo(meshModelId, id);
        const part = buildGeometry(item, mesh!, triangles, undefined);
        oo(part.id, meshModelId);
        for (const slot of part.slots) oo(materialObject(slot), meshModelId);
      }
      return;
    }
    const skin = item.skin === undefined ? undefined : skinsById.get(item.skin);
    if (item.skin !== undefined && !skin) fail("unknown-skin", `Node ${item.id} names the unknown skin ${item.skin}.`, `Der Knoten ${item.id} nennt den unbekannten Skin ${item.skin}.`);
    const built = buildGeometry(item, mesh!, triangles, skin);
    oo(built.id, id);
    for (const slot of built.slots) oo(materialObject(slot), id);
  });

  if (poseMatrices.size > 0) {
    const poseId = ids.of(`Pose:${document.id}`);
    const entries = [...poseMatrices.entries()];
    addObject("Pose", node("Pose", [long(poseId), text(named(document.id, "Pose")), text("BindPose")], [field("Type", text("BindPose")), field("Version", int(100)), field("NbPoseNodes", int(entries.length)), ...entries.map(([modelId, matrix]) => node("PoseNode", [], [field("Node", long(modelId)), node("Matrix", [doubles(matrix)])]))]));
  }

  operation.report(0.45);
  if (animation) {
    if (animation.events.length > 0) notices.push({ code: "events-dropped", subject: animation.id, message: { en: `Animation ${animation.id} has ${animation.events.length} event(s); FBX has no event track, they were not written.`, de: `Die Animation ${animation.id} hat ${animation.events.length} Ereignis(se); FBX kennt keine Ereignisspur, sie wurden nicht geschrieben.` } });
    const stackId = ids.of(`AnimationStack:${animation.id}`);
    const layerId = ids.of(`AnimationLayer:${animation.id}`);
    const stop = ticks(animation.durationS);
    const times = (name: string): FbxNode[] => [property70(name === "Local" ? "LocalStart" : "ReferenceStart", "KTime", "Time", "", 0), property70(name === "Local" ? "LocalStop" : "ReferenceStop", "KTime", "Time", "", stop)];
    addObject("AnimationStack", node("AnimationStack", [long(stackId), text(named(animation.id, "AnimStack")), text("")], [properties70(...times("Local"), ...times("Reference"))]));
    addObject("AnimationLayer", node("AnimationLayer", [long(layerId), text(named(animation.id, "AnimLayer")), text("")], [properties70(property70("Weight", "Number", "", "A", 100))]));
    oo(layerId, stackId);
    const seen = new Set<string>();
    const emit = (targetKey: string, curveNodeName: string, targetId: number, property: string, curves: Curves, defaults: readonly number[]): void => {
      const curveNodeId = ids.of(`AnimationCurveNode:${animation.id}:${targetKey}`);
      addObject("AnimationCurveNode", node("AnimationCurveNode", [long(curveNodeId), text(named(curveNodeName, "AnimCurveNode")), text("")], [properties70(...AXIS_NAMES.map((axis, index) => property70(`d|${axis}`, "Number", "", "A", defaults[index] ?? 0)))]));
      oo(curveNodeId, layerId);
      op(curveNodeId, targetId, property);
      const keyTimes = curves.times.map(ticks);
      AXIS_NAMES.forEach((axis, index) => {
        const curveId = ids.of(`AnimationCurve:${animation.id}:${targetKey}:${axis}`);
        const values = curves.components[index];
        addObject("AnimationCurve", node("AnimationCurve", [long(curveId), text(named("", "AnimCurve")), text("")], [field("Default", double(values[0] ?? defaults[index] ?? 0)), field("KeyVer", int(4009)), node("KeyTime", [longs(keyTimes)]), node("KeyValueFloat", [floats(values)]), node("KeyAttrFlags", [ints([curves.step ? 2 : 4])]), node("KeyAttrDataFloat", [floats([0, 0, KEY_ATTRIBUTE_MAGIC, 0])]), node("KeyAttrRefCount", [ints([keyTimes.length])])]));
        op(curveId, curveNodeId, `d|${axis}`);
      });
    };
    const kinds = { translation: ["T", "Lcl Translation"], rotation: ["R", "Lcl Rotation"], scale: ["S", "Lcl Scaling"] } as const;
    animation.channels.forEach((channel: Channel, position) => {
      check();
      operation.report(0.45 + (0.15 * position) / Math.max(1, animation.channels.length));
      if (channel.times.length === 0) return;
      const target = channel.target;
      if ("node" in target) {
        const item = nodesById.get(target.node);
        if (!item) return fail("unknown-target", `Channel targets the unknown node ${target.node}.`, `Der Kanal zielt auf den unbekannten Knoten ${target.node}.`);
        const key = `${target.node}:${target.path}`;
        if (seen.has(key)) {
          notices.push({ code: "channel-duplicate", subject: key, message: { en: `Channel ${key} appears twice; the first one is written.`, de: `Der Kanal ${key} kommt doppelt vor; der erste wird geschrieben.` } });
          return;
        }
        seen.add(key);
        const [curveNodeName, property] = kinds[target.path];
        const curves = target.path === "rotation" ? rotationCurves(channel, preRotations.get(item.id)) : vectorCurves(channel);
        const rest = target.path === "translation" ? item.translation : target.path === "scale" ? item.scale : eulerDegrees(item.rotation);
        emit(key, curveNodeName, modelIds.get(item.id)!, property, curves, rest);
      } else {
        const key = `${target.material}:${target.path}`;
        const fbxId = materialIds.get(target.material);
        if (fbxId === undefined) {
          notices.push({ code: "material-animation-unbound", subject: key, message: { en: `Material ${target.material} is not used by the exported variant; its ${target.path} channel was skipped.`, de: `Das Material ${target.material} wird von der exportierten Variante nicht verwendet; der Kanal ${target.path} wurde übersprungen.` } });
          return;
        }
        if (seen.has(key)) return;
        seen.add(key);
        const curves = colourCurves(channel, target.path === "baseColor" ? 4 : 3);
        if (curves.alphaVaries) notices.push({ code: "alpha-animation-skipped", subject: key, message: { en: `The alpha of ${key} varies; FBX colour curves carry rgb only, alpha was not animated.`, de: `Das Alpha von ${key} ändert sich; FBX-Farbkurven tragen nur RGB, Alpha wurde nicht animiert.` } });
        emit(key, `Surface ${COLOUR_PROPERTY[target.path]}`, fbxId, COLOUR_PROPERTY[target.path], curves, [curves.components[0][0], curves.components[1][0], curves.components[2][0]]);
      }
    });
  }

  operation.report(0.6);
  const sorted = objects.map((entry, index) => ({ ...entry, index })).sort((a, b) => a.group - b.group || a.index - b.index);
  const counts = new Map<string, number>();
  for (const entry of sorted) counts.set(entry.record.name, (counts.get(entry.record.name) ?? 0) + 1);
  const definitions = node("Definitions", [], [
    field("Version", int(100)),
    field("Count", int(sorted.length + 1)),
    node("ObjectType", [text("GlobalSettings")], [field("Count", int(1))]),
    ...[...counts.entries()].map(([name, count]) =>
      node("ObjectType", [text(name)], [field("Count", int(count)), ...(name === "Model" ? [node("PropertyTemplate", [text("FbxNode")], [properties70(property70("InheritType", "enum", "", "", 1), property70("RotationOrder", "enum", "", "", 0), property70("Lcl Translation", "Lcl Translation", "", "A", 0, 0, 0), property70("Lcl Rotation", "Lcl Rotation", "", "A", 0, 0, 0), property70("Lcl Scaling", "Lcl Scaling", "", "A", 1, 1, 1))])] : [])]),
    ),
  ]);
  const takes = animation
    ? node("Takes", [], [field("Current", text(animation.id)), node("Take", [text(animation.id)], [field("FileName", text(`${animation.id}.tak`)), node("LocalTime", [long(0), long(ticks(animation.durationS))]), node("ReferenceTime", [long(0), long(ticks(animation.durationS))])])])
    : node("Takes", [], [field("Current", text(""))]);
  const records: FbxNode[] = [
    node("FBXHeaderExtension", [], [
      field("FBXHeaderVersion", int(1003)),
      field("FBXVersion", int(VERSION)),
      node("CreationTimeStamp", [], [field("Version", int(1000)), field("Year", int(1970)), field("Month", int(1)), field("Day", int(1)), field("Hour", int(10)), field("Minute", int(0)), field("Second", int(0)), field("Millisecond", int(0))]),
      field("Creator", text(CREATOR)),
      sceneInfo(document),
    ]),
    field("FileId", raw(FILE_ID_BYTES)),
    field("CreationTime", text(CREATION_TIME)),
    field("Creator", text(CREATOR)),
    globalSettings(),
    node("Documents", [], [field("Count", int(1)), node("Document", [long(ids.of(`Document:${document.id}`)), text(document.id), text("Scene")], [properties70(property70("SourceObject", "object", "", ""), property70("ActiveAnimStackName", "KString", "", "", animation?.id ?? "")), field("RootNode", long(0))])]),
    node("References"),
    definitions,
    node("Objects", [], sorted.map((entry) => entry.record)),
    node("Connections", [], connections),
    takes,
  ];
  let bytes: Uint8Array;
  try {
    bytes = serialize(records, VERSION, compressor, (done, total) => {
      check();
      operation.report(0.6 + (0.4 * done) / Math.max(1, total));
    }, operation);
  } catch (error) {
    if (error instanceof CancelledError && !(error instanceof FbxCancelledError)) throw new FbxCancelledError();
    throw error;
  }
  operation.report(1);
  return { bytes, notices };
}

/** 📼️ Encodes a resolved document to binary FBX 7.4 (see {@link encodeFbxDetailed} for the notices). */
export function encodeFbx(document: ModelDocument, options: FbxOptions, operation: Operation): Uint8Array {
  return encodeFbxDetailed(document, options, operation).bytes;
}
