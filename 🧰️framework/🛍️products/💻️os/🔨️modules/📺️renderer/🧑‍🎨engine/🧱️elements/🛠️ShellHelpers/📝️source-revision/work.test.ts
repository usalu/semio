import { expect, test } from "bun:test";
import { LocalSourceRevisionV1, type SourceRevisionOwnerV1 } from "./🟦️.ts";
import { beginSourceRevisionWorkV1, type SourceWorkCompletionV1 } from "./work.ts";

function fixture() {
  const source = new LocalSourceRevisionV1(), plugin = {};
  let active = true, refreshes = 0;
  const retirement = new Set<() => void>();
  const completions = new Set<(completion: SourceWorkCompletionV1) => void>();
  const owner: SourceRevisionOwnerV1 = {
    identity: { sessionInstanceId: 7 },
    get active() { return active; },
    subscribeRetirement(listener) { retirement.add(listener); return () => { retirement.delete(listener); }; },
  };
  source.bind(owner, plugin);
  return {
    source, owner, plugin,
    begin: () => beginSourceRevisionWorkV1({ source, owner, plugin, subscribe: listener => { completions.add(listener); return () => { completions.delete(listener); }; }, refresh: () => { refreshes++; } })!,
    complete: (operation: bigint, instanceId = 7) => { for (const listener of [...completions]) listener({ instanceId, operation }); },
    retire: () => { active = false; for (const listener of [...retirement]) listener(); },
    get listeners() { return completions.size; },
    get refreshes() { return refreshes; },
  };
}

test("watchdog null keeps original hold until matching terminal completion", () => {
  const f = fixture(), work = f.begin();
  work.admit(9n);
  work.finish();
  expect(f.source.begin(f.owner, f.plugin)).toBeNull();
  f.complete(8n);
  f.complete(9n, 8);
  expect(f.source.begin(f.owner, f.plugin)).toBeNull();
  expect(f.refreshes).toBe(0);
  f.complete(9n);
  expect(f.source.begin(f.owner, f.plugin)).not.toBeNull();
  expect(f.listeners).toBe(0);
  expect(f.refreshes).toBe(1);
});

test("completion cannot reopen while dispatch effects are still active", () => {
  const f = fixture(), work = f.begin();
  work.admit(9n);
  f.complete(9n);
  expect(f.source.begin(f.owner, f.plugin)).toBeNull();
  work.finish();
  expect(f.source.begin(f.owner, f.plugin)).not.toBeNull();
  work.finish();
  f.complete(9n);
  expect(f.refreshes).toBe(1);
});

test("an authentic early completion from the exact subscribed channel closes the same hold", () => {
  const f = fixture(), work = f.begin();
  f.complete(9n);
  work.admit(9n);
  work.finish();
  expect(f.source.begin(f.owner, f.plugin)).not.toBeNull();
  expect(f.listeners).toBe(0);
});

test("no-operation admission closes only after dispatch, lost admission stays unresolved", () => {
  const f = fixture(), known = f.begin();
  known.admit(undefined);
  expect(f.source.begin(f.owner, f.plugin)).toBeNull();
  known.finish();
  expect(f.source.begin(f.owner, f.plugin)).not.toBeNull();
  const uncertain = f.begin();
  uncertain.finish();
  f.complete(4n);
  expect(f.source.begin(f.owner, f.plugin)).toBeNull();
  f.retire();
  expect(f.listeners).toBe(0);
});

test("retired owner and duplicate finish cannot release a replacement lifetime", () => {
  const f = fixture(), old = f.begin();
  old.admit(9n);
  old.finish();
  f.retire();
  expect(f.listeners).toBe(0);
  expect(f.refreshes).toBe(0);
  f.complete(9n);
  old.finish();
  expect(f.refreshes).toBe(0);
});

test("overlapping operations require every original completion", () => {
  const f = fixture(), one = f.begin(), two = f.begin();
  one.admit(1n); two.admit(2n);
  one.finish(); two.finish();
  f.complete(1n);
  expect(f.source.begin(f.owner, f.plugin)).toBeNull();
  f.complete(2n);
  expect(f.source.begin(f.owner, f.plugin)).not.toBeNull();
  expect(f.listeners).toBe(0);
});

test("settlement and admission replacements cannot forge a terminal completion", () => {
  const f = fixture(), work = f.begin();
  work.admit(9n);
  work.admit(undefined);
  f.complete(8n);
  work.finish();
  expect(f.source.begin(f.owner, f.plugin)).toBeNull();
  f.complete(9n);
  expect(f.source.begin(f.owner, f.plugin)).not.toBeNull();
});

test("duplicate pending operation authority stays unresolved even after an earlier terminal", () => {
  const f = fixture(), one = f.begin(), two = f.begin();
  one.admit(9n);
  f.complete(9n);
  two.admit(9n);
  one.finish(); two.finish();
  f.complete(9n);
  expect(f.source.begin(f.owner, f.plugin)).toBeNull();
  f.retire();
  expect(f.listeners).toBe(0);
});

test("bounded early completion evidence never invents an omitted terminal", () => {
  const f = fixture(), work = f.begin();
  for (let i = 0n; i < 64n; i++) f.complete(i);
  f.complete(70n);
  work.admit(70n);
  work.finish();
  expect(f.source.begin(f.owner, f.plugin)).toBeNull();
  f.complete(70n);
  expect(f.source.begin(f.owner, f.plugin)).not.toBeNull();
});
