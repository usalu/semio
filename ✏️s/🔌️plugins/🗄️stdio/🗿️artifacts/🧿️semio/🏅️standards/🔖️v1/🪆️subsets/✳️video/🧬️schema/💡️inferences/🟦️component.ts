/** 💡️ Semio video inference schema — real per-stream max-pts elapsed time. */

export interface SemioVideoDuration {
  durationSeconds: number;
  streamCount: number;
  sampleCount: number;
}

export interface SemioVideoInference {
  /** @derived */
  duration: SemioVideoDuration;
}
