/** 🧭️ Raster viewer — Navigator window: typed twin of `🦀️.rs`'s view-model. A read-only
 * minimap of the same composited image the Composite window shows, under its own distinct window
 * kind id (`framework.window.image` is already claimed by the Composite window in this manifest). */

import type { RasterViewImage } from "../🖼️composite/🟦️";
export type { RasterViewImage };

export interface RasterViewNavigatorViewModel {
  windowKindId: "raster-view-navigator";
  bodyKey: "raster.view.navigator";
  image: RasterViewImage;
}

export const RASTER_VIEW_WINDOW_NAVIGATOR = "raster-view-navigator" as const;
export const RASTER_VIEW_BODY_NAVIGATOR = "raster.view.navigator" as const;
