/** 🎭️ Unit suite of the feeling module, appraisal and contagion: what happens to a pet, what a mood does to its wishes and encounters, and the mood that travels. */
import { describe, expect, it } from "vitest";
import { ACTIVITIES, MOODS, type Activity, type Feeling, type Mood, type Trick } from "../../../../🧬️schema/🟦️.ts";
import { encounterShares } from "../../../🧠️behavior/🟦️.ts";
import { CONTAGION_BEAT, type Character, DROWSY_ENERGY, MOOD_HOLD, MOOD_REST, MOOD_WEIGHTS, OCCASIONS, PRONE_GAIN, TRICK_AMOUNT, appraised, caught, drowsed, encounterBias, moodWeights, performed, pronenessOf, settled, stirred, swayedShares } from "../../🟦️.ts";
import { agree, CALM, drawn, FEELINGS, FELT, frozen, HOST, units } from "../🧰️feeling-test-kit/🟦️.ts";

describe("the appraisal of what happens to a pet", () => {
  it("scales an impulse by the temperament and by the resting mood of the species", () => {
    const lively: Character = { mood: "playful", temperament: { energy: 1, sociability: 0, curiosity: 0.5 } };
    expect(pronenessOf(lively, "playful")).toBe(1.5 * PRONE_GAIN);
    expect(pronenessOf(lively, "sleepy")).toBe(0.5);
    expect(pronenessOf(lively, "grumpy")).toBe(1.5);
    expect(pronenessOf(lively, "happy")).toBe(0.5);
    expect(pronenessOf(lively, "curious")).toBe(1);
    expect(pronenessOf(lively, "scared")).toBe(1);
    expect(stirred(CALM, "playful", 0.5, lively, 9)).toEqual({ mood: "playful", intensity: 1, since: 9 });
    expect(stirred(CALM, "happy", 0.5, lively, 9)).toEqual({ mood: "happy", intensity: 0.25, since: 9 });
  });

  it("is never negative for any temperament", () => {
    for (const energy of [0, 0.5, 1]) for (const sociability of [0, 0.5, 1]) for (const curiosity of [0, 0.5, 1]) for (const mood of MOODS) expect(pronenessOf({ mood: "content", temperament: { energy, sociability, curiosity } }, mood)).toBeGreaterThanOrEqual(0);
  });

  it("makes a hello happy, a lift scary, a hard landing grumpy and a soft one under a parachute proud", () => {
    expect(appraised(CALM, "greeted", HOST, 5).mood).toBe("happy");
    expect(appraised(CALM, "lifted", HOST, 5).mood).toBe("scared");
    expect(appraised(appraised(CALM, "lifted", HOST, 5), "shaken", HOST, 6).intensity).toBe(0.8);
    expect(appraised(appraised(CALM, "lifted", HOST, 5), "dropped", HOST, 300).mood).toBe("grumpy");
    expect(appraised(appraised(CALM, "lifted", HOST, 5), "floated", HOST, 70).mood).toBe("proud");
    expect(appraised(CALM, "trampled", HOST, 5).mood).toBe("grumpy");
    expect(appraised(CALM, "drenched", HOST, 5).mood).toBe("sad");
    expect(appraised(CALM, "watched", HOST, 5).mood).toBe("curious");
  });

  it("ends fear, anger and sadness with a cuddle, and cheers a sad pet with two hellos", () => {
    for (const mood of ["scared", "grumpy", "sad"] as const) expect(appraised({ mood, intensity: 1, since: 0 }, "cuddled", HOST, 10).mood).toBe("happy");
    const sad: Feeling = { mood: "sad", intensity: 0.6, since: 0 };
    const once = appraised(sad, "greeted", HOST, 300);
    expect(once.mood).toBe("sad");
    expect(once.intensity).toBeLessThan(sad.intensity);
    expect(appraised(once, "greeted", HOST, 310).mood).toBe("happy");
  });

  it("lets a sulk end in peace and a sleeper wake up content", () => {
    const sulking = appraised(CALM, "squabbled", HOST, 0);
    expect(sulking).toEqual({ mood: "grumpy", intensity: 0.5, since: 0 });
    expect(appraised(sulking, "mended", { mood: "grumpy", temperament: HOST.temperament }, 400).intensity).toBeCloseTo(0.2, 12);
    expect(appraised(sulking, "mended", HOST, 400).intensity).toBeCloseTo(0.5 - 0.3 * PRONE_GAIN, 12);
    expect(settled(appraised({ mood: "sleepy", intensity: 0.9, since: 0 }, "woken", HOST, 50), "content", 50).mood).toBe("content");
  });

  it("leaves the mood a trick names, or playfulness when it names none", () => {
    expect(performed(CALM, { mood: "proud" } as Trick, HOST, 3)).toEqual({ mood: "proud", intensity: TRICK_AMOUNT, since: 3 });
    expect(performed(CALM, {} as Trick, HOST, 3)).toEqual({ mood: "playful", intensity: TRICK_AMOUNT, since: 3 });
  });

  it("lets sleepiness follow the energy need: a calm pet grows sleepy, a stirred one only when it is worn out", () => {
    expect(drowsed(CALM, DROWSY_ENERGY, 5)).toBe(CALM);
    expect(drowsed(CALM, 0.3, 5)).toEqual({ mood: "sleepy", intensity: 0.5, since: 5 });
    expect(drowsed({ mood: "sleepy", intensity: 0.5, since: 5 }, 0, 9)).toEqual({ mood: "sleepy", intensity: 1, since: 9 });
    const sleepy: Feeling = { mood: "sleepy", intensity: 0.75, since: 5 };
    expect(drowsed(sleepy, 0.3, 9)).toBe(sleepy);
    const happy: Feeling = { mood: "happy", intensity: 0.75, since: 0 };
    expect(drowsed(happy, 0.3, 1000)).toBe(happy);
    expect(drowsed(happy, 0, MOOD_HOLD - 1)).toBe(happy);
    expect(drowsed(happy, 0, 1000)).toEqual({ mood: "sleepy", intensity: 1, since: 1000 });
  });

  it("answers the committed characters", () => {
    for (const vector of FELT.appraisals) {
      const kinds: readonly (Mood | undefined)[] = [undefined, ...MOODS];
      agree(
        {
          proneness: MOODS.map((mood) => pronenessOf(vector.character, mood)),
          occasions: Object.fromEntries(OCCASIONS.map((occasion) => [occasion, vector.feelings.map((feeling) => appraised(feeling, occasion, vector.character, vector.tick))])),
          tricks: Object.fromEntries(kinds.map((mood) => [mood ?? "none", vector.feelings.map((feeling) => performed(feeling, { mood } as Trick, vector.character, vector.tick))])),
          drowsy: vector.feelings.map((feeling) => vector.energies.map((energy) => drowsed(feeling, energy, vector.tick))),
        },
        vector.expected,
        vector.id,
      );
    }
  });
});

describe("what a mood does to the wishes and the encounters of a pet", () => {
  it("multiplies nothing for a calm heart and the whole row at full strength", () => {
    for (const mood of MOODS) {
      expect(moodWeights(mood, 0)).toEqual(ACTIVITIES.map(() => 1));
      expect(moodWeights(mood, 1)).toEqual(MOOD_WEIGHTS[MOODS.indexOf(mood)]);
      expect(moodWeights(mood, 0.5).every((weight) => weight >= 0)).toBe(true);
    }
  });

  it("makes the playful perform, the sleepy sleep and the scared freeze", () => {
    const at = (weights: readonly number[], activity: Activity): number => weights[ACTIVITIES.indexOf(activity)]!;
    expect(at(moodWeights("playful", 1), "trick")).toBe(3);
    expect(at(moodWeights("playful", 0.5), "fidget")).toBe(1.5);
    expect(at(moodWeights("sleepy", 1), "sleep")).toBe(4);
    expect(at(moodWeights("scared", 1), "sleep")).toBe(0);
    expect(at(moodWeights("scared", 1), "trick")).toBe(0);
    expect(at(moodWeights("curious", 1), "climb")).toBe(2);
    expect(at(moodWeights("grumpy", 0.5), "idle")).toBe(1);
  });

  it("lets two grumpy pets squabble sooner and a scared one be comforted by a friend", () => {
    const share = (affinity: number, first: Feeling, second: Feeling, index: number): number => {
      const leaning = encounterBias(first, second);
      const shares = swayedShares(encounterShares(affinity + leaning.affinity), leaning);
      return shares[index]! / (shares[0]! + shares[1]! + shares[2]!);
    };
    const grumpy: Feeling = { mood: "grumpy", intensity: 0.9, since: 0 };
    const scared: Feeling = { mood: "scared", intensity: 0.9, since: 0 };
    const happy: Feeling = { mood: "happy", intensity: 0.9, since: 0 };
    expect(share(0, grumpy, grumpy, 2)).toBeGreaterThan(2 * share(0, CALM, CALM, 2));
    expect(share(0.3, grumpy, grumpy, 2)).toBeGreaterThan(0);
    expect(share(0.3, CALM, CALM, 2)).toBe(0);
    expect(share(0.5, scared, CALM, 1)).toBeGreaterThan(share(0.5, CALM, CALM, 1));
    expect(share(-0.2, scared, CALM, 2)).toBeLessThan(share(-0.2, CALM, CALM, 2));
    expect(share(0.2, happy, happy, 1)).toBeGreaterThan(share(0.2, CALM, CALM, 1));
  });

  it("lets the proud one show off: the prouder of two, the first of equals, nobody at rest", () => {
    const proud = (intensity: number): Feeling => ({ mood: "proud", intensity, since: 0 });
    expect(encounterBias(proud(0.6), CALM).show).toBe(0);
    expect(encounterBias(CALM, proud(0.6)).show).toBe(1);
    expect(encounterBias(proud(0.6), proud(0.9)).show).toBe(1);
    expect(encounterBias(proud(0.9), proud(0.9)).show).toBe(0);
    expect(encounterBias(proud(MOOD_REST), CALM).show).toBe(-1);
    expect(encounterBias(CALM, CALM)).toEqual({ affinity: 0, shares: [1, 1, 1], show: -1 });
  });

  it("answers the committed wishes and leanings", () => {
    for (const vector of FELT.wishes) agree(Object.fromEntries(MOODS.map((mood) => [mood, moodWeights(mood, vector.intensity)])), vector.expected, vector.id);
    for (const vector of FELT.leanings) {
      const leaning = encounterBias(vector.first, vector.second);
      agree({ leaning, swayed: swayedShares(vector.shares, leaning) }, vector.expected, vector.id);
    }
  });
});

describe("a mood that travels", () => {
  it("is adopted by a calm neighbour, weaker than where it came from, with a hold of its own", () => {
    const sleepy: Feeling = { mood: "sleepy", intensity: 1, since: 0 };
    expect(caught(CALM, sleepy, 1, 1, 64)).toEqual({ mood: "sleepy", intensity: 0.8, since: 64 });
    expect(caught(CALM, sleepy, 0, 0.5, 64)).toEqual({ mood: "sleepy", intensity: 0.2, since: 64 });
    expect(caught(CALM, sleepy, -0.6, 0.25, 64)).toBe(CALM);
  });

  it("lifts a neighbour in the same mood towards the stronger one and leaves its hold alone", () => {
    const mine: Feeling = { mood: "playful", intensity: 0.5, since: 30 };
    expect(caught(mine, { mood: "playful", intensity: 1, since: 0 }, 1, 1, 64)).toEqual({ mood: "playful", intensity: 0.8, since: 30 });
    expect(caught(mine, { mood: "playful", intensity: 0.4, since: 0 }, 1, 1, 64)).toBe(mine);
  });

  it("stays where it is when the neighbour is busy with a mood of its own, when it only rests, or when it does not spread", () => {
    const happy: Feeling = { mood: "happy", intensity: 0.75, since: 0 };
    expect(caught(happy, { mood: "scared", intensity: 1, since: 0 }, 1, 1, 64)).toBe(happy);
    expect(caught(CALM, { mood: "sleepy", intensity: MOOD_REST, since: 0 }, 1, 1, 64)).toBe(CALM);
    for (const mood of ["content", "proud", "sad"] as const) expect(caught(CALM, { mood, intensity: 1, since: 0 }, 1, 1, 64)).toBe(CALM);
    expect(caught(CALM, { mood: "playful", intensity: 1, since: 0 }, 1, 0, 64)).toBe(CALM);
  });

  it("never grows on the way", () => {
    const marks = units(5, FEELINGS * 8);
    for (let index = 0; index < FEELINGS; index++) {
      const mine = frozen(drawn(marks, index * 8));
      const theirs = frozen(drawn(marks, index * 8 + 3));
      const after = caught(mine, theirs, marks[index * 8 + 6]! * 1.6 - 0.6, marks[index * 8 + 7]!, 999);
      expect(after.intensity >= 0 && after.intensity <= 1).toBe(true);
      if (after !== mine) {
        expect(after.mood).toBe(theirs.mood);
        expect(after.intensity).toBeLessThanOrEqual(theirs.intensity);
      }
    }
  });

  it("answers the committed crowds beat by beat", () => {
    for (const vector of FELT.contagion) {
      const beats: Feeling[][] = [];
      let feelings: readonly Feeling[] = vector.feelings;
      for (let beat = 1; beat <= vector.beats; beat++) {
        const tick = beat * CONTAGION_BEAT;
        const present = feelings.map((feeling, index) => settled(feeling, vector.restings[index]!, tick));
        const next = present.map((mine, catcher) => {
          let feeling = mine;
          for (let giver = 0; giver < present.length; giver++) if (giver !== catcher && vector.near[giver]![catcher]!) feeling = caught(feeling, present[giver]!, vector.affinities[giver]![catcher]!, vector.sociabilities[catcher]!, tick);
          return feeling;
        });
        beats.push(next);
        feelings = next;
      }
      agree(beats, vector.expected, vector.id);
    }
  });
});
