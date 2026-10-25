/** 🔸️ The writes nothing frame changes tests of the pet depiction suite. */
import { expect, it, vi } from "vitest";
import { type ActorFrame, type Frame, type Species } from "@semio-tech/pets";
import { PAINT_LINEAR, PAINT_SHIFT, depictionMarkup, paint, stageScenery } from "@semio-tech/pets-react";
import { OPEN, SPECIMEN, depicted, frameFor, groups, pieces, writes } from "../🟠️harness/🟦️.tsx";

/** 🔹 Registers the writes nothing frame changes tests of the pet depiction suite. */
export function registerWritesNothingFrameChangesTests(): void {
  it("writes nothing when a frame changes nothing, and only what a frame changes", () => {
    const depiction = depicted(SPECIMEN);
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN)))).toEqual([]);
    const bones = [...frameFor(SPECIMEN).bones];
    bones[4] = bones[4]! + 0.0003;
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, { x: 0.004, opacity: 0.9996, spirits: 0.0001, bones, eyes: [{ x: 0.0004, y: -0.0004, lid: 0.0004 }, OPEN] })))).toEqual([]);
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, {}, { arm: { rotation: 30 } })))).toEqual(["attributes:g.transform"]);
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, {}, { arm: { rotation: 30 } })))).toEqual([]);
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, { spirits:0.5 }, { arm: { rotation: 30 } })))).toEqual(["attributes:path.d"]);
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, { spirits:0.5, eyes: [{ x: 0.5, y: 0, lid: 0.3 }, OPEN] }, { arm: { rotation: 30 } })))).toEqual(["attributes:circle.cx", "attributes:g.transform"]);
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, { spirits:0.5, x: 3, eyes: [{ x: 0.5, y: 0, lid: 0.3 }, OPEN] }, { arm: { rotation: 30 } })))).toEqual(["attributes:svg.style"]);
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, { spirits:0.5, x: 3, activity: "walk", eyes: [{ x: 0.5, y: 0, lid: 0.3 }, OPEN] }, { arm: { rotation: 30 } })))).toEqual(["attributes:svg.data-pet-activity"]);
    expect(depiction.element.getAttribute("data-pet-activity")).toBe("walk");
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, { spirits:0.5, x: 3, activity: "walk", eyes: [{ x: 0.5, y: 0, lid: 0.3 }, OPEN] }, { arm: { rotation: 30 } })))).toEqual([]);
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, { spirits:0.5, x: 3, activity: "walk", footing: "air", eyes: [{ x: 0.5, y: 0, lid: 0.3 }, OPEN] }, { arm: { rotation: 30 } })))).toEqual(["attributes:svg.data-pet-footing"]);
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, { spirits:0.5, x: 3, activity: "walk", footing: "air", mood: "grumpy", intensity: 0.6, eyes: [{ x: 0.5, y: 0, lid: 0.3 }, OPEN] }, { arm: { rotation: 30 } })))).toEqual(["attributes:svg.data-pet-mood"]);
    expect([depiction.element.getAttribute("data-pet-footing"), depiction.element.getAttribute("data-pet-mood")]).toEqual(["air", "grumpy"]);
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, { spirits:0.5, x: 3, activity: "walk", footing: "air", mood: "grumpy", intensity: 0.2, eyes: [{ x: 0.5, y: 0, lid: 0.3 }, OPEN] }, { arm: { rotation: 30 } })))).toEqual([]);
    paint(depiction, frameFor(SPECIMEN, { spirits:0.5, x: 3, activity: "walk", eyes: [{ x: 0.5, y: 0, lid: 0.3 }, OPEN] }, { arm: { rotation: 30 } }));
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, { spirits:0.5, x: 3, activity: "walk", eyes: [{ x: 0.5, y: 0, lid: 0.3 }, OPEN] }, { body: { y: -2 }, arm: { rotation: 30 } }))).sort()).toEqual(Array.from({ length: 2 }, () => "attributes:g.transform"));
  });
  it("rewrites a bone, a pupil, a lid or the mouth only once the drawing moved by a step one could see since it was written, so it never strays further from the frame", () => {
    expect([PAINT_SHIFT, PAINT_LINEAR]).toEqual([0.05, 0.002]);
    const depiction = depicted(SPECIMEN);
    const rest = frameFor(SPECIMEN).bones;
    const nudged = (shift: number, turn: number, change: Partial<ActorFrame> = {}): ActorFrame => frameFor(SPECIMEN, { bones: rest.map((value, index) => (index === 4 ? value + shift : index === 0 ? value + turn : value)), ...change });
    expect(writes(depiction, () => paint(depiction, nudged(0.03, 0)))).toEqual([]);
    expect(writes(depiction, () => paint(depiction, nudged(0.049, 0)))).toEqual([]);
    expect(writes(depiction, () => paint(depiction, nudged(0.05, 0)))).toEqual(["attributes:g.transform"]);
    expect(groups(depiction)[0]!.getAttribute("transform")).toBe("matrix(1 0 0 1 0.05 0)");
    expect(writes(depiction, () => paint(depiction, nudged(0.09, 0)))).toEqual([]);
    expect(writes(depiction, () => paint(depiction, nudged(0.05, 0.0014)))).toEqual([]);
    expect(writes(depiction, () => paint(depiction, nudged(0.05, 0.0021)))).toEqual(["attributes:g.transform"]);
    expect(groups(depiction)[0]!.getAttribute("transform")).toBe("matrix(1.002 0 0 1 0.05 0)");
    const still = nudged(0.05, 0.0021);
    expect(writes(depiction, () => paint(depiction, { ...still, eyes: [{ x: 0.04, y: -0.04, lid: 0.001 }, OPEN], spirits: 0.01 }))).toEqual([]);
    expect(writes(depiction, () => paint(depiction, { ...still, eyes: [{ x: 0.06, y: -0.04, lid: 0.003 }, OPEN], spirits: 0.02 }))).toEqual(["attributes:circle.cx", "attributes:g.transform", "attributes:path.d"]);
    expect([pieces(depiction)[3]!.getAttribute("transform"), pieces(depiction)[3]!.lastElementChild!.getAttribute("cx"), pieces(depiction)[5]!.getAttribute("d")]).toEqual(["translate(-5 -3) scale(1 0.997)", "0.06", "M -3 3 Q 0 3.06 3 3"]);
  });
  it("never writes in a way a policy without inline styles blocks", () => {
    const setAttribute = vi.spyOn(Element.prototype, "setAttribute");
    const innerHTML = vi.spyOn(Element.prototype, "innerHTML", "set");
    const outerHTML = vi.spyOn(Element.prototype, "outerHTML", "set");
    const cssText = vi.spyOn(CSSStyleDeclaration.prototype, "cssText", "set");
    const depiction = depicted(SPECIMEN, frameFor(SPECIMEN, { x: 40, y: 90, facing: -1, opacity: 0.25, spirits: -0.5 }), 0.8);
    paint(depiction, frameFor(SPECIMEN, { x: 41, spirits: 1, eyes: [{ x: 1, y: 1, lid: 1 }, OPEN] }, { body: { rotation: 20 } }), 0.8);
    expect(setAttribute.mock.calls.length).toBeGreaterThan(20);
    expect(setAttribute.mock.calls.map(([name]) => name)).not.toContain("style");
    expect(innerHTML).not.toHaveBeenCalled();
    expect(outerHTML).not.toHaveBeenCalled();
    expect(cssText).not.toHaveBeenCalled();
    vi.restoreAllMocks();
  });
  it("states the same drawing as text", () => {
    const frames = [frameFor(SPECIMEN), frameFor(SPECIMEN, { spirits:-0.7, eyes: [{ x: 0.9, y: -0.3, lid: 0.25 }, { x: 0.9, y: -0.3, lid: 1 }] }, { body: { rotation: 15, scaleY: 1.05 }, arm: { rotation: -70 }, tail: { x: 3 } })];
    const cases = [SPECIMEN, { ...SPECIMEN, id: "bare", face: { eyes: [] }, parts: [...SPECIMEN.parts, { id: "stray", bone: "nowhere", shape: { kind: "path", d: 'M 0 0 "<&>" Z' }, fill: "ink", stroke: "none" }] } satisfies Species];
    for (const species of cases) {
      for (const frame of frames) {
        const depiction = depicted(species, frame, 2);
        expect(depiction.element.getAttribute("style")).toContain("--pet-body");
        const live = document.createElement("div");
        live.append(depiction.element);
        depiction.element.removeAttribute("style");
        const stated = document.createElement("div");
        stated.innerHTML = depictionMarkup(species, frame);
        expect(stated.innerHTML).toBe(live.innerHTML);
        expect(stated.firstElementChild!.isEqualNode(depiction.element)).toBe(true);
      }
    }
    expect(depictionMarkup(SPECIMEN, frames[0]!)).toMatch(/^<svg class="pet" data-pet="specimen" focusable="false" overflow="visible" data-pet-activity="idle" data-pet-footing="perch" data-pet-state="resting" data-pet-mood="content"><g transform="matrix\(1 0 0 1 0 0\)"><line x1="-6" y1="-7" x2="-6" y2="0" class="pet-fill-none pet-stroke-ink" stroke-width="3"><\/line><\/g>/u);
    expect(depictionMarkup(SPECIMEN, frameFor(SPECIMEN, { footing: "chute", activity: "glide", state: "nowhere", mood: "proud" }))).toMatch(/^<svg class="pet" data-pet="specimen" focusable="false" overflow="visible" data-pet-activity="glide" data-pet-footing="chute" data-pet-mood="proud">/u);
    expect(depictionMarkup(SPECIMEN, frames[0]!)).not.toMatch(/style|\bid=/u);
  });
  it("stages what a frame shows besides the actors: copies and ladders behind them, tools inside them, dust and particles in front of them — lazily, writing nothing while nothing lives, and nothing left behind", () => {
    const sparkling: Species = {
      ...SPECIMEN,
      states: [
        { id: "resting", name: { en: "Resting", de: "In Ruhe" } },
        { id: "shining", name: { en: "Shining", de: "Strahlend" }, tint: { accent: "#ffe45c" } },
      ],
      emitters: [{ id: "spark", bone: "body", x: 0, y: 0, shape: { kind: "ellipse", cx: 0, cy: 0, rx: 2, ry: 2 }, fill: "accent", stroke: "none", motion: "burst", count: 4, life: 1, speed: 30, spread: 0.5 }],
    };
    const host = document.createElement("div");
    const copy = document.createElement("li");
    host.append(copy);
    document.body.append(host);
    const scenery = stageScenery(host, new Map([[sparkling.id, sparkling]]), new Set([copy]));
    const depiction = depicted(sparkling);
    host.append(depiction.element);
    const records = (act: () => void): number => {
      const observer = new MutationObserver(() => {});
      observer.observe(host, { attributes: true, childList: true, subtree: true });
      act();
      const taken = observer.takeRecords().length;
      observer.disconnect();
      return taken;
    };
    const still = frameFor(sparkling);
    const shown = (ladders: Frame["ladders"], particles: Frame["particles"], puffs: Frame["puffs"] = []): Pick<Frame, "ladders" | "particles" | "puffs"> => ({ ladders, particles, puffs });
    scenery.stage(shown([], []), 1);
    scenery.actor(depiction, still);
    expect([...host.children]).toEqual([copy, depiction.element]);
    expect(scenery.behind()).toBe(copy);
    const gear = [...depiction.element.children].filter((child) => child.getAttribute("class") === "pet-gear");
    expect([depiction.element.firstElementChild, depiction.element.lastElementChild]).toEqual(gear);
    expect(depiction.element.querySelectorAll('.pet-gear [visibility="visible"]')).toHaveLength(0);
    expect(records(() => {
      scenery.stage(shown([], []), 1);
      scenery.actor(depiction, still);
    })).toBe(0);
    scenery.actor(depiction, frameFor(sparkling, { tools: [{ kind: "chute", open: 1, sway: 0 }] }));
    expect(depiction.element.querySelector('[data-pet-tool="chute"]')!.getAttribute("visibility")).toBe("visible");
    const ladder = { x0: 10, y0: 300, x1: 40, y1: 200, rungs: 9, opacity: 1 };
    scenery.stage(shown([ladder], []), 1);
    const rack = host.querySelector("svg.pet-ladders")!;
    expect([...host.children]).toEqual([copy, rack, depiction.element]);
    expect(scenery.behind()).toBe(rack);
    const spark = { species: sparkling.id, emitter: "spark", x: 5, y: -20, scale: 1, rotation: 0, opacity: 1 };
    scenery.stage(shown([ladder], [spark]), 1);
    const effects = host.querySelector("svg.pet-effects")!;
    expect([...host.children]).toEqual([copy, rack, depiction.element, effects]);
    expect(records(() => scenery.stage(shown([ladder], [spark]), 1))).toBe(0);
    const puff = { x: 60, y: 280, width: 40, height: 30, phase: 0.25 };
    scenery.stage(shown([ladder], [spark], [puff]), 1);
    const cloud = effects.querySelector("[data-pet-puffs] > [data-pet-puff]")!;
    expect([effects.firstElementChild!.hasAttribute("data-pet-puffs"), cloud.getAttribute("visibility"), cloud.getAttribute("transform")]).toEqual([true, "visible", "translate(60 278.13)"]);
    expect(records(() => scenery.stage(shown([ladder], [spark], [puff]), 1))).toBe(0);
    scenery.stage(shown([ladder], [spark]), 1);
    expect(cloud.getAttribute("visibility")).toBe("hidden");
    const dusty = document.createElement("div");
    const settled = stageScenery(dusty, new Map(), new Set());
    settled.stage(shown([], [], [puff]), 2);
    expect([...dusty.children].map((child) => child.getAttribute("class"))).toEqual(["pet-effects"]);
    expect((dusty.firstElementChild as SVGSVGElement).style.transform).toBe("scale(2)");
    settled.strike();
    expect(dusty.childElementCount).toBe(0);
    const troupe = effects.querySelector<SVGGElement>(`[data-pet-effects="${sparkling.id}"]`)!;
    expect(troupe.style.getPropertyValue("--pet-accent")).toBe("#34d1bf");
    scenery.actor(depiction, frameFor(sparkling, { state: "shining" }));
    expect(troupe.style.getPropertyValue("--pet-accent")).toBe("#ffe45c");
    expect(records(() => scenery.actor(depiction, frameFor(sparkling, { state: "shining" })))).toBe(0);
    scenery.retire(sparkling.id);
    expect(troupe.isConnected).toBe(false);
    scenery.strike();
    expect([...host.children]).toEqual([copy, depiction.element]);
    host.remove();
  });
}
