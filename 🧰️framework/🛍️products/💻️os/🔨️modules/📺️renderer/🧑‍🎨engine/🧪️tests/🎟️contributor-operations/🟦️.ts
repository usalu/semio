/** 🛰️ Shared lifecycle vectors and a fast-check model pin contributor listener custody. */
import { it as test, expect } from "vitest";
import fc from "fast-check";
import fixture from "./🧫️fixtures/🔣️.json";
import { subscribeContributorOperationLeasesV1 } from "../../🧱️elements/🏛️ShellHost/⚡️host-effects/📡️operation-subscriptions/🎟️contributors/🟦️.ts";
async function execute(actions: readonly string[]) {
  let ready!: (id: number) => void;
  const lease = { instance: new Promise<number>(resolve => { ready = resolve; }) };
  const instances = { current: new Map([["slot", lease]]) };
  const counts = { attached: 0, delivered: 0, disposed: 0 };
  let emit = () => {};
  const cleanup = subscribeContributorOperationLeasesV1(instances, (_entry, id, current) => {
    expect(id).toBe(7);
    counts.attached++;
    emit = () => { if (current()) counts.delivered++; };
    return () => { counts.disposed++; };
  }, error => { throw error; });
  for (const action of actions) {
    if (action === "ready") ready(7);
    if (action === "emit") emit();
    if (action === "retire") instances.current.delete("slot");
    if (action === "replace") instances.current.set("slot", { instance: Promise.resolve(8) });
    if (action === "cleanup") cleanup();
    await Promise.resolve();
  }
  const result = { ...counts };
  cleanup();
  ready(7);
  await Promise.resolve();
  return result;
}
for (const item of fixture.cases) test(item.name, async () => {
  expect(await execute(item.actions)).toEqual({ attached: item.attached, delivered: item.delivered, disposed: item.disposed });
});
test("fast-check independently models retired and replaced contributor publication", async () => {
  await fc.assert(fc.asyncProperty(fc.array(fc.constantFrom("emit", "retire", "replace", "cleanup"), { maxLength: 30 }), async actions => {
    let current = true, disposed = false, delivered = 0;
    for (const action of actions) {
      if (action === "emit" && current) delivered++;
      if (action === "retire" || action === "replace" || action === "cleanup") current = false;
      if (action === "cleanup") disposed = true;
    }
    expect(await execute(["ready", ...actions])).toEqual({ attached: 1, delivered, disposed: Number(disposed) });
  }), { numRuns: 100 });
});
