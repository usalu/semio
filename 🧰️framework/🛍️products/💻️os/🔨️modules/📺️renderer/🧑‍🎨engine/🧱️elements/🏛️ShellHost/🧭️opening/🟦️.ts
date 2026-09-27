import type { AppRole } from "@semio-tech/framework";
import { emptyDirectoryReadModel, foldAll, type DirectoryEvent, type PersistenceBinding } from "@semio-tech/framework-os";
import accessPolicy from "../../../../../../../../../🌎️hub/🔐️auth/🛡️access-policy/🔣️.json" with { type: "json" };
import { hubAccessPermits, type HubAccessPolicyV1 } from "../../../../../../../../../🌎️hub/🔐️auth/🛡️access-policy/🟦️.ts";

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
  const folder: PersistenceBinding[] = spaceId && context.dataDir ? [{ kind: "folder", dataClass: "persistedLocalOnly", path: `${context.dataDir}/spaces/${spaceId}` }] : [];
  return [{ kind: "hub", dataClass: "persistedShared", baseUrl: context.identity.hubBaseUrl, spaceId, requestedSurfaceId: context.surface }, ...folder];
}

/** 👁️ The surface role a signed-in human opens a shared space's document with, decided the way the hub issues the open
 * plan (`surface_writable`: the caller's space role permits `document.write` under the declared access policy, whatever
 * the space kind): folded from the space's own directory events (its creation upserts the owner as a member, an archive
 * demotes every author), the caller's member role opens the editor when the policy lets it write, and a Spectator, a
 * removed member or a non-member opens the viewer. `null` when the events hold no such space, so there is nothing to
 * decide from. Requesting any other surface is refused by the hub (`component-unavailable`), which is how a Spectator's
 * opening used to end in an unattached editor.
 * @see ../../../../../../../../../🌎️hub/🔐️auth/🛡️access-policy/🔣️.json */
export function sharedDocumentOpeningRoleV1(events: readonly DirectoryEvent[], spaceId: string, userId: string): AppRole | null {
  const space = foldAll(emptyDirectoryReadModel(), events).spaces.get(spaceId);
  if (space === undefined) return null;
  const role = space.members.find((member) => member.userId === userId)?.role;
  return role !== undefined && hubAccessPermits(accessPolicy as HubAccessPolicyV1, [role], "document.write") ? "editor" : "viewer";
}
