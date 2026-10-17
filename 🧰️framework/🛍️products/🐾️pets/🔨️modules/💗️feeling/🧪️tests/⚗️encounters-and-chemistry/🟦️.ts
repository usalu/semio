/** ⚗️ Unit suite of the feeling module, encounters and chemistry: where two bodies are to each other, the chemistry of a menagerie and the arithmetic of the module. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { type Cooling, type Menagerie, type Reaction, type Slug } from "../../../../🧬️schema/🟦️.ts";
import { type Chemistry, type Consequence, EFFECT_AMOUNT, MOOD_REST, type Sighting, type Trial, drawsOf, everyTicks, heldTicks, matches, nearby, reactionsOf, seen, trialsOf } from "../../🟦️.ts";
import { agree, frozen, HERE, MENAGERIE, SET, sight, STAGES, units } from "../🧰️feeling-test-kit/🟦️.ts";

describe("where two bodies are to each other", () => {
  const box = (x: number, y: number, width: number, height: number): Sighting => ({ species: "glow", state: "bright", held: 0, mood: "content", intensity: MOOD_REST, activity: "idle", trick: null, x, y, width, height });

  it("measures the gap between the boxes, across and upright together", () => {
    const pet = box(100, 300, 40, 30);
    expect(nearby(pet, box(140, 300, 40, 30), 0)).toBe(true);
    expect(nearby(pet, box(150, 300, 40, 30), 9.5)).toBe(false);
    expect(nearby(pet, box(150, 300, 40, 30), 10)).toBe(true);
    expect(nearby(pet, box(100, 230, 40, 30), 39.5)).toBe(false);
    expect(nearby(pet, box(100, 230, 40, 30), 40)).toBe(true);
    expect(nearby(pet, box(170, 230, 40, 30), 49.5)).toBe(false);
    expect(nearby(pet, box(170, 230, 40, 30), 50)).toBe(true);
  });

  it("calls the first above or below the second when they share a column, beside it when they share only a row", () => {
    const pet = box(100, 300, 40, 30);
    const over = box(110, 240, 40, 30);
    expect([seen(over, pet, "above"), seen(over, pet, "below"), seen(over, pet, "beside"), seen(over, pet, "any")]).toEqual([true, false, false, true]);
    expect([seen(pet, over, "above"), seen(pet, over, "below"), seen(pet, over, "beside")]).toEqual([false, true, false]);
    const neighbour = box(150, 295, 40, 30);
    expect([seen(neighbour, pet, "above"), seen(neighbour, pet, "below"), seen(neighbour, pet, "beside")]).toEqual([false, false, true]);
    const diagonal = box(170, 240, 40, 30);
    expect([seen(diagonal, pet, "above"), seen(diagonal, pet, "below"), seen(diagonal, pet, "beside"), seen(diagonal, pet, "any")]).toEqual([false, false, false, true]);
  });

  it("is symmetric where it should be: near and beside both ways, above one way and below the other", () => {
    const marks = units(7, STAGES * 8);
    for (let index = 0; index < STAGES; index++) {
      const place = (offset: number): Sighting => box(Math.floor(marks[offset]! * 400) / 2, Math.floor(marks[offset + 1]! * 400) / 2, 10 + Math.floor(marks[offset + 2]! * 60), 10 + Math.floor(marks[offset + 3]! * 60));
      const first = place(index * 8);
      const second = place(index * 8 + 4);
      for (const reach of [0, 10, 50, 200]) expect(nearby(first, second, reach)).toBe(nearby(second, first, reach));
      expect(seen(first, second, "beside")).toBe(seen(second, first, "beside"));
      expect(seen(first, second, "above")).toBe(seen(second, first, "below"));
      expect(seen(first, second, "above") && seen(first, second, "below")).toBe(false);
      expect(seen(first, second, "beside") && (seen(first, second, "above") || seen(first, second, "below"))).toBe(false);
    }
  });

  it("answers the committed pairs", () => {
    for (const vector of SET.relations) agree({ nearby: vector.reaches.map((reach) => nearby(vector.first, vector.second, reach)), above: seen(vector.first, vector.second, "above"), below: seen(vector.first, vector.second, "below"), beside: seen(vector.first, vector.second, "beside"), any: seen(vector.first, vector.second, "any") }, vector.expected, vector.id);
  });
});

describe("the chemistry of a menagerie", () => {
  const eclipse = (): Sighting[] => [sight("mist", "heavy", 100, { y: 190 }), sight("glow", "bright", 100)];
  const happened = (chemistry: Chemistry): Slug[] => chemistry.consequences.map((consequence) => consequence.reaction);
  const odd = (id: Slug, then: Reaction["then"]): Reaction => ({ id, when: { species: "leaf" }, near: { species: "wick" }, within: 100, every: 1, then });

  it("asks an actor for the species, the state and how long it has held it, the activity, the trick and the mood it shows", () => {
    const actor = sight("glow", "dim", 0, { held: 320, mood: "grumpy", intensity: 0.5, activity: "trick", trick: "flare" });
    expect(matches({ species: "glow" }, actor)).toBe(true);
    expect(matches({}, actor)).toBe(true);
    expect(matches({ state: "dim", mood: "grumpy" }, actor)).toBe(true);
    expect(matches({ species: "mist" }, actor)).toBe(false);
    expect(matches({ species: "glow", state: "dim", mood: "grumpy", activity: "trick", trick: "flare" }, actor)).toBe(true);
    expect(matches({ species: "glow", trick: "spark" }, actor)).toBe(false);
    expect(matches({ species: "glow", trick: "flare" }, { ...actor, activity: "idle", trick: null })).toBe(false);
    expect(matches({ species: "glow", state: "bright" }, actor)).toBe(false);
    expect(matches({ species: "glow", activity: "idle" }, actor)).toBe(false);
    expect(matches({ species: "glow", mood: "content" }, actor)).toBe(false);
    expect(matches({ species: "glow", mood: "grumpy" }, { ...actor, intensity: MOOD_REST })).toBe(false);
    expect(matches({ species: "glow", mood: "content" }, { ...actor, intensity: MOOD_REST })).toBe(true);
    expect(matches({ state: "dim", held: 5 }, actor)).toBe(true);
    expect(matches({ state: "dim", held: 5 }, { ...actor, held: 319 })).toBe(false);
    expect([heldTicks(5), heldTicks(0.001), heldTicks(1.5)]).toEqual([320, 1, 96]);
  });

  it("finds what is due in pair order and applies it: a state, moods with their amounts, a shift of the rapport", () => {
    const beat = reactionsOf(MENAGERIE, eclipse(), 0, [], [], []);
    expect(beat.consequences).toEqual<Consequence[]>([
      { reaction: "eclipse", on: "glow", other: "mist", state: "dim", mood: "grumpy", amount: 0.5, rapport: 0, encounter: null, trick: null, activity: null },
      { reaction: "eclipse", on: "mist", other: "glow", state: null, mood: "playful", amount: EFFECT_AMOUNT, rapport: 0, encounter: null, trick: null, activity: null },
      { reaction: "eclipse", on: "mist", other: "glow", state: null, mood: null, amount: 0, rapport: -0.05, encounter: null, trick: null, activity: null },
    ]);
    expect(beat.coolings).toEqual<Cooling[]>([{ reaction: "eclipse", when: "mist", near: "glow", until: 1280 }]);
    expect(beat.drawn).toBe(0);
    expect(everyTicks(MENAGERIE.chemistry[0]!)).toBe(1280);
  });

  it("lets a reaction cool for its pair, and tries it again when its time is up", () => {
    const first = reactionsOf(MENAGERIE, eclipse(), 0, [], [], []);
    expect(reactionsOf(MENAGERIE, eclipse(), 1279, first.coolings, [], []).consequences).toEqual([]);
    expect(reactionsOf(MENAGERIE, eclipse(), 1279, first.coolings, [], []).coolings).toEqual(first.coolings);
    const again = reactionsOf(MENAGERIE, eclipse(), 1280, first.coolings, [], []);
    expect(happened(again)).toEqual(["eclipse", "eclipse", "eclipse"]);
    expect(again.coolings).toEqual([{ reaction: "eclipse", when: "mist", near: "glow", until: 2560 }]);
  });

  it("takes one unit per reaction with a chance, in order, and counts a missing unit as bad luck", () => {
    const stage = [sight("glow", "blazing", 100), sight("wick", "snuffed", 150)];
    const trials = trialsOf(MENAGERIE, stage, [], 0, []);
    expect(trials).toEqual<Trial[]>([{ reaction: 4, when: 0, near: 1, chancy: true }]);
    expect(drawsOf(trials)).toBe(1);
    expect(drawsOf(trialsOf(MENAGERIE, eclipse(), [], 0, []))).toBe(0);
    const lucky = reactionsOf(MENAGERIE, stage, 0, [], [0.499, 0], []);
    expect(happened(lucky)).toEqual(["relight", "relight", "relight"]);
    expect(lucky.drawn).toBe(1);
    for (const chances of [[0.5], [0.9], []]) {
      const unlucky = reactionsOf(MENAGERIE, stage, 0, [], chances, []);
      expect(unlucky.consequences).toEqual([]);
      expect(unlucky.drawn).toBe(1);
      expect(unlucky.coolings).toEqual([{ reaction: "relight", when: "glow", near: "wick", until: 1920 }]);
    }
  });

  it("applies nothing twice in a beat: the first state, mood, trick and activity an actor gets, the first shift and promise a pair gets", () => {
    const stage = [...eclipse(), sight("leaf", "lush", 160)];
    const beat = reactionsOf(MENAGERIE, stage, 0, [], [], []);
    expect(happened(beat)).toEqual(["eclipse", "eclipse", "eclipse", "old-friends", "old-friends"]);
    expect(beat.coolings.map((cooling) => cooling.reaction)).toEqual(["eclipse", "dazzle", "old-friends"]);
    expect(beat.consequences.slice(3).map((consequence) => [consequence.on, consequence.activity, consequence.encounter])).toEqual([
      ["leaf", "walk", null],
      ["glow", null, "cuddle"],
    ]);
    const sleeper = [sight("leaf", "green", 100, { activity: "sleep" }), sight("mist", "fluffy", 160, { y: 250 })];
    const lulled = reactionsOf(MENAGERIE, sleeper, 0, [], [], []).consequences;
    expect(lulled.map((consequence) => [consequence.reaction, consequence.on, consequence.mood, consequence.rapport])).toEqual([
      ["lullaby", "mist", "sleepy", 0],
      ["lullaby", "leaf", null, 0.02],
    ]);
    const twice = { ...MENAGERIE, chemistry: [odd("first", [{ on: "near", activity: "sleep" }, { on: "near", activity: "walk" }]), odd("second", [{ on: "near", activity: "hop" }])] };
    expect(reactionsOf(twice, [sight("leaf", "green", 84), sight("wick", "burning", 122)], 0, [], [], []).consequences.map((consequence) => consequence.activity)).toEqual(["sleep"]);
  });

  it("judges everybody as it stood when the beat began", () => {
    const stage = [sight("mist", "raining", 100, { y: 200 }), sight("leaf", "green", 84), sight("wick", "burning", 122)];
    const beat = reactionsOf(MENAGERIE, stage, 0, [], [], []);
    expect(happened(beat)).toEqual(["soak", "downpour", "downpour", "downpour"]);
    expect(beat.coolings.map((cooling) => cooling.reaction)).toEqual(["soak", "downpour", "encore"]);
    expect(beat.consequences.filter((consequence) => consequence.trick !== null)).toEqual([]);
  });

  it("drops a state the species does not have and a trick it does not offer in its state", () => {
    const stage = [sight("leaf", "green", 84), sight("wick", "burning", 122)];
    const tried = (then: Reaction["then"]): Chemistry => reactionsOf({ ...MENAGERIE, chemistry: [odd("odd", then)] }, stage, 0, [], [], []);
    expect(tried([{ on: "near", state: "frozen" }]).consequences).toEqual([]);
    expect(tried([{ on: "near", trick: "juggle" }]).consequences).toEqual([]);
    expect(tried([{ on: "near", trick: "catch" }]).consequences).toEqual([]);
    expect(tried([{ on: "near", rapport: 0 }]).consequences).toEqual([]);
    expect(tried([{ on: "near", state: "frozen", trick: "dance", mood: "happy", amount: 0 }]).consequences).toEqual<Consequence[]>([{ reaction: "odd", on: "wick", other: "leaf", state: null, mood: "happy", amount: 0, rapport: 0, encounter: null, trick: "dance", activity: null }]);
    expect(tried([{ on: "when", encounter: "cuddle" }, { on: "near", encounter: "squabble" }]).consequences).toEqual<Consequence[]>([{ reaction: "odd", on: "leaf", other: "wick", state: null, mood: null, amount: 0, rapport: 0, encounter: "cuddle", trick: null, activity: null }]);
    expect(tried([{ on: "when", activity: "fidget" }]).consequences).toEqual<Consequence[]>([{ reaction: "odd", on: "leaf", other: "wick", state: null, mood: null, amount: 0, rapport: 0, encounter: null, trick: null, activity: "fidget" }]);
    expect(tried([{ on: "near", state: "high" }]).coolings).toEqual([{ reaction: "odd", when: "leaf", near: "wick", until: 64 }]);
  });

  it("lets a side of any species match whoever stands there, the cooling still kept per pair of species", () => {
    const anyone = { ...MENAGERIE, chemistry: [{ ...odd("anyone", [{ on: "near", mood: "happy" }]), when: { mood: "playful" as const }, near: {} }] };
    const stage = [sight("leaf", "green", 84, { mood: "playful", intensity: 0.6 }), sight("wick", "burning", 122), sight("glow", "bright", 40)];
    const trials = trialsOf(anyone, stage, [], 0, []);
    expect(trials.map((trial) => [stage[trial.when]!.species, stage[trial.near]!.species])).toEqual([
      ["leaf", "glow"],
      ["leaf", "wick"],
    ]);
    const beat = reactionsOf(anyone, stage, 0, [], [], []);
    expect(beat.coolings).toEqual([
      { reaction: "anyone", when: "leaf", near: "glow", until: 64 },
      { reaction: "anyone", when: "leaf", near: "wick", until: 64 },
    ]);
    expect(trialsOf(anyone, stage, beat.coolings, 32, [])).toEqual([]);
  });

  it("holds a reaction back while a third actor that matches unless stands near the second", () => {
    const sheltered = { ...MENAGERIE, chemistry: [{ ...odd("shelter", [{ on: "near", mood: "scared" }]), unless: { species: "glow", state: "bright" }, within: 40 }] };
    const pair = [sight("leaf", "green", 84), sight("wick", "burning", 122)];
    expect(trialsOf(sheltered, pair, [], 0, [])).toHaveLength(1);
    expect(trialsOf(sheltered, [...pair, sight("glow", "bright", 182)], [], 0, [])).toEqual([]);
    expect(trialsOf(sheltered, [...pair, sight("glow", "bright", 200)], [], 0, [])).toHaveLength(1);
    expect(trialsOf(sheltered, [...pair, sight("glow", "dim", 182)], [], 0, [])).toHaveLength(1);
    expect(trialsOf(sheltered, [...pair, sight("glow", "bright", 30)], [], 0, [])).toHaveLength(1);
  });

  it("asks the affinity of the two — the authored bond moved by their rapport — to lie within the bounds", () => {
    const fond = (low: number, high: number): Menagerie => ({ ...MENAGERIE, chemistry: [{ ...odd("fond", [{ on: "near", encounter: "cuddle" }]), affinity: [low, high] }] });
    const pair = [sight("leaf", "green", 84), sight("wick", "burning", 122)];
    expect(trialsOf(fond(0.3, 1), pair, [], 0, [])).toHaveLength(1);
    expect(trialsOf(fond(0.31, 1), pair, [], 0, [])).toEqual([]);
    expect(trialsOf(fond(0.31, 1), pair, [], 0, [{ between: ["wick", "leaf"], drift: 0.25 }])).toHaveLength(1);
    expect(trialsOf(fond(-1, 0.29), pair, [], 0, [])).toEqual([]);
    expect(trialsOf(fond(-1, 0), pair, [], 0, [{ between: ["leaf", "wick"], drift: -0.5 }])).toHaveLength(1);
  });

  it("orders the trials by the species of the menagerie, not by how the actors are listed", () => {
    const marks = units(8, STAGES * 4);
    for (let index = 0; index < Math.min(STAGES, SET.matching.length * 4); index++) {
      const vector = SET.matching[index % SET.matching.length]!;
      const turn = 1 + Math.floor(marks[index]! * (vector.sightings.length - 1));
      const turned = [...vector.sightings.slice(turn), ...vector.sightings.slice(0, turn)];
      const name = (stage: readonly Sighting[], trials: readonly Trial[]): string[] => trials.map((trial) => `${MENAGERIE.chemistry[trial.reaction]!.id}:${stage[trial.when]!.species}:${stage[trial.near]!.species}`);
      const once = new Set(vector.sightings.map((sighting) => sighting.species)).size === vector.sightings.length;
      if (once) expect(name(turned, trialsOf(MENAGERIE, turned, vector.coolings, vector.tick, vector.rapports)), vector.id).toEqual(name(vector.sightings, trialsOf(MENAGERIE, vector.sightings, vector.coolings, vector.tick, vector.rapports)));
    }
  });

  it("never changes what it is given", () => {
    const menagerie = frozen(structuredClone(MENAGERIE));
    for (const vector of SET.reactions) {
      let coolings: readonly Cooling[] = frozen([]);
      for (const beat of vector.beats) coolings = frozen(reactionsOf(menagerie, frozen(structuredClone(beat.sightings)), beat.tick, coolings, frozen([...beat.units]), frozen(structuredClone(beat.rapports))).coolings as Cooling[]);
    }
  });

  it("answers the committed stages and stories", () => {
    for (const vector of SET.matching) agree(trialsOf(MENAGERIE, vector.sightings, vector.coolings, vector.tick, vector.rapports), vector.expected, vector.id);
    for (const vector of SET.reactions) {
      const beats: Chemistry[] = [];
      let coolings: readonly Cooling[] = [];
      for (const beat of vector.beats) {
        const outcome = reactionsOf(MENAGERIE, beat.sightings, beat.tick, coolings, beat.units, beat.rapports);
        coolings = outcome.coolings;
        beats.push(outcome);
      }
      agree(beats, vector.expected, vector.id);
    }
  });
});

describe("the arithmetic of the module", () => {
  it("uses nothing the Rust twin cannot reproduce bit for bit, and never the console", () => {
    const source = readFileSync(resolve(HERE, "../../🟦️.ts"), "utf8");
    expect(source).not.toMatch(/Math\.(sin|cos|tan|atan2|exp|pow|hypot|log|random)\b/);
    expect(source).not.toMatch(/\b(Date|performance|console)\b/);
    expect(source).not.toMatch(/\[DEBUG\]/);
  });
});
