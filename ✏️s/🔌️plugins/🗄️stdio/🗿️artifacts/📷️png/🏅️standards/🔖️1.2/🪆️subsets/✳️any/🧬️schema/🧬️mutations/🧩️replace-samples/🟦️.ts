export interface PngNativeRegion { readonly x: number; readonly y: number; readonly width: number; readonly height: number; }
export interface ReplaceSamples { readonly region: PngNativeRegion; readonly samples: readonly number[]; }
