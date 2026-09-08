import type { PersistenceBinding } from "@semio-tech/framework-os";

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

/** 🪪️ Document identity and optional shared space supplied by this exact opening request. */
export type DocumentOpeningReference = {
  readonly documentId: string;
  readonly schema: string;
  readonly spaceId?: string;
};

/** 📍 Resolves the persistence destinations at the document-opening boundary. */
export function resolveDocumentOpeningBindings(
  ref: DocumentOpeningReference,
  context: {
    readonly identity: { readonly hubBaseUrl: string } | null;
    readonly dataDir?: string;
    readonly surface?: string;
  },
): readonly PersistenceBinding[] {
  const spaceId = ref.spaceId;
  if (spaceId === undefined) return [];
  if (!context.identity) throw new Error("opening.identity-required");
  if (!context.surface) throw new Error("opening.surface-required");
  const folder: PersistenceBinding[] = spaceId && context.dataDir ? [{ kind: "folder", path: `${context.dataDir}/spaces/${spaceId}` }] : [];
  return [{ kind: "hub", baseUrl: context.identity.hubBaseUrl, spaceId, requestedSurfaceId: context.surface }, ...folder];
}
