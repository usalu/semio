/** 💡️ Semio animation inference schema — keyframe-derived playback duration. */

export interface SemioAnimationDuration {
  durationSeconds: number;
  timelineCount: number;
  channelCount: number;
  keyframeCount: number;
}

export interface SemioAnimationInference {
  /** @state inferred */
  duration: SemioAnimationDuration;
}
