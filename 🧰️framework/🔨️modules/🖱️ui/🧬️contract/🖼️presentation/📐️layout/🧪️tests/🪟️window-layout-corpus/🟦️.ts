/** 🪟️ Runs the shared window layout corpus (the Rust twin runs the same file) and validates it against the schema. */
import Ajv from "ajv";
import { describe, expect, test } from "vitest";
import corpus from "../🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import { applyWindowLayoutOp, collectWindowIds, evenWindowLayout, readLayoutAt, stackPathOf, type WindowLayoutNode, type WindowLayoutOp } from "../../🟦️.ts";

const plain = (value: unknown): unknown => JSON.parse(JSON.stringify(value ?? null));
const layoutOf = (value: unknown): WindowLayoutNode => value as WindowLayoutNode;

describe("🪟️ window layout corpus", () => {
  const ajv = new Ajv({ strict: false });
  ajv.addSchema(schema);
  const validate = (def: string) => ajv.getSchema(`${schema.$id}#/$defs/${def}`)!;

  test("every layout and operation satisfies the schema", () => {
    const layout = validate("WindowLayout");
    const operation = validate("Operation");
    for (const entry of [...corpus.operations, ...corpus.queries]) expect(layout({ root: entry.layout }), entry.id).toBe(true);
    for (const entry of corpus.operations) {
      expect(layout({ root: entry.expect }), entry.id).toBe(true);
      expect(operation(entry.op), entry.id).toBe(true);
    }
  });

  test.each(corpus.operations)("$id", (entry) => {
    expect(plain(applyWindowLayoutOp(layoutOf(entry.layout), entry.op as WindowLayoutOp))).toEqual(entry.expect);
  });

  test.each(corpus.queries)("$id", (entry) => {
    const layout = layoutOf(entry.layout);
    const query = entry as { query: string; windowId?: string; path?: number[] };
    const actual = query.query === "stackPathOf" ? stackPathOf(layout, query.windowId!) : query.query === "collectWindowIds" ? collectWindowIds(layout) : readLayoutAt(layout, query.path!);
    expect(plain(actual)).toEqual(entry.expect);
  });

  test.each(corpus.builders)("$id", (entry) => {
    expect(plain(evenWindowLayout(entry.windowIds))).toEqual(entry.expect);
  });
});
