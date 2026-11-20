import { COMPONENT_MODULE_DIRECTORIES } from "@semio-tech/s-dev-catalog-ts/generated/plugins";
import catalog from "./🔣️.json" with { type: "json" };
import { PLAY_EXCLUDED_VARIANTS, PLAY_HOST_VARIANT, PLAY_RUNTIME_PANES } from "./📇️catalog/🟦️.ts";
import { runtimeComponentClosure } from "@semio-tech/repo-lib/inventory/tooling";
import { PLAYGROUND_BUILD_TARGETS, type PlaygroundBuildTarget } from "../../../../✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🎮️playgrounds/🟦️.ts";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "@semio-tech/s-dev-catalog-ts/generated/plugins";
import { MODULE_SHARD_DIRECTORY, MODULE_VENDOR_DIRECTORY, moduleDirectoryName } from "@semio-tech/plugin-registry/deployment";

/** 🧭️ The playground variants play must show: every registry row except the host shell, the authored
 * `excludedVariants` and the rows that only re-skin another row under a partner brand. */
export function playExpectedVariants(targets: readonly PlaygroundBuildTarget[]): readonly string[] {
  return targets.filter(row => row.variant !== catalog.hostVariant && !PLAY_EXCLUDED_VARIANTS.includes(row.variant) && !row.brand?.startsWith(catalog.excludedBrandPrefix)).map(row => row.variant);
}

/** 🎯️ Registry rows of exactly the authored panes, in pane order. */
export const PLAY_RUNTIME_TARGETS: readonly PlaygroundBuildTarget[] = PLAY_RUNTIME_PANES.map(pane => {
  const target = PLAYGROUND_BUILD_TARGETS.find(row => row.variant === pane.variant);
  if (!target) throw new Error(`Unknown play pane variant: ${pane.variant}`);
  return target;
});

/** 🔭️ The registry rows one play scope serves: a pane variant or a group id; no scope serves every pane. */
export function playScopeTargets(scope?: string): readonly PlaygroundBuildTarget[] {
  if (scope === undefined) return PLAY_RUNTIME_TARGETS;
  const targets = PLAY_RUNTIME_TARGETS.filter((target, index) => target.variant === scope || PLAY_RUNTIME_PANES[index]!.group === scope);
  if (targets.length === 0) throw new Error(`Unknown Play scope: ${scope}`);
  return targets;
}

/** 🧩️ One pane's closure ROOT. A pane boots exactly ONE app, so a registry row naming an `app`
 * never pulls the whole catalog in even when its crate is the OS host (`🪐️space`'s Home and Space). */
export function playPaneClosureRoot(row: PlaygroundBuildTarget): { readonly id: string; readonly appScoped: boolean } {
  return { id: row.pluginId, appScoped: row.app !== undefined };
}

/** 🧩️ The union of the component closures the panes load — each pane boots its own plugin's closure. */
export function playRuntimeComponentIds(targets: readonly PlaygroundBuildTarget[] = PLAY_RUNTIME_TARGETS): string[] {
  const components = [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS];
  return [...new Set(targets.flatMap(row => runtimeComponentClosure(components, [playPaneClosureRoot(row)]) as string[]))].sort();
}

export type PlayRuntimeModuleLayout = {
  readonly pluginModuleDirNames: readonly string[];
  readonly extensionModuleDirNames: readonly string[];
};

/** 🛣️ Maps the complete runtime closure to its public deployment directories. */
export function playRuntimeModuleLayout(targets: readonly PlaygroundBuildTarget[] = PLAY_RUNTIME_TARGETS): PlayRuntimeModuleLayout {
  const byId = new Map([...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS].map(row => [row.pluginId, row]));
  const ids = playRuntimeComponentIds(targets);
  return {
    pluginModuleDirNames: [MODULE_VENDOR_DIRECTORY, MODULE_SHARD_DIRECTORY, ...ids.filter(id => byId.get(id)!.role === "plugin").map(id => moduleDirectoryName(id, COMPONENT_MODULE_DIRECTORIES))],
    extensionModuleDirNames: ids.filter(id => byId.get(id)!.role === "extension").map(id => moduleDirectoryName(id, COMPONENT_MODULE_DIRECTORIES)),
  };
}

if (import.meta.vitest) {
  const { join } = await import("node:path");
  const { isIconName } = await import("../../../../🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🖼️icons/🟦️.ts");
  const { registerTests1 } = await import("../../🧪️tests/🧪️playpanecoverage/🟦️.ts");
  const repoRoot = join(import.meta.dirname, "../../../..");
  await registerTests1(import.meta.vitest, { PLAYGROUND_BUILD_TARGETS, PLAY_RUNTIME_PANES, PLAY_RUNTIME_TARGETS, PLAY_HOST_VARIANT, PLAY_EXCLUDED_VARIANTS, playExpectedVariants, playRuntimeComponentIds, isIconName }, repoRoot);
}
