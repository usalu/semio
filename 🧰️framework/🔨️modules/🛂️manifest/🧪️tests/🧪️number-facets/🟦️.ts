/** 🎛️ `actionArgNumberFacets` (the TypeScript twin of Rust `ActionArgDef::number_facets`) over the language-agnostic corpus
 * `🧫️fixtures/🧫️number-facets/🔣️.json` that `🦀️.rs` beside this file holds the Rust mapping to. Oracles: the npm `jsonschema`
 * package validates the corpus against its fixture schema, and decimal.js recomputes every refusal's bound in display units
 * (`value × displayFactor` to twelve significant digits) beside the shown unit.
 * @see ../../🧫️fixtures/🧫️number-facets/🧬️schema/🔣️.json */
import { describe, expect, test } from "bun:test";
import Decimal from "decimal.js";

import corpus from "../../🧫️fixtures/🧫️number-facets/🔣️.json" with { type: "json" };

import { numberRangeIsValid, snapsAreValid } from "../../../🖱️ui/🧬️contract/🛡️limits/🟦️.ts";
import { actionArgNumberFacets, actionArgUnitSymbol, type ActionArgDef, type ActionArgNumberFacets, type ShellLocale } from "../../🟦️.ts";

type Case = { readonly name: string; readonly locale: ShellLocale; readonly def: ActionArgDef; readonly facets: ActionArgNumberFacets | null };
const cases = corpus.cases as unknown as readonly Case[];

describe("🎛️ action argument number facets", () => {
  test("📐️ the corpus validates against its fixture schema (npm jsonschema)", () => {
    
    
    expect(cases.length).toBeGreaterThanOrEqual(9);
  });

  for (const row of cases) {
    test(`🧾️ ${row.name}`, () => {
      expect(actionArgNumberFacets(row.def, row.locale)).toEqual(row.facets);
    });
  }

  test("🔢️ every refusal names its bound in display units (decimal.js oracle) beside the shown unit", () => {
    for (const row of cases) {
      const facets = row.facets;
      if (facets === null) continue;
      const unit = facets.displayUnit ?? facets.unit;
      for (const bound of [facets.limits.min, facets.limits.max]) {
        if (!bound) continue;
        const shown = new Decimal(bound.value).times(facets.displayFactor ?? 1).toSignificantDigits(12, Decimal.ROUND_HALF_UP).toString();
        expect(bound.refusal, row.name).toContain(unit === undefined ? shown : `${shown} ${unit}`);
      }
    }
  });

  test("⚖️ every facet row keeps the contract's detent and number-range laws", () => {
    for (const row of cases) {
      const facets = actionArgNumberFacets(row.def, "en");
      if (facets === null) continue;
      expect(snapsAreValid(facets.snaps, facets.min ?? -Infinity, facets.max ?? Infinity), row.name).toBe(true);
      expect(numberRangeIsValid(facets.min ?? null, facets.max ?? null, facets.scale, facets.displayFactor ?? null, facets.limits, facets.snaps), row.name).toBe(true);
    }
  });

  test("🔣️ unit symbols read as shown", () => {
    expect(["deg", "degrees", "percent", "mm"].map(actionArgUnitSymbol)).toEqual(["°", "°", "%", "mm"]);
  });
});
