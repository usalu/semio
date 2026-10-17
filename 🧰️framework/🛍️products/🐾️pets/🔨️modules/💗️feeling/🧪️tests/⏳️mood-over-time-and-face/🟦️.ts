/** ⏳️ Unit suite of the feeling module, time and face: how a feeling comes, holds and fades over ticks, and the face a feeling makes. */
import { describe, expect, it } from "vitest";
import { MOODS, type Feeling, type Mood } from "../../../../🧬️schema/🟦️.ts";
import { MOOD_DROPS, MOOD_FAINT, MOOD_HOLD, MOOD_LIDS, MOOD_REST, MOOD_RISE, MOOD_SLANTS, MOOD_VALENCES, PRONE_DECAY, atRest, calmsAt, faceOf, settled, settlesAt, shownMood, spiritsOf, valenceOf } from "../../🟦️.ts";
import { agree, CALM, drawn, FEELINGS, FELT, frozen, units } from "../🧰️feeling-test-kit/🟦️.ts";

describe("a feeling over time", () => {
  it("holds for two seconds, fades at the rate of its mood and gives way to the resting mood, which rises back to its rest", () => {
    const scared: Feeling = { mood: "scared", intensity: 0.5, since: 100 };
    expect(settled(scared, "content", 100 + MOOD_HOLD)).toBe(scared);
    expect(settled(scared, "content", 100 + MOOD_HOLD + 64)).toEqual({ mood: "scared", intensity: 0.5 - 0.15625, since: 164 });
    const gone = settled(scared, "content", 100 + MOOD_HOLD + 192);
    expect(gone.mood).toBe("content");
    expect(gone.intensity).toBeLessThan(MOOD_REST);
    expect(settled(scared, "content", 100000)).toEqual({ mood: "content", intensity: MOOD_REST, since: settlesAt(scared, "content") });
  });

  it("lets a pet stay twice as long in the mood it is prone to, down to its rest and no further", () => {
    const proud: Feeling = { mood: "proud", intensity: 0.75, since: 0 };
    const elsewhere = settled(proud, "content", MOOD_HOLD + 640);
    const prone = settled(proud, "proud", MOOD_HOLD + 640);
    expect(0.75 - prone.intensity).toBeCloseTo((0.75 - elsewhere.intensity) * PRONE_DECAY, 12);
    expect(settled(proud, "proud", 1000000)).toEqual({ mood: "proud", intensity: MOOD_REST, since: settlesAt(proud, "proud") });
    expect(settled({ mood: "proud", intensity: 0, since: 0 }, "proud", 64).intensity).toBe(MOOD_RISE);
  });

  it("shows its mood until no more than the rest level is left, and content from calmsAt on", () => {
    const marks = units(11, FEELINGS * 4);
    for (let index = 0; index < FEELINGS; index++) {
      const feeling = drawn(marks, index * 4);
      const resting = MOODS[Math.floor(marks[index * 4 + 3]! * MOODS.length)]!;
      const calm = calmsAt(feeling, resting);
      const shown = (tick: number): Mood => {
        const present = settled(feeling, resting, tick);
        return shownMood(present.mood, present.intensity);
      };
      expect(shown(calm), `${JSON.stringify(feeling)} ${resting}`).toBe("content");
      if (calm > feeling.since) {
        expect(shown(calm - 1), `${JSON.stringify(feeling)} ${resting}`).toBe(feeling.mood);
        expect(shown(feeling.since)).toBe(feeling.mood);
      }
      for (const later of [1, 64, 6400]) expect(shown(calm + later)).toBe("content");
    }
    expect(calmsAt({ mood: "scared", intensity: 0.5, since: 100 }, "content")).toBe(100 + MOOD_HOLD + 103);
    expect(calmsAt({ mood: "proud", intensity: 0.75, since: 0 }, "proud")).toBe(settlesAt({ mood: "proud", intensity: 0.75, since: 0 }, "proud"));
    expect(calmsAt({ mood: "content", intensity: 0.9, since: 7 }, "happy")).toBe(7);
    expect(calmsAt({ mood: "grumpy", intensity: MOOD_REST, since: 9 }, "content")).toBe(9);
  });

  it("is at rest from the first tick for a pet that nothing has stirred", () => {
    for (const mood of MOODS) {
      const rest = atRest(mood, 40);
      expect(rest).toEqual({ mood, intensity: MOOD_REST, since: 40 });
      expect(settlesAt(rest, mood)).toBe(40);
      for (const tick of [0, 40, 41, 4000, 4000000]) expect(settled(rest, mood, tick)).toBe(rest);
    }
  });

  it("comes out the same however time is cut, and settling twice at one tick changes nothing", () => {
    const marks = units(2, FEELINGS * 6);
    for (let index = 0; index < FEELINGS; index++) {
      const feeling = frozen(drawn(marks, index * 6));
      const resting = index % 3 === 0 ? feeling.mood : MOODS[Math.floor(marks[index * 6 + 3]! * MOODS.length)]!;
      const middle = feeling.since + Math.floor(marks[index * 6 + 4]! * 9000);
      const end = middle + Math.floor(marks[index * 6 + 5]! * 9000);
      const direct = settled(feeling, resting, end);
      expect(settled(settled(feeling, resting, middle), resting, end), `${feeling.mood} ${feeling.intensity} ${resting} ${middle} ${end}`).toEqual(direct);
      expect(settled(direct, resting, end)).toEqual(direct);
      expect(direct.intensity >= 0 && direct.intensity <= 1).toBe(true);
    }
  });

  it("settles exactly at the tick settlesAt names, and not a tick sooner", () => {
    const marks = units(3, FEELINGS * 4);
    for (let index = 0; index < FEELINGS; index++) {
      const feeling = drawn(marks, index * 4);
      const resting = index % 3 === 0 ? feeling.mood : MOODS[Math.floor(marks[index * 4 + 3]! * MOODS.length)]!;
      const rest = settlesAt(feeling, resting);
      const there = settled(feeling, resting, rest);
      expect([there.mood, there.intensity], `${feeling.mood} ${feeling.intensity} ${resting}`).toEqual([resting, MOOD_REST]);
      const before = settled(feeling, resting, rest - 1);
      if (rest > feeling.since) expect(before.mood === resting && before.intensity === MOOD_REST, `${feeling.mood} ${feeling.intensity} ${resting}`).toBe(false);
    }
  });

  it("treats an intensity that is no number in [0, 1] as no feeling at all, and always answers", () => {
    for (const intensity of [Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY, 1.5, -0.5]) {
      for (const mood of ["happy", "content"] as const) {
        const broken: Feeling = { mood, intensity, since: 100 };
        expect(settlesAt(broken, "content")).toBe(100);
        expect(settled(broken, "content", 100)).toEqual({ mood: "content", intensity: MOOD_REST, since: 100 });
        expect(settled(broken, "content", 50).since).toBeLessThanOrEqual(100);
      }
    }
  });

  it("never grows by itself except back up to its rest", () => {
    const marks = units(4, FEELINGS * 4);
    for (let index = 0; index < FEELINGS; index++) {
      const feeling = drawn(marks, index * 4);
      const resting = MOODS[Math.floor(marks[index * 4 + 3]! * MOODS.length)]!;
      let last = feeling;
      for (let tick = feeling.since; tick < feeling.since + 12000; tick += 257) {
        const now = settled(feeling, resting, tick);
        if (now.mood === last.mood && now.intensity > last.intensity) expect(now.mood === resting && now.intensity <= MOOD_REST).toBe(true);
        last = now;
      }
    }
  });

  it("answers the committed feelings over their ticks and the committed cuts of time", () => {
    for (const vector of FELT.decays) agree({ views: vector.ticks.map((tick) => settled(vector.feeling, vector.resting, tick)), settles: settlesAt(vector.feeling, vector.resting) }, vector.expected, vector.id);
    for (const vector of FELT.jumps) {
      let stepped = vector.feeling;
      for (const cut of vector.cuts) stepped = settled(stepped, vector.resting, cut);
      const direct = settled(vector.feeling, vector.resting, vector.cuts[vector.cuts.length - 1]!);
      expect(stepped, vector.id).toEqual(direct);
      agree({ stepped, direct }, vector.expected, vector.id);
    }
  });
});

describe("the face of a feeling", () => {
  it("is the face of content for a content pet and for a mood of no intensity", () => {
    const content = faceOf(CALM);
    expect(content).toEqual({ bend: 0.3, lid: 0.1, slant: 0, drop: 0 });
    for (const mood of MOODS) expect(faceOf({ mood, intensity: 0, since: 0 })).toEqual(content);
    expect(spiritsOf(CALM)).toBe(0.3);
  });

  it("is the face of the mood at full strength, and stays inside what a face can show", () => {
    MOODS.forEach((mood, index) => {
      const face = faceOf({ mood, intensity: 1, since: 0 });
      agree(face, { bend: MOOD_VALENCES[index], lid: MOOD_LIDS[index], slant: MOOD_SLANTS[index], drop: MOOD_DROPS[index] }, mood);
      expect(valenceOf(mood)).toBe(MOOD_VALENCES[index]);
      for (const intensity of [0.1, 0.25, 0.5, 0.9]) {
        const part = faceOf({ mood, intensity, since: 0 });
        expect(Math.abs(part.bend) <= 1 && part.lid >= 0 && part.lid <= 1 && Math.abs(part.slant) <= 14 && Math.abs(part.drop) <= 2).toBe(true);
        expect(spiritsOf({ mood, intensity, since: 0 })).toBe(part.bend);
      }
    });
  });

  it("eases back without a jump when a mood fades", () => {
    for (const mood of MOODS) {
      const last = faceOf({ mood, intensity: MOOD_FAINT, since: 0 });
      const rest = faceOf({ mood: "content", intensity: 0, since: 0 });
      expect(Math.abs(last.bend - rest.bend)).toBeLessThan(0.06);
      expect(Math.abs(last.lid - rest.lid)).toBeLessThan(0.03);
      expect(Math.abs(last.slant - rest.slant)).toBeLessThan(0.8);
      expect(Math.abs(last.drop - rest.drop)).toBeLessThan(0.11);
    }
  });

  it("answers the committed faces", () => {
    for (const vector of FELT.faces) agree({ valence: valenceOf(vector.feeling.mood), spirits: spiritsOf(vector.feeling), face: faceOf(vector.feeling) }, vector.expected, vector.id);
  });
});
