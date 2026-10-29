import { test, expect } from "bun:test";
import Ajv from "ajv";
import { thesisFilledHeadingNumberPaintV1 } from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import cases from "./🔣️.json";
const valid = new Ajv({ strict: true }).compile(schema);
for (const row of cases) test("Filled heading paint: " + row.name, () => {
  expect(valid(row.input)).toBe(true);
  const invoke = () => thesisFilledHeadingNumberPaintV1(row.input.runs, row.input.heading, row.input.foreground);
  if (row.accepted) expect(invoke().map(run => run.text.replace(/\s+/g, "")).join("")).toBe(row.input.heading.number);
  else expect(invoke).toThrow();
});
