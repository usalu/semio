// #region 🧲️Header
/** 🧭️ Shell-local owner identities for transient document interaction state. */
// #endregion 🧲️Header

import { createContext, useContext } from "react";

export type LocalDocumentOwnerIdentityV1 = Readonly<{
  pluginId: string;
  appId: string;
  sessionInstanceId: number;
  runtimeKey: string;
  clientInstanceId: string;
}>;

export type LocalDocumentSessionIdentityV1 = Readonly<{ pluginId: string; appId: string; sessionInstanceId: number }>;
export type LocalDocumentAttachmentIdentityV1 = Readonly<{ runtimeKey: string; clientInstanceId: string }>;

export type LocalDocumentOwnerV1 = Readonly<{
  id: string;
  identity: LocalDocumentOwnerIdentityV1;
  active: boolean;
  subscribeRetirement(listener: () => void): () => void;
}>;

const ownerPart = (value: string): string => `${new TextEncoder().encode(value).byteLength}:${value}`;

/** 🪪️ Encodes every exact document authority component without delimiter ambiguity. */
export function localDocumentOwnerIdV1(identity: LocalDocumentOwnerIdentityV1): string {
  return [identity.pluginId, identity.appId, String(identity.sessionInstanceId), identity.runtimeKey, identity.clientInstanceId].map(ownerPart).join("");
}

/** 🐣️ Gives a native/demo session a real transient owner before it is attached to a catalog document, then resolves the
 * exact document authority once attachment supplies its runtime and client identities. */
export function localDocumentOwnerIdentityForSessionV1(session: LocalDocumentSessionIdentityV1, attachment?: LocalDocumentAttachmentIdentityV1): LocalDocumentOwnerIdentityV1 {
  return {
    ...session,
    runtimeKey: attachment?.runtimeKey ?? "shell.primary-session",
    clientInstanceId: attachment?.clientInstanceId ?? "shell.primary-session",
  };
}

class LocalDocumentOwnerRecordV1 implements LocalDocumentOwnerV1 {
  readonly id: string;
  readonly identity: LocalDocumentOwnerIdentityV1;
  #active = true;
  readonly #retirement = new Set<() => void>();

  constructor(identity: LocalDocumentOwnerIdentityV1) {
    this.identity = Object.freeze({ ...identity });
    this.id = localDocumentOwnerIdV1(identity);
  }

  get active(): boolean { return this.#active; }

  subscribeRetirement(listener: () => void): () => void {
    if (!this.#active) {
      listener();
      return () => {};
    }
    this.#retirement.add(listener);
    return () => { this.#retirement.delete(listener); };
  }

  retire(): void {
    if (!this.#active) return;
    this.#active = false;
    const listeners = [...this.#retirement];
    this.#retirement.clear();
    for (const listener of listeners) listener();
  }
}

export const LOCAL_DOCUMENT_OWNER_CAPACITY_V1 = 256;

export const LocalDocumentOwnerContext = createContext<LocalDocumentOwnerV1 | null>(null);
export const LocalDocumentWindowContext = createContext<string | null>(null);

/** 🧩️ Reads the exact shell-owned document lifetime of the interpreted window. */
export function useLocalDocumentOwnerV1(): LocalDocumentOwnerV1 | null { return useContext(LocalDocumentOwnerContext); }

/** 🪟️ Reads the interpreted window lane inside the exact local document owner. */
export function useLocalDocumentWindowV1(): string | null { return useContext(LocalDocumentWindowContext); }

/** 🧺️ Owns stable transient identities for one shell and retires them with their exact document. */
export class LocalDocumentOwnerRegistryV1 {
  readonly #owners = new Map<string, LocalDocumentOwnerRecordV1>();

  constructor(readonly capacity: number = LOCAL_DOCUMENT_OWNER_CAPACITY_V1) {
    if (!Number.isSafeInteger(capacity) || capacity < 1) throw new RangeError("local document owner capacity must be a positive safe integer");
  }

  acquire(identity: LocalDocumentOwnerIdentityV1): LocalDocumentOwnerV1 {
    const id = localDocumentOwnerIdV1(identity);
    const current = this.#owners.get(id);
    if (current?.active) return current;
    if (this.#owners.size >= this.capacity) throw new RangeError("local document owner capacity is exhausted");
    const owner = new LocalDocumentOwnerRecordV1(identity);
    this.#owners.set(id, owner);
    return owner;
  }

  retire(identity: LocalDocumentOwnerIdentityV1): void { this.retireId(localDocumentOwnerIdV1(identity)); }

  retireId(id: string): void {
    const owner = this.#owners.get(id);
    if (!owner) return;
    this.#owners.delete(id);
    owner.retire();
  }

  retain(identities: readonly LocalDocumentOwnerIdentityV1[]): void {
    const active = new Set(identities.map(localDocumentOwnerIdV1));
    for (const id of [...this.#owners.keys()]) if (!active.has(id)) this.retireId(id);
  }

  retireAll(): void {
    const owners = [...this.#owners.values()];
    this.#owners.clear();
    for (const owner of owners) owner.retire();
  }

  get size(): number { return this.#owners.size; }
}
