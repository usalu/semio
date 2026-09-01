/** 🧬️ SemioAnimationMutation schema — real mirror of `🦀️component.rs` (the source of truth).
 * Discriminated union on the `mutation` tag (`#[serde(tag = "mutation", rename_all = "camelCase")]`). */
import type { SemioAnimationSnapshot, AnimTimeline, AnimChannel, AnimKeyframe, AnimTarget, AnimInterpolation, AnimValue } from "../📸️snapshot/🟦️component";

export type SemioAnimationMutation =
  | { mutation: "setSnapshot"; snapshot: SemioAnimationSnapshot }
  | { mutation: "insertTimeline"; index: number; timeline: AnimTimeline }
  | { mutation: "removeTimeline"; index: number }
  | { mutation: "setTimelineName"; index: number; name: string | null }
  | { mutation: "insertChannel"; timelineIndex: number; index: number; channel: AnimChannel }
  | { mutation: "removeChannel"; timelineIndex: number; index: number }
  | { mutation: "setChannelTarget"; timelineIndex: number; index: number; target: AnimTarget }
  | { mutation: "setChannelInterpolation"; timelineIndex: number; index: number; interpolation: AnimInterpolation }
  | { mutation: "insertKeyframe"; timelineIndex: number; channelIndex: number; index: number; keyframe: AnimKeyframe }
  | { mutation: "removeKeyframe"; timelineIndex: number; channelIndex: number; index: number }
  | { mutation: "setKeyframeTime"; timelineIndex: number; channelIndex: number; index: number; t: number }
  | { mutation: "setKeyframeValue"; timelineIndex: number; channelIndex: number; index: number; value: AnimValue };
