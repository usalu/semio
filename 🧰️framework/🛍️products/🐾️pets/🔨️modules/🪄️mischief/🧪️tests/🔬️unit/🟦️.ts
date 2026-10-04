/** 🪄️ Unit suite of the pets mischief: the topic match, the pick and the proof that nothing but the topic and a draw enters it, the gates, the station, the path of a lifted copy, the throw, and the ban on platform transcendentals.
 *
 * @see ../../🟦️.ts — the module under test
 * @see ../../../../🧫️fixtures/🪄️mischief-choice/🔣️.json — the answers of Python's strings and numpy (case 🪄️mischief-choice)
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import type { Perch, Pitch } from "../../../../🧬️schema/🟦️.ts";
import { randomUnit } from "../../../🎲️randomness/🟦️.ts";
import {
  LIFT_BRACE,
  LIFT_FADE,
  LIFT_GIVE,
  LIFT_HOLD,
  LIFT_LEAST,
  LIFT_LIMIT,
  LIFT_RETURN,
  LIFT_RETURNS,
  LIFT_RISE,
  LIFT_ROOM,
  LIFT_SHOVE,
  LIFT_TICKS,
  LIFT_TILT,
  LIFT_WOBBLE,
  type Lift,
  MISCHIEF_COOLDOWN_CALM,
  MISCHIEF_COOLDOWN_LIVELY,
  MISCHIEF_PATIENCE,
  MISCHIEF_PATIENCE_QUIET,
  MISCHIEF_WIDTH,
  NO_LIFT,
  type Circumstances,
  STATION_GAP,
  STATION_STEP,
  type Station,
  THROW_LIFT,
  THROW_SPEED,
  THROW_SPREAD,
  allowed,
  allowedFrom,
  chosenFixture,
  cooldownOf,
  fits,
  fixtureFor,
  liftAt,
  liftEnds,
  liftWake,
  patienceOf,
  stationFor,
  thrownOff,
} from "../../🟦️.ts";

type Box = { readonly id: string; readonly key: string; readonly x: number; readonly y: number; readonly width: number; readonly height: number };
type Described = Box & { readonly value: unknown; readonly correct: boolean; readonly answered: unknown };
type Path = { readonly dx: readonly number[]; readonly dy: readonly number[]; readonly tilt: readonly number[]; readonly opacity: readonly number[]; readonly ends: number };
type Vectors = {
  readonly matches: readonly { readonly id: string; readonly ground: string; readonly key: string; readonly expected: boolean }[];
  readonly candidates: readonly { readonly id: string; readonly grounds: readonly string[]; readonly fixtures: readonly Box[]; readonly expected: readonly string[] }[];
  readonly choices: readonly { readonly id: string; readonly count: number; readonly units: readonly number[]; readonly expected: readonly number[] }[];
  readonly leaks: readonly { readonly id: string; readonly grounds: readonly string[]; readonly unit: number; readonly items: readonly Described[]; readonly shuffles: readonly (readonly number[])[]; readonly expected: { readonly plain: string | null; readonly shuffled: readonly (string | null)[] } }[];
  readonly gates: readonly (Circumstances & { readonly id: string; readonly expected: { readonly allowed: boolean; readonly from: number | null } })[];
  readonly stations: readonly { readonly id: string; readonly fixture: Box; readonly pitches: readonly Pitch[]; readonly perches: readonly Perch[]; readonly width: number; readonly expected: Station | null }[];
  readonly lifts: readonly { readonly id: string; readonly since: number; readonly side: 1 | -1; readonly room: number; readonly span: number; readonly unit: number; readonly first: number; readonly last: number; readonly step: number; readonly expected: Path }[];
  readonly throws: readonly { readonly id: string; readonly pusher: { readonly x: number; readonly y: number }; readonly fixture: Box; readonly unit: number; readonly expected: { readonly vx: number; readonly vy: number } }[];
};

const VECTORS = JSON.parse(readFileSync(new URL("../../../../🧫️fixtures/🪄️mischief-choice/🔣️.json", import.meta.url), "utf8")) as Vectors;
const SOURCE = readFileSync(new URL("../../🟦️.ts", import.meta.url), "utf8");
const SEED = 20261003;
const QUIZZES = ["physics", "heating", "cooling", "demand"];
const TASKS = ["powers", "energies", "u-values", "heating-load", "heating-load-and-demand", "final-energy"];
const ITEMS = ["sun", "sunlight-on-earth", "kettle", "wall-geg", "wall", "kfw-40"];
const ROW: Box = { id: "row-2", key: "heating/heating-load-and-demand", x: 278.4, y: 221.2, width: 883.2, height: 38 };
const OPEN: Circumstances = { permitted: true, fine: true, width: 1440, mode: "calm", quiet: false, lifting: false, tick: 20000, stirred: 19000, rested: 8000 };

/** 🎰️ How many drawn cases a law is checked on at the level of the run. */
const CASES = sampled(200, 2000, 20000);

/** 🗝️ Every key of the vocabulary: quizzes, their tasks and their items. */
const KEYS = QUIZZES.flatMap((quiz) => [quiz, ...TASKS.flatMap((task) => [`${quiz}/${task}`, ...ITEMS.map((item) => `${quiz}/${task}/${item}`)])]);

/** 🔢️ A drawn whole number below `below`. */
function whole(stream: number, counter: number, below: number): number {
  return Math.floor(randomUnit([SEED, stream, counter]) * below);
}

/** 🕵️ A list whose elements report every property that is read from them. */
function watched<Item extends object>(items: readonly Item[], reads: string[]): Item[] {
  return items.map((item) => new Proxy(item, { get: (target, property, receiver) => (reads.push(String(property)), Reflect.get(target, property, receiver)) }));
}

/** 📄️ A drawn host description: items of the vocabulary with boxes, values, correctness flags and answers. */
function described(counter: number): Described[] {
  return Array.from({ length: 3 + whole(1, counter, 8) }, (_, place) => ({ id: `item-${place}`, key: KEYS[whole(2, counter * 16 + place, KEYS.length)]!, x: 150.4, y: 200 + 41.2 * place, width: 1139.2, height: 38, value: whole(3, counter * 16 + place, 1000), correct: whole(4, counter * 16 + place, 2) === 1, answered: whole(5, counter * 16 + place, 3) === 0 ? null : whole(6, counter * 16 + place, 1000) }));
}

/** 🔀️ The description with the values, the correctness flags and the answers of its items moved to other items by a drawn permutation; ids, keys and boxes stay. */
function shuffled(items: readonly Described[], counter: number): Described[] {
  const order = items.map((_, place) => place).sort((left, right) => randomUnit([SEED, 7, counter * 16 + left]) - randomUnit([SEED, 7, counter * 16 + right]));
  return items.map((item, place) => ({ ...item, value: items[order[place]!]!.value, correct: items[order[place]!]!.correct, answered: items[order[place]!]!.answered }));
}

describe("fits", () => {
  it("answers as Python's strings do for every committed ground and key", () => {
    expect(VECTORS.matches.length).toBeGreaterThan(20);
    for (const vector of VECTORS.matches) expect(fits(vector.ground, vector.key), vector.id).toBe(vector.expected);
  });

  it("covers a key that is the ground or continues it after a slash, over the whole vocabulary", () => {
    let covered = 0;
    const wrong: string[] = [];
    for (const ground of KEYS) {
      for (const key of KEYS) {
        const fitting = fits(ground, key);
        if (fitting !== (key === ground || key.startsWith(`${ground}/`)) || fitting !== (key.split("/").slice(0, ground.split("/").length).join("/") === ground)) wrong.push(`${ground} ← ${key}`);
        covered += fitting ? 1 : 0;
      }
    }
    expect(wrong).toEqual([]);
    expect(covered).toBeGreaterThan(KEYS.length);
    expect(fits("heating/heating-load", "heating/heating-load-and-demand")).toBe(false);
    expect(fits("heating/u-values/wall", "heating/u-values/wall-geg")).toBe(false);
    expect([fits("", "heating"), fits("", ""), fits("heating", "")]).toEqual([false, false, false]);
  });
});

describe("fixtureFor", () => {
  it("keeps the fitting fixtures of every committed species in the survey's order", () => {
    expect(VECTORS.candidates.length).toBeGreaterThan(10);
    for (const vector of VECTORS.candidates) expect(fixtureFor(vector.grounds, vector.fixtures).map((fixture) => fixture.id), vector.id).toEqual(vector.expected);
  });

  it("returns the fixtures themselves, each once, whatever the order of the grounds", () => {
    const fixtures = VECTORS.candidates.find((vector) => vector.id === "cloudy-everywhere")!.fixtures;
    const grounds = ["physics", "cooling", "physics/powers/sun", "physics"];
    const fitting = fixtureFor(grounds, fixtures);
    expect(fitting.length).toBeGreaterThan(5);
    expect(new Set(fitting).size).toBe(fitting.length);
    for (const fixture of fitting) expect(fixtures.includes(fixture)).toBe(true);
    expect(fixtureFor([...grounds].reverse(), fixtures)).toEqual(fitting);
    expect(fixtureFor(grounds, [...fixtures].reverse())).toEqual([...fitting].reverse());
  });

  it("reads nothing of a fixture but its key", () => {
    const reads: string[] = [];
    const items = described(1);
    fixtureFor(["physics", "heating/u-values", "cooling/powers/sun"], watched(items, reads));
    expect(reads.length).toBe(items.length);
    expect(new Set(reads)).toEqual(new Set(["key"]));
  });
});

describe("chosenFixture", () => {
  it("picks numpy's position for every committed count and unit", () => {
    for (const vector of VECTORS.choices) {
      const candidates = Array.from({ length: vector.count }, (_, position) => position);
      expect(vector.units.map((unit) => chosenFixture(candidates, unit) ?? -1), vector.id).toEqual(vector.expected);
    }
  });

  it("answers null without candidates and stays inside the list for any unit", () => {
    expect(chosenFixture([], 0.5)).toBeNull();
    expect([chosenFixture(["a", "b"], Number.NaN), chosenFixture(["a", "b"], Number.NEGATIVE_INFINITY), chosenFixture(["a", "b"], Number.POSITIVE_INFINITY)]).toEqual(["a", "a", "b"]);
    expect([chosenFixture(["a", "b", "c"], -3), chosenFixture(["a", "b", "c"], 0), chosenFixture(["a", "b", "c"], 0.34), chosenFixture(["a", "b", "c"], 1 - 2 ** -32), chosenFixture(["a", "b", "c"], 1), chosenFixture(["a", "b", "c"], 99)]).toEqual(["a", "a", "b", "c", "c", "c"]);
  });

  it("gives every candidate the same chance", () => {
    const counts = [0, 0, 0, 0, 0, 0, 0];
    const draws = CASES * 10;
    for (let counter = 0; counter < draws; counter++) counts[chosenFixture([0, 1, 2, 3, 4, 5, 6], randomUnit([SEED, 8, counter]))!]! += 1;
    for (const count of counts) expect(Math.abs(count / draws - 1 / 7)).toBeLessThan(0.35 / Math.sqrt(draws) + 0.004);
  });

  it("has two inputs, the candidates and the draw, and reads no property of any candidate", () => {
    expect(chosenFixture.length).toBe(2);
    expect(fixtureFor.length).toBe(2);
    const reads: string[] = [];
    const items = watched(described(2), reads);
    for (let counter = 0; counter < 50; counter++) expect(items.includes(chosenFixture(items, randomUnit([SEED, 9, counter]))!)).toBe(true);
    expect(reads).toEqual([]);
  });

  it("names the committed fixture for every committed host description, however its values, flags and answers are permuted", () => {
    expect(VECTORS.leaks.length).toBeGreaterThan(6);
    for (const vector of VECTORS.leaks) {
      expect(chosenFixture(fixtureFor(vector.grounds, vector.items), vector.unit)?.id ?? null, vector.id).toBe(vector.expected.plain);
      vector.shuffles.forEach((shuffle, index) => {
        const permuted = vector.items.map((item, place) => ({ ...item, value: vector.items[shuffle[place]!]!.value, correct: vector.items[shuffle[place]!]!.correct, answered: vector.items[shuffle[place]!]!.answered }));
        expect(chosenFixture(fixtureFor(vector.grounds, permuted), vector.unit)?.id ?? null, `${vector.id} shuffle ${index}`).toBe(vector.expected.shuffled[index]);
        expect(vector.expected.shuffled[index], `${vector.id} shuffle ${index}`).toBe(vector.expected.plain);
      });
    }
  });

  it("never follows a value, a correctness flag or an answer in any drawn host description, where a pick that does is found out", () => {
    let told = 0;
    let picked = 0;
    for (let counter = 0; counter < CASES; counter++) {
      const items = described(counter);
      const grounds = [QUIZZES[whole(10, counter, 4)]!, `${QUIZZES[whole(11, counter, 4)]!}/${TASKS[whole(12, counter, 6)]!}`];
      const unit = randomUnit([SEED, 13, counter]);
      const plain = chosenFixture(fixtureFor(grounds, items), unit);
      const leaky = fixtureFor(grounds, items).find((item) => item.correct) ?? plain;
      picked += plain === null ? 0 : 1;
      for (let round = 0; round < 4; round++) {
        const permuted = shuffled(items, counter * 4 + round);
        expect(chosenFixture(fixtureFor(grounds, permuted), unit)?.id ?? null).toBe(plain?.id ?? null);
        told += (fixtureFor(grounds, permuted).find((item) => item.correct) ?? chosenFixture(fixtureFor(grounds, permuted), unit))?.id === leaky?.id ? 0 : 1;
      }
    }
    expect(picked).toBeGreaterThan(CASES / 4);
    expect(told).toBeGreaterThan(CASES / 20);
  });
});

describe("allowed", () => {
  it("gives numpy's verdict and tick for every committed occasion", () => {
    expect(VECTORS.gates.length).toBeGreaterThan(60);
    for (const vector of VECTORS.gates) expect({ allowed: allowed(vector), from: allowedFrom(vector) }, vector.id).toEqual(vector.expected);
  });

  it("is shut by every gate alone", () => {
    expect(allowed(OPEN)).toBe(true);
    for (const change of [{ permitted: false }, { fine: false }, { lifting: true }, { mode: "still" }, { width: MISCHIEF_WIDTH - 1 }, { width: Number.NaN }, { stirred: OPEN.tick - MISCHIEF_PATIENCE + 1 }, { rested: OPEN.tick - MISCHIEF_COOLDOWN_CALM + 1 }] as Partial<Circumstances>[]) expect(allowed({ ...OPEN, ...change }), JSON.stringify(change)).toBe(false);
    for (const change of [{ width: MISCHIEF_WIDTH }, { stirred: OPEN.tick - MISCHIEF_PATIENCE }, { rested: OPEN.tick - MISCHIEF_COOLDOWN_CALM }, { mode: "lively", rested: OPEN.tick - MISCHIEF_COOLDOWN_LIVELY }] as Partial<Circumstances>[]) expect(allowed({ ...OPEN, ...change }), JSON.stringify(change)).toBe(true);
  });

  it("waits 12 seconds after the learner stirred, 30 in a time of concentration, 3 minutes between lifts when calm and 45 seconds when lively", () => {
    expect([MISCHIEF_WIDTH, MISCHIEF_PATIENCE / 64, MISCHIEF_PATIENCE_QUIET / 64, MISCHIEF_COOLDOWN_CALM / 64, MISCHIEF_COOLDOWN_LIVELY / 64]).toEqual([1024, 12, 30, 180, 45]);
    expect([patienceOf(false), patienceOf(true), cooldownOf("calm"), cooldownOf("lively"), cooldownOf("still")]).toEqual([768, 1920, 11520, 2880, null]);
    expect(allowed({ ...OPEN, quiet: true, stirred: OPEN.tick - MISCHIEF_PATIENCE_QUIET + 1 })).toBe(false);
    expect(allowed({ ...OPEN, quiet: true, stirred: OPEN.tick - MISCHIEF_PATIENCE_QUIET })).toBe(true);
    expect(allowed({ ...OPEN, mode: "lively", rested: OPEN.tick - MISCHIEF_COOLDOWN_LIVELY + 1 })).toBe(false);
  });

  it("opens exactly at the tick allowedFrom names and stays open", () => {
    for (let counter = 0; counter < CASES; counter++) {
      const occasion: Circumstances = { ...OPEN, mode: whole(14, counter, 2) === 0 ? "calm" : "lively", quiet: whole(15, counter, 2) === 0, stirred: whole(16, counter, 30000), rested: whole(17, counter, 30000) };
      const from = allowedFrom(occasion)!;
      expect(from).toBe(Math.max(occasion.stirred + patienceOf(occasion.quiet), occasion.rested + cooldownOf(occasion.mode)!));
      expect([allowed({ ...occasion, tick: from - 1 }), allowed({ ...occasion, tick: from }), allowed({ ...occasion, tick: from + 100000 })]).toEqual([false, true, true]);
    }
    for (const change of [{ permitted: false }, { fine: false }, { lifting: true }, { mode: "still" }, { width: 800 }] as Partial<Circumstances>[]) expect(allowedFrom({ ...OPEN, ...change })).toBeNull();
  });
});

describe("stationFor", () => {
  const leftWall: Pitch = { wall: "card:left", surface: "card", side: -1, x: 272, y0: 150, y1: 420 };
  const rightWall: Pitch = { wall: "card:right", surface: "card", side: 1, x: 1168, y0: 150, y1: 420 };

  it("finds numpy's station for every committed fixture", () => {
    expect(VECTORS.stations.length).toBeGreaterThan(25);
    for (const vector of VECTORS.stations) {
      const station = stationFor(vector.fixture, vector.pitches, vector.perches, vector.width);
      if (vector.expected === null) expect(station, vector.id).toBeNull();
      else {
        expect({ ...station, y: 0, room: 0 }, vector.id).toEqual({ ...vector.expected, y: 0, room: 0 });
        expect(station!.y, vector.id).toBeCloseTo(vector.expected.y, 9);
        expect(station!.room, vector.id).toBeCloseTo(vector.expected.room, 9);
      }
    }
  });

  it("puts a pusher on the left wall to shove right, with the room beyond the far end, its feet at the lower edge", () => {
    expect(stationFor(ROW, [leftWall], [], 1440)).toEqual({ footing: "wall", wall: "card:left", surface: "card", x: 272, y: ROW.y + ROW.height, side: 1, room: 1440 - (ROW.x + ROW.width) });
    expect(stationFor(ROW, [rightWall], [], 1440)).toEqual({ footing: "wall", wall: "card:right", surface: "card", x: 1168, y: ROW.y + ROW.height, side: -1, room: ROW.x });
    expect(stationFor(ROW, [{ ...leftWall, y1: 240 }], [], 1440)!.y).toBe(240);
  });

  it("takes the side with room, the earlier station when both have the same, a perch before a wall", () => {
    expect(stationFor(ROW, [leftWall, rightWall], [], 1600)!.side).toBe(1);
    expect(stationFor(ROW, [rightWall, leftWall], [], 1600)!.side).toBe(1);
    expect(stationFor(ROW, [leftWall, rightWall], [], 1300)!.side).toBe(-1);
    expect(stationFor({ ...ROW, x: 280, width: 880 }, [{ ...leftWall, x: 276 }, { ...rightWall, x: 1164 }], [], 1440)!.wall).toBe("card:left");
    expect(stationFor({ ...ROW, x: 280, width: 880 }, [{ ...rightWall, x: 1164 }, { ...leftWall, x: 276 }], [], 1440)!.wall).toBe("card:right");
    expect(stationFor(ROW, [leftWall], [{ surface: "shelf", x0: 100, x1: 270, y: 259.2 }], 1440)).toEqual({ footing: "perch", wall: null, surface: "shelf", x: 270, y: 259.2, side: 1, room: 1440 - (ROW.x + ROW.width) });
  });

  it("finds nothing where nothing is beside the fixture or no shove has room", () => {
    expect(stationFor(ROW, [], [], 1440)).toBeNull();
    expect(stationFor(ROW, [{ ...leftWall, x: ROW.x - STATION_GAP - 1 }], [], 1440)).toBeNull();
    expect(stationFor(ROW, [{ ...leftWall, y1: ROW.y }], [], 1440)).toBeNull();
    expect(stationFor(ROW, [{ ...leftWall, y0: ROW.y + ROW.height }], [], 1440)).toBeNull();
    expect(stationFor(ROW, [{ ...leftWall, side: 1 }], [], 1440)).toBeNull();
    expect(stationFor(ROW, [leftWall], [], ROW.x + ROW.width + LIFT_ROOM - 1)).toBeNull();
    expect(stationFor(ROW, [], [{ surface: "shelf", x0: 100, x1: 270, y: ROW.y + ROW.height + STATION_STEP + 1 }], 1440)).toBeNull();
    expect(stationFor(ROW, [], [{ surface: "shelf", x0: 100, x1: 270, y: ROW.y }], 1440)).toBeNull();
  });

  it("always names a station on a stretch it was given, beside the fixture and with room", () => {
    let found = 0;
    for (let counter = 0; counter < CASES; counter++) {
      const fixture: Box = { id: "row", key: "heating", x: 200 + whole(18, counter, 200), y: 100 + whole(19, counter, 300), width: 400 + whole(20, counter, 500), height: 20 + whole(21, counter, 60) };
      const pitches: Pitch[] = Array.from({ length: whole(22, counter, 4) }, (_, index) => ({ wall: `wall-${index}`, surface: "card", side: whole(23, counter * 8 + index, 2) === 0 ? -1 : 1, x: whole(23, counter * 8 + index, 2) === 0 ? fixture.x - whole(24, counter * 8 + index, 40) + 4 : fixture.x + fixture.width + whole(24, counter * 8 + index, 40) - 4, y0: whole(25, counter * 8 + index, 300), y1: 300 + whole(26, counter * 8 + index, 300) }));
      const perches: Perch[] = Array.from({ length: whole(27, counter, 3) }, (_, index) => ({ surface: `shelf-${index}`, x0: whole(28, counter * 8 + index, 300), x1: 300 + whole(29, counter * 8 + index, 1000), y: fixture.y + whole(30, counter * 8 + index, 120) - 20 }));
      const width = 1024 + whole(31, counter, 600);
      const station = stationFor(fixture, pitches, perches, width);
      if (station === null) continue;
      found += 1;
      expect(station.room).toBeGreaterThanOrEqual(LIFT_ROOM);
      expect(station.room).toBe(station.side === 1 ? width - (fixture.x + fixture.width) : fixture.x);
      expect(station.side === 1 ? station.x <= fixture.x + 2 : station.x >= fixture.x + fixture.width - 2).toBe(true);
      expect(station.y > fixture.y && station.y <= fixture.y + fixture.height + STATION_STEP).toBe(true);
      if (station.footing === "wall") expect(pitches.some((pitch) => pitch.wall === station.wall && pitch.x === station.x && station.y >= pitch.y0 && station.y <= pitch.y1)).toBe(true);
      else expect(perches.some((perch) => perch.surface === station.surface && perch.y === station.y && station.x >= perch.x0 && station.x <= perch.x1)).toBe(true);
    }
    expect(found).toBeGreaterThan(CASES / 10);
  });
});

describe("liftAt", () => {
  const at = (age: number, side: 1 | -1 = 1, room = 40, span = 883.2, unit = 0.5): Lift => liftAt(1000, 1000 + age, side, room, span, unit);

  it("follows numpy's branch-free path at every committed tick", () => {
    expect(VECTORS.lifts.length).toBeGreaterThan(5);
    for (const vector of VECTORS.lifts) {
      expect(liftEnds(vector.since), vector.id).toBe(vector.expected.ends);
      const wrong: string[] = [];
      let place = 0;
      for (let tick = vector.first; tick <= vector.last; tick += vector.step) {
        const lift = liftAt(vector.since, tick, vector.side, vector.room, vector.span, vector.unit);
        for (const field of ["dx", "dy", "tilt", "opacity"] as const) if (!(Math.abs(lift[field] - vector.expected[field][place]!) <= 1e-9)) wrong.push(`${field} at ${tick}: ${lift[field]} instead of ${vector.expected[field][place]}`);
        place += 1;
      }
      expect(wrong, vector.id).toEqual([]);
      expect(place).toBe(vector.expected.dx.length);
    }
  });

  it("lasts 688 ticks, well inside the 20 seconds a lift may take, and is nothing before and after", () => {
    expect([LIFT_BRACE, LIFT_SHOVE, LIFT_WOBBLE, LIFT_HOLD, LIFT_RETURN, LIFT_FADE, LIFT_RETURNS, LIFT_TICKS, LIFT_LIMIT]).toEqual([24, 40, 48, 512, 56, 8, 624, 688, 1280]);
    expect(LIFT_TICKS).toBeLessThanOrEqual(LIFT_LIMIT);
    expect(liftEnds(1000)).toBe(1688);
    for (const age of [-100, -1, LIFT_TICKS, LIFT_TICKS + 1, 5000]) expect(at(age)).toBe(NO_LIFT);
    expect(NO_LIFT).toEqual({ dx: 0, dy: 0, tilt: 0, opacity: 0 });
    expect(at(LIFT_TICKS - 1).opacity).toBeGreaterThan(0);
  });

  it("appears and vanishes lying exactly on its element", () => {
    for (let age = 0; age <= LIFT_FADE; age++) expect(at(age)).toEqual({ dx: 0, dy: 0, tilt: 0, opacity: age === LIFT_FADE ? 1 : (age / LIFT_FADE) ** 2 * (3 - (2 * age) / LIFT_FADE) });
    for (let age = LIFT_TICKS - LIFT_FADE; age < LIFT_TICKS; age++) {
      const lift = at(age);
      expect([lift.dx, lift.dy, lift.tilt]).toEqual([0, 0, 0]);
      expect(lift.opacity).toBeCloseTo(1 - ((age - LIFT_TICKS + LIFT_FADE) / LIFT_FADE) ** 2 * (3 - (2 * (age - LIFT_TICKS + LIFT_FADE)) / LIFT_FADE), 12);
    }
    for (let age = LIFT_FADE; age <= LIFT_TICKS - LIFT_FADE; age++) expect(at(age).opacity).toBe(1);
  });

  it("gives towards the pusher, slides out, wobbles, rests and slides home", () => {
    const travel = 40 * (LIFT_LEAST + (1 - LIFT_LEAST) * 0.5);
    for (let age = LIFT_FADE + 1; age < LIFT_BRACE; age++) expect(at(age).dx < 0 && at(age).dx >= -LIFT_GIVE).toBe(true);
    for (let age = LIFT_BRACE + 1; age < LIFT_BRACE + LIFT_SHOVE; age++) expect(at(age).dx).toBeGreaterThan(at(age - 1).dx);
    for (let age = LIFT_BRACE + LIFT_SHOVE; age < LIFT_BRACE + LIFT_SHOVE + LIFT_WOBBLE; age++) expect(Math.abs(at(age).dx - travel)).toBeLessThanOrEqual(LIFT_GIVE);
    for (let age = LIFT_BRACE + LIFT_SHOVE + LIFT_WOBBLE; age <= LIFT_RETURNS; age++) expect(at(age)).toEqual({ dx: travel, dy: 0, tilt: 0, opacity: 1 });
    for (let age = LIFT_RETURNS + 1; age <= LIFT_RETURNS + LIFT_RETURN; age++) expect(at(age).dx).toBeLessThan(at(age - 1).dx);
    expect(at(LIFT_BRACE).dx).toBe(0);
    expect(at(LIFT_RETURNS + LIFT_RETURN).dx).toBe(0);
  });

  it("travels between six tenths and all of the granted room, by the draw alone", () => {
    for (const [room, unit, travel] of [[40, 0, 24], [40, 1, 40], [56, 0.5, 44.8], [0, 0.7, 0], [-9, 0.7, 0]] as const) expect(at(LIFT_RETURNS - 1, 1, room, 300, unit).dx).toBeCloseTo(travel, 12);
    expect(at(300, 1, 40, 300, 0.25).dx).toBe(at(300, 1, 40, 9000, 0.25).dx);
  });

  it("never jumps, never leaves its row by more than 2 px and never tilts by more than 0.004 turns, less the wider it is", () => {
    for (let counter = 0; counter < sampled(6, 40, 400); counter++) {
      const side = counter % 2 === 0 ? 1 : -1;
      const room = whole(32, counter, 60);
      const span = 20 + whole(33, counter, 1200);
      const unit = randomUnit([SEED, 34, counter]);
      const lean = Math.min(LIFT_TILT, LIFT_RISE / (Math.PI * span));
      const wrong: string[] = [];
      let previous = liftAt(0, -1, side, room, span, unit);
      for (let tick = 0; tick <= LIFT_TICKS; tick++) {
        const lift = liftAt(0, tick, side, room, span, unit);
        if (Math.abs(lift.dx - previous.dx) > 0.6 + (1.5 * room) / LIFT_SHOVE) wrong.push(`dx jumps at ${tick}`);
        if (Math.abs(lift.dy - previous.dy) > 0.55) wrong.push(`dy jumps at ${tick}`);
        if (Math.abs(lift.tilt - previous.tilt) > 0.0012) wrong.push(`tilt jumps at ${tick}`);
        if (Math.abs(lift.opacity - previous.opacity) > 0.2) wrong.push(`opacity jumps at ${tick}`);
        if (Math.abs(lift.dy) > LIFT_RISE) wrong.push(`dy ${lift.dy} at ${tick}`);
        if (Math.abs(lift.tilt) > lean + 1e-15 || Math.sin(Math.abs(lift.tilt) * 2 * Math.PI) * (span / 2) > LIFT_RISE + 1e-9) wrong.push(`tilt ${lift.tilt} at ${tick}`);
        if (!(lift.dx * side >= -LIFT_GIVE && lift.dx * side <= room + LIFT_GIVE)) wrong.push(`dx ${lift.dx} at ${tick}`);
        if (!(lift.opacity >= 0 && lift.opacity <= 1)) wrong.push(`opacity ${lift.opacity} at ${tick}`);
        previous = lift;
      }
      expect(wrong, `side ${side}, room ${room}, span ${span}, unit ${unit}`).toEqual([]);
    }
  });

  it("mirrors a shove to the left", () => {
    for (let age = -2; age < LIFT_TICKS + 2; age++) {
      const right = at(age, 1, 37, 410, 0.3);
      const left = at(age, -1, 37, 410, 0.3);
      expect([left.dx + right.dx, left.tilt + right.tilt, left.dy - right.dy, left.opacity - right.opacity]).toEqual([0, 0, 0, 0]);
    }
  });

  it("wakes the stage for every tick the copy changes and lets it sleep while the copy rests", () => {
    let tick = 990;
    let wakes = 0;
    for (let next = liftWake(1000, tick); next !== null; next = liftWake(1000, tick)) {
      expect(next).toBeGreaterThan(tick);
      for (let between = tick + 1; between < next; between++) expect(liftAt(1000, between, 1, 40, 883.2, 0.5)).toEqual(liftAt(1000, tick, 1, 40, 883.2, 0.5));
      tick = next;
      wakes += 1;
    }
    expect(tick).toBe(liftEnds(1000));
    expect(wakes).toBe(1 + LIFT_TICKS - (LIFT_HOLD - 1));
    expect([liftWake(1000, 1000 + LIFT_BRACE + LIFT_SHOVE + LIFT_WOBBLE), liftWake(1000, 1000 + LIFT_RETURNS - 1), liftWake(1000, 1000 + LIFT_RETURNS), liftWake(1000, 2000)]).toEqual([1000 + LIFT_RETURNS, 1000 + LIFT_RETURNS, 1001 + LIFT_RETURNS, null]);
  });
});

describe("thrownOff", () => {
  it("throws as numpy says for every committed pusher", () => {
    for (const vector of VECTORS.throws) {
      const toss = thrownOff(vector.pusher, vector.fixture, vector.unit);
      expect(toss.vx, vector.id).toBeCloseTo(vector.expected.vx, 9);
      expect(toss.vy, vector.id).toBe(vector.expected.vy);
    }
  });

  it("throws away from the middle of the fixture, upwards, at 120 to 200 pixels per second", () => {
    const middle = ROW.x + ROW.width / 2;
    for (let counter = 0; counter < CASES; counter++) {
      const x = whole(35, counter, 1440);
      const unit = randomUnit([SEED, 36, counter]);
      const toss = thrownOff({ x, y: 259.2 }, ROW, unit);
      expect(Math.sign(toss.vx)).toBe(x < middle ? -1 : 1);
      expect(Math.abs(toss.vx)).toBe(THROW_SPEED + THROW_SPREAD * unit);
      expect(Math.abs(toss.vx) >= 120 && Math.abs(toss.vx) < 200).toBe(true);
      expect(toss.vy).toBe(-THROW_LIFT);
    }
    expect(thrownOff({ x: middle, y: 0 }, ROW, 0)).toEqual({ vx: 120, vy: -220 });
  });
});

describe("determinism", () => {
  it("the module calls no platform transcendental, random source or clock", () => {
    const banned = ["sin", "cos", "tan", "atan2", "exp", "pow", "hypot", "log", "random"].map((name) => `Math.${name}(`).concat(["Date", "performance", "console"]);
    for (const call of banned) expect(SOURCE.includes(call), call).toBe(false);
  });
});
