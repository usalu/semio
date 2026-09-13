import { loadTaxonomy } from "../../../🟦️.ts";
import { policySurfaceRoots } from "../../../🔍️discovery/🗺️surface/🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceDirectory, policySourceText, type PolicySourceOperations } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF_NAME } from "../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { POLICY_APP_CONFIG_DIR, POLICY_APP_CONFIG_LEGACY_DIR, POLICY_APP_PRESENCE_DIR, policyAppPresenceTypeName, type PolicyAppSchemaOwner } from "../🧱️contract/🟦️.ts";

/** 🗂️ Discovers canonical config and presence schema owners from authored surface Config bindings. */
export function policyDiscoverAppSchemaOwners(repoRoot: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): PolicyAppSchemaOwner[] {
  const pluginsRoot = "✏️s/🔌️plugins",
    taxonomy = loadTaxonomy(),
    plugins = policySourceDirectory(repoRoot, pluginsRoot, operations),
    byOwner = new Map<string, PolicyAppSchemaOwner>();
  if (plugins.state !== "directory") {
    if (plugins.state === "missing") return [];
    throw new Error(`Surface owner source directory ${pluginsRoot} is ${plugins.state}.`);
  }
  for (const plugin of plugins.entries.filter((entry) => entry.isDirectory && !entry.isSymbolicLink)) {
    const pluginRel = `${pluginsRoot}/${plugin.name}`;
    for (const surfaceRel of policySurfaceRoots(repoRoot, pluginRel, taxonomy, operations)) {
      const componentRel = `${surfaceRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`,
        component = policySourceText(repoRoot, componentRel, operations);
      if (component.state === "missing") continue;
      if (component.state !== "file") throw new Error(`Surface component source ${componentRel} is ${component.state}.`);
      const match = /\btype\s+Config\s*=\s*([A-Za-z_][A-Za-z0-9_]*)\s*;/.exec(component.text);
      if (!match) continue;
      const configType = match[1]!,
        sliderRel = `${surfaceRel}/${POLICY_APP_CONFIG_DIR}`,
        legacyRel = `${surfaceRel}/${POLICY_APP_CONFIG_LEGACY_DIR}`,
        pluginConfigRel = `${pluginRel}/${POLICY_APP_CONFIG_DIR}`;
      let ownerRel: string | null = null;
      for (const candidate of [sliderRel, legacyRel]) {
        const source = policySourceDirectory(repoRoot, candidate, operations);
        if (source.state === "directory") {
          ownerRel = candidate;
          break;
        }
        if (source.state !== "missing") throw new Error(`Surface owner source directory ${candidate} is ${source.state}.`);
      }
      if (!ownerRel) {
        const pluginConfigComponent = `${pluginConfigRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`,
          source = policySourceText(repoRoot, pluginConfigComponent, operations);
        if (source.state === "file" && new RegExp(`\\bpub\\s+struct\\s+${configType}\\b`).test(source.text)) ownerRel = pluginConfigRel;
        else if (source.state !== "missing" && source.state !== "file") throw new Error(`Surface config source ${pluginConfigComponent} is ${source.state}.`);
      }
      if (!ownerRel) continue;
      const presenceType = policyAppPresenceTypeName(configType),
        parentRel = ownerRel.split("/").slice(0, -1).join("/"),
        presenceRel = `${parentRel}/${POLICY_APP_PRESENCE_DIR}`,
        [subsetName, roleDirName] = surfaceRel.split("/").slice(-2),
        surfaceId = `${plugin.name}/${subsetName}/${roleDirName}`,
        existing = byOwner.get(ownerRel);
      if (existing) existing.apps.push(surfaceId);
      else byOwner.set(ownerRel, { ownerRel, configType, presenceType, presenceRel, apps: [surfaceId] });
    }
  }
  return [...byOwner.values()].sort((left, right) => left.ownerRel.localeCompare(right.ownerRel));
}
