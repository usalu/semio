/** ⏱ `duration` — the wav snapshot's `fmt`/`data`-derived playback duration. */

export interface WavDuration {
  durationSeconds: number;
  frameCount: number;
  bitsPerSample: number;
}
