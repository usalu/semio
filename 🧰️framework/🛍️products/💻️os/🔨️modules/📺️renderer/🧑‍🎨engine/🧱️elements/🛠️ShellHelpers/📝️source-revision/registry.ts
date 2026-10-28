import { LocalSourceRevisionV1, type SourceRevisionOwnerV1 } from "./🟦️.ts";

/** 🗂️ Keeps each real document's source work alive while focus moves to another document. */
export class LocalSourceRevisionRegistryV1 {
  #owners = new WeakMap<SourceRevisionOwnerV1, WeakMap<object, LocalSourceRevisionV1>>();
  #refused = new WeakMap<SourceRevisionOwnerV1, WeakSet<object>>();
  readonly #retiredPlugins = new WeakSet<object>();
  readonly #active = new Map<LocalSourceRevisionV1, Readonly<{ owner: SourceRevisionOwnerV1; plugin: object; unsubscribe(): void }>>();

  constructor(readonly capacity = 1024) {
    if (!Number.isSafeInteger(capacity) || capacity < 1) throw new RangeError("source owner capacity must be a positive safe integer");
  }

  acquire(owner: SourceRevisionOwnerV1, plugin: object): LocalSourceRevisionV1 | null {
    if (!owner.active || this.#retiredPlugins.has(plugin) || this.#refused.get(owner)?.has(plugin)) return null;
    let plugins = this.#owners.get(owner);
    const existing = plugins?.get(plugin);
    if (existing) return existing.isBound(owner, plugin) ? existing : null;
    if (this.#active.size >= this.capacity) {
      let plugins = this.#refused.get(owner);
      if (!plugins) this.#refused.set(owner, plugins = new WeakSet());
      plugins.add(plugin);
      return null;
    }
    if (!plugins) this.#owners.set(owner, plugins = new WeakMap());
    const source = new LocalSourceRevisionV1();
    source.bind(owner, plugin);
    if (!source.isBound(owner, plugin)) return null;
    plugins.set(plugin, source);
    const unsubscribe = source.subscribe(() => {
      if (source.isBound(owner, plugin)) return;
      this.#active.delete(source);
      unsubscribe();
    });
    this.#active.set(source, { owner, plugin, unsubscribe });
    return source;
  }

  retirePlugin(plugin: object): void {
    this.#retiredPlugins.add(plugin);
    for (const [source, entry] of this.#active) if (entry.plugin === plugin) source.retire();
  }

  retireAll(): void {
    for (const source of this.#active.keys()) source.retire();
    this.#owners = new WeakMap();
    this.#refused = new WeakMap();
  }

  get size(): number { return this.#active.size; }
}
