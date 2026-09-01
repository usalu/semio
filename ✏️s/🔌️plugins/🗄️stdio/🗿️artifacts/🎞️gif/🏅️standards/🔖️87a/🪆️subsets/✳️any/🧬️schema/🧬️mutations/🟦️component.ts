/** 🧬️ GifMutation union. Mirrors only `SetSnapshot` of the Rust `GifMutation` enum's 11 variants
 * — `../📸️snapshot/🟦️component.ts`'s `GifSnapshot` is still a raw-`entries` stub with no
 * structured screen/GCT/image model, so the other 10 variants (SetScreenSize,
 * SetGlobalColorTable, SetBackgroundColorIndex, SetPixelAspectRatio, InsertImage, RemoveImage,
 * MoveImage, SetImageGeometry, SetImagePixels, SetImageInterlace) have no TS payload types to
 * mirror against yet; see `🦀️.rs` in this directory. */
export type GifMutation =
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').GifSnapshot };
