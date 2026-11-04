import type { BmpImage, BmpNativeSample, BmpRegion } from '../📸️snapshot/🟦️.ts';

export interface BmpRegionAddress {
  revision: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface ReplaceSamples {
  readonly region: BmpRegion;
  readonly indices?: readonly number[];
  readonly samples?: readonly BmpNativeSample[];
}

export interface SetPaletteEntry {
  index: number
  red: number
  green: number
  blue: number
  reserved: number
}

export interface InsertPaletteEntry {
  index: number
  red: number
  green: number
  blue: number
  reserved: number
  colorsUsed: number
}

export interface RemovePaletteEntry {
  index: number
  colorsUsed: number
}

export type BmpMutation =
  | { readonly mutation: 'replace-image'; readonly payload: { readonly image: BmpImage } }
  | { readonly mutation: 'paint-indexed-region'; readonly payload: BmpRegionAddress & { readonly paletteIndex: number } }
  | { readonly mutation: 'paint-direct-region'; readonly payload: BmpRegionAddress & { readonly red: number; readonly green: number; readonly blue: number; readonly alpha: number } }
  | { readonly mutation: 'replace-samples'; readonly payload: ReplaceSamples }
  | { readonly mutation: 'set-palette-entry'; readonly payload: SetPaletteEntry }
  | { readonly mutation: 'insert-palette-entry'; readonly payload: InsertPaletteEntry }
  | { readonly mutation: 'remove-palette-entry'; readonly payload: RemovePaletteEntry };
