/** 🖌️ The depiction draws a frame of the core by the drawing rules: one `<svg class="pet">` per actor, one group per
 * part carrying the matrix of its bone, the face above the part the species names, colours through classes and palette
 * custom properties. Its matrix strings are held to gl-matrix's `mat2d`, which composes the same rig and carries the
 * same points; a frame that changes nothing writes nothing, and nothing is ever written in a way a
 * Content-Security-Policy without inline styles would block.
 *
 * @see ../../🎯️targets/⚛️react/🔨️modules/🖌️depiction/🟦️.ts
 * @see ../../🎯️targets/⚛️react/🎨️.css
 * @see https://glmatrix.net/docs/module-mat2d.html — the third-party oracle
 */

import { glMatrix, mat2d, vec2 } from "gl-matrix";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it, vi } from "vitest";
import { PAINTS, type ActorFrame, type EyeFrame, type Species } from "@semio-tech/pets";
import { depict, depictionMarkup, paint, type Depiction } from "@semio-tech/pets-react";

glMatrix.setMatrixArrayType(Array);

const css = readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../🎯️targets/⚛️react/🎨️.css"), "utf8");
const ROUNDING = 0.0005 + 1e-9;

const SPECIMEN: Species = {
  id: "specimen",
  name: { en: "Specimen", de: "Exemplar" },
  thing: { en: "test rig", de: "Prüfgerüst" },
  grounds: [],
  size: { width: 40, height: 30 },
  palette: { body: "#1e9b8d", accent: "#34d1bf", detail: "#fa9500" },
  bones: [
    { id: "root", x: 0, y: 0 },
    { id: "body", parent: "root", x: 0, y: -16, rotation: 10 },
    { id: "arm", parent: "body", x: 13, y: 0, rotation: -20 },
    { id: "tail", parent: "root", x: -4, y: -2 },
  ],
  parts: [
    { id: "leg", bone: "root", shape: { kind: "line", x1: -6, y1: -7, x2: -6, y2: 0 }, fill: "none", stroke: "ink", strokeWidth: 3 },
    { id: "body", bone: "body", shape: { kind: "ellipse", cx: 0, cy: 0, rx: 14, ry: 12 }, fill: "body", stroke: "ink" },
    { id: "belly", bone: "body", shape: { kind: "rect", x: -5, y: 2, width: 10, height: 6, radius: 2 }, fill: "accent", stroke: "none" },
    { id: "arm", bone: "arm", shape: { kind: "path", d: "M 0 0 L 6 0" }, fill: "detail", stroke: "paper", strokeWidth: 1.5 },
  ],
  face: {
    eyes: [
      { id: "eye-left", bone: "body", x: -5, y: -3, radius: 3.5, pupil: 1.6 },
      { id: "eye-right", bone: "body", x: 5, y: -3, radius: 3.5, pupil: 1.6 },
    ],
    mouth: { bone: "body", x: 0, y: 3, width: 6 },
    above: "belly",
  },
  clips: [],
  repertoire: {},
  locomotion: { gait: "walk", speed: 36 },
  temperament: { energy: 0.5, sociability: 0.5, curiosity: 0.5 },
};

type Offset = { readonly x?: number; readonly y?: number; readonly rotation?: number; readonly scaleX?: number; readonly scaleY?: number };

/** 🔮️ gl-matrix: the world matrix of every bone, parents first — translate, rotate, scale, then the parent's matrix. */
function worlds(species: Species, pose: Readonly<Record<string, Offset>> = {}): mat2d[] {
  const solved: mat2d[] = [];
  for (const bone of species.bones) {
    const offset = pose[bone.id] ?? {};
    const local = mat2d.create();
    mat2d.translate(local, local, [bone.x + (offset.x ?? 0), bone.y + (offset.y ?? 0)]);
    mat2d.rotate(local, local, glMatrix.toRadian((bone.rotation ?? 0) + (offset.rotation ?? 0)));
    mat2d.scale(local, local, [offset.scaleX ?? 1, offset.scaleY ?? 1]);
    const parent = species.bones.findIndex((candidate) => candidate.id === bone.parent);
    solved.push(parent < 0 ? local : mat2d.multiply(mat2d.create(), solved[parent]!, local));
  }
  return solved;
}

const OPEN: EyeFrame = { x: 0, y: 0, lid: 0 };

function frameFor(species: Species, change: Partial<ActorFrame> = {}, pose: Readonly<Record<string, Offset>> = {}): ActorFrame {
  return { species: species.id, x: 0, y: 0, facing: 1, activity: "idle", opacity: 1, bones: worlds(species, pose).flatMap((world) => [...world]), eyes: species.face.eyes.map(() => OPEN), mood: 0, ...change };
}

function depicted(species: Species, frame: ActorFrame = frameFor(species), scale?: number): Depiction {
  const depiction = depict(species, document);
  paint(depiction, frame, scale);
  return depiction;
}

/** 🔎️ The six numbers of a `matrix(a b c d e f)` attribute, each written with at most three decimals. */
function matrixOf(element: Element): mat2d {
  const text = element.getAttribute("transform") ?? "";
  expect(text).toMatch(/^matrix\((?:-?\d+(?:\.\d{1,3})?(?: |\)$)){6}/u);
  return mat2d.fromValues(...(text.slice("matrix(".length, -1).split(" ").map(Number) as [number, number, number, number, number, number]));
}

const groups = (depiction: Depiction): Element[] => [...depiction.element.children];
const shapes = (depiction: Depiction): string[] => groups(depiction).map((group) => group.firstElementChild!.tagName);

/** 👀️ Everything written to the tree while `act` runs: attribute names per element tag, in order. */
function writes(depiction: Depiction, act: () => void): string[] {
  const observer = new MutationObserver(() => {});
  observer.observe(depiction.element, { attributes: true, childList: true, characterData: true, subtree: true });
  act();
  const records = observer.takeRecords();
  observer.disconnect();
  return records.map((record) => `${record.type}:${(record.target as Element).tagName}.${record.attributeName}`);
}

describe("pet depiction", () => {
  it("draws one svg per actor that names its species and takes no focus", () => {
    const { element } = depicted(SPECIMEN);
    expect(element.namespaceURI).toBe("http://www.w3.org/2000/svg");
    expect(element.tagName).toBe("svg");
    expect(element.getAttribute("class")).toBe("pet");
    expect(element.getAttribute("data-pet")).toBe("specimen");
    expect(element.getAttribute("focusable")).toBe("false");
    expect(element.getAttribute("overflow")).toBe("visible");
    expect(element.getAttribute("data-pet-activity")).toBe("idle");
    expect(depicted(SPECIMEN, frameFor(SPECIMEN, { activity: "squabble" })).element.getAttribute("data-pet-activity")).toBe("squabble");
    expect(element.querySelector("style, script, title, desc, a, [tabindex]")).toBeNull();
    expect(element.id).toBe("");
    expect(element.querySelector("[id]")).toBeNull();
    expect(element.querySelector("[style]")).toBeNull();
  });

  it("draws one group per part, back to front, with the face above the part the species names", () => {
    expect(shapes(depicted(SPECIMEN))).toEqual(["line", "ellipse", "rect", "g", "g", "path", "path"]);
    const { face } = SPECIMEN;
    expect(shapes(depicted({ ...SPECIMEN, face: { eyes: face.eyes, mouth: face.mouth } }))).toEqual(["line", "ellipse", "rect", "path", "g", "g", "path"]);
    expect(shapes(depicted({ ...SPECIMEN, face: { eyes: face.eyes, above: "leg" } }))).toEqual(["line", "g", "g", "ellipse", "rect", "path"]);
  });

  it("gives every shape its geometry, its paint classes and its stroke width", () => {
    const [leg, body, belly, , , , arm] = groups(depicted(SPECIMEN)).map((group) => group.firstElementChild!);
    const attributes = (element: Element): Record<string, string> => Object.fromEntries([...element.attributes].map((attribute) => [attribute.name, attribute.value]));
    expect(attributes(leg!)).toEqual({ x1: "-6", y1: "-7", x2: "-6", y2: "0", class: "pet-fill-none pet-stroke-ink", "stroke-width": "3" });
    expect(attributes(body!)).toEqual({ cx: "0", cy: "0", rx: "14", ry: "12", class: "pet-fill-body pet-stroke-ink", "stroke-width": "2" });
    expect(attributes(belly!)).toEqual({ x: "-5", y: "2", width: "10", height: "6", rx: "2", class: "pet-fill-accent pet-stroke-none" });
    expect(attributes(arm!)).toEqual({ d: "M 0 0 L 6 0", class: "pet-fill-detail pet-stroke-paper", "stroke-width": "1.5" });
  });

  it("has a paint rule for every class it writes, and a layer that is decoration only", () => {
    for (const paintName of PAINTS) {
      expect(css).toContain(`.pet-fill-${paintName} {`);
      expect(css).toContain(`.pet-stroke-${paintName} {`);
    }
    expect(css).toContain(".pet-pupil {");
    const rule = (selector: string): string => css.slice(css.indexOf(`\n${selector} {`), css.indexOf("}", css.indexOf(`\n${selector} {`)));
    for (const declaration of ["position: fixed;", "inset: 0;", "overflow: hidden;", "pointer-events: none;", "contain: strict;"]) expect(rule(".pet-layer")).toContain(declaration);
    for (const declaration of ["position: absolute;", "left: 0;", "top: 0;", "transform-origin: 0 0;", "will-change: transform;", "stroke-linejoin: round;", "stroke-linecap: round;"]) expect(rule(".pet")).toContain(declaration);
    expect(css).toMatch(/@media \(forced-colors: active\) \{\s*\.pet-layer \{\s*display: none;/u);
    expect(css).toMatch(/@media print \{\s*\.pet-layer \{\s*display: none;/u);
    const rules = css.replace(/\/\*[\s\S]*?\*\//gu, "");
    expect(rules.match(/transition[^;]*;/gu)).toEqual(["transition-property: none !important;"]);
    expect(rules.replace("transition-property: none !important;", "")).not.toMatch(/animation|@keyframes|transition|@apply|@import|@tailwind/u);
    expect(css).not.toMatch(/@keyframes|@apply|@import|@tailwind/u);
  });

  it("carries the palette as custom properties and places the actor by its CSSOM transform and opacity", () => {
    const { element } = depicted(SPECIMEN, frameFor(SPECIMEN, { x: 120.504, y: 80.25, facing: -1, opacity: 0.5 }), 1.5);
    expect(element.style.getPropertyValue("--pet-body")).toBe("#1e9b8d");
    expect(element.style.getPropertyValue("--pet-accent")).toBe("#34d1bf");
    expect(element.style.getPropertyValue("--pet-detail")).toBe("#fa9500");
    expect(element.style.transform).toBe("translate(120.5px, 80.25px) scale(-1.5, 1.5)");
    expect(element.style.opacity).toBe("0.5");
    const standing = depicted(SPECIMEN, frameFor(SPECIMEN, { x: 12, y: 300 }));
    expect(standing.element.style.transform).toBe("translate(12px, 300px) scale(1, 1)");
    expect(standing.element.style.opacity).toBe("1");
  });

  it("writes the matrix of every bone as gl-matrix composes it, rounded to three decimals", () => {
    const pose = { body: { y: -1.5, rotation: 7.3, scaleX: 0.98, scaleY: 1.05 }, arm: { rotation: -70 }, root: { x: 0.25 } };
    const expected = worlds(SPECIMEN, pose);
    const depiction = depicted(SPECIMEN, frameFor(SPECIMEN, {}, pose));
    const follows = ["root", "body", "body", "body", "body", "body", "arm"];
    groups(depiction).forEach((group, index) => {
      const world = expected[SPECIMEN.bones.findIndex((bone) => bone.id === follows[index])]!;
      const written = matrixOf(group);
      for (let entry = 0; entry < 6; entry++) expect(Math.abs(written[entry]! - world[entry]!)).toBeLessThanOrEqual(ROUNDING);
    });
    const tip = vec2.transformMat2d(vec2.create(), [6, 0], matrixOf(groups(depiction)[6]!));
    const reach = vec2.transformMat2d(vec2.create(), [6, 0], expected[2]!);
    expect(vec2.distance(tip, reach)).toBeLessThan(0.01);
    expect(groups(depicted(SPECIMEN, frameFor(SPECIMEN, { bones: [1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, -16, 0.5, 0.25, -0.25, 0.5, 13.0004, -15.9996, 1, 0, 0, 1, 0, 0] })))[6]!.getAttribute("transform")).toBe("matrix(0.5 0.25 -0.25 0.5 13 -16)");
    expect(depiction.followers.map((follower) => follower.length)).toEqual([1, 5, 1, 0]);
  });

  it("lets the pupils look and the lids squash the eyes on their bone", () => {
    const pose = { body: { rotation: -12 } };
    const depiction = depicted(SPECIMEN, frameFor(SPECIMEN, { eyes: [{ x: 1.15, y: -0.4004, lid: 0 }, { x: -0.25, y: 0.6, lid: 0.5 }] }, pose));
    const [left, right] = groups(depiction).slice(3, 5).map((group) => group.firstElementChild!);
    expect(left!.getAttribute("transform")).toBe("translate(-5 -3) scale(1 1)");
    expect(right!.getAttribute("transform")).toBe("translate(5 -3) scale(1 0.55)");
    const [white, pupil] = [...left!.children];
    expect([white!.tagName, white!.getAttribute("class"), white!.getAttribute("stroke-width"), white!.getAttribute("r")]).toEqual(["circle", "pet-fill-paper pet-stroke-ink", "1.5", "3.5"]);
    expect([pupil!.tagName, pupil!.getAttribute("class"), pupil!.getAttribute("r"), pupil!.getAttribute("cx"), pupil!.getAttribute("cy")]).toEqual(["circle", "pet-pupil", "1.6", "1.15", "-0.4"]);
    const squashed = mat2d.scale(mat2d.create(), mat2d.translate(mat2d.create(), matrixOf(groups(depiction)[4]!), [5, -3]), [1, 0.55]);
    const drawn = vec2.transformMat2d(vec2.create(), [Number(right!.children[1]!.getAttribute("cx")), Number(right!.children[1]!.getAttribute("cy"))], squashed);
    const meant = vec2.transformMat2d(vec2.create(), [5 - 0.25, -3 + 0.55 * 0.6], worlds(SPECIMEN, pose)[1]!);
    expect(vec2.distance(drawn, meant)).toBeLessThan(0.01);
    paint(depiction, frameFor(SPECIMEN, { eyes: [{ x: 0, y: 0, lid: 1 }, OPEN] }, pose));
    expect(left!.getAttribute("transform")).toBe("translate(-5 -3) scale(1 0.1)");
    expect(right!.getAttribute("transform")).toBe("translate(5 -3) scale(1 1)");
  });

  it("bends the mouth with the mood: up when sad, flat when neutral, down into a smile when happy", () => {
    const depiction = depicted(SPECIMEN, frameFor(SPECIMEN, { mood: 1 }));
    const mouth = groups(depiction)[5]!.firstElementChild!;
    expect([mouth.getAttribute("class"), mouth.getAttribute("stroke-width")]).toEqual(["pet-fill-none pet-stroke-ink", "1.5"]);
    expect(mouth.getAttribute("d")).toBe("M -3 3 Q 0 6 3 3");
    paint(depiction, frameFor(SPECIMEN, { mood: 0 }));
    expect(mouth.getAttribute("d")).toBe("M -3 3 Q 0 3 3 3");
    paint(depiction, frameFor(SPECIMEN, { mood: -1 }));
    expect(mouth.getAttribute("d")).toBe("M -3 3 Q 0 0 3 3");
    paint(depiction, frameFor(SPECIMEN, { mood: 0.4 }));
    expect(mouth.getAttribute("d")).toBe("M -3 3 Q 0 4.2 3 3");
  });

  it("writes nothing when a frame changes nothing, and only what a frame changes", () => {
    const depiction = depicted(SPECIMEN);
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN)))).toEqual([]);
    const bones = [...frameFor(SPECIMEN).bones];
    bones[4] = bones[4]! + 0.0003;
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, { x: 0.004, opacity: 0.9996, mood: 0.0001, bones, eyes: [{ x: 0.0004, y: -0.0004, lid: 0.0004 }, OPEN] })))).toEqual([]);
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, {}, { arm: { rotation: 30 } })))).toEqual(["attributes:g.transform"]);
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, {}, { arm: { rotation: 30 } })))).toEqual([]);
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, { mood: 0.5 }, { arm: { rotation: 30 } })))).toEqual(["attributes:path.d"]);
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, { mood: 0.5, eyes: [{ x: 0.5, y: 0, lid: 0.3 }, OPEN] }, { arm: { rotation: 30 } })))).toEqual(["attributes:circle.cx", "attributes:g.transform"]);
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, { mood: 0.5, x: 3, eyes: [{ x: 0.5, y: 0, lid: 0.3 }, OPEN] }, { arm: { rotation: 30 } })))).toEqual(["attributes:svg.style"]);
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, { mood: 0.5, x: 3, activity: "walk", eyes: [{ x: 0.5, y: 0, lid: 0.3 }, OPEN] }, { arm: { rotation: 30 } })))).toEqual(["attributes:svg.data-pet-activity"]);
    expect(depiction.element.getAttribute("data-pet-activity")).toBe("walk");
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, { mood: 0.5, x: 3, activity: "walk", eyes: [{ x: 0.5, y: 0, lid: 0.3 }, OPEN] }, { arm: { rotation: 30 } })))).toEqual([]);
    expect(writes(depiction, () => paint(depiction, frameFor(SPECIMEN, { mood: 0.5, x: 3, activity: "walk", eyes: [{ x: 0.5, y: 0, lid: 0.3 }, OPEN] }, { body: { y: -2 }, arm: { rotation: 30 } }))).sort()).toEqual(Array.from({ length: 6 }, () => "attributes:g.transform"));
  });

  it("never writes in a way a policy without inline styles blocks", () => {
    const setAttribute = vi.spyOn(Element.prototype, "setAttribute");
    const innerHTML = vi.spyOn(Element.prototype, "innerHTML", "set");
    const outerHTML = vi.spyOn(Element.prototype, "outerHTML", "set");
    const cssText = vi.spyOn(CSSStyleDeclaration.prototype, "cssText", "set");
    const depiction = depicted(SPECIMEN, frameFor(SPECIMEN, { x: 40, y: 90, facing: -1, opacity: 0.25, mood: -0.5 }), 0.8);
    paint(depiction, frameFor(SPECIMEN, { x: 41, mood: 1, eyes: [{ x: 1, y: 1, lid: 1 }, OPEN] }, { body: { rotation: 20 } }), 0.8);
    expect(setAttribute.mock.calls.length).toBeGreaterThan(20);
    expect(setAttribute.mock.calls.map(([name]) => name)).not.toContain("style");
    expect(innerHTML).not.toHaveBeenCalled();
    expect(outerHTML).not.toHaveBeenCalled();
    expect(cssText).not.toHaveBeenCalled();
    vi.restoreAllMocks();
  });

  it("states the same drawing as text", () => {
    const frames = [frameFor(SPECIMEN), frameFor(SPECIMEN, { mood: -0.7, eyes: [{ x: 0.9, y: -0.3, lid: 0.25 }, { x: 0.9, y: -0.3, lid: 1 }] }, { body: { rotation: 15, scaleY: 1.05 }, arm: { rotation: -70 }, tail: { x: 3 } })];
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
    expect(depictionMarkup(SPECIMEN, frames[0]!)).toMatch(/^<svg class="pet" data-pet="specimen" focusable="false" overflow="visible" data-pet-activity="idle"><g transform="matrix\(1 0 0 1 0 0\)"><line x1="-6" y1="-7" x2="-6" y2="0" class="pet-fill-none pet-stroke-ink" stroke-width="3"><\/line><\/g>/u);
    expect(depictionMarkup(SPECIMEN, frames[0]!)).not.toMatch(/style|\bid=/u);
  });
});
