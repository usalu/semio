/** 🧬️ Puzzle5d direct-mutation discriminated union — TS mirror of `Puzzle5dMutation` (see the
 * sibling `🦀️.rs` union enum and each variant's `<slug>/🦠️mutation/🦀️.rs`
 * payload struct). */
import type { Puzzle5dCompatSpecificity, Puzzle5dKindCatalogs, Puzzle5dPart, Puzzle5dPartAnchor } from "../📸️snapshot/🟦️.ts";

/** 🔘️ One rim grip's 2D-projection presentation (board handle). */
export interface Puzzle5dGrip2d {
  angle: number;
  gripKind?: string;
  radius?: number;
}

/** 🟢️ One rim grip's 3D-projection presentation (world vortex). */
export interface Puzzle5dGrip3d {
  position: [number, number, number];
  direction?: [number, number, number];
  radius?: number;
  label?: string;
}

/** 🔘️ One rim grip on a part, unified across both projections. */
export interface Puzzle5dGrip {
  id: string;
  gripKind?: string;
  "2d": Puzzle5dGrip2d;
  "3d": Puzzle5dGrip3d;
}

/** 📐️ A part's freeform 3D scale — a bare number scales all three axes uniformly, an `[x, y, z]`
 * tuple scales each axis independently. */
export type Puzzle5dScale = number | [number, number, number];

/** 🌱 `create-part` payload — full initial payload at an optional FINAL-state index (`null` appends). */
export interface CreatePart {
  part: Puzzle5dPart;
  index: number | null;
}

/** 🗑️ `delete-part` payload. */
export interface DeletePart {
  id: string;
}

/** 📍 `move-part2d` payload — absolute reposition of a part's 2D-projection anchor point. */
export interface MovePart2d {
  id: string;
  newX: number;
  newY: number;
}

/** 🧊 `replace-part2d-geometry` payload — whole-value swap of a part's 2D shape/extent. */
export interface ReplacePart2dGeometry {
  id: string;
  newShape: string | null;
  newRadius: number | null;
  newWidth: number | null;
  newHeight: number | null;
}

/** ✏️ `edit-part2d-text` payload — replaces a part's 2D-projection authored display text. */
export interface EditPart2dText {
  id: string;
  newText: string | null;
}

/** 🎨 `change-part2d-icon` payload — changes a part's 2D-projection icon. */
export interface ChangePart2dIcon {
  id: string;
  newIconKind: string | null;
}

/** 🙈 `change-part2d-hidden` payload — changes a part's 2D-projection hidden flag. */
export interface ChangePart2dHidden {
  id: string;
  newHidden: boolean | null;
}

/** 🔒 `change-part2d-locked` payload — changes a part's 2D-projection locked flag. */
export interface ChangePart2dLocked {
  id: string;
  newLocked: boolean | null;
}

/** 🚀 `move-part3d` payload — absolute reposition of a part's 3D-projection origin. */
export interface MovePart3d {
  id: string;
  newOrigin: [number, number, number];
}

/** 🔃 `rotate-part3d` payload — changes a part's 3D-projection orientation quaternion. */
export interface RotatePart3d {
  id: string;
  newOrientation: [number, number, number, number] | null;
}

/** 📏 `scale-part3d` payload — changes a part's 3D-projection freeform scale. */
export interface ScalePart3d {
  id: string;
  newScale: Puzzle5dScale | null;
}

/** 🧱 `change-part3d-mesh` payload — changes a part's 3D-projection geometry reference. */
export interface ChangePart3dMesh {
  id: string;
  newMeshUrl: string | null;
}

/** 🖋️ `edit-part3d-label` payload — replaces a part's 3D-projection authored display label. */
export interface EditPart3dLabel {
  id: string;
  newLabel: string | null;
}

/** 🏗️ `change-part-kind` payload — changes a part's `part_kind` catalog reference. */
export interface ChangePartKind {
  id: string;
  newPartKind: string | null;
}

/** ⚓ `change-part-anchor` payload — changes whether a part keeps its stored plane or resets to
 * default XY. */
export interface ChangePartAnchor {
  id: string;
  newAnchor: Puzzle5dPartAnchor;
}

/** ➕ `add-part-grip` payload — attaches a new rim grip to a part at an optional FINAL-state index
 * (`null` appends). */
export interface AddPartGrip {
  partId: string;
  grip: Puzzle5dGrip;
  index: number | null;
}

/** ➖ `remove-part-grip` payload — detaches a rim grip from a part. */
export interface RemovePartGrip {
  partId: string;
  gripId: string;
}

/** 🔌 `replace-part-grip` payload — whole-value swap of one grip's presentation fields. */
export interface ReplacePartGrip {
  partId: string;
  gripId: string;
  newGrip: Puzzle5dGrip;
}

/** 🔗 `connect-grips` payload — creates a fastener between two full grip ids, full initial
 * connection-parameterization payload included. */
export interface ConnectGrips {
  id: string;
  source: string;
  target: string;
  fastenerKind: string | null;
  gap: number;
  shift: number;
  rise: number;
  rotation: number;
  turn: number;
  tilt: number;
  x: number;
  y: number;
}

/** ✂️ `disconnect-grips` payload — removes a fastener between two grips. */
export interface DisconnectGrips {
  id: string;
}

/** 🧮 `replace-fastener-geometry` payload — whole-value swap of a fastener's pose-solver connection
 * pose. */
export interface ReplaceFastenerGeometry {
  id: string;
  newGap: number;
  newShift: number;
  newRise: number;
  newRotation: number;
  newTurn: number;
  newTilt: number;
  newX: number;
  newY: number;
}

/** 🎯 `change-fastener-kind` payload — changes a fastener's `fastener_kind` catalog reference. */
export interface ChangeFastenerKind {
  id: string;
  newFastenerKind: string | null;
}

/** 🏷️ `rename-puzzle5d` payload — changes the document's display label. */
export interface RenamePuzzle5d {
  newLabel: string | null;
}

/** 🌐 `change-domain` payload — changes the document's design domain classification. */
export interface ChangeDomain {
  newDomain: string;
}

/** 📝 `change-description` payload — changes the document's free-text scene description. */
export interface ChangeDescription {
  newDescription: string;
}

/** 🤝 `connect-kind-compatibility` payload — allows one grip-kind-id pair to fasten. A duplicate
 * `(source, target)` pair is a no-op. */
export interface ConnectKindCompatibility {
  source: string;
  target: string;
  bidirectional: boolean;
  important: boolean;
  specificity: Puzzle5dCompatSpecificity;
}

/** 💔 `disconnect-kind-compatibility` payload — revokes one grip-kind-id pair's fasten allowance. */
export interface DisconnectKindCompatibility {
  source: string;
  target: string;
}

/** 📚 `replace-kind-catalogs` payload — whole-value swap of the fixture-carried typed kind-catalog
 * bundle (`null` clears the catalogs). */
export interface ReplaceKindCatalogs {
  newCatalogs: Puzzle5dKindCatalogs | null;
}

/** 🧮️ Semantic puzzle-5d document mutation vocabulary — id-keyed part create-delete plus per-2d/
 * per-3d-projection field edits, grip membership, a grip-to-grip fastener connect/disconnect
 * relationship, and document-level edits, in `Puzzle5dMutation` declaration order. */
export type Puzzle5dMutation =
  | ({ mutation: "createPart" } & CreatePart)
  | ({ mutation: "deletePart" } & DeletePart)
  | ({ mutation: "movePart2d" } & MovePart2d)
  | ({ mutation: "replacePart2dGeometry" } & ReplacePart2dGeometry)
  | ({ mutation: "editPart2dText" } & EditPart2dText)
  | ({ mutation: "changePart2dIcon" } & ChangePart2dIcon)
  | ({ mutation: "changePart2dHidden" } & ChangePart2dHidden)
  | ({ mutation: "changePart2dLocked" } & ChangePart2dLocked)
  | ({ mutation: "movePart3d" } & MovePart3d)
  | ({ mutation: "rotatePart3d" } & RotatePart3d)
  | ({ mutation: "scalePart3d" } & ScalePart3d)
  | ({ mutation: "changePart3dMesh" } & ChangePart3dMesh)
  | ({ mutation: "editPart3dLabel" } & EditPart3dLabel)
  | ({ mutation: "changePartKind" } & ChangePartKind)
  | ({ mutation: "changePartAnchor" } & ChangePartAnchor)
  | ({ mutation: "addPartGrip" } & AddPartGrip)
  | ({ mutation: "removePartGrip" } & RemovePartGrip)
  | ({ mutation: "replacePartGrip" } & ReplacePartGrip)
  | ({ mutation: "connectGrips" } & ConnectGrips)
  | ({ mutation: "disconnectGrips" } & DisconnectGrips)
  | ({ mutation: "replaceFastenerGeometry" } & ReplaceFastenerGeometry)
  | ({ mutation: "changeFastenerKind" } & ChangeFastenerKind)
  | ({ mutation: "renamePuzzle5d" } & RenamePuzzle5d)
  | ({ mutation: "changeDomain" } & ChangeDomain)
  | ({ mutation: "changeDescription" } & ChangeDescription)
  | ({ mutation: "connectKindCompatibility" } & ConnectKindCompatibility)
  | ({ mutation: "disconnectKindCompatibility" } & DisconnectKindCompatibility)
  | ({ mutation: "replaceKindCatalogs" } & ReplaceKindCatalogs);
