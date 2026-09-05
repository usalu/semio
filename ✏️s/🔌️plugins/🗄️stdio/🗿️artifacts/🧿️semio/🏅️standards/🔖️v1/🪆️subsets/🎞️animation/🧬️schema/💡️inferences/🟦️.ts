/** 💡️ Semio animation inference schema — keyframe-derived playback duration. */

export interface SemioAnimationDuration {
  durationSeconds: number;
  timelineCount: number;
  channelCount: number;
  keyframeCount: number;
}

export interface SemioAnimationInference {
  /** @derived */
  duration: SemioAnimationDuration;
}
