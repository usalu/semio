/** 🧭️ The repository root of the dev server's Vite plugins. */
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

/** 🗂️ Repository root derived from this module's own location — the config bundler must not
 * reach `getWorkspaceRoot` (and the discovery walk behind it) just to place two dev databases. */
export const REPO_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../..");
