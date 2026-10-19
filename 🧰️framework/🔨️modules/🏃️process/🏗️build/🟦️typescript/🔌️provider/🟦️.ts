import { defineScriptProviderV1, loadScriptProviderV1 } from "../../../🔌️providers/🟦️.ts";
import type { TypeScriptCheckOptions } from "../🟦️.ts";

/** 📨️ Carries one TypeScript project check; the provider completes it with its selected compiler state, storage and build mode. */
export type TypeScriptProjectCheckRequestV1 = Omit<TypeScriptCheckOptions, "stateRoot" | "storage" | "storageControl" | "mode">;

/** 🔌️ Checks TypeScript projects under the authority an issuing parent selected for the whole script process. */
export interface TypeScriptCheckProviderV1 {
  typecheck(request: TypeScriptProjectCheckRequestV1): Promise<void>;
}

/** 🧷️ Names the port and the exact functions its implementation must expose. */
export const TYPESCRIPT_CHECK_PROVIDER_V1 = defineScriptProviderV1<TypeScriptCheckProviderV1>("typescript-check", ["typecheck"]);

/** 🟦️ Checks one project with compiler state confined to the provider's bounded allocation. */
export async function typecheckAdmittedProjectV1(request: TypeScriptProjectCheckRequestV1): Promise<void> {
  return (await loadScriptProviderV1(TYPESCRIPT_CHECK_PROVIDER_V1)).typecheck(request);
}
