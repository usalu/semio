import type { LocalSourceRevisionRegistryV1 } from "./registry.ts";
import type { SourceRevisionOwnerV1 } from "./🟦️.ts";
import { beginSourceRevisionWorkV1, type SourceRevisionWorkV1, type SourceWorkCompletionV1 } from "./work.ts";

export type DocumentSourceIdentityV1 = Readonly<{ pluginId: string; appId: string; sessionInstanceId: number; runtimeKey: string; clientInstanceId: string }>;
export type SourceRevisionPluginV1 = Readonly<{ subscribeOperationCompletions(instanceId: number, listener: (completion: SourceWorkCompletionV1) => void): () => void }>;

/** 📎️ Pins background ingestion to the same real document owner used by the focused renderer. */
export function beginDocumentSourceWorkV1<Plugin extends SourceRevisionPluginV1>(
  host: Readonly<{
    owners: Readonly<{ acquire(identity: DocumentSourceIdentityV1): SourceRevisionOwnerV1 }>;
    sources: LocalSourceRevisionRegistryV1;
    refresh(plugin: Plugin, instanceId: number): void;
  }>,
  identity: DocumentSourceIdentityV1,
  plugin: Plugin,
): SourceRevisionWorkV1 | null {
  const owner = host.owners.acquire(identity);
  const source = host.sources.acquire(owner, plugin);
  if (source === null) return null;
  return beginSourceRevisionWorkV1({
    source, owner, plugin,
    subscribe: listener => plugin.subscribeOperationCompletions(identity.sessionInstanceId, listener),
    refresh: () => host.refresh(plugin, identity.sessionInstanceId),
  });
}
