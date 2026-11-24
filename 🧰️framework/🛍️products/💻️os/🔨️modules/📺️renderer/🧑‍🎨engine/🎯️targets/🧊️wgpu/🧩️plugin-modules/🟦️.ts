import { fetchPackageDescriptor, type PluginPackageDescriptor } from "@semio-tech/framework-kernel/descriptor-admission";
import { resolvePluginLoadOrder } from "@semio-tech/framework-kernel/plugin-graph";
import { type PluginCatalog, type PluginCatalogTarget } from "@semio-tech/framework-kernel/plugin-catalog";
import { createOnDemandActivator, expandPluginRegistry, onDemandPluginEntries, requiredDependencies, resolvePluginHostConfig, resolvePluginRegistryId, type OnDemandActivator, type OnDemandStatus } from "@semio-tech/framework";
import { dependsOnToPluginDependencies, type PluginDependency, type PluginRegistryEntry } from "@semio-tech/framework-kernel/plugin-registry";
import type { PlaygroundBoot } from "@semio-tech/framework-kernel/playground-boot";
import { type PackageReceiptSource } from "@semio-tech/framework-kernel/descriptor/package";
import schema from "../../../🧬️schema/🧩️plugin-modules/🔣️.json" with { type: "json" };

export { admitWgpuPluginModules, type WgpuPluginModule } from "./🛂️admission/🟦️.ts";
import type { WgpuPluginModule } from "./🛂️admission/🟦️.ts";
export type WgpuPreparedPluginModules = {
  readonly catalog: PluginCatalog;
  readonly packages: ReadonlyMap<string, PluginPackageDescriptor>;
  readonly dependencies: ReadonlyMap<string, readonly PluginDependency[]>;
  /** ⏳️ Supplied modules the boot plan leaves out and a request may activate later ({@link createWgpuOnDemandActivator}); a missing or unpublished one never fails the boot. */
  readonly deferred: readonly PluginRegistryEntry[];
  /** 🗃️ Every supplied module as a registry entry, where an on-demand activation looks up its required dependencies. */
  readonly installed: readonly PluginRegistryEntry[];
};
const policy = schema["x-semio-admission"];
const idPattern = new RegExp(schema.items.properties.pluginId.pattern, "u");
const versionPattern = new RegExp(schema.$defs.dependency.properties.version.pattern, "u");
const scalarLength = (value: string) => [...value].length;
const refused = (detail: string): never => { throw new Error(`plugin-registry-invalid: ${detail}`); };

/** 🔗️ Admits descriptor dependency edges before a supplied module starts any actor. An edge is `required` unless it says `activation: "on-demand"`. */
function descriptorDependencies(descriptor: PluginPackageDescriptor): readonly PluginDependency[] {
  const raw = (descriptor.manifest as unknown as Record<string, unknown>).dependencies;
  if (raw === undefined) return [];
  if (!Array.isArray(raw) || raw.length > policy.maxDependencies) return refused("dependency count");
  const ids = new Set<string>();
  return raw.map(edge => {
    if (!edge || typeof edge !== "object" || Object.keys(edge).some(key => key !== "pluginId" && key !== "version" && key !== "activation") || typeof edge.pluginId !== "string" || !idPattern.test(edge.pluginId) || scalarLength(edge.pluginId) > schema.items.properties.pluginId.maxLength || ids.has(edge.pluginId)) return refused("dependency identity");
    if (edge.version !== undefined && (typeof edge.version !== "string" || edge.version.length > policy.maxVersionLength || !versionPattern.test(edge.version))) return refused("dependency version");
    if (edge.activation !== undefined && !policy.dependencyActivations.includes(edge.activation)) return refused("dependency activation");
    ids.add(edge.pluginId);
    return { pluginId: edge.pluginId, ...(edge.version === undefined ? {} : { version: edge.version }), ...(edge.activation === "on-demand" ? { activation: "on-demand" as const } : {}) };
  });
}

type DescriptorRead = { readonly deadline: number; readonly now: () => number; readonly signal?: AbortSignal; readonly check: () => void; readonly readDescriptor?: typeof fetchPackageDescriptor };

/** 🛂️ Reads one descriptor in a withdrawable turn and admits its exact identity and version. */
async function readAdmittedDescriptor(row: WgpuPluginModule, receiptSource: PackageReceiptSource, read: DescriptorRead, heartbeat: () => void): Promise<PluginPackageDescriptor> {
  const timeout = AbortSignal.timeout(Math.max(1, Math.ceil(read.deadline - read.now())));
  const signal = read.signal ? AbortSignal.any([read.signal, timeout]) : timeout;
  let withdraw: (() => void) | undefined;
  const stopped = new Promise<never>((_, reject) => { withdraw = () => reject(signal.reason); signal.addEventListener("abort", withdraw, { once: true }); });
  let descriptor: PluginPackageDescriptor;
  try {
    descriptor = await Promise.race([(read.readDescriptor ?? fetchPackageDescriptor)(row.pluginId, row.moduleUrl, receiptSource, signal, () => { read.check(); heartbeat(); }), stopped]);
  } finally { if (withdraw) signal.removeEventListener("abort", withdraw); }
  read.check();
  if (descriptor.manifest.pluginId !== row.pluginId || typeof descriptor.manifest.version !== "string" || descriptor.manifest.version.length === 0 || descriptor.manifest.version.length > policy.maxVersionLength) return refused("descriptor identity or version");
  return descriptor;
}

/** 📇️ Reads trusted descriptors in withdrawable turns and restricts the planner to supplied modules. */
export async function prepareWgpuPluginModules(catalog: PluginCatalog, modules: readonly WgpuPluginModule[], receiptSource:PackageReceiptSource, options: { readonly bootVariant?: string; readonly signal?: AbortSignal; readonly deadlineMs?: number; readonly now?: () => number; readonly maxDescriptors?: number; readonly readDescriptor?: typeof fetchPackageDescriptor; readonly progress?: (pluginId: string, index: number, count: number) => void; readonly yieldTurn?: () => Promise<void> } = {}): Promise<WgpuPreparedPluginModules> {
  const now = options.now ?? (() => performance.now());
  const start = now(), deadline = options.deadlineMs ?? start + 30000, maximum = options.maxDescriptors ?? schema.maxItems;
  if (!Number.isFinite(start) || !Number.isFinite(deadline) || deadline > start + 30000 || !Number.isSafeInteger(maximum) || maximum < 0 || maximum > schema.maxItems || modules.length > maximum) return refused("descriptor finite authority");
  const check = () => { options.signal?.throwIfAborted(); if (!Number.isFinite(now()) || now() >= deadline) return refused("descriptor deadline"); };
  check();
  const packages = new Map<string, PluginPackageDescriptor>();
  const dependencies = new Map<string, readonly PluginDependency[]>();
  const targets: PluginCatalogTarget[] = [];
  const modulesById = new Map(modules.map(row => [row.pluginId, row]));
  const known = new Map([...catalog.plugins, ...catalog.extensions].map(row => [row.pluginId, row]));
  const primary = options.bootVariant && !resolvePluginHostConfig(catalog, options.bootVariant)
    ? resolvePluginRegistryId(catalog, options.bootVariant)
    : undefined;
  const queue = primary ? [...expandPluginRegistry(modules.map(row => {
    const metadata = known.get(row.pluginId);
    return { ...row, contributes: metadata?.contributes, consumes: metadata?.consumes, activationEvents: metadata?.activationEvents, dependencies: dependsOnToPluginDependencies(metadata?.dependsOn) };
  }), primary)] : [...modules];
  const queued = new Set(queue.map(row => row.pluginId));
  for (let index = 0; index < queue.length; index++) {
    check();
    await options.yieldTurn?.();
    check();
    const row = queue[index]!;
    const descriptor = await readAdmittedDescriptor(row, receiptSource, { deadline, now, signal: options.signal, check, readDescriptor: options.readDescriptor }, () => options.progress?.(row.pluginId, index, modules.length));
    const edges = descriptorDependencies(descriptor);
    for (const { pluginId } of requiredDependencies({ dependencies: edges })) {
      const dependency = modulesById.get(pluginId);
      if (!dependency || queued.has(pluginId)) continue;
      queued.add(pluginId);
      queue.push(dependency);
    }
    for (const edge of edges) if (edge.activation === "on-demand" && known.has(edge.pluginId) && (known.get(edge.pluginId)!.activationEvents ?? []).length === 0) return refused("on-demand dependency without activation events");
    const metadata = known.get(row.pluginId);
    const declared = (dependsOnToPluginDependencies(metadata?.dependsOn) ?? []).filter(edge => !edges.some(candidate => candidate.pluginId === edge.pluginId));
    const joined = [...edges, ...declared];
    packages.set(row.pluginId, descriptor);
    dependencies.set(row.pluginId, joined);
    targets.push({ pluginId: row.pluginId, wasmOut: row.moduleUrl, role: metadata?.role ?? "plugin", contributes: metadata?.contributes ?? [], consumes: metadata?.consumes ?? [], dependsOn: requiredDependencies({ dependencies: joined }).map(edge => edge.pluginId), activationEvents: metadata?.activationEvents, capabilities: metadata?.capabilities, extends: metadata?.extends, ...(descriptor.assets ? { assets: descriptor.assets } : {}) });
  }
  const moduleUrl = (id: string) => modulesById.get(id)?.moduleUrl ?? refused(`module ${id} is absent`);
  const candidates = modules.map(row => {
    const metadata = known.get(row.pluginId);
    return { ...row, contributes: metadata?.contributes, consumes: metadata?.consumes, activationEvents: metadata?.activationEvents, dependencies: dependsOnToPluginDependencies(metadata?.dependsOn) };
  });
  const deferred = primary ? onDemandPluginEntries(candidates, targets.map(row => ({ pluginId: row.pluginId, consumes: row.consumes, dependencies: dependencies.get(row.pluginId) }))).filter(entry => (entry.activationEvents ?? []).length > 0) : [];
  return { packages, dependencies, deferred, installed: candidates, catalog: { plugins: targets.filter(row => row.role === "plugin"), extensions: targets.filter(row => row.role === "extension"), hosts: catalog.hosts, playgrounds: catalog.playgrounds, moduleUrl, extensionModuleUrl: moduleUrl } };
}

/** 🛂️ Refuses missing, cyclic or incompatible selected dependencies without starting actors. */
export function assertWgpuPluginPlan(plan: PlaygroundBoot, prepared: WgpuPreparedPluginModules, selection: "variant" | "all"): void {
  const idle = selection === "all" && prepared.catalog.plugins.length === 0 && prepared.catalog.extensions.length === 0;
  if ((plan.plugins.length === 0 && !idle) || plan.dependencyErrors.length) return refused("selected module dependency graph");
  const graph = resolvePluginLoadOrder(plan.plugins.map(row => ({ pluginId: row.pluginId, version: prepared.packages.get(row.pluginId)?.manifest.version, dependencies: requiredDependencies({ dependencies: prepared.dependencies.get(row.pluginId) }) })));
  if (graph.errors.length || graph.order.length !== plan.plugins.length) return refused("selected descriptor dependency graph");
}

/** ⏳️ What starting one deferred module needs from its host: `mount` loads the module and starts its actor (the frame Worker passes its lazy-install door), `descriptor` is the one just admitted. */
export type WgpuOnDemandOptions = {
  readonly mount: (pluginId: string, moduleUrl: string, descriptor: PluginPackageDescriptor) => Promise<void>;
  readonly onStatus?: (status: OnDemandStatus) => void;
  readonly readDescriptor?: typeof fetchPackageDescriptor;
  readonly deadlineMs?: number;
  readonly now?: () => number;
};

/** ⏳️ The on-demand activator of a prepared boot: a request resolves among {@link WgpuPreparedPluginModules.deferred}, then each started plugin has its descriptor read, its identity and required dependencies admitted, and its module mounted, with progress and cancellation. Activated descriptors join `prepared.packages`; a missing or unpublished one reports `unavailable` and the session keeps running. */
export function createWgpuOnDemandActivator(prepared: WgpuPreparedPluginModules, receiptSource: PackageReceiptSource, options: WgpuOnDemandOptions): OnDemandActivator {
  const now = options.now ?? (() => performance.now());
  const packages = prepared.packages as Map<string, PluginPackageDescriptor>;
  const dependencies = prepared.dependencies as Map<string, readonly PluginDependency[]>;
  return createOnDemandActivator({
    candidates: () => prepared.deferred,
    installed: () => prepared.installed,
    active: () => new Set(packages.keys()),
    ...(options.onStatus ? { onStatus: options.onStatus } : {}),
    activate: async (entry, context) => {
      const deadline = options.deadlineMs ?? now() + 30000;
      const check = () => { context.signal.throwIfAborted(); if (!Number.isFinite(now()) || now() >= deadline) return refused("descriptor deadline"); };
      check();
      const descriptor = await readAdmittedDescriptor({ pluginId: entry.pluginId, moduleUrl: entry.moduleUrl }, receiptSource, { deadline, now, signal: context.signal, check, readDescriptor: options.readDescriptor }, () => context.progress(0, 2));
      const edges = descriptorDependencies(descriptor);
      for (const { pluginId } of requiredDependencies({ dependencies: edges })) if (!packages.has(pluginId)) return refused(`required dependency ${pluginId} is not active`);
      context.progress(1, 2);
      await options.mount(entry.pluginId, entry.moduleUrl, descriptor);
      check();
      packages.set(entry.pluginId, descriptor);
      dependencies.set(entry.pluginId, edges);
    },
  });
}
