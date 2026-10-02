import { existsSync, readFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";
import { stageArtifacts, type ArtifactPublicationOptions } from "../../../../../../🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts";

function retiredCargoOwner(owner: string, root: string): boolean {
  const split = owner.indexOf(":browser:");
  if (split <= 0) return false;
  const relative = owner.slice(0, split);
  if (!relative.endsWith("Cargo.toml") || relative.startsWith("/") || relative.split("/").includes("..")) return false;
  return !existsSync(join(root, relative));
}

/** 📦️ Supplies the Repo publication store and replaces a tree whose Cargo owner manifest is gone. */
export async function stageRepositoryArtifacts(staging:string,owner:string,files:ReadonlyMap<string,string>,options:Partial<ArtifactPublicationOptions>={}):Promise<void>{
  const root = getWorkspaceRoot();
  const marker = join(staging, ".nx-artifact.json");
  if (existsSync(marker)) {
    let recorded = "";
    try { const parsed = JSON.parse(readFileSync(marker, "utf8")).owner; if (typeof parsed === "string") recorded = parsed; } catch { recorded = ""; }
    if (recorded !== owner && retiredCargoOwner(recorded, root)) rmSync(staging, { recursive: true, force: true });
  }
  await stageArtifacts(staging,owner,files,{...options,leaseDirectory:options.leaseDirectory??join(root,".🧬semio/🦑️repo/⚡️cache/agents/resource-leases")});
}
