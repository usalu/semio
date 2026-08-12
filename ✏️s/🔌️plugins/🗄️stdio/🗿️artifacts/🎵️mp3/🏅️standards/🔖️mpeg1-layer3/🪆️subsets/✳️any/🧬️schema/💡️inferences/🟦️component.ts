/** 💡️ mp3 inference schema — real MPEG-1/2/2.5 Layer III frame-header-derived playback duration. */

export interface Mp3Duration {
  durationSeconds: number;
  frameCount: number;
  channelCount: number;
}

export interface Mp3Inference {
  /** @state inferred */
  duration: Mp3Duration;
}
