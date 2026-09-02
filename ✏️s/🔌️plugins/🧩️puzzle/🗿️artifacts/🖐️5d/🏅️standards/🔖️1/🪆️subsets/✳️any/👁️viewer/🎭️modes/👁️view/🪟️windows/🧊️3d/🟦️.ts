/** 🧊️ Puzzle 5D viewer — World3d window: typed twin of `🦀️.rs`'s view-model. Read-only
 * mirror of the frozen `MeshWindowKit` (contract §2.6) payload `render()` produces — no
 * selection/gumball/engagement fields, matching the viewer's `ViewEmit`-only contract. */

/** 👁️ One placed part instance, read straight off `Puzzle5dSnapshot.parts`. */
export interface Puzzle5dViewWorld3dInstance {
  id: string;
  meshId: string;
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: number | [number, number, number];
  label: string;
}

/** 👁️ The World3d window's typed view-model — the TS mirror of the Rust `render()` boundary's
 * inputs (a bare `Puzzle5dSnapshot`, no runtime/config/utility state: a viewer has none of those). */
export interface Puzzle5dViewWorld3dViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
  instances: Puzzle5dViewWorld3dInstance[];
}

export const PUZZLE5D_VIEW_WORLD3D_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const PUZZLE5D_VIEW_WORLD3D_BODY_KEY = "framework.window.mesh" as const;
