/** 👁️ Unit suite of the stage, attention: attending to the pointer, a hopping gait and seeing through a pet.
 *
 * @see ../🧰️stage-helpers/🟦️.ts — the shared kit; ../../🟦️.ts — the façade under test
 */
import { describe, expect, it } from "vitest";
import { type Activity, type Actor, type Menagerie, type Rect, type Stage } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { restPose, solveRig } from "../../../🦴️rig/🟦️.ts";
import { faceOf, settled } from "../../../💗️feeling/🟦️.ts";
import { advance, frameOf } from "../../🟦️.ts";
import { SECOND, FLOOR, CARD, FAR, COMPANIES, ticked, surveyed, kindOf, actorOf, staged, run, craft, alone } from "../🧰️stage-helpers/🟦️.ts";

describe.each(COMPANIES)("attending to the pointer in $name", (company) => {
  const menagerie = company.menagerie;
  const species = company.walker;
  const kind = kindOf(menagerie, species);
  const half = kind.size.width / 2;
  const level = CARD.y - kind.size.height * 0.6;
  const sated = { energy: 1, sociability: 0.5, curiosity: 0 };
  const eager = { energy: 1, sociability: 0.5, curiosity: 1 };

  /** 🧘️ A pet that has stood on the card for two seconds, facing right, with the drives given. */
  function rested(needs: Actor["needs"], menagerieOf: Menagerie = menagerie): Stage {
    return craft(run(menagerieOf, alone({ ...company, menagerie: menagerieOf }, species, { facing: 1 }), 2 * SECOND), species, { needs });
  }

  it("turns round to a pointer that is clearly behind it, eased, and not to one that is only just behind", () => {
    const facings = (x: number): (1 | -1)[] => {
      const seen: (1 | -1)[] = [];
      run(menagerie, advance(menagerie, rested(sated), [{ kind: "pointed", over: "free", x, y: level }]), 2 * SECOND, (after) => seen.push(actorOf(after, species).facing));
      return seen;
    };
    expect(new Set(facings(550 - half - 11))).toEqual(new Set([1]));
    expect(new Set(facings(550 + 300))).toEqual(new Set([1]));
    const behind = facings(550 - half - 13);
    expect(behind[0]).toBe(-1);
    expect(new Set(behind)).toEqual(new Set([-1]));
    let stage = advance(menagerie, rested(sated), [{ kind: "pointed", over: "free", x: 100, y: level }]);
    stage = run(menagerie, stage, 1);
    expect(actorOf(stage, species)).toMatchObject({ facing: -1, faced: stage.tick + 8, activity: "idle", x: 550 });
    const widths: number[] = [];
    for (let step = 0; step <= 8; step++) {
      const frame = frameOf(menagerie, stage);
      expect(frame.rate).toBe(step < 8 ? 64 : 32);
      widths.push(frame.actors[0]!.bones[0]!);
      stage = run(menagerie, stage, 1);
    }
    expect(widths[0]).toBeLessThan(0);
    expect(widths[4]).toBeCloseTo(0, 12);
    expect(widths[8]).toBeGreaterThan(0);
    for (let step = 1; step <= 8; step++) expect(widths[step]!).toBeGreaterThan(widths[step - 1]!);
  });

  it("never flips back and forth: after a turn it rests for most of a second before the pointer can turn it again", () => {
    let stage = rested(sated);
    let turns = 0;
    let last = -FAR;
    for (let tick = 0; tick < 4 * SECOND; tick++) {
      const before = actorOf(stage, species).facing;
      stage = advance(menagerie, stage, [{ kind: "pointed", over: "free", x: tick % 4 < 2 ? 100 : 1000, y: level }, ticked()]);
      if (actorOf(stage, species).facing !== before) {
        expect(stage.tick - last).toBeGreaterThanOrEqual(SECOND);
        last = stage.tick;
        turns++;
      }
    }
    expect(turns).toBeGreaterThan(1);
    expect(turns).toBeLessThanOrEqual(4);
  });

  it("stays as it is for a pointer in a time of concentration, on a still stage, while it walks or sleeps, and once the pointer is old news", () => {
    const pointedAt = (stage: Stage): Stage => run(menagerie, advance(menagerie, stage, [{ kind: "pointed", over: "free", x: 100, y: level }]), 2 * SECOND);
    const standing = rested(sated);
    expect(actorOf(pointedAt(standing), species).facing).toBe(-1);
    expect(actorOf(pointedAt(advance(menagerie, standing, [{ kind: "hushed", quiet: true }])), species).facing).toBe(1);
    expect(actorOf(pointedAt(advance(menagerie, standing, [{ kind: "tuned", mode: "still" }])), species).facing).toBe(1);
    expect(actorOf(pointedAt(craft(standing, species, { activity: "sleep", clip: null })), species).facing).toBe(1);
    expect(actorOf(pointedAt(craft(standing, species, { activity: "walk", goal: 780 })), species)).toMatchObject({ facing: 1, activity: "walk" });
    const hushed = pointedAt(advance(menagerie, standing, [{ kind: "hushed", quiet: true }]));
    const stale = run(menagerie, advance(menagerie, run(menagerie, hushed, 3 * SECOND), [{ kind: "hushed", quiet: false }]), 2 * SECOND);
    expect(actorOf(stale, species).facing).toBe(1);
  });

  it("leans after its eyes: the bone that carries its first eye turns and shifts with the gaze, eased, and not at all in a time of concentration or on a still stage", () => {
    const bone = kind.bones.findIndex((candidate) => candidate.id === kind.face.eyes[0]!.bone);
    expect(bone).toBeGreaterThanOrEqual(0);
    const upright = (stage: Stage): readonly number[] => frameOf(menagerie, craft(stage, species, { gaze: { x: 0, y: 0, vx: 0, vy: 0 } })).actors[0]!.bones;
    const tilt = (stage: Stage): number => {
      const bones = frameOf(menagerie, stage).actors[0]!.bones;
      const straight = upright(stage);
      return bones[bone * 6]! * straight[bone * 6 + 1]! - bones[bone * 6 + 1]! * straight[bone * 6]!;
    };
    let stage = advance(menagerie, rested(sated), [{ kind: "pointed", over: "free", x: 1100, y: level }]);
    const tilts: number[] = [];
    stage = run(menagerie, stage, SECOND, (after) => tilts.push(tilt(after)));
    const gaze = actorOf(stage, species).gaze;
    expect(gaze.x).toBeGreaterThan(0.8);
    for (let step = 1; step < tilts.length; step++) expect(Math.abs(tilts[step]! - tilts[step - 1]!)).toBeLessThan(0.02);
    expect(Math.abs(tilts.at(-1)!)).toBeGreaterThan(0.05);
    const drop = faceOf(settled(actorOf(stage, species).feeling, kind.mood, stage.tick)).drop;
    const pose = restPose(kind).map((rest, index) => (index === bone ? { ...rest, x: rest.x + 1.5 * gaze.x, y: rest.y + drop + gaze.y, rotation: rest.rotation + 5 * gaze.x } : rest));
    const plain: Menagerie = { ...menagerie, species: menagerie.species.map((entry) => ({ ...entry, repertoire: {}, clips: [] })) };
    const leaning = frameOf(plain, craft(stage, species, { clip: null })).actors[0]!.bones;
    solveRig(kind, pose).forEach((number, index) => expect(leaning[index]!).toBeCloseTo(number, 12));
    expect(frameOf(menagerie, advance(menagerie, stage, [{ kind: "hushed", quiet: true }])).actors[0]!.bones).toEqual(upright(stage));
    const still = advance(menagerie, stage, [{ kind: "tuned", mode: "still" }]);
    expect(frameOf(menagerie, still).actors[0]!.bones).toEqual(solveRig(kind, restPose(kind)));
    const mirrored = craft(stage, species, { facing: -1, gaze: { x: 0 - gaze.x, y: gaze.y, vx: 0, vy: 0 } });
    expect(frameOf(menagerie, mirrored).actors[0]!.bones).toEqual(frameOf(menagerie, craft(stage, species, { gaze: { x: gaze.x, y: gaze.y, vx: 0, vy: 0 } })).actors[0]!.bones);
  });

  it("perks up when the pointer has come to rest beside it for half a second: it greets it, which costs curiosity, and curiosity is the pause before the next greeting", () => {
    const beside = { kind: "pointed", over: "free", x: 550 + half + 20, y: level } as const;
    let stage = advance(menagerie, rested(eager), [beside]);
    const pointed = stage.tick;
    stage = run(menagerie, stage, 31);
    expect(actorOf(stage, species).activity).toBe("idle");
    stage = run(menagerie, stage, 1);
    expect(stage.tick).toBe(pointed + 32);
    const greeting = actorOf(stage, species);
    expect(greeting).toMatchObject({ activity: "greet", since: stage.tick, facing: 1, x: 550, partner: null });
    expect(kind.repertoire.greet).toContain(greeting.clip);
    expect(greeting.until - stage.tick).toBeGreaterThanOrEqual(2 * SECOND);
    expect(greeting.until - stage.tick).toBeLessThan(5 * SECOND);
    expect(greeting.needs.curiosity).toBeCloseTo(0.4, 9);
    expect(frameOf(menagerie, stage).actors[0]!.activity).toBe("greet");
    stage = run(menagerie, stage, greeting.until - stage.tick);
    expect(actorOf(stage, species).activity).toBe("idle");
    let again = run(menagerie, advance(menagerie, stage, [{ ...beside, x: beside.x + 1 }]), 40);
    expect(actorOf(again, species).activity).toBe("idle");
    again = craft(again, species, { needs: { energy: 1, sociability: 0.5, curiosity: 0.45 } });
    again = run(menagerie, advance(menagerie, again, [beside]), 40);
    expect(actorOf(again, species).activity).toBe("idle");
    again = craft(again, species, { needs: { energy: 1, sociability: 0.5, curiosity: 0.5 } });
    again = run(menagerie, advance(menagerie, again, [{ ...beside, x: beside.x + 1 }]), 32);
    expect(actorOf(again, species)).toMatchObject({ activity: "greet", needs: { curiosity: 0 } });
    let rest = run(menagerie, again, actorOf(again, species).until - again.tick);
    rest = craft(rest, species, { until: rest.tick + FAR, blink: rest.tick + FAR });
    let waited = 0;
    while (actorOf(rest, species).activity !== "greet" && waited < 200) {
      rest = run(menagerie, advance(menagerie, rest, [{ ...beside, x: beside.x + (waited % 2) }]), SECOND);
      waited++;
    }
    const pause = 0.5 / (0.01 * (0.5 + kind.temperament.curiosity));
    expect(waited).toBeGreaterThanOrEqual(Math.floor(pause));
    expect(waited).toBeLessThanOrEqual(Math.ceil(pause) + 2);
  });

  it("does not perk up for a pointer that keeps moving, is far away, or in a time of concentration — a pointer that rests on it is greeted like one beside it, and the pet stays whole under it", () => {
    const curious = rested(eager);
    let moving = curious;
    for (let tick = 0; tick < 2 * SECOND; tick++) {
      moving = advance(menagerie, moving, [{ kind: "pointed", over: "free", x: 550 + half + 20 + (tick % 20), y: level }, ticked()]);
      expect(actorOf(moving, species).activity).toBe("idle");
    }
    const resting = (x: number, y: number, quiet = false): Activity => actorOf(run(menagerie, advance(menagerie, curious, [{ kind: "hushed", quiet }, { kind: "pointed", over: "free", x, y }]), 40), species).activity;
    expect(resting(550 + half + 20, level)).toBe("greet");
    expect(resting(550, level)).toBe("greet");
    expect(resting(550 + 4 * kind.size.height, level)).toBe("idle");
    expect(resting(550 + half + 20, level, true)).toBe("idle");
    run(menagerie, advance(menagerie, curious, [{ kind: "pointed", over: "free", x: 550, y: level }]), 40, (after) => expect(actorOf(after, species).opacity).toBe(1));
  });

  it("jumps over the time in which a resting pointer changes nothing, and stops for the tick it has lingered and the tick an actor may turn again", () => {
    const plain: Menagerie = { ...menagerie, species: menagerie.species.map((entry) => ({ ...entry, repertoire: {}, clips: [] })) };
    for (const [needs, x] of [[eager, 550 + half + 20], [sated, 100], [sated, 550 + half + 20]] as const) {
      const pointed = advance(plain, rested(needs, plain), [{ kind: "pointed", over: "free", x, y: level }]);
      for (const chunk of [1, 31, 32, 33, 200, 2000]) expect(advance(plain, pointed, [ticked(chunk)]), `${x} in one chunk of ${chunk}`).toEqual(run(plain, pointed, chunk));
      let stage = pointed;
      for (let step = 0; step < 6 * SECOND; ) {
        const frame = frameOf(plain, stage);
        const ticks = frame.rate === 0 && frame.wake !== null ? Math.min(frame.wake - stage.tick, 6 * SECOND - step) : 1;
        expect(ticks).toBeGreaterThan(0);
        const woken = advance(plain, stage, [ticked(ticks)]);
        expect(woken, `${x} woken ${ticks} ticks after ${stage.tick}`).toEqual(run(plain, stage, ticks));
        if (frame.rate === 0) expect(frameOf(plain, run(plain, stage, ticks - 1)).actors).toEqual(frameOf(plain, stage).actors);
        stage = woken;
        step += ticks;
      }
    }
    const lingering = advance(plain, rested(eager, plain), [{ kind: "pointed", over: "free", x: 550 + half + 20, y: level }]);
    expect(actorOf(advance(plain, lingering, [ticked(31)]), species).activity).toBe("idle");
    expect(actorOf(advance(plain, lingering, [ticked(32)]), species).activity).toBe("greet");
  });
});

describe.each(COMPANIES)("a hopping gait in $name", (company) => {
  const menagerie = company.menagerie;
  const species = company.hopper;
  const kind = kindOf(menagerie, species);

  it("covers ground in whole hops: every walk is a whole number of hops long and ends where a hop ends", () => {
    expect(kind.locomotion.gait).toBe("hop");
    let walks = 0;
    let ended = 0;
    for (let seed = 1; seed <= sampled(2, 4, 8); seed++) {
      run(menagerie, staged(menagerie, [species], { seed, mode: "lively", surfaces: [FLOOR] }), sampled(120, 300, 300) * SECOND, (after, before) => {
        const actor = actorOf(after, species);
        const earlier = actorOf(before, species);
        if (actor.activity === "walk" && earlier.activity !== "walk") {
          walks++;
          const clip = kind.clips.find((candidate) => candidate.id === actor.clip)!;
          const hop = (kind.locomotion.speed * Math.floor(clip.seconds * SECOND + 0.5)) / SECOND;
          const hops = Math.abs(actor.goal - earlier.x) / hop;
          expect(Math.round(hops)).toBeGreaterThanOrEqual(1);
          expect(Math.abs(hops - Math.round(hops))).toBeLessThan(1e-9);
        }
        if (earlier.activity === "walk" && actor.activity !== "walk") {
          ended++;
          const clip = kind.clips.find((candidate) => candidate.id === earlier.clip)!;
          expect((after.tick - earlier.since) % Math.floor(clip.seconds * SECOND + 0.5)).toBe(0);
          expect(actor.x).toBe(earlier.goal);
        }
      });
    }
    expect(walks).toBeGreaterThan(3);
    expect(ended).toBeGreaterThan(2);
  });

  it("finishes its hop on the spot when its goal comes nearer on the way", () => {
    const clip = kind.repertoire.walk?.[0] ?? kind.repertoire.hop![0]!;
    const beat = Math.floor(kind.clips.find((candidate) => candidate.id === clip)!.seconds * SECOND + 0.5);
    const hop = (kind.locomotion.speed * beat) / SECOND;
    let stage = alone(company, species, { activity: "walk", goal: 550 + 2 * hop, facing: 1, clip });
    stage = craft(run(menagerie, stage, 5), species, { goal: 550 + hop / 2 });
    let rested = -1;
    run(menagerie, stage, 3 * beat, (after) => {
      if (rested < 0 && actorOf(after, species).activity !== "walk") rested = after.tick;
    });
    expect(rested - stage.tick + 5).toBe(beat);
  });
});

describe.each(COMPANIES)("seeing through a pet in $name", (company) => {
  const menagerie = company.menagerie;
  const species = company.walker;
  const kind = kindOf(menagerie, species);
  const middle = CARD.y - kind.size.height / 2;
  const NOTE: Rect = { x: 500, y: 470, width: 100, height: 120 };

  /** 🍂️ The opacities of the walker, tick by tick, while it falls — or floats down under its parachute — from the card at x 550 (the card taken away) to the floor and stands there a while, with `keepouts` on the page. */
  function falling(keepouts: readonly Rect[]): { readonly opacities: readonly number[]; readonly footings: readonly string[]; readonly stage: Stage } {
    const stage = advance(menagerie, alone(company, species), [surveyed([FLOOR], keepouts)]);
    const opacities: number[] = [];
    const footings: string[] = [];
    const end = run(menagerie, stage, 16 * SECOND, (after) => {
      const actor = actorOf(after, species);
      opacities.push(actor.opacity);
      footings.push(actor.footing);
      expect(frameOf(menagerie, after).actors[0]!.opacity).toBe(actor.opacity);
    });
    return { opacities, footings, stage: end };
  }

  it("stays whole under a resting pointer: whatever the pointer does, a pet on its perch is never see-through", () => {
    let stage = advance(menagerie, alone(company, species), [{ kind: "pointed", over: "free", x: 550, y: middle }]);
    stage = run(menagerie, stage, 10 * SECOND, (after) => expect(actorOf(after, species).opacity).toBe(1));
    stage = run(menagerie, advance(menagerie, stage, [{ kind: "pointed", over: "free", x: 550 + kind.size.width / 2 + 3, y: CARD.y - 3 }]), SECOND, (after) => expect(actorOf(after, species).opacity).toBe(1));
    expect(actorOf(run(menagerie, advance(menagerie, stage, [{ kind: "unpointed" }]), 11), species).opacity).toBe(1);
  });

  it("turns see-through while it falls in front of what the page keeps free, eased, and whole again once it is past it — and stays whole falling past nothing", () => {
    const shy = falling([NOTE]);
    const lowest = Math.min(...shy.opacities);
    expect(lowest).toBeLessThan(0.9);
    expect(lowest).toBeGreaterThanOrEqual(0.35);
    for (let tick = 1; tick < shy.opacities.length; tick++) expect(Math.abs(shy.opacities[tick]! - shy.opacities[tick - 1]!)).toBeLessThanOrEqual(1 / 16);
    expect(shy.footings.slice(0, shy.opacities.indexOf(lowest) + 1).every((footing) => footing !== "perch")).toBe(true);
    expect(actorOf(shy.stage, species)).toMatchObject({ footing: "perch", perch: FLOOR.id, opacity: 1 });
    expect(Math.min(...falling([]).opacities)).toBe(1);
    expect(Math.min(...falling([{ ...NOTE, x: 900 }]).opacities)).toBe(1);
  });

  it("does not keep a resting stage awake: time is still jumped while the pointer rests on a pet", () => {
    const sated = craft(alone(company, species), species, { needs: { energy: 1, sociability: 0.5, curiosity: 0 } });
    const stage = run(menagerie, advance(menagerie, sated, [{ kind: "pointed", over: "free", x: 550, y: middle }]), 6 * SECOND);
    expect(actorOf(stage, species)).toMatchObject({ opacity: 1, activity: "idle" });
    const started = performance.now();
    const later = advance(menagerie, stage, [ticked(FAR - 10 * SECOND)]);
    expect(performance.now() - started).toBeLessThan(200);
    expect(later.actors).toEqual(stage.actors);
  });

  it("shows every pet whole on a still stage, under the pointer too, and leaves the actor as it is", () => {
    const still = advance(menagerie, alone(company, species), [{ kind: "tuned", mode: "still" }]);
    const pointed = advance(menagerie, still, [{ kind: "pointed", over: "free", x: 550, y: middle }, ticked(100)]);
    expect(pointed.actors).toEqual(still.actors);
    expect(frameOf(menagerie, pointed)).toMatchObject({ rate: 0, wake: null });
    expect(frameOf(menagerie, pointed).actors[0]!.opacity).toBe(1);
  });

  it("lets a newcomer under the pointer fade in all the way", () => {
    let stage = staged(menagerie, [species], { surfaces: [FLOOR] });
    const actor = actorOf(stage, species);
    stage = run(menagerie, advance(menagerie, stage, [{ kind: "pointed", over: "free", x: actor.x, y: actor.y - 5 }]), 40);
    expect(actorOf(stage, species).opacity).toBe(1);
  });
});
