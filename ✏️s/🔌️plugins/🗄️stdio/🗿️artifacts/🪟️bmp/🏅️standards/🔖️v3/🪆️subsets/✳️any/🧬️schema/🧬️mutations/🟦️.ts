import type { BmpImage } from '../📸️snapshot/🟦️.ts';

export interface BmpRegionAddress {
  revision: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

export type BmpMutation =
  | { readonly mutation: 'replace-image'; readonly payload: { readonly image: BmpImage } }
  | { readonly mutation: 'paint-indexed-region'; readonly payload: BmpRegionAddress & { readonly paletteIndex: number } }
  | { readonly mutation: 'paint-direct-region'; readonly payload: BmpRegionAddress & { readonly red: number; readonly green: number; readonly blue: number; readonly alpha: number } };
