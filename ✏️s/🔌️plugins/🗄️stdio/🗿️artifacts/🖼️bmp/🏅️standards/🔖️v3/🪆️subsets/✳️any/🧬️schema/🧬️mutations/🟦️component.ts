/** 🧬️ BmpMutation union — mirrors 🦀️component.rs's `#[serde(tag = "mutation")]` enum. */
export type BmpMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').BmpSnapshot }
  | {
      mutation: 'setHeaderFields';
      headerSize?: number;
      width?: number;
      height?: number;
      rowOrder?: import('../📸️snapshot/🟦️component.ts').BmpRowOrder;
      planes?: number;
      bitsPerPixel?: number;
      compression?: number;
      imageSize?: number;
      xPixelsPerMeter?: number;
      yPixelsPerMeter?: number;
      colorsUsed?: number;
      colorsImportant?: number;
    }
  | { mutation: 'insertPaletteEntry'; index: number; entry: import('../📸️snapshot/🟦️component.ts').BmpPaletteEntry }
  | { mutation: 'removePaletteEntry'; index: number }
  | { mutation: 'setPaletteEntry'; index: number; entry: import('../📸️snapshot/🟦️component.ts').BmpPaletteEntry }
  | { mutation: 'setPixelData'; pixels: number[] };
