/** 💗️ Unit suite of the feeling module, the tables and the impulse: the mood tables (affinities, decays, drops, lids) and what an impulse does to a feeling.
 *
 * @see ../🧰️feeling-test-kit/🟦️.ts — the shared fixtures and helpers
 * @see ../⏳️mood-over-time-and-face/🟦️.ts — time and face
 * @see ../🎭️appraisal-wishes-and-contagion/🟦️.ts — appraisal, wishes and contagion
 * @see ../🪜️species-states-and-tricks/🟦️.ts — the states of a species and its tricks
 * @see ../⚗️encounters-and-chemistry/🟦️.ts — encounters, chemistry and the arithmetic of the module
 * @see ../../🟦️.ts — the implementation under test
 * @see ../../../../🧫️fixtures/💗️feeling-dynamics/🔣️.json
 * @see ../../../../🧫️fixtures/⚗️chemistry-rules/🔣️.json
 */
import { describe, expect, it } from "vitest";
import { ACTIVITIES, MOODS, type Activity, type Feeling, type Mood } from "../../../../🧬️schema/🟦️.ts";
import { APPRAISALS, CHEMISTRY_BEAT, CONTAGION_BEAT, CONTAGION_REACH, DROWSY_ENERGY, EFFECT_AMOUNT, LEAN_TICKS, MOOD_AFFINITIES, MOOD_DECAYS, MOOD_DROPS, MOOD_FAINT, MOOD_HOLD, MOOD_LIDS, MOOD_OVERRIDE, MOOD_PRIORITIES, MOOD_REST, MOOD_RISE, MOOD_SHARES, MOOD_SLANTS, MOOD_SPREADS, MOOD_VALENCES, MOOD_WEIGHTS, OCCASIONS, PRONENESS, PRONE_DECAY, PRONE_GAIN, TRICK_AMOUNT, WHIM_FAVOR, WILLING_MOODS, impulse, settled } from "../../🟦️.ts";
import { agree, CALM, drawn, FEELINGS, FELT, frozen, units } from "../🧰️feeling-test-kit/🟦️.ts";

describe("the tables of the moods", () => {
  it("carry one entry per mood, in MOODS order, and one multiplier per activity", () => {
    for (const table of [MOOD_PRIORITIES, MOOD_DECAYS, MOOD_VALENCES, MOOD_LIDS, MOOD_SLANTS, MOOD_DROPS, MOOD_AFFINITIES, MOOD_SPREADS, MOOD_SHARES, MOOD_WEIGHTS, PRONENESS, WILLING_MOODS]) expect(table.length).toBe(MOODS.length);
    for (const row of MOOD_WEIGHTS) expect(row.length).toBe(ACTIVITIES.length);
    for (const row of MOOD_SHARES) expect(row.length).toBe(3);
    for (const row of PRONENESS) expect(row.length).toBe(4);
  });

  it("rank the moods as the design says: scared, grumpy, sad, the three joys alike, curious, sleepy, content", () => {
    const rank = (mood: Mood): number => MOOD_PRIORITIES[MOODS.indexOf(mood)]!;
    expect(rank("scared")).toBeGreaterThan(rank("grumpy"));
    expect(rank("grumpy")).toBeGreaterThan(rank("sad"));
    expect(rank("sad")).toBeGreaterThan(rank("proud"));
    expect(rank("proud")).toBe(rank("happy"));
    expect(rank("happy")).toBe(rank("playful"));
    expect(rank("playful")).toBeGreaterThan(rank("curious"));
    expect(rank("curious")).toBeGreaterThan(rank("sleepy"));
    expect(rank("sleepy")).toBeGreaterThan(rank("content"));
  });

  it("keep every number in its range", () => {
    for (const decay of MOOD_DECAYS) expect(decay).toBeGreaterThan(0);
    for (const valence of MOOD_VALENCES) expect(Math.abs(valence)).toBeLessThanOrEqual(1);
    for (const lid of MOOD_LIDS) expect(lid >= 0 && lid < 1).toBe(true);
    for (const spread of MOOD_SPREADS) expect(spread >= 0 && spread < 1).toBe(true);
    for (const row of MOOD_WEIGHTS) for (const weight of row) expect(weight).toBeGreaterThanOrEqual(0);
    for (const row of MOOD_SHARES) for (const share of row) expect(share).toBeGreaterThan(0);
    expect(MOOD_FAINT).toBeLessThan(MOOD_REST);
    expect(MOOD_REST).toBeLessThan(1);
    expect(MOOD_OVERRIDE).toBeGreaterThan(1);
    expect(PRONE_DECAY > 0 && PRONE_DECAY <= 1).toBe(true);
    expect(PRONE_GAIN).toBeGreaterThanOrEqual(1);
    expect(MOOD_HOLD % CONTAGION_BEAT).toBe(0);
    expect(CONTAGION_BEAT).toBe(CHEMISTRY_BEAT);
    expect(CONTAGION_REACH).toBeGreaterThan(0);
    expect(LEAN_TICKS).toBeGreaterThan(0);
    expect(EFFECT_AMOUNT > MOOD_REST && EFFECT_AMOUNT <= 1).toBe(true);
    expect(WHIM_FAVOR).toBeGreaterThan(1);
  });

  it("leave content, the calm mood, without any effect", () => {
    expect(MOOD_WEIGHTS[0]!.every((weight) => weight === 1)).toBe(true);
    expect(MOOD_SHARES[0]).toEqual([1, 1, 1]);
    expect(MOOD_AFFINITIES[0]).toBe(0);
    expect(MOOD_SPREADS[0]).toBe(0);
  });

  it("only move what a pet starts by itself", () => {
    const own: readonly Activity[] = ["fidget", "walk", "hop", "sleep", "aim", "climb", "carry", "trick", "push"];
    for (const row of MOOD_WEIGHTS) ACTIVITIES.forEach((activity, index) => expect(own.includes(activity) || row[index] === 1, activity).toBe(true));
  });

  it("give every occasion at least one impulse, of a mood that exists and an amount a pet can feel", () => {
    for (const occasion of OCCASIONS) expect(APPRAISALS.some((stir) => stir.occasion === occasion), occasion).toBe(true);
    for (const stir of APPRAISALS) {
      expect(OCCASIONS.includes(stir.occasion)).toBe(true);
      expect(MOODS.includes(stir.mood)).toBe(true);
      expect(stir.amount > 0 && stir.amount <= 1).toBe(true);
      if (stir.mood !== "content") expect(stir.amount).toBeGreaterThan(MOOD_REST);
    }
    expect(APPRAISALS.find((stir) => stir.occasion === "tricked")!.amount).toBe(TRICK_AMOUNT);
  });

  it("are the committed tables", () => {
    const tables: Record<string, unknown> = {
      priorities: MOOD_PRIORITIES,
      hold: MOOD_HOLD,
      faint: MOOD_FAINT,
      rest: MOOD_REST,
      rise: MOOD_RISE,
      override: MOOD_OVERRIDE,
      decays: MOOD_DECAYS,
      proneDecay: PRONE_DECAY,
      valences: MOOD_VALENCES,
      lids: MOOD_LIDS,
      slants: MOOD_SLANTS,
      drops: MOOD_DROPS,
      occasions: OCCASIONS,
      appraisals: APPRAISALS,
      proneness: PRONENESS,
      proneGain: PRONE_GAIN,
      trickAmount: TRICK_AMOUNT,
      drowsyEnergy: DROWSY_ENERGY,
      weights: MOOD_WEIGHTS,
      affinities: MOOD_AFFINITIES,
      shares: MOOD_SHARES,
      spreads: MOOD_SPREADS,
      contagionBeat: CONTAGION_BEAT,
      contagionReach: CONTAGION_REACH,
    };
    for (const vector of FELT.tables) agree(tables, vector.expected, vector.id);
  });
});

describe("an impulse", () => {
  it("reinforces the same mood up to 1 and begins its hold anew", () => {
    expect(impulse({ mood: "happy", intensity: 0.5, since: 10 }, "happy", 0.25, 90)).toEqual({ mood: "happy", intensity: 0.75, since: 90 });
    expect(impulse({ mood: "happy", intensity: 0.75, since: 10 }, "happy", 0.5, 90)).toEqual({ mood: "happy", intensity: 1, since: 90 });
  });

  it("replaces a calm feeling with anything a pet can feel", () => {
    for (const mood of MOODS) {
      if (mood === "content") continue;
      expect(impulse(CALM, mood, 0.25, 7)).toEqual({ mood, intensity: 0.25, since: 7 });
      expect(impulse({ mood: "scared", intensity: MOOD_REST, since: 7 }, mood, MOOD_FAINT, 7).mood).toBe(mood === "scared" ? "scared" : mood);
    }
    expect(impulse(CALM, "happy", MOOD_FAINT / 2, 7)).toBe(CALM);
  });

  it("lets a higher rank in at once, an equal one after the hold and a lower one only when it is much stronger", () => {
    const happy: Feeling = { mood: "happy", intensity: 0.75, since: 0 };
    expect(impulse(happy, "scared", 0.25, 1).mood).toBe("scared");
    expect(impulse(happy, "playful", 0.5, MOOD_HOLD - 1)).toBe(happy);
    expect(impulse(happy, "playful", 0.5, MOOD_HOLD).mood).toBe("playful");
    expect(impulse(happy, "curious", 0.75, MOOD_HOLD)).toBe(happy);
    expect(impulse(happy, "curious", 0.75 * MOOD_OVERRIDE, MOOD_HOLD)).toBe(happy);
    expect(impulse(happy, "curious", 1, MOOD_HOLD).mood).toBe("curious");
    expect(impulse(happy, "curious", 1, MOOD_HOLD - 1)).toBe(happy);
  });

  it("soothes with content whatever is worse than content, and never takes joy away", () => {
    const sad: Feeling = { mood: "sad", intensity: 0.75, since: 40 };
    expect(impulse(sad, "content", 0.25, 99)).toEqual({ mood: "sad", intensity: 0.5, since: 40 });
    expect(impulse(sad, "content", 0.75, 99)).toEqual({ mood: "sad", intensity: 0, since: 99 });
    expect(settled(impulse(sad, "content", 1, 99), "content", 99).mood).toBe("content");
    for (const mood of ["happy", "playful", "proud"] as const) {
      const joy: Feeling = { mood, intensity: 0.75, since: 0 };
      expect(impulse(joy, "content", 1, 500)).toBe(joy);
    }
    for (const mood of ["curious", "sleepy", "grumpy", "sad", "scared"] as const) expect(impulse({ mood, intensity: 0.75, since: 0 }, "content", 0.5, 1).intensity).toBe(0.25);
  });

  it("of nothing changes nothing", () => {
    const feeling: Feeling = { mood: "curious", intensity: 0.5, since: 3 };
    for (const mood of MOODS) for (const amount of [0, -0.5, Number.NaN]) expect(impulse(feeling, mood, amount, 500)).toBe(feeling);
  });

  it("keeps every intensity between 0 and 1 and never changes the feeling it is given", () => {
    const marks = units(1, FEELINGS * 6);
    for (let index = 0; index < FEELINGS; index++) {
      const feeling = frozen(drawn(marks, index * 6));
      const after = impulse(feeling, MOODS[Math.floor(marks[index * 6 + 3]! * MOODS.length)]!, marks[index * 6 + 4]! * 1.5, feeling.since + Math.floor(marks[index * 6 + 5]! * 400));
      expect(after.intensity >= 0 && after.intensity <= 1).toBe(true);
      expect(after.since).toBeGreaterThanOrEqual(feeling.since);
    }
  });

  it("answers the committed contest of ranks and the committed stories", () => {
    const contest = (tick: number): number[][] => MOODS.map((present) => MOODS.map((stirring) => (stirring !== present && impulse({ mood: present, intensity: 0.8, since: 0 }, stirring, 0.5, tick).mood === stirring ? 1 : 0)));
    for (const vector of FELT.priorities) agree({ during: contest(MOOD_HOLD / 2), after: contest(MOOD_HOLD) }, vector.expected, vector.id);
    for (const vector of FELT.impulses) {
      let feeling = vector.feeling;
      const feelings = vector.events.map((event) => (feeling = impulse(settled(feeling, vector.resting, event.tick), event.mood, event.amount, event.tick)));
      agree(feelings, vector.expected, vector.id);
    }
  });
});
