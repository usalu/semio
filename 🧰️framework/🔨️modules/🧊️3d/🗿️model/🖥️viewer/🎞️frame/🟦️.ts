/** 🎞️ `frameOf`: pose (through the kernel's `poseModel`), joint palettes, variant-bound material colours with animated colour channels, camera matrices, light and a sorted draw list, written into a reusable {@link Frame}.
 *
 * Skinned primitives are drawn with the identity matrix and the palette `world(joint) * inverseBind` (the glTF `inverse(meshNodeWorld)` factor cancels against the mesh node matrix). Draw order: opaque triangles, blended triangles back to front by the view depth of their bounds centre, lines.
 * Pose and colour sampling allocate inside the kernel; everything else writes into preallocated arrays. CPU-posed vertices ({@link posedPositions}) are shared by bounds, picking and the software rasteriser.
 * @see ../🧬️contract/🟦️.ts */
import { poseModel, sampleMaterials } from "../../🦴️mechanism/🟦️.ts";
import { normalize, type V3 } from "../../🧮️math/🟦️.ts";
import { MATERIAL_STRIDE, type Bounds, type Draw, type Frame, type FramePose, type Scene } from "../🧬️contract/🟦️.ts";
import { basisOf, projectionMatrix, viewMatrix, type CameraState } from "../📷️camera/🟦️.ts";
import { followPoint, removeRootTravel, rootIndexOf, smoothFollow, type FollowInput, type RootMotion } from "../🎥️follow/🟦️.ts";
import type { Slug } from "../../🧬️schema/🟦️.ts";

/** 💡️ Light request: `camera` keeps the key light fixed relative to the view, `world` fixed in the scene. */
export type Lighting = { readonly mode?: "camera" | "world"; readonly direction?: V3; readonly intensity?: number; readonly ambient?: number };
/** 🎛️ What one frame shows. `pose` is a pose id, `{ animation, timeS }` or nothing (rest); `variant` a variant id or null. `rootMotion` (default `travel`) and `rootNode` are described in `../🎥️follow/🟦️.ts`; with `follow` (default none) the frame looks at `follow.state.target`, the smoothed followed point, instead of `camera.target`; see {@link FollowInput}. */
export type FrameInput = { readonly pose?: FramePose; readonly variant?: string | null; readonly camera: CameraState; readonly aspect: number; readonly lighting?: Lighting; readonly rootMotion?: RootMotion; readonly rootNode?: Slug; readonly follow?: FollowInput };

/** 🆕️ Allocates a frame sized for `scene`. */
export function createFrame(scene: Scene): Frame {
  const matrices = new Float32Array((scene.nodeIds.length + 1) * 16);
  matrices.set([1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1], scene.nodeIds.length * 16);
  return {
    view: new Float32Array(16), projection: new Float32Array(16), viewProjection: new Float32Array(16), eye: new Float32Array(3), matrices,
    palette: new Float32Array(Math.max(scene.jointCount, 1) * 16), materials: new Float32Array(scene.materials.length * MATERIAL_STRIDE), materialMap: new Int32Array(scene.materials.length),
    draws: scene.primitives.map((_, i): Draw => ({ primitive: i, matrixOffset: 0, material: 0, paletteOffset: -1, pass: 0, depth: 0 })), drawCount: 0,
    light: { direction: new Float32Array(3), color: new Float32Array(3), sky: new Float32Array(3), ground: new Float32Array(3) }, issues: [],
  };
}

const looking: CameraState = { projection: "perspective", target: [0, 0, 0], yaw: 0, pitch: 0, distance: 1, fovY: 1, orthoHeight: 1, extent: 1 };
const point: [number, number, number] = [0, 0, 0];

/** 🎥️ The camera a frame looks through: the request's own, or, with `follow`, a shared copy whose target is the smoothed followed point (nothing to track keeps the request's target). Allocates nothing. */
function lookCamera(scene: Scene, f: Frame, input: FrameInput): CameraState {
  const follow = input.follow;
  if (!follow || follow.mode === "off" || !followPoint(scene, f, follow.mode, point, (primitive) => posedPositions(scene, f, primitive))) return input.camera;
  smoothFollow(follow.state, point, follow.dtS, follow.smoothingS);
  Object.assign(looking, input.camera);
  looking.target = follow.state.target;
  return looking;
}

function mulInto(out: Float32Array, o: number, a: Float32Array, ao: number, b: Float32Array, bo: number): void {
  for (let c = 0; c < 4; c++) for (let r = 0; r < 4; r++) {
    let s = 0;
    for (let k = 0; k < 4; k++) s += a[ao + k * 4 + r]! * b[bo + c * 4 + k]!;
    out[o + c * 4 + r] = s;
  }
}

function mulFloats(out: Float32Array, a: ArrayLike<number>, b: ArrayLike<number>): void {
  for (let c = 0; c < 4; c++) for (let r = 0; r < 4; r++) {
    let s = 0;
    for (let k = 0; k < 4; k++) s += a[k * 4 + r]! * b[c * 4 + k]!;
    out[c * 4 + r] = s;
  }
}

/** 🖼️ Fills `into` (allocated by {@link createFrame} for the same scene) for `input`; throws on unknown pose, animation or variant. */
export function frameOf(scene: Scene, input: FrameInput, into: Frame = createFrame(scene)): Frame {
  const f = into;
  const posed = poseModel(scene.document, input.pose);
  f.issues = posed.issues;
  for (let i = 0; i < scene.nodeIds.length; i++) f.matrices.set(posed.world[scene.nodeIds[i]!]!, i * 16);
  if (input.rootMotion === "in-place") {
    const root = rootIndexOf(scene, input.rootNode);
    if (root >= 0) removeRootTravel(scene, f, root, posed.local[scene.nodeIds[root]!]!.translation);
  }

  for (const skin of scene.skins) {
    for (let j = 0; j < skin.jointNodes.length; j++) mulInto(f.palette, (skin.paletteOffset + j) * 16, f.matrices, skin.jointNodes[j]! * 16, skin.inverseBind, j * 16);
  }

  const variantMap = input.variant ? scene.variants.get(input.variant) : null;
  if (input.variant && !variantMap) throw new Error(`unknown variant ${input.variant}`);
  for (let i = 0; i < scene.materials.length; i++) f.materialMap[i] = variantMap ? variantMap[i]! : i;
  const animated = input.pose && typeof input.pose === "object" ? sampleMaterials(scene.document, input.pose.animation, input.pose.timeS) : null;
  for (let i = 0; i < scene.materials.length; i++) {
    const m = scene.materials[i]!, a = animated?.[m.id], o = i * MATERIAL_STRIDE;
    const base = a?.baseColor ?? m.baseColor, em = a?.emissive ?? m.emissive;
    f.materials[o] = base[0]!; f.materials[o + 1] = base[1]!; f.materials[o + 2] = base[2]!; f.materials[o + 3] = base[3]!;
    f.materials[o + 4] = em[0]!; f.materials[o + 5] = em[1]!; f.materials[o + 6] = em[2]!;
    f.materials[o + 7] = m.unlit ? 1 : 0; f.materials[o + 8] = m.metallic; f.materials[o + 9] = m.roughness; f.materials[o + 10] = m.doubleSided ? 1 : 0; f.materials[o + 11] = m.blend ? 1 : 0;
  }

  const camera = lookCamera(scene, f, input);
  viewMatrix(camera, f.view);
  projectionMatrix(camera, input.aspect, f.projection);
  mulFloats(f.viewProjection, f.projection, f.view);
  const { right, up, forward, eye } = basisOf(camera);
  f.eye[0] = eye[0]; f.eye[1] = eye[1]; f.eye[2] = eye[2];
  const lighting = input.lighting ?? {};
  const dir: V3 = lighting.mode === "world" && lighting.direction
    ? normalize(lighting.direction)
    : (() => {
      const l = lighting.direction ?? [-0.5, 0.8, 0.6];
      return normalize([right[0] * l[0] - forward[0] * l[2] + up[0] * l[1], right[1] * l[0] - forward[1] * l[2] + up[1] * l[1], right[2] * l[0] - forward[2] * l[2] + up[2] * l[1]]);
    })();
  const k = lighting.intensity ?? 0.8, amb = lighting.ambient ?? 1;
  f.light.direction.set(dir);
  f.light.color.fill(k);
  f.light.sky.fill(0.5 * amb);
  f.light.ground.fill(0.3 * amb);

  let n = 0;
  const depthOf = (px: number, py: number, pz: number, m: number | -1): number => {
    let x = px, y = py, z = pz;
    if (m >= 0) {
      const a = f.matrices;
      x = a[m]! * px + a[m + 4]! * py + a[m + 8]! * pz + a[m + 12]!;
      y = a[m + 1]! * px + a[m + 5]! * py + a[m + 9]! * pz + a[m + 13]!;
      z = a[m + 2]! * px + a[m + 6]! * py + a[m + 10]! * pz + a[m + 14]!;
    }
    return -(f.view[2]! * x + f.view[6]! * y + f.view[10]! * z + f.view[14]!);
  };
  const identity = scene.nodeIds.length * 16;
  for (let pass = 0 as 0 | 1 | 2; pass < 3; pass = (pass + 1) as 0 | 1 | 2) {
    const start = n;
    for (const p of scene.primitives) {
      const material = f.materialMap[p.materialIndex]!;
      const blend = f.materials[material * MATERIAL_STRIDE + 11] === 1;
      const want: 0 | 1 | 2 = p.mode === "lines" ? 2 : blend ? 1 : 0;
      if (want !== pass) continue;
      const d = f.draws[n++]!;
      const skinned = p.skin >= 0;
      d.primitive = p.index;
      d.matrixOffset = skinned ? identity : p.nodeIndex * 16;
      d.material = material;
      d.paletteOffset = skinned ? scene.skins[p.skin]!.paletteOffset : -1;
      d.pass = pass;
      d.depth = pass === 1 ? depthOf((p.bounds.min[0] + p.bounds.max[0]) / 2, (p.bounds.min[1] + p.bounds.max[1]) / 2, (p.bounds.min[2] + p.bounds.max[2]) / 2, skinned ? -1 : d.matrixOffset) : 0;
    }
    if (pass === 1) {
      for (let i = start + 1; i < n; i++) {
        const cur = f.draws[i]!;
        let j = i - 1;
        while (j >= start && f.draws[j]!.depth < cur.depth) {
          f.draws[j + 1] = f.draws[j]!;
          j--;
        }
        f.draws[j + 1] = cur;
      }
    }
  }
  f.drawCount = n;
  return f;
}

let scratch = new Float64Array(0);

/** 🦴️ World-space positions of one primitive under the frame's pose: rigid through its node matrix, skinned through the palette (linear blend). Returns a shared scratch array valid until the next call; `count` is the vertex count. */
export function posedPositions(scene: Scene, frame: Frame, primitiveIndex: number): Float64Array {
  const p = scene.primitives[primitiveIndex]!;
  if (scratch.length < p.vertexCount * 3) scratch = new Float64Array(p.vertexCount * 3);
  const out = scratch, v = scene.vertices;
  const skin = p.skin >= 0 ? scene.skinning : null;
  const m = frame.matrices, node = p.nodeIndex * 16;
  for (let i = 0; i < p.vertexCount; i++) {
    const vi = p.vertexStart + i, o = vi * 10, x = v[o]!, y = v[o + 1]!, z = v[o + 2]!;
    if (skin) {
      const so = vi * 8;
      let ox = 0, oy = 0, oz = 0;
      for (let k = 0; k < 4; k++) {
        const w = skin[so + 4 + k]!;
        if (w === 0) continue;
        const a = (scene.skins[p.skin]!.paletteOffset + skin[so + k]!) * 16, P = frame.palette;
        ox += w * (P[a]! * x + P[a + 4]! * y + P[a + 8]! * z + P[a + 12]!);
        oy += w * (P[a + 1]! * x + P[a + 5]! * y + P[a + 9]! * z + P[a + 13]!);
        oz += w * (P[a + 2]! * x + P[a + 6]! * y + P[a + 10]! * z + P[a + 14]!);
      }
      out[i * 3] = ox; out[i * 3 + 1] = oy; out[i * 3 + 2] = oz;
    } else {
      out[i * 3] = m[node]! * x + m[node + 4]! * y + m[node + 8]! * z + m[node + 12]!;
      out[i * 3 + 1] = m[node + 1]! * x + m[node + 5]! * y + m[node + 9]! * z + m[node + 13]!;
      out[i * 3 + 2] = m[node + 2]! * x + m[node + 6]! * y + m[node + 10]! * z + m[node + 14]!;
    }
  }
  return out;
}

/** 📦️ Exact world bounds of all triangle and line vertices under the frame's pose (skinning included). Empty scenes give an inverted box. */
export function boundsOf(scene: Scene, frame: Frame): Bounds {
  const min: [number, number, number] = [Infinity, Infinity, Infinity], max: [number, number, number] = [-Infinity, -Infinity, -Infinity];
  for (const p of scene.primitives) {
    const w = posedPositions(scene, frame, p.index);
    for (let i = 0; i < p.vertexCount; i++) for (let k = 0; k < 3; k++) {
      const x = w[i * 3 + k]!;
      if (x < min[k]!) min[k] = x;
      if (x > max[k]!) max[k] = x;
    }
  }
  return { min, max };
}
