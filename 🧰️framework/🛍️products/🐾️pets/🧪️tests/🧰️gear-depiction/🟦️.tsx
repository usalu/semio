/** 🧰️ The gear draws what an actor gets around with — a parachute, a rope with its hook, a grappling gun, a carried
 * ladder, the ladders that stand on the stage — and tilts a whole drawing about its pivot. Every transform it writes is
 * held to gl-matrix's `mat2d`, which composes the same placement the way the stage means it (feet, a turn about the
 * pivot on screen, the mirror, the size) and carries the same points; the geometry is held to what the tools are: cords
 * that end on the rim of the canopy, a rope whose middle hangs by its slack, rungs perpendicular to their rails and
 * evenly spaced. A frame that changes nothing writes nothing, and nothing is ever written in a way a
 * Content-Security-Policy without inline styles would block.
 *
 * @see ../../🎯️targets/⚛️react/🔨️modules/🧰️gear/🟦️.ts
 * @see ../../🎯️targets/⚛️react/🎨️.css
 * @see https://glmatrix.net/docs/module-mat2d.html — the third-party oracle
 */

import { glMatrix, mat2d, vec2 } from "gl-matrix";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it, vi } from "vitest";
import type { ActorFrame, Point, Species } from "@semio-tech/pets";
import { CHUTE_DOME, CHUTE_RISE, CHUTE_SPAN, GUN_HEIGHT, GUN_MUZZLE, GUN_REACH, LADDER_CARRY, LADDER_RUNG, LADDER_WIDTH, canopyRim, depict, equip, paint, paintLadders, paintTools, rackLadders, tiltDegrees, tiltedPlacement, type Equipment, type GearBearer, type GearLadder, type GearTool } from "@semio-tech/pets-react";

glMatrix.setMatrixArrayType(Array);

const css = readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../🎯️targets/⚛️react/🎨️.css"), "utf8");
const WIDTH = 40;
const HEIGHT = 30;
const GRIP = 27;
const NEAR = 0.02;

const SPECIMEN: Species = {
  id: "specimen",
  name: { en: "Specimen", de: "Exemplar" },
  thing: { en: "test rig", de: "Prüfgerüst" },
  grounds: [],
  size: { width: WIDTH, height: HEIGHT },
  palette: { body: "#1e9b8d", accent: "#34d1bf", detail: "#fa9500" },
  bones: [{ id: "root", x: 0, y: 0 }],
  parts: [{ id: "body", bone: "root", shape: { kind: "ellipse", cx: 0, cy: -15, rx: 14, ry: 12 }, fill: "body", stroke: "ink" }],
  face: { eyes: [] },
  clips: [],
  repertoire: {},
  locomotion: { gait: "walk", speed: 36 },
  temperament: { energy: 0.5, sociability: 0.5, curiosity: 0.5 },
  states: [{ id: "resting", name: { en: "Resting", de: "In Ruhe" } }],
  tricks: [],
  purr: { clip: "purr" },
  emitters: [],
  gear: ["climb", "ladder", "grapple", "parachute"],
  grip: GRIP,
  reach: 10,
  mood: "content",
};

const SCRUFF: Point = { x: 0, y: -GRIP };
const FEET: Point = { x: 0, y: 0 };

function bearing(tools: readonly GearTool[], change: Partial<GearBearer> = {}): GearBearer {
  return { x: 200, y: 120, facing: 1, tilt: 0, pivot: SCRUFF, tools, ...change };
}

/** 🔮️ gl-matrix: what a transform means, read from an SVG `transform` attribute or a CSS `transform` value: its functions multiplied in their order. */
function meant(text: string | null): mat2d {
  const matrix = mat2d.create();
  for (const [, name, body] of (text ?? "").matchAll(/(\w+)\(([^)]*)\)/gu)) {
    const numbers = body!.split(/[\s,]+/u).filter((entry) => entry !== "").map((entry) => Number.parseFloat(entry));
    if (name === "translate") mat2d.translate(matrix, matrix, [numbers[0]!, numbers[1] ?? 0]);
    else if (name === "scale") mat2d.scale(matrix, matrix, [numbers[0]!, numbers[1] ?? numbers[0]!]);
    else if (name === "rotate") mat2d.rotate(matrix, matrix, glMatrix.toRadian(numbers[0]!));
    else if (name === "matrix") mat2d.multiply(matrix, matrix, mat2d.fromValues(...(numbers as [number, number, number, number, number, number])));
    else throw new Error(`unknown transform function ${name}`);
  }
  return matrix;
}

/** 🧭️ gl-matrix: where the stage means an actor to be drawn at `size` — its feet, a turn of `tilt` on screen about the pivot (which lies on the side the actor faces), then the mirror and the size. */
function placed(bearer: GearBearer, size = 1): mat2d {
  const world = mat2d.fromTranslation(mat2d.create(), [bearer.x * size, bearer.y * size]);
  const pivot: [number, number] = [bearer.facing * bearer.pivot.x * size, bearer.pivot.y * size];
  mat2d.translate(world, world, pivot);
  mat2d.rotate(world, world, bearer.tilt * 2 * Math.PI);
  mat2d.translate(world, world, [-pivot[0], -pivot[1]]);
  return mat2d.scale(world, world, [bearer.facing * size, size]);
}

/** 🎯️ gl-matrix: the place in the tools' own frame (the axes of the stage around the feet) of a point of the rig of a tilted, mirrored actor. */
function carried(bearer: GearBearer, x: number, y: number): vec2 {
  const world = vec2.transformMat2d(vec2.create(), [x, y], placed(bearer));
  return vec2.subtract(world, world, [bearer.x, bearer.y]);
}

const numbersOf = (text: string | null): number[] => [...(text ?? "").matchAll(/-?\d+(?:\.\d+)?/gu)].map(([number]) => Number(number));
const through = (matrix: mat2d, x: number, y: number): vec2 => vec2.transformMat2d(vec2.create(), [x, y], matrix);
const apart = (one: vec2, other: readonly [number, number] | vec2): number => vec2.distance(one, other as vec2);
const shown = (element: Element): boolean => element.getAttribute("visibility") === "visible";

function equipped(bearer: GearBearer, species: Pick<Species, "size" | "grip" | "canopy"> = SPECIMEN): Equipment {
  const equipment = equip(species, document);
  paintTools(equipment, bearer);
  return equipment;
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

/** 🪜️ The two rails and the rungs of a ladder as the points its group's transform puts them at, through gl-matrix. */
function ladderGeometry(group: Element, outer: mat2d = mat2d.create()): { rails: [vec2, vec2][]; rungs: [vec2, vec2][] } {
  const matrix = mat2d.multiply(mat2d.create(), outer, meant(group.getAttribute("transform")));
  const [rails, rungs] = [...group.children];
  const [, top, length, , bottom] = numbersOf(rails!.getAttribute("d"));
  const steps = numbersOf(rungs!.getAttribute("d"));
  return {
    rails: [top!, bottom!].map((side) => [through(matrix, 0, side), through(matrix, length!, side)] as [vec2, vec2]),
    rungs: Array.from({ length: steps.length / 3 }, (_, rung) => [through(matrix, steps[rung * 3]!, steps[rung * 3 + 1]!), through(matrix, steps[rung * 3]!, steps[rung * 3 + 2]!)] as [vec2, vec2]),
  };
}

/** 📐️ What a ladder must be between its foot and its top: two straight rails of that length, {@link LADDER_WIDTH} apart and parallel, and `count` rungs across them, perpendicular, evenly spaced, half a spacing from both ends. */
function expectLadder(geometry: { rails: [vec2, vec2][]; rungs: [vec2, vec2][] }, foot: readonly [number, number], top: readonly [number, number], count: number): void {
  const middle = (one: vec2, other: vec2): vec2 => vec2.lerp(vec2.create(), one, other, 0.5);
  const [left, right] = geometry.rails;
  const length = Math.hypot(top[0] - foot[0], top[1] - foot[1]);
  expect(apart(middle(left![0], right![0]), foot)).toBeLessThan(NEAR);
  expect(apart(middle(left![1], right![1]), top)).toBeLessThan(NEAR);
  expect(vec2.distance(left![0], right![0])).toBeCloseTo(LADDER_WIDTH, 2);
  expect(vec2.distance(left![1], right![1])).toBeCloseTo(LADDER_WIDTH, 2);
  const along = vec2.normalize(vec2.create(), vec2.subtract(vec2.create(), left![1], left![0]));
  expect(geometry.rungs.length).toBe(count);
  geometry.rungs.forEach(([from, to], rung) => {
    const across = vec2.subtract(vec2.create(), to, from);
    expect(Math.abs(vec2.dot(across, along))).toBeLessThan(1e-3);
    expect(vec2.length(across)).toBeCloseTo(LADDER_WIDTH, 2);
    const reached = vec2.distance(middle(from, to), foot as unknown as vec2);
    expect(reached).toBeCloseTo(((rung + 0.5) * length) / count, 1);
  });
}

describe("gear depiction", () => {
  it("builds every tool once and hidden: the parachute and the carried ladder for the back, rope, hook and gun for the front", () => {
    const equipment = equip(SPECIMEN, document);
    const tags = (element: Element): string[] => [...element.children].map((child) => `${child.tagName}${child.getAttribute("data-pet-tool") === null ? "" : `:${child.getAttribute("data-pet-tool")}`}`);
    expect([equipment.back.tagName, equipment.back.getAttribute("class"), equipment.front.getAttribute("class")]).toEqual(["g", "pet-gear", "pet-gear"]);
    expect(equipment.back.namespaceURI).toBe("http://www.w3.org/2000/svg");
    expect(tags(equipment.back)).toEqual(["g:chute", "g:ladder"]);
    expect(tags(equipment.front)).toEqual(["path:rope", "g:hook", "g:gun"]);
    expect(tags(equipment.chute)).toEqual(["path", "g"]);
    expect(tags(equipment.canopy)).toEqual(["path", "path"]);
    expect(tags(equipment.ladder.group)).toEqual(["path", "path"]);
    expect(tags(equipment.hook)).toEqual(["line", "path"]);
    expect(tags(equipment.gun)).toEqual(["line", "rect", "rect"]);
    const tools = [...equipment.back.querySelectorAll("[data-pet-tool]"), ...equipment.front.querySelectorAll("[data-pet-tool]")];
    expect(tools.map((tool) => tool.getAttribute("visibility"))).toEqual(["hidden", "hidden", "hidden", "hidden", "hidden"]);
    expect([equipment.back.hasAttribute("transform"), equipment.front.hasAttribute("transform")]).toEqual([false, false]);
    for (const root of [equipment.back, equipment.front]) {
      expect(root.querySelector("[id], [style], defs, use, symbol, clipPath, marker, style, script, title, desc, a, text, [tabindex]")).toBeNull();
      for (const shape of root.querySelectorAll("path, line, rect, ellipse")) {
        const classes = (shape.getAttribute("class") ?? "").split(" ");
        expect(classes.length).toBe(2);
        for (const name of classes) expect(css).toContain(`.${name} {`);
      }
    }
    expect([equipment.cords.getAttribute("class"), equipment.rope.getAttribute("class")]).toEqual(["pet-fill-none pet-stroke-ink", "pet-fill-none pet-stroke-ink"]);
    expect([...equipment.canopy.children].map((piece) => piece.getAttribute("class"))).toEqual(["pet-fill-accent pet-stroke-ink", "pet-fill-paper pet-stroke-ink"]);
    expect(equipment.gun.children[1]!.getAttribute("class")).toBe("pet-fill-accent pet-stroke-ink");
  });

  it("tilts the whole drawing about its pivot as gl-matrix composes it, the same way on screen whichever way the actor faces", () => {
    const points: [number, number][] = [[0, 0], [12, -30], [-9, -4], [0, -GRIP]];
    for (const facing of [1, -1] as const) {
      for (const tilt of [20 / 360, -20 / 360, 0.3, -0.731]) {
        for (const pivot of [SCRUFF, { x: 4.25, y: -20.5 }, FEET]) {
          for (const size of [1, 0.8]) {
            const bearer = bearing([], { x: 123.25, y: 77.5, facing, tilt, pivot });
            const written = meant(tiltedPlacement(bearer.x * size, bearer.y * size, facing * size, size, tilt, pivot));
            const stage = placed(bearer, size);
            for (const [x, y] of points) expect(apart(through(written, x, y), through(stage, x, y))).toBeLessThan(NEAR);
            const still = placed({ ...bearer, tilt: 0 }, size);
            expect(apart(through(written, pivot.x, pivot.y), through(still, pivot.x, pivot.y))).toBeLessThan(NEAR);
          }
        }
      }
      const hung = meant(tiltedPlacement(50, 90, facing, 1, 0.05, SCRUFF));
      expect(through(hung, 0, 0)[0]).toBeLessThan(50 - 5);
      expect(through(hung, 0, 0)[1]).toBeLessThan(90);
    }
    expect([tiltDegrees(1, 0.05), tiltDegrees(-1, 0.05), tiltDegrees(-0.8, -0.25), tiltDegrees(1, 0)]).toEqual([18, -18, 90, 0]);
    expect(tiltedPlacement(12, 300, -1.5, 1.5, 0.05, { x: 2, y: -27 })).toBe("translate(12px, 300px) scale(-1.5, 1.5) translate(2px, -27px) rotate(-18deg) translate(-2px, 27px)");
  });

  it("places an actor that is not tilted exactly as the depiction places it", () => {
    const frame: ActorFrame = { species: "specimen", x: 120.504, y: 80.25, facing: -1, activity: "idle", opacity: 1, bones: [1, 0, 0, 1, 0, 0], eyes: [], footing: "perch", state: "resting", mood: "content", intensity: 0, spirits: 0, tilt: 0, pivot: SCRUFF, tools: [], body: { x: 100, y: 30, width: 40, height: 50 } };
    const depiction = depict(SPECIMEN, document);
    paint(depiction, frame, 1.5);
    expect(tiltedPlacement(120.5, 80.25, -1.5, 1.5, 0, SCRUFF)).toBe(depiction.element.style.transform);
    expect(tiltedPlacement(120.5, 80.25, -1.5, 1.5, 0.004 / 360, SCRUFF)).toBe("translate(120.5px, 80.25px) scale(-1.5, 1.5)");
    expect(tiltedPlacement(120.5, 80.25, -1.5, 1.5, 1, SCRUFF)).not.toBe(depiction.element.style.transform);
  });

  it("lays the tools out in the axes of the stage around the feet: its groups undo the mirror and the tilt of the drawing", () => {
    for (const facing of [1, -1] as const) {
      for (const tilt of [0, 20 / 360, -0.2, 0.31]) {
        for (const pivot of [SCRUFF, { x: 4.25, y: -20.5 }]) {
          const bearer = bearing([{ kind: "gun", aim: 0 }], { facing, tilt, pivot });
          const equipment = equipped(bearer);
          expect(equipment.front.getAttribute("transform")).toBe(equipment.back.getAttribute("transform"));
          for (const size of [1, 0.8]) {
            const drawing = meant(tiltedPlacement(bearer.x * size, bearer.y * size, facing * size, size, tilt, pivot));
            const inside = mat2d.multiply(mat2d.create(), drawing, meant(equipment.back.getAttribute("transform")));
            const stage = mat2d.fromValues(size, 0, 0, size, bearer.x * size, bearer.y * size);
            for (const [x, y] of [[0, 0], [46, -70], [-30, 12]] as const) expect(apart(through(inside, x, y), through(stage, x, y))).toBeLessThan(NEAR);
          }
        }
      }
    }
    expect(equipped(bearing([{ kind: "gun", aim: 0 }])).back.getAttribute("transform")).toBe("matrix(1 0 0 1 0 0)");
    expect(equipped(bearing([{ kind: "gun", aim: 0 }], { facing: -1 })).back.getAttribute("transform")).toBe("matrix(-1 0 0 1 0 0)");
  });

  it("hangs the parachute from the grip, its cords ending on the rim of the canopy", () => {
    const span = CHUTE_SPAN * WIDTH;
    const rim: [number, number][] = [[-span, 0], [-span / 3, 0], [span / 3, 0], [span, 0]];
    for (const open of [0.15, 0.4, 0.7, 1, 1.25]) {
      for (const [facing, tilt, sway] of [[1, 0, 0], [1, -15 / 360, 0.04], [-1, 15 / 360, -0.04]] as const) {
        const bearer = bearing([{ kind: "chute", open, sway }], { facing, tilt });
        const equipment = equipped(bearer);
        expect(shown(equipment.chute)).toBe(true);
        const hung = meant(equipment.chute.getAttribute("transform"));
        expect(apart(through(hung, 0, 0), carried(bearer, 0, -GRIP))).toBeLessThan(NEAR);
        const lean = mat2d.rotate(mat2d.create(), mat2d.fromTranslation(mat2d.create(), carried(bearer, 0, -GRIP)), sway * 2 * Math.PI);
        expect(apart(through(hung, 0, -20), through(lean, 0, -20))).toBeLessThan(NEAR);
        const canopy = meant(equipment.canopy.getAttribute("transform"));
        const cords = numbersOf(equipment.cords.getAttribute("d"));
        expect(cords.length).toBe(16);
        rim.forEach(([x, y], cord) => {
          expect(cords.slice(cord * 4, cord * 4 + 2)).toEqual([0, 0]);
          expect(apart(through(canopy, x, y), [cords[cord * 4 + 2]!, cords[cord * 4 + 3]!])).toBeLessThan(NEAR);
        });
      }
    }
    const dome = numbersOf(equip(SPECIMEN, document).canopy.children[0]!.getAttribute("d"));
    expect([dome.slice(0, 2), dome.slice(6, 8), dome.slice(10, 12), dome.slice(14, 16), dome.slice(18, 20)]).toEqual([[-span, 0], [span, 0], [span / 3, 0], [-span / 3, 0], [-span, 0]]);
    expect(Math.min(...dome.filter((_, index) => index % 2 === 1))).toBeCloseTo((-CHUTE_DOME * HEIGHT * 4) / 3, 2);
    const gore = numbersOf(equip(SPECIMEN, document).canopy.children[1]!.getAttribute("d"));
    expect([gore.slice(0, 2), gore.slice(4, 6), gore.slice(8, 10)]).toEqual([[0, -CHUTE_DOME * HEIGHT], [span / 3, 0], [-span / 3, 0]]);
  });

  it("opens the canopy from a narrow streamer close to the pet to its full span, squashed as it overshoots, and hides a packed one", () => {
    const canopy = (open: number): string | null => equipped(bearing([{ kind: "chute", open, sway: 0 }])).canopy.getAttribute("transform");
    expect(CHUTE_RISE * HEIGHT).toBeCloseTo(21, 9);
    expect(canopy(1)).toBe("translate(0 -21) scale(1 1)");
    expect(canopy(1.25)).toBe("translate(0 -21) scale(0.938 1.25)");
    expect(canopy(0.5)).toBe("translate(0 -14.7) scale(0.65 0.5)");
    expect(canopy(0.15)).toBe("translate(0 -10.29) scale(0.405 0.15)");
    expect(equipped(bearing([{ kind: "chute", open: 1, sway: 0.04 }])).chute.getAttribute("transform")).toBe("translate(0 -27) rotate(14.4)");
    for (const open of [0, 0.0004, -1]) expect(shown(equipped(bearing([{ kind: "chute", open, sway: 0 }])).chute)).toBe(false);
    const own = equipped(bearing([{ kind: "chute", open: 1, sway: 0 }]), { size: SPECIMEN.size, grip: GRIP, canopy: { kind: "ellipse", cx: 0, cy: -8, rx: 30, ry: 8 } });
    expect(own.canopy.children.length).toBe(1);
    const flown = own.canopy.children[0]!;
    expect([flown.tagName, flown.getAttribute("class"), flown.getAttribute("stroke-width"), flown.getAttribute("rx"), flown.getAttribute("cy")]).toEqual(["ellipse", "pet-fill-accent pet-stroke-ink", "2", "30", "-8"]);
    expect(numbersOf(own.cords.getAttribute("d")).slice(2, 4)).toEqual([-30, -29]);
  });

  it("runs the cords of a canopy of its own to its rim: the ends of what the shape spans, wherever it was drawn", () => {
    const rim = (canopy: Species["canopy"]): number[] => canopyRim({ size: SPECIMEN.size, grip: GRIP, canopy }).flatMap((end) => [end.x, end.y]);
    expect(rim(undefined)).toEqual([-CHUTE_SPAN * WIDTH, 0, CHUTE_SPAN * WIDTH, 0]);
    expect(rim({ kind: "path", d: "M -16 0 Q 0 -18 16 0 Z" })).toEqual([-16, 0, 16, 0]);
    expect(rim({ kind: "path", d: "M-16,0Q0-18,16,0Z" })).toEqual([-16, 0, 16, 0]);
    expect(rim({ kind: "path", d: "m -12 2 q 12 -20 24 0 z" })).toEqual([-12, 2, 12, 2]);
    expect(rim({ kind: "path", d: "M -10 0 V -8 H 10 v 8 Z" })).toEqual([-10, 0, 10, 0]);
    expect(rim({ kind: "path", d: "m -12 0 l 12 -10 l 12 10 z l -4 3" })).toEqual([-16, 3, 12, 0]);
    expect(rim({ kind: "path", d: "M 0 -20 C 30 -20 22 -4 18 0 L 6 -3 l -12 0 L -18 0 C -22 -4 -30 -20 0 -20 Z" })).toEqual([-18, 0, 18, 0]);
    expect(rim({ kind: "path", d: "M 14 1 A 14 9 0 0 0 -14 1.5 Z" })).toEqual([-14, 1.5, 14, 1]);
    expect(rim({ kind: "ellipse", cx: 2, cy: -8, rx: 30, ry: 8 })).toEqual([-28, -8, 32, -8]);
    expect(rim({ kind: "rect", x: -14, y: -10, width: 28, height: 10, radius: 3 })).toEqual([-14, 0, 14, 0]);
    expect(rim({ kind: "line", x1: 9, y1: -2, x2: -9, y2: -4 })).toEqual([-9, -4, 9, -2]);
    for (const d of ["", "nonsense", "M 5 5", "M 5 5 L 5 -9", "M 1e999 0 L 3 0", "L"]) expect(rim({ kind: "path", d })).toEqual([-30, 0, 30, 0]);
    for (const canopy of [{ kind: "path", d: "M -16 0 Q 0 -18 16 0 Z" }, { kind: "line", x1: 9, y1: -2, x2: -9, y2: -4 }, { kind: "ellipse", cx: 2, cy: -8, rx: 30, ry: 8 }] as const) {
      for (const open of [0.5, 1, 1.2]) {
        const species = { size: SPECIMEN.size, grip: GRIP, canopy };
        const equipment = equipped(bearing([{ kind: "chute", open, sway: 0.03 }], { tilt: 0.02, facing: -1 }), species);
        const [left, right] = canopyRim(species);
        expect(equipment.rim).toEqual([left, right]);
        const flown = meant(equipment.canopy.getAttribute("transform"));
        const cords = numbersOf(equipment.cords.getAttribute("d"));
        for (let cord = 0; cord < 4; cord++) {
          const end = vec2.lerp(vec2.create(), [left.x, left.y], [right.x, right.y], cord / 3);
          expect(apart(through(flown, end[0], end[1]), [cords[cord * 4 + 2]!, cords[cord * 4 + 3]!])).toBeLessThan(NEAR);
        }
      }
    }
  });

  it("runs the rope from the pivot the actor hangs from to its point of the stage, taut or hanging by its slack, with the hook pointing the way it arrives", () => {
    for (const facing of [1, -1] as const) {
      for (const tilt of [0, 20 / 360, -20 / 360]) {
        for (const slack of [0, 12, -8]) {
          const bearer = bearing([{ kind: "rope", x: 246, y: 50, slack }, { kind: "hook", x: 246, y: 50 }], { facing, tilt, pivot: { x: 3, y: -GRIP } });
          const equipment = equipped(bearer);
          const text = equipment.rope.getAttribute("d")!;
          const rope = numbersOf(text);
          const near: [number, number] = [rope[0]!, rope[1]!];
          const far: [number, number] = [rope.at(-2)!, rope.at(-1)!];
          expect(apart(carried(bearer, 3, -GRIP), near)).toBeLessThan(NEAR);
          expect(near).toEqual([facing * 3, -GRIP]);
          expect(far).toEqual([46, -70]);
          expect(text).toMatch(slack === 0 ? /^M \S+ \S+ L \S+ \S+$/u : /^M \S+ \S+ Q \S+ \S+ \S+ \S+$/u);
          const control: [number, number] = slack === 0 ? near : [rope[2]!, rope[3]!];
          if (slack !== 0) {
            const halfway = vec2.lerp(vec2.create(), vec2.lerp(vec2.create(), near, control, 0.5), vec2.lerp(vec2.create(), control, far, 0.5), 0.5);
            expect(halfway[1] - (near[1] + far[1]) / 2).toBeCloseTo(slack, 2);
            expect(halfway[0]).toBeCloseTo((near[0] + far[0]) / 2, 2);
          }
          const hook = meant(equipment.hook.getAttribute("transform"));
          expect(apart(through(hook, 0, 0), far)).toBeLessThan(NEAR);
          const arriving = vec2.normalize(vec2.create(), [far[0] - control[0], far[1] - control[1]]);
          const pointing = vec2.subtract(vec2.create(), through(hook, 1, 0), through(hook, 0, 0));
          expect(apart(pointing, arriving)).toBeLessThan(1e-4);
          expect(mat2d.determinant(hook)).toBeCloseTo(1, 4);
        }
      }
    }
    const alone = equipped(bearing([{ kind: "hook", x: 230, y: 70 }]));
    expect(alone.hook.getAttribute("transform")).toBe("matrix(0 -1 1 0 30 -50)");
    expect([shown(alone.hook), shown(alone.rope)]).toEqual([true, false]);
    const reeled = equipped(bearing([{ kind: "rope", x: 200, y: 120 - GRIP, slack: 0 }, { kind: "hook", x: 200, y: 120 - GRIP }]));
    expect([reeled.rope.getAttribute("d"), reeled.hook.getAttribute("transform")]).toEqual(["M 0 -27 L 0 -27", "matrix(0 -1 1 0 0 -27)"]);
  });

  it("holds the gun at the actor's side, turned to its aim with the handle down, and lets the rope leave its muzzle", () => {
    for (const facing of [1, -1] as const) {
      for (const tilt of [0, -20 / 360]) {
        for (const aim of [0, -0.125, -0.25, 0.08, 0.5, 0.625, -0.3]) {
          const bearer = bearing([{ kind: "gun", aim }, { kind: "rope", x: 246, y: 50, slack: 0 }], { facing, tilt });
          const equipment = equipped(bearer);
          const gun = meant(equipment.gun.getAttribute("transform"));
          const hand = carried(bearer, GUN_REACH * WIDTH, -GUN_HEIGHT * HEIGHT);
          expect(apart(through(gun, 0, 0), hand)).toBeLessThan(NEAR);
          expect(Math.sign(hand[0])).toBe(facing);
          const barrel = mat2d.rotate(mat2d.create(), mat2d.fromTranslation(mat2d.create(), hand), aim * 2 * Math.PI);
          expect(apart(through(gun, GUN_MUZZLE, 0), through(barrel, GUN_MUZZLE, 0))).toBeLessThan(NEAR);
          const handle = vec2.subtract(vec2.create(), through(gun, 0, 5), through(gun, 0, 0));
          const below = vec2.subtract(vec2.create(), through(barrel, 0, Math.cos(aim * 2 * Math.PI) < 0 ? -5 : 5), through(barrel, 0, 0));
          expect(apart(handle, below)).toBeLessThan(1e-3);
          if (Math.abs(Math.cos(aim * 2 * Math.PI)) > 0.1) expect(handle[1]).toBeGreaterThan(0);
          const rope = numbersOf(equipment.rope.getAttribute("d"));
          expect(apart(through(barrel, GUN_MUZZLE, 0), [rope[0]!, rope[1]!])).toBeLessThan(NEAR);
        }
      }
    }
    expect([GUN_REACH * WIDTH, GUN_HEIGHT * HEIGHT].map((value) => Math.round(value * 100) / 100)).toEqual([16.8, 15]);
    expect(equipped(bearing([{ kind: "gun", aim: -0.125 }])).gun.getAttribute("transform")).toBe("translate(16.8 -15) rotate(-45) scale(1 1)");
    expect(equipped(bearing([{ kind: "gun", aim: 0.625 }], { facing: -1 })).gun.getAttribute("transform")).toBe("translate(-16.8 -15) rotate(225) scale(1 -1)");
  });

  it("carries a ladder at its middle on the back: two rails, and rungs perpendicular to them and evenly spaced; raised, its foot stands at the feet", () => {
    const hold = LADDER_CARRY * HEIGHT;
    for (const facing of [1, -1] as const) {
      for (const [lean, length] of [[0.25, 60], [0.22, 110], [-0.22, 110], [0.23, 37]] as const) {
        const bearer = bearing([{ kind: "ladder", lean, length }], { facing, pivot: FEET });
        const equipment = equipped(bearer);
        expect(shown(equipment.ladder.group)).toBe(true);
        const axis = through(mat2d.fromRotation(mat2d.create(), lean * 2 * Math.PI), 0, -length / 2);
        expectLadder(ladderGeometry(equipment.ladder.group), [-axis[0], -hold - axis[1]], [axis[0], -hold + axis[1]], Math.floor(length / LADDER_RUNG));
      }
    }
    for (const lean of [0.04, -0.1, 0]) {
      const geometry = ladderGeometry(equipped(bearing([{ kind: "ladder", lean, length: 110 }], { pivot: FEET })).ladder.group);
      const axis = through(mat2d.fromRotation(mat2d.create(), lean * 2 * Math.PI), 0, -1);
      const foot: [number, number] = [(hold * axis[0]) / axis[1], 0];
      expectLadder(geometry, foot, [foot[0] + 110 * axis[0], 110 * axis[1]], 10);
      const held = vec2.cross([0, 0, 0], vec2.fromValues(axis[0], axis[1]), vec2.fromValues(0 - foot[0], -hold - foot[1]));
      expect(Math.abs(held[2]!)).toBeLessThan(NEAR);
    }
    expect(shown(equipped(bearing([{ kind: "ladder", lean: 0.25, length: 0 }])).ladder.group)).toBe(false);
    expect(ladderGeometry(equipped(bearing([{ kind: "ladder", lean: 0.25, length: 8 }])).ladder.group).rungs.length).toBe(1);
    expect(equipped(bearing([{ kind: "ladder", lean: 0.25, length: 60 }], { pivot: FEET })).ladder.rails.getAttribute("d")).toBe("M 0 -5 H 60 M 0 5 H 60");
  });

  it("stands the ladders of the stage between foot and top at the size pets are drawn at, builds each once and hides the ones that left", () => {
    const rack = rackLadders(document);
    expect([rack.element.tagName, rack.element.getAttribute("class"), rack.element.getAttribute("focusable"), rack.element.getAttribute("overflow"), rack.element.childElementCount]).toEqual(["svg", "pet-ladders", "false", "visible", 0]);
    expect(writes(rack.element, () => paintLadders(rack, [], 0.8))).toEqual([]);
    expect(rack.element.getAttribute("style")).toBeNull();
    const ladders: GearLadder[] = [
      { x0: 240, y0: 300, x1: 257.5, y1: 230, rungs: 6, opacity: 1 },
      { x0: 90, y0: 300, x1: 60, y1: 180, rungs: 11, opacity: 0.5 },
    ];
    paintLadders(rack, ladders, 0.8);
    expect(rack.element.style.transform).toBe("scale(0.8)");
    expect(rack.element.childElementCount).toBe(2);
    const size = meant(rack.element.style.transform);
    ladders.forEach((ladder, index) => {
      const group = rack.element.children[index]!;
      expect([group.getAttribute("data-pet-tool"), group.getAttribute("visibility"), group.getAttribute("opacity")]).toEqual(["ladder", "visible", String(ladder.opacity)]);
      expectLadder(ladderGeometry(group), [ladder.x0, ladder.y0], [ladder.x1, ladder.y1], ladder.rungs);
      const drawn = ladderGeometry(group, size).rails[0]!;
      expect(vec2.distance(drawn[0], drawn[1])).toBeCloseTo(0.8 * Math.hypot(ladder.x1 - ladder.x0, ladder.y1 - ladder.y0), 1);
    });
    expect(writes(rack.element, () => paintLadders(rack, ladders, 0.8))).toEqual([]);
    expect(writes(rack.element, () => paintLadders(rack, [{ ...ladders[0]!, opacity: 0.7 }, ladders[1]!], 0.8))).toEqual(["attributes:g.opacity"]);
    expect(writes(rack.element, () => paintLadders(rack, [{ ...ladders[0]!, opacity: 0.7, x1: 300, y1: 270 }, ladders[1]!], 0.8))).toEqual(["attributes:g.transform", "attributes:path.d", "attributes:path.d"]);
    expect(writes(rack.element, () => paintLadders(rack, [ladders[1]!], 0.8)).sort()).toEqual(["attributes:g.opacity", "attributes:g.transform", "attributes:g.visibility", "attributes:path.d", "attributes:path.d"]);
    expect([...rack.element.children].map((group) => group.getAttribute("visibility"))).toEqual(["visible", "hidden"]);
    expect(writes(rack.element, () => paintLadders(rack, [], 0.8))).toEqual(["attributes:g.visibility"]);
    expect(writes(rack.element, () => paintLadders(rack, [], 0.8))).toEqual([]);
    expect(writes(rack.element, () => paintLadders(rack, [ladders[1]!, ladders[0]!, ladders[0]!], 1)).filter((record) => record.startsWith("childList"))).toEqual(["childList:svg.null"]);
    expect(rack.element.childElementCount).toBe(3);
    expect(rack.element.style.transform).toBe("scale(1)");
    expect(shown(rack.element.children[0]!)).toBe(true);
    paintLadders(rack, [{ ...ladders[0]!, opacity: 0 }, { ...ladders[1]!, rungs: 0 }], 1);
    expect([...rack.element.children].map(shown)).toEqual([false, false, false]);
    expect(rack.element.querySelector("[id], defs, use, style")).toBeNull();
  });

  it("writes nothing when a frame changes nothing, and only what a frame changes", () => {
    const equipment = equip(SPECIMEN, document);
    const drawing = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    drawing.append(equipment.back, equipment.front);
    const setAttribute = vi.spyOn(Element.prototype, "setAttribute");
    paintTools(equipment, bearing([], { tilt: 0.1, facing: -1 }));
    expect(setAttribute).not.toHaveBeenCalled();
    vi.restoreAllMocks();
    const chute: GearTool = { kind: "chute", open: 1, sway: 0 };
    expect(writes(drawing, () => paintTools(equipment, bearing([chute])))).toEqual(["attributes:g.transform", "attributes:g.transform", "attributes:g.visibility", "attributes:g.transform", "attributes:g.transform", "attributes:path.d"]);
    expect(writes(drawing, () => paintTools(equipment, bearing([chute])))).toEqual([]);
    expect(writes(drawing, () => paintTools(equipment, bearing([chute], { x: 205.5, y: 131 })))).toEqual([]);
    expect(writes(drawing, () => paintTools(equipment, bearing([{ ...chute, sway: 0.02 }])))).toEqual(["attributes:g.transform"]);
    expect(writes(drawing, () => paintTools(equipment, bearing([{ ...chute, sway: 0.020001, open: 1.0004 }])))).toEqual([]);
    expect(writes(drawing, () => paintTools(equipment, bearing([{ ...chute, sway: 0.02, open: 0.9 }])))).toEqual(["attributes:g.transform", "attributes:path.d"]);
    const leaning = { tilt: 0.05 };
    expect(writes(drawing, () => paintTools(equipment, bearing([{ ...chute, sway: 0.02, open: 0.9 }], leaning)))).toEqual(["attributes:g.transform", "attributes:g.transform"]);
    const line: GearTool[] = [{ ...chute, sway: 0.02, open: 0.9 }, { kind: "rope", x: 246, y: 50, slack: 0 }, { kind: "hook", x: 246, y: 50 }];
    expect(writes(drawing, () => paintTools(equipment, bearing(line, leaning)))).toEqual(["attributes:path.visibility", "attributes:path.d", "attributes:g.visibility", "attributes:g.transform"]);
    expect(writes(drawing, () => paintTools(equipment, bearing(line, leaning)))).toEqual([]);
    expect(writes(drawing, () => paintTools(equipment, bearing(line, { ...leaning, x: 201 })))).toEqual(["attributes:path.d", "attributes:g.transform"]);
    expect(writes(drawing, () => paintTools(equipment, bearing([line[1]!, { kind: "rope", x: 0, y: 0, slack: 9 }, line[2]!], { ...leaning, x: 201 })))).toEqual(["attributes:g.visibility"]);
    expect(writes(drawing, () => paintTools(equipment, bearing([], { ...leaning, x: 201 })))).toEqual(["attributes:path.visibility", "attributes:g.visibility"]);
    expect([equipment.chute, equipment.ladder.group, equipment.rope, equipment.hook, equipment.gun].map(shown)).toEqual([false, false, false, false, false]);
    expect(writes(drawing, () => paintTools(equipment, bearing([], { tilt: -0.2, x: 0 })))).toEqual([]);
    expect(writes(drawing, () => paintTools(equipment, bearing([{ kind: "gun", aim: 0 }, { kind: "ladder", lean: 0.25, length: 60 }], leaning))).sort()).toEqual(["attributes:g.transform", "attributes:g.transform", "attributes:g.visibility", "attributes:g.visibility", "attributes:path.d", "attributes:path.d"]);
  });

  it("never writes in a way a policy without inline styles blocks, and never builds or removes an element for a frame", () => {
    const setAttribute = vi.spyOn(Element.prototype, "setAttribute");
    const innerHTML = vi.spyOn(Element.prototype, "innerHTML", "set");
    const outerHTML = vi.spyOn(Element.prototype, "outerHTML", "set");
    const cssText = vi.spyOn(CSSStyleDeclaration.prototype, "cssText", "set");
    const equipment = equip(SPECIMEN, document);
    const drawing = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    drawing.append(equipment.back, equipment.front);
    const everything: GearTool[] = [{ kind: "chute", open: 1.1, sway: 0.03 }, { kind: "rope", x: 246, y: 50, slack: 5 }, { kind: "hook", x: 246, y: 50 }, { kind: "gun", aim: -0.1 }, { kind: "ladder", lean: 0.2, length: 80 }];
    const records = writes(drawing, () => {
      paintTools(equipment, bearing(everything, { tilt: 0.05, facing: -1 }));
      paintTools(equipment, bearing(everything.slice(2), { tilt: -0.05 }));
      paintTools(equipment, bearing([]));
    });
    const rack = rackLadders(document);
    paintLadders(rack, [{ x0: 0, y0: 0, x1: 20, y1: -80, rungs: 7, opacity: 1 }], 0.8);
    expect(records.length).toBeGreaterThan(20);
    expect(records.filter((record) => !record.startsWith("attributes:"))).toEqual([]);
    expect(setAttribute.mock.calls.map(([name]) => name)).not.toContain("style");
    expect(setAttribute.mock.calls.map(([name]) => name)).not.toContain("id");
    expect(innerHTML).not.toHaveBeenCalled();
    expect(outerHTML).not.toHaveBeenCalled();
    expect(cssText).not.toHaveBeenCalled();
    vi.restoreAllMocks();
    expect(drawing.querySelector("[style], [id], defs, use, style")).toBeNull();
  });

  it("has the rules its drawings need: the roots of ladders and particles at the layer's origin, and the hand of the learner as a cursor", () => {
    const rules = css.replace(/\/\*[\s\S]*?\*\//gu, "");
    const roots = /\.pet-ladders,\s*\.pet-effects \{([^}]*)\}/u.exec(rules)?.[1] ?? "";
    for (const declaration of ["position: absolute;", "left: 0;", "top: 0;", "width: 1px;", "height: 1px;", "overflow: visible;", "transform-origin: 0 0;", "stroke-linejoin: round;", "stroke-linecap: round;"]) expect(roots).toContain(declaration);
    expect(roots).not.toContain("will-change");
    expect(rules).toMatch(/\[data-pet-held\],\s*\[data-pet-held\] \* \{\s*cursor: grabbing !important;\s*\}/u);
    expect(rules.match(/cursor:[^;]*;/gu), "grab is shown on the element under the pointer by the hand itself").toEqual(["cursor: grabbing !important;"]);
    expect(rules, "a rule naming the mark that comes and goes with every pass over a pet would restyle the page with it").not.toContain("data-pet-cursor");
    expect([...rules.matchAll(/([^{}]+)\{([^}]*)\}/gu)].filter(([, , body]) => /pointer-events:\s*(?!none)\S/u.test(body!)).map(([, selector]) => selector!.trim()), "only the touch pads take pointer events").toEqual([".pet-pad"]);
    expect(css).not.toContain("animation-name:");
    expect(rules).not.toMatch(/\.pet-gear/u);
  });
});
