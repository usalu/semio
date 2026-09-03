/** 🧬️ block2d 🧬️mutations facade — the real per-kind dispatch union (26 kinds), mirroring
 * `🦀️.rs`'s `Block2dMutation` enum. Not a snapshot mirror.
 */

export type { RenameNodeKind } from "./✏️rename-node-kind/🟦️";
export type { ChangeNodeKindLabel } from "./🏷️change-node-kind-label/🟦️";
export type { ChangeNodeKindVariant } from "./🔀️change-node-kind-variant/🟦️";
export type { ChangeNodeKindDescription } from "./📃️change-node-kind-description/🟦️";
export type { ChangeNodeKindIcon } from "./🖼️change-node-kind-icon/🟦️";
export type { ChangeNodeKindUnit } from "./📐️change-node-kind-unit/🟦️";
export type { UpdatePresentation } from "./🖌️update-presentation/🟦️";
export type { CreateHandleKind } from "./🌱️create-handle-kind/🟦️";
export type { DeleteHandleKind } from "./🗑️delete-handle-kind/🟦️";
export type { RenameHandleKind } from "./✒️rename-handle-kind/🟦️";
export type { ChangeHandleKindLabel } from "./🔖️change-handle-kind-label/🟦️";
export type { ChangeHandleKindColor } from "./🎨️change-handle-kind-color/🟦️";
export type { ChangeHandleKindDefaultWireKind } from "./🔌️change-handle-kind-default-wire-kind/🟦️";
export type { CreateHandle } from "./🌿️create-handle/🟦️";
export type { DeleteHandle } from "./❌️delete-handle/🟦️";
export type { MoveHandle } from "./📍️move-handle/🟦️";
export type { ChangeHandleHandleKind } from "./🧷️change-handle-handle-kind/🟦️";
export type { AddCompatibilityRule } from "./➕️add-compatibility-rule/🟦️";
export type { RemoveCompatibilityRule } from "./➖️remove-compatibility-rule/🟦️";
export type { AddAttribute } from "./🧩️add-attribute/🟦️";
export type { RemoveAttribute } from "./🚫️remove-attribute/🟦️";
export type { AddAuthor } from "./👤️add-author/🟦️";
export type { RemoveAuthor } from "./🚷️remove-author/🟦️";
export type { MoveCamera2d } from "./🎥️move-camera2d/🟦️";
export type { ScaleCamera2d } from "./🔍️scale-camera2d/🟦️";
export type { ChangeMetaDescription } from "./💬️change-meta-description/🟦️";

export type Block2dMutation =
  | { mutation: "renameNodeKind" } & import("./✏️rename-node-kind/🟦️").RenameNodeKind
  | { mutation: "changeNodeKindLabel" } & import("./🏷️change-node-kind-label/🟦️").ChangeNodeKindLabel
  | { mutation: "changeNodeKindVariant" } & import("./🔀️change-node-kind-variant/🟦️").ChangeNodeKindVariant
  | { mutation: "changeNodeKindDescription" } & import("./📃️change-node-kind-description/🟦️").ChangeNodeKindDescription
  | { mutation: "changeNodeKindIcon" } & import("./🖼️change-node-kind-icon/🟦️").ChangeNodeKindIcon
  | { mutation: "changeNodeKindUnit" } & import("./📐️change-node-kind-unit/🟦️").ChangeNodeKindUnit
  | { mutation: "updatePresentation" } & import("./🖌️update-presentation/🟦️").UpdatePresentation
  | { mutation: "createHandleKind" } & import("./🌱️create-handle-kind/🟦️").CreateHandleKind
  | { mutation: "deleteHandleKind" } & import("./🗑️delete-handle-kind/🟦️").DeleteHandleKind
  | { mutation: "renameHandleKind" } & import("./✒️rename-handle-kind/🟦️").RenameHandleKind
  | { mutation: "changeHandleKindLabel" } & import("./🔖️change-handle-kind-label/🟦️").ChangeHandleKindLabel
  | { mutation: "changeHandleKindColor" } & import("./🎨️change-handle-kind-color/🟦️").ChangeHandleKindColor
  | { mutation: "changeHandleKindDefaultWireKind" } & import("./🔌️change-handle-kind-default-wire-kind/🟦️").ChangeHandleKindDefaultWireKind
  | { mutation: "createHandle" } & import("./🌿️create-handle/🟦️").CreateHandle
  | { mutation: "deleteHandle" } & import("./❌️delete-handle/🟦️").DeleteHandle
  | { mutation: "moveHandle" } & import("./📍️move-handle/🟦️").MoveHandle
  | { mutation: "changeHandleHandleKind" } & import("./🧷️change-handle-handle-kind/🟦️").ChangeHandleHandleKind
  | { mutation: "addCompatibilityRule" } & import("./➕️add-compatibility-rule/🟦️").AddCompatibilityRule
  | { mutation: "removeCompatibilityRule" } & import("./➖️remove-compatibility-rule/🟦️").RemoveCompatibilityRule
  | { mutation: "addAttribute" } & import("./🧩️add-attribute/🟦️").AddAttribute
  | { mutation: "removeAttribute" } & import("./🚫️remove-attribute/🟦️").RemoveAttribute
  | { mutation: "addAuthor" } & import("./👤️add-author/🟦️").AddAuthor
  | { mutation: "removeAuthor" } & import("./🚷️remove-author/🟦️").RemoveAuthor
  | { mutation: "moveCamera2d" } & import("./🎥️move-camera2d/🟦️").MoveCamera2d
  | { mutation: "scaleCamera2d" } & import("./🔍️scale-camera2d/🟦️").ScaleCamera2d
  | { mutation: "changeMetaDescription" } & import("./💬️change-meta-description/🟦️").ChangeMetaDescription;
