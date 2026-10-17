/** 🪜️ Unit suite of the feeling module, the states of a species: how the states follow each other, the ladder of states and the tricks. */
import { describe, expect, it } from "vitest";
import { CUES, type Feeling, type Slug, type Species, type Trick } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { weightedIndex } from "../../../🎲️randomness/🟦️.ts";
import { MOOD_REST, WHIM_FAVOR, clickTrick, ladderOf, lastingTicks, rungsOf, showTrick, shownMood, stateAfterTrick, stateAt, stateEnds, stepRung, stepState, tricksFor, whimTrick } from "../../🟦️.ts";
import { agree, CALM, kind, MENAGERIE, SET, units } from "../🧰️feeling-test-kit/🟦️.ts";

describe("the states of a species", () => {
  it("last as long as they say and give way to the state they name", () => {
    const wick = kind("wick");
    expect(lastingTicks(wick.states[0]!)).toBe(0);
    expect(lastingTicks(wick.states[1]!)).toBe(640);
    expect(lastingTicks({ id: "blink", name: { en: "", de: "" }, lasts: 0.001 })).toBe(1);
    expect(stateEnds(wick, "burning", 100)).toBeNull();
    expect(stateEnds(wick, "high", 100)).toBe(740);
    expect(stateEnds(wick, "nowhere", 100)).toBeNull();
    expect(stateAt(wick, "high", 100, 739)).toEqual({ state: "high", since: 100 });
    expect(stateAt(wick, "high", 100, 740)).toEqual({ state: "ember", since: 740 });
    expect(stateAt(wick, "burning", 100, 100000000)).toEqual({ state: "burning", since: 100 });
    expect(stateAt(wick, "nowhere", 100, 5000)).toEqual({ state: "burning", since: 100 });
  });

  it("run round a circle of states for as long as anyone likes without walking every lap", () => {
    const wick = kind("wick");
    const lap = 640 + 384 + 96;
    expect(stateAt(wick, "high", 0, lap)).toEqual({ state: "high", since: lap });
    expect(stateAt(wick, "high", 0, lap * 1000000 + 700)).toEqual({ state: "ember", since: lap * 1000000 + 640 });
    expect(stateAt(wick, "sparked", 0, 32 + lap * 123456789 + 1030)).toEqual({ state: "flicker", since: 32 + lap * 123456789 + 1024 });
  });

  it("fall back to the resting state when a lasting state names no state of the species, and never give way to themselves", () => {
    const odd = { states: [{ id: "rest" }, { id: "flash", lasts: 1 }, { id: "lost", lasts: 1, then: "nowhere" }, { id: "loop", lasts: 1, then: "loop" }] } as unknown as Species;
    expect(stateAt(odd, "flash", 0, 64)).toEqual({ state: "rest", since: 64 });
    expect(stateAt(odd, "lost", 0, 640)).toEqual({ state: "rest", since: 64 });
    expect(stateAt(odd, "loop", 0, 640)).toEqual({ state: "loop", since: 0 });
    expect(stateEnds(odd, "loop", 0)).toBeNull();
    expect(stateAt({ states: [] } as unknown as Species, "any", 3, 9)).toEqual({ state: "any", since: 3 });
  });

  it("come out the same however time is cut", () => {
    for (const species of MENAGERIE.species) {
      for (const state of species.states) {
        for (const middle of [0, 31, 640, 1119, 1120, 5000, 77777]) {
          for (const end of [middle, middle + 1, middle + 999, middle + 123456]) {
            const half = stateAt(species, state.id, 100, 100 + middle);
            expect(stateAt(species, half.state, half.since, 100 + end), `${species.id} ${state.id} ${middle} ${end}`).toEqual(stateAt(species, state.id, 100, 100 + end));
          }
        }
      }
    }
  });

  it("answer the committed states over their ticks", () => {
    for (const vector of SET.states) {
      const species = kind(vector.species);
      const state = species.states.find((entry) => entry.id === vector.state);
      agree({ standings: vector.ticks.map((tick) => stateAt(species, vector.state, vector.since, tick)), ends: stateEnds(species, vector.state, vector.since), lasts: state === undefined ? 0 : lastingTicks(state) }, vector.expected, vector.id);
    }
  });
});

describe("the ladder of states and the tricks of a species", () => {
  it("is the order of the states as authored, stepped one rung at a time and held at both ends", () => {
    const mist = kind("mist");
    expect(ladderOf(mist)).toEqual(["fluffy", "heavy", "raining", "thin"]);
    expect(stepState(mist, "fluffy", 1)).toBe("heavy");
    expect(stepState(mist, "fluffy", -1)).toBe("fluffy");
    expect(stepState(mist, "thin", 1)).toBe("thin");
    expect(stepState(mist, "thin", -3)).toBe("raining");
    expect(stepState(mist, "heavy", 0)).toBe("heavy");
    expect(stepRung(["a", "b"], "c", 1)).toBe("c");
  });

  it("lets a circling trick name its own rungs, so a resting state can lie in the middle", () => {
    const glow = kind("glow");
    const up = glow.tricks.find((trick) => trick.id === "wind-up")!;
    const down = glow.tricks.find((trick) => trick.id === "wind-down")!;
    expect(ladderOf(glow)[0]).toBe("bright");
    expect(rungsOf(glow, up)).toEqual(["dusk", "dim", "bright", "blazing"]);
    expect(stateAfterTrick(glow, "bright", up)).toBe("blazing");
    expect(stateAfterTrick(glow, "blazing", up)).toBe("blazing");
    expect(stateAfterTrick(glow, "bright", down)).toBe("dim");
    expect(stateAfterTrick(glow, "dim", down)).toBe("dusk");
    expect(stateAfterTrick(glow, "dusk", down)).toBe("dusk");
    expect(rungsOf(glow, { ...up, from: ["nowhere"] })).toEqual(ladderOf(glow));
  });

  it("leaves the state a trick names, or the state as it was", () => {
    const wick = kind("wick");
    const trick = (id: Slug): Trick => wick.tricks.find((entry) => entry.id === id)!;
    expect(stateAfterTrick(wick, "burning", trick("whoosh"))).toBe("high");
    expect(stateAfterTrick(wick, "high", trick("blow-out"))).toBe("snuffed");
    expect(stateAfterTrick(wick, "ember", trick("dance"))).toBe("ember");
    expect(stateAfterTrick(wick, "ember", { ...trick("whoosh"), to: "nowhere" })).toBe("ember");
  });

  it("offers a trick for a cue in the states it names, and none of a pet's own to one that is sleepy, grumpy, sad or scared", () => {
    const glow = kind("glow");
    const ids = (tricks: readonly Trick[]): Slug[] => tricks.map((trick) => trick.id);
    expect(ids(tricksFor(glow, "click", "bright", CALM))).toEqual(["flare", "spark"]);
    expect(ids(tricksFor(glow, "click", "dim", CALM))).toEqual(["flare"]);
    expect(ids(tricksFor(glow, "whim", "bright", CALM))).toEqual(["flare", "shimmer"]);
    for (const mood of ["sleepy", "grumpy", "sad", "scared"] as const) {
      expect(tricksFor(glow, "whim", "bright", { mood, intensity: 0.6, since: 0 })).toEqual([]);
      expect(tricksFor(glow, "show", "bright", { mood, intensity: 0.6, since: 0 })).toEqual([]);
      expect(ids(tricksFor(glow, "whim", "bright", { mood, intensity: MOOD_REST, since: 0 }))).toEqual(["flare", "shimmer"]);
      expect(ids(tricksFor(glow, "shake", "bright", { mood, intensity: 0.6, since: 0 }))).toEqual(["sputter"]);
      expect(shownMood(mood, MOOD_REST)).toBe("content");
      expect(shownMood(mood, 0.26)).toBe(mood);
    }
  });

  it("plays the click tricks round and round and picks a whim in proportion, favouring the mood a pet shows", () => {
    const glow = kind("glow");
    expect([0, 1, 2, 3, -1].map((index) => clickTrick(glow, "bright", CALM, index)!.id)).toEqual(["flare", "spark", "flare", "spark", "spark"]);
    expect(clickTrick(kind("leaf"), "green", CALM, 5)!.id).toBe("unfurl");
    expect(clickTrick({ ...glow, tricks: [] }, "bright", CALM, 0)).toBeNull();
    const proud: Feeling = { mood: "proud", intensity: 0.6, since: 0 };
    expect(whimTrick(glow, "bright", CALM, 0.49)!.id).toBe("flare");
    expect(whimTrick(glow, "bright", CALM, 0.5)!.id).toBe("shimmer");
    expect(whimTrick(glow, "bright", proud, 0.74)!.id).toBe("flare");
    expect(whimTrick(glow, "bright", proud, 0.75)!.id).toBe("shimmer");
    expect(showTrick(glow, "bright", proud, 0.99)!.id).toBe("shimmer");
    expect(showTrick(kind("wick"), "burning", CALM, 0.5)).toBeNull();
    expect(showTrick(kind("wick"), "snuffed", CALM, 0.5)!.id).toBe("catch");
    expect(whimTrick(glow, "bright", { mood: "sad", intensity: 0.9, since: 0 }, 0.5)).toBeNull();
    const draws = units(6, sampled(300, 3000, 30000));
    const flares = draws.filter((unit) => whimTrick(glow, "bright", proud, unit)!.id === "flare").length;
    expect(Math.abs(flares / draws.length - WHIM_FAVOR / (WHIM_FAVOR + 1))).toBeLessThan(sampled(0.08, 0.03, 0.01));
    expect(weightedIndex([WHIM_FAVOR, 1], 0.74)).toBe(0);
  });

  it("answers the committed ladders and tricks", () => {
    for (const vector of SET.ladders) {
      const species = kind(vector.id);
      const names = ladderOf(species);
      agree(
        {
          ladder: names,
          rungs: Object.fromEntries(species.tricks.map((trick) => [trick.id, rungsOf(species, trick)])),
          steps: Object.fromEntries([...names, "nowhere"].map((name) => [name, { up: stepState(species, name, 1), down: stepState(species, name, -1), stay: stepState(species, name, 0) }])),
          after: Object.fromEntries(species.tricks.map((trick) => [trick.id, Object.fromEntries(names.map((name) => [name, stateAfterTrick(species, name, trick)]))])),
        },
        vector.expected,
        vector.id,
      );
    }
    for (const vector of SET.tricks) {
      const species = kind(vector.id);
      const names = ladderOf(species);
      agree(
        {
          offers: Object.fromEntries(CUES.map((cue) => [cue, Object.fromEntries(names.map((name) => [name, Object.fromEntries(vector.feelings.map((feeling) => [feeling.id, tricksFor(species, cue, name, feeling).map((trick) => trick.id)]))]))])),
          clicks: Object.fromEntries(names.map((name) => [name, vector.clicks.map((index) => clickTrick(species, name, CALM, index)?.id ?? null)])),
          whims: Object.fromEntries(names.map((name) => [name, Object.fromEntries(vector.feelings.map((feeling) => [feeling.id, vector.units.map((unit) => whimTrick(species, name, feeling, unit)?.id ?? null)]))])),
          shows: Object.fromEntries(names.map((name) => [name, Object.fromEntries(vector.feelings.map((feeling) => [feeling.id, vector.units.map((unit) => showTrick(species, name, feeling, unit)?.id ?? null)]))])),
        },
        vector.expected,
        vector.id,
      );
    }
  });

  it("reaches every state of every committed species by its tricks, its lasting states or a reaction", () => {
    for (const vector of SET.reachability) {
      const species = kind(vector.id);
      expect(vector.expected.unreachable, vector.id).toEqual([]);
      expect([...vector.expected.reachable].sort(), vector.id).toEqual([...ladderOf(species)].sort());
      for (const [start, end] of vector.expected.edges) {
        const byTrick = species.tricks.some((trick) => (trick.from === undefined || trick.from.includes(start)) && stateAfterTrick(species, start, trick) === end);
        const byTime = stateEnds(species, start, 0) !== null && stateAt(species, start, 0, stateEnds(species, start, 0)!).state === end;
        const byReaction = MENAGERIE.chemistry.some((reaction) => reaction.then.some((effect) => effect.state === end && (reaction[effect.on].species ?? species.id) === species.id &&(reaction[effect.on].state ?? start) === start));
        expect(byTrick || byTime || byReaction, `${vector.id}: ${start} → ${end}`).toBe(true);
      }
    }
  });
});
