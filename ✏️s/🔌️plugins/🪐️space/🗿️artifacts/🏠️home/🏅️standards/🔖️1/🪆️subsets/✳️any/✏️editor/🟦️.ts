/** ✏️ S Home launcher editor — subset-level typed twin. Re-exports the one window's typed view-model
 * binding so a host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️.rs`'s `create_home_app()` stitching the mode/window modules together. */

export const HOME_EDITOR_DIALECT = { artifactKind: "s.space.home", standard: "1", subset: "*" } as const;

export const HOME_EXPLORE_MODE = "explore" as const;

export * as mainWindow from "./🎭️modes/🔎️explore/🪟️windows/🏠️main/🟦️component";
