import { join } from "node:path";
import { getSemioRoot, HUB_DATA_DIR_NAME, MAP_CACHE_DIR_NAME, SPACE_DATA_DIR_NAME } from "../🧬️metadata/🟦️.ts";

/** 🌐Hub data directory (`.🧬semio/🌐hub/`). */
export function getHubDataDir(repoRoot: string): string {
  return join(getSemioRoot(repoRoot), HUB_DATA_DIR_NAME);
}

/** 🔗Space instance data directory (`.🧬semio/🔗space/<instance>/`). */
export function getSpaceDataDir(repoRoot: string, instance?: string): string {
  const base = join(getSemioRoot(repoRoot), SPACE_DATA_DIR_NAME);
  return instance ? join(base, instance) : base;
}

/** 🗺️Per-provider map tile cache (`.🧬semio/🗺️map/<provider>/`). */
export function getMapCacheDir(repoRoot: string, provider: string): string {
  return join(getSemioRoot(repoRoot), MAP_CACHE_DIR_NAME, provider);
}
