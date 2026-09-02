/** ⏱ `duration` — the mp4 snapshot's per-track `stts`-derived container duration (longest track
 * wins, matching gltf-style clip-duration bounding). */

export interface Mp4Duration {
  durationSeconds: number;
  trackCount: number;
  sampleCount: number;
}
