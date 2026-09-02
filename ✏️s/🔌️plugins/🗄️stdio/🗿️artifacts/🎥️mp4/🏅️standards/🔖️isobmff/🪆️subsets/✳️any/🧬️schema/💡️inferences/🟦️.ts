/** 💡️ mp4 inference schema — real ISO-BMFF `stts`-derived (per-track sample-table) duration. */

export interface Mp4Duration {
  durationSeconds: number;
  trackCount: number;
  sampleCount: number;
}

export interface Mp4Inference {
  /** @derived */
  duration: Mp4Duration;
}
