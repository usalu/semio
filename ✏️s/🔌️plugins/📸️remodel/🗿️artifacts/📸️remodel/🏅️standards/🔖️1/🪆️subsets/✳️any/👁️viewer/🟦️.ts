/** 👁️ Remodel viewer — subset-level typed twin. Re-exports the one window's typed view-model
 * binding so a host-side TS consumer has one import surface for the whole viewer manifest,
 * mirroring `🦀️.rs`'s `create_remodel_viewer()` stitching the mode/window together. */

export const REMODEL_VIEWER_DIALECT = { artifactKind: "s.remodel.remodel", standard: "1", subset: "*" } as const;

export const REMODEL_VIEW_MODE_VIEW = "view" as const;

export * as modelWindow from "./🎭️modes/👁️view/🪟️windows/🧊️model/🟦️component";
