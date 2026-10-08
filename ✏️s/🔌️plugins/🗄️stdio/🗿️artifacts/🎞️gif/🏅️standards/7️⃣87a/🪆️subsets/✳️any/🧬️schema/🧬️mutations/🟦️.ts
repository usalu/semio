/** 🧬️ One GIF87a palette entry. */
export interface GifRgb { readonly r: number; readonly g: number; readonly b: number }
/** 🎨️ A Global or Local Color Table. */
export interface GifColorTable { readonly sorted: boolean; readonly colors: readonly GifRgb[] }
/** 🖼️ One table-based image: rectangle, optional Local Color Table, interlace flag and palette indices. */
export interface GifImage {
  readonly left: number;
  readonly top: number;
  readonly width: number;
  readonly height: number;
  readonly interlace: boolean;
  readonly lct: GifColorTable | null;
  readonly indices: readonly number[];
}
/** 🧬️ GifMutation union, one member per Rust `GifMutation` leaf (87a). */
export type GifMutation =
  | { readonly mutation: 'setScreenSize'; readonly width: number; readonly height: number }
  | { readonly mutation: 'setGlobalColorTable'; readonly gct: GifColorTable | null }
  | { readonly mutation: 'setBackgroundColorIndex'; readonly index: number }
  | { readonly mutation: 'setPixelAspectRatio'; readonly ratio: number }
  | { readonly mutation: 'insertImage'; readonly index: number; readonly image: GifImage }
  | { readonly mutation: 'removeImage'; readonly index: number }
  | { readonly mutation: 'moveImage'; readonly from: number; readonly to: number }
  | { readonly mutation: 'setImageGeometry'; readonly index: number; readonly left: number; readonly top: number; readonly width: number; readonly height: number }
  | { readonly mutation: 'setImagePixels'; readonly index: number; readonly indices: readonly number[] }
  | { readonly mutation: 'setImageInterlace'; readonly index: number; readonly interlace: boolean };
