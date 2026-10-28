import { expect, test } from "bun:test";
import { LocalSourceRevisionRegistryV1 } from "./registry.ts";
import type { SourceRevisionOwnerV1 } from "./🟦️.ts";

function owner() {
  let active = true;
  const listeners = new Set<() => void>();
  const value: SourceRevisionOwnerV1 = {
    identity: { sessionInstanceId: 7 }, get active() { return active; },
    subscribeRetirement(listener) { listeners.add(listener); return () => { listeners.delete(listener); }; },
  };
  return { value, retire() { active = false; for (const listener of [...listeners]) listener(); } };
}

test("focus changes retain each exact source and original in-flight hold", () => {
  const registry = new LocalSourceRevisionRegistryV1(), a = owner(), b = owner(), plugin = {};
  const first = registry.acquire(a.value, plugin)!;
  const release = first.hold(a.value, plugin)!;
  const second = registry.acquire(b.value, plugin)!;
  expect(first).not.toBe(second);
  expect(second.begin(b.value, plugin)).not.toBeNull();
  expect(registry.acquire(a.value, plugin)).toBe(first);
  expect(first.begin(a.value, plugin)).toBeNull();
  release();
  expect(first.begin(a.value, plugin)).not.toBeNull();
  a.retire();
  expect(registry.acquire(a.value, plugin)).toBeNull();
  expect(registry.acquire(b.value, plugin)).toBe(second);
});

test("retiring one exact plugin cannot authorize its replacement or another document", () => {
  const registry = new LocalSourceRevisionRegistryV1(), a = owner(), old = {}, next = {}, other = {};
  const previous = registry.acquire(a.value, old)!;
  const pending = previous.begin(a.value, old)!;
  const unrelated = registry.acquire(a.value, other)!;
  registry.retirePlugin(old);
  expect(pending.signal.aborted).toBe(true);
  expect(registry.acquire(a.value, old)).toBeNull();
  expect(registry.acquire(a.value, next)).not.toBe(previous);
  expect(registry.acquire(a.value, other)).toBe(unrelated);
});

test("registry capacity refuses excess owners and host reinitialization issues fresh sources", () => {
  const registry = new LocalSourceRevisionRegistryV1(1), a = owner(), b = owner(), plugin = {};
  const first = registry.acquire(a.value, plugin)!;
  expect(registry.acquire(b.value, plugin)).toBeNull();
  a.retire();
  expect(registry.acquire(b.value, plugin)).toBeNull();
  b.retire();
  const c = owner();
  const next = registry.acquire(c.value, plugin)!;
  expect(next).not.toBeNull();
  const pending = next.begin(c.value, plugin)!;
  registry.retireAll();
  expect(pending.signal.aborted).toBe(true);
  expect(registry.size).toBe(0);
  const restarted = registry.acquire(c.value, plugin)!;
  expect(restarted).not.toBe(next);
  expect(next.isBound(c.value, plugin)).toBe(false);
  expect(restarted.begin(c.value, plugin)).not.toBeNull();
  expect(first.isBound(a.value, plugin)).toBe(false);
});
