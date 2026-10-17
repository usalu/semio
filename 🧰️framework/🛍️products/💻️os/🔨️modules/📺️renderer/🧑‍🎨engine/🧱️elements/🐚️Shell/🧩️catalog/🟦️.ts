import type { PluginCatalogAdmissionV1 } from "@semio-tech/framework/plugin-catalog";
import { PlaygroundBootPlanner } from "@semio-tech/framework";
import type { CatalogAdmissionPortV1 } from "../../🏛️ShellHost/🪝️host-ports/🟦️.ts";

/** 🐚️ Admits the exact per-mount inventory and configures the real React boot without generated defaults. */
export function resolveFrameworkOsCatalogConfigurationV1(admission: CatalogAdmissionPortV1 | undefined, input: unknown, variant: string | undefined, control: PluginCatalogAdmissionV1) {
  if (admission === undefined) throw new Error("plugin-catalog-invalid: the composition root supplied no catalog admission port");
  const rows = admission.admit(input, control), catalog = admission.catalogOf(rows);
  const boot = new PlaygroundBootPlanner(catalog, variant ?? "", undefined, variant === undefined ? "all" : "variant").finish();
  const idle = variant === undefined && rows.targets.length === 0;
  if ((!idle && boot.plugins.length === 0) || boot.dependencyErrors.length) throw new Error("plugin-catalog-invalid: selected inventory dependency graph");
  return { rows, catalog, boot, idle };
}
