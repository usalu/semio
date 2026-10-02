/** 🦴️ Unit suite of the pets rig: affine products, inverses and carried points against gl-matrix `mat2d`, solved skeletons against a gl-matrix chain and numpy's committed answers, the laws of the look offset, and the ban on platform transcendentals.
 *
 * @see ../../🟦️.ts — the module under test
 * @see ../../../../🧫️fixtures/🦴️rig-solving/🔣️.json — numpy's answers (case 🦴️rig-solving)
 * @see ../../../../🧫️fixtures/👀️gaze-tracking/🔣️.json — numpy's answers (case 👀️gaze-tracking)
 * @see https://glmatrix.net/docs/module-mat2d.html — the JavaScript oracle
 */
import { mat2d, vec2 } from "gl-matrix";
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import type { Bone, Point, Species } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { randomBetween } from "../../../🎲️randomness/🟦️.ts";
import { cosTurns, sinTurns } from "../../../📐️trigonometry/🟦️.ts";
import { type Affine, type BonePose, IDENTITY, type Pose, compose, invert, lookOffset, restPose, solveRig, transform } from "../../🟦️.ts";

type RigVectors = {
  readonly products: readonly { readonly id: string; readonly parent: Affine; readonly local: Affine; readonly expected: Affine; readonly bits: readonly string[] }[];
  readonly inverses: readonly { readonly id: string; readonly matrix: Affine; readonly singular: boolean; readonly expected: Affine; readonly bits: readonly string[] }[];
  readonly points: readonly { readonly id: string; readonly matrix: Affine; readonly x: number; readonly y: number; readonly expected: Point; readonly bits: PointBits }[];
  readonly species: readonly Species[];
  readonly restPoses: readonly { readonly id: string; readonly species: string; readonly expected: { readonly pose: Pose; readonly bones: readonly number[] } }[];
  readonly skeletons: readonly { readonly id: string; readonly species: string; readonly pose: Pose; readonly expected: readonly number[]; readonly bits: readonly string[] }[];
};

type GazeVectors = {
  readonly offsets: readonly { readonly id: string; readonly eye: Point; readonly target: Point; readonly reach: number; readonly expected: Point; readonly bits: PointBits }[];
  readonly eyes: readonly { readonly id: string; readonly bone: Affine; readonly eye: Point; readonly target: Point; readonly reach: number; readonly expected: { readonly eye: Point; readonly offset: Point }; readonly bits: { readonly eye: PointBits; readonly offset: PointBits } }[];
};

type PointBits = { readonly x: string; readonly y: string };

type Matrix = [number, number, number, number, number, number];

const RIG = JSON.parse(readFileSync(new URL("../../../../🧫️fixtures/🦴️rig-solving/🔣️.json", import.meta.url), "utf8")) as RigVectors;
const GAZE = JSON.parse(readFileSync(new URL("../../../../🧫️fixtures/👀️gaze-tracking/🔣️.json", import.meta.url), "utf8")) as GazeVectors;
const SOURCE = readFileSync(new URL("../../🟦️.ts", import.meta.url), "utf8");
const REST: BonePose = { x: 0, y: 0, rotation: 0, scaleX: 1, scaleY: 1 };
const BOUND = 1e-12;
const SEED = 20261002;
const VIEW = new DataView(new ArrayBuffer(8));

/** 🎯️ How many drawn matrices a comparison with gl-matrix takes at the level of the run; the laws take a fifth of it, the posed skeletons a tenth per species and the look offsets four times as many. */
const DRAWN = sampled(50, 500, 5000);

/** 🧮️ The 64-bit IEEE pattern of a double as sixteen hexadecimal digits. */
function bits(value: number): string {
  VIEW.setFloat64(0, value);
  return VIEW.getBigUint64(0).toString(16).padStart(16, "0");
}

/** 🔬️ The bit patterns of a point. */
function pointBits(point: Point): PointBits {
  return { x: bits(point.x), y: bits(point.y) };
}

/** 🧬️ The committed species of that id. */
function speciesOf(id: string): Species {
  const species = RIG.species.find((candidate) => candidate.id === id);
  if (species === undefined) throw new Error(`the vectors name the unknown species ${id}`);
  return species;
}

/** 🦿️ A species that has nothing but the given bones; `solveRig` reads no other field. */
function rigOf(bones: readonly Bone[]): Species {
  return { ...speciesOf("blobby"), id: "probe", bones };
}

/** 🤸️ The rest pose of a species with some bones changed. */
function posed(species: Species, changes: Readonly<Record<string, Partial<BonePose>>>): Pose {
  return species.bones.map((bone) => ({ ...REST, ...changes[bone.id] }));
}

/** 🎰️ A matrix with entries drawn from [−4, 4] and a translation from [−40, 40], a pure function of its index. */
function drawn(stream: number, index: number): Affine {
  const entry = (slot: number, reach: number): number => randomBetween([SEED, stream, index * 6 + slot], -reach, reach);
  return [entry(0, 4), entry(1, 4), entry(2, 4), entry(3, 4), entry(4, 40), entry(5, 40)];
}

/** 📏️ The largest difference between two lists of numbers, relative beyond 1. */
function gap(left: readonly number[], right: readonly number[]): number {
  expect(left.length).toBe(right.length);
  let worst = 0;
  for (let index = 0; index < left.length; index++) worst = Math.max(worst, Math.abs(left[index]! - right[index]!) / Math.max(1, Math.abs(right[index]!)));
  return worst;
}

/** 🪞️ Whether two lists hold the same numbers bit for bit, a negative zero counting as zero. */
function same(left: readonly number[], right: readonly number[]): boolean {
  return left.length === right.length && left.every((value, index) => value === right[index]);
}

/** 🔗️ The skeleton gl-matrix solves: per bone `translate`, `rotate` (radians, the platform's sine and cosine) and `scale`, multiplied onto the parent's world matrix. */
function chained(species: Species, pose: Pose): number[] {
  const worlds = new Map<string, Matrix>();
  const numbers: number[] = [];
  species.bones.forEach((bone, index) => {
    const entry = pose[index] ?? REST;
    const local: Matrix = [1, 0, 0, 1, 0, 0];
    mat2d.translate(local, local, [bone.x + entry.x, bone.y + entry.y]);
    mat2d.rotate(local, local, (((bone.rotation ?? 0) + entry.rotation) * Math.PI) / 180);
    mat2d.scale(local, local, [entry.scaleX, entry.scaleY]);
    const parent = bone.parent === undefined ? undefined : worlds.get(bone.parent);
    const world: Matrix = [1, 0, 0, 1, 0, 0];
    if (parent === undefined) mat2d.copy(world, local);
    else mat2d.multiply(world, parent, local);
    worlds.set(bone.id, world);
    numbers.push(...world);
  });
  return numbers;
}

/** 🧱️ The skeleton built with `compose` from local matrices in the documented form `translate × rotate × scale`. */
function composed(species: Species, pose: Pose): number[] {
  const worlds = new Map<string, Affine>();
  const numbers: number[] = [];
  species.bones.forEach((bone, index) => {
    const entry = pose[index] ?? REST;
    const turns = ((bone.rotation ?? 0) + entry.rotation) / 360;
    const local: Affine = [cosTurns(turns) * entry.scaleX, sinTurns(turns) * entry.scaleX, (0 - sinTurns(turns)) * entry.scaleY, cosTurns(turns) * entry.scaleY, bone.x + entry.x, bone.y + entry.y];
    const parent = bone.parent === undefined ? undefined : worlds.get(bone.parent);
    const world = parent === undefined ? local : compose(parent, local);
    worlds.set(bone.id, world);
    numbers.push(...world);
  });
  return numbers;
}

describe("compose", () => {
  it("is gl-matrix's mat2d.multiply(parent, local), bit for bit", () => {
    expect([...IDENTITY]).toEqual([...mat2d.identity([0, 0, 0, 0, 0, 0])]);
    for (let index = 0; index < DRAWN; index++) {
      const parent = drawn(1, index);
      const local = drawn(2, index);
      expect(same(compose(parent, local), mat2d.multiply([0, 0, 0, 0, 0, 0], parent, local) as Matrix)).toBe(true);
    }
  });

  it("leaves a matrix unchanged next to the identity and is associative", () => {
    for (let index = 0; index < DRAWN / 5; index++) {
      const first = drawn(3, index);
      const second = drawn(4, index);
      const third = drawn(5, index);
      expect(same(compose(IDENTITY, first), first)).toBe(true);
      expect(same(compose(first, IDENTITY), first)).toBe(true);
      expect(gap(compose(compose(first, second), third), compose(first, compose(second, third)))).toBeLessThanOrEqual(BOUND);
    }
  });

  it("applies the local matrix first", () => {
    const moved = transform(compose([1, 0, 0, 1, 10, 0], [0, 1, -1, 0, 0, 0]), 1, 0);
    expect(moved).toEqual({ x: 10, y: 1 });
  });

  it("matches numpy's homogeneous product for every committed pair", () => {
    for (const vector of RIG.products) expect(gap(compose(vector.parent, vector.local), vector.expected), vector.id).toBeLessThanOrEqual(BOUND);
  });
});

describe("invert", () => {
  it("agrees with gl-matrix's mat2d.invert and undoes the matrix", () => {
    for (let index = 0; index < DRAWN; index++) {
      const matrix = drawn(6, index);
      const determinant = matrix[0] * matrix[3] - matrix[1] * matrix[2];
      if (Math.abs(determinant) < 0.05) continue;
      const inverse = invert(matrix);
      expect(gap(inverse, mat2d.invert([0, 0, 0, 0, 0, 0], matrix) as Matrix)).toBeLessThanOrEqual(1e-11);
      expect(gap(compose(matrix, inverse), IDENTITY)).toBeLessThanOrEqual(1e-11);
      expect(gap(compose(inverse, matrix), IDENTITY)).toBeLessThanOrEqual(1e-11);
    }
  });

  it("yields the identity for a matrix gl-matrix cannot invert either", () => {
    for (const matrix of [[0, 0, 0, 0, 3, 4], [1, 2, 2, 4, 5, 6], [2, 0.5, 0, 0, 3, 4], [0, 0, 1, 1, 0, 0]] as const) {
      expect(mat2d.invert([0, 0, 0, 0, 0, 0], matrix)).toBeNull();
      expect(invert(matrix)).toEqual(IDENTITY);
    }
  });

  it("inverts the identity, a shift and a quarter turn exactly", () => {
    expect(same(invert(IDENTITY), IDENTITY)).toBe(true);
    expect(same(invert([1, 0, 0, 1, 12.5, -16]), [1, 0, 0, 1, -12.5, 16])).toBe(true);
    expect(same(invert([0, 1, -1, 0, 3, 4]), [0, -1, 1, 0, -4, 3])).toBe(true);
    expect(same(invert([2, 0, 0, 4, 6, 8]), [0.5, 0, 0, 0.25, -3, -2])).toBe(true);
  });

  it("matches numpy.linalg.inv for every committed matrix, the identity where numpy finds it singular", () => {
    for (const vector of RIG.inverses) {
      expect(gap(invert(vector.matrix), vector.expected), vector.id).toBeLessThanOrEqual(1e-11);
      if (vector.singular) expect(invert(vector.matrix), vector.id).toEqual(IDENTITY);
    }
  });
});

describe("transform", () => {
  it("is gl-matrix's vec2.transformMat2d, bit for bit", () => {
    for (let index = 0; index < DRAWN; index++) {
      const matrix = drawn(7, index);
      const x = randomBetween([SEED, 8, index], -2000, 2000);
      const y = randomBetween([SEED, 9, index], -2000, 2000);
      const carried = transform(matrix, x, y);
      expect(same([carried.x, carried.y], vec2.transformMat2d([0, 0], [x, y], matrix) as [number, number])).toBe(true);
    }
  });

  it("carries the origin to the translation and returns through the inverse", () => {
    for (let index = 0; index < DRAWN / 5; index++) {
      const matrix = drawn(10, index);
      expect(transform(matrix, 0, 0)).toEqual({ x: matrix[4], y: matrix[5] });
      if (Math.abs(matrix[0] * matrix[3] - matrix[1] * matrix[2]) < 0.05) continue;
      const there = transform(matrix, 3.5, -7.25);
      const back = transform(invert(matrix), there.x, there.y);
      expect(gap([back.x, back.y], [3.5, -7.25])).toBeLessThanOrEqual(1e-10);
    }
  });

  it("matches numpy for every committed point", () => {
    for (const vector of RIG.points) {
      const carried = transform(vector.matrix, vector.x, vector.y);
      expect(gap([carried.x, carried.y], [vector.expected.x, vector.expected.y]), vector.id).toBeLessThanOrEqual(BOUND);
    }
  });
});

describe("restPose", () => {
  it("holds one rest entry per bone", () => {
    for (const species of RIG.species) {
      expect(restPose(species)).toEqual(species.bones.map(() => REST));
      expect(restPose(species).length).toBe(species.bones.length);
    }
    expect(restPose(rigOf([]))).toEqual([]);
  });

  it("matches the committed rest poses and their skeletons", () => {
    for (const vector of RIG.restPoses) {
      const species = speciesOf(vector.species);
      expect(restPose(species), vector.id).toEqual(vector.expected.pose);
      expect(gap(solveRig(species, restPose(species)), vector.expected.bones), vector.id).toBeLessThanOrEqual(BOUND);
    }
  });
});

describe("solveRig", () => {
  it("places the reference species at rest: the root on the feet, the body above it, limbs on the body", () => {
    const blobby = speciesOf("blobby");
    const bones = solveRig(blobby, restPose(blobby));
    const at = (id: string): number[] => bones.slice(blobby.bones.findIndex((bone) => bone.id === id) * 6).slice(0, 6);
    expect(bones.length).toBe(blobby.bones.length * 6);
    expect(at("root")).toEqual([1, 0, 0, 1, 0, 0]);
    expect(at("body")).toEqual([1, 0, 0, 1, 0, -16]);
    expect(at("leg-left")).toEqual([1, 0, 0, 1, -6, -7]);
    expect(at("tuft")).toEqual([1, 0, 0, 1, 0, -29]);
    expect(at("arm-left")).toEqual([cosTurns(20 / 360), sinTurns(20 / 360), 0 - sinTurns(20 / 360), cosTurns(20 / 360), -13, -16]);
  });

  it("multiplies with the expressions of compose, bit for bit", () => {
    for (const vector of RIG.skeletons) expect(same(solveRig(speciesOf(vector.species), vector.pose), composed(speciesOf(vector.species), vector.pose)), vector.id).toBe(true);
  });

  it("agrees with the chain gl-matrix translates, rotates, scales and multiplies", () => {
    for (const vector of RIG.skeletons) expect(gap(solveRig(speciesOf(vector.species), vector.pose), chained(speciesOf(vector.species), vector.pose)), vector.id).toBeLessThanOrEqual(BOUND);
    for (const species of RIG.species) {
      for (let index = 0; index < DRAWN / 10; index++) {
        const pose = species.bones.map((_, bone) => ({
          x: randomBetween([SEED, 11, index * 64 + bone], -8, 8),
          y: randomBetween([SEED, 12, index * 64 + bone], -8, 8),
          rotation: randomBetween([SEED, 13, index * 64 + bone], -720, 720),
          scaleX: randomBetween([SEED, 14, index * 64 + bone], -1.5, 1.5),
          scaleY: randomBetween([SEED, 15, index * 64 + bone], 0.5, 1.5),
        }));
        expect(gap(solveRig(species, pose), chained(species, pose))).toBeLessThanOrEqual(1e-11);
      }
    }
  });

  it("matches numpy's chain of homogeneous matrices for every committed pose", () => {
    expect(RIG.skeletons.length).toBeGreaterThan(15);
    for (const vector of RIG.skeletons) expect(gap(solveRig(speciesOf(vector.species), vector.pose), vector.expected), vector.id).toBeLessThanOrEqual(BOUND);
  });

  it("carries children along exactly when a parent makes a quarter turn or is scaled", () => {
    const blobby = speciesOf("blobby");
    const turned = solveRig(blobby, posed(blobby, { root: { rotation: 90 } }));
    expect(turned.slice(0, 6)).toEqual([0, 1, -1, 0, 0, 0]);
    expect(turned.slice(6, 12)).toEqual([0, 1, -1, 0, 16, 0]);
    const squashed = solveRig(blobby, posed(blobby, { root: { scaleX: 2, scaleY: 0.5 } }));
    expect(squashed.slice(6, 12)).toEqual([2, 0, 0, 0.5, 0, -8]);
    expect(squashed.slice(12, 18)).toEqual([2, 0, 0, 0.5, -12, -3.5]);
  });

  it("returns to rest after whole turns", () => {
    const blobby = speciesOf("blobby");
    const rest = solveRig(blobby, restPose(blobby));
    const spun = solveRig(blobby, posed(blobby, { root: { rotation: 360 }, body: { rotation: -720 }, "leg-left": { rotation: 1080 } }));
    expect(same(spun.slice(0, 18), rest.slice(0, 18))).toBe(true);
    expect(gap(spun, rest)).toBeLessThanOrEqual(BOUND);
  });

  it("leaves bones the pose does not cover at rest and treats an unlisted parent as none", () => {
    const blobby = speciesOf("blobby");
    const full = posed(blobby, { body: { rotation: 30, scaleY: 1.1 } });
    expect(solveRig(blobby, full.slice(0, 2))).toEqual(solveRig(blobby, full));
    expect(solveRig(blobby, [])).toEqual(solveRig(blobby, restPose(blobby)));
    expect(solveRig(rigOf([]), [])).toEqual([]);
    const orphan = rigOf([
      { id: "root", x: 1, y: 2 },
      { id: "stray", parent: "later", x: 5, y: 6 },
      { id: "later", parent: "root", x: 10, y: 20 },
    ]);
    expect(solveRig(orphan, [])).toEqual([1, 0, 0, 1, 1, 2, 1, 0, 0, 1, 5, 6, 1, 0, 0, 1, 11, 22]);
  });

  it("does not change the species or the pose", () => {
    const lanky = speciesOf("lanky");
    const pose = posed(lanky, { elbow: { rotation: 45 } });
    const before = JSON.stringify([lanky, pose]);
    solveRig(lanky, pose);
    expect(JSON.stringify([lanky, pose])).toBe(before);
  });
});

describe("lookOffset", () => {
  it("is (0, 0) when the eye and the target coincide", () => {
    expect(lookOffset({ x: 12.5, y: -3 }, { x: 12.5, y: -3 }, 40)).toEqual({ x: 0, y: 0 });
    expect(lookOffset({ x: 0, y: 0 }, { x: 0, y: 0 }, 0)).toEqual({ x: 0, y: 0 });
  });

  it("is half way out when the target is the reach away, exactly along the axes", () => {
    expect(lookOffset({ x: 0, y: 0 }, { x: 40, y: 0 }, 40)).toEqual({ x: 0.5, y: 0 });
    expect(lookOffset({ x: 0, y: 0 }, { x: 0, y: -40 }, 40)).toEqual({ x: 0, y: -0.5 });
    expect(lookOffset({ x: 10, y: 20 }, { x: 13, y: 24 }, 5)).toEqual({ x: 0.3, y: 0.4 });
  });

  it("points at the target with the length d ÷ (d + reach), inside the unit disc", () => {
    for (let index = 0; index < 4 * DRAWN; index++) {
      const eye = { x: randomBetween([SEED, 16, index], 0, 1920), y: randomBetween([SEED, 17, index], 0, 1080) };
      const target = { x: randomBetween([SEED, 18, index], -200, 2120), y: randomBetween([SEED, 19, index], -200, 1280) };
      const reach = randomBetween([SEED, 20, index], 1, 200);
      const offset = lookOffset(eye, target, reach);
      const distance = Math.sqrt((target.x - eye.x) * (target.x - eye.x) + (target.y - eye.y) * (target.y - eye.y));
      const length = Math.sqrt(offset.x * offset.x + offset.y * offset.y);
      expect(length).toBeLessThan(1);
      expect(Math.abs(length - distance / (distance + reach))).toBeLessThanOrEqual(BOUND);
      expect(Math.abs(offset.x * (target.y - eye.y) - offset.y * (target.x - eye.x))).toBeLessThanOrEqual(1e-9);
      expect(offset.x * (target.x - eye.x) + offset.y * (target.y - eye.y)).toBeGreaterThan(0);
    }
  });

  it("grows with the distance and reaches the rim only without a reach", () => {
    let last = 0;
    for (const distance of [0.001, 1, 10, 40, 100, 1000, 1e6, 1e12]) {
      const offset = lookOffset({ x: 5, y: 5 }, { x: 5 + distance, y: 5 }, 40);
      expect(offset.x).toBeGreaterThan(last);
      expect(offset.x).toBeLessThan(1);
      last = offset.x;
    }
    expect(lookOffset({ x: 3, y: 4 }, { x: 0, y: 0 }, 0)).toEqual({ x: -0.6, y: -0.8 });
    expect(lookOffset({ x: 3, y: 4 }, { x: 0, y: 0 }, -25)).toEqual({ x: -0.6, y: -0.8 });
  });

  it("matches numpy's norm for every committed pair and every committed eye on a posed bone", () => {
    for (const vector of GAZE.offsets) {
      const offset = lookOffset(vector.eye, vector.target, vector.reach);
      expect(gap([offset.x, offset.y], [vector.expected.x, vector.expected.y]), vector.id).toBeLessThanOrEqual(BOUND);
    }
    for (const vector of GAZE.eyes) {
      const eye = transform(vector.bone, vector.eye.x, vector.eye.y);
      const offset = lookOffset(eye, vector.target, vector.reach);
      expect(gap([eye.x, eye.y, offset.x, offset.y], [vector.expected.eye.x, vector.expected.eye.y, vector.expected.offset.x, vector.expected.offset.y]), vector.id).toBeLessThanOrEqual(BOUND);
    }
  });
});

describe("determinism", () => {
  it("reproduces the committed 64-bit pattern of every product, inverse, carried point, skeleton, offset and eye", () => {
    for (const vector of RIG.products) expect(compose(vector.parent, vector.local).map(bits), vector.id).toEqual(vector.bits);
    for (const vector of RIG.inverses) expect(invert(vector.matrix).map(bits), vector.id).toEqual(vector.bits);
    for (const vector of RIG.points) expect(pointBits(transform(vector.matrix, vector.x, vector.y)), vector.id).toEqual(vector.bits);
    for (const vector of RIG.skeletons) expect(solveRig(speciesOf(vector.species), vector.pose).map(bits), vector.id).toEqual(vector.bits);
    for (const vector of GAZE.offsets) expect(pointBits(lookOffset(vector.eye, vector.target, vector.reach)), vector.id).toEqual(vector.bits);
    for (const vector of GAZE.eyes) {
      const eye = transform(vector.bone, vector.eye.x, vector.eye.y);
      expect({ eye: pointBits(eye), offset: pointBits(lookOffset(eye, vector.target, vector.reach)) }, vector.id).toEqual(vector.bits);
    }
  });

  it("the module calls no platform transcendental, random source or clock", () => {
    const banned = ["sin", "cos", "tan", "atan2", "exp", "pow", "hypot", "log", "random"].map((name) => `Math.${name}(`).concat(["Date", "performance"]);
    for (const call of banned) expect(SOURCE.includes(call), call).toBe(false);
  });
});
