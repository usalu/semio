/** 🌐️ Block 5D editor — World window: typed twin of `🦀️.rs`'s view-model. A lightweight
 * 3D-projection summary surface (part kind label + first representation's mesh url), matching
 * `render()`'s inputs. */

/** 🌐️ The World window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (a bare `Block5dSnapshot`, resolved locale labels). */
export interface Block5dWorldViewModel {
  windowKindId: "block5d-world";
  bodyKey: "block5d.play.world";
  partLabel: string;
  meshUrl: string;
}

export const BLOCK5D_WORLD_WINDOW_KIND_ID = "block5d-world" as const;
export const BLOCK5D_WORLD_BODY_KEY = "block5d.play.world" as const;
