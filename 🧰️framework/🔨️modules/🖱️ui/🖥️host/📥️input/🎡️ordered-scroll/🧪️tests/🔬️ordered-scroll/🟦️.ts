/** 🎡️ Validates the language-neutral ordered Scroll ingress contract. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

export type OrderedScrollModifiers = { readonly shift: boolean; readonly ctrl: boolean; readonly alt: boolean; readonly meta: boolean };
export type OrderedScrollEvent =
  | { readonly kind: "scroll"; readonly x: number; readonly y: number; readonly deltaX: number; readonly deltaY: number; readonly modifiers: OrderedScrollModifiers }
  | { readonly kind: "pointer-move"; readonly x: number; readonly y: number; readonly modifiers: OrderedScrollModifiers }
  | { readonly kind: "key-down"; readonly key: string; readonly modifiers: OrderedScrollModifiers };
export type OrderedScrollFixture = {
  readonly version: 1;
  readonly capacity: number;
  readonly cases: readonly { readonly id: string; readonly physical: readonly OrderedScrollEvent[]; readonly expectedDom: readonly OrderedScrollEvent[]; readonly expectedQueue: readonly OrderedScrollEvent[] }[];
  readonly overflow: {
    readonly fill: Extract<OrderedScrollEvent, { readonly kind: "key-down" }>;
    readonly fillCount: number;
    readonly candidate: Extract<OrderedScrollEvent, { readonly kind: "scroll" }>;
    readonly expectedOutcome: "overflow";
    readonly expectedOverflowCount: number;
    readonly expectedPreservedCount: number;
  };
};

export function orderedScrollFixture(): OrderedScrollFixture {
  return JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8")) as OrderedScrollFixture;
}

function retainedQueueEvents(events: readonly OrderedScrollEvent[]): OrderedScrollEvent[] {
  let retainedPointer: { readonly sequence: number; readonly event: OrderedScrollEvent } | undefined;
  const retained: { readonly sequence: number; readonly event: OrderedScrollEvent }[] = events.flatMap((event, sequence) => {
    if (event.kind === "pointer-move") {
      retainedPointer = { sequence, event };
      return [];
    }
    return [{ sequence, event }];
  });
  if (retainedPointer) retained.push(retainedPointer);
  return retained.sort((left, right) => left.sequence - right.sequence).map(({ event }) => event);
}

export function testOrderedScrollFixture(): void {
  const fixture = orderedScrollFixture();
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert.ok(validate(fixture), JSON.stringify(validate.errors));
  assert.equal(new Set(fixture.cases.map(({ id }) => id)).size, fixture.cases.length);
  for (const row of fixture.cases) {
    assert.deepEqual(row.expectedDom, row.physical, `${row.id}: DOM keeps every physical event`);
    assert.deepEqual(retainedQueueEvents(row.physical), row.expectedQueue, `${row.id}: queue result keeps Scroll and repositions only the retained pointer sample`);
  }
  assert.equal(fixture.overflow.fillCount, fixture.capacity);
  assert.equal(fixture.overflow.expectedPreservedCount, fixture.capacity);
  const merged = structuredClone(fixture) as unknown as { cases: { physical: OrderedScrollEvent[]; expectedQueue: OrderedScrollEvent[] }[] };
  const second = merged.cases[0]!.expectedQueue[1]! as Extract<OrderedScrollEvent, { readonly kind: "scroll" }>;
  merged.cases[0]!.expectedQueue.splice(0, 2, {
    ...second,
    deltaX: 0,
    deltaY: 10,
  });
  assert.equal(validate(merged), true, "schema validates shape while the neutral oracle rejects semantic coalescing");
  assert.notDeepEqual(retainedQueueEvents(merged.cases[0]!.physical), merged.cases[0]!.expectedQueue);
  assert.equal(validate({ ...fixture, extra: true }), false);
  console.log(`[DEBUG] ordered Scroll oracle: ${fixture.cases.length} sequences preserve physical wheels through fixed capacity ${fixture.capacity}`);
}
