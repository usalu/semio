/** 📝️ Text representation for `stdio.semio.animation.mutations`. */
export type SemioAnimationMutation =
  | { mutation: 'setSnapshot'; snapshot: unknown }
  | { mutation: 'insertTimeline'; index: number; timeline: unknown }
  | { mutation: 'removeTimeline'; index: number }
  | { mutation: 'setTimelineName'; index: number; name: string | null }
  | { mutation: 'insertChannel'; timelineIndex: number; index: number; channel: unknown }
  | { mutation: 'removeChannel'; timelineIndex: number; index: number }
  | { mutation: 'setChannelTarget'; timelineIndex: number; index: number; target: unknown }
  | { mutation: 'setChannelInterpolation'; timelineIndex: number; index: number; interpolation: unknown }
  | { mutation: 'insertKeyframe'; timelineIndex: number; channelIndex: number; index: number; keyframe: unknown }
  | { mutation: 'removeKeyframe'; timelineIndex: number; channelIndex: number; index: number }
  | { mutation: 'setKeyframeTime'; timelineIndex: number; channelIndex: number; index: number; t: number }
  | { mutation: 'setKeyframeValue'; timelineIndex: number; channelIndex: number; index: number; value: unknown }
