/** 🪟️ The browser actor's visible-surface contract (`🏪️store/👷️worker/🪟️visible-surfaces/🔣️.json`) on its own: Ajv (the
 * independent validator) admits the contract against its schema, every case announces exactly its listed surfaces, every
 * turn carries every live section, and the contract's sections are exactly the reserved refresh sections the manifest
 * declares — the language-neutral `🛂️manifest/🧫️fixtures/🔬️ui-refresh-section/🔣️.json` and the TS `UI_REFRESH_SECTIONS` —
 * so no section an app renders can stay frozen on a hub document. */
import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import { UI_REFRESH_SECTIONS } from "../../../../🔨️modules/🛂️manifest/🟦️.ts";
import neutral from "../../../../🔨️modules/🛂️manifest/🧫️fixtures/🔬️ui-refresh-section/🔣️.json" with { type: "json" };
import { BROWSER_ACTOR_VISIBLE_SURFACES_V1, browserActorVisibleSurfacesV1, type BrowserActorVisibleTurnV1 } from "../../🔨️modules/🏪️store/👷️worker/🪟️visible-surfaces/🟦️.ts";
import schema from "../../🔨️modules/🏪️store/👷️worker/🪟️visible-surfaces/🧬️schema/🔣️.json" with { type: "json" };

import examples from "../../🔨️modules/🏪️store/👷️worker/🪟️visible-surfaces/🧫️fixtures/🔣️.json" with { type: "json" };

const contract = BROWSER_ACTOR_VISIBLE_SURFACES_V1;
const turns = Object.keys(contract.turns) as BrowserActorVisibleTurnV1[];

describe("🪟️ browser actor visible surfaces", () => {
  it("is a schema-valid contract (Ajv)", () => {
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(contract), JSON.stringify(validate.errors)).toBe(true);
  });

  it.each(examples.cases.map((row) => [row.name, row] as const))("%s", (_name, row) => {
    expect(browserActorVisibleSurfacesV1(row.surfaces, row.turn as BrowserActorVisibleTurnV1)).toEqual(row.visible);
  });

  it("announces exactly the reserved refresh sections the manifest declares, and only the catalogue is static", () => {
    const own = contract.sections.map(({ key, bodyKey }) => ({ key, bodyKey }));
    expect(own).toEqual(UI_REFRESH_SECTIONS.map(({ key, bodyKey }) => ({ key, bodyKey })));
    expect(own).toEqual(neutral.sections.map(({ key, bodyKey }) => ({ key, bodyKey })));
    expect(contract.sections.filter((section) => section.static).map(({ key }) => key)).toEqual(["catalogue"]);
  });

  it("carries every live section on every turn, and the catalogue only at mount", () => {
    for (const turn of turns) {
      const announced = browserActorVisibleSurfacesV1({ windows: [], panels: [] }, turn).map(({ bodyKey }) => bodyKey);
      for (const section of contract.sections) expect(announced.includes(section.bodyKey), `${turn} ${section.key}`).toBe(!section.static || turn === "mount");
    }
  });
});
