import { defineScriptProviderV1, loadScriptProviderV1 } from "../🔌️providers/🟦️.ts";
import type { CompilerAdmissionControl } from "../🏗️build/🎟️capacity/🟦️.ts";
import type { ScriptInvocation } from "../🧭️routing/📥️invocation/🟦️.ts";

/** 📨️ Carries one extension package request: the issued compiler admission, the declared extension component directory and the workspace it belongs to. */
export type ExtensionPackageRequestV1 = Readonly<{ invocation: ScriptInvocation; control: CompilerAdmissionControl; componentDir: string; repoRoot: string }>;
/** 🧾️ Names the published extension package: its absolute path and the owner it was published under. */
export type ExtensionPackageReceiptV1 = Readonly<{ path: string; owner: string }>;

/** 🔌️ Builds one declared extension component and publishes its package into the managed product store an issuing parent selected for the whole script process. */
export interface ExtensionPackageProviderV1 {
  packageComponent(request: ExtensionPackageRequestV1): Promise<ExtensionPackageReceiptV1>;
}

/** 🧷️ Names the port and the exact functions its implementation must expose. */
export const EXTENSION_PACKAGE_PROVIDER_V1 = defineScriptProviderV1<ExtensionPackageProviderV1>("extension-package", ["packageComponent"]);

/** 🎁️ Packages the declared extension component with the designated implementation. */
export async function packageAdmittedExtensionComponentV1(request: ExtensionPackageRequestV1): Promise<ExtensionPackageReceiptV1> {
  return (await loadScriptProviderV1(EXTENSION_PACKAGE_PROVIDER_V1)).packageComponent(request);
}
