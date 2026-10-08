export type PngNativeProfile = "indexed" | "grayscale" | "grayscale-alpha" | "rgb" | "rgba";
export interface PngNativeRegion { readonly x: number; readonly y: number; readonly width: number; readonly height: number; }
export interface PngNativePaint { readonly profile: PngNativeProfile; readonly first: number; readonly second: number; readonly third: number; readonly fourth: number; }
export interface PaintNativeSamplesMutation { readonly revision: string; readonly region: PngNativeRegion; readonly paint: PngNativePaint; }
