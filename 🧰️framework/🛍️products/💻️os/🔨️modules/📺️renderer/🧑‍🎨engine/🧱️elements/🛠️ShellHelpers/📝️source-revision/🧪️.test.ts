import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { compare, applyPatch } from "fast-json-patch";
import cases from "./cases.json";
import schema from "./cases.schema.json";
import { LocalSourceRevisionV1, SOURCE_REVISION_HOLD_LIMIT_V1, type SourceRevisionObservationV1, type SourceRevisionOwnerV1 } from "./🟦️.ts";
import { readSourceRevisionV1 } from "./observation.ts";

function owner(): SourceRevisionOwnerV1 & { retire(): void } {
  let active = true;
  const listeners = new Set<() => void>();
  return {
    identity: { sessionInstanceId: 7 },
    get active() { return active; },
    subscribeRetirement(listener) { listeners.add(listener); return () => { listeners.delete(listener); }; },
    retire() { active = false; for (const listener of [...listeners]) listener(); },
  };
}

const identity = () => ({ appInstanceId: 7, generation: "18446744073709551615", revision: "a".repeat(64), documentRevision: "b".repeat(64), topologyRevision: "c".repeat(64) });

describe("canonical source revision custody", () => {
  test("neutral closed cases", () => {
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(cases)).toBe(true);
    expect(validate({ ...cases, unexpected: true })).toBe(false);
  });

  for (const row of cases.cases) test(row.id, () => {
    const cache = new LocalSourceRevisionV1();
    let active = owner();
    let plugin = {};
    let current: SourceRevisionObservationV1 | null = null;
    let old: SourceRevisionObservationV1 | null = null;
    cache.bind(active, plugin);
    for (const event of row.events) {
      if (event === "begin") { old = current; current = cache.begin(active, plugin); }
      else if (event === "publish") { if (current) cache.publish(current, identity()); }
      else if (event === "publish-old") { if (old ?? current) cache.publish((old ?? current)!, identity()); }
      else if (event === "invalidate") cache.invalidate(active, plugin);
      else if (event === "retire") active.retire();
      else if (event === "replace-owner") { old = current; active = owner(); cache.bind(active, plugin); }
      else if (event === "replace-plugin") { old = current; plugin = {}; cache.bind(active, plugin); }
    }
    const actual = { ready: cache.read(active, plugin) !== null };
    expect(actual.ready).toBe(row.ready);
    const before = { ready: false };
    expect(applyPatch(before, compare(before, { ready: row.ready }), true, false).newDocument).toEqual(actual);
  });

  test("invalidation synchronously aborts and clears before callbacks", () => {
    const cache = new LocalSourceRevisionV1(), active = owner(), plugin = {};
    cache.bind(active, plugin);
    const first = cache.begin(active, plugin)!;
    let observed: unknown = "not-called";
    first.signal.addEventListener("abort", () => { observed = cache.read(active, plugin); });
    const second = cache.begin(active, plugin)!;
    expect(first.signal.aborted).toBe(true);
    expect(observed).toBeNull();
    expect(second.signal.aborted).toBe(false);
    expect(cache.publish(first, identity())).toBe(false);
    expect(cache.publish(second, identity())).toBe(true);
  });

  test("same-owner invalidation cannot retire another document", () => {
    const cache = new LocalSourceRevisionV1(), active = owner(), plugin = {};
    cache.bind(active, plugin);
    cache.publish(cache.begin(active, plugin)!, identity());
    expect(cache.invalidate(owner(), plugin)).toBe(false);
    expect(cache.invalidate(active, {})).toBe(false);
    expect(cache.read(active, plugin)?.documentRevision).toBe("b".repeat(64));
    expect(cache.read(owner(), plugin)).toBeNull();
    expect(cache.read(active, {})).toBeNull();
  });

  test("scalar identity is privately retained and frozen", () => {
    const cache = new LocalSourceRevisionV1(), active = owner(), plugin = {}, captured = identity();
    cache.bind(active, plugin);
    cache.publish(cache.begin(active, plugin)!, captured);
    captured.documentRevision = "d".repeat(64);
    expect(cache.read(active, plugin)?.documentRevision).toBe("b".repeat(64));
    expect(Object.isFrozen(cache.read(active, plugin))).toBe(true);
  });

  test("closed identities reject mismatched instance, truncated hash and overflow generation", () => {
    for (const invalid of [{ ...identity(), appInstanceId: 8 }, { ...identity(), documentRevision: "b".repeat(16) }, { ...identity(), revision: "A".repeat(64) }, { ...identity(), generation: "18446744073709551616" }]) {
      const cache = new LocalSourceRevisionV1(), active = owner(), plugin = {};
      cache.bind(active, plugin);
      expect(cache.publish(cache.begin(active, plugin)!, invalid)).toBe(false);
      expect(cache.read(active, plugin)).toBeNull();
    }
  });

  test("forged, finished, failed and retired observations cannot publish", () => {
    const cache = new LocalSourceRevisionV1(), active = owner(), plugin = {};
    cache.bind(active, plugin);
    expect(cache.publish({ signal: new AbortController().signal }, identity())).toBe(false);
    const failed = cache.begin(active, plugin)!;
    cache.fail(failed);
    expect(failed.signal.aborted).toBe(true);
    expect(cache.publish(failed, identity())).toBe(false);
    const done = cache.begin(active, plugin)!;
    expect(cache.publish(done, identity())).toBe(true);
    expect(cache.publish(done, identity())).toBe(false);
    active.retire();
    expect(cache.begin(active, plugin)).toBeNull();
    expect(cache.read(active, plugin)).toBeNull();
  });

  test("old retirement cannot clear a replacement owner", () => {
    const cache = new LocalSourceRevisionV1(), previous = owner(), active = owner(), plugin = {};
    cache.bind(previous, plugin);
    cache.begin(previous, plugin);
    cache.bind(active, plugin);
    cache.publish(cache.begin(active, plugin)!, identity());
    previous.retire();
    expect(cache.read(active, plugin)?.documentRevision).toBe("b".repeat(64));
  });

  test("overlapping source work keeps readiness retired until every original hold closes", () => {
    const cache = new LocalSourceRevisionV1(), active = owner(), plugin = {};
    cache.bind(active, plugin);
    const pending = cache.begin(active, plugin)!;
    const one = cache.hold(active, plugin)!;
    const two = cache.hold(active, plugin)!;
    expect(pending.signal.aborted).toBe(true);
    expect(cache.begin(active, plugin)).toBeNull();
    one();
    one();
    expect(cache.begin(active, plugin)).toBeNull();
    two();
    expect(cache.read(active, plugin)).toBeNull();
    expect(cache.publish(cache.begin(active, plugin)!, identity())).toBe(true);
    const oldRelease = cache.hold(active, plugin)!;
    const next = owner();
    cache.bind(next, plugin);
    cache.publish(cache.begin(next, plugin)!, identity());
    oldRelease();
    expect(cache.read(next, plugin)).not.toBeNull();
  });

  test("abort reentrancy cannot replace a newer source observation", () => {
    const cache = new LocalSourceRevisionV1(), active = owner(), plugin = {};
    cache.bind(active, plugin);
    const first = cache.begin(active, plugin)!;
    let reentrant: SourceRevisionObservationV1 | null = null;
    first.signal.addEventListener("abort", () => { reentrant = cache.begin(active, plugin); });
    expect(cache.begin(active, plugin)).toBeNull();
    expect(reentrant).not.toBeNull();
    expect(cache.publish(reentrant!, identity())).toBe(true);
  });

  test("subscribers observe pending, ready and retirement", () => {
    const cache = new LocalSourceRevisionV1(), active = owner(), plugin = {};
    const ready: boolean[] = [];
    const unsubscribe = cache.subscribe(() => ready.push(cache.read(active, plugin) !== null));
    cache.bind(active, plugin);
    cache.publish(cache.begin(active, plugin)!, identity());
    cache.begin(active, plugin);
    active.retire();
    expect(ready).toContain(true);
    expect(ready.at(-1)).toBe(false);
    unsubscribe();
    const length = ready.length;
    cache.retire();
    expect(ready.length).toBe(length);
  });

  test("hold saturation remains unresolved after retained holds release", () => {
    const cache = new LocalSourceRevisionV1(), active = owner(), plugin = {};
    cache.bind(active, plugin);
    const releases: (() => void)[] = [];
    for (let i = 0; i < SOURCE_REVISION_HOLD_LIMIT_V1; i++) releases.push(cache.hold(active, plugin)!);
    expect(cache.hold(active, plugin)).toBeNull();
    for (const release of releases) release();
    expect(cache.begin(active, plugin)).toBeNull();
    const next = owner();
    cache.bind(next, plugin);
    expect(cache.begin(next, plugin)).not.toBeNull();
  });

  test("actual observation adapter uses the original signal and newest admitted response", async () => {
    const cache = new LocalSourceRevisionV1(), active = owner(), plugin = {};
    cache.bind(active, plugin);
    const first = cache.begin(active, plugin)!;
    const accepted: string[] = [];
    let finish!: (capture: { identity: ReturnType<typeof identity>; state: string }) => void;
    const old = readSourceRevisionV1<string>({
      source: cache, observation: first, current: () => active.active,
      capture: signal => { expect(signal).toBe(first.signal); return new Promise(resolve => { finish = resolve; }); },
      accept: state => { accepted.push(state); },
    });
    const second = cache.begin(active, plugin)!;
    expect(await readSourceRevisionV1({
      source: cache, observation: second, current: () => active.active,
      capture: async signal => { expect(signal).toBe(second.signal); return { identity: identity(), state: "new" }; },
      accept: state => { accepted.push(state); },
    })).toBe("new");
    finish({ identity: identity(), state: "old" });
    expect(await old).toBeNull();
    expect(accepted).toEqual(["new"]);
    expect(cache.isCurrent(first)).toBe(false);
    expect(cache.isCurrent(second)).toBe(true);
  });

  test("actual observation adapter retires mismatched owner, cancellation and failed transport", async () => {
    const cache = new LocalSourceRevisionV1(), active = owner(), plugin = {};
    cache.bind(active, plugin);
    const accept = () => { throw new Error("stale response accepted"); };
    expect(await readSourceRevisionV1({ source: cache, observation: cache.begin(active, plugin)!, current: () => false, capture: async () => ({ identity: identity(), state: 1 }), accept })).toBeNull();
    const cancelled = cache.begin(active, plugin)!;
    cache.invalidate(active, plugin);
    expect(await readSourceRevisionV1({ source: cache, observation: cancelled, current: () => true, capture: async () => { throw new Error("retired transport invoked"); }, accept })).toBeNull();
    const failure = new Error("original transport failure");
    const failed = cache.begin(active, plugin)!;
    await expect(readSourceRevisionV1({ source: cache, observation: failed, current: () => true, capture: async () => { throw failure; }, accept })).rejects.toBe(failure);
    expect(failed.signal.aborted).toBe(true);
    expect(cache.read(active, plugin)).toBeNull();
  });

  test("reentrant publication retirement cannot accept a stale interaction state", async () => {
    const cache = new LocalSourceRevisionV1(), active = owner(), plugin = {};
    cache.bind(active, plugin);
    const observation = cache.begin(active, plugin)!;
    cache.subscribe(() => { if (cache.isCurrent(observation)) active.retire(); });
    let accepted = false;
    expect(await readSourceRevisionV1({
      source: cache, observation, current: () => active.active,
      capture: async () => ({ identity: identity(), state: 1 }),
      accept: () => { accepted = true; },
    })).toBeNull();
    expect(accepted).toBe(false);
  });

  test("reentrant subscription cannot lose the replacement owner retirement", () => {
    const cache = new LocalSourceRevisionV1(), active = owner(), plugin = {};
    let disposed = false;
    const previous: SourceRevisionOwnerV1 = {
      identity: { sessionInstanceId: 7 }, active: true,
      subscribeRetirement() { cache.bind(active, plugin); return () => { disposed = true; }; },
    };
    cache.bind(previous, plugin);
    expect(disposed).toBe(true);
    expect(cache.publish(cache.begin(active, plugin)!, identity())).toBe(true);
    active.retire();
    expect(cache.read(active, plugin)).toBeNull();
  });
});
