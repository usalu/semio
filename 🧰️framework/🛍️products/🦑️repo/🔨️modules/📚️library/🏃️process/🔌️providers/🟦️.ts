import { fileURLToPath } from "node:url";
import { SCRIPT_PROVIDERS_ENV, writeScriptProviderMapV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔌️providers/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import designation from "./🔣️.json" with { type: "json" };
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🔌️ Designates, by framework port name, the repository modules that implement every port a framework script may request; the declaration `./🔣️.json` is also the source of the build graph inputs of every target whose script loads providers. */
export function repositoryScriptProvidersV1(): Readonly<Record<string, string>> {
  if (validateJsonSchemaSubset(schema, designation).length) throw Error("Complete repository script provider declaration required");
  return Object.freeze(Object.fromEntries(Object.entries(designation.providers).map(([name, path]) => [name, fileURLToPath(new URL(path, import.meta.url))])));
}

/** 📤️ Issues the repository provider designation into the environment every descendant script process inherits. */
export function issueRepositoryScriptProvidersV1(environment: NodeJS.ProcessEnv): void {
  const wire = writeScriptProviderMapV1(repositoryScriptProvidersV1());
  environment[SCRIPT_PROVIDERS_ENV] = wire;
  process.env[SCRIPT_PROVIDERS_ENV] = wire;
}
