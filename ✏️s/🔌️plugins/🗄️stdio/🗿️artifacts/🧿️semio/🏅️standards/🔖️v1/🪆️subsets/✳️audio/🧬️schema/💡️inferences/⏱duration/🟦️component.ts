/** ⏱ `duration` — the semio audio snapshot's sample-count-derived playback duration. */

export interface SemioAudioDuration {
  durationSeconds: number;
  sampleCount: number;
  channelCount: number;
}
