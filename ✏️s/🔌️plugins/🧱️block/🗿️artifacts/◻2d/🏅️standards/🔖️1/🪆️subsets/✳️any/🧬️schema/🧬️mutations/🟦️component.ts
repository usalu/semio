/** 🧬️ block2d 🧬️mutations facade — the real per-kind dispatch union (26 kinds), mirroring
 * `🦀️component.rs`'s `Block2dMutation` enum. Not a snapshot mirror.
 */

export type { RenameNodeKind } from "./✏️rename-node-kind/🦠️mutation/🟦️component";
export type { ChangeNodeKindLabel } from "./🏷️change-node-kind-label/🦠️mutation/🟦️component";
export type { ChangeNodeKindVariant } from "./🔀️change-node-kind-variant/🦠️mutation/🟦️component";
export type { ChangeNodeKindDescription } from "./📃️change-node-kind-description/🦠️mutation/🟦️component";
export type { ChangeNodeKindIcon } from "./🖼️change-node-kind-icon/🦠️mutation/🟦️component";
export type { ChangeNodeKindUnit } from "./📐️change-node-kind-unit/🦠️mutation/🟦️component";
export type { UpdatePresentation } from "./🖌️update-presentation/🦠️mutation/🟦️component";
export type { CreateHandleKind } from "./🌱️create-handle-kind/🦠️mutation/🟦️component";
export type { DeleteHandleKind } from "./🗑️delete-handle-kind/🦠️mutation/🟦️component";
export type { RenameHandleKind } from "./✒️rename-handle-kind/🦠️mutation/🟦️component";
export type { ChangeHandleKindLabel } from "./🔖️change-handle-kind-label/🦠️mutation/🟦️component";
export type { ChangeHandleKindColor } from "./🎨️change-handle-kind-color/🦠️mutation/🟦️component";
export type { ChangeHandleKindDefaultWireKind } from "./🔌️change-handle-kind-default-wire-kind/🦠️mutation/🟦️component";
export type { CreateHandle } from "./🌿️create-handle/🦠️mutation/🟦️component";
export type { DeleteHandle } from "./❌️delete-handle/🦠️mutation/🟦️component";
export type { MoveHandle } from "./📍️move-handle/🦠️mutation/🟦️component";
export type { ChangeHandleHandleKind } from "./🧷️change-handle-handle-kind/🦠️mutation/🟦️component";
export type { AddCompatibilityRule } from "./➕️add-compatibility-rule/🦠️mutation/🟦️component";
export type { RemoveCompatibilityRule } from "./➖️remove-compatibility-rule/🦠️mutation/🟦️component";
export type { AddAttribute } from "./🧩️add-attribute/🦠️mutation/🟦️component";
export type { RemoveAttribute } from "./🚫️remove-attribute/🦠️mutation/🟦️component";
export type { AddAuthor } from "./👤️add-author/🦠️mutation/🟦️component";
export type { RemoveAuthor } from "./🚷️remove-author/🦠️mutation/🟦️component";
export type { MoveCamera2d } from "./🎥️move-camera2d/🦠️mutation/🟦️component";
export type { ScaleCamera2d } from "./🔍️scale-camera2d/🦠️mutation/🟦️component";
export type { ChangeMetaDescription } from "./💬️change-meta-description/🦠️mutation/🟦️component";

export type Block2dMutation =
  | { mutation: "rename-node-kind" } & import("./✏️rename-node-kind/🦠️mutation/🟦️component").RenameNodeKind
  | { mutation: "change-node-kind-label" } & import("./🏷️change-node-kind-label/🦠️mutation/🟦️component").ChangeNodeKindLabel
  | { mutation: "change-node-kind-variant" } & import("./🔀️change-node-kind-variant/🦠️mutation/🟦️component").ChangeNodeKindVariant
  | { mutation: "change-node-kind-description" } & import("./📃️change-node-kind-description/🦠️mutation/🟦️component").ChangeNodeKindDescription
  | { mutation: "change-node-kind-icon" } & import("./🖼️change-node-kind-icon/🦠️mutation/🟦️component").ChangeNodeKindIcon
  | { mutation: "change-node-kind-unit" } & import("./📐️change-node-kind-unit/🦠️mutation/🟦️component").ChangeNodeKindUnit
  | { mutation: "update-presentation" } & import("./🖌️update-presentation/🦠️mutation/🟦️component").UpdatePresentation
  | { mutation: "create-handle-kind" } & import("./🌱️create-handle-kind/🦠️mutation/🟦️component").CreateHandleKind
  | { mutation: "delete-handle-kind" } & import("./🗑️delete-handle-kind/🦠️mutation/🟦️component").DeleteHandleKind
  | { mutation: "rename-handle-kind" } & import("./✒️rename-handle-kind/🦠️mutation/🟦️component").RenameHandleKind
  | { mutation: "change-handle-kind-label" } & import("./🔖️change-handle-kind-label/🦠️mutation/🟦️component").ChangeHandleKindLabel
  | { mutation: "change-handle-kind-color" } & import("./🎨️change-handle-kind-color/🦠️mutation/🟦️component").ChangeHandleKindColor
  | { mutation: "change-handle-kind-default-wire-kind" } & import("./🔌️change-handle-kind-default-wire-kind/🦠️mutation/🟦️component").ChangeHandleKindDefaultWireKind
  | { mutation: "create-handle" } & import("./🌿️create-handle/🦠️mutation/🟦️component").CreateHandle
  | { mutation: "delete-handle" } & import("./❌️delete-handle/🦠️mutation/🟦️component").DeleteHandle
  | { mutation: "move-handle" } & import("./📍️move-handle/🦠️mutation/🟦️component").MoveHandle
  | { mutation: "change-handle-handle-kind" } & import("./🧷️change-handle-handle-kind/🦠️mutation/🟦️component").ChangeHandleHandleKind
  | { mutation: "add-compatibility-rule" } & import("./➕️add-compatibility-rule/🦠️mutation/🟦️component").AddCompatibilityRule
  | { mutation: "remove-compatibility-rule" } & import("./➖️remove-compatibility-rule/🦠️mutation/🟦️component").RemoveCompatibilityRule
  | { mutation: "add-attribute" } & import("./🧩️add-attribute/🦠️mutation/🟦️component").AddAttribute
  | { mutation: "remove-attribute" } & import("./🚫️remove-attribute/🦠️mutation/🟦️component").RemoveAttribute
  | { mutation: "add-author" } & import("./👤️add-author/🦠️mutation/🟦️component").AddAuthor
  | { mutation: "remove-author" } & import("./🚷️remove-author/🦠️mutation/🟦️component").RemoveAuthor
  | { mutation: "move-camera2d" } & import("./🎥️move-camera2d/🦠️mutation/🟦️component").MoveCamera2d
  | { mutation: "scale-camera2d" } & import("./🔍️scale-camera2d/🦠️mutation/🟦️component").ScaleCamera2d
  | { mutation: "change-meta-description" } & import("./💬️change-meta-description/🦠️mutation/🟦️component").ChangeMetaDescription;
