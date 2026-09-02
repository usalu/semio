/** 🧬️ block2d 🧬️mutations facade — the real per-kind dispatch union (26 kinds), mirroring
 * `🦀️.rs`'s `Block2dMutation` enum. Not a snapshot mirror.
 */

export type { RenameNodeKind } from "./✏️rename-node-kind/🟦️component";
export type { ChangeNodeKindLabel } from "./🏷️change-node-kind-label/🟦️component";
export type { ChangeNodeKindVariant } from "./🔀️change-node-kind-variant/🟦️component";
export type { ChangeNodeKindDescription } from "./📃️change-node-kind-description/🟦️component";
export type { ChangeNodeKindIcon } from "./🖼️change-node-kind-icon/🟦️component";
export type { ChangeNodeKindUnit } from "./📐️change-node-kind-unit/🟦️component";
export type { UpdatePresentation } from "./🖌️update-presentation/🟦️component";
export type { CreateHandleKind } from "./🌱️create-handle-kind/🟦️component";
export type { DeleteHandleKind } from "./🗑️delete-handle-kind/🟦️component";
export type { RenameHandleKind } from "./✒️rename-handle-kind/🟦️component";
export type { ChangeHandleKindLabel } from "./🔖️change-handle-kind-label/🟦️component";
export type { ChangeHandleKindColor } from "./🎨️change-handle-kind-color/🟦️component";
export type { ChangeHandleKindDefaultWireKind } from "./🔌️change-handle-kind-default-wire-kind/🟦️component";
export type { CreateHandle } from "./🌿️create-handle/🟦️component";
export type { DeleteHandle } from "./❌️delete-handle/🟦️component";
export type { MoveHandle } from "./📍️move-handle/🟦️component";
export type { ChangeHandleHandleKind } from "./🧷️change-handle-handle-kind/🟦️component";
export type { AddCompatibilityRule } from "./➕️add-compatibility-rule/🟦️component";
export type { RemoveCompatibilityRule } from "./➖️remove-compatibility-rule/🟦️component";
export type { AddAttribute } from "./🧩️add-attribute/🟦️component";
export type { RemoveAttribute } from "./🚫️remove-attribute/🟦️component";
export type { AddAuthor } from "./👤️add-author/🟦️component";
export type { RemoveAuthor } from "./🚷️remove-author/🟦️component";
export type { MoveCamera2d } from "./🎥️move-camera2d/🟦️component";
export type { ScaleCamera2d } from "./🔍️scale-camera2d/🟦️component";
export type { ChangeMetaDescription } from "./💬️change-meta-description/🟦️component";

export type Block2dMutation =
  | { mutation: "renameNodeKind" } & import("./✏️rename-node-kind/🟦️component").RenameNodeKind
  | { mutation: "changeNodeKindLabel" } & import("./🏷️change-node-kind-label/🟦️component").ChangeNodeKindLabel
  | { mutation: "changeNodeKindVariant" } & import("./🔀️change-node-kind-variant/🟦️component").ChangeNodeKindVariant
  | { mutation: "changeNodeKindDescription" } & import("./📃️change-node-kind-description/🟦️component").ChangeNodeKindDescription
  | { mutation: "changeNodeKindIcon" } & import("./🖼️change-node-kind-icon/🟦️component").ChangeNodeKindIcon
  | { mutation: "changeNodeKindUnit" } & import("./📐️change-node-kind-unit/🟦️component").ChangeNodeKindUnit
  | { mutation: "updatePresentation" } & import("./🖌️update-presentation/🟦️component").UpdatePresentation
  | { mutation: "createHandleKind" } & import("./🌱️create-handle-kind/🟦️component").CreateHandleKind
  | { mutation: "deleteHandleKind" } & import("./🗑️delete-handle-kind/🟦️component").DeleteHandleKind
  | { mutation: "renameHandleKind" } & import("./✒️rename-handle-kind/🟦️component").RenameHandleKind
  | { mutation: "changeHandleKindLabel" } & import("./🔖️change-handle-kind-label/🟦️component").ChangeHandleKindLabel
  | { mutation: "changeHandleKindColor" } & import("./🎨️change-handle-kind-color/🟦️component").ChangeHandleKindColor
  | { mutation: "changeHandleKindDefaultWireKind" } & import("./🔌️change-handle-kind-default-wire-kind/🟦️component").ChangeHandleKindDefaultWireKind
  | { mutation: "createHandle" } & import("./🌿️create-handle/🟦️component").CreateHandle
  | { mutation: "deleteHandle" } & import("./❌️delete-handle/🟦️component").DeleteHandle
  | { mutation: "moveHandle" } & import("./📍️move-handle/🟦️component").MoveHandle
  | { mutation: "changeHandleHandleKind" } & import("./🧷️change-handle-handle-kind/🟦️component").ChangeHandleHandleKind
  | { mutation: "addCompatibilityRule" } & import("./➕️add-compatibility-rule/🟦️component").AddCompatibilityRule
  | { mutation: "removeCompatibilityRule" } & import("./➖️remove-compatibility-rule/🟦️component").RemoveCompatibilityRule
  | { mutation: "addAttribute" } & import("./🧩️add-attribute/🟦️component").AddAttribute
  | { mutation: "removeAttribute" } & import("./🚫️remove-attribute/🟦️component").RemoveAttribute
  | { mutation: "addAuthor" } & import("./👤️add-author/🟦️component").AddAuthor
  | { mutation: "removeAuthor" } & import("./🚷️remove-author/🟦️component").RemoveAuthor
  | { mutation: "moveCamera2d" } & import("./🎥️move-camera2d/🟦️component").MoveCamera2d
  | { mutation: "scaleCamera2d" } & import("./🔍️scale-camera2d/🟦️component").ScaleCamera2d
  | { mutation: "changeMetaDescription" } & import("./💬️change-meta-description/🟦️component").ChangeMetaDescription;
