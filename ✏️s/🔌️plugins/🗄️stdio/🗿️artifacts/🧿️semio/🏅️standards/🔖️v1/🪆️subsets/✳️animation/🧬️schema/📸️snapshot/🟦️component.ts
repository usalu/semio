/** 🧬️ SemioAnimationSnapshot schema — real mirror of `🦀️component.rs` (the source of truth).
 * timelines -> channels{target{node,property}, interpolation, keyframes{t, value}}, informed by
 * gltf's Animation/Channel/Sampler triad. Tagged unions use the real `#[serde(tag = "kind", ...)]`
 * discriminant. */

export interface SemioPoint3 { x: number; y: number; z: number; }
export interface SemioQuaternion { x: number; y: number; z: number; w: number; }

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
  | { kind: "scalar"; value: number }
  | { kind: "vec3"; value: SemioPoint3 }
  | { kind: "quat"; value: SemioQuaternion }
  | { kind: "weights"; values: number[] };

export interface AnimKeyframe {
  t: number;
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
  /** @state persistent */ schema: string;
  /** @state persistent */ timelines: AnimTimeline[];
}
