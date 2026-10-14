import { join } from "node:path";

export const SEMIO_ROOT_DIR = ".🧬semio";
export const REPO_META_DIR_NAME = "🦑️repo";
export const HUB_DATA_DIR_NAME="🌐hub";
export const SPACE_DATA_DIR_NAME="🔗space";
export const MAP_CACHE_DIR_NAME="🗺️map";
export const TICKET_GENERATED_OUTPUT_DIRECTORY="🗑️temp";

/** 🧬️ Locates the workspace-local semio root. */
export function getSemioRoot(repoRoot: string): string {
  return join(repoRoot, SEMIO_ROOT_DIR);
}

/** 🦑️ Locates the repository metadata owned by this workspace. */
export function getRepoMetaDir(repoRoot: string): string {
  return join(getSemioRoot(repoRoot), REPO_META_DIR_NAME);
}
