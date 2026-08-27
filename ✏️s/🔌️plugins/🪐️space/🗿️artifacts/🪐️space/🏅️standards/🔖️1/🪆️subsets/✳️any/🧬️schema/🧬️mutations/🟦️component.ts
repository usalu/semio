/** 🪐️ S Space direct-mutation discriminated union. */
import type { CreateArtifact } from "./🌱create-artifact/🟦️component.ts";
import type { DeleteArtifact } from "./🗑️delete-artifact/🟦️component.ts";
import type { RenameArtifact } from "./🏷️rename-artifact/🟦️component.ts";
import type { TouchArtifact } from "./🕒touch-artifact/🟦️component.ts";

export type SSpaceMutation =
  | ({ mutation: "createArtifact" } & CreateArtifact)
  | ({ mutation: "deleteArtifact" } & DeleteArtifact)
  | ({ mutation: "renameArtifact" } & RenameArtifact)
  | ({ mutation: "touchArtifact" } & TouchArtifact);
