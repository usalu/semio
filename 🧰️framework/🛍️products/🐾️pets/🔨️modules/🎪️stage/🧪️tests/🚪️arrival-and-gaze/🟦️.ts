/** 🚪️ Unit suite of the stage, arrival: opening the stage, what an actor arrives with, the default liveliness, the gaze and blinking.
 *
 * @see ../🧰️stage-helpers/🟦️.ts — the shared kit; ../../🟦️.ts — the façade under test
 */
import { describe, expect, it } from "vitest";
import { ACTIVITIES, type Actor, type Menagerie, type PetMode, type Rect, type Slug, type Stage, type StageEvent, type Surface } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { BLINK_TICKS } from "../../../🎞️animation/🟦️.ts";
import { perchAt } from "../../../🏞️terrain/🟦️.ts";
import { lookOffset } from "../../../🦴️rig/🟦️.ts";
import { menagerieIssues } from "../../../✅️validation/🟦️.ts";
import { MODE_LIMITS, followersOf } from "../../../🧠️behavior/🟦️.ts";
import { faceOf, settled } from "../../../💗️feeling/🟦️.ts";
import { IDLE, noShaking } from "../../../👆️gesture/🟦️.ts";
import { advance, frameOf, openStage } from "../../🟦️.ts";
import { SECOND, FLOOR, CARD, FAR, SAMPLE, COMPANIES, ticked, surveyed, clicked, kindOf, hoverOf, actorOf, staged, run, craft, alone, covers } from "../🧰️stage-helpers/🟦️.ts";

/** 👁️ Where an actor's eyes are, as the stage sees them. */
function eyeOf(menagerie: Menagerie, actor: Actor): { x: number; y: number } {
  return { x: actor.x, y: actor.y - kindOf(menagerie, actor.species).size.height * 0.6 };
}

describe("openStage", () => {
  it("opens empty, calm, not quiet, at tick 0, with nothing permitted, no press, no ladder, no lift and no trip", () => {
    expect(openStage(7)).toEqual({
      seed: 7,
      tick: 0,
      mode: "calm",
      quiet: false,
      width: 0,
      height: 0,
      pointer: null,
      pointed: 0,
      over: "free",
      glances: [],
      surfaces: [],
      keepouts: [],
      walls: [],
      fixtures: [],
      perches: [],
      pitches: [],
      wanted: [],
      actors: [],
      rapports: [],
      met: 0,
      draws: 0,
      play: false,
      mischief: false,
      stirred: 0,
      scrolled: 0,
      press: IDLE,
      touched: null,
      shaking: noShaking(0),
      trail: [],
      coolings: [],
      pledges: [],
      ladders: [],
      lift: null,
      rested: 0,
      poofs: 0,
      puffs: [],
      claims: [],
      courses: [],
      trips: [],
      origin: null,
    });
    expect(openStage(-1).seed).toBe(4294967295);
    expect(frameOf(SAMPLE.menagerie, openStage(7))).toEqual({ tick: 0, actors: [], rate: 0, wake: null, ladders: [], particles: [], lifts: [], puffs: [], held: null });
  });

  it("plays menageries its own validator accepts", () => {
    for (const company of COMPANIES) expect(menagerieIssues(company.menagerie), company.name).toEqual([]);
  });
});

describe.each(COMPANIES)("arrival in $name", (company) => {
  const menagerie = company.menagerie;
  const everyone = menagerie.species.map((species) => species.id);

  it("puts every summoned species on a perch, in menagerie order, idle and invisible, and lets it fade in", () => {
    const stage = staged(menagerie, [...everyone].reverse());
    expect(stage.actors.map((actor) => actor.species)).toEqual(everyone);
    for (const actor of stage.actors) {
      const kind = kindOf(menagerie, actor.species);
      const perch = perchAt(stage.perches, actor.perch!, actor.x)!;
      expect(perch, actor.species).not.toBeNull();
      expect(actor.x).toBeGreaterThanOrEqual(perch.x0 + kind.size.width / 2);
      expect(actor.x).toBeLessThanOrEqual(perch.x1 - kind.size.width / 2);
      expect(actor.y).toBe(perch.y - hoverOf(kind));
      expect([actor.activity, actor.opacity, actor.leaving, actor.partner]).toEqual(["idle", 0, false, null]);
      expect(actor.until).toBeGreaterThan(stage.tick);
      expect(actor.blink).toBeGreaterThan(stage.tick);
    }
    let previous = 0;
    const shown = run(menagerie, stage, 16, (after) => {
      expect(after.actors[0]!.opacity).toBeGreaterThan(previous);
      previous = after.actors[0]!.opacity;
    });
    for (const actor of shown.actors) expect(actor.opacity).toBe(1);
  });

  it("keeps newcomers a comfortable gap apart from whoever stands there already", () => {
    for (let seed = 1; seed <= sampled(6, 20, 200); seed++) {
      const stage = staged(menagerie, everyone, { seed });
      expect(stage.actors).toHaveLength(everyone.length);
      for (const one of stage.actors) for (const two of stage.actors) if (one.species < two.species && one.perch === two.perch) expect(Math.abs(one.x - two.x)).toBeGreaterThanOrEqual((kindOf(menagerie, one.species).size.width + kindOf(menagerie, two.species).size.width) / 2 + 8 - 1e-9);
    }
  });

  it("spreads newcomers out: nobody shares a perch while another one is empty, and the ground is taken last", () => {
    const shelves: Surface[] = [
      { id: "one", x0: 40, x1: 400, y: 300 },
      { id: "two", x0: 440, x1: 800, y: 350 },
      { id: "three", x0: 840, x1: 1240, y: 300 },
    ];
    for (let seed = 1; seed <= sampled(4, 12, 120); seed++) {
      const few = staged(menagerie, everyone.slice(0, 3), { seed, surfaces: [...shelves, FLOOR] });
      expect(few.actors.map((actor) => actor.perch).sort(), `seed ${seed}`).toEqual(["one", "three", "two"]);
      const more = advance(menagerie, few, [{ kind: "summoned", species: everyone.slice(0, 3).concat(everyone.slice(3, 4)) }]);
      if (everyone.length > 3) expect(actorOf(more, everyone[3]!).perch).toBe(FLOOR.id);
      const alone = staged(menagerie, everyone.slice(0, 1), { seed, surfaces: [FLOOR] });
      expect(alone.actors[0]!.perch).toBe(FLOOR.id);
    }
    const perches = new Set<string | null>();
    for (let seed = 1; seed <= 40; seed++) perches.add(staged(menagerie, everyone.slice(0, 1), { seed, surfaces: [...shelves, FLOOR] }).actors[0]!.perch);
    expect(perches).toEqual(new Set(["one", "two", "three"]));
  });

  it("spreads out over new ground at once: when a survey brings a surface, whoever shares a perch leaves for one that holds nobody", () => {
    const shelf: Surface = { id: "shelf", x0: 40, x1: 400, y: 300 };
    let stage = run(menagerie, staged(menagerie, everyone, { surfaces: [FLOOR] }), 2 * SECOND);
    for (const actor of stage.actors) stage = craft(stage, actor.species, { activity: "idle", until: stage.tick + FAR, partner: null });
    expect(stage.actors.every((actor) => actor.perch === FLOOR.id)).toBe(true);
    expect(advance(menagerie, stage, [surveyed([FLOOR])]).actors.some((actor) => actor.leaving)).toBe(false);
    const hushed = advance(menagerie, stage, [{ kind: "hushed", quiet: true }, surveyed([CARD, shelf, FLOOR])]);
    expect(hushed.actors.some((actor) => actor.leaving)).toBe(false);
    const widened = advance(menagerie, stage, [surveyed([CARD, shelf, FLOOR])]);
    const leavers = widened.actors.filter((actor) => actor.leaving).map((actor) => actor.species);
    expect(leavers).toEqual(everyone.slice(1, 3));
    expect(advance(menagerie, widened, [surveyed([CARD, shelf, FLOOR])])).toEqual(widened);
    const landed = new Map<Slug, string | null>();
    run(menagerie, widened, 8 * SECOND, (after, before) => {
      for (const species of leavers) {
        const actor = after.actors.find((candidate) => candidate.species === species);
        if (actor !== undefined && !actor.leaving && !landed.has(species) && before.actors.find((candidate) => candidate.species === species)?.leaving !== false) landed.set(species, actor.perch);
      }
    });
    expect([...landed.values()].sort()).toEqual([CARD.id, shelf.id]);
    const still = advance(menagerie, advance(menagerie, stage, [{ kind: "tuned", mode: "still" }]), [surveyed([CARD, shelf, FLOOR])]);
    expect(still.actors.map((actor) => actor.species)).toEqual(everyone);
    expect(leavers.map((species) => actorOf(still, species)).map((actor) => [actor.opacity, actor.leaving])).toEqual([[1, false], [1, false]]);
    expect(leavers.map((species) => actorOf(still, species).perch).sort()).toEqual([CARD.id, shelf.id]);
  });

  it("is gone with ground that vanishes when the same survey brings new ground, and arrives anew on it; without new ground it falls", () => {
    const shelf: Surface = { id: "shelf", x0: 40, x1: 400, y: 300 };
    const species = company.walker;
    let stage = run(menagerie, staged(menagerie, [species], { surfaces: [CARD, FLOOR] }), SECOND);
    expect(actorOf(stage, species)).toMatchObject({ perch: CARD.id, opacity: 1 });
    stage = craft(stage, species, { activity: "idle", until: stage.tick + FAR, x: 550, goal: 550 });
    const fallen = advance(menagerie, stage, [surveyed([FLOOR])]);
    expect(actorOf(fallen, species)).toMatchObject({ activity: "fall", perch: null, x: 550 });
    const moved = advance(menagerie, stage, [surveyed([shelf, FLOOR])]);
    expect(actorOf(moved, species)).toMatchObject({ activity: "idle", perch: shelf.id, opacity: 0, since: stage.tick });
    expect(actorOf(moved, species).x).toBeLessThanOrEqual(shelf.x1);
    const replaced = advance(menagerie, stage, [surveyed([{ ...CARD, id: "card-again" }, FLOOR])]);
    expect(actorOf(replaced, species)).toMatchObject({ activity: "idle", perch: "card-again", x: 550, opacity: 1 });
    const hushed = advance(menagerie, stage, [{ kind: "hushed", quiet: true }, surveyed([shelf, FLOOR])]);
    expect(actorOf(hushed, species)).toMatchObject({ perch: shelf.id, opacity: 0 });
  });

  it("lets the summoned wait while there is no perch, and arrive with the first one", () => {
    const waiting = staged(menagerie, everyone, { surfaces: [] });
    expect(waiting.actors).toEqual([]);
    expect(waiting.wanted).toEqual(everyone);
    expect(frameOf(menagerie, run(menagerie, waiting, 100))).toMatchObject({ rate: 0, wake: null, actors: [] });
    const arrived = advance(menagerie, waiting, [ticked(5000), surveyed([FLOOR])]);
    expect(arrived.actors.map((actor) => actor.species)).toEqual(everyone);
    expect(arrived.tick).toBe(5000);
  });

  it("never lets anyone stand inside a keep-out", () => {
    const keepouts: Rect[] = [
      { x: 380, y: 360, width: 120, height: 30 },
      { x: 0, y: 690, width: 500, height: 30 },
      { x: 900, y: 600, width: 100, height: 200 },
    ];
    const covered: string[] = [];
    for (let seed = 1; seed <= sampled(2, 10, 30); seed++) {
      run(menagerie, staged(menagerie, everyone, { seed, mode: "lively", keepouts }), sampled(10, 30, 30) * SECOND, (after) => {
        for (const actor of after.actors) if (actor.perch !== null) for (const keepout of keepouts) if (covers(menagerie, actor, keepout)) covered.push(`seed ${seed} tick ${after.tick}: ${actor.species} at ${actor.x}`);
      });
    }
    expect(covered).toEqual([]);
  });
});

describe.each(COMPANIES)("the default liveliness of $name", (company) => {
  const menagerie = company.menagerie;
  const everyone = menagerie.species.map((species) => species.id);

  const SESSION = sampled(120, 600, 600) * SECOND;

  /** 📊️ What happens in a session of ten minutes (two at the fundamental level): the share of ticks spent idle or asleep, how often each activity begins, and the most movers at once. */
  function session(mode: PetMode, seed: number): { rest: number; starts: Record<string, number>; movers: number; moving: number } {
    const starts: Record<string, number> = Object.fromEntries(ACTIVITIES.map((activity) => [activity, 0]));
    let resting = 0;
    let all = 0;
    let movers = 0;
    let moving = 0;
    run(menagerie, staged(menagerie, everyone, { seed, mode }), SESSION, (after, before) => {
      let now = 0;
      for (const actor of after.actors) {
        all++;
        if (actor.activity === "idle" || actor.activity === "sleep") resting++;
        if (actor.activity === "walk" || actor.activity === "hop") now++;
        const earlier = before.actors.find((candidate) => candidate.species === actor.species);
        if (earlier !== undefined && (earlier.activity !== actor.activity || earlier.since !== actor.since) && !(earlier.activity === "idle" && actor.activity === "idle")) starts[actor.activity]!++;
      }
      movers = Math.max(movers, now);
      if (now > 0) moving++;
    });
    return { rest: resting / all, starts, movers, moving };
  }

  it("is slightly active in calm: mostly idle, a fidget and a walk now and then, never two movers at once", () => {
    let fidgets = 0;
    let walks = 0;
    const seeds = sampled(1, 3, 8);
    for (let seed = 1; seed <= seeds; seed++) {
      const story = session("calm", seed);
      expect(story.rest, `seed ${seed}`).toBeGreaterThan(0.8);
      expect(story.movers).toBeLessThanOrEqual(MODE_LIMITS.calm.movers);
      expect(story.moving / SESSION).toBeLessThan(0.3);
      fidgets += story.starts.fidget!;
      walks += story.starts.walk! + story.starts.hop!;
    }
    expect(fidgets).toBeGreaterThanOrEqual(2 * seeds);
    expect(walks).toBeGreaterThanOrEqual(seeds);
  });

  it("is busier in lively, with at most two movers at once", () => {
    const calm = session("calm", 4);
    const lively = session("lively", 4);
    expect(lively.movers).toBeLessThanOrEqual(MODE_LIMITS.lively.movers);
    expect(lively.starts.fidget! + lively.starts.walk!).toBeGreaterThan(calm.starts.fidget! + calm.starts.walk!);
    expect(lively.rest).toBeGreaterThan(0.5);
  });

  it("only ever changes activity along the activity graph, whatever happens to the stage", () => {
    let stage = staged(menagerie, everyone, { seed: 9, mode: "lively" });
    const follow = (after: Stage, before: Stage): void => {
      for (const actor of after.actors) {
        const earlier = before.actors.find((candidate) => candidate.species === actor.species);
        if (earlier !== undefined && earlier.activity !== actor.activity) expect(followersOf(earlier.activity), `${actor.species}: ${earlier.activity} → ${actor.activity}`).toContain(actor.activity);
      }
    };
    const happenings: Record<number, StageEvent[]> = {
      20: clicked(500, 385),
      40: [surveyed([FLOOR])],
      60: [surveyed([CARD, FLOOR])],
      75: [{ kind: "summoned", species: everyone.slice(0, 2) }],
      90: [{ kind: "summoned", species: everyone }],
      120: [{ kind: "hushed", quiet: true }],
      150: [{ kind: "hushed", quiet: false }, { kind: "tuned", mode: "calm" }],
      170: [{ kind: "tuned", mode: "still" }],
      175: [{ kind: "tuned", mode: "lively" }],
    };
    for (let second = 0; second < sampled(180, 240, 240); second++) {
      const events = happenings[second];
      if (events !== undefined) {
        const next = advance(menagerie, stage, events);
        follow(next, stage);
        stage = next;
      }
      stage = run(menagerie, stage, SECOND, follow);
    }
  });
});

describe.each(COMPANIES)("the gaze in $name", (company) => {
  const menagerie = company.menagerie;
  const species = company.walker;

  it("follows the pointer with the eyes while standing still, within a quarter of a second", () => {
    let stage = alone(company, species, { facing: 1 });
    const pointer = { x: 250, y: 150 };
    stage = advance(menagerie, stage, [{ kind: "pointed", over: "free", x: pointer.x, y: pointer.y }]);
    const goal = lookOffset(eyeOf(menagerie, actorOf(stage, species)), pointer, 32);
    expect(goal.x).toBeLessThan(-0.5);
    expect(goal.y).toBeLessThan(0);
    const soon = run(menagerie, stage, 16);
    expect(Math.abs(actorOf(soon, species).gaze.x - goal.x)).toBeLessThan(0.02);
    expect(Math.abs(actorOf(soon, species).gaze.y - goal.y)).toBeLessThan(0.02);
    const settled = run(menagerie, soon, 48);
    expect(actorOf(settled, species).gaze).toEqual({ x: goal.x, y: goal.y, vx: 0, vy: 0 });
    expect(actorOf(settled, species).x).toBe(550);
    expect(actorOf(settled, species).activity).toBe("idle");
  });

  it("draws the pupils towards the pointer inside the eye — as far as the outline of the white —, mirrored with the actor", () => {
    for (const facing of [1, -1] as const) {
      let stage = advance(menagerie, alone(company, species, { facing }), [{ kind: "hushed", quiet: true }]);
      stage = run(menagerie, advance(menagerie, stage, [{ kind: "pointed", over: "free", x: 250, y: 150 }]), 64);
      const actor = actorOf(stage, species);
      expect(actor.facing).toBe(facing);
      const frame = frameOf(menagerie, stage).actors[0]!;
      const kind = kindOf(menagerie, species);
      expect(frame.eyes).toHaveLength(kind.face.eyes.length);
      frame.eyes.forEach((eye, index) => {
        const span = kind.face.eyes[index]!.radius - kind.face.eyes[index]!.pupil - 0.25;
        expect(eye.x * facing).toBeCloseTo(actor.gaze.x * span, 12);
        expect(eye.y).toBeCloseTo(actor.gaze.y * span, 12);
        expect(Math.sqrt(eye.x * eye.x + eye.y * eye.y) + kind.face.eyes[index]!.pupil).toBeLessThan(kind.face.eyes[index]!.radius);
        expect(Math.sqrt(eye.x * eye.x + eye.y * eye.y)).toBeGreaterThan(0.7 * span);
        expect(eye.x * facing).toBeLessThan(0);
      });
    }
  });

  it("loses interest four seconds after the pointer last moved and looks ahead again, and at once when the pointer leaves", () => {
    let stage = alone(company, species, { facing: 1 });
    stage = advance(menagerie, stage, [{ kind: "pointed", over: "free", x: 900, y: 300 }]);
    stage = run(menagerie, stage, 4 * SECOND - 1);
    expect(actorOf(stage, species).gaze.x).toBeGreaterThan(0.5);
    stage = run(menagerie, stage, SECOND);
    expect(actorOf(stage, species).gaze).toEqual({ x: 0.3, y: 0, vx: 0, vy: 0 });
    stage = run(menagerie, advance(menagerie, stage, [{ kind: "pointed", over: "free", x: 900, y: 300 }]), SECOND);
    expect(actorOf(stage, species).gaze.x).toBeGreaterThan(0.5);
    stage = run(menagerie, advance(menagerie, stage, [{ kind: "unpointed" }]), SECOND);
    expect(actorOf(stage, species).gaze).toEqual({ x: 0.3, y: 0, vx: 0, vy: 0 });
  });

  it("looks where it goes while it walks, not at the pointer", () => {
    let stage = alone(company, species, { facing: 1, activity: "walk", goal: 700 });
    stage = advance(menagerie, stage, [{ kind: "pointed", over: "free", x: 100, y: 100 }]);
    run(menagerie, stage, 64, (after) => {
      if (actorOf(after, species).activity === "walk") expect(actorOf(after, species).gaze.x).toBeGreaterThan(0);
    });
  });

  it("looks at the nearest of the things worth a glance when no pointer moves", () => {
    let stage = alone(company, species, { facing: 1 });
    stage = run(menagerie, advance(menagerie, stage, [{ kind: "glanced", points: [{ x: 100, y: 380 }, { x: 760, y: 380 }] }]), SECOND);
    expect(actorOf(stage, species).gaze.x).toBeGreaterThan(0.5);
    stage = run(menagerie, advance(menagerie, stage, [{ kind: "glanced", points: [{ x: 100, y: 380 }] }]), SECOND);
    expect(actorOf(stage, species).gaze.x).toBeLessThan(-0.5);
    stage = run(menagerie, advance(menagerie, stage, [{ kind: "pointed", over: "free", x: 1200, y: 380 }]), SECOND);
    expect(actorOf(stage, species).gaze.x).toBeGreaterThan(0.5);
  });

  it("keeps the eyes shut and still while asleep, and wakes when the pointer comes close", () => {
    let stage = alone(company, species, { activity: "sleep" });
    stage = run(menagerie, advance(menagerie, stage, [{ kind: "pointed", over: "free", x: 1200, y: 100 }]), SECOND);
    expect(actorOf(stage, species).activity).toBe("sleep");
    expect(actorOf(stage, species).gaze).toEqual({ x: 0, y: 0, vx: 0, vy: 0 });
    for (const eye of frameOf(menagerie, stage).actors[0]!.eyes) expect(eye).toEqual({ x: 0, y: 0, lid: 1 });
    stage = run(menagerie, advance(menagerie, stage, [{ kind: "pointed", over: "free", x: 560, y: 380 }]), 1);
    expect(actorOf(stage, species).activity).toBe("idle");
  });
});

describe.each(COMPANIES)("blinking in $name", (company) => {
  const menagerie = company.menagerie;
  const species = company.walker;

  it("blinks every two to six seconds, sometimes twice, with both eyes, and is open in between — as far as its mood leaves its lids open", () => {
    const begun: number[] = [];
    let shut = 0;
    let open = 0;
    let opened = true;
    const stage = craft(alone(company, species), species, { blink: 200 });
    const seconds = sampled(30, 120, 600);
    const minute = seconds * SECOND;
    const kind = kindOf(menagerie, species);
    run(menagerie, { ...stage, tick: 0 }, minute, (after) => {
      const eyes = frameOf(menagerie, after).actors[0]!.eyes;
      const resting = faceOf(settled(actorOf(after, species).feeling, kind.mood, after.tick)).lid;
      for (const eye of eyes) expect(eye.lid).toBe(eyes[0]!.lid);
      expect(eyes[0]!.lid).toBeGreaterThanOrEqual(resting);
      if (eyes[0]!.lid > resting && opened) begun.push(after.tick);
      opened = eyes[0]!.lid === resting;
      if (opened) open++;
      if (eyes[0]!.lid >= 0.99) shut++;
    });
    expect(begun.length).toBeGreaterThanOrEqual(Math.floor(seconds / 6.2));
    expect(begun.length).toBeLessThanOrEqual(seconds / 2 + 20);
    expect(shut).toBeGreaterThanOrEqual(begun.length);
    expect(open / minute).toBeGreaterThan(0.85);
    for (let index = 1; index < begun.length; index++) {
      const gap = begun[index]! - begun[index - 1]!;
      expect(gap === BLINK_TICKS + 7 || (gap >= BLINK_TICKS + 2 * SECOND && gap < BLINK_TICKS + 6 * SECOND), `gap ${gap}`).toBe(true);
    }
  });

  it("gives every actor a rhythm of its own", () => {
    const everyone = menagerie.species.map((kind) => kind.id);
    const rhythms: Record<string, number[]> = Object.fromEntries(everyone.map((id) => [id, []]));
    run(menagerie, staged(menagerie, everyone, { surfaces: [FLOOR] }), 60 * SECOND, (after, before) => {
      for (const actor of after.actors) if (actor.blink !== actorOf(before, actor.species).blink) rhythms[actor.species]!.push(actor.blink);
    });
    expect(new Set(everyone.map((id) => rhythms[id]!.join(" "))).size).toBe(everyone.length);
    for (const id of everyone) expect(rhythms[id]!.length).toBeGreaterThan(5);
  });
});
