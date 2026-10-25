import type { ExtensionPackageProviderV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧩️extension-package/🟦️.ts";
import { runExtensionComponentPackage } from "../🟦️.ts";

/** 🔌️ Implements the framework extension package port with the plugin store's actual component model and funded publication. */
export const provider: ExtensionPackageProviderV1 = { packageComponent: runExtensionComponentPackage };
