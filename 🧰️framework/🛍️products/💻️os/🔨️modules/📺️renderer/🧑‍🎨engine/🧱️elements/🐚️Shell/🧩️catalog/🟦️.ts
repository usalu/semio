import { admitPluginCatalogV1, pluginCatalogV1, type PluginCatalogAdmissionV1 } from "../../../../../🔌️plugin/📇️registry/🟦️.ts";
import { PlaygroundBootPlanner } from "../../../../../../../../🔨️modules/🎠️kernel/🟦️.ts";

/** 🐚️ Admits the exact per-mount inventory and configures the real React boot without generated defaults. */
export function resolveFrameworkOsCatalogConfigurationV1(input: unknown, variant: string | undefined, control: PluginCatalogAdmissionV1) {
  const rows = admitPluginCatalogV1(input, control), catalog = pluginCatalogV1(rows);
  const boot = new PlaygroundBootPlanner(catalog, variant ?? "", undefined, variant === undefined ? "all" : "variant").finish();
  const idle = variant === undefined && rows.targets.length === 0;
  if ((!idle && boot.plugins.length === 0) || boot.dependencyErrors.length) throw new Error("plugin-catalog-invalid: selected inventory dependency graph");
  return { rows, catalog, boot, idle };
}
