/** 🔢️ Sourcing curation app — the grid window: every filtered stock object laid out on a 3D grid.
 *  Typed twin of the Rust `render(document: &CurationSnapshot, cfg: &SourcingCurationConfig) -> UiNode`
 *  boundary (`🎭️modes/✏️edit/🪟️windows/🔢️grid/🦀️.rs`).
 */

export const windowKindId = "sourcing-grid";
export const bodyKey = "sourcing.grid";
export const surfaceId = "sourcing.grid.world";

/** 🧱️ One placed instance — mirrors `render`'s per-filtered-`ObjectKind` grid placement/scale. */
export interface GridInstance {
  objectId: string;
  meshId: string;
  position: [number, number, number];
  scale: number;
}

/** 🪟️ The grid window's typed view model. */
export interface GridViewModel {
  instances: GridInstance[];
}
