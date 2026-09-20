import catalog from "./🔣️.json";
import { runtimeComponentClosure } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs";
import { PLAYGROUND_BUILD_TARGETS, type PlaygroundBuildTarget } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts";
import { MODULE_SHARD_DIRECTORY, MODULE_VENDOR_DIRECTORY, moduleDirectoryName } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";

export const PLAY_HOST = catalog.host;
/** @emoji 🏠️ The OS shell variant — it hosts apps rather than being one, so play never lists it. */
export const PLAY_HOST_VARIANT = catalog.hostVariant;
export const PLAY_GROUPS = catalog.groups;

/** @emoji 🎡️ One authored pane of the play grid, tagged with the group it is listed under. Mirrors
 * `🧬️schema/🔣️.json`'s `PlayPane` — `example` is the curated `PlayExampleId` the pane boots on and is
 * absent exactly for the panes whose app publishes no example for its dialect. Stated rather than
 * inferred from the JSON module, because a catalog of heterogeneous rows infers as a union that hides
 * the optional key. */
export type PlayRuntimePane = {
  readonly variant: string;
  readonly label: string;
  readonly tagline: string;
  readonly description: string;
  readonly icon: string;
  readonly example?: string;
  readonly group: string;
};

/** @emoji 📋️ Every authored pane in grid order (row-major, groups in authored order). */
export const PLAY_RUNTIME_PANES: readonly PlayRuntimePane[] = catalog.groups.flatMap(group => group.panes.map(pane => ({ ...(pane as Omit<PlayRuntimePane, "group">), group: group.id })));

/** @emoji 🧭️ The playground variants play must show: every registry row except the host shell and the rows
 * that only re-skin another row under a partner brand. */
export function playExpectedVariants(targets: readonly PlaygroundBuildTarget[]): readonly string[] {
  return targets.filter(row => row.variant !== catalog.hostVariant && !row.brand?.startsWith(catalog.excludedBrandPrefix)).map(row => row.variant);
}

/** @emoji 🎯️ Registry rows of exactly the authored panes, in pane order. */
export const PLAY_RUNTIME_TARGETS: readonly PlaygroundBuildTarget[] = PLAY_RUNTIME_PANES.map(pane => {
  const target = PLAYGROUND_BUILD_TARGETS.find(row => row.variant === pane.variant);
  if (!target) throw new Error(`Unknown play pane variant: ${pane.variant}`);
  return target;
});

/** @emoji 🧩️ One pane's closure ROOT. A pane boots exactly ONE app, so a registry row naming an `app`
 * never pulls the whole catalog in even when its crate is the OS host (`🪐️space`'s Home and Space). */
export function playPaneClosureRoot(row: PlaygroundBuildTarget): { readonly id: string; readonly appScoped: boolean } {
  return { id: row.pluginId, appScoped: row.app !== undefined };
}

/** @emoji 🧩️ The union of the component closures the panes load — each pane boots its own plugin's closure. */
export function playRuntimeComponentIds(): string[] {
  const components = [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS];
  return [...new Set(PLAY_RUNTIME_TARGETS.flatMap(row => runtimeComponentClosure(components, [playPaneClosureRoot(row)]) as string[]))].sort();
}

export type PlayRuntimeModuleLayout = {
  readonly pluginModuleDirNames: readonly string[];
  readonly extensionModuleDirNames: readonly string[];
};

/** @emoji 🛣️ Maps the complete runtime closure to its public deployment directories. */
export function playRuntimeModuleLayout(): PlayRuntimeModuleLayout {
  const byId = new Map([...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS].map(row => [row.pluginId, row]));
  const ids = playRuntimeComponentIds();
  return {
    pluginModuleDirNames: [MODULE_VENDOR_DIRECTORY, MODULE_SHARD_DIRECTORY, ...ids.filter(id => byId.get(id)!.role === "plugin").map(moduleDirectoryName)],
    extensionModuleDirNames: ids.filter(id => byId.get(id)!.role === "extension").map(moduleDirectoryName),
  };
}

if (import.meta.vitest) {
  const { join } = await import("node:path");
  const { isIconName } = await import("../../../../🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🖼️icons/🟦️.ts");
  const { registerTests1 } = await import("../../🧪️tests/🧪️playpanecoverage/🟦️.ts");
  const repoRoot = join(import.meta.dirname, "../../../..");
  await registerTests1(import.meta.vitest, { PLAYGROUND_BUILD_TARGETS, PLAY_RUNTIME_PANES, PLAY_RUNTIME_TARGETS, PLAY_HOST_VARIANT, playExpectedVariants, playRuntimeComponentIds, isIconName }, repoRoot);
  const { PLAY_PANES } = await import("../../🪧️brand.ts");
  const { registerTests1: registerPaneDefaultTests } = await import("../../🧪️tests/🧪️playpanedefaults/🟦️.ts");
  await registerPaneDefaultTests(import.meta.vitest, { PLAY_RUNTIME_PANES, PLAY_RUNTIME_TARGETS, PLAY_PANES }, repoRoot);
}
