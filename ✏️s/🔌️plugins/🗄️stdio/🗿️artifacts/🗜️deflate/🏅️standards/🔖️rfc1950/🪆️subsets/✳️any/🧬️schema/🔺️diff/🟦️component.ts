/** 🔺️ DeflateDiff — sparse per-field RFC1950 container diff. No collections, no full-replace
 * `snapshot` slot: every field is independently optional. */
import type { DeflateLevelHint } from '../📸️snapshot/🟦️component.ts';

export interface DeflateDiff {
  compressionMethod?: number;
  windowBits?: number;
  compressionLevelHint?: DeflateLevelHint;
  /** 🪆️ Tri-state: absent = unchanged, `null` = dictionary cleared, number = dictionary set. */
  dictId?: number | null;
  payload?: number[];
}
