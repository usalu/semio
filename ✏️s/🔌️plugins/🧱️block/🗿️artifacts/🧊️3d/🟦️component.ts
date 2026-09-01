/** 🧊️ Block 3D artifact — the document entity the 3d app edits. Mirrors `🦀️component.rs`. */

/** 🔘️ One vortex-kind catalog row, composed half — the shared `SemioKitType` fields (id/name/category)
 * live in the joined stdio kit catalog child; this carries only the block-owned label/color/cable-kind
 * overflow via `Block3dVortexKindExtra`. Kept for API-shape parity with the 2d/5d siblings' plain kind
 * type — not itself round-tripped through storage (see `Block3dVortexKindExtra` for the wire form).
 */
export interface Block3dVortexKind {
  id: string;
  name: string;
  label: string;
  color: string;
  defaultCableKind: string;
}

/** 🧩️ Block3d-owned per-vortex-kind overflow NOT representable in stdio's composed `s.stdio.semio.kit`
 * subset (`SemioKitType` carries only `id`/`name`/`category`) — label, color, default cable kind.
 * Id-joined 1:1 to a `SemioKitType` in the composed `Block3dSnapshot.catalog` child by `id`.
 */
export interface Block3dVortexKindExtra {
  id: string;
  name: string;
  label: string;
  color: string;
  defaultCableKind: string;
}

/** 🌱️ One rim-vortex template — where a vortex of `vortexKind` sits on the object's surface. */
export interface Block3dVortexTemplate {
  id: string;
  vortexKind: string;
  position: [number, number, number];
  direction: [number, number, number];
  radius: number;
  label?: string;
}

/** 🪟 Per-window-instance view state (representation subset, layout, active utility). */
export interface Block3dWindowView {
  windowId: string;
  representationIds: string[];
  arrangement: string;
  spacing: number;
  activeUtility: string;
}

/** 🖌️ Transient brush hover pose in world space (config/preview). */
export interface Block3dBrushPreview {
  position: [number, number, number];
  direction: [number, number, number];
}
