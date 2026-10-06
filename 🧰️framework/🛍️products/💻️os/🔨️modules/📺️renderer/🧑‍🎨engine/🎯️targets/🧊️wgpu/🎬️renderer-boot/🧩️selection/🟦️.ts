import { admitWgpuPluginModules, type WgpuPluginModule, type WgpuPluginRegistrySelection } from "../../🧩️plugin-modules/🛂️admission/🟦️.ts";
import { resolveWgpuBootDescriptor, type WgpuBootDescriptor, type WgpuBootOverrides } from "../../🧭️boot-descriptor/🟦️.ts";

/** 🧩️ Resolves one explicit admitted contribution registry and its existing boot contract. */
export function resolveEmbeddedWgpuBoot(input: { readonly modules: unknown; readonly baseUrl: string; readonly hash?: string; readonly overrides?: WgpuBootOverrides }): { readonly plugins: readonly WgpuPluginModule[]; readonly descriptor: WgpuBootDescriptor; readonly pluginRegistrySelection: WgpuPluginRegistrySelection } {
  const plugins = admitWgpuPluginModules(input.modules, input.baseUrl);
  const overrides = input.overrides ?? {};
  return { plugins, descriptor: resolveWgpuBootDescriptor({ hash: input.hash, defaultVariant: plugins[0]?.pluginId ?? "", overrides }), pluginRegistrySelection: overrides.plugin === undefined ? "all" : "variant" };
}
