/** 🔶️ The ships vectors rule survey tests of the 📡️ surface survey suite. */
import { getRoles } from "@testing-library/react";
import { roles } from "aria-query";
import { expect, it } from "vitest";
import { FIXTURE_ELEMENTS, FLOOR, FOCUS_MARGIN, KEEPOUT_MARGIN, PET_CONTROLS, PET_KEEPOUTS, PET_PROPS, PET_SURFACES, SURFACE_WIDTH, fixtureId, stageBox, surfaceId, survey, wallId } from "@semio-tech/pets-react";
import vectors from "../../🧫️fixtures/📡️surface-survey/🔣️.json";
import { build, card, place, scenes, viewport } from "../🟣️shared/🟦️.tsx";

/** 🔷 Registers the ships vectors rule survey tests of the 📡️ surface survey suite. */
export function registerShipsVectorsRuleSurveyTests(): void {
  it("ships vectors for every rule of the survey", () => {
    expect(scenes.length).toBeGreaterThanOrEqual(10);
    expect(vectors.margins).toEqual({ surfaceWidth: SURFACE_WIDTH, keepout: KEEPOUT_MARGIN, focus: FOCUS_MARGIN, fixtureElements: FIXTURE_ELEMENTS });
    expect(PET_SURFACES).toBe("[data-pet-surface]");
    expect(PET_PROPS).toBe("[data-pet-prop]");
    expect(scenes.filter((scene) => (scene.expected.fixtures ?? []).length > 0).length).toBeGreaterThanOrEqual(3);
  });
  for (const scene of scenes) {
    it(`surveys: ${scene.name}`, () => {
      viewport(scene.viewport.width, scene.viewport.height);
      const built = new Map<string, HTMLElement>();
      build(scene.nodes, document.body, built);
      const frame = scene.frame === undefined ? undefined : place(document.createElement("div"), scene.frame);
      if (frame !== undefined) document.body.append(frame);
      if (scene.focus !== undefined) {
        built.get(scene.focus)!.focus();
        expect(document.activeElement).toBe(built.get(scene.focus));
      }
      const before = document.body.outerHTML;
      const surveyed = survey(document, {
        ...(scene.options?.surfaces === undefined ? {} : { surfaces: scene.options.surfaces }),
        ...(scene.options?.keepoutsBesidesDefaults === undefined ? {} : { keepouts: `${PET_KEEPOUTS}, ${scene.options.keepoutsBesidesDefaults}` }),
        ...(scene.options?.walls === undefined ? {} : { walls: scene.options.walls }),
        ...(scene.options?.props === undefined ? {} : { props: scene.options.props }),
        ...(scene.pointer === undefined ? {} : { pointer: scene.pointer }),
        frame,
      });
      expect(surveyed).toEqual({
        ...scene.expected,
        fixtures: (scene.expected.fixtures ?? []).map(({ element, ...fixture }) => ({ id: fixtureId(built.get(element)!), ...fixture })),
        surfaces: scene.expected.surfaces.map(({ element, ...edge }) => ({ id: element === "floor" ? FLOOR : surfaceId(built.get(element)!), ...edge })),
        walls: scene.expected.walls.map(({ element, side, ...wall }) => ({ id: wallId(built.get(element)!, side), surface: surfaceId(built.get(element)!), side, ...wall })),
      });
      expect(document.body.outerHTML).toBe(before);
    });
  }
  it("measures inside a root only, in the pixels of the stage", () => {
    viewport(1000, 700);
    const inside = document.createElement("section");
    document.body.append(inside);
    const mine = place(document.createElement("article"), { left: 100, top: 300, width: 200, height: 100 });
    mine.setAttribute("data-pet-surface", "");
    inside.append(mine);
    const theirs = card({ left: 500, top: 300, width: 200, height: 100 });
    expect(survey(inside).surfaces.map((surface) => surface.id)).toEqual([surfaceId(mine), FLOOR]);
    expect(survey(document).surfaces.map((surface) => surface.id)).toEqual([surfaceId(mine), surfaceId(theirs), FLOOR]);
    expect(stageBox(document)).toEqual({ x: 0, y: 0, width: 1000, height: 700 });
    expect(stageBox(document, place(document.createElement("div"), { left: 10, top: 20, width: 300, height: 200 }))).toEqual({ x: 10, y: 20, width: 300, height: 200 });
  });
  it("lets the page hide its own overflow without losing a surface: that clip is the viewport's", () => {
    viewport(1000, 700);
    for (const element of [document.documentElement, document.body]) {
      element.style.overflowX = "hidden";
      element.style.overflowY = "hidden";
    }
    const inside = card({ left: 100, top: 300, width: 200, height: 100 });
    expect(getComputedStyle(document.body).overflowY).toBe("hidden");
    expect(survey(document).surfaces).toEqual([
      { id: surfaceId(inside), x0: 100, x1: 300, y: 300 },
      { id: FLOOR, x0: 0, x1: 1000, y: 700 },
    ]);
    document.documentElement.removeAttribute("style");
    document.body.removeAttribute("style");
  });
  it("sees nothing from a root that is itself behind the scenes, and everything again once it is not", () => {
    viewport(1000, 700);
    const backstage = document.createElement("section");
    backstage.setAttribute("inert", "");
    const root = document.createElement("div");
    backstage.append(root);
    document.body.append(backstage);
    const inside = place(document.createElement("article"), { left: 100, top: 300, width: 200, height: 100 });
    inside.setAttribute("data-pet-surface", "");
    const words = place(document.createElement("p"), { left: 400, top: 300, width: 200, height: 40 });
    const folded = document.createElement("div");
    folded.setAttribute("hidden", "");
    folded.append(place(document.createElement("p"), { left: 400, top: 400, width: 200, height: 40 }));
    const after = place(document.createElement("p"), { left: 400, top: 500, width: 200, height: 40 });
    root.append(inside, words, folded, after);
    const floor = { id: FLOOR, x0: 0, x1: 1000, y: 700 };
    expect(survey(root)).toEqual({ kind: "surveyed", width: 1000, height: 700, surfaces: [floor], keepouts: [], walls: [], fixtures: [] });
    expect(survey(document)).toEqual({ kind: "surveyed", width: 1000, height: 700, surfaces: [floor], keepouts: [], walls: [], fixtures: [] });
    backstage.removeAttribute("inert");
    const walls = [
      { id: wallId(inside, -1), surface: surfaceId(inside), side: -1, x: 100, y0: 300, y1: 400 },
      { id: wallId(inside, 1), surface: surfaceId(inside), side: 1, x: 300, y0: 300, y1: 400 },
    ];
    const shown = { kind: "surveyed", width: 1000, height: 700, surfaces: [{ id: surfaceId(inside), x0: 100, x1: 300, y: 300 }, floor], keepouts: [{ x: 396, y: 296, width: 208, height: 48 }, { x: 396, y: 496, width: 208, height: 48 }, { x: 96, y: 300, width: 208, height: 100 }], walls, fixtures: [] };
    expect(survey(root)).toEqual(shown);
    expect(survey(document)).toEqual(shown);
    folded.remove();
    expect(survey(root)).toEqual(shown);
  });
  it("falls back to the window's own size where the page reports none", () => {
    expect(stageBox(document)).toEqual({ x: 0, y: 0, width: window.innerWidth, height: window.innerHeight });
    expect(window.innerWidth).toBeGreaterThan(0);
  });
  it("names every surface once and for good, however the page is rearranged", () => {
    viewport(1200, 800);
    const first = card({ left: 0, top: 100, width: 200, height: 100 });
    const second = card({ left: 300, top: 100, width: 200, height: 100 });
    const names = survey(document).surfaces.map((surface) => surface.id);
    expect(names).toEqual([surfaceId(first), surfaceId(second), FLOOR]);
    expect(new Set(names).size).toBe(3);
    document.body.prepend(second);
    place(second, { left: 600, top: 400, width: 250, height: 100 });
    expect(survey(document).surfaces).toEqual([
      { id: names[1], x0: 600, x1: 850, y: 400 },
      { id: names[0], x0: 0, x1: 200, y: 100 },
      { id: FLOOR, x0: 0, x1: 1200, y: 800 },
    ]);
    first.remove();
    const third = card({ left: 0, top: 500, width: 200, height: 100 });
    const later = survey(document).surfaces.map((surface) => surface.id);
    expect(later).toEqual([names[1], surfaceId(third), FLOOR]);
    expect(names).not.toContain(surfaceId(third));
    document.body.append(first);
    expect(survey(document).surfaces.map((surface) => surface.id)).toEqual([names[1], later[1], names[0], FLOOR]);
  });
  it("counts as controls exactly what the WAI-ARIA role model of aria-query calls a widget", () => {
    const widget = (role: string): boolean => {
      const definition = roles.get(role as never);
      return definition !== undefined && !definition.abstract && definition.superClass.some((chain) => (chain as readonly string[]).includes("widget"));
    };
    const named = [...PET_CONTROLS.matchAll(/\[role="([a-z]+)"\]/g)].map((match) => match[1]!);
    expect(named.length).toBeGreaterThanOrEqual(9);
    for (const role of named) expect(widget(role), role).toBe(true);
    const host = document.createElement("div");
    host.innerHTML =
      '<button>b</button><a href="#x">a</a><a>plain</a><input type="text"><input type="checkbox"><input type="radio"><input type="range"><select><option>o</option></select><textarea></textarea>' +
      "<div>d</div><p>p</p><h2>h</h2><ul><li>l</li></ul><section>s</section><span>t</span><img alt=\"i\">" +
      named.map((role) => `<span role="${role}">r</span>`).join("") +
      '<span role="note">n</span><span role="img">m</span><span role="status">u</span>';
    document.body.append(host);
    const judged = new Map<Element, boolean>();
    for (const [role, elements] of Object.entries(getRoles(host))) for (const element of elements) judged.set(element, (judged.get(element) ?? false) || widget(role));
    const sample = [...host.querySelectorAll("*")].filter((element) => element.tagName !== "OPTION");
    expect(sample.length).toBeGreaterThanOrEqual(25);
    for (const element of sample) expect(element.matches(PET_CONTROLS), element.outerHTML).toBe(judged.get(element) ?? false);
    const beyond = document.createElement("div");
    beyond.innerHTML = '<summary>s</summary><div tabindex="0">t</div><div tabindex="-1">u</div><div contenteditable="true">c</div><div contenteditable="false">f</div>';
    expect([...beyond.children].map((element) => element.matches(PET_CONTROLS))).toEqual([true, true, false, true, false]);
  });
}
