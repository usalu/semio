/** 📜️ Imperative direct-mutation discriminated union. */
import type { CreateStep } from "./🌱create-step/🟦️.ts";
import type { DeleteStep } from "./🗑️delete-step/🟦️.ts";
import type { EditStepParams } from "./🔧edit-step-params/🟦️.ts";
import type { ReorderSteps } from "./🔀reorder-steps/🟦️.ts";

export type ProcedureMutation =
  | ({ mutation: "createStep" } & CreateStep)
  | ({ mutation: "deleteStep" } & DeleteStep)
  | ({ mutation: "reorderSteps" } & ReorderSteps)
  | ({ mutation: "editStepParams" } & EditStepParams);
