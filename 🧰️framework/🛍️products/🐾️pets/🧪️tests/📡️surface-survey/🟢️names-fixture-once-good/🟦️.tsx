/** 🟢️ The names fixture once good tests of the 📡️ surface survey suite. */
import { expect, it, vi } from "vitest";
import { FLOOR, fixtureId, surfaceId, survey, wallId } from "@semio-tech/pets-react";
import { card, place, viewport } from "../🟣️harness/🟦️.tsx";

/** 🔵 Registers the names fixture once good tests of the 📡️ surface survey suite. */
export function registerNamesFixtureOnceGoodTests(): void {
  it("names every fixture once and for good, never sees one inside the stage's own element and reads nothing but its key", () => {
    viewport(1000, 700);
    const row = place(document.createElement("li"), { left: 100, top: 300, width: 400, height: 30 });
    row.setAttribute("data-pet-prop", "heating/u-values");
    row.setAttribute("data-correct", "true");
    row.setAttribute("data-value", "0.24");
    const layer = place(document.createElement("div"), { left: 0, top: 0, width: 1000, height: 700 });
    const inside = place(document.createElement("div"), { left: 600, top: 300, width: 200, height: 30 });
    inside.setAttribute("data-pet-prop", "heating");
    layer.append(inside);
    document.body.append(row, layer);
    const fixture = { id: fixtureId(row), key: "heating/u-values", x: 100, y: 300, width: 400, height: 30 };
    expect(survey(document, { frame: layer }).fixtures).toEqual([fixture]);
    expect(survey(document).fixtures.map((each) => each.id)).toEqual([fixtureId(row), fixtureId(inside)]);
    place(row, { left: 120, top: 340, width: 400, height: 30 });
    expect(survey(document, { frame: layer, pointer: null }).fixtures).toEqual([{ ...fixture, x: 120, y: 340 }]);
    expect(survey(document, { frame: layer, pointer: { x: 520, y: 370 } }).fixtures).toEqual([]);
    expect(survey(document, { frame: layer, props: "[data-other]" }).fixtures).toEqual([]);
    expect(fixtureId(row)).not.toBe(surfaceId(row));
    const reads = vi.spyOn(Element.prototype, "getAttribute");
    const dataset = vi.spyOn(HTMLElement.prototype, "dataset", "get");
    survey(document, { frame: layer });
    expect(reads.mock.calls.map(([name]) => name)).toContain("data-pet-prop");
    expect(reads.mock.calls.map(([name]) => name).filter((name) => name === "data-correct" || name === "data-value")).toEqual([]);
    expect(dataset).not.toHaveBeenCalled();
  });
  it("never looks into the stage's own element, and asks every element it walks once whether it is anything at all", () => {
    viewport(1000, 700);
    const outside = card({ left: 100, top: 300, width: 200, height: 100 });
    const plain = place(document.createElement("div"), { left: 600, top: 100, width: 100, height: 100 });
    const layer = place(document.createElement("div"), { left: 0, top: 0, width: 1000, height: 700 });
    const drawn = place(document.createElement("article"), { left: 500, top: 300, width: 200, height: 100 });
    drawn.setAttribute("data-pet-surface", "");
    drawn.append(place(document.createElement("p"), { left: 510, top: 310, width: 100, height: 20 }), place(document.createElement("button"), { left: 510, top: 340, width: 100, height: 20 }));
    layer.append(drawn);
    document.body.append(plain, layer);
    const page = { kind: "surveyed", width: 1000, height: 700, surfaces: [{ id: surfaceId(outside), x0: 100, x1: 300, y: 300 }, { id: FLOOR, x0: 0, x1: 1000, y: 700 }], keepouts: [{ x: 96, y: 300, width: 208, height: 100 }], walls: [{ id: wallId(outside, -1), surface: surfaceId(outside), side: -1, x: 100, y0: 300, y1: 400 }, { id: wallId(outside, 1), surface: surfaceId(outside), side: 1, x: 300, y0: 300, y1: 400 }], fixtures: [] };
    expect(survey(document, { frame: layer, walls: "article" })).toEqual(page);
    expect(survey(document).surfaces.map((surface) => surface.id)).toEqual([surfaceId(outside), surfaceId(drawn), FLOOR]);
    const backstage = document.createElement("section");
    backstage.setAttribute("inert", "");
    document.body.append(backstage);
    const asked = vi.spyOn(Element.prototype, "matches");
    expect(survey(document, { frame: layer, walls: "article" })).toEqual(page);
    const askedOf = (element: Element): number => asked.mock.contexts.filter((context) => context === element).length;
    expect([layer, drawn, ...drawn.children].map(askedOf)).toEqual([0, 0, 0, 0]);
    expect(askedOf(plain)).toBe(1);
    expect(askedOf(outside)).toBeGreaterThan(1);
    expect(survey(layer, { frame: layer })).toEqual({ kind: "surveyed", width: 1000, height: 700, surfaces: [{ id: FLOOR, x0: 0, x1: 1000, y: 700 }], keepouts: [], walls: [], fixtures: [] });
  });
}
