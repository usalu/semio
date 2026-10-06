/** 🧬️ Puzzle5d direct-mutation discriminated union — TS mirror of `Puzzle5dMutation` (see the
 * sibling `🦀️.rs` union enum and each variant's `<slug>/🦠️mutation/🦀️.rs`
 * payload struct). */
import type { Puzzle5dCompatSpecificity, Puzzle5dKindCatalogs, Puzzle5dPart, Puzzle5dPartAnchor, Puzzle5dTargetVolume, Puzzle5dGrip, Puzzle5dScale, Puzzle5dVector3, Puzzle5dVector4 } from "../📸️snapshot/🟦️.ts";
import type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
export type { Puzzle5dGrip2d, Puzzle5dGrip3d, Puzzle5dGrip, Puzzle5dScale } from "../📸️snapshot/🟦️.ts";

/** 🌱 `create-part` payload — full initial payload at an optional FINAL-state index (`null` appends). */
export interface CreatePart {
  part: Puzzle5dPart;
  index: bigint | null;
}

/** 🗑️ `delete-part` payload. */
export interface DeletePart {
  id: string;
}

/** 📍 `move-part2d` payload — absolute reposition of a part's 2D-projection anchor point. */
export interface MovePart2d {
  id: string;
  newX: Binary64;
  newY: Binary64;
}

/** 🧊 `replace-part2d-geometry` payload — whole-value swap of a part's 2D shape/extent. */
export interface ReplacePart2dGeometry {
  id: string;
  newShape: string | null;
  newRadius: Binary64 | null;
  newWidth: Binary64 | null;
  newHeight: Binary64 | null;
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
  newOrigin: Puzzle5dVector3;
}

/** 🔃 `rotate-part3d` payload — changes a part's 3D-projection orientation quaternion. */
export interface RotatePart3d {
  id: string;
  newOrientation: Puzzle5dVector4 | null;
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
  index: bigint | null;
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
  gap: Binary64;
  shift: Binary64;
  rise: Binary64;
  rotation: Binary64;
  turn: Binary64;
  tilt: Binary64;
  x: Binary64;
  y: Binary64;
}

/** ✂️ `disconnect-grips` payload — removes a fastener between two grips. */
export interface DisconnectGrips {
  id: string;
}

/** 🧮 `replace-fastener-geometry` payload — whole-value swap of a fastener's pose-solver connection
 * pose. */
export interface ReplaceFastenerGeometry {
  id: string;
  newGap: Binary64;
  newShift: Binary64;
  newRise: Binary64;
  newRotation: Binary64;
  newTurn: Binary64;
  newTilt: Binary64;
  newX: Binary64;
  newY: Binary64;
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

/** 🌍 `create-target-volume` payload — full initial payload at an optional FINAL-state index
 * (`null` appends). `index: Option<usize>` carries no `skip_serializing_if`, so the key stays
 * required with a nullable value. */
export interface CreateTargetVolume {
  targetVolume: Puzzle5dTargetVolume;
  index: bigint | null;
}

/** 🪦 `delete-target-volume` payload. */
export interface DeleteTargetVolume {
  id: string;
}

/** 🚀 `move-target-volume` payload — absolute reposition of a target volume's origin. */
export interface MoveTargetVolume {
  id: string;
  newOrigin: Puzzle5dVector3;
}

/** 🌀 `rotate-target-volume` payload — `new_orientation: Option<[f64; 4]>` carries no
 * `skip_serializing_if`, so the key stays required with a nullable value. */
export interface RotateTargetVolume {
  id: string;
  newOrientation: Puzzle5dVector4 | null;
}

/** 📐 `scale-target-volume` payload — `new_scale: Option<Puzzle5dScale>` carries no
 * `skip_serializing_if`, so the key stays required with a nullable value. */
export interface ScaleTargetVolume {
  id: string;
  newScale: Puzzle5dScale | null;
}

/** 🙈 `change-target-volume-hidden` payload. */
export interface ChangeTargetVolumeHidden {
  id: string;
  newHidden: boolean;
}

/** 🔐 `change-target-volume-locked` payload. */
export interface ChangeTargetVolumeLocked {
  id: string;
  newLocked: boolean;
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
  | ({ mutation: "replaceKindCatalogs" } & ReplaceKindCatalogs)
  | ({ mutation: "createTargetVolume" } & CreateTargetVolume)
  | ({ mutation: "deleteTargetVolume" } & DeleteTargetVolume)
  | ({ mutation: "moveTargetVolume" } & MoveTargetVolume)
  | ({ mutation: "rotateTargetVolume" } & RotateTargetVolume)
  | ({ mutation: "scaleTargetVolume" } & ScaleTargetVolume)
  | ({ mutation: "changeTargetVolumeHidden" } & ChangeTargetVolumeHidden)
  | ({ mutation: "changeTargetVolumeLocked" } & ChangeTargetVolumeLocked)
  | ({ mutation: "dragSelection2d" } & DragSelection2d)
  | ({ mutation: "dragSelection3d" } & DragSelection3d)
  | ({ mutation: "rotateSelection3d" } & RotateSelection3d)
  | ({ mutation: "scaleSelection3d" } & ScaleSelection3d);

/** ✋️ `drag-selection2d` payload — part ids moved on the board by one relative flat offset. */
export interface DragSelection2d {
  targets: string[];
  dx: Binary64;
  dy: Binary64;
}

/** 🚚️ `drag-selection3d` payload — part and target-volume ids moved in the world by one relative offset. */
export interface DragSelection3d {
  targets: string[];
  offset: Puzzle5dVector3;
}

/** 🔄️ `rotate-selection3d` payload — part and target-volume ids turned, each about its own origin, by `angle` radians about the world `axis`. */
export interface RotateSelection3d {
  targets: string[];
  axis: Puzzle5dVector3;
  angle: Binary64;
}

/** 🔍️ `scale-selection3d` payload — part and target-volume ids whose scales are multiplied per axis by `factors`. */
export interface ScaleSelection3d {
  targets: string[];
  factors: Puzzle5dVector3;
}
