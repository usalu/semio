/** ⏱ `duration` — the semio video's own real per-stream max-pts elapsed time. */

export interface SemioVideoDuration {
  durationSeconds: number;
  streamCount: number;
  sampleCount: number;
}
