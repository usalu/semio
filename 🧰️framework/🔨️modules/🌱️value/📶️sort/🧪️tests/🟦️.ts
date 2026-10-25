import assert from "node:assert/strict";
import { createRequire } from "node:module";
import Ajv from "ajv";
import { HeapSortCursor } from "../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import snapshotSchema from "../🧬️schema/📸️snapshot.json" with { type: "json" };

const orderBy = createRequire(import.meta.url)("lodash/orderBy") as <T>(values: T[], keys: ((value: T) => number)[], orders: string[]) => T[];

export function verifyRetainedIndexSortOracle(): void {
  const validate = new Ajv({ strict: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  const validateSnapshot = new Ajv({ strict: true }).compile(snapshotSchema);
  let actions = 0;
  for (const row of fixture.cases) {
    const order = row.keys.map((_, index) => index);
    let cursor = new HeapSortCursor(order.length);
    while (true) {
      const work = cursor.next();
      if (work.kind === "complete") break;
      if (work.kind === "compare") {
        const left = row.keys[order[work.left]!]!, right = row.keys[order[work.right]!]!;
        const difference = left.findIndex((value, axis) => value !== right[axis]);
        cursor.advance(difference < 0 ? 0 : left[difference]! < right[difference]! ? -1 : 1);
      } else {
        if (work.kind === "swap") [order[work.left], order[work.right]] = [order[work.right]!, order[work.left]!];
        cursor.advance();
      }
      assert(validateSnapshot(cursor.snapshot()), JSON.stringify(validateSnapshot.errors));
      const restored = HeapSortCursor.fromSnapshot(cursor.snapshot());
      assert.deepEqual(restored.next(), cursor.next());
      cursor = restored;
      actions++;
    }
    assert.deepEqual(order, row.order, row.id);
    assert.deepEqual(order, orderBy(row.keys.map((key, index) => ({ key, index })), [value => value.key[0]!, value => value.key[1]!, value => value.key[2]!], ["asc", "asc", "asc"]).map(value => value.index), `${row.id} lodash`);
  }
}
