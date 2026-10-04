import type { PngSnapshot } from '../📸️snapshot/🟦️.ts';
export interface ChangeGammaMutation { readonly revision: string; readonly gama?: number | null; }
export interface PatchPixelsMutation { readonly revision: string; readonly x: number; readonly y: number; readonly width: number; readonly height: number; readonly red: number; readonly green: number; readonly blue: number; readonly alpha: number; }
export type PngNativeProfile = 'indexed' | 'grayscale' | 'grayscale-alpha' | 'rgb' | 'rgba';
export interface PaintNativeSamplesMutation { readonly revision: string; readonly region: { readonly x: number; readonly y: number; readonly width: number; readonly height: number }; readonly paint: { readonly profile: PngNativeProfile; readonly first: number; readonly second: number; readonly third: number; readonly fourth: number }; readonly result: PngSnapshot; }
export interface SetSnapshot { readonly snapshot: PngSnapshot; }
export type PngMutation =
  | { mutation: 'set-snapshot'; payload: SetSnapshot }
  | { mutation: 'patch-snapshot'; payload: unknown }
  | { mutation: 'change-gamma'; payload: ChangeGammaMutation }
  | { mutation: 'patch-pixels'; payload: PatchPixelsMutation }
  | { mutation: 'paint-native-samples'; payload: PaintNativeSamplesMutation };
