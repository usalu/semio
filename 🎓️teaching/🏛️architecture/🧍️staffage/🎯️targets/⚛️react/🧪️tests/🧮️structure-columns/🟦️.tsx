/** 🧮️ The structure tables of the asset page show every component of force and moment with unit and label in both languages.
 * @see ./🥒️.feature — the scenarios */
import { cleanup, render } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import type { Language } from "@teaching/staffage";
import { ArchiveProvider } from "../../🔨️modules/🧭️context/🟦️.tsx";
import { translator } from "../../🔨️modules/🌐️i18n/🟦️.ts";
import { ModelSection } from "../../🔨️modules/🔬️detail/🧬️model/🟦️.tsx";
import { CatalogStore } from "../../🔨️modules/📚️catalog/🟦️.ts";
import { ArchiveStore, memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";
import { evaluateState, structureOf, thermalOf } from "../../🔨️modules/🧮️pipeline/🟦️.ts";
import { walker } from "../🧰️support/🧰️.ts";

const catalog = new CatalogStore({ manifest: async () => { throw new Error("unused"); }, category: async () => null });
afterEach(cleanup);

function open(language: Language): HTMLElement {
  const state = evaluateState(walker, { kind: "pose", pose: walker.presentation.defaultPose });
  const { container } = render(
    <ArchiveProvider language={language} t={translator(language)} catalog={catalog} store={new ArchiveStore(memoryStorage(), ["dark-line"])}>
      <ModelSection asset={walker} state={state} thermal={thermalOf(state)} structure={structureOf(state)} environment={walker.thermal.environment} language={language} />
    </ArchiveProvider>,
  );
  return container;
}

describe("🧮️ structure columns", () => {
  it("lists Fx, Fy, Fz, Mx, My and Mz for loads and reactions and torsion for joints in English", () => {
    const heads = [...open("en").querySelectorAll("th")].map((cell) => cell.textContent);
    for (const label of ["Force x", "Force y", "Force z (lateral)", "Moment about x", "Moment about y", "Moment about z", "Torsion"]) expect(heads).toContain(label);
  });

  it("labels them in German and gives every cell its unit", () => {
    const container = open("de");
    const heads = [...container.querySelectorAll("th")].map((cell) => cell.textContent);
    for (const label of ["Kraft z (seitlich)", "Moment um x", "Moment um y", "Moment um z", "Torsion"]) expect(heads).toContain(label);
    const cells = [...container.querySelectorAll("[data-totals=structure]")].map((node) => node.textContent ?? "");
    expect(cells.join(" ")).toMatch(/N·m/u);
  });
});
