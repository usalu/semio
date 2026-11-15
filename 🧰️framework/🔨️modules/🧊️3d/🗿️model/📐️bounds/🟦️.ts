/** 📐️ Axis-aligned bounds of a resolved document at a pose: rigid vertices through the world matrix of their node, skinned vertices through the weighted joint palette `world(joint) * inverseBind` (the node's own matrix cancels, as in glTF). Triangle and line primitives both count; with `option.scene` only the meshes of that scene.
 * @see ../🎞️scenes/🟦️.ts */
import type { ModelDocument, Operation } from "../🧬️schema/🟦️.ts";
import { poseModel, type PoseSelector } from "../🦴️mechanism/🟦️.ts";
import { multiply4, transformPoint, type V3 } from "../🧮️math/🟦️.ts";
import { Pace, silentOperation } from "../⏱️operation/🟦️.ts";
import { selectScene, type SceneOption } from "../🎞️scenes/🟦️.ts";

/** 📦️ Minimum and maximum corner. */
export type ModelBounds = { readonly min: V3; readonly max: V3 };

/** 📐️ The posed bounds, or `null` when no vertex is drawn. Throws on procedural documents; cancellation throws `CancelledError`. */
export function boundsOf(document: ModelDocument, selector?: PoseSelector, option: SceneOption = {}, operation: Operation = silentOperation): ModelBounds | null {
  if (document.solids.length > 0) throw new Error(`document ${document.id} is procedural: tessellate it before measuring`);
  const doc = option.scene === undefined ? document : selectScene(document, option.scene);
  const pace = new Pace(operation);
  const { world } = poseModel(doc, selector);
  const meshes = new Map(doc.meshes.map((m) => [m.id, m]));
  const skins = new Map(doc.skins.map((s) => [s.id, s]));
  const min: V3 = [Infinity, Infinity, Infinity], max: V3 = [-Infinity, -Infinity, -Infinity];
  const grow = (p: V3): void => {
    for (let k = 0; k < 3; k++) {
      if (p[k]! < min[k]!) min[k] = p[k]!;
      if (p[k]! > max[k]!) max[k] = p[k]!;
    }
  };
  for (const node of doc.nodes) {
    const mesh = node.mesh === undefined ? undefined : meshes.get(node.mesh);
    if (!mesh) continue;
    const skin = node.skin === undefined ? undefined : skins.get(node.skin);
    const palette = skin?.joints.map((j, k) => multiply4(world[j]!, skin.inverseBind[k]! as readonly number[]));
    for (const p of mesh.primitives) {
      const skinned = palette !== undefined && p.joints !== undefined && p.weights !== undefined;
      const matrix = world[node.id]!;
      for (let v = 0; v < p.positions.length / 3; v++) {
        pace.tick(3);
        const at: V3 = [p.positions[v * 3]!, p.positions[v * 3 + 1]!, p.positions[v * 3 + 2]!];
        if (!skinned) {
          grow(transformPoint(matrix, at));
          continue;
        }
        const sum: V3 = [0, 0, 0];
        for (let k = 0; k < 4; k++) {
          const w = p.weights![v * 4 + k]!;
          if (!(w > 0)) continue;
          const q = transformPoint(palette![p.joints![v * 4 + k]!]!, at);
          sum[0] += w * q[0]; sum[1] += w * q[1]; sum[2] += w * q[2];
        }
        grow(sum);
      }
    }
  }
  return min[0] === Infinity ? null : { min, max };
}
