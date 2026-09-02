/** 🪐️ S Space direct-mutation discriminated union. */
import type { CreateArtifact } from "./🌱create-artifact/🟦️.ts";
import type { DeleteArtifact } from "./🗑️delete-artifact/🟦️.ts";
import type { RenameArtifact } from "./🏷️rename-artifact/🟦️.ts";
import type { TouchArtifact } from "./🕒touch-artifact/🟦️.ts";

export type SSpaceMutation =
  | ({ mutation: "createArtifact" } & CreateArtifact)
  | ({ mutation: "deleteArtifact" } & DeleteArtifact)
  | ({ mutation: "renameArtifact" } & RenameArtifact)
  | ({ mutation: "touchArtifact" } & TouchArtifact);
