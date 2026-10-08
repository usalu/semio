/** 🧬️ Puzzle3d direct-mutation discriminated union — mirrors `Puzzle3dMutation` field-for-field. */
import type {
  Puzzle3dCatalogObjectKind,
  Puzzle3dCatalogVortexKind,
  Puzzle3dObject,
  Puzzle3dObjectAnchor,
  Puzzle3dCompatSpecificity,
  Puzzle3dReference,
  Puzzle3dTargetVolume,
  Puzzle3dVortex,
} from "../📸️snapshot/🟦️.ts";

import type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import type {Puzzle3dScale,Puzzle3dReferenceSource,Puzzle3dKindCatalogs,Puzzle3dVector3,Puzzle3dVector4} from "../📸️snapshot/🟦️.ts";
export type {Puzzle3dScale,Puzzle3dReferenceSource,Puzzle3dKindCatalogs,Puzzle3dCatalogCableKind,Puzzle3dCatalogAttractionKind} from "../📸️snapshot/🟦️.ts";

export type Puzzle3dMutation =
  | ({ mutation: "createObject" } & CreateObject)
  | ({ mutation: "deleteObject" } & DeleteObject)
  | ({ mutation: "moveObject" } & MoveObject)
  | ({ mutation: "rotateObject" } & RotateObject)
  | ({ mutation: "scaleObject" } & ScaleObject)
  | ({ mutation: "changeObjectMesh" } & ChangeObjectMesh)
  | ({ mutation: "editObjectLabel" } & EditObjectLabel)
  | ({ mutation: "changeObjectKind" } & ChangeObjectKind)
  | ({ mutation: "changeObjectAnchor" } & ChangeObjectAnchor)
  | ({ mutation: "changeObjectHidden" } & ChangeObjectHidden)
  | ({ mutation: "changeObjectLocked" } & ChangeObjectLocked)
  | ({ mutation: "addObjectVortex" } & AddObjectVortex)
  | ({ mutation: "removeObjectVortex" } & RemoveObjectVortex)
  | ({ mutation: "replaceObjectVortex" } & ReplaceObjectVortex)
  | ({ mutation: "connectVortices" } & ConnectVortices)
  | ({ mutation: "disconnectVortices" } & DisconnectVortices)
  | ({ mutation: "replaceAttractionGeometry" } & ReplaceAttractionGeometry)
  | ({ mutation: "createTargetVolume" } & CreateTargetVolume)
  | ({ mutation: "deleteTargetVolume" } & DeleteTargetVolume)
  | ({ mutation: "moveTargetVolume" } & MoveTargetVolume)
  | ({ mutation: "rotateTargetVolume" } & RotateTargetVolume)
  | ({ mutation: "scaleTargetVolume" } & ScaleTargetVolume)
  | ({ mutation: "changeTargetVolumeHidden" } & ChangeTargetVolumeHidden)
  | ({ mutation: "changeTargetVolumeLocked" } & ChangeTargetVolumeLocked)
  | ({ mutation: "createReference" } & CreateReference)
  | ({ mutation: "deleteReference" } & DeleteReference)
  | ({ mutation: "moveReference" } & MoveReference)
  | ({ mutation: "resizeReference" } & ResizeReference)
  | ({ mutation: "replaceReferenceSource" } & ReplaceReferenceSource)
  | ({ mutation: "changeReferenceHidden" } & ChangeReferenceHidden)
  | ({ mutation: "changeReferenceLocked" } & ChangeReferenceLocked)
  | ({ mutation: "changeDomain" } & ChangeDomain)
  | ({ mutation: "connectKindCompatibility" } & ConnectKindCompatibility)
  | ({ mutation: "disconnectKindCompatibility" } & DisconnectKindCompatibility)
  | ({ mutation: "replaceKindCatalogs" } & ReplaceKindCatalogs)
  | ({ mutation: "dragSelection" } & DragSelection)
  | ({ mutation: "rotateSelection" } & RotateSelection)
  | ({ mutation: "scaleSelection" } & ScaleSelection);

/** 🌱 `create-object` payload — full initial payload at an optional FINAL-state `index` (`null` appends). */
export interface CreateObject {
  object: Puzzle3dObject;
  index: bigint | null;
}

/** 🗑 `delete-object` payload. */
export interface DeleteObject {
  id: string;
}

/** 📍 `move-object` payload — absolute reposition of an object's origin. */
export interface MoveObject {
  id: string;
  newOrigin: Puzzle3dVector3;
}

/** 🔃 `rotate-object` payload — changes an object's orientation quaternion. */
export interface RotateObject {
  id: string;
  newOrientation: Puzzle3dVector4 | null;
}

/** 📏 `scale-object` payload — changes an object's freeform pose scale. */
export interface ScaleObject {
  id: string;
  newScale: Puzzle3dScale | null;
}

/** 🧱 `change-object-mesh` payload — changes an object's geometry reference. */
export interface ChangeObjectMesh {
  id: string;
  newMeshUrl: string | null;
}

/** 🖋️ `edit-object-label` payload — replaces an object's authored display label. */
export interface EditObjectLabel {
  id: string;
  newLabel: string | null;
}

/** 🏗 `change-object-kind` payload — changes an object's `objectKind` catalog reference. */
export interface ChangeObjectKind {
  id: string;
  newObjectKind: string | null;
}

/** ⚓ `change-object-anchor` payload — changes whether a root object keeps its stored plane or resets to default XY. */
export interface ChangeObjectAnchor {
  id: string;
  newAnchor: Puzzle3dObjectAnchor;
}

/** 👁 `change-object-hidden` payload — changes an object's hidden flag. */
export interface ChangeObjectHidden {
  id: string;
  newHidden: boolean;
}

/** 🔒 `change-object-locked` payload — changes an object's locked flag. */
export interface ChangeObjectLocked {
  id: string;
  newLocked: boolean;
}

/** ➕ `add-object-vortex` payload — attaches a new rim vortex to an object at an optional FINAL-state `index` (`null` appends). */
export interface AddObjectVortex {
  objectId: string;
  vortex: Puzzle3dVortex;
  index: bigint | null;
}

/** ➖ `remove-object-vortex` payload — detaches a rim vortex from an object. */
export interface RemoveObjectVortex {
  objectId: string;
  vortexId: string;
}

/** 🔌 `replace-object-vortex` payload — whole-value swap of one vortex's presentation fields. */
export interface ReplaceObjectVortex {
  objectId: string;
  vortexId: string;
  newVortex: Puzzle3dVortex;
}

/** 🔗 `connect-vortices` payload — creates an attraction between two full vortex ids, full initial connection-parameterization payload included. */
export interface ConnectVortices {
  id: string;
  attracting: string;
  attracted: string;
  gap: Binary64;
  shift: Binary64;
  rise: Binary64;
  rotation: Binary64;
  turn: Binary64;
  tilt: Binary64;
  x: Binary64;
  y: Binary64;
  index?: number | null;
}

/** ✂️ `disconnect-vortices` payload — removes an attraction between two vortices. */
export interface DisconnectVortices {
  id: string;
}

/** 🧮 `replace-attraction-geometry` payload — whole-value swap of an attraction's pose-solver connection pose. */
export interface ReplaceAttractionGeometry {
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

/** 🌍 `create-target-volume` payload — full initial payload at an optional FINAL-state `index` (`null` appends). */
export interface CreateTargetVolume {
  targetVolume: Puzzle3dTargetVolume;
  index: bigint | null;
}

/** 🪦 `delete-target-volume` payload. */
export interface DeleteTargetVolume {
  id: string;
}

/** 🚀 `move-target-volume` payload — absolute reposition of a target volume's origin. */
export interface MoveTargetVolume {
  id: string;
  newOrigin: Puzzle3dVector3;
}

/** 🌀 `rotate-target-volume` payload — changes a target volume's orientation quaternion. */
export interface RotateTargetVolume {
  id: string;
  newOrientation: Puzzle3dVector4 | null;
}

/** 📐 `scale-target-volume` payload — changes a target volume's freeform pose scale. */
export interface ScaleTargetVolume {
  id: string;
  newScale: Puzzle3dScale | null;
}

/** 🙈 `change-target-volume-hidden` payload — changes a target volume's hidden flag. */
export interface ChangeTargetVolumeHidden {
  id: string;
  newHidden: boolean;
}

/** 🔐 `change-target-volume-locked` payload — changes a target volume's locked flag. */
export interface ChangeTargetVolumeLocked {
  id: string;
  newLocked: boolean;
}

/** 🖼 `create-reference` payload — full initial payload at an optional FINAL-state `index` (`null` appends). */
export interface CreateReference {
  reference: Puzzle3dReference;
  index: bigint | null;
}

/** 🚮 `delete-reference` payload. */
export interface DeleteReference {
  id: string;
}

/** 🎯 `move-reference` payload — absolute reposition of a reference plane's pinned origin. */
export interface MoveReference {
  id: string;
  newOrigin: Puzzle3dVector3;
}

/** 📎 `resize-reference` payload — changes a reference plane's world-space width. */
export interface ResizeReference {
  id: string;
  newWidthWorld: Binary64;
}

/** 🖇 `replace-reference-source` payload — whole-value swap of a reference's media source. */
export interface ReplaceReferenceSource {
  id: string;
  newSource: Puzzle3dReferenceSource;
}

/** 👀 `change-reference-hidden` payload — changes a reference plane's hidden flag. */
export interface ChangeReferenceHidden {
  id: string;
  newHidden: boolean;
}

/** 🗝 `change-reference-locked` payload — changes a reference plane's locked flag. */
export interface ChangeReferenceLocked {
  id: string;
  newLocked: boolean;
}

/** 🌐 `change-domain` payload — changes the document's design domain classification. */
export interface ChangeDomain {
  newDomain: string;
}

/** 🤝 `connect-kind-compatibility` payload — allows one vortex-kind-id pair to attract. */
export interface ConnectKindCompatibility {
  source: string;
  target: string;
  bidirectional: boolean;
  important: boolean;
  specificity: Puzzle3dCompatSpecificity;
  index?: number | null;
}

/** 💔 `disconnect-kind-compatibility` payload — revokes one vortex-kind-id pair's attraction allowance. */
export interface DisconnectKindCompatibility {
  source: string;
  target: string;
}

/** 📚 `replace-kind-catalogs` payload — whole-value swap of the fixture-carried typed kind-catalog bundle (`null` clears the catalogs). */
export interface ReplaceKindCatalogs {
  newCatalogs: Puzzle3dKindCatalogs | null;
}

/** ✋️ `drag-selection` payload — object and target-volume ids moved by one relative world offset. */
export interface DragSelection {
  targets: string[];
  offset: Puzzle3dVector3;
}

/** 🔄️ `rotate-selection` payload — object and target-volume ids turned, each about its own origin, by `angle` radians about the world `axis`. */
export interface RotateSelection {
  targets: string[];
  axis: Puzzle3dVector3;
  angle: Binary64;
}

/** 🔍️ `scale-selection` payload — object and target-volume ids whose scales are multiplied per axis by `factors`. */
export interface ScaleSelection {
  targets: string[];
  factors: Puzzle3dVector3;
}
