import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

/** 🧭️ Declares the bounded Nx namespace of the repository (`nx.json` `cacheDirectory`) when the run carries none: the repository's patched Nx refuses to read that cache directory without `SEMIO_NX_FIXED_STORAGE` and `NX_CACHE_DIRECTORY`, so a law that loads Nx as its independent oracle names the same namespace under a bare `bun test ./file` as under the suite runner; a run that already declares either variable keeps its own. */
export function declareNxOracleNamespaceV1(root: string, environment: NodeJS.ProcessEnv = process.env): void {
  const declared = JSON.parse(readFileSync(join(root, "nx.json"), "utf8")).cacheDirectory;
  if (typeof declared !== "string" || environment.SEMIO_NX_FIXED_STORAGE !== undefined || environment.NX_CACHE_DIRECTORY !== undefined) return;
  const cache = resolve(root, declared);
  environment.NX_CACHE_DIRECTORY = cache;
  environment.SEMIO_NX_FIXED_STORAGE = dirname(dirname(cache));
}
