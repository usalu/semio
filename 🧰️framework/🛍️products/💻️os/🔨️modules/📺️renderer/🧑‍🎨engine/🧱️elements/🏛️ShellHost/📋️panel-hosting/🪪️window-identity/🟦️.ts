/** 🪪 Exposes the identity of the session and document that owns a rendered window. */
export function windowIdentityAttributesV1(input: Readonly<{ app: string; role: string; instance: number; window: string; kind: string; document: Readonly<{ spaceId: string; documentId: string }> | null }>) {
  return { "data-surface-app-id": input.app, "data-surface-role": input.role, "data-surface-session-instance-id": input.instance, "data-surface-window-id": input.window, "data-surface-window-kind-id": input.kind, ...(input.document === null ? {} : { "data-surface-space-id": input.document.spaceId, "data-surface-document-id": input.document.documentId }) };
}

/** 🔎 Resolves a document only when exactly one open session owns it. */
export function windowDocumentScopeV1(entries: Iterable<Readonly<{ session: Readonly<{ pluginId: string; instanceId: number; app: Readonly<{ id: string }> }>; scope?: Readonly<{ spaceId: string; documentId: string }> }>>, owner: Readonly<{ pluginId: string; appId: string; instanceId: number }>) {
  const matches = [...entries].filter((entry) => entry.session.pluginId === owner.pluginId && entry.session.app.id === owner.appId && entry.session.instanceId === owner.instanceId);
  return matches.length === 1 ? matches[0].scope ?? null : null;
}
