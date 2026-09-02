/** 🧬️ GifMutation union. Mirrors only `SetSnapshot` of the Rust `GifMutation` enum's 20 variants
 * — `../📸️snapshot/🟦️.ts`'s `GifSnapshot` is still the pre-rewrite raw-`entries` stub
 * (the Rust snapshot moved to a structured screen/GCT/frame model per `📸️snapshot/🦀️.rs`),
 * so the other 19 variants (SetScreenSize, SetGlobalColorTable, SetBackgroundColorIndex,
 * SetPixelAspectRatio, SetLoopCount, InsertFrame, RemoveFrame, MoveFrame, SetFrameGeometry,
 * SetFramePixels, SetFrameInterlace, SetFrameDelay, SetFrameDisposal, SetFrameTransparency,
 * SetFrameUserInput, InsertComment, RemoveComment, AddAppExtension, RemoveAppExtension) have no
 * TS payload types to mirror against yet; see `🦀️.rs` in this directory. */
export type GifMutation =
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️.ts').GifSnapshot };
