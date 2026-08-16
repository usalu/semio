/** 🖼️ Raster viewer — Composite window: typed twin of `🦀️component.rs`'s view-model. Uses the frozen
 * `framework.window.image` kind id (contract §2.6 `ImageWindowKit`) — no options, no utilities: a
 * viewer has no editing chrome. */

/** 👁️ Mirrors the framework's `ImageView` — the pixel payload `ImageWindowKit::render` consumes. */
export interface RasterViewImage {
  width: number;
  height: number;
  mime: string;
  base64: string;
}

export interface RasterViewCompositeViewModel {
  windowKindId: "framework.window.image";
  bodyKey: "framework.window.image";
  image: RasterViewImage;
}

export const RASTER_VIEW_WINDOW_COMPOSITE = "framework.window.image" as const;
export const RASTER_VIEW_BODY_COMPOSITE = "framework.window.image" as const;
