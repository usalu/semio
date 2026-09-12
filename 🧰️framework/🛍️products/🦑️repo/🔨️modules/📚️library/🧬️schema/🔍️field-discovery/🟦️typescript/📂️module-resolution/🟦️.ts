import { existsSync, readFileSync, statSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { policyExtractTypescriptSchemaFields } from "../🟦️.ts";
import type { PolicySchemaLeafExtract } from "../../🧱️contract/🟦️.ts";

/** 📂 Resolves relative TypeScript schema modules without executing their code. */
export function policyExtractTypescriptSchemaFile(abs: string, text: string, expectedTypeName: string | null): PolicySchemaLeafExtract {
  return policyExtractTypescriptSchemaFields(text, expectedTypeName, (specifier, from) => {
    if (!specifier.startsWith(".")) return null;
    const moduleId = resolve(dirname(from), specifier);
    return existsSync(moduleId) && statSync(moduleId).isFile() ? { moduleId, text: readFileSync(moduleId, "utf8") } : null;
  }, abs);
}
