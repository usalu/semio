import { join } from "node:path";

export const SEMIO_ROOT_DIR = ".🧬semio";
export const REPO_META_DIR_NAME = "🦑️repo";

/** 🧬️ Locates the workspace-local semio root. */
export function getSemioRoot(repoRoot: string): string {
  return join(repoRoot, SEMIO_ROOT_DIR);
}

/** 🦑️ Locates the repository metadata owned by this workspace. */
export function getRepoMetaDir(repoRoot: string): string {
  return join(getSemioRoot(repoRoot), REPO_META_DIR_NAME);
}
