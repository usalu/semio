/** 🧬️ SemioAnimationSnapshot schema — real mirror of `🦀️.rs` (the source of truth).
 * timelines -> channels{target{node,property}, interpolation, keyframes{t, value}}, informed by
 * gltf's Animation/Channel/Sampler triad. Tagged unions use the real `#[serde(tag = "kind", ...)]`
 * discriminant. */

import type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
export interface SemioPoint3 { x: Binary64; y: Binary64; z: Binary64; }
export interface SemioQuaternion { x: Binary64; y: Binary64; z: Binary64; w: Binary64; }

export type AnimInterpolation = "linear" | "step" | "cubicSpline";

export type AnimTargetProperty =
  | { kind: "translation" }
  | { kind: "rotation" }
  | { kind: "scale" }
  | { kind: "weights" }
  | { kind: "custom"; name: string };

export interface AnimTarget {
  node: string;
  property: AnimTargetProperty;
}

export type AnimValue =
  | { kind: "scalar"; value: Binary64 }
  | { kind: "vec3"; value: SemioPoint3 }
  | { kind: "quat"; value: SemioQuaternion }
  | { kind: "weights"; values: Binary64[] };

export interface AnimKeyframe {
  t: Binary64;
  value: AnimValue;
}

export interface AnimChannel {
  target: AnimTarget;
  interpolation: AnimInterpolation;
  keyframes: AnimKeyframe[];
}

export interface AnimTimeline {
  name: string | null;
  channels: AnimChannel[];
}

export interface SemioAnimationSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ timelines: AnimTimeline[];
}
