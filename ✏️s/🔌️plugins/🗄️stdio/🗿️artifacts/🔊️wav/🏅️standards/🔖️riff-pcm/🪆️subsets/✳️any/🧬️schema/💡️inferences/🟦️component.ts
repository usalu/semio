/** 💡️ wav inference schema — real RIFF/WAVE `fmt ` + decoded `data` sample-count playback duration. */

export interface WavDuration {
  durationSeconds: number;
  frameCount: number;
  bitsPerSample: number;
}

export interface WavInference {
  /** @state inferred */
  duration: WavDuration;
}
