import { parseResolvedPluginViewState, type BuiltNode, type PluginUiRefreshRequest, type PluginUiRefreshResponse, type PluginViewState } from "../../🛂️manifest/🟦️.ts";
import type { Effect } from "../🟦️.ts";

/** 🧩️ The actor operations an embedded surface owns. */
export type ExternalSlotContributor = {
  readonly manifest: { readonly apps: readonly {
    readonly id: string;
    readonly controllerId: string;
    readonly defaultModeId: string;
    readonly modes: readonly { readonly id: string }[];
    readonly windowKinds: readonly { readonly id: string; readonly bodyKey: string }[];
  }[] };
  createApp(appId: string): Promise<number>;
  destroyApp(instanceId: number): Promise<void>;
  refreshUi(instanceId: number, request: PluginUiRefreshRequest): Promise<PluginUiRefreshResponse>;
};

/** 🎟️ One parent surface's exact contributor instance, including in-flight creation. */
export type ContributorInstance = {
  readonly pluginId: string;
  readonly appId: string;
  readonly ownerId: string;
  readonly passes: Map<string, symbol>;
  readonly handle: ExternalSlotContributor;
  readonly instance: Promise<number>;
  pending: Promise<unknown>;
  body?: BuiltNode;
  hash?: string;
};

export type ExternalSlotResolverContext = {
  readonly plugins: ReadonlyMap<string, ExternalSlotContributor>;
  readonly contributorInstances: Map<string, ContributorInstance>;
  readonly contributorPasses: Map<string, symbol>;
  readonly viewState: PluginViewState;
  readonly ownerId: string;
  readonly hostExtensions?: ReadonlySet<string>;
  readonly onEffects?: (pluginId: string, instanceId: number, effects: readonly Effect[]) => void;
  readonly onError?: (extension: string, error: unknown) => void;
};

/** 🧩️ Identifies parent bodies whose embedded content must refresh even when the parent hash is unchanged. */
export function hasExternalSlots(node: BuiltNode): boolean {
  return node.component.type === "extension" || node.children.some(hasExternalSlots);
}

/** 🧹️ Detaches leases before awaiting creation so no retired instance can be reused. */
export async function retireContributorInstances(instances: Map<string, ContributorInstance>, matches: (entry: ContributorInstance) => boolean = () => true): Promise<void> {
  const retiring: ContributorInstance[] = [];
  for (const [key, entry] of instances) if (matches(entry)) { entry.passes.delete(entry.ownerId); instances.delete(key); retiring.push(entry); }
  await Promise.allSettled(retiring.map(async (entry) => { await entry.pending.catch(() => {}); await entry.handle.destroyApp(await entry.instance); }));
}

/** 🎟️ Publishes the creation promise before its first await; parallel refreshes share the exact lease. */
export function ensureContributorInstance(pluginId: string, appId: string, path: readonly string[], context: ExternalSlotResolverContext): ContributorInstance | null {
  const handle = context.plugins.get(pluginId);
  if (!handle) return null;
  const key = JSON.stringify([context.ownerId, pluginId, appId, path]);
  const existing = context.contributorInstances.get(key);
  if (existing) return existing;
  const instance = Promise.resolve().then(() => handle.createApp(appId));
  const entry: ContributorInstance = { pluginId, appId, ownerId: context.ownerId, passes: context.contributorPasses, handle, instance, pending: Promise.resolve() };
  context.contributorInstances.set(key, entry);
  void instance.catch(() => { if (context.contributorInstances.get(key) === entry) context.contributorInstances.delete(key); });
  return entry;
}

/** 🪟️ Resolves contributed bodies through the same retained UI refresh path as ordinary windows. */
export async function resolveExternalSlots(node: BuiltNode, context: ExternalSlotResolverContext): Promise<BuiltNode> {
  const pass = Symbol();
  context.contributorPasses.set(context.ownerId, pass);
  const used = new Set<ContributorInstance>();
  const current = () => context.contributorPasses.get(context.ownerId) === pass;
  const visit = async (node: BuiltNode, path: readonly string[], ancestors: readonly string[]): Promise<BuiltNode> => {
    if (!current()) return node;
    if (node.component.type !== "extension" || context.hostExtensions?.has(node.component.extension)) {
      const children = await Promise.all(node.children.map((child) => visit(child, [...path, child.key], ancestors)));
      return children.every((child, index) => child === node.children[index]) ? node : { ...node, children };
    }
    const address = node.component.extension;
    const separator = address.indexOf("/");
    const pluginId = separator < 0 ? address : address.slice(0, separator);
    const appId = separator < 0 ? "" : address.slice(separator + 1);
    const props = node.component.props;
    try {
      const app = context.plugins.get(pluginId)?.manifest.apps.find((app) => app.id === appId);
      const bodyKey = props && typeof props === "object" && !Array.isArray(props) ? props.bodyKey : undefined;
      const window = app?.windowKinds.find((window) => window.bodyKey === bodyKey);
      const mode = app?.modes.find((mode) => mode.id === app.defaultModeId);
      const addressKey = JSON.stringify([address, bodyKey]);
      if (!app || !window || !mode || ancestors.length >= 8 || ancestors.includes(addressKey)) throw new Error("unavailable or recursive extension surface");
      const viewState = parseResolvedPluginViewState({
        locale: context.viewState.locale, terminology: context.viewState.terminology,
        sessionIdentity: context.viewState.sessionIdentity,
        activeModeId: mode.id, windowId: "extension", activeWindowKindId: window.id,
        windowInstances: [{ id: "extension", windowKindId: window.id }],
        extensionInputJson: JSON.stringify(props),
      });
      const entry = ensureContributorInstance(pluginId, appId, [...path, window.bodyKey], context);
      if (!entry) throw new Error("extension actor unavailable");
      used.add(entry);
      const refresh = entry.pending.catch(() => {}).then(async () => {
        const instanceId = await entry.instance;
        if (!current()) throw new Error("extension surface retired");
        const sectionRequest = { key: "extension", bodyKey: window.bodyKey, ...(entry.hash === undefined ? {} : { hash: entry.hash }) };
        const response = await entry.handle.refreshUi(instanceId, { viewState, windows: [sectionRequest] });
        if (!current()) throw new Error("extension surface retired");
        if (response.requestedEffects?.length) {
          if (!context.onEffects) throw new Error("extension effects have no owner");
          context.onEffects(pluginId, instanceId, response.requestedEffects);
        }
        const section = response.windows?.find((section) => section.key === "extension");
        if (!section) throw new Error("extension returned no UI section");
        const value = section?.value ?? entry.body;
        if (!value || typeof value !== "object" || !("component" in value) || !("children" in value) || !Array.isArray(value.children)) throw new Error("extension returned no UI body");
        entry.body = value as BuiltNode;
        entry.hash = section?.hash;
        return value as BuiltNode;
      });
      entry.pending = refresh;
      const body = await visit(await refresh, [...path, "body"], [...ancestors, addressKey]);
      return { ...node, component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null }, children: [body] };
    } catch (error) {
      if (!current()) return node;
      context.onError?.(address, error);
      const label = context.viewState.locale === "de" ? "Erweiterung nicht verfügbar" : "Extension unavailable";
      return { ...node, component: { type: "text", value: `${label}: ${pluginId}`, emphasize: null, dataAttributes: null }, children: [] };
    }
  };
  const result = await visit(node, [node.key], []);
  if (context.contributorPasses.get(context.ownerId) === pass) {
    await retireContributorInstances(context.contributorInstances, (entry) => entry.ownerId === context.ownerId && !used.has(entry));
    if (context.contributorPasses.get(context.ownerId) === pass) context.contributorPasses.delete(context.ownerId);
  }
  return result;
}
