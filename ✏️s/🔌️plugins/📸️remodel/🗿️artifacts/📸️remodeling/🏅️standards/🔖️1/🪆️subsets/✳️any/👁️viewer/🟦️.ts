/** 👁️ Remodeling viewer — subset-level typed twin. Re-exports the one window's typed view-model
 * binding so a host-side TS consumer has one import surface for the whole viewer manifest,
 * mirroring `🦀️.rs`'s `create_remodeling_viewer()` stitching the mode/window together. */

export const REMODELING_VIEWER_DIALECT = { artifactKind: "s.remodeling.remodeling", standard: "1", subset: "*" } as const;

export const REMODELING_VIEW_MODE_VIEW = "view" as const;

export * as modelWindow from "./🎭️modes/👁️view/🪟️windows/🧊️model/🟦️";
