/** ⏱ `duration` — the avi snapshot's `avih` MainAVIHeader-derived playback duration. */

export interface AviDuration {
  durationSeconds: number;
  streamCount: number;
  totalFrames: number;
}
