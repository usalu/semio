/** 🦴️ Mechanism joints as FBX limit properties of a limb node.
 *
 * FBX limits are per Euler axis of the local rotation (after `PreRotation`) and per component of the local translation, so only joints whose axis is a principal axis can be expressed; others are reported and skipped.
 * A limited joint carries its rest rotation as `PreRotation` and a zero `Lcl Rotation`, so the limits are relative to the rest pose like the mechanism's own.
 *
 * @see https://help.autodesk.com/view/FBX/2017/ENU/?guid=__cpp_ref_class_fbx_node_html
 */
import type { Joint, Node, Vec3 } from "../../../🧬️schema/🟦️.ts";
import { DEGREES } from "../📐️euler/🟦️.ts";
import { property70 } from "../🧱️records/🟦️.ts";
import type { FbxNode } from "../🧱️records/🟦️.ts";

const AXES = ["X", "Y", "Z"] as const;

/** 🔔️ A finding the encoder reports instead of silently dropping data. */
export type Notice = { readonly code: string; readonly subject: string; readonly message: { readonly en: string; readonly de: string } };

/** 🧷️ How one node expresses its joint. */
export type Mechanism = { readonly preRotation: boolean; readonly properties: readonly FbxNode[]; readonly notices: readonly Notice[] };

type Range = { min?: number; max?: number };

function principal(axis: Vec3): { index: number; sign: 1 | -1 } | undefined {
  for (let index = 0; index < 3; index++) {
    const others = axis.filter((_, other) => other !== index).every((value) => Math.abs(value) < 1e-6);
    if (others && Math.abs(Math.abs(axis[index]) - 1) < 1e-6) return { index, sign: axis[index] > 0 ? 1 : -1 };
  }
  return undefined;
}

function limitProperties(prefix: "Rotation" | "Translation", ranges: readonly Range[]): FbxNode[] {
  const active = ranges.some((range) => range.min !== undefined || range.max !== undefined);
  if (!active) return [];
  const min = ranges.map((range) => range.min ?? 0);
  const max = ranges.map((range) => range.max ?? 0);
  const out = [property70(`${prefix}Active`, "bool", "", "", true), property70(`${prefix}Min`, "Vector3D", "Vector", "", ...min), property70(`${prefix}Max`, "Vector3D", "Vector", "", ...max)];
  ranges.forEach((range, index) => {
    if (range.min !== undefined) out.push(property70(`${prefix}Min${AXES[index]}`, "bool", "", "", true));
    if (range.max !== undefined) out.push(property70(`${prefix}Max${AXES[index]}`, "bool", "", "", true));
  });
  return out;
}

function about(index: number, sign: 1 | -1, lower: number | undefined, upper: number | undefined, scale: number, offset: number): Range[] {
  const ranges: Range[] = [{}, {}, {}];
  const a = lower === undefined ? undefined : lower * scale;
  const b = upper === undefined ? undefined : upper * scale;
  ranges[index] = sign > 0 ? { min: a === undefined ? undefined : a + offset, max: b === undefined ? undefined : b + offset } : { min: b === undefined ? undefined : -b + offset, max: a === undefined ? undefined : -a + offset };
  return ranges;
}

const locked = (value = 0): Range => ({ min: value, max: value });

/** 🦿️ The FBX expression of `item.joint`. */
export function mechanismOf(item: Node): Mechanism {
  const joint: Joint | undefined = item.joint;
  if (!joint || joint.type === "free") return { preRotation: false, properties: [], notices: [] };
  const notices: Notice[] = [];
  const axis = principal(joint.axis);
  if (joint.type !== "fixed" && !axis) {
    notices.push({ code: "joint-axis-skipped", subject: item.id, message: { en: `Joint ${item.id} (${joint.type}) turns about a non-principal axis; FBX limits are per Euler axis, so no limit was written.`, de: `Das Gelenk ${item.id} (${joint.type}) dreht um eine Nicht-Hauptachse; FBX-Grenzen gelten je Euler-Achse, daher wurde keine Grenze geschrieben.` } });
    return { preRotation: false, properties: [], notices };
  }
  let rotation: Range[] = [locked(), locked(), locked()];
  let translation: Range[] = [locked(item.translation[0]), locked(item.translation[1]), locked(item.translation[2])];
  if (axis) {
    const { index, sign } = axis;
    if (joint.type === "revolute") {
      rotation = rotation.map((range, other) => (other === index ? {} : range));
      rotation[index] = about(index, sign, joint.min, joint.max, DEGREES, 0)[index];
    } else if (joint.type === "prismatic") {
      translation = [{}, {}, {}].map((_, other) => (other === index ? {} : locked(item.translation[other])));
      translation[index] = about(index, sign, joint.min, joint.max, 1, item.translation[index])[index];
    } else if (joint.type === "spherical") {
      const swing = joint.max === undefined ? undefined : joint.max * DEGREES;
      rotation = rotation.map((_, other) => (other === index || swing === undefined ? {} : { min: -swing, max: swing }));
      rotation[index] = about(index, sign, joint.twistMin, joint.twistMax, DEGREES, 0)[index];
    }
  }
  const properties = [...limitProperties("Rotation", rotation), ...limitProperties("Translation", translation)];
  return { preRotation: true, properties, notices };
}
