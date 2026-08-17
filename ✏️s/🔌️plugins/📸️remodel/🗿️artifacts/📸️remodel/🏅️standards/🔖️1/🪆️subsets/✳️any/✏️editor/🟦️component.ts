/** ✏️ Remodel editor — subset-level typed twin. Re-exports every window's typed view-model binding
 * so a host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️component.rs`'s `create_remodel_app()` stitching every window/mode module together. Three
 * modes (`capture`/`analyze`/`model`), three windows (`frames`/`report`/`model`), one each. */

export const REMODEL_EDITOR_DIALECT = { artifactKind: "s.remodel.remodel", standard: "1", subset: "*" } as const;

export const REMODEL_PLAY_MODE_CAPTURE = "capture" as const;
export const REMODEL_PLAY_MODE_ANALYZE = "analyze" as const;
export const REMODEL_PLAY_MODE_MODEL = "model" as const;

// 🪟️ Namespaced re-export: kept consistent even though today's three windows do not collide on any
// exported name, so a future window addition never silently becomes ambiguous.
export * as framesWindow from "./🎭️modes/📷️capture/🪟️windows/🖼️frames/🟦️component";
export * as reportWindow from "./🎭️modes/🔍️analyze/🪟️windows/📊️report/🟦️component";
export * as modelWindow from "./🎭️modes/🧊️model/🪟️windows/🧊️model/🟦️component";
