/** 🧬️ S Space index mutation vocabulary — TS twin of `🧬️mutations/🦀️component.rs`. */
import type { SpaceArtifactRow } from "../📸️snapshot/🟦️component.ts";

export interface CreateArtifact { mutation: "createArtifact"; artifact: SpaceArtifactRow }
export interface DeleteArtifact { mutation: "deleteArtifact"; id: string }
export interface RenameArtifact { mutation: "renameArtifact"; id: string; newName: string }
export interface TouchArtifact { mutation: "touchArtifact"; id: string; updatedAtMs: number; updatedBy: string }

export type SSpaceMutation = CreateArtifact | DeleteArtifact | RenameArtifact | TouchArtifact;
