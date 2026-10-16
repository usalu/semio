/** 📋️ Exact output plan of the GraphQL type generation. */
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { emitTypeScript, type GraphqlTypesSource } from "../📽️projection/🟦️.ts";

//#region 📋️Plan
export type GeneratedTarget = { readonly path: string; readonly content: string };

/** 🤖️ The one generated module: `🧩️vscode/🤖️generated/🔗️graphql-types/🟦️.ts`. */
export function generatedTargets(source: GraphqlTypesSource): readonly GeneratedTarget[] {
  const vscodeRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
  return [{ path: join(vscodeRoot, "🤖️generated", "🔗️graphql-types", "🟦️.ts"), content: emitTypeScript(source) }];
}
//#endregion 📋️Plan
