/** 🚪️ Approves every captured program before any attachment or instance retires. */
import type { AppDefinition } from "@semio-tech/framework";
import { parsePanelState } from "../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import type { PluginWasmHandle } from "../../🔌️PluginRuntime/🟦️.tsx";
import type { ActiveSession } from "../../🐚️Shell/📦️program-sessions/🟦️.ts";

export type ProgramRetirementOwnerV1 = Readonly<{ handle: PluginWasmHandle; session: ActiveSession; name: string }>;
export type ProgramRetirementAdmissionV1 = Readonly<{ current: () => boolean; release: () => void }>;
export type ProgramRetirementLeaseV1 = ProgramRetirementAdmissionV1 & Readonly<{ retire: () => Promise<boolean> }>;
export type ProgramRetirementRequestV1 = (owners: readonly ProgramRetirementOwnerV1[], current: () => boolean) => Promise<ProgramRetirementLeaseV1 | null>;

export async function admitProgramRetirementV1<T>(ports: Readonly<{
  owners: readonly T[];
  signal: AbortSignal;
  current: () => boolean;
  quiesce: (owner: T) => Promise<boolean>;
  seal: (owner: T) => () => void;
  persist: (owner: T) => Promise<void>;
  confirmDiscard: (owner: T, failure: unknown) => Promise<boolean>;
}>): Promise<ProgramRetirementAdmissionV1 | null> {
  const wait = <R>(operation: Promise<R>): Promise<R> => new Promise((resolve, reject) => {
    const abort = (): void => reject(new Error("program-retirement.cancelled"));
    operation.then(resolve, reject).finally(() => ports.signal.removeEventListener("abort", abort));
    if (ports.signal.aborted) return abort();
    ports.signal.addEventListener("abort", abort, { once: true });
  });
  const releases: (() => void)[] = [];
  let released = false, approved = false;
  const current = (): boolean => !released && !ports.signal.aborted && ports.current();
  const release = (): void => {
    if (released) return;
    released = true;
    for (const unseal of releases.reverse()) unseal();
  };
  try {
    for (const owner of ports.owners) {
      if (!current() || !await wait(ports.quiesce(owner)) || !current()) return null;
      releases.push(ports.seal(owner));
    }
    for (const owner of ports.owners) {
      if (!current()) return null;
      try {
        await wait(ports.persist(owner));
      } catch (failure) {
        if (!current() || !await wait(ports.confirmDiscard(owner, failure))) return null;
      }
      if (!current()) return null;
    }
    approved = current();
    return approved ? { current, release } : null;
  } catch (failure) {
    if (ports.signal.aborted) return null;
    throw failure;
  } finally {
    if (!approved) release();
  }
}

/** 🪪️ Captures the exact main and spawned owners a host or plugin retirement removes. */
export function captureProgramRetirementOwnersV1(loaded: readonly Readonly<{ handle: PluginWasmHandle; manifest: Readonly<{ apps: readonly AppDefinition[] }> }>[], host: ActiveSession | null, pluginId?: string): readonly ProgramRetirementOwnerV1[] {
  if (host === null) return [];
  const owners: ProgramRetirementOwnerV1[] = [];
  const add = (session: ActiveSession, name: string): void => {
    if (pluginId !== undefined && session.pluginId !== pluginId) return;
    const handle = loaded.find(entry => entry.handle.pluginId === session.pluginId)?.handle;
    if (!handle) throw new Error("program-retirement.owner-unavailable");
    if (!owners.some(owner => owner.handle === handle && owner.session.instanceId === session.instanceId)) owners.push({ handle, session, name });
  };
  add(host, host.app.id);
  for (const spawned of parsePanelState(host.viewState)?.spawnedApps ?? []) {
    if (pluginId !== undefined && spawned.pluginId !== pluginId) continue;
    const app = loaded.find(entry => entry.handle.pluginId === spawned.pluginId)?.manifest.apps.find(app => app.id === spawned.appId);
    if (!app) throw new Error("program-retirement.app-unavailable");
    add({ pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: host.viewState }, spawned.label);
  }
  return owners;
}

export function sameProgramRetirementOwnersV1(captured: readonly ProgramRetirementOwnerV1[], live: readonly ProgramRetirementOwnerV1[]): boolean {
  return captured.length === live.length && captured.every(owner => live.some(candidate => candidate.handle === owner.handle && candidate.session.pluginId === owner.session.pluginId && candidate.session.app.id === owner.session.app.id && candidate.session.instanceId === owner.session.instanceId));
}
