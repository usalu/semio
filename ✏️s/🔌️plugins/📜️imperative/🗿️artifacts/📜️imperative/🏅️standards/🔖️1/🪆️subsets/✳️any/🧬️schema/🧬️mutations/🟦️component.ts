/** 📜️ Imperative direct-mutation discriminated union. */
import type { CreateStep } from "./🌱create-step/🟦️component.ts";
import type { DeleteStep } from "./🗑️delete-step/🟦️component.ts";
import type { EditStepParams } from "./🔧edit-step-params/🟦️component.ts";
import type { ReorderSteps } from "./🔀reorder-steps/🟦️component.ts";

export type ImperativeMutation =
  | ({ mutation: "createStep" } & CreateStep)
  | ({ mutation: "deleteStep" } & DeleteStep)
  | ({ mutation: "reorderSteps" } & ReorderSteps)
  | ({ mutation: "editStepParams" } & EditStepParams);
