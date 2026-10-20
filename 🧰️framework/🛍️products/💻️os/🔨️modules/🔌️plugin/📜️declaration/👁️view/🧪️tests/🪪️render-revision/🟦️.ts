import { expect, test } from "vitest";
import Ajv from "ajv";
import { compare } from "fast-json-patch";
import { renderRevisionMatches } from "../../🧬️schema/🪪️render-revision/🟦️.ts";
import renderSchema from "../../🧬️schema/🪪️render-revision/🔣️.json";
import renderFixture from "../../🧫️fixtures/🪪️render-revision/🔣️.json";

test("mounted render identity refuses reused counters from another canonical document", () => {
  const validate = new Ajv({ strict: true }).compile(renderSchema);
  for (const row of renderFixture.cases) {
    const left = { ...renderFixture.base, ...row.left };
    const right = { ...renderFixture.base, ...row.right };
    expect(validate(left)).toBe(true);
    expect(validate(right)).toBe(true);
    expect(renderRevisionMatches(left, right), row.id).toBe(row.expected);
    expect(renderRevisionMatches(right, left), row.id).toBe(row.expected);
    expect(compare(left, right).length === 0, row.id).toBe(row.expected);
  }
  for (const counter of ["00", "-1", "18446744073709551616", "99999999999999999999"]) {
    expect(validate({ ...renderFixture.base, baseRevision: counter })).toBe(false);
    expect(validate({ ...renderFixture.base, generation: counter })).toBe(false);
  }
});
