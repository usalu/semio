// #region 🧲️Header
/** 🎬️ Exact owner/Store/target/action admission for retained row commands. */
// #endregion 🧲️Header

import type { RowAction, RowTarget } from "@semio-tech/framework";

export type RowActionAdmissionOwnerV1 = Readonly<{
  active: boolean;
  subscribeRetirement(listener: () => void): () => void;
}>;

const canonical = (value: unknown): string => {
  if (value === null) return "n";
  if (value === undefined) return "u";
  if (typeof value === "string") return `s${new TextEncoder().encode(value).byteLength}:${value}`;
  if (typeof value === "number") return `d${Object.is(value, -0) ? "-0" : String(value)}`;
  if (typeof value === "bigint") return `i${value}`;
  if (typeof value === "boolean") return value ? "t" : "f";
  if (Array.isArray(value)) return `[${value.map(canonical).join("")}]`;
  if (typeof value === "object") return `{${Object.entries(value).sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0).map(([key, entry]) => `${canonical(key)}${canonical(entry)}`).join("")}}`;
  throw new TypeError(`Unsupported row-action identity value: ${typeof value}`);
};

/** 🔑️ Identifies one authored target and verb while excluding cosmetic or availability fields. */
export function rowActionAdmissionKeyV1(target: RowTarget, action: RowAction): string {
  return canonical([target, action.verb]);
}

export type RowActionAdmissionTokenV1 = Readonly<{ finish(): void }>;

/** 🎟️ Owns only currently admitted actions and retires them with their exact document owner. */
export class RowActionAdmissionRegistryV1 {
  readonly #owner: RowActionAdmissionOwnerV1;
  readonly #pending = new Map<string, object>();
  readonly #listeners = new Set<() => void>();
  #active = true;
  #revision = 0;

  constructor(owner: RowActionAdmissionOwnerV1) {
    this.#owner = owner;
    owner.subscribeRetirement(() => this.retire());
    if (!owner.active) this.retire();
  }

  get active(): boolean { return this.#active && this.#owner.active; }
  readonly subscribe = (listener: () => void): (() => void) => { this.#listeners.add(listener); return () => this.#listeners.delete(listener); };
  readonly snapshot = (): number => this.#revision;
  isPending(key: string): boolean { return this.#pending.has(key); }

  begin(key: string): RowActionAdmissionTokenV1 | null {
    if (!this.active || this.#pending.has(key)) return null;
    const identity = {};
    this.#pending.set(key, identity);
    this.#publish();
    let active = true;
    return {
      finish: () => {
        if (!active) return;
        active = false;
        if (this.#pending.get(key) !== identity) return;
        this.#pending.delete(key);
        this.#publish();
      },
    };
  }

  retire(): void {
    if (!this.#active) return;
    this.#active = false;
    this.#pending.clear();
    this.#publish();
  }

  #publish(): void {
    this.#revision += 1;
    for (const listener of [...this.#listeners]) listener();
  }
}

const registries = new WeakMap<object, WeakMap<object, RowActionAdmissionRegistryV1>>();

/** 🧭️ Shares pending state across Table and Tree projections of one exact document Store. */
export function rowActionAdmissionRegistryV1(owner: RowActionAdmissionOwnerV1, store: object): RowActionAdmissionRegistryV1 {
  let stores = registries.get(owner);
  if (!stores) {
    stores = new WeakMap();
    registries.set(owner, stores);
  }
  let registry = stores.get(store);
  if (!registry) {
    registry = new RowActionAdmissionRegistryV1(owner);
    stores.set(store, registry);
  }
  return registry;
}
