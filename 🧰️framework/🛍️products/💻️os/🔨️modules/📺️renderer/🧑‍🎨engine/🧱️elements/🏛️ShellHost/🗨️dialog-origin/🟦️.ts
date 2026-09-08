/** 🗨️ Exact ephemeral session and mount ownership for Shell dialog effects. */
export type ShellDialogOriginV1 = Readonly<{
  pluginId: string;
  appId: string;
  controllerId: string;
  sessionInstanceId: number;
  document: Readonly<{
    runtimeKey: string;
    clientInstanceId: string;
    scope: Readonly<{ spaceId: string; documentId: string }> | null;
  }> | null;
}>;

export type ShellDialogV1 = Readonly<{
  openingId: number;
  dialogId: string;
  origin: ShellDialogOriginV1;
  seedArgs?: Readonly<Record<string, unknown>>;
}>;

export type ShellDialogSessionV1 = Readonly<{
  pluginId: string;
  instanceId: number;
  app: Readonly<{ id: string; controllerId: string }>;
}>;

export type ShellDialogMountV1 = Readonly<{
  session: ShellDialogSessionV1;
  runtimeKey: string;
  clientInstanceId: string;
  scope?: Readonly<{ spaceId: string; documentId: string }> | null;
}>;

/** 🪪️ Numeric instances are never identities across plugins or apps. */
export function shellDialogSessionIsCurrentV1(owner: ShellDialogSessionV1 | null, current: ShellDialogSessionV1 | null): boolean {
  return owner !== null && current !== null && owner.pluginId === current.pluginId && owner.instanceId === current.instanceId
    && owner.app.id === current.app.id && owner.app.controllerId === current.app.controllerId;
}

/** 📍️ Copies one unambiguous mount, or a documentless session without document authority. */
export function shellDialogOriginV1(session: ShellDialogSessionV1 | null, mounts: readonly ShellDialogMountV1[]): ShellDialogOriginV1 | null {
  if (session === null) return null;
  const matches = mounts.filter((mount) => shellDialogSessionIsCurrentV1(session, mount.session));
  if (matches.length > 1) return null;
  const mount = matches[0];
  return {
    pluginId: session.pluginId,
    appId: session.app.id,
    controllerId: session.app.controllerId,
    sessionInstanceId: session.instanceId,
    document: mount === undefined ? null : {
      runtimeKey: mount.runtimeKey,
      clientInstanceId: mount.clientInstanceId,
      scope: mount.scope == null ? null : { ...mount.scope },
    },
  };
}

/** 🔒️ Refuses mount replacement, scope substitution and detached owners before any UI dispatch. */
export function shellDialogOriginIsCurrentV1(owner: ShellDialogOriginV1 | null, current: ShellDialogOriginV1 | null): boolean {
  if (owner === null || current === null || owner.pluginId !== current.pluginId || owner.appId !== current.appId
    || owner.controllerId !== current.controllerId || owner.sessionInstanceId !== current.sessionInstanceId) return false;
  if (owner.document === null || current.document === null) return owner.document === current.document;
  return owner.document.runtimeKey === current.document.runtimeKey && owner.document.clientInstanceId === current.document.clientInstanceId
    && (owner.document.scope === null || current.document.scope === null
      ? owner.document.scope === current.document.scope
      : owner.document.scope.spaceId === current.document.scope.spaceId && owner.document.scope.documentId === current.document.scope.documentId);
}

/** 🎭️ An effect source must remain the primary app or an exact live spawned app, with its original mount. */
export function shellEffectSourceIsCurrentV1(
  source: ShellDialogOriginV1 | null,
  mounted: ShellDialogOriginV1 | null,
  primary: ShellDialogSessionV1 | null,
  spawned: readonly Readonly<{ pluginId: string; appId: string; instanceId: number }>[],
): boolean {
  if (source === null || !shellDialogOriginIsCurrentV1(source, mounted) || primary === null) return false;
  return (source.pluginId === primary.pluginId && source.appId === primary.app.id
    && source.controllerId === primary.app.controllerId && source.sessionInstanceId === primary.instanceId)
    || spawned.some((entry) => entry.pluginId === source.pluginId && entry.appId === source.appId && entry.instanceId === source.sessionInstanceId);
}

/** 🚪️ Creation is admitted before its await and before publication; an expired result is retired exactly once. */
export async function createAdmittedShellInstanceV1<T>(admit: () => boolean, create: () => Promise<T>, retire: (instance: T) => Promise<void>): Promise<T | null> {
  if (!admit()) return null;
  const instance = await create();
  if (admit()) return instance;
  await retire(instance);
  return null;
}
