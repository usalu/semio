/** 🧪️ Shared map transport and composition agree with independent schema and JSON Patch evaluators. */
import assert from "node:assert/strict";
import Ajv from "ajv";
import { applyPatch, type Operation } from "fast-json-patch";
import fixture from "../../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../../🧬️schema/🔣️.json" with { type: "json" };
import valueSchema from "../../../../../🌱️value/🧬️schema/🔣️.json" with { type: "json" };
import { applyMapDelta, composeMapDelta, parseMapDelta, MapDeltaApplyError, type MapEntryDelta } from "../../🧬️schema/🟦️.ts";
import type { DslValue } from "../../../../../🌱️value/🧬️schema/🟦️.ts";

function patch(entry: MapEntryDelta<DslValue>, base: Record<string, DslValue>): Record<string, DslValue> {
  const present = Object.hasOwn(base, entry.key);
  if (entry.precondition === "never" || entry.precondition === "present" && !present || entry.precondition === "absent" && present) throw new Error("independent presence precondition refused");
  const path = "../../../../../../../../../../.." + entry.key.replaceAll("~", "~0").replaceAll("../../../../../../../../../../..", "~1");
  const operation = entry.operation;
  if (operation.kind === "reject") throw new Error("explicit rejected transformation");
  if (operation.kind === "remove" && !present) return base;
  const op: Operation = operation.kind === "set" ? { op: "add", path, value: operation.value } : { op: "remove", path };
  return applyPatch(base, [op], true, false).newDocument;
}

/** 🧾️ Neutral examples and exhaustive single-key triples verify partial-function composition. */
export function testSharedMapDeltaOracle(): void {
  const ajv = new Ajv({ strict: false, allErrors: true }).addSchema(valueSchema);
  const validate = ajv.compile(schema);
  for (const vector of fixture.cases) {
    assert.equal(validate(vector.compact), true, JSON.stringify(validate.errors));
    const compact = parseMapDelta(vector.compact);
    assert.deepEqual(compact, vector.compact);
    let composed = parseMapDelta({ entries: [] });
    for (const source of vector.source) composed = composeMapDelta(composed, parseMapDelta({ entries: [source] }));
    assert.deepEqual(composed, compact, vector.name);
    const base = structuredClone(vector.before) as Record<string, DslValue>;
    if ("error" in vector) {
      assert.throws(() => vector.source.reduce((state, entry) => patch(entry as MapEntryDelta<DslValue>, state), base));
      assert.throws(() => applyMapDelta(compact, base), (error) => error instanceof MapDeltaApplyError && error.code === vector.error, vector.name);
      assert.deepEqual(base, vector.before);
    } else {
      const expected = vector.source.reduce((state, entry) => patch(entry as MapEntryDelta<DslValue>, state), base);
      assert.deepEqual(expected, vector.after, vector.name);
      assert.deepEqual(applyMapDelta(compact, base), expected, vector.name);
    }
  }
  for (const value of fixture.invalid) {
    assert.equal(validate(value), false);
    assert.throws(() => parseMapDelta(value));
  }
  const changes = (["any", "present", "absent"] as const).flatMap((precondition) => [
    parseMapDelta({ entries: [{ key: "key", precondition, operation: { kind: "set", value: null } }] }),
    parseMapDelta({ entries: [{ key: "key", precondition, operation: { kind: "remove" } }] }),
  ]);
  const outcome = (run: () => unknown): unknown => { try { return { value: run() }; } catch { return { rejected: true }; } };
  for (const a of changes) for (const b of changes) for (const c of changes) {
    assert.deepEqual(composeMapDelta(composeMapDelta(a, b), c), composeMapDelta(a, composeMapDelta(b, c)));
    for (const base of [{}, { key: "old" }, { key: null }]) {
      const expected = outcome(() => [...a.entries, ...b.entries, ...c.entries].reduce((state, entry) => patch(entry, state), base as Record<string, DslValue>));
      assert.deepEqual(outcome(() => applyMapDelta(composeMapDelta(composeMapDelta(a, b), c), base)), expected);
    }
  }
  assert.throws(() => parseMapDelta({ entries: [fixture.cases[0]!.source[0], fixture.cases[0]!.source[0]] }));
  console.log("[DEBUG] shared map delta matched Ajv, 12 JSON Patch vectors and 648 exhaustive sequential-composition bases with associative compact output");
}
