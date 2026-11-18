import { describe, expect, test } from "vitest";
import Ajv from "ajv/dist/2020.js";
import cases from "./bands.cases.json";
import schema from "./bands.schema.json";
import { resolveCardBands } from "./bands.ts";

describe("card stack bands", () => {
  test("neutral bands schema", () => expect(new Ajv().validate(schema, cases)).toBe(true));
  for (const fixture of cases.cases) test(fixture.name, () => expect(resolveCardBands(fixture.cards)).toEqual(fixture.expected));
});
