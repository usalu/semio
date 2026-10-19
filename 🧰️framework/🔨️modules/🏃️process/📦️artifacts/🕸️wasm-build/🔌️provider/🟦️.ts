import { defineScriptProviderV1, loadScriptProviderV1 } from "../../../🔌️providers/🟦️.ts";
import type { ScriptInvocation } from "../../../🧭️routing/📥️invocation/🟦️.ts";
import type { WasmPackWebBuildOptions } from "../🟦️.ts";

/** 📨️ Carries one browser WASM package production request; the provider completes it with its selected storage, toolchain and dependency authority. */
export type WasmWebBuildRequestV1 = Omit<WasmPackWebBuildOptions, "storage" | "environment" | "cargoConfigurationSelectors" | "withCargoInputs"> & Readonly<{ invocation: ScriptInvocation; producer: "browser-canvas" | "browser-module" }>;

/** 🔌️ Produces and captures browser WASM packages under the authority an issuing parent selected for the whole script process. */
export interface WasmBuildProviderV1 {
  buildWeb(request: WasmWebBuildRequestV1): Promise<void>;
}

/** 🧷️ Names the port and the exact functions its implementation must expose. */
export const WASM_BUILD_PROVIDER_V1 = defineScriptProviderV1<WasmBuildProviderV1>("wasm-build", ["buildWeb"]);

/** 🕸️ Builds one browser WASM package and runs the declared consumer over the captured package. */
export async function buildAdmittedWasmWebV1(request: WasmWebBuildRequestV1): Promise<void> {
  return (await loadScriptProviderV1(WASM_BUILD_PROVIDER_V1)).buildWeb(request);
}
