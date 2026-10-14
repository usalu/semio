import schema from "./🧬️schema/🔣️.json";
import { join } from "node:path";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";
import { stageArtifacts, type ArtifactPublicationOptions } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/📤️publication/🟦️.ts";

/** 📦️ Repository publication shares the canonical consumer store and retains explicit ownership. */
export async function stageRepositoryArtifacts(staging:string,owner:string,files:ReadonlyMap<string,string>,options:Partial<ArtifactPublicationOptions>={}):Promise<void>{
  const root = getWorkspaceRoot();
  await stageArtifacts(staging,owner,files,{...options,leaseDirectory:options.leaseDirectory??join(root,schema.const.publicationStore)});
}
