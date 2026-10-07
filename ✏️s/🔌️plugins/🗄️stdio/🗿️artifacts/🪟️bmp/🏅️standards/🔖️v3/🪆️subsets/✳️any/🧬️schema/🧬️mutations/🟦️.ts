import type {SnapshotPatch} from "../../../../../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🟦️.ts";
import type { BmpSnapshot } from '../📸️snapshot/🟦️.ts';

export interface BmpRegionAddress {
  revision: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

export type BmpMutation =
  | { readonly mutation: 'set-snapshot'; readonly payload: { readonly snapshot: BmpSnapshot } }
  | { readonly mutation: 'patch-snapshot'; readonly payload: { readonly patch: SnapshotPatch } }
  | { readonly mutation: 'paint-indexed-region'; readonly payload: BmpRegionAddress & { readonly paletteIndex: number } }
  | { readonly mutation: 'paint-direct-region'; readonly payload: BmpRegionAddress & { readonly red: number; readonly green: number; readonly blue: number; readonly alpha: number } };
