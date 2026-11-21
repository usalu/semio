/** 🎯️ Explicit case execution must name a case that matches discovery. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { caseSelectionProblem } from "../../🔍️discovery/🎛️selection/🟦️.ts";
import fixture from "../🧫️fixtures/🎯️case-selection/🔣️.json";

const reference = new Ajv().compile(fixture.schema);

for (const row of fixture.cases) {
  test(row.name, () => {
    expect(reference(row.selection)).toBe(row.accepted);
    const problem = caseSelectionProblem(row.arguments, row.selection.matched);
    expect(problem === undefined).toBe(row.accepted);
    if (row.problem !== undefined) expect(problem).toContain(row.problem);
  });
}
