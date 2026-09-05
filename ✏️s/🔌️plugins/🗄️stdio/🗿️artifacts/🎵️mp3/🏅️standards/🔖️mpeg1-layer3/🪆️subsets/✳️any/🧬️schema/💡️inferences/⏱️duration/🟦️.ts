/** ⏱ `duration` — the mp3 snapshot's frame-header-derived playback duration (real MPEG-1/2/2.5
 * Layer I/II/III samples-per-frame + sample-rate table lookups, not a guess). */

export interface Mp3Duration {
  durationSeconds: number;
  frameCount: number;
  channelCount: number;
}
