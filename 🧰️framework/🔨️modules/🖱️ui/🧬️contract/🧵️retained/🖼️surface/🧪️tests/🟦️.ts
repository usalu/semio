import { expect, spyOn, test } from "bun:test";
import { Database } from "bun:sqlite";
import { encodePackValue } from "../../../../../🎁️packs/🟦️.ts";
import { DEFAULT_UI_DOCUMENT_LIMITS } from "../../../🛡️limits/🟦️.ts";
import { OwnedUiOperation } from "../../🩹️operations/🟦️.ts";
import { RetainedUiNumericTable } from "../../🧮️tables/🟦️.ts";
import { RetainedUiTypedCursor } from "../../📦️wire/🧾️typed/🟦️.ts";
import fields from "../../📦️wire/🧪️tests/🧫️fixtures/🏷️fields/🔣️.json";
import { OwnedUiSurface } from "../🟦️.ts";
import fixture from "./🔣️.json";

const grant = { maxItems: 1, maxBytes: 4096 };
const empty = { maxItems: 0, maxBytes: 0 };

function settle(next: () => { kind: string; items: number; bytes: number }, terminal: readonly string[]): string {
  for (let turn = 0; turn < 10_000; turn++) {
    const step = next();
    expect(step.items).toBeLessThanOrEqual(grant.maxItems);
    expect(step.bytes).toBeLessThanOrEqual(grant.maxBytes);
    if (terminal.includes(step.kind)) return step.kind;
  }
  throw Error("Owned UI fixture did not settle");
}

function candidate(row: typeof fixture.cases[number]) {
  const surface = new OwnedUiSurface({ actor: "fixture", instance: 1, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
  const patch = surface.beginPatch(0, 1);
  for (const id of [fixture.root, ...row.leaves]) {
    const decoder = new RetainedUiTypedCursor(encodePackValue({
      ...fields.node, id, key: id === fixture.root ? "root" : fixture.privateKey,
      component: { type: "text", value: fixture.privateKey },
      children: id === fixture.root ? row.children : [],
    }), "node");
    expect(settle(() => decoder.advance(grant), ["ready", "rejected"])).toBe("ready");
    const payload = decoder.takeResult()!;
    patch.pushOperation(OwnedUiOperation.upsert(payload));
    const retirement = payload.beginClose();
    settle(() => retirement.advance(grant), ["complete"]);
    decoder.beginClose();
    settle(() => decoder.closeStep(grant), ["complete"]);
    expect(settle(() => patch.advance(grant), ["ready", "rejected"])).toBe("ready");
  }
  patch.pushOperation(OwnedUiOperation.setRoot(fixture.root));
  expect(settle(() => patch.advance(grant), ["ready", "rejected"])).toBe("ready");
  patch.finishInput();
  return { surface, patch };
}

function close(active: ReturnType<typeof candidate>) {
  const { patch, surface } = active;
  expect(surface.view).toMatchObject({ revision: 0, root: null, hash: null });
  expect(patch.takeAcknowledgement()).toBeNull();
  patch.beginClose();
  expect(patch.closeStep(empty).kind).toBe("blocked");
  let violationsClosed = false;
  settle(() => {
    const step = patch.closeStep(grant);
    violationsClosed ||= step.phase === "violations-close";
    return step;
  }, ["complete"]);
  expect(violationsClosed).toBe(true);
  expect(patch.terminalIsEmpty()).toBe(true);
  surface.beginClose();
  settle(() => surface.closeStep(grant), ["complete"]);
  expect(surface.terminalIsEmpty()).toBe(true);
}

test("retained publication reports the first graph violation with numeric identity", () => {
  using oracle = new Database(":memory:");
  const first = oracle.query(`with edges as (
    select cast(key as integer) as ordinal, value as child from json_each(?1)
  ), violations as (
    select ordinal, 'orphanChild' as type, printf('parent=%d child=%d', ?3, child) as identity
    from edges where child not in (select value from json_each(?2))
    union all
    select ordinal, 'duplicateSiblingKey', printf('parent=%d', ?3)
    from edges where child in (select value from json_each(?2)) and exists (
      select 1 from edges earlier where earlier.ordinal < edges.ordinal and earlier.child in (select value from json_each(?2))
    )
    union all
    select 10000 + value, 'danglingRoot', printf('node=%d', value) from json_each(?2)
    where value not in (select child from edges)
  ) select type, identity from violations order by ordinal limit 1`);
  for (const row of fixture.cases) {
    expect(first.get(JSON.stringify(row.children),JSON.stringify(row.leaves),fixture.root)).toEqual({type:row.type,identity:row.identity});
    const active = candidate(row);
    expect(settle(() => active.patch.advance(grant), ["ready", "rejected"])).toBe("rejected");
    expect(active.patch.failure).toBe("Owned UI graph invariants violated (" + row.type + ") " + row.identity);
    expect(active.patch.failure).not.toContain(fixture.privateKey);
    close(active);
  }
});

test("cancelling the first-violation lookup retires its table and reader", () => {
  const active = candidate(fixture.cases[0]!);
  let reading = false;
  const original = RetainedUiNumericTable.prototype.lookup;
  const read = spyOn(RetainedUiNumericTable.prototype, "lookup").mockImplementation(function* (this: RetainedUiNumericTable<unknown>, id: number) {
    const result = original.call(this, id);
    let step = result.next();
    if (id === 0 && !step.done) reading = true;
    while (!step.done) { yield step.value; step = result.next(); }
    return step.value;
  });
  try {
    for (let turn = 0; !reading; turn++) {
      expect(turn).toBeLessThan(10_000);
      expect(active.patch.advance(grant).kind).toBe("pending");
    }
    expect(active.patch.advance(empty).kind).toBe("blocked");
    expect(active.patch.failure).toBeNull();
    close(active);
  } finally {
    read.mockRestore();
  }
});
