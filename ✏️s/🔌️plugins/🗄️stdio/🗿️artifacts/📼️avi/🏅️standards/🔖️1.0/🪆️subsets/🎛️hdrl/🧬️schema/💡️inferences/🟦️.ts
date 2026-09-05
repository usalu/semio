/** 💡️ avi inference schema — real `avih` MainAVIHeader-derived playback duration. */

export interface AviDuration {
  durationSeconds: number;
  streamCount: number;
  totalFrames: number;
}

export interface AviInference {
  /** @derived */
  duration: AviDuration;
}
