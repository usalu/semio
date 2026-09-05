/** 🧭 Pure target selection for attaching an opened document to the session that was just
 * created by an artifact-opening relay, even before React publishes that session as current. */

export type DocumentOpeningTarget<TSession, TPlugin> = {
  readonly session: TSession;
  readonly plugin: TPlugin;
};

/** 📍 Prefers the relay's explicit newly-created session and otherwise resolves the current
 * session's plugin from the live loaded-plugin collection. */
export function resolveDocumentOpeningTarget<TSession extends { readonly pluginId: string }, TPlugin extends { readonly pluginId: string }>(
  explicit: DocumentOpeningTarget<TSession, TPlugin> | undefined,
  currentSession: TSession | null,
  loadedPlugins: readonly { readonly handle: TPlugin }[],
): DocumentOpeningTarget<TSession, TPlugin> | null {
  if (explicit) return explicit;
  if (!currentSession) return null;
  const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === currentSession.pluginId)?.handle;
  return plugin ? { session: currentSession, plugin } : null;
}
