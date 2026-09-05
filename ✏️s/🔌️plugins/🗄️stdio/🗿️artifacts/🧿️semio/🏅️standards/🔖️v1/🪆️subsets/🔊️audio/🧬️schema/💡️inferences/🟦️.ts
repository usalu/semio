/** 💡️ Semio audio inference schema — sample-count-derived playback duration. */

export interface SemioAudioDuration {
  durationSeconds: number;
  sampleCount: number;
  channelCount: number;
}

export interface SemioAudioInference {
  /** @derived */
  duration: SemioAudioDuration;
}
