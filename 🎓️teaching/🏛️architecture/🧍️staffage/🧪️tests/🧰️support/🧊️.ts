/** 🧊️ Shared helpers of the 3D model tests: the sample figure, the catalog assets that already have a volume, GLB and FBX loading through three.js (tests only), the world extent of a loaded scene in staffage axes and the extent of the 2D shapes of an evaluated state.
 *
 * Third-party libraries here are test oracles only (three, gltf-validator); the production code depends on none of them.
 */
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import validator from "gltf-validator";
import { Box3, Vector3, type Object3D } from "three";
import { FBXLoader } from "three/examples/jsm/loaders/FBXLoader.js";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import { poseModel, tessellate, transformPoint, type ModelDocument } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { qualityOf } from "../../🔨️modules/🧊️volume/⏱️job/🟦️.ts";
import type { EvaluatedState } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { STAFFAGE_ROOT, fixture } from "./🧰️.ts";

/** 🧍️ The hand-authored sample figure with its 17-solid volume. */
export const sampleFigure = (): Asset => fixture<{ asset: Asset }>("👤️sample-figure").asset;

/** 🗂️ Every catalog asset that already has a `volume`, read straight from the files (no catalog validation: that is another test's work). */
export function catalogAssetsWithVolume(): Asset[] {
  const root = join(STAFFAGE_ROOT, "🗂️catalog");
  const out: Asset[] = [];
  for (const category of readdirSync(root, { withFileTypes: true })) {
    if (!category.isDirectory()) continue;
    for (const slug of readdirSync(join(root, category.name), { withFileTypes: true })) {
      if (!slug.isDirectory()) continue;
      let text: string;
      try {
        text = readFileSync(join(root, category.name, slug.name, "🔣️.json"), "utf8");
      } catch {
        continue;
      }
      if (!text.includes('"volume"')) continue;
      const asset = (JSON.parse(text) as { asset: Asset }).asset;
      if (asset.volume !== undefined) out.push(asset);
    }
  }
  return out.sort((a, b) => a.id.localeCompare(b.id));
}

/** 🧪️ The assets the model tests run over: the sample figure and every catalog asset with a volume. */
export const volumeAssets = (): Asset[] => [sampleFigure(), ...catalogAssetsWithVolume()];

const arrayBuffer = (bytes: Uint8Array): ArrayBuffer => bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength) as ArrayBuffer;

/** 📥️ A GLB read by three's `GLTFLoader`. */
export const loadGlb = (bytes: Uint8Array): Promise<{ readonly scene: Object3D; readonly animations: readonly unknown[]; readonly parser: { readonly json: any } }> => new Promise((resolve, reject) => new GLTFLoader().parse(arrayBuffer(bytes), "", resolve as never, reject));

/** 📤️ An FBX read by three's `FBXLoader`. */
export const loadFbx = (bytes: Uint8Array): Object3D & { animations: any[] } => new FBXLoader().parse(arrayBuffer(bytes), "") as never;

/** 🔍️ The Khronos validation of GLB bytes: the counts and the messages of severity error or warning. */
export async function validateGlb(bytes: Uint8Array, uri = "model.glb"): Promise<{ readonly errors: number; readonly warnings: number; readonly messages: readonly { readonly code: string; readonly message: string; readonly pointer?: string; readonly severity: number }[] }> {
  const report = await validator.validateBytes(bytes, { uri, maxIssues: 50 });
  return { errors: report.issues.numErrors, warnings: report.issues.numWarnings, messages: report.issues.messages.filter((m: { severity: number }) => m.severity <= 1) };
}

/** 🪧️ Whether an object or one of its ancestors is a glyph node (tagged `glyph` in the node extras). */
export function isGlyph(object: Object3D): boolean {
  for (let o: Object3D | null = object; o !== null; o = o.parent) if (Array.isArray(o.userData?.tags) && (o.userData.tags as string[]).includes("glyph")) return true;
  return false;
}

/** 📐️ A world extent in the staffage axes: ahead (x), up (y), left (z). */
export type Extent = { readonly min: readonly [number, number, number]; readonly max: readonly [number, number, number] };

/** 📦️ The extent of a loaded three scene (skinning applied, `precise`) converted from the neutral frame (X left, Y up, Z ahead) to the staffage axes (ahead, up, left). */
export function sceneExtent(scene: Object3D, include: (object: Object3D) => boolean = (object) => !isGlyph(object)): Extent {
  scene.updateMatrixWorld(true);
  const box = new Box3();
  scene.traverse((object) => {
    if (!(object as { isMesh?: boolean }).isMesh && !(object as { isLineSegments?: boolean }).isLineSegments) return;
    if ((object as { isLineSegments?: boolean }).isLineSegments || !include(object)) return;
    box.expandByObject(object, true);
  });
  const lo: Vector3 = box.min;
  const hi: Vector3 = box.max;
  return { min: [lo.z, lo.y, lo.x], max: [hi.z, hi.y, hi.x] };
}

/** 🖼️ The extent `[min, max]` per staffage axis of the closed shapes of one viewpoint of an evaluated state: elevation `(x, y)`, front `(z, y)`, plan `(x, z)`. */
export function shapeExtent(state: EvaluatedState, viewpoint: "elevation" | "front" | "plan"): { readonly horizontal: readonly [number, number]; readonly vertical: readonly [number, number] } {
  const points = state.shapes[viewpoint].filter((s) => s.shape.role === "body" || s.shape.role === "detail").flatMap((s) => (s.closed ? s.rings.flat() : []));
  const xs = points.map((p) => p.x);
  const ys = points.map((p) => p.y);
  return { horizontal: [Math.min(...xs), Math.max(...xs)], vertical: [Math.min(...ys), Math.max(...ys)] };
}

/** 📏️ The extent tolerance of the catalog (decision D18): the larger of 2 % and 15 mm. */
export const extentTolerance = (length: number): number => Math.max(0.02 * length, 0.015);

/** 🔺️ The rest extent in the staffage axes of a procedural model: tessellated by the kernel, rigid meshes carried by the kernel's rest world matrices, skinned meshes already in model space at rest (the extent of the solids at rest, glyphs excluded). */
export async function restExtent(asset: Asset, document: ModelDocument): Promise<Extent> {
  const resolved = await tessellate({ ...document, solids: document.solids.filter((s) => !s.tags?.includes("glyph")) }, qualityOf(asset));
  const world = poseModel(resolved).world;
  const lo = [Infinity, Infinity, Infinity];
  const hi = [-Infinity, -Infinity, -Infinity];
  for (const node of resolved.nodes) {
    if (node.mesh === undefined) continue;
    const matrix = node.skin === undefined ? world[node.id]! : undefined;
    for (const primitive of resolved.meshes.find((m) => m.id === node.mesh)!.primitives) {
      for (let i = 0; i < primitive.positions.length; i += 3) {
        const p = [primitive.positions[i]!, primitive.positions[i + 1]!, primitive.positions[i + 2]!];
        const q = matrix === undefined ? p : transformPoint(matrix, p);
        for (let k = 0; k < 3; k++) {
          lo[k] = Math.min(lo[k]!, q[k]!);
          hi[k] = Math.max(hi[k]!, q[k]!);
        }
      }
    }
  }
  return { min: [lo[2]!, lo[1]!, lo[0]!], max: [hi[2]!, hi[1]!, hi[0]!] };
}
