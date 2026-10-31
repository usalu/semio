/** 🗃️ Own bounded opaque templates without renderer or external runtime dependencies. */
const MAX_INTEGER = Number.MAX_SAFE_INTEGER;
const integer = (value: number): void => { if (!Number.isSafeInteger(value) || value < 0) throw new RangeError("Expected a nonnegative safe integer"); };
const keyValue = (key: string): void => { if (typeof key !== "string" || [...key].length < 1 || [...key].length > 4096 || /[\uD800-\uDFFF]/u.test(key)) throw new RangeError("Expected a bounded Unicode scalar string key"); };

/** ⚖️ Measured resource rows retained by one opaque template. */
export class CacheCost {
  readonly members: number;
  readonly points: number;
  constructor(members: number, points: number) { integer(members); integer(points); this.members = members; this.points = points; Object.freeze(this); }
}

/** 📏 Independent retained-resource and scalar-history limits. */
export class CachePolicy {
  readonly max_entries: number;
  readonly max_members: number;
  readonly max_points: number;
  readonly max_history: number;
  constructor(max_entries = 8, max_members = 1000000, max_points = 10000000, max_history = 64) {
    for (const value of [max_entries, max_members, max_points, max_history]) integer(value);
    this.max_entries = max_entries; this.max_members = max_members; this.max_points = max_points; this.max_history = max_history; Object.freeze(this);
  }
}

/** 🛑 Caller cancellation at a resource lifecycle checkpoint. */
export class CacheCancelled extends Error {}

/** 🧩 Request-local controls are never retained by the cache. */
export type CacheRequest<T> = { reservation?: CacheCost; valid?: (value: T) => boolean; cancelled?: () => boolean };

/** 📊 Detached oldest-first retention and lifecycle accounting. */
export type CacheStats = { entries: number; members: number; points: number; keys: readonly string[]; history_keys: readonly string[]; reserved_members: number; reserved_points: number; hits: number; misses: number; evictions: number; bypasses: number; factory_calls: number; clone_calls: number; underestimates: number };

/** 📦 Retain canonical templates within independently accounted resource budgets. */
export class TemplateCache<T = unknown> {
  private entries = new Map<string, { value: T; cost: CacheCost }>();
  private history = new Map<string, CacheCost>();
  private members = 0;
  private points = 0;
  private reserved = new CacheCost(0, 0);
  private busy = false;
  private policy!: CachePolicy;
  private counts = { hits: 0, misses: 0, evictions: 0, bypasses: 0, factory_calls: 0, clone_calls: 0, underestimates: 0 };
  constructor(policy = new CachePolicy()) { this.configure(policy); }
  private idle(): void { if (this.busy) throw Error("Cache callbacks cannot mutate or reenter their cache"); }
  private count(name: keyof TemplateCache<T>["counts"]): void { this.counts[name] = Math.min(MAX_INTEGER, this.counts[name] + 1); }
  private drop(key: string): void {
    this.members -= this.entries.get(key)!.cost.members;
    this.points -= this.entries.get(key)!.cost.points;
    this.entries.delete(key); this.count("evictions");
  }
  private enabled(): boolean { return Math.min(this.policy.max_entries, this.policy.max_members, this.policy.max_points) > 0; }
  private space(cost: CacheCost, incoming = 1): void {
    while (this.entries.size && (this.entries.size > this.policy.max_entries - incoming || this.members > this.policy.max_members - cost.members || this.points > this.policy.max_points - cost.points)) this.drop(this.entries.keys().next().value!);
  }
  private remember(key: string, cost: CacheCost): void {
    this.history.delete(key); this.history.set(key, cost);
    while (this.history.size > this.policy.max_history) this.history.delete(this.history.keys().next().value!);
  }
  /** 🎚️ Validate a complete immutable policy before immediately applying its bounds. */
  configure(policy: CachePolicy): void {
    this.idle(); if (!(policy instanceof CachePolicy)) throw new TypeError("Expected CachePolicy");
    this.policy = policy; this.space(new CacheCost(0, 0), 0);
    if (!this.enabled()) while (this.entries.size) this.drop(this.entries.keys().next().value!);
    while (this.history.size > policy.max_history) this.history.delete(this.history.keys().next().value!);
  }
  /** 📈 Return detached resource counts and scalar keys in oldest-first order. */
  stats(): CacheStats {
    return { ...this.counts, entries: this.entries.size, members: this.members, points: this.points, keys: [...this.entries.keys()], history_keys: [...this.history.keys()], reserved_members: this.reserved.members, reserved_points: this.reserved.points };
  }
  /** 🧹 Drop owned templates and cost history without touching returned client values. */
  release(): void { this.idle(); this.entries.clear(); this.history.clear(); this.members = this.points = 0; }
  /** 🔄 Invalidate a template and every historical reservation for its exact key. */
  invalidate(key: string): void { this.idle(); keyValue(key); if (this.entries.has(key)) this.drop(key); this.history.delete(key); }
  /** 🏭 Evict before lazy construction, then admit actual cost before cloning. */
  resolve(key: string, factory: () => T, measure: (value: T) => CacheCost, clone: (value: T) => T, options: CacheRequest<T> = {}): T {
    this.idle(); keyValue(key);
    if (options.reservation !== undefined && !(options.reservation instanceof CacheCost)) throw new TypeError("Expected CacheCost reservation");
    this.busy = true;
    const checkpoint = (): void => { if (options.cancelled?.()) throw new CacheCancelled("Template request cancelled"); };
    try {
      checkpoint();
      if (this.entries.has(key)) {
        if (!options.valid || options.valid(this.entries.get(key)!.value)) {
          this.count("hits");
          const cost = this.history.get(key);
          if (cost) { this.history.delete(key); this.history.set(key, cost); }
          this.touch(key); this.count("clone_calls");
          const result = clone(this.entries.get(key)!.value); checkpoint(); return result;
        }
        this.drop(key); this.history.delete(key);
      }
      this.count("misses");
      let wanted = options.reservation ?? this.history.get(key);
      if (!wanted) {
        while (this.entries.size) this.drop(this.entries.keys().next().value!);
        wanted = new CacheCost(this.policy.max_members, this.policy.max_points);
      }
      this.history.delete(key);
      this.reserved = new CacheCost(Math.min(wanted.members, this.policy.max_members), Math.min(wanted.points, this.policy.max_points));
      this.space(this.reserved);
      if (!this.enabled()) while (this.entries.size) this.drop(this.entries.keys().next().value!);
      checkpoint(); this.count("factory_calls"); const value = factory(); checkpoint();
      const cost = measure(value); if (!(cost instanceof CacheCost)) throw new TypeError("Measurement must return CacheCost"); checkpoint();
      if (cost.members > wanted.members || cost.points > wanted.points) this.count("underestimates");
      this.reserved = new CacheCost(0, 0);
      if (!this.enabled() || cost.members > this.policy.max_members || cost.points > this.policy.max_points) { this.count("bypasses"); this.remember(key, cost); return value; }
      this.space(cost); this.count("clone_calls"); const result = clone(value); checkpoint();
      this.entries.set(key, { value, cost }); this.members += cost.members; this.points += cost.points; this.remember(key, cost); return result;
    } finally { this.reserved = new CacheCost(0, 0); this.busy = false; }
  }
  private touch(key: string): void { const entry = this.entries.get(key)!; this.entries.delete(key); this.entries.set(key, entry); }
}

/** 🧭 Restore borrowed outer contexts and release only newly owned inner contexts. */
export class CacheOwner<T = unknown> {
  current: TemplateCache<T> | undefined;
  /** 🔐 Own a fresh scope unless an explicit cache is borrowed. */
  run<R>(callback: (cache: TemplateCache<T>) => R, options: { cache?: TemplateCache<T>; policy?: CachePolicy } = {}): R {
    if (options.cache !== undefined && (!(options.cache instanceof TemplateCache) || options.policy !== undefined)) throw new TypeError("Borrow a TemplateCache or configure an owned policy");
    const previous = this.current, chosen = options.cache ?? new TemplateCache<T>(options.policy);
    this.current = chosen;
    try {
      const result = callback(chosen);
      if (result && (typeof result === "object" || typeof result === "function") && typeof (result as { then?: unknown }).then === "function") throw new TypeError("Cache scopes require synchronous callbacks");
      return result;
    } finally { this.current = previous; if (options.cache === undefined) chosen.release(); }
  }
}
