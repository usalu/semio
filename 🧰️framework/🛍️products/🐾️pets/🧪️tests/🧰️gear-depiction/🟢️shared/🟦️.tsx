/** 🟢️ The shared declarations of the gear depiction suite tests. */
import { glMatrix, mat2d, vec2 } from "gl-matrix";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { expect } from "vitest";
import type { Point, Species } from "@semio-tech/pets";
import { LADDER_WIDTH, equip, paintTools, type Equipment, type GearBearer, type GearTool } from "@semio-tech/pets-react";

export const css = readFileSync(resolve(dirname(dirname(fileURLToPath(import.meta.url))), "../../🎯️targets/⚛️react/🎨️.css"), "utf8");

export const WIDTH = 40;

export const HEIGHT = 30;

export const GRIP = 27;

export const NEAR = 0.02;

export const SPECIMEN: Species = {
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

export const SCRUFF: Point = { x: 0, y: -GRIP };

export const FEET: Point = { x: 0, y: 0 };

export function bearing(tools: readonly GearTool[], change: Partial<GearBearer> = {}): GearBearer {
  return { x: 200, y: 120, facing: 1, tilt: 0, pivot: SCRUFF, tools, ...change };
}

/** 🔮️ gl-matrix: what a transform means, read from an SVG `transform` attribute or a CSS `transform` value: its functions multiplied in their order. */
export function meant(text: string | null): mat2d {
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
export function placed(bearer: GearBearer, size = 1): mat2d {
  const world = mat2d.fromTranslation(mat2d.create(), [bearer.x * size, bearer.y * size]);
  const pivot: [number, number] = [bearer.facing * bearer.pivot.x * size, bearer.pivot.y * size];
  mat2d.translate(world, world, pivot);
  mat2d.rotate(world, world, bearer.tilt * 2 * Math.PI);
  mat2d.translate(world, world, [-pivot[0], -pivot[1]]);
  return mat2d.scale(world, world, [bearer.facing * size, size]);
}

/** 🎯️ gl-matrix: the place in the tools' own frame (the axes of the stage around the feet) of a point of the rig of a tilted, mirrored actor. */
export function carried(bearer: GearBearer, x: number, y: number): vec2 {
  const world = vec2.transformMat2d(vec2.create(), [x, y], placed(bearer));
  return vec2.subtract(world, world, [bearer.x, bearer.y]);
}

export const numbersOf = (text: string | null): number[] => [...(text ?? "").matchAll(/-?\d+(?:\.\d+)?/gu)].map(([number]) => Number(number));

export const through = (matrix: mat2d, x: number, y: number): vec2 => vec2.transformMat2d(vec2.create(), [x, y], matrix);

export const apart = (one: vec2, other: readonly [number, number] | vec2): number => vec2.distance(one, other as vec2);

export const shown = (element: Element): boolean => element.getAttribute("visibility") === "visible";

export function equipped(bearer: GearBearer, species: Pick<Species, "size" | "grip" | "canopy"> = SPECIMEN): Equipment {
  const equipment = equip(species, document);
  paintTools(equipment, bearer);
  return equipment;
}

/** 👀️ Everything written to `root` and below while `act` runs: the kind of record, the element and the attribute, in order. */
export function writes(root: Node, act: () => void): string[] {
  const observer = new MutationObserver(() => {});
  observer.observe(root, { attributes: true, childList: true, characterData: true, subtree: true });
  act();
  const records = observer.takeRecords();
  observer.disconnect();
  return records.map((record) => `${record.type}:${(record.target as Element).tagName}.${record.attributeName}`);
}

/** 🪜️ The two rails and the rungs of a ladder as the points its group's transform puts them at, through gl-matrix. */
export function ladderGeometry(group: Element, outer: mat2d = mat2d.create()): { rails: [vec2, vec2][]; rungs: [vec2, vec2][] } {
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
export function expectLadder(geometry: { rails: [vec2, vec2][]; rungs: [vec2, vec2][] }, foot: readonly [number, number], top: readonly [number, number], count: number): void {
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
