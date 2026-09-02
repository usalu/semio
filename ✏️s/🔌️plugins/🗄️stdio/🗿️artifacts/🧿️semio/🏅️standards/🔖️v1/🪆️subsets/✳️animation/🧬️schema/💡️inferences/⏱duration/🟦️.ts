/** ⏱ `duration` — the semio animation snapshot's keyframe-derived playback duration. */

export interface SemioAnimationDuration {
  durationSeconds: number;
  timelineCount: number;
  channelCount: number;
  keyframeCount: number;
}
