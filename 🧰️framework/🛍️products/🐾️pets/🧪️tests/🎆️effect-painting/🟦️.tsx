/** 🎆️ The effects paint the particles of a frame from pools of elements that are built once per emitter, and the tint
 * of a state over the palette of its species. A particle is one shape of its emitter, placed, turned and sized by a
 * `transform` that is held to gl-matrix's `mat2d`, which composes the same placement and carries the same points. A
 * frame writes only what changed — nothing at all when nothing lives —, never builds or removes an element while
 * particles fly, hides what it does not use, and leaves nothing behind when it is struck. The dust where a pet vanished
 * is pooled the same way, behind the particles, and every blob of it lies where gl-matrix carries the blob's middle and
 * rim. Nothing is ever written in a way a Content-Security-Policy without inline styles would block.
 *
 * @see ../../🎯️targets/⚛️react/🔨️modules/✨️effects/🟦️.ts
 * @see ../../🎯️targets/⚛️react/🎨️.css
 * @see https://glmatrix.net/docs/module-mat2d.html — the third-party oracle
 */

import { glMatrix, mat2d, vec2 } from "gl-matrix";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it, vi } from "vitest";
import type { Emitter, Palette, Tint } from "@semio-tech/pets";
import { PUFF_CLOUDS, PUFF_OUTLINE, PUFF_RING, PUFF_RISE, paintEffects, paintPuffs, retireEffects, stageEffects, stockEffects, strikeEffects, tintEffects, tintPalette, type EffectParticle, type EffectPuff, type EffectSpecies, type Effects } from "@semio-tech/pets-react";

glMatrix.setMatrixArrayType(Array);

const css = readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../🎯️targets/⚛️react/🎨️.css"), "utf8");

const SPARK: Emitter = { id: "spark", bone: "root", x: 0, y: -20, shape: { kind: "path", d: "M 0 -4 L 1 -1 L 4 0 L 1 1 L 0 4 L -1 1 L -4 0 L -1 -1 Z" }, fill: "accent", stroke: "ink", strokeWidth: 0.75, motion: "burst", count: 4, life: 0.9, speed: 40, spread: 1 };
const DROP: Emitter = { id: "drop", bone: "root", x: 0, y: -8, shape: { kind: "ellipse", cx: 0, cy: 0, rx: 1.6, ry: 3 }, fill: "detail", stroke: "none", motion: "fall", count: 3, life: 0.8, speed: 60, spread: 0.1 };
const STREAK: Emitter = { id: "streak", bone: "root", x: 0, y: 0, shape: { kind: "line", x1: 0, y1: 0, x2: 8, y2: 0 }, fill: "none", stroke: "paper", motion: "drift", count: 2, life: 1, speed: 10, spread: 0.2 };
const CHIP: Emitter = { id: "chip", bone: "root", x: 0, y: 0, shape: { kind: "rect", x: -1.5, y: -1, width: 3, height: 2, radius: 0.5 }, fill: "body", stroke: "ink", motion: "orbit", count: 2, life: 1, speed: 10, spread: 1 };

const SUNNY: EffectSpecies = { id: "sunny", palette: { body: "#fccf05", accent: "#fa9500", detail: "#ff344f" }, emitters: [SPARK, DROP] };
const CLOUDY: EffectSpecies = { id: "cloudy", palette: { body: "#63939a", accent: "#34d1bf", detail: "#c4e4d5" }, emitters: [DROP, STREAK, CHIP] };
const KINDS = new Map([SUNNY, CLOUDY].map((species) => [species.id, species]));

function particle(species: string, emitter: string, change: Partial<EffectParticle> = {}): EffectParticle {
  return { species, emitter, x: 100, y: 50, scale: 1, rotation: 0, opacity: 1, ...change };
}

/** 🔮️ gl-matrix: what an SVG `transform` attribute or a CSS `transform` value means: its functions multiplied in their order. */
function meant(text: string | null): mat2d {
  const matrix = mat2d.create();
  for (const [, name, body] of (text ?? "").matchAll(/(\w+)\(([^)]*)\)/gu)) {
    const numbers = body!.split(/[\s,]+/u).filter((entry) => entry !== "").map((entry) => Number.parseFloat(entry));
    if (name === "translate") mat2d.translate(matrix, matrix, [numbers[0]!, numbers[1] ?? 0]);
    else if (name === "scale") mat2d.scale(matrix, matrix, [numbers[0]!, numbers[1] ?? numbers[0]!]);
    else if (name === "rotate") mat2d.rotate(matrix, matrix, glMatrix.toRadian(numbers[0]!));
    else throw new Error(`unknown transform function ${name}`);
  }
  return matrix;
}

/** 🧭️ gl-matrix: where the stage means a particle to be drawn at `size` — its place, a turn on screen, its size. */
function placed(drawn: EffectParticle, size: number): mat2d {
  const world = mat2d.fromScaling(mat2d.create(), [size, size]);
  mat2d.translate(world, world, [drawn.x, drawn.y]);
  mat2d.rotate(world, world, drawn.rotation * 2 * Math.PI);
  return mat2d.scale(world, world, [drawn.scale, drawn.scale]);
}

/** 👀️ Everything written to `root` and below while `act` runs: the kind of record, the element and the attribute, in order. */
function writes(root: Node, act: () => void): string[] {
  const observer = new MutationObserver(() => {});
  observer.observe(root, { attributes: true, childList: true, characterData: true, subtree: true });
  act();
  const records = observer.takeRecords();
  observer.disconnect();
  return records.map((record) => `${record.type}:${(record.target as Element).tagName}.${record.attributeName}`);
}

const pool = (effects: Effects, species: string, emitter: string): Element[] => [...effects.element.querySelectorAll(`[data-pet-effects="${species}"] > [data-pet-emitter="${emitter}"]`)];
const clouds = (effects: Effects): Element[] => [...effects.element.querySelectorAll("[data-pet-puffs] > [data-pet-puff]")];

const PUFF: EffectPuff = { x: 200, y: 120, width: 48, height: 40, phase: 0 };

/** 🌫️ gl-matrix: where the dust of `puff` means the middle of blob `blob` to be on the stage — the middle of the body, risen by the phase, then for a blob of the ring a turn of `index / ring + phase / 20` turns carried onto the ellipse of the body's half-width and half-height, spread out by the phase. */
function blobMiddle(puff: EffectPuff, blob: number): vec2 {
  const spread = 1 - (1 - puff.phase) * (1 - puff.phase);
  const reach = 0.45 + 0.55 * spread;
  const world = mat2d.fromTranslation(mat2d.create(), [puff.x, puff.y - PUFF_RISE * puff.height * puff.phase]);
  if (blob === 0) return vec2.transformMat2d(vec2.create(), [0, 0], world);
  mat2d.scale(world, world, [(reach * puff.width) / 2, (reach * puff.height) / 2]);
  mat2d.rotate(world, world, ((blob - 1) / PUFF_RING + 0.05 * puff.phase) * 2 * Math.PI);
  return vec2.transformMat2d(vec2.create(), [1, 0], world);
}
const visible = (elements: readonly Element[]): number => elements.filter((element) => element.getAttribute("visibility") === "visible").length;

function staged(): { effects: Effects; layer: HTMLElement } {
  const layer = document.createElement("div");
  const effects = stageEffects(document);
  layer.append(effects.element);
  return { effects, layer };
}

describe("effect painting", () => {
  it("builds one empty root for all particles and writes nothing while nothing lives", () => {
    const setAttribute = vi.spyOn(Element.prototype, "setAttribute");
    const { effects, layer } = staged();
    const built = setAttribute.mock.calls.length;
    expect([effects.element.tagName, effects.element.namespaceURI, effects.element.getAttribute("class"), effects.element.getAttribute("focusable"), effects.element.getAttribute("overflow")]).toEqual(["svg", "http://www.w3.org/2000/svg", "pet-effects", "false", "visible"]);
    expect(effects.element.getAttributeNames()).toEqual(["class", "focusable", "overflow"]);
    expect(writes(layer, () => {
      for (let frame = 0; frame < 5; frame++) {
        paintEffects(effects, KINDS, [], 0.8);
        paintPuffs(effects, [], 0.8);
      }
    })).toEqual([]);
    expect(setAttribute.mock.calls.length).toBe(built);
    expect(effects.element.childElementCount).toBe(0);
    expect(effects.element.getAttribute("style")).toBeNull();
    vi.restoreAllMocks();
  });

  it("stocks a pool per emitter once: as many hidden elements as it can have particles alive, each its shape with the paints of a part", () => {
    const { effects } = staged();
    stockEffects(effects, SUNNY);
    stockEffects(effects, CLOUDY);
    expect([...effects.element.children].map((group) => [group.tagName, group.getAttribute("data-pet-effects"), group.childElementCount])).toEqual([["g", "sunny", 7], ["g", "cloudy", 7]]);
    const attributes = (element: Element): Record<string, string> => Object.fromEntries([...element.attributes].map((attribute) => [attribute.name, attribute.value]));
    expect(pool(effects, "sunny", "spark").length).toBe(4);
    expect(attributes(pool(effects, "sunny", "spark")[0]!)).toEqual({ d: SPARK.shape.kind === "path" ? SPARK.shape.d : "", class: "pet-fill-accent pet-stroke-ink", "stroke-width": "0.75", "data-pet-emitter": "spark", visibility: "hidden" });
    expect(attributes(pool(effects, "sunny", "drop")[2]!)).toEqual({ cx: "0", cy: "0", rx: "1.6", ry: "3", class: "pet-fill-detail pet-stroke-none", "data-pet-emitter": "drop", visibility: "hidden" });
    expect(attributes(pool(effects, "cloudy", "streak")[1]!)).toEqual({ x1: "0", y1: "0", x2: "8", y2: "0", class: "pet-fill-none pet-stroke-paper", "stroke-width": "2", "data-pet-emitter": "streak", visibility: "hidden" });
    expect(attributes(pool(effects, "cloudy", "chip")[0]!)).toEqual({ x: "-1.5", y: "-1", width: "3", height: "2", rx: "0.5", class: "pet-fill-body pet-stroke-ink", "stroke-width": "2", "data-pet-emitter": "chip", visibility: "hidden" });
    expect([pool(effects, "sunny", "spark")[0]!.tagName, pool(effects, "sunny", "drop")[0]!.tagName, pool(effects, "cloudy", "streak")[0]!.tagName, pool(effects, "cloudy", "chip")[0]!.tagName]).toEqual(["path", "ellipse", "line", "rect"]);
    for (const shape of effects.element.querySelectorAll("[data-pet-emitter]")) for (const name of shape.getAttribute("class")!.split(" ")) expect(css).toContain(`.${name} {`);
    const sunny = effects.element.children[0] as SVGGElement;
    expect(["--pet-body", "--pet-accent", "--pet-detail"].map((name) => sunny.style.getPropertyValue(name))).toEqual(["#fccf05", "#fa9500", "#ff344f"]);
    expect(writes(effects.element, () => {
      stockEffects(effects, SUNNY);
      stockEffects(effects, CLOUDY);
      stockEffects(effects, { id: "bare", palette: SUNNY.palette, emitters: [] });
    })).toEqual([]);
    expect(effects.element.querySelector("[id], defs, use, symbol, style, script, title, a, [tabindex]")).toBeNull();
    expect(effects.element.querySelectorAll("[style]").length).toBe(2);
  });

  it("draws the n-th particle of an emitter with the n-th element of its pool, placed, turned and sized as gl-matrix composes it", () => {
    const { effects } = staged();
    const frame = [
      particle("sunny", "spark", { x: 120.504, y: 80.25, scale: 0.8, rotation: 0.125, opacity: 0.6004 }),
      particle("cloudy", "drop", { x: 30, y: 200.119, scale: 1.5, rotation: -0.02 }),
      particle("sunny", "spark", { x: -4.2, y: 7.001, scale: 1.2504, rotation: 0.731, opacity: 0.25 }),
      particle("sunny", "drop", { x: 61, y: 62 }),
    ];
    for (const size of [1, 0.8]) {
      paintEffects(effects, KINDS, frame, size);
      expect(effects.element.style.transform).toBe(`scale(${size})`);
      const drawn = [pool(effects, "sunny", "spark")[0]!, pool(effects, "cloudy", "drop")[0]!, pool(effects, "sunny", "spark")[1]!, pool(effects, "sunny", "drop")[0]!];
      const root = meant(effects.element.style.transform);
      frame.forEach((flying, index) => {
        const written = mat2d.multiply(mat2d.create(), root, meant(drawn[index]!.getAttribute("transform")));
        const stage = placed(flying, size);
        for (const point of [[0, 0], [4, 0], [-1, 3]] as const) expect(vec2.distance(vec2.transformMat2d(vec2.create(), point, written), vec2.transformMat2d(vec2.create(), point, stage))).toBeLessThan(0.012);
        expect(drawn[index]!.getAttribute("visibility")).toBe("visible");
      });
    }
    expect(pool(effects, "sunny", "spark")[0]!.getAttribute("transform")).toBe("translate(120.5 80.25) rotate(45) scale(0.8)");
    expect(pool(effects, "sunny", "spark")[0]!.getAttribute("opacity")).toBe("0.6");
    expect(pool(effects, "sunny", "spark")[1]!.getAttribute("transform")).toBe("translate(-4.2 7) rotate(263.16) scale(1.25)");
    expect(pool(effects, "cloudy", "drop")[0]!.getAttribute("transform")).toBe("translate(30 200.12) rotate(-7.2) scale(1.5)");
    expect([visible(pool(effects, "sunny", "spark")), visible(pool(effects, "sunny", "drop")), visible(pool(effects, "cloudy", "drop")), visible(pool(effects, "cloudy", "streak"))]).toEqual([2, 1, 1, 0]);
    expect(effects.live).toBe(4);
  });

  it("writes only what changed, hides what fell out of use and never builds or removes an element while particles fly", () => {
    const { effects, layer } = staged();
    stockEffects(effects, SUNNY);
    const one = particle("sunny", "spark", { x: 10, y: 10 });
    const two = particle("sunny", "spark", { x: 20, y: 10, opacity: 0.5 });
    expect(writes(layer, () => paintEffects(effects, KINDS, [one, two]))).toEqual(["attributes:svg.style", "attributes:path.transform", "attributes:path.opacity", "attributes:path.visibility", "attributes:path.transform", "attributes:path.opacity", "attributes:path.visibility"]);
    expect(writes(layer, () => paintEffects(effects, KINDS, [one, two]))).toEqual([]);
    expect(writes(layer, () => paintEffects(effects, KINDS, [{ ...one, x: 10.004, rotation: 0.00001, scale: 1.0004, opacity: 0.9996 }, two]))).toEqual([]);
    expect(writes(layer, () => paintEffects(effects, KINDS, [{ ...one, x: 11 }, two]))).toEqual(["attributes:path.transform"]);
    expect(writes(layer, () => paintEffects(effects, KINDS, [{ ...one, x: 11 }, { ...two, opacity: 0.4 }]))).toEqual(["attributes:path.opacity"]);
    expect(writes(layer, () => paintEffects(effects, KINDS, [{ ...one, x: 11 }, { ...two, opacity: 0.4, rotation: 0.1, scale: 2 }]))).toEqual(["attributes:path.transform"]);
    expect(writes(layer, () => paintEffects(effects, KINDS, [{ ...two, opacity: 0.4, rotation: 0.1, scale: 2 }]))).toEqual(["attributes:path.transform", "attributes:path.opacity", "attributes:path.visibility"]);
    expect(pool(effects, "sunny", "spark").map((element) => element.getAttribute("visibility"))).toEqual(["visible", "hidden", "hidden", "hidden"]);
    expect(writes(layer, () => paintEffects(effects, KINDS, []))).toEqual(["attributes:path.visibility"]);
    expect(visible(pool(effects, "sunny", "spark"))).toBe(0);
    const setAttribute = vi.spyOn(Element.prototype, "setAttribute");
    expect(writes(layer, () => {
      for (let frame = 0; frame < 5; frame++) paintEffects(effects, KINDS, []);
    })).toEqual([]);
    expect(setAttribute).not.toHaveBeenCalled();
    vi.restoreAllMocks();
    expect(writes(layer, () => paintEffects(effects, KINDS, [{ ...two, opacity: 0.4, rotation: 0.1, scale: 2 }]))).toEqual(["attributes:path.visibility"]);
    const flight = writes(layer, () => {
      for (let frame = 0; frame < 30; frame++) paintEffects(effects, KINDS, Array.from({ length: frame % 5 }, (_, index) => particle("sunny", index % 2 === 0 ? "spark" : "drop", { x: frame * 3 + index, y: 40 - frame, rotation: frame / 30, opacity: 1 - frame / 30 })));
    });
    expect(flight.length).toBeGreaterThan(60);
    expect(flight.filter((record) => !record.startsWith("attributes:"))).toEqual([]);
    expect(new Set(flight)).toEqual(new Set(["attributes:path.transform", "attributes:path.opacity", "attributes:path.visibility", "attributes:ellipse.transform", "attributes:ellipse.opacity", "attributes:ellipse.visibility"]));
  });

  it("builds the pool of an emitter nobody stocked with its first particle, once, and draws no particle the contract does not allow", () => {
    const { effects, layer } = staged();
    const first = writes(layer, () => paintEffects(effects, KINDS, [particle("cloudy", "streak")]));
    expect(first.filter((record) => record.startsWith("childList"))).toEqual(["childList:svg.null", "childList:g.null"]);
    expect(pool(effects, "cloudy", "streak").length).toBe(2);
    expect(pool(effects, "cloudy", "drop").length).toBe(0);
    expect(writes(layer, () => paintEffects(effects, KINDS, [particle("cloudy", "streak", { x: 1 }), particle("cloudy", "streak", { x: 2 })])).filter((record) => record.startsWith("childList"))).toEqual([]);
    const crowd = [particle("cloudy", "streak", { x: 1 }), particle("cloudy", "streak", { x: 2 }), particle("cloudy", "streak", { x: 3 }), particle("nobody", "spark"), particle("cloudy", "spark"), particle("sunny", "streak"), particle("cloudy", "chip", { x: 4 })];
    const crowded = writes(layer, () => paintEffects(effects, KINDS, crowd));
    expect(crowded.filter((record) => record.startsWith("childList"))).toEqual(["childList:g.null"]);
    expect(pool(effects, "cloudy", "streak").map((element) => element.getAttribute("transform"))).toEqual(["translate(1 50) rotate(0) scale(1)", "translate(2 50) rotate(0) scale(1)"]);
    expect([effects.live, effects.element.childElementCount, visible(pool(effects, "cloudy", "chip"))]).toEqual([3, 1, 1]);
    expect(effects.element.querySelector('[data-pet-effects="nobody"], [data-pet-effects="sunny"]')).toBeNull();
    const fractional: EffectSpecies = { id: "odd", palette: SUNNY.palette, emitters: [{ ...SPARK, count: 2.9 }, { ...DROP, count: -3 }] };
    stockEffects(effects, fractional);
    expect([pool(effects, "odd", "spark").length, pool(effects, "odd", "drop").length]).toEqual([2, 0]);
    paintEffects(effects, new Map([[fractional.id, fractional]]), [particle("odd", "drop")]);
    expect(effects.live).toBe(0);
  });

  it("paints the tint of a state over the palette through the custom properties, restores the palette, and writes only a colour that changes", () => {
    const palette: Palette = { body: "#63939a", accent: "#34d1bf", detail: "#c4e4d5" };
    const heavy: Tint = { body: "#46606a", accent: "#2fa89a", detail: "#8ea9a8" };
    const element = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    const colours = (): string[] => ["--pet-body", "--pet-accent", "--pet-detail"].map((name) => element.style.getPropertyValue(name));
    tintPalette(element, palette);
    expect(colours()).toEqual(["#63939a", "#34d1bf", "#c4e4d5"]);
    tintPalette(element, palette, heavy);
    expect(colours()).toEqual(["#46606a", "#2fa89a", "#8ea9a8"]);
    tintPalette(element, palette, { body: "#ffe45c" });
    expect(colours()).toEqual(["#ffe45c", "#34d1bf", "#c4e4d5"]);
    tintPalette(element, palette, { detail: "#ffffff" });
    expect(colours()).toEqual(["#63939a", "#34d1bf", "#ffffff"]);
    tintPalette(element, palette, {});
    expect(colours()).toEqual(["#63939a", "#34d1bf", "#c4e4d5"]);
    tintPalette(element, palette, heavy);
    tintPalette(element, palette, undefined);
    expect(colours()).toEqual(["#63939a", "#34d1bf", "#c4e4d5"]);
    const setProperty = vi.spyOn(CSSStyleDeclaration.prototype, "setProperty");
    const cssText = vi.spyOn(CSSStyleDeclaration.prototype, "cssText", "set");
    const setAttribute = vi.spyOn(Element.prototype, "setAttribute");
    tintPalette(element, palette);
    tintPalette(element, palette, {});
    expect(setProperty).not.toHaveBeenCalled();
    tintPalette(element, palette, { accent: "#000000" });
    expect(setProperty.mock.calls).toEqual([["--pet-accent", "#000000"]]);
    tintPalette(element, palette, { accent: "#000000" });
    expect(setProperty.mock.calls.length).toBe(1);
    tintPalette(element, palette, heavy);
    expect(setProperty.mock.calls.slice(1)).toEqual([["--pet-body", "#46606a"], ["--pet-accent", "#2fa89a"], ["--pet-detail", "#8ea9a8"]]);
    expect(cssText).not.toHaveBeenCalled();
    expect(setAttribute).not.toHaveBeenCalled();
    vi.restoreAllMocks();
    const page = document.createElement("div");
    tintPalette(page, palette, heavy);
    expect(page.style.getPropertyValue("--pet-body")).toBe("#46606a");
  });

  it("lets particles follow the tint of their species' state, also a tint given before the species has any", () => {
    const { effects } = staged();
    const wet: Tint = { detail: "#2bc2b0" };
    tintEffects(effects, CLOUDY, wet);
    expect(effects.element.childElementCount).toBe(0);
    paintEffects(effects, KINDS, [particle("cloudy", "drop"), particle("sunny", "drop")]);
    const group = (species: string): SVGGElement => effects.element.querySelector<SVGGElement>(`[data-pet-effects="${species}"]`)!;
    const colours = (species: string): string[] => ["--pet-body", "--pet-accent", "--pet-detail"].map((name) => group(species).style.getPropertyValue(name));
    expect(colours("cloudy")).toEqual(["#63939a", "#34d1bf", "#2bc2b0"]);
    expect(colours("sunny")).toEqual(["#fccf05", "#fa9500", "#ff344f"]);
    tintEffects(effects, SUNNY, { body: "#ffe45c", accent: "#ff7a00" });
    expect(colours("sunny")).toEqual(["#ffe45c", "#ff7a00", "#ff344f"]);
    expect(writes(effects.element, () => tintEffects(effects, SUNNY, { body: "#ffe45c", accent: "#ff7a00" }))).toEqual([]);
    tintEffects(effects, CLOUDY);
    tintEffects(effects, SUNNY, undefined);
    expect([colours("cloudy"), colours("sunny")]).toEqual([["#63939a", "#34d1bf", "#c4e4d5"], ["#fccf05", "#fa9500", "#ff344f"]]);
  });

  it("takes a species that left away and leaves nothing behind when it is struck", () => {
    const { effects, layer } = staged();
    tintEffects(effects, CLOUDY, { detail: "#2bc2b0" });
    paintEffects(effects, KINDS, [particle("cloudy", "drop"), particle("sunny", "spark"), particle("sunny", "spark")], 0.8);
    expect([effects.live, effects.element.childElementCount]).toEqual([3, 2]);
    retireEffects(effects, "sunny");
    retireEffects(effects, "nobody");
    expect([effects.live, effects.element.childElementCount, effects.element.querySelector('[data-pet-effects="sunny"]')]).toEqual([1, 1, null]);
    paintEffects(effects, KINDS, [particle("cloudy", "drop")], 0.8);
    expect(effects.live).toBe(1);
    retireEffects(effects, "cloudy");
    expect(effects.live).toBe(0);
    paintEffects(effects, KINDS, [particle("cloudy", "drop")], 0.8);
    expect(effects.element.querySelector<SVGGElement>('[data-pet-effects="cloudy"]')!.style.getPropertyValue("--pet-detail")).toBe("#c4e4d5");
    paintEffects(effects, KINDS, [particle("sunny", "drop")], 0.8);
    strikeEffects(effects);
    expect(layer.childElementCount).toBe(0);
    expect([effects.element.childElementCount, effects.element.isConnected, effects.live, effects.troupes.size, effects.tints.size]).toEqual([0, false, 0, 0, 0]);
    layer.append(effects.element);
    expect(writes(layer, () => paintEffects(effects, KINDS, []))).toEqual([]);
    paintEffects(effects, KINDS, [particle("sunny", "drop")], 0.5);
    expect([effects.element.style.transform, visible(pool(effects, "sunny", "drop")), pool(effects, "sunny", "drop").length]).toEqual(["scale(0.5)", 1, 3]);
  });

  it("draws the dust where a pet vanished behind every particle: an outline in ink round the union of blobs in paper that spread from the body to its outline, rise and fade — every blob where gl-matrix carries it", () => {
    const { effects } = staged();
    paintEffects(effects, KINDS, [particle("sunny", "spark")], 0.8);
    for (const phase of [0, 0.25, 0.5, 0.9, 1.4]) {
      const puff = { ...PUFF, phase: Math.min(phase, 1) };
      paintPuffs(effects, [{ ...PUFF, phase }], 0.8);
      expect([effects.element.firstElementChild!.getAttribute("data-pet-puffs"), effects.element.lastElementChild!.getAttribute("data-pet-effects")]).toEqual(["", "sunny"]);
      const [cloud] = clouds(effects);
      const circles = [...cloud!.children];
      const blobs = PUFF_RING + 1;
      expect(circles.map((circle) => `${circle.tagName} ${circle.getAttribute("class")} ${circle.getAttribute("stroke-width")}`)).toEqual([...Array.from({ length: blobs }, () => `circle pet-fill-ink pet-stroke-ink ${PUFF_OUTLINE}`), ...Array.from({ length: blobs }, () => "circle pet-fill-paper pet-stroke-none null")]);
      const placed = mat2d.multiply(mat2d.create(), meant(effects.element.style.transform), meant(cloud!.getAttribute("transform")));
      const root = meant(effects.element.style.transform);
      const size = ((puff.width + puff.height) / 4) * (1 - puff.phase / 2);
      for (let blob = 0; blob < blobs; blob++) {
        const [outline, fill] = [circles[blob]!, circles[blobs + blob]!];
        expect(["cx", "cy", "r"].map((name) => outline.getAttribute(name))).toEqual(["cx", "cy", "r"].map((name) => fill.getAttribute(name)));
        const middle = vec2.transformMat2d(vec2.create(), [Number(fill.getAttribute("cx")), Number(fill.getAttribute("cy"))], placed);
        const rim = vec2.transformMat2d(vec2.create(), [Number(fill.getAttribute("cx")) + Number(fill.getAttribute("r")), Number(fill.getAttribute("cy"))], placed);
        expect(vec2.distance(middle, vec2.transformMat2d(vec2.create(), blobMiddle(puff, blob), root)), `${phase} ${blob}`).toBeLessThan(0.02);
        expect(Math.abs(vec2.distance(middle, rim) - 0.8 * size * (blob === 0 ? 0.5 : (blob - 1) % 2 === 0 ? 0.36 : 0.28))).toBeLessThan(0.01);
      }
      expect(Math.abs(Number(cloud!.getAttribute("opacity")) - (1 - puff.phase * puff.phase))).toBeLessThan(0.0006);
      expect(cloud!.getAttribute("visibility")).toBe("visible");
    }
  });

  it("pools the clouds of the dust: one is built only when more puffs are in the air than ever before, a frame writes only what changed, a cloud that cleared is hidden, and no more than the cap is drawn", () => {
    const { effects, layer } = staged();
    expect(writes(layer, () => paintPuffs(effects, []))).toEqual([]);
    const first = writes(layer, () => paintPuffs(effects, [PUFF]));
    expect(first.filter((record) => record.startsWith("childList"))).toEqual(["childList:svg.null", "childList:g.null"]);
    expect(first.filter((record) => record.startsWith("attributes:")).slice(0, 3)).toEqual(["attributes:svg.style", "attributes:g.transform", "attributes:g.opacity"]);
    expect(first.at(-1)).toBe("attributes:g.visibility");
    expect(writes(layer, () => paintPuffs(effects, [PUFF]))).toEqual([]);
    expect(writes(layer, () => paintPuffs(effects, [{ ...PUFF, x: 200.004, phase: 1e-7 }]))).toEqual([]);
    const spreading = writes(layer, () => paintPuffs(effects, [{ ...PUFF, phase: 0.5 }]));
    expect(spreading.filter((record) => !record.startsWith("attributes:"))).toEqual([]);
    expect(new Set(spreading)).toEqual(new Set(["attributes:g.transform", "attributes:g.opacity", "attributes:circle.cx", "attributes:circle.cy", "attributes:circle.r"]));
    expect(writes(layer, () => paintPuffs(effects, [{ ...PUFF, phase: 0.5, x: 210 }]))).toEqual(["attributes:g.transform"]);
    const second = writes(layer, () => paintPuffs(effects, [{ ...PUFF, phase: 0.5, x: 210 }, { ...PUFF, x: 40 }]));
    expect(second.filter((record) => record.startsWith("childList"))).toEqual(["childList:g.null"]);
    expect(writes(layer, () => paintPuffs(effects, [{ ...PUFF, phase: 0.5, x: 210 }]))).toEqual(["attributes:g.visibility"]);
    expect(clouds(effects).map((cloud) => cloud.getAttribute("visibility"))).toEqual(["visible", "hidden"]);
    expect(writes(layer, () => paintPuffs(effects, [{ ...PUFF, phase: 0.5, x: 210 }, { ...PUFF, x: 40 }]))).toEqual(["attributes:g.visibility"]);
    expect(writes(layer, () => paintPuffs(effects, []))).toEqual(["attributes:g.visibility", "attributes:g.visibility"]);
    const setAttribute = vi.spyOn(Element.prototype, "setAttribute");
    expect(writes(layer, () => {
      for (let frame = 0; frame < 5; frame++) paintPuffs(effects, []);
    })).toEqual([]);
    expect(setAttribute).not.toHaveBeenCalled();
    vi.restoreAllMocks();
    const crowd = writes(layer, () => paintPuffs(effects, Array.from({ length: PUFF_CLOUDS + 3 }, (_, index) => ({ ...PUFF, x: 60 * index, phase: index / 20 }))));
    expect(crowd.filter((record) => record.startsWith("childList")).length).toBe(PUFF_CLOUDS - 2);
    expect([clouds(effects).length, clouds(effects).filter((cloud) => cloud.getAttribute("visibility") === "visible").length, effects.dust.live]).toEqual([PUFF_CLOUDS, PUFF_CLOUDS, PUFF_CLOUDS]);
    expect(clouds(effects).map((cloud) => cloud.getAttribute("transform")).at(-1)).toBe(`translate(${60 * (PUFF_CLOUDS - 1)} ${120 - PUFF_RISE * 40 * ((PUFF_CLOUDS - 1) / 20)})`);
    expect(effects.element.querySelector("[id], defs, use, symbol, style, script, title, a, [tabindex], [style]")).toBeNull();
    strikeEffects(effects);
    expect([layer.childElementCount, effects.element.childElementCount, effects.dust.group, effects.dust.clouds.length, effects.dust.live]).toEqual([0, 0, null, 0, 0]);
    layer.append(effects.element);
    paintPuffs(effects, [PUFF], 0.5);
    expect([effects.element.style.transform, clouds(effects).length, clouds(effects)[0]!.getAttribute("visibility")]).toEqual(["scale(0.5)", 1, "visible"]);
  });

  it("never writes in a way a policy without inline styles blocks", () => {
    const setAttribute = vi.spyOn(Element.prototype, "setAttribute");
    const innerHTML = vi.spyOn(Element.prototype, "innerHTML", "set");
    const outerHTML = vi.spyOn(Element.prototype, "outerHTML", "set");
    const cssText = vi.spyOn(CSSStyleDeclaration.prototype, "cssText", "set");
    const { effects } = staged();
    stockEffects(effects, SUNNY);
    tintEffects(effects, SUNNY, { body: "#ffe45c" });
    paintEffects(effects, KINDS, [particle("sunny", "spark"), particle("cloudy", "chip", { rotation: 0.3 })], 0.8);
    paintPuffs(effects, [PUFF, { ...PUFF, phase: 0.6 }], 0.8);
    paintEffects(effects, KINDS, [particle("cloudy", "chip", { rotation: 0.4, opacity: 0.2 })], 1);
    paintPuffs(effects, [{ ...PUFF, phase: 0.7 }], 1);
    paintEffects(effects, KINDS, []);
    paintPuffs(effects, []);
    strikeEffects(effects);
    expect(setAttribute.mock.calls.length).toBeGreaterThan(30);
    expect(setAttribute.mock.calls.map(([name]) => name)).not.toContain("style");
    expect(setAttribute.mock.calls.map(([name]) => name)).not.toContain("id");
    expect(innerHTML).not.toHaveBeenCalled();
    expect(outerHTML).not.toHaveBeenCalled();
    expect(cssText).not.toHaveBeenCalled();
    vi.restoreAllMocks();
  });
});
