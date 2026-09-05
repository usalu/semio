/** 🧩️ Jack direct-mutation discriminated union. */
import type { ChangeDataProperty } from "./🔧️change-data-property/🟦️.ts";
import type { CreateEdge } from "./🌉️create-edge/🟦️.ts";
import type { CreateNode } from "./➕️create-node/🟦️.ts";
import type { DeleteEdge } from "./✂️delete-edge/🟦️.ts";
import type { DeleteNode } from "./🗑️delete-node/🟦️.ts";
import type { MoveNode } from "./📍️move-node/🟦️.ts";
import type { RemoveDataProperty } from "./🧹️remove-data-property/🟦️.ts";
import type { RenameNode } from "./✏️rename-node/🟦️.ts";

export type JackMutation =
  | ({ mutation: "changeDataProperty" } & ChangeDataProperty)
  | ({ mutation: "createEdge" } & CreateEdge)
  | ({ mutation: "createNode" } & CreateNode)
  | ({ mutation: "deleteEdge" } & DeleteEdge)
  | ({ mutation: "deleteNode" } & DeleteNode)
  | ({ mutation: "moveNode" } & MoveNode)
  | ({ mutation: "removeDataProperty" } & RemoveDataProperty)
  | ({ mutation: "renameNode" } & RenameNode);
