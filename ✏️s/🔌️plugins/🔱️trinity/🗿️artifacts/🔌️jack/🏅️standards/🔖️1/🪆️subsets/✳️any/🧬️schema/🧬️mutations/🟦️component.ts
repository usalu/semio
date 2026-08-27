/** 🧩️ Jack direct-mutation discriminated union. */
import type { ChangeDataProperty } from "./🔧️change-data-property/🟦️component.ts";
import type { CreateEdge } from "./🔗️create-edge/🟦️component.ts";
import type { CreateNode } from "./🌱️create-node/🟦️component.ts";
import type { DeleteEdge } from "./✂️delete-edge/🟦️component.ts";
import type { DeleteNode } from "./🗑️delete-node/🟦️component.ts";
import type { MoveNode } from "./📍️move-node/🟦️component.ts";
import type { RemoveDataProperty } from "./🧹️remove-data-property/🟦️component.ts";
import type { RenameNode } from "./✏️rename-node/🟦️component.ts";

export type JackMutation =
  | ({ mutation: "changeDataProperty" } & ChangeDataProperty)
  | ({ mutation: "createEdge" } & CreateEdge)
  | ({ mutation: "createNode" } & CreateNode)
  | ({ mutation: "deleteEdge" } & DeleteEdge)
  | ({ mutation: "deleteNode" } & DeleteNode)
  | ({ mutation: "moveNode" } & MoveNode)
  | ({ mutation: "removeDataProperty" } & RemoveDataProperty)
  | ({ mutation: "renameNode" } & RenameNode);
