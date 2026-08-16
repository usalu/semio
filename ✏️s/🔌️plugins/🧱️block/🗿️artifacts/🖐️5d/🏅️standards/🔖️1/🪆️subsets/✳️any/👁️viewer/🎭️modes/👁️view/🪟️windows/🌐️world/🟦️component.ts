/** 🌐️ Block 5D viewer — World window: typed twin of `🦀️component.rs`'s view-model. Read-only mirror
 * of the mesh scene payload `render()` produces — no mutation-shaped fields (no gumball/dislocate),
 * matching the viewer's `ViewEmit`-only contract. Uses the framework's frozen `MeshWindowKit` kind
 * id/body key (contract §2.6), not an app-minted one. */

/** 👁️ The World window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (a bare `Block5dSnapshot`, no runtime/config/utility state: a viewer has none of those). */
export interface Block5dViewWorldViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
}

export const BLOCK5D_VIEW_WORLD_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const BLOCK5D_VIEW_WORLD_BODY_KEY = "framework.window.mesh" as const;
