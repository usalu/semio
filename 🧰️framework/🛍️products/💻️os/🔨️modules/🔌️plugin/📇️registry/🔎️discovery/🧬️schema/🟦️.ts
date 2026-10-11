import contract from "./🔣️.json";
import identity from "../../../../../../../🔨️modules/🪪️identity/📁️installation/🧬️schema/🔣️.json";
import { parseInstallationDirectoryV1, type InstallationDirectoryV1 } from "../../../../../../../🔨️modules/🪪️identity/📁️installation/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";

export type PluginHostMetadata = { readonly landingAppId: string; readonly hostAppId: string };
export type PluginDescriptorHashes = { readonly wasmSha256: string; readonly coreWasmSha256: string; readonly descriptorSha256: string };
export type ComponentSourceOwnerV1 = {
  readonly pluginId: string;
  readonly packageId: string;
  readonly cratePath: string;
  readonly packageName: string;
  readonly wasmOut: string;
  readonly directoryName?: InstallationDirectoryV1;
  readonly role: "plugin" | "extension";
  readonly extends?: string;
  readonly consumes: readonly string[];
  readonly dependsOn: readonly string[];
  readonly host?: PluginHostMetadata;
};
export type PluginBuildTargetV1 = ComponentSourceOwnerV1 & { readonly directoryName: InstallationDirectoryV1 };
export type CompiledComponentOwnerV1 = ComponentSourceOwnerV1 & {
  readonly capabilities: readonly string[];
  readonly contributes: readonly string[];
  readonly activationEvents: readonly string[];
  readonly extensionPoints: readonly string[];
  readonly executionMode: string;
  readonly hashes: PluginDescriptorHashes;
};
export type DeployedRegistryEntryV1 = CompiledComponentOwnerV1 & { readonly directoryName: InstallationDirectoryV1 };

function resolveIdentity(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(resolveIdentity);
  if (value === null || typeof value !== "object") return value;
  const row = value as Record<string, unknown>;
  if (row.$ref === identity.$id) return identity;
  return Object.fromEntries(Object.entries(row).map(([key, member]) => [key, resolveIdentity(member)]));
}
const schemas = resolveIdentity(contract) as typeof contract;
function admit(stage: keyof typeof contract.$defs, value: unknown): Record<string, unknown> {
  const errors = validateJsonSchemaSubset(schemas.$defs[stage], value, schemas);
  if (errors.length) throw Error(`Invalid ${stage}: ${errors.slice(0, 8).join("; ")}`);
  const row = value as Record<string, unknown>;
  if (row.directoryName !== undefined) parseInstallationDirectoryV1(row.directoryName);
  return row;
}

/** 📄️Admits source-authored component facts without descriptor fields. */
export function parseComponentSourceRowV1(value: unknown): ComponentSourceOwnerV1 {
  return admit("ComponentSourceOwnerV1", value) as ComponentSourceOwnerV1;
}
/** 🔨️Admits one discovered plugin crate's source facts as a build target, independent of its descriptor. */
export function parsePluginBuildTargetV1(value: unknown): PluginBuildTargetV1 {
  return admit("PluginBuildTargetV1", value) as PluginBuildTargetV1;
}
/** 🛂️Admits real compiled descriptor facts with optional deployment declaration. */
export function parseCompiledComponentRowV1(value: unknown): CompiledComponentOwnerV1 {
  return admit("CompiledComponentOwnerV1", value) as CompiledComponentOwnerV1;
}
/** 📦️Admits a deployed registry row with mandatory neutral installation identity. */
export function parseDeployedRegistryEntryV1(value: unknown): DeployedRegistryEntryV1 {
  return admit("DeployedRegistryEntryV1", value) as DeployedRegistryEntryV1;
}
