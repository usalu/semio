/** 🖌️ The depiction draws a frame of the core by the drawing rules: one `<svg class="pet">` per actor, one group per
 * run of parts on the same bone carrying the matrix of that bone, the face above the part the species names, colours
 * through classes and palette custom properties. Its matrix strings are held to gl-matrix's `mat2d`, which composes the
 * same rig and carries the same points; a frame that changes nothing writes nothing, a move nobody could see writes
 * nothing either, and nothing is ever written in a way a Content-Security-Policy without inline styles would block.
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
import { PAINTS, type ActorFrame, type EyeFrame, type Frame, type Species } from "@semio-tech/pets";
import { PAINT_LINEAR, PAINT_SHIFT, depict, depictionMarkup, paint, stageScenery, type Depiction } from "@semio-tech/pets-react";

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
  states: [{ id: "resting", name: { en: "Resting", de: "In Ruhe" } }],
  tricks: [],
  purr: { clip: "purr" },
  emitters: [],
  gear: [],
  grip: 27,
  reach: 10,
  mood: "content",
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
  return { species: species.id, x: 0, y: 0, facing: 1, activity: "idle", opacity: 1, bones: worlds(species, pose).flatMap((world) => [...world]), eyes: species.face.eyes.map(() => OPEN), footing: "perch", state: "resting", mood: "content", intensity: 0, spirits: 0, tilt: 0, pivot: { x: 0, y: -species.grip }, tools: [], body: { x: -species.size.width / 2, y: -species.size.height, width: species.size.width, height: species.size.height }, ...change };
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

/** 🪩️ gl-matrix: what a CSS `transform` of translations, scalings and turns means, its functions multiplied in their order. */
function cssMatrix(text: string): mat2d {
  const matrix = mat2d.create();
  for (const [, name, body] of text.matchAll(/(\w+)\(([^)]*)\)/gu)) {
    const numbers = body!.split(/[\s,]+/u).map((entry) => Number.parseFloat(entry));
    if (name === "translate") mat2d.translate(matrix, matrix, [numbers[0]!, numbers[1]!]);
    else if (name === "scale") mat2d.scale(matrix, matrix, [numbers[0]!, numbers[1]!]);
    else if (name === "rotate") mat2d.rotate(matrix, matrix, glMatrix.toRadian(numbers[0]!));
    else throw new Error(`unexpected ${name}`);
  }
  return matrix;
}

const groups = (depiction: Depiction): Element[] => [...depiction.element.children];
const pieces = (depiction: Depiction): Element[] => groups(depiction).flatMap((group) => [...group.children]);
const shapes = (depiction: Depiction): string[][] => groups(depiction).map((group) => [...group.children].map((piece) => piece.tagName));

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
    expect([element.getAttribute("data-pet-footing"), element.getAttribute("data-pet-state"), element.getAttribute("data-pet-mood")]).toEqual(["perch", "resting", "content"]);
    expect(depicted(SPECIMEN, frameFor(SPECIMEN, { activity: "squabble" })).element.getAttribute("data-pet-activity")).toBe("squabble");
    const held = depicted(SPECIMEN, frameFor(SPECIMEN, { activity: "hang", footing: "hand", mood: "scared", state: "nowhere" })).element;
    expect([held.getAttribute("data-pet-activity"), held.getAttribute("data-pet-footing"), held.hasAttribute("data-pet-state"), held.getAttribute("data-pet-mood")]).toEqual(["hang", "hand", false, "scared"]);
    expect(element.querySelector("style, script, title, desc, a, [tabindex]")).toBeNull();
    expect(element.id).toBe("");
    expect(element.querySelector("[id]")).toBeNull();
    expect(element.querySelector("[style]")).toBeNull();
  });

  it("draws one group per run of parts on the same bone, back to front, with the face above the part the species names", () => {
    expect(shapes(depicted(SPECIMEN))).toEqual([["line"], ["ellipse", "rect", "g", "g", "path"], ["path"]]);
    const { face } = SPECIMEN;
    expect(shapes(depicted({ ...SPECIMEN, face: { eyes: face.eyes, mouth: face.mouth } }))).toEqual([["line"], ["ellipse", "rect"], ["path"], ["g", "g", "path"]]);
    expect(shapes(depicted({ ...SPECIMEN, face: { eyes: face.eyes, above: "leg" } }))).toEqual([["line"], ["g", "g", "ellipse", "rect"], ["path"]]);
  });

  it("gives every shape its geometry, its paint classes and its stroke width", () => {
    const [leg, body, belly, , , , arm] = pieces(depicted(SPECIMEN));
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

  it("tilts the whole drawing about its pivot, leaning the same way on screen whichever way it faces, as gl-matrix composes it", () => {
    for (const [facing, tilt, scale, pivot] of [
      [1, 0.05, 1, { x: 0, y: -27 }],
      [-1, 0.05, 1.5, { x: 0, y: -27 }],
      [-1, -0.12, 0.8, { x: 6, y: -15 }],
      [1, 0.3, 1, { x: 0, y: 0 }],
    ] as const) {
      const frame = frameFor(SPECIMEN, { x: 120.5, y: 80.25, facing, tilt, pivot });
      const { element } = depicted(SPECIMEN, frame, scale);
      const written = cssMatrix(element.style.transform);
      const meant = mat2d.fromTranslation(mat2d.create(), [120.5, 80.25]);
      const lever: [number, number] = [facing * scale * pivot.x, scale * pivot.y];
      mat2d.translate(meant, meant, lever);
      mat2d.rotate(meant, meant, tilt * 2 * Math.PI);
      mat2d.translate(meant, meant, [-lever[0], -lever[1]]);
      mat2d.scale(meant, meant, [facing * scale, scale]);
      for (const point of [[0, 0], [10, -20], [-14, -30], [pivot.x, pivot.y]] as const) {
        const drawn = vec2.transformMat2d(vec2.create(), point, written);
        expect(vec2.distance(drawn, vec2.transformMat2d(vec2.create(), point, meant)), `${facing} ${tilt} ${scale} ${point}`).toBeLessThan(0.02);
      }
      const head = vec2.transformMat2d(vec2.create(), [0, -SPECIMEN.size.height], written);
      if (pivot.y === 0) expect(Math.sign(head[0]! - 120.5), "a positive tilt about the feet leans the head to the right").toBe(Math.sign(tilt));
    }
    const upright = depicted(SPECIMEN, frameFor(SPECIMEN, { x: 12, y: 300, tilt: 0.00001, pivot: { x: 3, y: -9 } }));
    expect(upright.element.style.transform).toBe("translate(12px, 300px) scale(1, 1)");
    expect(writes(upright, () => paint(upright, frameFor(SPECIMEN, { x: 12, y: 300, tilt: 0, pivot: { x: 8, y: -40 } })))).toEqual([]);
    expect(writes(upright, () => paint(upright, frameFor(SPECIMEN, { x: 12, y: 300, tilt: 0.01, pivot: { x: 8, y: -40 } })))).toEqual(["attributes:svg.style"]);
    expect(upright.element.style.transform).toBe("translate(12px, 300px) scale(1, 1) translate(8px, -40px) rotate(3.6deg) translate(-8px, 40px)");
    expect(writes(upright, () => paint(upright, frameFor(SPECIMEN, { x: 12, y: 300, tilt: 0.01, pivot: { x: 8.001, y: -40 } })))).toEqual([]);
    expect(writes(upright, () => paint(upright, frameFor(SPECIMEN, { x: 12, y: 300, tilt: 0.01, pivot: { x: 9, y: -40 } })))).toEqual(["attributes:svg.style"]);
  });

  it("paints the tint of its species' state over the palette and names the state it shows, and writes both only when the state changes", () => {
    const shining: Species = {
      ...SPECIMEN,
      states: [
        { id: "resting", name: { en: "Resting", de: "In Ruhe" } },
        { id: "shining", name: { en: "Shining", de: "Strahlend" }, tint: { body: "#ffe45c" } },
        { id: "glowing", name: { en: "Glowing", de: "Glühend" }, tint: { body: "#ff8a00", accent: "#fff3b0", detail: "#7a2e00" } },
      ],
    };
    const palette = (element: SVGSVGElement): string[] => ["--pet-body", "--pet-accent", "--pet-detail"].map((name) => element.style.getPropertyValue(name));
    const depiction = depicted(shining, frameFor(shining));
    expect(palette(depiction.element)).toEqual(["#1e9b8d", "#34d1bf", "#fa9500"]);
    expect(depiction.element.getAttribute("data-pet-state")).toBe("resting");
    expect(writes(depiction, () => paint(depiction, frameFor(shining, { state: "shining" })))).toEqual(["attributes:svg.style", "attributes:svg.data-pet-state"]);
    expect([palette(depiction.element), depiction.element.getAttribute("data-pet-state")]).toEqual([["#ffe45c", "#34d1bf", "#fa9500"], "shining"]);
    expect(writes(depiction, () => paint(depiction, frameFor(shining, { state: "shining", x: 0.001 })))).toEqual([]);
    expect(writes(depiction, () => paint(depiction, frameFor(shining, { state: "glowing" })))).toEqual(["attributes:svg.style", "attributes:svg.style", "attributes:svg.style", "attributes:svg.data-pet-state"]);
    expect([palette(depiction.element), depiction.element.getAttribute("data-pet-state")]).toEqual([["#ff8a00", "#fff3b0", "#7a2e00"], "glowing"]);
    paint(depiction, frameFor(shining, { state: "nowhere" }));
    expect([palette(depiction.element), depiction.element.hasAttribute("data-pet-state")]).toEqual([["#1e9b8d", "#34d1bf", "#fa9500"], false]);
    expect(writes(depiction, () => paint(depiction, frameFor(shining, { state: "resting" })))).toEqual(["attributes:svg.data-pet-state"]);
    expect(writes(depiction, () => paint(depiction, frameFor(shining, { state: "resting" })))).toEqual([]);
  });

  it("writes the matrix of every bone as gl-matrix composes it, rounded to three decimals", () => {
    const pose = { body: { y: -1.5, rotation: 7.3, scaleX: 0.98, scaleY: 1.05 }, arm: { rotation: -70 }, root: { x: 0.25 } };
    const expected = worlds(SPECIMEN, pose);
    const depiction = depicted(SPECIMEN, frameFor(SPECIMEN, {}, pose));
    const follows = ["root", "body", "arm"];
    expect(groups(depiction)).toHaveLength(follows.length);
    groups(depiction).forEach((group, index) => {
      const world = expected[SPECIMEN.bones.findIndex((bone) => bone.id === follows[index])]!;
      const written = matrixOf(group);
      for (let entry = 0; entry < 6; entry++) expect(Math.abs(written[entry]! - world[entry]!)).toBeLessThanOrEqual(ROUNDING);
    });
    const tip = vec2.transformMat2d(vec2.create(), [6, 0], matrixOf(groups(depiction)[2]!));
    const reach = vec2.transformMat2d(vec2.create(), [6, 0], expected[2]!);
    expect(vec2.distance(tip, reach)).toBeLessThan(0.01);
    expect(groups(depicted(SPECIMEN, frameFor(SPECIMEN, { bones: [1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, -16, 0.5, 0.25, -0.25, 0.5, 13.0004, -15.9996, 1, 0, 0, 1, 0, 0] })))[2]!.getAttribute("transform")).toBe("matrix(0.5 0.25 -0.25 0.5 13 -16)");
    expect(depiction.followers.map((follower) => follower.length)).toEqual([1, 1, 1, 0]);
  });

  it("lets the pupils look and the lids squash the eyes on their bone", () => {
    const pose = { body: { rotation: -12 } };
    const depiction = depicted(SPECIMEN, frameFor(SPECIMEN, { eyes: [{ x: 1.15, y: -0.4004, lid: 0 }, { x: -0.25, y: 0.6, lid: 0.5 }] }, pose));
    const [left, right] = pieces(depiction).slice(3, 5);
    expect(left!.getAttribute("transform")).toBe("translate(-5 -3) scale(1 1)");
    expect(right!.getAttribute("transform")).toBe("translate(5 -3) scale(1 0.55)");
    const [white, pupil] = [...left!.children];
    expect([white!.tagName, white!.getAttribute("class"), white!.getAttribute("stroke-width"), white!.getAttribute("r")]).toEqual(["circle", "pet-fill-paper pet-stroke-ink", "1.5", "3.5"]);
    expect([pupil!.tagName, pupil!.getAttribute("class"), pupil!.getAttribute("r"), pupil!.getAttribute("cx"), pupil!.getAttribute("cy")]).toEqual(["circle", "pet-pupil", "1.6", "1.15", "-0.4"]);
    const squashed = mat2d.scale(mat2d.create(), mat2d.translate(mat2d.create(), matrixOf(groups(depiction)[1]!), [5, -3]), [1, 0.55]);
    const drawn = vec2.transformMat2d(vec2.create(), [Number(right!.children[1]!.getAttribute("cx")), Number(right!.children[1]!.getAttribute("cy"))], squashed);
    const meant = vec2.transformMat2d(vec2.create(), [5 - 0.25, -3 + 0.55 * 0.6], worlds(SPECIMEN, pose)[1]!);
    expect(vec2.distance(drawn, meant)).toBeLessThan(0.01);
    paint(depiction, frameFor(SPECIMEN, { eyes: [{ x: 0, y: 0, lid: 1 }, OPEN] }, pose));
    expect(left!.getAttribute("transform")).toBe("translate(-5 -3) scale(1 0.1)");
    expect(right!.getAttribute("transform")).toBe("translate(5 -3) scale(1 1)");
  });

  it("bends the mouth with the spirits: up when sad, flat when neutral, down into a smile when happy", () => {
    const depiction = depicted(SPECIMEN, frameFor(SPECIMEN, { spirits:1 }));
    const mouth = pieces(depiction)[5]!;
    expect([mouth.getAttribute("class"), mouth.getAttribute("stroke-width")]).toEqual(["pet-fill-none pet-stroke-ink", "1.5"]);
    expect(mouth.getAttribute("d")).toBe("M -3 3 Q 0 6 3 3");
    paint(depiction, frameFor(SPECIMEN, { spirits:0 }));
    expect(mouth.getAttribute("d")).toBe("M -3 3 Q 0 3 3 3");
    paint(depiction, frameFor(SPECIMEN, { spirits:-1 }));
    expect(mouth.getAttribute("d")).toBe("M -3 3 Q 0 0 3 3");
    paint(depiction, frameFor(SPECIMEN, { spirits:0.4 }));
    expect(mouth.getAttribute("d")).toBe("M -3 3 Q 0 4.2 3 3");
  });

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
});
