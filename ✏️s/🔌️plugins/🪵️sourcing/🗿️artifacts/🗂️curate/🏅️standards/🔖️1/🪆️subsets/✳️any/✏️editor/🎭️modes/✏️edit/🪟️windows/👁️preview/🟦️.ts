/** 👁️ Sourcing curate app — the preview window: a 3D preview of the currently-selected object.
 *  Typed twin of the Rust `render(document: &CurateSnapshot, selected_ids: &[String], labels) -> UiNode`
 *  boundary (`🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs`).
 */

export const windowKindId = "sourcing-preview";
export const bodyKey = "sourcing.preview";
export const surfaceId = "sourcing.preview.world";

/** 🧱️ The single selected object kind's mesh/instance identity — `null` renders the "no selection"
 *  placeholder (see `SourcingLabels.noSelection`).
 */
export interface PreviewSelection {
  objectId: string;
  meshId: string;
}

/** 🪟️ The preview window's typed view model. */
export interface PreviewViewModel {
  selected: PreviewSelection | null;
}
