/** 📍️ The checkout root of this repository, resolved from this module's own location and never from the working directory.
 * @see ../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts
 */
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { findWorkspaceRoot } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

export const WORKSPACE_ROOT = findWorkspaceRoot(dirname(fileURLToPath(import.meta.url)));
