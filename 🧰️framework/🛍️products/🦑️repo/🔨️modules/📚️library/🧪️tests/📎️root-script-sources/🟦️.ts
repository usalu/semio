/** 📎️ The text of the root `📜️script.ts` and of every module it owns, for tests that audit the source of the root task router. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { policyRootScriptModulePaths } from "../../⚖️laws/🧩️plugin-consistency/🏛️foundation/🟦️.ts";

/** 📖️ Repository-relative path and text of the root script and of every module it owns. */
export function rootScriptSources(repoRoot: string): { path: string; text: string }[] {
  return policyRootScriptModulePaths(repoRoot).map((path) => ({ path, text: readFileSync(join(repoRoot, path), "utf8") }));
}
