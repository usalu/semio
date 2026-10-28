/** 🪪️ Retains an admitted scalar source identity for one exact document and plugin lifetime. */
export type SourceRevisionIdentityV1 = Readonly<{
  appInstanceId: number;
  generation: string;
  revision: string;
  documentRevision: string;
  topologyRevision: string;
}>;

export type SourceRevisionOwnerV1 = Readonly<{
  identity: Readonly<{ sessionInstanceId: number }>;
  active: boolean;
  subscribeRetirement(listener: () => void): () => void;
}>;

export type SourceRevisionObservationV1 = Readonly<{ signal: AbortSignal }>;

type Pending = Readonly<{ observation: SourceRevisionObservationV1; controller: AbortController }>;

const hash = (value: string): boolean => typeof value === "string" && /^[0-9a-f]{64}$/u.test(value);
const generation = (value: string): boolean => typeof value === "string" && /^(0|[1-9][0-9]{0,19})$/u.test(value) && BigInt(value) <= 18_446_744_073_709_551_615n;
export const SOURCE_REVISION_HOLD_LIMIT_V1 = 256;

/** 🔐️ Requires exact owner and handle identity at every read and publication; a hash is never lifetime authority. */
export class LocalSourceRevisionV1 {
  #owner: SourceRevisionOwnerV1 | null = null;
  #plugin: object | null = null;
  #identity: SourceRevisionIdentityV1 | null = null;
  #pending: Pending | null = null;
  #accepted: SourceRevisionObservationV1 | null = null;
  #unsubscribe: (() => void) | null = null;
  #epoch: object = {};
  #holds = new Set<symbol>();
  #exhausted = false;
  readonly #listeners = new Set<() => void>();

  bind(owner: SourceRevisionOwnerV1, plugin: object): void {
    if (this.#matches(owner, plugin)) return;
    const epoch = this.#clear(true);
    if (this.#epoch !== epoch || !owner.active) return;
    this.#owner = owner;
    this.#plugin = plugin;
    const unsubscribe = owner.subscribeRetirement(() => {
      if (this.#owner === owner && this.#plugin === plugin) this.retire();
    });
    if (this.#epoch === epoch && this.#matches(owner, plugin)) this.#unsubscribe = unsubscribe;
    else unsubscribe();
  }

  begin(owner: SourceRevisionOwnerV1, plugin: object): SourceRevisionObservationV1 | null {
    if (!this.#matches(owner, plugin)) return null;
    if (this.#holds.size > 0 || this.#exhausted) return null;
    const epoch = this.#clear();
    if (this.#epoch !== epoch || !this.#matches(owner, plugin) || this.#holds.size > 0 || this.#exhausted) return null;
    const controller = new AbortController();
    const observation = Object.freeze({ signal: controller.signal });
    this.#pending = { observation, controller };
    this.#notify();
    return observation;
  }

  publish(observation: SourceRevisionObservationV1, identity: SourceRevisionIdentityV1): boolean {
    if (this.#pending?.observation !== observation || !this.#owner?.active) return false;
    if (identity.appInstanceId !== this.#owner.identity.sessionInstanceId || !generation(identity.generation) || !hash(identity.revision) || !hash(identity.documentRevision) || !hash(identity.topologyRevision)) {
      this.fail(observation);
      return false;
    }
    this.#identity = Object.freeze({
      appInstanceId: identity.appInstanceId,
      generation: identity.generation,
      revision: identity.revision,
      documentRevision: identity.documentRevision,
      topologyRevision: identity.topologyRevision,
    });
    this.#pending = null;
    this.#accepted = observation;
    this.#epoch = {};
    this.#notify();
    return true;
  }

  fail(observation: SourceRevisionObservationV1): void {
    if (this.#pending?.observation === observation) this.#clear();
  }

  isCurrent(observation: SourceRevisionObservationV1): boolean {
    return this.#accepted === observation && this.#owner?.active === true && this.#holds.size === 0;
  }

  invalidate(owner: SourceRevisionOwnerV1, plugin: object): boolean {
    if (!this.#matches(owner, plugin)) return false;
    this.#clear();
    return true;
  }

  hold(owner: SourceRevisionOwnerV1, plugin: object): (() => void) | null {
    if (!this.#matches(owner, plugin)) return null;
    if (this.#exhausted || this.#holds.size >= SOURCE_REVISION_HOLD_LIMIT_V1) {
      this.#exhausted = true;
      this.#clear();
      return null;
    }
    const holds = this.#holds;
    const token = Symbol();
    holds.add(token);
    this.#clear();
    return () => {
      if (this.#holds === holds && holds.delete(token)) this.#clear();
    };
  }

  read(owner: SourceRevisionOwnerV1, plugin: object): SourceRevisionIdentityV1 | null {
    return this.#holds.size === 0 && !this.#exhausted && this.#matches(owner, plugin) ? this.#identity : null;
  }

  isBound(owner: SourceRevisionOwnerV1, plugin: object): boolean { return this.#matches(owner, plugin); }

  retire(): void {
    this.#clear(true);
  }

  subscribe(listener: () => void): () => void {
    this.#listeners.add(listener);
    return () => { this.#listeners.delete(listener); };
  }

  #matches(owner: SourceRevisionOwnerV1, plugin: object): boolean {
    return owner.active && this.#owner === owner && this.#plugin === plugin;
  }

  #clear(retire = false): object {
    const epoch = {};
    this.#epoch = epoch;
    const pending = this.#pending;
    const changed = pending !== null || this.#identity !== null || (retire && this.#owner !== null);
    this.#pending = null;
    this.#accepted = null;
    this.#identity = null;
    const unsubscribe = retire ? this.#unsubscribe : null;
    if (retire) {
      this.#unsubscribe = null;
      this.#owner = null;
      this.#plugin = null;
      this.#holds = new Set();
      this.#exhausted = false;
    }
    unsubscribe?.();
    pending?.controller.abort();
    if (changed) this.#notify();
    return epoch;
  }

  #notify(): void {
    for (const listener of [...this.#listeners]) listener();
  }
}
