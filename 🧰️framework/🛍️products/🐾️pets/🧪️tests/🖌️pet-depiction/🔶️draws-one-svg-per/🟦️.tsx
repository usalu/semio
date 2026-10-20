/** 🔶️ The draws one svg per tests of the pet depiction suite. */
import { mat2d, vec2 } from "gl-matrix";
import { expect, it } from "vitest";
import { PAINTS, type Species } from "@semio-tech/pets";
import { paint } from "@semio-tech/pets-react";
import { OPEN, ROUNDING, SPECIMEN, css, cssMatrix, depicted, frameFor, groups, matrixOf, pieces, shapes, worlds, writes } from "../🟠️shared/🟦️.tsx";

/** 🔷 Registers the draws one svg per tests of the pet depiction suite. */
export function registerDrawsOneSvgPerTests(): void {
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
}
