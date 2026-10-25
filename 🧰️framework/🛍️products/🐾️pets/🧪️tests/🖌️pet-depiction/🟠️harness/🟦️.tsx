/** 🟠️ The shared declarations of the pet depiction suite tests. */
import { glMatrix, mat2d } from "gl-matrix";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { expect } from "vitest";
import { type ActorFrame, type EyeFrame, type Species } from "@semio-tech/pets";
import { depict, paint, type Depiction } from "@semio-tech/pets-react";

export const css = readFileSync(resolve(dirname(dirname(fileURLToPath(import.meta.url))), "../../🎯️targets/⚛️react/🎨️.css"), "utf8");

export const ROUNDING = 0.0005 + 1e-9;

export const SPECIMEN: Species = {
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

export type Offset = { readonly x?: number; readonly y?: number; readonly rotation?: number; readonly scaleX?: number; readonly scaleY?: number };

/** 🔮️ gl-matrix: the world matrix of every bone, parents first — translate, rotate, scale, then the parent's matrix. */
export function worlds(species: Species, pose: Readonly<Record<string, Offset>> = {}): mat2d[] {
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

export const OPEN: EyeFrame = { x: 0, y: 0, lid: 0 };

export function frameFor(species: Species, change: Partial<ActorFrame> = {}, pose: Readonly<Record<string, Offset>> = {}): ActorFrame {
  return { species: species.id, x: 0, y: 0, facing: 1, activity: "idle", opacity: 1, bones: worlds(species, pose).flatMap((world) => [...world]), eyes: species.face.eyes.map(() => OPEN), footing: "perch", state: "resting", mood: "content", intensity: 0, spirits: 0, tilt: 0, pivot: { x: 0, y: -species.grip }, tools: [], body: { x: -species.size.width / 2, y: -species.size.height, width: species.size.width, height: species.size.height }, ...change };
}

export function depicted(species: Species, frame: ActorFrame = frameFor(species), scale?: number): Depiction {
  const depiction = depict(species, document);
  paint(depiction, frame, scale);
  return depiction;
}

/** 🔎️ The six numbers of a `matrix(a b c d e f)` attribute, each written with at most three decimals. */
export function matrixOf(element: Element): mat2d {
  const text = element.getAttribute("transform") ?? "";
  expect(text).toMatch(/^matrix\((?:-?\d+(?:\.\d{1,3})?(?: |\)$)){6}/u);
  return mat2d.fromValues(...(text.slice("matrix(".length, -1).split(" ").map(Number) as [number, number, number, number, number, number]));
}

/** 🪩️ gl-matrix: what a CSS `transform` of translations, scalings and turns means, its functions multiplied in their order. */
export function cssMatrix(text: string): mat2d {
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

export const groups = (depiction: Depiction): Element[] => [...depiction.element.children];

export const pieces = (depiction: Depiction): Element[] => groups(depiction).flatMap((group) => [...group.children]);

export const shapes = (depiction: Depiction): string[][] => groups(depiction).map((group) => [...group.children].map((piece) => piece.tagName));

/** 👀️ Everything written to the tree while `act` runs: attribute names per element tag, in order. */
export function writes(depiction: Depiction, act: () => void): string[] {
  const observer = new MutationObserver(() => {});
  observer.observe(depiction.element, { attributes: true, childList: true, characterData: true, subtree: true });
  act();
  const records = observer.takeRecords();
  observer.disconnect();
  return records.map((record) => `${record.type}:${(record.target as Element).tagName}.${record.attributeName}`);
}
