/** 🧬️ The owning `🧬️schema/` modules the renderer oracles judge against, and the strict draft-07 validator that compiles one named `$defs` export of each.
 * @see ../../🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts — the verbs that run the oracles */

import { readFileSync } from "node:fs";
import { join } from "node:path";
import type { ValidateFunction } from "ajv";
import { semioSchemaAjvV1 } from "../../../../../../../📦️packages/🟦️typescript/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";

const MODULE_SCHEMAS = {
  renderer: "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🔣️.json",
  interaction: "🧰️framework/🔨️modules/🕹️interaction/🧬️schema/🔣️.json",
  directory: "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json",
  presence: "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🧬️schema/🔣️.json",
} as const;

/** 🧬️ A strict draft-07 validator that knows the `x-semio-*` vendor annotation vocabulary (`💻️os/🧪️tests/🧬️schema-oracle`). */
const ownedAjv = () => semioSchemaAjvV1({ strict: true, allErrors: true });

/** 🧬️ Compiles one named `$defs` export of an owning `🧬️schema/` module against its draft-07 `$id`. */
export function ownedExport(repoRoot: string, scope: keyof typeof MODULE_SCHEMAS, exportId: string): ValidateFunction {
  const doc = JSON.parse(readFileSync(join(repoRoot, MODULE_SCHEMAS[scope]), "utf8")) as { $id: string };
  const compiled = ownedAjv().addSchema(doc).getSchema(`${doc.$id}#/$defs/${exportId}`);
  if (!compiled) throw new Error(`${scope} schema module publishes no export ${exportId}`);
  return compiled as ValidateFunction;
}
