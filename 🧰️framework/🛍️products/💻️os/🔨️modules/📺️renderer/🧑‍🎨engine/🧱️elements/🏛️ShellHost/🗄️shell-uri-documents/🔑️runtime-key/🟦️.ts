/** 🔑️ The entry shape the runtime-key lookup reads. */
export type RuntimeKeyEntryV1 = Readonly<{ session: Readonly<{ pluginId: string; instanceId: number }>; documentId: string }>;

/** 🔑️ The runtime key of the document a session shows: the entry must match the session's plugin and instance and, when the caller knows it, the document identity. Zero or several matches answer `null` — the lookup fails closed instead of guessing the first. */
export function resolveRuntimeKeyV1(entries: Iterable<readonly [string, RuntimeKeyEntryV1]>, session: Readonly<{ pluginId: string; instanceId: number }>, documentId?: string): string | null {
  let found: string | null = null;
  for (const [runtimeKey, entry] of entries) {
    if (entry.session.pluginId !== session.pluginId || entry.session.instanceId !== session.instanceId) continue;
    if (documentId !== undefined && entry.documentId !== documentId) continue;
    if (found !== null) return null;
    found = runtimeKey;
  }
  return found;
}
