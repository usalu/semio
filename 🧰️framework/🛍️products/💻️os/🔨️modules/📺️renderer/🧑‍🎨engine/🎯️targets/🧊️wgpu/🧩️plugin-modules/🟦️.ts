import { fetchPackageDescriptor, resolvePluginLoadOrder, type PluginCatalog, type PluginCatalogTarget, type PluginDependency, type PluginPackageDescriptor, type PlaygroundBoot } from "../../../../../../../../🔨️modules/🎠️kernel/🟦️.ts";
import schema from "../../../🧬️schema/🧩️plugin-modules/🔣️.json";

export { admitWgpuPluginModules, type WgpuPluginModule } from "./🛂️admission/🟦️.ts";
import type { WgpuPluginModule } from "./🛂️admission/🟦️.ts";
export type WgpuPreparedPluginModules = { readonly catalog: PluginCatalog; readonly packages: ReadonlyMap<string, PluginPackageDescriptor>; readonly dependencies: ReadonlyMap<string, readonly PluginDependency[]> };
const policy = schema["x-semio-admission"];
const idPattern = new RegExp(schema.items.properties.pluginId.pattern, "u");
const versionPattern = new RegExp(schema.$defs.dependency.properties.version.pattern, "u");
const scalarLength = (value: string) => [...value].length;
const refused = (detail: string): never => { throw new Error(`plugin-registry-invalid: ${detail}`); };

/** 🔗️ Admits descriptor dependency edges before a supplied module starts any actor. */
function descriptorDependencies(descriptor: PluginPackageDescriptor): readonly PluginDependency[] {
  const raw = (descriptor.manifest as unknown as Record<string, unknown>).dependencies;
  if (raw === undefined) return [];
  if (!Array.isArray(raw) || raw.length > policy.maxDependencies) return refused("dependency count");
  const ids = new Set<string>();
  return raw.map(edge => {
    if (!edge || typeof edge !== "object" || Object.keys(edge).some(key => key !== "pluginId" && key !== "version") || typeof edge.pluginId !== "string" || !idPattern.test(edge.pluginId) || scalarLength(edge.pluginId) > schema.items.properties.pluginId.maxLength || ids.has(edge.pluginId)) return refused("dependency identity");
    if (edge.version !== undefined && (typeof edge.version !== "string" || edge.version.length > policy.maxVersionLength || !versionPattern.test(edge.version))) return refused("dependency version");
    ids.add(edge.pluginId);
    return { pluginId: edge.pluginId, ...(edge.version === undefined ? {} : { version: edge.version }) };
  });
}

/** 📇️ Reads trusted descriptors in withdrawable turns and restricts the planner to supplied modules. */
export async function prepareWgpuPluginModules(catalog: PluginCatalog, modules: readonly WgpuPluginModule[], options: { readonly signal?: AbortSignal; readonly deadlineMs?: number; readonly now?: () => number; readonly maxDescriptors?: number; readonly readDescriptor?: typeof fetchPackageDescriptor; readonly progress?: (pluginId: string, index: number, count: number) => void; readonly yieldTurn?: () => Promise<void> } = {}): Promise<WgpuPreparedPluginModules> {
  const now = options.now ?? (() => performance.now());
  const start = now(), deadline = options.deadlineMs ?? start + 30000, maximum = options.maxDescriptors ?? schema.maxItems;
  if (!Number.isFinite(start) || !Number.isFinite(deadline) || deadline > start + 30000 || !Number.isSafeInteger(maximum) || maximum < 0 || maximum > schema.maxItems || modules.length > maximum) return refused("descriptor finite authority");
  const check = () => { options.signal?.throwIfAborted(); if (!Number.isFinite(now()) || now() >= deadline) return refused("descriptor deadline"); };
  check();
  const packages = new Map<string, PluginPackageDescriptor>();
  const dependencies = new Map<string, readonly PluginDependency[]>();
  const targets: PluginCatalogTarget[] = [];
  const urls = new Map(modules.map(row => [row.pluginId, row.moduleUrl]));
  const known = new Map([...catalog.plugins, ...catalog.extensions].map(row => [row.pluginId, row]));
  for (let index = 0; index < modules.length; index++) {
    check();
    await options.yieldTurn?.();
    check();
    const row = modules[index]!;
    const timeout = AbortSignal.timeout(Math.max(1, Math.ceil(deadline - now())));
    const signal = options.signal ? AbortSignal.any([options.signal, timeout]) : timeout;
    let withdraw: (() => void) | undefined;
    const stopped = new Promise<never>((_, reject) => { withdraw = () => reject(signal.reason); signal.addEventListener("abort", withdraw, { once: true }); });
    let descriptor: PluginPackageDescriptor;
    try {
      descriptor = await Promise.race([(options.readDescriptor ?? fetchPackageDescriptor)(row.pluginId, row.moduleUrl, signal, () => { check(); options.progress?.(row.pluginId, index, modules.length); }), stopped]);
    } finally { if (withdraw) signal.removeEventListener("abort", withdraw); }
    check();
    if (descriptor.manifest.pluginId !== row.pluginId || typeof descriptor.manifest.version !== "string" || descriptor.manifest.version.length === 0 || descriptor.manifest.version.length > policy.maxVersionLength) return refused("descriptor identity or version");
    const edges = descriptorDependencies(descriptor);
    packages.set(row.pluginId, descriptor);
    dependencies.set(row.pluginId, edges);
    const metadata = known.get(row.pluginId);
    targets.push({ pluginId: row.pluginId, wasmOut: row.moduleUrl, role: metadata?.role ?? "plugin", contributes: metadata?.contributes ?? [], consumes: metadata?.consumes ?? [], dependsOn: edges.map(edge => edge.pluginId), activationEvents: metadata?.activationEvents, capabilities: metadata?.capabilities, extends: metadata?.extends });
  }
  const moduleUrl = (id: string) => urls.get(id) ?? refused(`module ${id} is absent`);
  return { packages, dependencies, catalog: { plugins: targets.filter(row => row.role === "plugin"), extensions: targets.filter(row => row.role === "extension"), hosts: catalog.hosts, playgrounds: catalog.playgrounds, moduleUrl, extensionModuleUrl: moduleUrl } };
}

/** 🛂️ Refuses missing, cyclic or incompatible selected dependencies without starting actors. */
export function assertWgpuPluginPlan(plan: PlaygroundBoot, prepared: WgpuPreparedPluginModules, selection: "variant" | "all"): void {
  const idle = selection === "all" && prepared.catalog.plugins.length === 0 && prepared.catalog.extensions.length === 0;
  if ((plan.plugins.length === 0 && !idle) || plan.dependencyErrors.length) return refused("selected module dependency graph");
  const graph = resolvePluginLoadOrder(plan.plugins.map(row => ({ pluginId: row.pluginId, version: prepared.packages.get(row.pluginId)?.manifest.version, dependencies: prepared.dependencies.get(row.pluginId) })));
  if (graph.errors.length || graph.order.length !== plan.plugins.length) return refused("selected descriptor dependency graph");
}
