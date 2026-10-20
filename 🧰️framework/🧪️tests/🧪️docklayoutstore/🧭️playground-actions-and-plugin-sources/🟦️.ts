import { registerPlaygroundResolutionTests } from "./🔶️playground-resolution/🟦️.ts";
import { registerPluginSourceTests } from "./🔸️plugin-source/🟦️.ts";
import { registerLeasePoolEvictNowTests } from "./🟠️lease-pool-evict-now/🟦️.ts";

export async function registerPlaygroundActionAndPluginSourceTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: typeof import("@semio-tech/framework")): Promise<void> {
  const { describe, it, createDevPluginSource, createExtensionSource, SYNTHETIC_PLUGIN_CATALOG, expect, vi, PluginModuleUnavailableError, extensionSourceEventToPluginSourceEvent, multiplexPluginSources, createLeasePool } = registerPlaygroundResolutionTests({ dependencies, vitest });
  registerPluginSourceTests({ describe, it, createDevPluginSource, createExtensionSource, SYNTHETIC_PLUGIN_CATALOG, expect, vi, PluginModuleUnavailableError, extensionSourceEventToPluginSourceEvent, multiplexPluginSources });
  registerLeasePoolEvictNowTests({ describe, it, createLeasePool, expect });
}
