/** 🦴️ The rig as the neutral node tree and its poses: one node per bone, the rest placement from the rest frames, the joint of a bone as a mechanism `Joint`, and the local transform of every bone of an evaluated state.
 *
 * One source of truth: no kinematics is repeated here. Every placement is derived from bone frames that `restFrames` or `evaluate` solved; this module only changes frame (see ../🧭️frame) and expresses the world frames relative to the parent. The mechanism mirrors the staffage joint: a revolute joint turns about the node's own
 * `+Z` axis (the staffage lateral axis) by the angle limits in radians; a bone that any pose or clip key spreads (`abductionDeg`) or twists (`twistDeg`), or whose joint limits a twist, is `spherical` about its own `+X` axis (the bone axis) with the twist limits and an unrestricted swing; a prismatic joint slides along the axis `axisDeg` in
 * the parent frame; a fixed joint is fixed; the rig root is `free` (the evaluated root placement moves it).
 *
 * @see ../../🦴️rig/🟦️.ts — `restFrames`, `solveChain`
 * @see ../../🧮️evaluation/🟦️.ts — `evaluate`
 */
import type { Asset, Bone, Slug } from "../../../🧬️schema/🟦️.ts";
import type { Joint as ModelJoint, Node as ModelNode } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { parentFirstOrder, restFrames } from "../../🦴️rig/🟦️.ts";
import { type Quat, type Triple, type WorldFrame, basisOf, inverseBasis, multiply, quaternionOf, rotate, toNeutral } from "../🧭️frame/🟦️.ts";
import * as portable from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

const RAD = Math.PI / 180;

/** 📍️ A local transform of a node: translation in its parent's frame and rotation relative to the parent. */
export type Placement = { readonly translation: Triple; readonly rotation: Quat };

/** 🔄️ The bones whose motion leaves the sagittal plane: spread or twisted by any pose or clip key of the asset, or limited in twist by their joint. */
export function outOfPlaneBones(asset: Asset): ReadonlySet<Slug> {
  const out = new Set<Slug>();
  const scan = (bones: Readonly<Record<Slug, { readonly abductionDeg?: number; readonly twistDeg?: number }>> | undefined): void => {
    for (const [id, offset] of Object.entries(bones ?? {})) if ((offset.abductionDeg ?? 0) !== 0 || (offset.twistDeg ?? 0) !== 0) out.add(id);
  };
  for (const pose of asset.poses) scan(pose.bones);
  for (const clip of asset.clips) for (const key of clip.keys) scan(key.bones);
  for (const bone of asset.rig.bones) if (bone.joint.type === "revolute" && (bone.joint.twistMinDeg !== undefined || bone.joint.twistMaxDeg !== undefined)) out.add(bone.id);
  return out;
}

/** 🔩️ The mechanism joint of a bone: see the module description. */
export function jointOf(bone: Bone, root: boolean, outOfPlane: boolean): ModelJoint {
  if (root) return { type: "free", axis: [0, 1, 0] };
  const joint = bone.joint;
  switch (joint.type) {
    case "fixed":
      return { type: "fixed", axis: [0, 0, 1] };
    case "prismatic": {
      const angle = (joint.axisDeg ?? 0) * RAD;
      return { type: "prismatic", axis: [portable.cos(angle), portable.sin(angle), 0], ...(joint.minM === undefined ? {} : { min: joint.minM }), ...(joint.maxM === undefined ? {} : { max: joint.maxM }) };
    }
    case "revolute":
      if (outOfPlane) return { type: "spherical", axis: [1, 0, 0], max: Math.PI, twistMin: (joint.twistMinDeg ?? -180) * RAD, twistMax: (joint.twistMaxDeg ?? 180) * RAD };
      return { type: "revolute", axis: [0, 0, 1], ...(joint.minDeg === undefined ? {} : { min: joint.minDeg * RAD }), ...(joint.maxDeg === undefined ? {} : { max: joint.maxDeg * RAD }) };
  }
}

/** 🌍️ A staffage frame given as `(ahead, up, left)` tuples or as the `{ x, y, z }` vectors of an evaluated bone. */
export type FrameLike = { readonly o: Triple | { readonly x: number; readonly y: number; readonly z: number }; readonly u: Triple | { readonly x: number; readonly y: number; readonly z: number }; readonly v: Triple | { readonly x: number; readonly y: number; readonly z: number }; readonly w: Triple | { readonly x: number; readonly y: number; readonly z: number } };

const tuple = (p: Triple | { readonly x: number; readonly y: number; readonly z: number }): Triple => (Array.isArray(p) ? (p as Triple) : [(p as { x: number }).x, (p as { y: number }).y, (p as { z: number }).z]);

/** 🧭️ A frame of either shape as tuples. */
export const worldFrameOf = (frame: FrameLike): WorldFrame => ({ o: tuple(frame.o), u: tuple(frame.u), v: tuple(frame.v), w: tuple(frame.w) });

/** 🧮️ The local placement of every bone from world frames (parents first): the world pose of a node is its parent's times its local placement, so the local rotation is `Rpᵀ · R` and the local translation `Rpᵀ · (o - op)`. */
export function placementsOf(asset: Asset, frames: Readonly<Record<Slug, FrameLike>>): Record<Slug, Placement> {
  const world = new Map<Slug, { readonly o: Triple; readonly basis: ReturnType<typeof basisOf> }>();
  const out: Record<Slug, Placement> = {};
  for (const bone of parentFirstOrder(asset.rig)) {
    const frame = frames[bone.id];
    if (frame === undefined) throw new Error(`no frame for bone "${bone.id}" of asset "${asset.id}"`);
    const f = worldFrameOf(frame);
    const here = { o: toNeutral(f.o), basis: basisOf(f) };
    world.set(bone.id, here);
    if (bone.parent === null) {
      out[bone.id] = { translation: here.o, rotation: quaternionOf(here.basis) };
      continue;
    }
    const parent = world.get(bone.parent)!;
    const inverse = inverseBasis(parent.basis);
    out[bone.id] = { translation: rotate(inverse, [here.o[0] - parent.o[0], here.o[1] - parent.o[1], here.o[2] - parent.o[2]]), rotation: quaternionOf(multiply(inverse, here.basis)) };
  }
  return out;
}

/** 🧊️ The rest placements: the rest frames of the rig with the root at the origin. */
export const restPlacements = (asset: Asset): Record<Slug, Placement> => placementsOf(asset, Object.fromEntries(Object.entries(restFrames(asset.rig)).map(([id, solved]) => [id, solved.frame])));

/** 🌳️ The nodes of the rig at rest, parents first, with the mechanism joints. */
export function skeletonOf(asset: Asset): ModelNode[] {
  const rest = restPlacements(asset);
  const moving = outOfPlaneBones(asset);
  return parentFirstOrder(asset.rig).map((bone) => ({
    id: bone.id,
    parent: bone.parent,
    translation: rest[bone.id]!.translation,
    rotation: rest[bone.id]!.rotation,
    scale: [1, 1, 1] as const,
    joint: jointOf(bone, bone.parent === null, moving.has(bone.id)),
    tags: ["bone"],
  }));
}
