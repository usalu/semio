/** 🧬️ One GIF palette entry. */
export interface GifRgb { readonly r: number; readonly g: number; readonly b: number }
/** 🎨️ A Global or Local Color Table. */
export interface GifColorTable { readonly sorted: boolean; readonly colors: readonly GifRgb[] }
/** 🧹️ Graphic Control Extension disposal method. */
export type GifDisposal = 'unspecified' | 'doNotDispose' | 'restoreToBackground' | 'restoreToPrevious';
/** 🔤️ Plain Text Extension carried by a frame. */
export interface GifPlainText {
  readonly left: number;
  readonly top: number;
  readonly width: number;
  readonly height: number;
  readonly cellWidth: number;
  readonly cellHeight: number;
  readonly fgColorIndex: number;
  readonly bgColorIndex: number;
  readonly text: string;
}
/** 🎞️ One frame: image block plus its Graphic Control Extension state. */
export interface GifFrame {
  readonly left: number;
  readonly top: number;
  readonly width: number;
  readonly height: number;
  readonly interlace: boolean;
  readonly lct: GifColorTable | null;
  readonly indices: readonly number[];
  readonly delayCs: number;
  readonly disposal: GifDisposal;
  readonly transparentIndex: number | null;
  readonly userInput: boolean;
  readonly plainText: GifPlainText | null;
}
/** 🧩️ One Application Extension. */
export interface GifAppExtension { readonly identifier: readonly number[]; readonly authCode: readonly number[]; readonly data: readonly number[] }
/** 🧬️ GifMutation union, one member per Rust `GifMutation` leaf (89a). */
export type GifMutation =
  | { readonly mutation: 'setScreenSize'; readonly width: number; readonly height: number }
  | { readonly mutation: 'setGlobalColorTable'; readonly gct: GifColorTable | null }
  | { readonly mutation: 'setBackgroundColorIndex'; readonly index: number }
  | { readonly mutation: 'setPixelAspectRatio'; readonly ratio: number }
  | { readonly mutation: 'setLoopCount'; readonly loopCount: number | null }
  | { readonly mutation: 'insertFrame'; readonly index: number; readonly frame: GifFrame }
  | { readonly mutation: 'removeFrame'; readonly index: number }
  | { readonly mutation: 'moveFrame'; readonly from: number; readonly to: number }
  | { readonly mutation: 'setFrameGeometry'; readonly index: number; readonly left: number; readonly top: number; readonly width: number; readonly height: number }
  | { readonly mutation: 'setFramePixels'; readonly index: number; readonly indices: readonly number[] }
  | { readonly mutation: 'setFrameInterlace'; readonly index: number; readonly interlace: boolean }
  | { readonly mutation: 'setFrameDelay'; readonly index: number; readonly delayCs: number }
  | { readonly mutation: 'setFrameDisposal'; readonly index: number; readonly disposal: GifDisposal }
  | { readonly mutation: 'setFrameTransparency'; readonly index: number; readonly transparentIndex: number | null }
  | { readonly mutation: 'setFrameUserInput'; readonly index: number; readonly userInput: boolean }
  | { readonly mutation: 'insertComment'; readonly index: number; readonly text: string }
  | { readonly mutation: 'removeComment'; readonly index: number }
  | { readonly mutation: 'addAppExtension'; readonly index: number; readonly extension: GifAppExtension }
  | { readonly mutation: 'removeAppExtension'; readonly index: number };
