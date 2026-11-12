/** 🦴️ Pose and animation evaluation of the node mechanism: pose overrides or clip sampling (step, linear, quaternion slerp, looping), joint limits clamped and reported, world matrices.
 *
 * A joint rotation is `q = rest * q_joint` (motion in the node's own frame, axis node-local). `revolute` keeps the twist about `axis` clamped to `[min, max]` rad and drops swing; `prismatic` keeps the translation along `axis` clamped to `[min, max]` m and drops the rest and the rotation;
 * `spherical` clamps the twist to `[twistMin, twistMax]` and the swing cone half angle to `max`; `fixed` ignores the pose; `free` is unlimited.
 * @see ../🧬️schema/🟦️.ts */
import type { Animation, Channel, Issue, Joint, ModelDocument, Node, Slug, Text } from "../🧬️schema/🟦️.ts";
import { compose, dot, normalize, qAxisAngle, qConj, qMul, qNormalize, qSlerp, clamp, lerp3, sub, mul, length, worldMatrices, type M4, type Q4, type V3 } from "../🧮️math/🟦️.ts";
import * as portable from "../🧮️math/🎯️portable/🟦️.ts";

export type PoseSelector = Slug | { readonly animation: Slug; readonly timeS: number } | null | undefined;
export type LocalTransform = { translation: V3; rotation: Q4; scale: V3 };
export type PoseResult = { world: Record<Slug, M4>; local: Record<Slug, LocalTransform>; issues: Issue[] };

const LIMIT: Text = { en: "The joint limit was clamped.", de: "Die Gelenkgrenze wurde begrenzt." };
const DROPPED: Text = { en: "Motion the joint does not allow was dropped.", de: "Vom Gelenk nicht erlaubte Bewegung wurde verworfen." };

const twistSwing = (q: readonly number[], axis: V3): { twist: number; swing: Q4; twistQ: Q4 } => {
  const proj = dot([q[0]!, q[1]!, q[2]!], axis);
  const twistQ = qNormalize([axis[0] * proj, axis[1] * proj, axis[2] * proj, q[3]!]);
  const swing = qMul(q, qConj(twistQ));
  const twist = 2 * portable.atan2(proj, q[3]!);
  return { twist: portable.atan2(portable.sin(twist), portable.cos(twist)), swing: qNormalize(swing), twistQ };
};

const swingAngle = (swing: readonly number[]): number => 2 * portable.atan2(portable.hypot(swing[0]!, swing[1]!, swing[2]!), Math.abs(swing[3]!));

function applyJoint(node: Node, joint: Joint, t: LocalTransform, issues: Issue[], index: number): LocalTransform {
  const path = `/nodes/${index}/joint`;
  const report = (code: string, message: Text) => issues.push({ severity: "warning", code, path, message });
  const axis = normalize(joint.axis);
  const rest = node.rotation as Q4;
  const out: LocalTransform = { translation: t.translation, rotation: t.rotation, scale: t.scale };
  const rel = qNormalize(qMul(qConj(rest), t.rotation));
  const sign = rel[3] < 0 ? -1 : 1;
  const q: Q4 = [rel[0] * sign, rel[1] * sign, rel[2] * sign, rel[3] * sign];
  const same = (a: readonly number[], b: readonly number[], eps: number) => a.every((v, i) => Math.abs(v - b[i]!) <= eps);
  switch (joint.type) {
    case "fixed":
      if (!same(t.translation, node.translation, 1e-9) || !same(t.rotation, rest, 1e-9) && !same(t.rotation, [-rest[0], -rest[1], -rest[2], -rest[3]], 1e-9)) report("joint-fixed", DROPPED);
      out.translation = node.translation as V3;
      out.rotation = rest;
      break;
    case "revolute": {
      const { twist, swing } = twistSwing(q, axis);
      if (swingAngle(swing) > 1e-6) report("joint-swing", DROPPED);
      const lo = joint.min ?? -Infinity, hi = joint.max ?? Infinity;
      const clamped = clamp(twist, lo, hi);
      if (Math.abs(clamped - twist) > 1e-9) report("joint-limit", LIMIT);
      out.rotation = qNormalize(qMul(rest, qAxisAngle(axis, clamped)));
      out.translation = node.translation as V3;
      break;
    }
    case "prismatic": {
      const delta = sub(t.translation, node.translation);
      const along = dot(delta, axis);
      const lo = joint.min ?? -Infinity, hi = joint.max ?? Infinity;
      const clamped = clamp(along, lo, hi);
      if (Math.abs(clamped - along) > 1e-9) report("joint-limit", LIMIT);
      if (length(sub(delta, mul(axis, along))) > 1e-6 || swingAngle(q) > 1e-6) report("joint-swing", DROPPED);
      out.translation = [node.translation[0] + axis[0] * clamped, node.translation[1] + axis[1] * clamped, node.translation[2] + axis[2] * clamped];
      out.rotation = rest;
      break;
    }
    case "spherical": {
      const { twist, swing } = twistSwing(q, axis);
      const tlo = joint.twistMin ?? -Infinity, thi = joint.twistMax ?? Infinity;
      const ct = clamp(twist, tlo, thi);
      let s: Q4 = swing;
      const angle = swingAngle(swing);
      const cone = joint.max ?? Infinity;
      if (angle > cone) {
        const sw = swing[3] < 0 ? [-swing[0], -swing[1], -swing[2], -swing[3]] : swing;
        s = qAxisAngle(normalize([sw[0]!, sw[1]!, sw[2]!]), cone);
      }
      if (Math.abs(ct - twist) > 1e-9 || angle > cone + 1e-9) report("joint-limit", LIMIT);
      out.rotation = qNormalize(qMul(rest, qMul(s, qAxisAngle(axis, ct))));
      break;
    }
    case "free":
      break;
  }
  return out;
}

const keyed = (c: Channel, width: number, timeS: number): number[] => {
  const n = c.times.length;
  const at = (i: number) => c.values.slice(i * width, i * width + width);
  if (timeS <= c.times[0]!) return at(0);
  if (timeS >= c.times[n - 1]!) return at(n - 1);
  let hi = 1;
  while (c.times[hi]! < timeS) hi++;
  const lo = hi - 1;
  if (c.interpolation === "step") return at(lo);
  const f = (timeS - c.times[lo]!) / (c.times[hi]! - c.times[lo]!);
  const a = at(lo), b = at(hi);
  return width === 4 && c.target && "path" in c.target && c.target.path === "rotation" ? qSlerp(a, b, f) : width === 3 ? lerp3(a, b, f) : a.map((v, i) => v + (b[i]! - v) * f);
};

const clipTime = (a: Animation, timeS: number): number => (a.loop ? ((timeS % a.durationS) + a.durationS) % a.durationS : clamp(timeS, 0, a.durationS));

/** 🎨️ Sampled material colour channels of a clip at `timeS`: material id to the animated colours. */
export function sampleMaterials(document: ModelDocument, animationId: Slug, timeS: number): Record<Slug, { baseColor?: number[]; emissive?: number[] }> {
  const animation = document.animations.find((a) => a.id === animationId);
  if (!animation) throw new Error(`unknown animation ${animationId}`);
  const t = clipTime(animation, timeS);
  const out: Record<Slug, { baseColor?: number[]; emissive?: number[] }> = {};
  for (const c of animation.channels) {
    if (!("material" in c.target)) continue;
    (out[c.target.material] ??= {})[c.target.path] = keyed(c, c.target.path === "baseColor" ? 4 : 3, t);
  }
  return out;
}

/** 🧍️ Local transforms, world matrices (column-major) and findings of a pose id, a clip sampled at `timeS`, or the rest pose. Throws on an unknown pose or animation. */
export function poseModel(document: ModelDocument, selector?: PoseSelector): PoseResult {
  const local = new Map<Slug, LocalTransform>(document.nodes.map((n) => [n.id, { translation: [...n.translation] as V3, rotation: [...n.rotation] as Q4, scale: [...n.scale] as V3 }]));
  if (typeof selector === "string") {
    const pose = document.poses.find((p) => p.id === selector);
    if (!pose) throw new Error(`unknown pose ${selector}`);
    for (const [id, patch] of Object.entries(pose.nodes)) {
      const t = local.get(id);
      if (!t) continue;
      if (patch.translation) t.translation = [...patch.translation] as V3;
      if (patch.rotation) t.rotation = qNormalize(patch.rotation);
      if (patch.scale) t.scale = [...patch.scale] as V3;
    }
  } else if (selector) {
    const animation = document.animations.find((a) => a.id === selector.animation);
    if (!animation) throw new Error(`unknown animation ${selector.animation}`);
    const time = clipTime(animation, selector.timeS);
    for (const c of animation.channels) {
      if (!("node" in c.target)) continue;
      const t = local.get(c.target.node);
      if (!t) continue;
      const path = c.target.path;
      const v = keyed(c, path === "rotation" ? 4 : 3, time);
      if (path === "translation") t.translation = v as V3;
      else if (path === "scale") t.scale = v as V3;
      else t.rotation = qNormalize(v);
    }
  }
  const issues: Issue[] = [];
  document.nodes.forEach((n, i) => {
    if (n.joint) local.set(n.id, applyJoint(n, n.joint, local.get(n.id)!, issues, i));
  });
  const world = worldMatrices(document.nodes, (n) => {
    const t = local.get(n.id)!;
    return compose(t.translation, t.rotation, t.scale);
  });
  return { world: Object.fromEntries(world), local: Object.fromEntries(local), issues };
}
