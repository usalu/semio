/** 🧪️ Line layout fixtures checked against the independent lines-and-columns parser. */
import { expect, test } from "vitest";
import Ajv from "ajv";
import { LinesAndColumns } from "lines-and-columns";
import { drawingTextLines, drawingTextFallbackExtent } from "../../🟦️.ts";
import fixture from "../../🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";

test("text fixtures satisfy their neutral schema", () => expect(new Ajv().compile(schema)(fixture)).toBe(true));
for (const item of fixture.cases) test(`line layout ${JSON.stringify(item.content)}`, () => {
  const lines = [...drawingTextLines(item.content)];
  expect(lines).toEqual(item.lines);
  expect(drawingTextFallbackExtent(item.content, item.size)).toEqual(item.extent);
  const oracle = new LinesAndColumns(item.content);
  const count = oracle.locationForIndex(item.content.length)!.line + 1;
  const expected = Array.from({length: count}, (_, line) => {
    const start = oracle.indexForLocation({line, column: 0})!;
    const end = oracle.indexForLocation({line: line + 1, column: 0}) ?? item.content.length;
    return item.content.slice(start, end).replace(/(?:\r\n|[\r\n])$/, "");
  });
  expect(lines).toEqual(expected);
});
