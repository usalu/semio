/** 🎪️ Unit suite of the stage: everything the owner asked of the pets, checked on the fold itself — slightly active by default, eyes that follow the pointer while standing still, blinking, walking along perches and riding a surface that moves, falling when a perch vanishes and landing, encounters that are affectionate for friends and small disputes (squabble → sulk → mending) for rivals, quiet and still, summon and leave, pokes, determinism, and the rate and wake a frame reports.
 *
 * Two menageries are played: the sample menagerie of the schema-conformance vectors (a walker, a hopper, a floater)
 * and the troupe of the stage-trace vectors (five blobs). Where a test needs a precise situation it writes the state
 * of an actor directly — the stage is a pure fold over plain data, so any valid state is a fair starting point.
 *
 * @see ../../🟦️.ts — the implementation under test
 * @see ../../../../🧫️fixtures/🧬️schema-conformance/🔣️.json — the sample menagerie
 * @see ../../../../🧫️fixtures/🎪️stage-trace/🔣️.json — the trace troupe
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { ACTIVITIES, TICKS_PER_SECOND, type Activity, type Actor, type Frame, type Menagerie, type PetMode, type Rect, type Slug, type Species, type Stage, type StageEvent, type Surface } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { BLINK_TICKS } from "../../../🎞️animation/🟦️.ts";
import { perchAt } from "../../../🏞️terrain/🟦️.ts";
import { lookOffset, restPose, solveRig } from "../../../🦴️rig/🟦️.ts";
import { menagerieIssues } from "../../../✅️validation/🟦️.ts";
import { ENCOUNTERS, MODE_LIMITS, followersOf, moodOf } from "../../../🧠️behavior/🟦️.ts";
import { advance, frameOf, openStage } from "../../🟦️.ts";

const HERE = dirname(fileURLToPath(import.meta.url));
const SECOND = TICKS_PER_SECOND;
const WIDTH = 1280;
const HEIGHT = 720;
const FLOOR: Surface = { id: "floor", x0: 0, x1: WIDTH, y: HEIGHT };
const CARD: Surface = { id: "card", x0: 300, x1: 800, y: 400 };
const FAR = 100000000;
const MEET = 6;

/** 🧫️ One committed fixture of the product. */
function fixture<T>(name: string): T {
  return JSON.parse(readFileSync(resolve(HERE, "../../../../🧫️fixtures", name, "🔣️.json"), "utf8")) as T;
}

type Company = { readonly name: string; readonly menagerie: Menagerie; readonly walker: Slug; readonly hopper: Slug; readonly floater: Slug; readonly friends: readonly [Slug, Slug]; readonly rivals: readonly [Slug, Slug]; readonly strangers: readonly [Slug, Slug] };

const SAMPLE: Company = { name: "the sample menagerie", menagerie: fixture<{ menagerie: Menagerie }>("🧬️schema-conformance").menagerie, walker: "blobby", hopper: "hoppy", floater: "floaty", friends: ["blobby", "hoppy"], rivals: ["blobby", "floaty"], strangers: ["hoppy", "floaty"] };
const TROUPE: Company = { name: "the trace troupe", menagerie: fixture<{ menagerie: Menagerie }>("🎪️stage-trace").menagerie, walker: "mossy", hopper: "sparky", floater: "misty", friends: ["mossy", "sparky"], rivals: ["mossy", "thorny"], strangers: ["mossy", "pebble"] };
const COMPANIES = [SAMPLE, TROUPE];

/** ⏭️ Time passing. */
function ticked(ticks = 1): StageEvent {
  return { kind: "ticked", ticks };
}

/** 🗺️ A survey of the stage. */
function surveyed(surfaces: readonly Surface[], keepouts: readonly Rect[] = [], width = WIDTH, height = HEIGHT): StageEvent {
  return { kind: "surveyed", width, height, surfaces, keepouts };
}

/** 🧬️ A species of a menagerie. */
function kindOf(menagerie: Menagerie, id: Slug): Species {
  return menagerie.species.find((species) => species.id === id)!;
}

/** 🎈️ The hover of a species. */
function hoverOf(kind: Species): number {
  return kind.locomotion.gait === "float" ? (kind.locomotion.hover ?? 0) : 0;
}

/** 🦘️ How far one hop carries a species that walks in hops (its speed × the length of its hop clip), 0 for every other gait. */
function hopOf(kind: Species): number {
  if (kind.locomotion.gait !== "hop") return 0;
  const clip = kind.clips.find((candidate) => candidate.id === (kind.repertoire.walk ?? kind.repertoire.hop ?? [])[0]);
  return clip === undefined ? 0 : (kind.locomotion.speed * Math.max(Math.floor(clip.seconds * SECOND + 0.5), 1)) / SECOND;
}

/** 🧸️ The actor of a species on a stage. */
function actorOf(stage: Stage, species: Slug): Actor {
  const actor = stage.actors.find((candidate) => candidate.species === species);
  if (actor === undefined) throw new Error(`${species} is not on stage`);
  return actor;
}

/** 🎬️ A stage on which species were summoned onto surveyed surfaces. */
function staged(menagerie: Menagerie, species: readonly Slug[], options: { seed?: number; mode?: PetMode; surfaces?: readonly Surface[]; keepouts?: readonly Rect[] } = {}): Stage {
  return advance(menagerie, openStage(options.seed ?? 1), [{ kind: "tuned", mode: options.mode ?? "calm" }, surveyed(options.surfaces ?? [CARD, FLOOR], options.keepouts ?? []), { kind: "summoned", species }]);
}

/** 🏃️ A stage after `ticks` single ticks; `watch` sees every stage with the one before it. */
function run(menagerie: Menagerie, stage: Stage, ticks: number, watch?: (after: Stage, before: Stage) => void): Stage {
  let current = stage;
  for (let tick = 0; tick < ticks; tick++) {
    const next = advance(menagerie, current, [ticked()]);
    if (watch !== undefined) watch(next, current);
    current = next;
  }
  return current;
}

/** ✍️ A stage in which the actor of a species is written anew. */
function craft(stage: Stage, species: Slug, changes: Partial<Actor>): Stage {
  return { ...stage, actors: stage.actors.map((actor) => (actor.species === species ? { ...actor, ...changes } : actor)) };
}

/** 🪑️ A stage in which an actor stands whole and idle at `x` on the card for as long as the test runs, without a blink in sight. */
function seated(menagerie: Menagerie, stage: Stage, species: Slug, x: number, changes: Partial<Actor> = {}): Stage {
  return craft(stage, species, { perch: CARD.id, x, y: CARD.y - hoverOf(kindOf(menagerie, species)), vx: 0, vy: 0, goal: x, activity: "idle", since: stage.tick, until: stage.tick + FAR, blink: stage.tick + FAR, partner: null, opacity: 1, leaving: false, mood: moodOf("idle"), gaze: { x: 0, y: 0, vx: 0, vy: 0 }, ...changes });
}

/** 🧍️ A stage with one actor seated on the card. */
function alone(company: Company, species: Slug, changes: Partial<Actor> = {}): Stage {
  return seated(company.menagerie, staged(company.menagerie, [species]), species, 550, changes);
}

/** 🧊️ An object and everything in it, frozen. */
function frozen<T>(value: T): T {
  if (typeof value === "object" && value !== null && !Object.isFrozen(value)) {
    Object.freeze(value);
    for (const inner of Object.values(value)) frozen(inner);
  }
  return value;
}

/** 👁️ Where an actor's eyes are, as the stage sees them. */
function eyeOf(menagerie: Menagerie, actor: Actor): { x: number; y: number } {
  return { x: actor.x, y: actor.y - kindOf(menagerie, actor.species).size.height * 0.6 };
}

/** 📦️ Whether the box of a grounded actor touches a keep-out. */
function covers(menagerie: Menagerie, actor: Actor, keepout: Rect): boolean {
  const kind = kindOf(menagerie, actor.species);
  return Math.max(actor.x - kind.size.width / 2, keepout.x) < Math.min(actor.x + kind.size.width / 2, keepout.x + keepout.width) && Math.max(actor.y - kind.size.height, keepout.y) < Math.min(actor.y, keepout.y + keepout.height);
}

describe("openStage", () => {
  it("opens empty, calm, not quiet, at tick 0", () => {
    expect(openStage(7)).toEqual({ seed: 7, tick: 0, mode: "calm", quiet: false, width: 0, height: 0, pointer: null, pointed: 0, glances: [], surfaces: [], keepouts: [], perches: [], wanted: [], actors: [], rapports: [], met: 0, draws: 0 });
    expect(openStage(-1).seed).toBe(4294967295);
    expect(frameOf(SAMPLE.menagerie, openStage(7))).toEqual({ tick: 0, actors: [], rate: 0, wake: null });
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
      20: [{ kind: "poked", x: 500, y: 385 }],
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
    stage = advance(menagerie, stage, [{ kind: "pointed", x: pointer.x, y: pointer.y }]);
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
      stage = run(menagerie, advance(menagerie, stage, [{ kind: "pointed", x: 250, y: 150 }]), 64);
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
    stage = advance(menagerie, stage, [{ kind: "pointed", x: 900, y: 300 }]);
    stage = run(menagerie, stage, 4 * SECOND - 1);
    expect(actorOf(stage, species).gaze.x).toBeGreaterThan(0.5);
    stage = run(menagerie, stage, SECOND);
    expect(actorOf(stage, species).gaze).toEqual({ x: 0.3, y: 0, vx: 0, vy: 0 });
    stage = run(menagerie, advance(menagerie, stage, [{ kind: "pointed", x: 900, y: 300 }]), SECOND);
    expect(actorOf(stage, species).gaze.x).toBeGreaterThan(0.5);
    stage = run(menagerie, advance(menagerie, stage, [{ kind: "unpointed" }]), SECOND);
    expect(actorOf(stage, species).gaze).toEqual({ x: 0.3, y: 0, vx: 0, vy: 0 });
  });

  it("looks where it goes while it walks, not at the pointer", () => {
    let stage = alone(company, species, { facing: 1, activity: "walk", goal: 700 });
    stage = advance(menagerie, stage, [{ kind: "pointed", x: 100, y: 100 }]);
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
    stage = run(menagerie, advance(menagerie, stage, [{ kind: "pointed", x: 1200, y: 380 }]), SECOND);
    expect(actorOf(stage, species).gaze.x).toBeGreaterThan(0.5);
  });

  it("keeps the eyes shut and still while asleep, and wakes when the pointer comes close", () => {
    let stage = alone(company, species, { activity: "sleep" });
    stage = run(menagerie, advance(menagerie, stage, [{ kind: "pointed", x: 1200, y: 100 }]), SECOND);
    expect(actorOf(stage, species).activity).toBe("sleep");
    expect(actorOf(stage, species).gaze).toEqual({ x: 0, y: 0, vx: 0, vy: 0 });
    for (const eye of frameOf(menagerie, stage).actors[0]!.eyes) expect(eye).toEqual({ x: 0, y: 0, lid: 1 });
    stage = run(menagerie, advance(menagerie, stage, [{ kind: "pointed", x: 560, y: 380 }]), 1);
    expect(actorOf(stage, species).activity).toBe("idle");
  });
});

describe.each(COMPANIES)("blinking in $name", (company) => {
  const menagerie = company.menagerie;
  const species = company.walker;

  it("blinks every two to six seconds, sometimes twice, with both eyes, and is open in between", () => {
    const begun: number[] = [];
    let shut = 0;
    let open = 0;
    let lid = 0;
    const stage = craft(alone(company, species), species, { blink: 200 });
    const seconds = sampled(30, 120, 600);
    const minute = seconds * SECOND;
    run(menagerie, { ...stage, tick: 0 }, minute, (after) => {
      const eyes = frameOf(menagerie, after).actors[0]!.eyes;
      for (const eye of eyes) expect(eye.lid).toBe(eyes[0]!.lid);
      if (eyes[0]!.lid > 0 && lid === 0) begun.push(after.tick);
      lid = eyes[0]!.lid;
      if (lid === 0) open++;
      if (lid >= 0.99) shut++;
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

describe.each(COMPANIES)("walking and riding in $name", (company) => {
  const menagerie = company.menagerie;
  const species = company.walker;
  const kind = kindOf(menagerie, species);

  it("walks along its perch at its own speed to its goal and rests there", () => {
    const stage = alone(company, species, { activity: "walk", goal: 700, facing: 1 });
    let ticks = 0;
    let arrived: Stage | null = null;
    run(menagerie, stage, 20 * SECOND, (after, before) => {
      if (arrived !== null) return;
      const actor = actorOf(after, species);
      const earlier = actorOf(before, species);
      if (earlier.activity !== "walk") return;
      ticks++;
      expect(actor.x - earlier.x).toBeGreaterThan(0);
      expect(actor.x - earlier.x).toBeLessThanOrEqual(kind.locomotion.speed / SECOND + 1e-9);
      expect(actor.y).toBe(CARD.y - hoverOf(kind));
      expect(actor.perch).toBe(CARD.id);
      expect(frameOf(menagerie, before).rate).toBe(64);
      if (actor.activity !== "walk" && arrived === null) arrived = after;
    });
    expect(arrived).not.toBeNull();
    expect(actorOf(arrived!, species).x).toBe(700);
    expect(actorOf(arrived!, species).activity).toBe("idle");
    expect(ticks).toBe(Math.ceil((150 * SECOND) / kind.locomotion.speed));
  });

  it("chooses goals on its own perch, at least three quarters of a body away and within the stroll of the mode", () => {
    let walks = 0;
    for (const mode of ["calm", "lively"] as const) {
      for (let seed = 1; seed <= sampled(2, 4, 8); seed++) {
        run(menagerie, staged(menagerie, [species], { seed, mode, surfaces: [FLOOR] }), sampled(120, 300, 300) * SECOND, (after, before) => {
          const actor = actorOf(after, species);
          const earlier = actorOf(before, species);
          if (actor.x !== earlier.x) expect(actor.facing, "nobody walks backwards").toBe(actor.x > earlier.x ? 1 : -1);
          if (actor.activity !== "walk" || earlier.activity === "walk") return;
          walks++;
          const way = Math.abs(actor.goal - earlier.x);
          expect(way).toBeGreaterThanOrEqual(0.75 * kind.size.width - 1e-9);
          expect(way).toBeLessThanOrEqual(MODE_LIMITS[mode].stroll * kind.size.width + 1e-9);
          expect(actor.goal).toBeGreaterThanOrEqual(kind.size.width / 2);
          expect(actor.goal).toBeLessThanOrEqual(WIDTH - kind.size.width / 2);
        });
      }
    }
    expect(walks).toBeGreaterThan(sampled(2, 5, 10));
  });

  it("rides a surface that moves, and stays put when nothing moved", () => {
    const stage = alone(company, species);
    const moved = advance(menagerie, stage, [surveyed([{ ...CARD, x0: CARD.x0 + 50, x1: CARD.x1 + 50, y: CARD.y - 20 }, FLOOR])]);
    expect(actorOf(moved, species)).toMatchObject({ x: 600, y: CARD.y - 20 - hoverOf(kind), perch: CARD.id, activity: "idle", opacity: 1 });
    const again = advance(menagerie, moved, [surveyed([{ ...CARD, x0: CARD.x0 + 50, x1: CARD.x1 + 50, y: CARD.y - 20 }, FLOOR])]);
    expect(again.actors).toEqual(moved.actors);
    expect(again.draws).toBe(moved.draws);
    let scrolled = stage;
    for (let step = 1; step <= 40; step++) {
      scrolled = advance(menagerie, scrolled, [ticked(8), surveyed([{ ...CARD, y: CARD.y - 3 * step }, FLOOR])]);
      expect(actorOf(scrolled, species).y).toBe(CARD.y - 3 * step - hoverOf(kind));
      expect(actorOf(scrolled, species).x).toBe(550);
    }
  });

  it("carries a walker and its goal along with the surface", () => {
    const stage = alone(company, species, { activity: "walk", goal: 700, facing: 1 });
    const moved = advance(menagerie, stage, [surveyed([{ ...CARD, x0: CARD.x0 - 100, x1: CARD.x1 - 100 }, FLOOR])]);
    expect(actorOf(moved, species)).toMatchObject({ x: 450, goal: 600, activity: "walk" });
  });

  it("is held on its perch when the perch shrinks, and steps aside for a keep-out", () => {
    const stage = alone(company, species);
    const shrunk = advance(menagerie, stage, [surveyed([{ ...CARD, x1: 560 }, FLOOR])]);
    expect(actorOf(shrunk, species)).toMatchObject({ x: 560 - kind.size.width / 2, perch: CARD.id, activity: "idle", opacity: 1 });
    const keepout: Rect = { x: 520, y: 380, width: 60, height: 20 };
    const blocked = advance(menagerie, stage, [surveyed([CARD, FLOOR], [keepout])]);
    expect(covers(menagerie, actorOf(blocked, species), keepout)).toBe(false);
    expect(actorOf(blocked, species).perch).toBe(CARD.id);
    expect(Math.abs(actorOf(blocked, species).x - 550)).toBeLessThanOrEqual(30 + kind.size.width);
    const shoved = advance(menagerie, stage, [surveyed([CARD, FLOOR], [{ x: 400, y: 380, width: 400, height: 20 }])]);
    expect(actorOf(shoved, species)).toMatchObject({ perch: CARD.id, opacity: 0 });
    expect(actorOf(shoved, species).x).toBeLessThanOrEqual(400 - kind.size.width / 2);
  });
});

describe.each(COMPANIES)("falling and landing in $name", (company) => {
  const menagerie = company.menagerie;

  it.each([company.walker, company.floater])("lets %s fall when its perch vanishes, land on the floor and rest", (species) => {
    const kind = kindOf(menagerie, species);
    let stage = advance(menagerie, alone(company, species), [surveyed([FLOOR])]);
    expect(actorOf(stage, species)).toMatchObject({ activity: "fall", perch: null, x: 550, vy: 0 });
    let fallen = 0;
    let landed: Stage | null = null;
    stage = run(menagerie, stage, 3 * SECOND, (after, before) => {
      const actor = actorOf(after, species);
      const earlier = actorOf(before, species);
      if (earlier.activity !== "fall") return;
      fallen++;
      expect(frameOf(menagerie, before).rate).toBe(64);
      if (actor.activity === "fall") expect(actor.y).toBeGreaterThan(earlier.y);
      else landed = after;
    });
    expect(landed).not.toBeNull();
    expect(actorOf(landed!, species)).toMatchObject({ activity: "land", perch: FLOOR.id, x: 550, y: HEIGHT - hoverOf(kind), vy: 0 });
    expect(fallen).toBeGreaterThan(30);
    expect(fallen).toBeLessThan(50);
    expect(actorOf(stage, species).activity).toBe("idle");
    expect(actorOf(stage, species).perch).toBe(FLOOR.id);
  });

  it("lands on the highest perch in the way", () => {
    const species = company.walker;
    const shelf: Surface = { id: "shelf", x0: 400, x1: 700, y: 520 };
    const stage = run(menagerie, advance(menagerie, alone(company, species), [surveyed([shelf, FLOOR])]), 2 * SECOND);
    expect(actorOf(stage, species)).toMatchObject({ perch: "shelf", y: 520 });
  });

  it("stays on a surface that was replaced by its like", () => {
    const species = company.walker;
    const stage = advance(menagerie, alone(company, species), [surveyed([{ ...CARD, id: "card-again" }, FLOOR])]);
    expect(actorOf(stage, species)).toMatchObject({ perch: "card-again", activity: "idle", x: 550, y: CARD.y, opacity: 1 });
  });

  it("is gone when it falls out of the stage, and arrives anew once a perch exists", () => {
    const species = company.walker;
    let stage = run(menagerie, advance(menagerie, alone(company, species), [surveyed([])]), 3 * SECOND);
    expect(stage.actors).toEqual([]);
    expect(stage.wanted).toEqual([species]);
    stage = advance(menagerie, stage, [surveyed([FLOOR])]);
    expect(actorOf(stage, species)).toMatchObject({ perch: FLOOR.id, activity: "idle", opacity: 0 });
  });

  it("is gone when the floor under its fall is a keep-out, and arrives anew beside it within a second", () => {
    const species = company.walker;
    const stage = run(menagerie, advance(menagerie, alone(company, species), [surveyed([FLOOR], [{ x: 400, y: 680, width: 300, height: 40 }])]), 3 * SECOND);
    const actor = actorOf(stage, species);
    expect(actor.perch).toBe(FLOOR.id);
    expect(actor.x <= 400 - kindOf(menagerie, species).size.width / 2 || actor.x >= 700 + kindOf(menagerie, species).size.width / 2).toBe(true);
    expect(actor.opacity).toBe(1);
  });
});

describe.each(COMPANIES)("encounters in $name", (company) => {
  const menagerie = company.menagerie;

  /** 💑️ A stage on which two species stand on the card, one body apart, each waiting for the other. */
  function waiting(pair: readonly [Slug, Slug], draws: number): Stage {
    const apart = (kindOf(menagerie, pair[0]).size.width + kindOf(menagerie, pair[1]).size.width) / 2 + 2;
    let stage = staged(menagerie, pair, { mode: "lively" });
    stage = seated(menagerie, stage, pair[0], 500, { partner: pair[1], facing: 1 });
    stage = seated(menagerie, stage, pair[1], 500 + apart, { partner: pair[0], facing: -1 });
    return { ...stage, tick: 2000, draws };
  }

  const MEETINGS = sampled(80, 200, 2000);
  const MOSTLY = (11 * MEETINGS) / 20;

  /** 🧮️ How the encounters of a pair turn out: 200 of them (80 at the fundamental level, 2000 at the exhaustive one), of which more than 11 in 20 count as "mostly". */
  function outcomes(pair: readonly [Slug, Slug]): Record<string, number> {
    const counts: Record<string, number> = { greet: 0, cuddle: 0, squabble: 0 };
    for (let draws = 0; draws < MEETINGS; draws++) counts[actorOf(run(menagerie, waiting(pair, draws), 1), pair[0]).activity]!++;
    return counts;
  }

  it("is affectionate between friends: mostly a cuddle, never a squabble", () => {
    const counts = outcomes(company.friends);
    expect(counts.cuddle).toBeGreaterThan(MOSTLY);
    expect(counts.squabble).toBe(0);
    expect(counts.greet).toBeGreaterThan(0);
  });

  it("is a small dispute between rivals: mostly a squabble, never a cuddle", () => {
    const counts = outcomes(company.rivals);
    expect(counts.squabble).toBeGreaterThan(MOSTLY);
    expect(counts.cuddle).toBe(0);
  });

  it("is mostly a greeting between everyone else", () => {
    expect(outcomes(company.strangers).greet).toBeGreaterThan(MOSTLY);
  });

  it("has both partners act together for the same span, facing each other, and remembers it", () => {
    const [first, second] = company.friends;
    const stage = run(menagerie, waiting(company.friends, 3), 1);
    const one = actorOf(stage, first);
    const two = actorOf(stage, second);
    expect(ENCOUNTERS).toContain(one.activity);
    expect(two.activity).toBe(one.activity);
    expect(two.until).toBe(one.until);
    expect(one.until - stage.tick).toBeGreaterThanOrEqual(2 * SECOND);
    expect(one.until - stage.tick).toBeLessThan(5 * SECOND);
    expect([one.facing, two.facing]).toEqual([1, -1]);
    expect([one.partner, two.partner]).toEqual([second, first]);
    expect(stage.met).toBe(stage.tick);
    expect(stage.rapports).toHaveLength(1);
    expect(stage.rapports[0]!.drift).toBe(one.activity === "cuddle" ? 0.1 : 0.05);
    const after = run(menagerie, stage, 5 * SECOND);
    expect([actorOf(after, first).activity, actorOf(after, second).activity]).toEqual(["idle", "idle"]);
    expect([actorOf(after, first).partner, actorOf(after, second).partner]).toEqual([null, null]);
  });

  it("turns a squabble into a sulk with the backs turned, and mends the rapport when the sulk is over", () => {
    const [first, second] = company.rivals;
    let draws = 0;
    while (actorOf(run(menagerie, waiting(company.rivals, draws), 1), first).activity !== "squabble") draws++;
    let stage = run(menagerie, waiting(company.rivals, draws), 1);
    expect(stage.rapports).toEqual([{ between: [actorOf(stage, first).species, actorOf(stage, second).species].sort((a, b) => menagerie.species.findIndex((kind) => kind.id === a) - menagerie.species.findIndex((kind) => kind.id === b)), drift: -0.15 }]);
    const story: Record<string, Activity[]> = { [first]: ["squabble"], [second]: ["squabble"] };
    let sulked = false;
    stage = run(menagerie, stage, 14 * SECOND, (after) => {
      for (const species of [first, second]) {
        const actor = actorOf(after, species);
        if (story[species]!.at(-1) !== actor.activity) story[species]!.push(actor.activity);
      }
      const one = actorOf(after, first);
      const two = actorOf(after, second);
      if (one.activity === "squabble") expect(one.mood).toBeLessThan(0.3);
      if (one.activity === "sulk" && two.activity === "sulk") {
        sulked = true;
        if (after.tick - one.since > 8 && after.tick - two.since > 8) expect([one.facing, two.facing]).toEqual([-1, 1]);
        expect(after.rapports[0]!.drift).toBe(-0.15);
      }
    });
    expect(sulked).toBe(true);
    expect(story[first]!.slice(0, 3)).toEqual(["squabble", "sulk", "idle"]);
    expect(story[second]!.slice(0, 3)).toEqual(["squabble", "sulk", "idle"]);
    expect(stage.rapports).toHaveLength(1);
    expect(stage.rapports[0]!.drift).toBeGreaterThan(-0.05);
    expect(stage.rapports[0]!.drift).toBeLessThan(-0.047);
    expect([actorOf(stage, first).partner, actorOf(stage, second).partner]).toEqual([null, null]);
  });

  it("brings two actors together by walking, one pair at a time, and both walk only when the mode has room for two movers", () => {
    const everyone = menagerie.species.map((kind) => kind.id);
    for (const mode of ["calm", "lively"] as const) {
      let met = 0;
      let approaches = 0;
      let partners = 0;
      let movers = 0;
      const seeds = sampled(1, 3, 8);
      const span = sampled(120, 480, 480) * SECOND;
      const acts = (actor: Actor): boolean => actor.activity === "greet" || actor.activity === "cuddle" || actor.activity === "squabble";
      for (let seed = 1; seed <= seeds; seed++) {
        run(menagerie, staged(menagerie, everyone, { seed, mode, surfaces: [{ ...CARD, x0: 200, x1: 700 }] }), span, (after, before) => {
          let partnered = 0;
          let walking = 0;
          let acting = 0;
          for (const actor of after.actors) {
            if (actor.partner !== null) partnered++;
            if ((actor.activity === "walk" || actor.activity === "hop") && !actor.leaving) walking++;
            if (actor.partner !== null && acts(actor)) acting++;
          }
          partners = Math.max(partners, partnered);
          movers = Math.max(movers, walking);
          if (partnered === 2 && before.actors.every((actor) => actor.partner === null)) approaches++;
          if (acting !== 2 || before.actors.some(acts)) return;
          met++;
          const [one, two] = after.actors.filter((actor) => actor.partner !== null) as [Actor, Actor];
          const apart = Math.abs(one.x - two.x) - (kindOf(menagerie, one.species).size.width + kindOf(menagerie, two.species).size.width) / 2;
          expect(apart).toBeGreaterThanOrEqual(MEET - 1 / 128);
          expect(apart).toBeLessThanOrEqual(MEET + Math.max(hopOf(kindOf(menagerie, one.species)), hopOf(kindOf(menagerie, two.species))) + 1);
        });
      }
      expect(partners, mode).toBe(2);
      expect(movers, mode).toBeLessThanOrEqual(MODE_LIMITS[mode].movers);
      expect(approaches, mode).toBeGreaterThanOrEqual(met);
      expect(met, mode).toBeGreaterThanOrEqual(mode === "calm" ? seeds : 2 * seeds);
      if (mode === "calm") expect(met).toBeLessThanOrEqual(seeds * Math.ceil(span / MODE_LIMITS.calm.encounterGap));
    }
  });

  it("waits out the gap of the mode between two encounters, and the warm-up before the first", () => {
    const everyone = menagerie.species.map((kind) => kind.id);
    for (const mode of ["calm", "lively"] as const) {
      for (const seed of sampled([5], [5], [5, 1, 2, 3])) {
        const begun: number[] = [];
        run(menagerie, staged(menagerie, everyone, { seed, mode, surfaces: [{ ...CARD, x0: 200, x1: 700 }] }), sampled(3 * MODE_LIMITS[mode].encounterGap, 600 * SECOND, 600 * SECOND), (after, before) => {
          if (before.actors.every((actor) => actor.partner === null) && after.actors.some((actor) => actor.partner !== null)) begun.push(after.tick);
        });
        expect(begun.length).toBeGreaterThan(1);
        expect(begun[0]).toBeGreaterThanOrEqual(20 * SECOND);
        for (let index = 1; index < begun.length; index++) expect(begun[index]! - begun[index - 1]!).toBeGreaterThanOrEqual(MODE_LIMITS[mode].encounterGap);
      }
    }
  });
});

describe.each(COMPANIES)("a time of concentration in $name", (company) => {
  const menagerie = company.menagerie;
  const everyone = menagerie.species.map((kind) => kind.id);

  it("lets everyone finish what they do, then only idle and sleep, without a single encounter", () => {
    let stage = run(menagerie, staged(menagerie, everyone, { seed: 2, mode: "lively" }), 45 * SECOND);
    stage = advance(menagerie, stage, [{ kind: "hushed", quiet: true }]);
    const hushed = stage.tick;
    const partnered = stage.actors.some((actor) => actor.partner !== null);
    const noise: string[] = [];
    stage = run(menagerie, stage, sampled(60, 300, 300) * SECOND, (after, before) => {
      for (let index = 0; index < after.actors.length; index++) {
        const actor = after.actors[index]!;
        const earlier = before.actors[index]!;
        const activity = actor.activity;
        const began = earlier.activity !== activity || earlier.since !== actor.since;
        if (began && earlier.partner === null && activity !== "idle" && activity !== "sleep" && activity !== "land") noise.push(`${actor.species} began ${activity} at ${after.tick}`);
        if (!partnered && actor.partner !== null) noise.push(`${actor.species} found a partner at ${after.tick}`);
        if (after.tick > hushed + 40 * SECOND && activity !== "idle" && activity !== "sleep") noise.push(`${actor.species} still ${activity} at ${after.tick}`);
      }
    });
    expect(noise).toEqual([]);
    expect(stage.quiet).toBe(true);
    stage = advance(menagerie, stage, [{ kind: "hushed", quiet: false }]);
    let lively = 0;
    run(menagerie, stage, sampled(30, 120, 120) * SECOND, (after) => {
      for (const actor of after.actors) if (actor.activity === "walk" || actor.activity === "fidget") lively++;
    });
    expect(lively).toBeGreaterThan(0);
  });

  it("lets the tired fall asleep sooner than at other times", () => {
    const asleep = (quiet: boolean): number => {
      let stage = staged(menagerie, everyone, { seed: 3 });
      for (const species of everyone) stage = craft(stage, species, { needs: { energy: 0.3, sociability: 0.5, curiosity: 0.5 } });
      stage = advance(menagerie, stage, [{ kind: "hushed", quiet }]);
      let sleeping = 0;
      run(menagerie, stage, sampled(40, 120, 120) * SECOND, (after) => {
        sleeping += after.actors.filter((actor) => actor.activity === "sleep").length;
      });
      return sleeping;
    };
    expect(asleep(true)).toBeGreaterThan(asleep(false));
  });
});

describe.each(COMPANIES)("a still stage of $name", (company) => {
  const menagerie = company.menagerie;
  const everyone = menagerie.species.map((kind) => kind.id);

  it("freezes every actor in idle at rest with open eyes and a centred gaze, and reports that nothing moves", () => {
    let stage = run(menagerie, staged(menagerie, everyone, { seed: 6, mode: "lively" }), 50 * SECOND);
    stage = advance(menagerie, stage, [{ kind: "pointed", x: 10, y: 10 }, ticked(20), { kind: "tuned", mode: "still" }]);
    expect(stage.actors.map((actor) => actor.species)).toEqual(everyone);
    for (const actor of stage.actors) {
      expect(actor).toMatchObject({ activity: "idle", clip: null, partner: null, opacity: 1, leaving: false, vx: 0, vy: 0, gaze: { x: 0, y: 0, vx: 0, vy: 0 } });
      expect(actor.perch).not.toBeNull();
    }
    const frame = frameOf(menagerie, stage);
    expect([frame.rate, frame.wake]).toEqual([0, null]);
    for (const drawn of frame.actors) {
      const kind = kindOf(menagerie, drawn.species);
      expect(drawn.bones).toEqual(solveRig(kind, restPose(kind)));
      for (const eye of drawn.eyes) expect(eye).toEqual({ x: 0, y: 0, lid: 0 });
    }
    const later = advance(menagerie, stage, [ticked(123456789), { kind: "poked", x: stage.actors[0]!.x, y: stage.actors[0]!.y - 10 }, { kind: "pointed", x: 600, y: 300 }, ticked(1000)]);
    expect(later.tick).toBe(stage.tick + 123456789 + 1000);
    expect(later.actors).toEqual(stage.actors);
    expect(frameOf(menagerie, later).actors).toEqual(frame.actors);
  });

  it("lets whoever was in the air arrive anew on a perch and drops whoever was leaving", () => {
    let stage = alone(company, company.walker);
    stage = advance(menagerie, stage, [surveyed([FLOOR]), ticked(5), { kind: "tuned", mode: "still" }]);
    expect(actorOf(stage, company.walker)).toMatchObject({ activity: "idle", perch: FLOOR.id, y: HEIGHT });
    let leaving = staged(menagerie, everyone, { seed: 2 });
    leaving = advance(menagerie, leaving, [ticked(100), { kind: "summoned", species: [everyone[0]!] }, { kind: "tuned", mode: "still" }]);
    expect(leaving.actors.map((actor) => actor.species)).toEqual([everyone[0]]);
  });

  it("swaps the company at once and keeps actors on their surfaces without any motion", () => {
    let stage = advance(menagerie, staged(menagerie, [everyone[0]!], { mode: "still" }), [ticked(50)]);
    expect(actorOf(stage, everyone[0]!).opacity).toBe(1);
    stage = advance(menagerie, stage, [{ kind: "summoned", species: [everyone[1]!] }]);
    expect(stage.actors.map((actor) => actor.species)).toEqual([everyone[1]]);
    expect(stage.actors[0]!.opacity).toBe(1);
    const perch = stage.actors[0]!.perch!;
    stage = advance(menagerie, stage, [surveyed([perch === CARD.id ? FLOOR : CARD])]);
    expect(stage.actors[0]).toMatchObject({ activity: "idle", opacity: 1, perch: perch === CARD.id ? FLOOR.id : CARD.id });
    expect(frameOf(menagerie, stage).rate).toBe(0);
  });

  it("comes back to life when the mode changes", () => {
    let stage = advance(menagerie, staged(menagerie, everyone, { mode: "still" }), [ticked(5000), { kind: "tuned", mode: "calm" }]);
    for (const actor of stage.actors) {
      expect(actor.until).toBeGreaterThanOrEqual(stage.tick + MODE_LIMITS.calm.idleLow);
      expect(actor.blink).toBeGreaterThan(stage.tick);
      expect(actor.clip).not.toBeNull();
    }
    expect(frameOf(menagerie, stage).rate).toBeGreaterThan(0);
    let changes = 0;
    stage = run(menagerie, stage, sampled(30, 120, 120) * SECOND, (after, before) => {
      for (const actor of after.actors) if (before.actors.find((earlier) => earlier.species === actor.species)?.activity !== actor.activity) changes++;
    });
    expect(changes).toBeGreaterThan(0);
  });

  it("lets the walkers beyond the limit come to rest when lively turns calm", () => {
    const [first, second] = [everyone[0]!, everyone[1]!];
    let stage = staged(menagerie, [first, second], { mode: "lively", surfaces: [FLOOR] });
    stage = craft(craft(stage, first, { activity: "walk", goal: actorOf(stage, first).x + 5, until: stage.tick + FAR, opacity: 1 }), second, { activity: "walk", goal: actorOf(stage, second).x + 5, until: stage.tick + FAR, opacity: 1 });
    stage = advance(menagerie, stage, [{ kind: "tuned", mode: "calm" }]);
    expect([actorOf(stage, first).activity, actorOf(stage, second).activity]).toEqual(["walk", "idle"]);
  });
});

describe.each(COMPANIES)("summoning and leaving in $name", (company) => {
  const menagerie = company.menagerie;
  const everyone = menagerie.species.map((kind) => kind.id);

  it("lets whoever is no longer wanted walk off and fade out, and leaves the others alone", () => {
    const [stays, ...leave] = everyone;
    let stage = run(menagerie, staged(menagerie, everyone, { seed: 4 }), 5 * SECOND);
    const before = actorOf(stage, stays!);
    stage = advance(menagerie, stage, [{ kind: "summoned", species: [stays!] }]);
    expect(actorOf(stage, stays!)).toEqual(before);
    for (const species of leave) {
      expect(actorOf(stage, species).leaving).toBe(true);
      expect(["walk", "idle"]).toContain(actorOf(stage, species).activity);
    }
    const opacities: Record<string, number> = Object.fromEntries(leave.map((species) => [species, 1]));
    stage = run(menagerie, stage, 12 * SECOND, (after) => {
      for (const actor of after.actors) {
        if (!actor.leaving) continue;
        expect(actor.opacity).toBeLessThanOrEqual(opacities[actor.species]!);
        opacities[actor.species] = actor.opacity;
        expect(frameOf(menagerie, after).rate).toBe(64);
      }
    });
    expect(stage.actors.map((actor) => actor.species)).toEqual([stays]);
    expect(actorOf(stage, stays!).leaving).toBe(false);
  });

  it("walks to the nearer end of its perch when that is close, and fades where it stands when it is not", () => {
    const species = company.walker;
    const width = kindOf(menagerie, species).size.width;
    const near = advance(menagerie, seated(menagerie, staged(menagerie, [species]), species, CARD.x0 + width / 2 + 30), [{ kind: "summoned", species: [] }]);
    expect(actorOf(near, species)).toMatchObject({ leaving: true, activity: "walk", goal: CARD.x0 + width / 2 });
    expect(actorOf(run(menagerie, near, 9), species)).toMatchObject({ leaving: true, activity: "walk", facing: -1 });
    const far = advance(menagerie, seated(menagerie, staged(menagerie, [species]), species, 550), [{ kind: "summoned", species: [] }]);
    expect(actorOf(far, species)).toMatchObject({ leaving: true, activity: "idle", x: 550 });
    expect(run(menagerie, far, 16).actors).toEqual([]);
    const gone = run(menagerie, near, 10 * SECOND);
    expect(gone.actors).toEqual([]);
  });

  it("lets a newcomer wait until whoever it replaces is gone: the stage never holds more actors than were summoned", () => {
    const [first, second, third] = everyone as [Slug, Slug, Slug];
    for (const surfaces of [[FLOOR], [CARD, FLOOR]]) {
      let stage = run(menagerie, staged(menagerie, [first, second], { seed: 3, surfaces }), 40);
      stage = seated(menagerie, stage, second, CARD.x0 + kindOf(menagerie, second).size.width / 2 + 40, surfaces.length === 1 ? { perch: FLOOR.id, y: HEIGHT - hoverOf(kindOf(menagerie, second)) } : {});
      stage = advance(menagerie, stage, [{ kind: "summoned", species: [first, third] }]);
      expect(stage.actors.map((actor) => actor.species).sort()).toEqual([first, second].sort());
      expect(actorOf(stage, second).leaving).toBe(true);
      let arrived = -1;
      let left = -1;
      stage = run(menagerie, stage, 12 * SECOND, (after) => {
        expect(after.actors.length).toBeLessThanOrEqual(2);
        if (left < 0 && !after.actors.some((actor) => actor.species === second)) left = after.tick;
        if (arrived < 0 && after.actors.some((actor) => actor.species === third)) arrived = after.tick;
      });
      expect(left).toBeGreaterThan(0);
      expect(arrived).toBeGreaterThanOrEqual(left);
      expect(arrived - left).toBeLessThanOrEqual(SECOND);
      expect(stage.actors.map((actor) => actor.species).sort()).toEqual([first, third].sort());
    }
  });

  it("lets a leaver stay when it is wanted again", () => {
    const species = company.walker;
    let stage = advance(menagerie, seated(menagerie, staged(menagerie, [species]), species, 550), [{ kind: "summoned", species: [] }, ticked(6)]);
    expect(actorOf(stage, species).opacity).toBeLessThan(1);
    stage = run(menagerie, advance(menagerie, stage, [{ kind: "summoned", species: [species] }]), 32);
    expect(actorOf(stage, species)).toMatchObject({ leaving: false, opacity: 1, activity: "idle" });
  });

  it("lets go of a partner that is summoned away", () => {
    const [first, second] = company.friends;
    let stage = staged(menagerie, [first, second], { mode: "lively" });
    stage = seated(menagerie, stage, first, 450, { partner: second, activity: "cuddle", until: stage.tick + 300 });
    stage = seated(menagerie, stage, second, 500, { partner: first, activity: "cuddle", until: stage.tick + 300 });
    stage = advance(menagerie, stage, [{ kind: "summoned", species: [first] }]);
    expect(actorOf(stage, first)).toMatchObject({ partner: null, activity: "idle", leaving: false });
    expect(actorOf(stage, second)).toMatchObject({ partner: null, leaving: true });
  });
});

describe.each(COMPANIES)("a poke in $name", (company) => {
  const menagerie = company.menagerie;
  const species = company.walker;
  const kind = kindOf(menagerie, species);

  it("makes the nearest actor greet towards it and cheers it up", () => {
    for (const side of [1, -1] as const) {
      const stage = alone(company, species, { facing: side === 1 ? -1 : 1 });
      const poked = advance(menagerie, stage, [{ kind: "poked", x: 550 + side * 12, y: CARD.y - kind.size.height / 2 }]);
      const actor = actorOf(poked, species);
      expect(actor).toMatchObject({ activity: "greet", facing: side, partner: null, x: 550 });
      expect(actor.mood).toBeCloseTo(moodOf("idle") + 0.3, 12);
      expect(actor.until - poked.tick).toBeGreaterThanOrEqual(2 * SECOND);
      expect(actor.until - poked.tick).toBeLessThan(5 * SECOND);
      expect(actorOf(run(menagerie, poked, 5 * SECOND), species).activity).toBe("idle");
    }
  });

  it("reaches 1.2 heights and no farther, and picks the nearest of two", () => {
    const stage = alone(company, species);
    const reach = 1.2 * kind.size.height;
    const middle = CARD.y - kind.size.height / 2;
    expect(actorOf(advance(menagerie, stage, [{ kind: "poked", x: 550 + reach - 0.5, y: middle }]), species).activity).toBe("greet");
    expect(advance(menagerie, stage, [{ kind: "poked", x: 550 + reach + 0.5, y: middle }])).toEqual(stage);
    const [first, second] = company.friends;
    let pair = staged(menagerie, [first, second]);
    pair = seated(menagerie, seated(menagerie, pair, first, 500), second, 560);
    pair = advance(menagerie, pair, [{ kind: "poked", x: 540, y: 385 }]);
    expect([actorOf(pair, first).activity, actorOf(pair, second).activity]).toEqual(["idle", "greet"]);
  });

  it("is ignored in a time of concentration: hushed actors rest", () => {
    const hushed = advance(menagerie, alone(company, species), [{ kind: "hushed", quiet: true }]);
    expect(advance(menagerie, hushed, [{ kind: "poked", x: 550, y: CARD.y - kind.size.height / 2 }])).toEqual(hushed);
  });

  it("wakes a sleeper and stops a walker", () => {
    expect(actorOf(advance(menagerie, alone(company, species, { activity: "sleep" }), [{ kind: "poked", x: 550, y: 390 }]), species).activity).toBe("greet");
    const stopped = actorOf(advance(menagerie, alone(company, species, { activity: "walk", goal: 700 }), [ticked(10), { kind: "poked", x: 560, y: 390 }]), species);
    expect(stopped.activity).toBe("greet");
    expect(stopped.goal).toBe(stopped.x);
  });

  it("only cheers up an actor that is busy with its partner, and reconciles a sulker", () => {
    const [first, second] = company.rivals;
    let stage = staged(menagerie, [first, second], { mode: "lively" });
    stage = seated(menagerie, stage, first, 450, { partner: second, activity: "squabble", until: stage.tick + 200, mood: -0.5 });
    stage = seated(menagerie, stage, second, 495, { partner: first, activity: "squabble", until: stage.tick + 200, mood: -0.5 });
    const cheered = advance(menagerie, stage, [{ kind: "poked", x: 450, y: 385 }]);
    expect(actorOf(cheered, first)).toMatchObject({ activity: "squabble", partner: second });
    expect(actorOf(cheered, first).mood).toBeCloseTo(-0.2, 12);
    let sulking = { ...craft(craft(stage, first, { activity: "sulk" }), second, { activity: "idle", partner: null }), rapports: [{ between: [first, second] as const, drift: -0.15 }], met: stage.tick };
    if (menagerie.species.findIndex((candidate) => candidate.id === first) > menagerie.species.findIndex((candidate) => candidate.id === second)) sulking = { ...sulking, rapports: [{ between: [second, first] as const, drift: -0.15 }] };
    const consoled = advance(menagerie, sulking, [{ kind: "poked", x: 450, y: 385 }]);
    expect(actorOf(consoled, first)).toMatchObject({ activity: "greet", partner: null });
    expect(consoled.rapports[0]!.drift).toBeCloseTo(-0.05, 12);
  });
});

describe.each(COMPANIES)("keeping their distance in $name", (company) => {
  const menagerie = company.menagerie;
  const everyone = menagerie.species.map((kind) => kind.id);
  const widthOf = (id: Slug): number => kindOf(menagerie, id).size.width;
  const shoulders = (one: Slug, two: Slug): number => (widthOf(one) + widthOf(two)) / 2;

  /** 👥️ Every pair of grounded actors of one surface whose bodies overlap. */
  function overlapping(stage: Stage): string[] {
    const found: string[] = [];
    for (const one of stage.actors) {
      for (const two of stage.actors) {
        if (one.species >= two.species || one.perch === null || one.perch !== two.perch) continue;
        if (Math.abs(one.x - two.x) < shoulders(one.species, two.species) - 1 / 128) found.push(`${one.species}+${two.species} at tick ${stage.tick}`);
      }
    }
    return found;
  }

  /** 📏️ The width a company needs on one perch: every body and 8 px per neighbour pair. */
  function needOf(species: readonly Slug[]): number {
    return species.reduce((sum, id) => sum + widthOf(id), 0) + 8 * Math.max(species.length - 1, 0);
  }

  const widest = Math.max(...everyone.map(widthOf));
  const STRIP: Surface = { id: "floor", x0: 0, x1: 2 * widest + 20, y: HEIGHT };

  it("lets only as many arrive on a perch as fit with a comfortable gap; the others wait off stage until there is room", () => {
    for (let seed = 1; seed <= sampled(2, 6, 24); seed++) {
      let stage = staged(menagerie, everyone, { seed, surfaces: [STRIP] });
      const arrived = stage.actors.map((actor) => actor.species);
      expect(arrived.length).toBeGreaterThanOrEqual(1);
      expect(arrived.length).toBeLessThan(everyone.length);
      expect(needOf(arrived)).toBeLessThanOrEqual(STRIP.x1 - STRIP.x0);
      expect(stage.wanted).toEqual(everyone);
      stage = run(menagerie, stage, 5 * SECOND, (after) => {
        expect(overlapping(after)).toEqual([]);
        for (const actor of after.actors) expect(arrived).toContain(actor.species);
      });
      stage = advance(menagerie, stage, [surveyed([FLOOR])]);
      expect(stage.actors.map((actor) => actor.species)).toEqual(everyone);
      expect(overlapping(stage)).toEqual([]);
    }
  });

  it("lets whoever waits arrive by itself on the next whole second once there is room, and says so in the wake of a resting stage", () => {
    const plain: Menagerie = { ...menagerie, species: menagerie.species.map((kind) => ({ ...kind, repertoire: {}, clips: [] })) };
    let stage = run(plain, staged(plain, everyone, { surfaces: [FLOOR] }), 130);
    const absent = everyone[1]!;
    stage = { ...stage, actors: stage.actors.filter((actor) => actor.species !== absent).map((actor) => ({ ...actor, until: stage.tick + FAR, blink: stage.tick + FAR })) };
    const frame = frameOf(plain, stage);
    const second = Math.ceil((stage.tick + 1) / SECOND) * SECOND;
    expect([frame.rate, frame.wake]).toEqual([0, second]);
    const before = run(plain, stage, second - stage.tick - 1);
    expect(before.actors.some((actor) => actor.species === absent)).toBe(false);
    const after = run(plain, before, 1);
    expect(actorOf(after, absent)).toMatchObject({ perch: FLOOR.id, opacity: 0, activity: "idle" });
    expect(advance(plain, stage, [ticked(second - stage.tick)])).toEqual(after);
    expect(overlapping(after)).toEqual([]);
    const empty = { ...stage, actors: [] };
    expect(frameOf(plain, empty)).toMatchObject({ rate: 0, wake: second, actors: [] });
    expect(advance(plain, empty, [ticked(10 * SECOND)]).actors.map((actor) => actor.species)).toEqual(everyone);
    expect(frameOf(plain, { ...empty, perches: [] }).wake).toBeNull();
    expect(frameOf(plain, { ...empty, wanted: [] }).wake).toBeNull();
  });

  it("stops a walker before a neighbour: bodies never closer than 6 px, and nobody is walked through", () => {
    const [first, second] = company.strangers;
    for (const side of [1, -1] as const) {
      let stage = staged(menagerie, [first, second], { mode: "lively", surfaces: [CARD] });
      stage = seated(menagerie, stage, second, 550);
      stage = seated(menagerie, stage, first, 550 - side * 150, { activity: "walk", goal: 550 + side * 150, facing: side, clip: kindOf(menagerie, first).repertoire.walk?.[0] ?? kindOf(menagerie, first).repertoire.hop?.[0] ?? null });
      let stopped: Actor | null = null;
      run(menagerie, stage, 20 * SECOND, (after, before) => {
        if (stopped !== null) return;
        expect(overlapping(after)).toEqual([]);
        const walker = actorOf(after, first);
        expect((550 - walker.x) * side).toBeGreaterThanOrEqual(shoulders(first, second) + MEET - 1 / 128);
        if (actorOf(before, first).activity === "walk" && walker.activity !== "walk") stopped = walker;
      });
      expect(stopped).not.toBeNull();
      expect(stopped!.activity).toBe("idle");
      expect((550 - stopped!.x) * side).toBeLessThan(shoulders(first, second) + MEET + Math.max(hopOf(kindOf(menagerie, first)), 1));
      expect(stopped!.goal).toBe(stopped!.x);
    }
  });

  it("never chooses a goal beyond or inside a neighbour, and never lets two bodies overlap however lively the stage is", () => {
    const narrow: Surface = { ...CARD, x0: 200, x1: 200 + needOf(everyone) + 3 * widest };
    const trouble: string[] = [];
    let walks = 0;
    for (let seed = 1; seed <= sampled(2, 6, 12); seed++) {
      run(menagerie, staged(menagerie, everyone, { seed, mode: "lively", surfaces: [narrow] }), sampled(60, 300, 300) * SECOND, (after, before) => {
        trouble.push(...overlapping(after));
        for (const actor of after.actors) {
          const earlier = before.actors.find((candidate) => candidate.species === actor.species);
          if (earlier === undefined || actor.activity !== "walk" || earlier.activity === "walk") continue;
          walks++;
          for (const other of after.actors) {
            if (other.species === actor.species || other.species === actor.partner || other.perch !== actor.perch) continue;
            if ((other.x - earlier.x) * (other.x - actor.goal) <= 0) trouble.push(`${actor.species} aims past ${other.species} at tick ${after.tick}`);
            if (Math.abs(other.x - actor.goal) < shoulders(actor.species, other.species) + MEET - 1 / 128) trouble.push(`${actor.species} aims into ${other.species} at tick ${after.tick}`);
          }
        }
      });
    }
    expect(trouble).toEqual([]);
    expect(walks).toBeGreaterThan(5);
  });

  it("sets everyone apart when a perch shrinks, crowds out whoever does not fit where it stood, and lets it return when there is room", () => {
    for (let seed = 1; seed <= sampled(2, 6, 24); seed++) {
      const wide = run(menagerie, staged(menagerie, everyone, { seed, surfaces: [FLOOR] }), 32);
      const shrunk = advance(menagerie, wide, [surveyed([STRIP])]);
      const staying = shrunk.actors.filter((actor) => !actor.leaving);
      const crowded = shrunk.actors.filter((actor) => actor.leaving);
      expect(overlapping(shrunk)).toEqual([]);
      expect(staying.length).toBeGreaterThanOrEqual(1);
      expect(crowded.length).toBe(everyone.length - staying.length);
      expect(crowded.length).toBeGreaterThan(0);
      expect(needOf(staying.map((actor) => actor.species))).toBeLessThanOrEqual(STRIP.x1);
      for (const actor of staying) {
        expect(actor.perch).toBe(STRIP.id);
        expect(actor.x).toBeGreaterThanOrEqual(widthOf(actor.species) / 2);
        expect(actor.x).toBeLessThanOrEqual(STRIP.x1 - widthOf(actor.species) / 2);
      }
      for (const actor of crowded) expect(actor).toMatchObject({ perch: null, x: actorOf(wide, actor.species).x, y: actorOf(wide, actor.species).y, opacity: 1, partner: null });
      const later = run(menagerie, shrunk, 3 * SECOND, (after) => expect(overlapping(after)).toEqual([]));
      for (const actor of staying) expect(later.actors.map((candidate) => candidate.species)).toContain(actor.species);
      expect(later.actors.length).toBeLessThan(everyone.length);
      expect(later.actors.every((actor) => actor.perch === STRIP.id && !actor.leaving)).toBe(true);
      expect(needOf(later.actors.map((actor) => actor.species))).toBeLessThanOrEqual(STRIP.x1);
      expect(later.wanted).toEqual(everyone);
      const again = advance(menagerie, shrunk, [surveyed([STRIP])]);
      expect(again.actors).toEqual(shrunk.actors);
      const widened = advance(menagerie, later, [surveyed([FLOOR])]);
      expect(widened.actors.map((actor) => actor.species)).toEqual(everyone);
      expect(overlapping(widened)).toEqual([]);
    }
  });

  it("keeps neighbours as close as they were when a ride only moves them, and never closer than their bodies allow", () => {
    const [first, second] = company.friends;
    let stage = staged(menagerie, [first, second], { surfaces: [CARD] });
    stage = seated(menagerie, seated(menagerie, stage, first, 500), second, 500 + shoulders(first, second) + 2);
    const moved = advance(menagerie, stage, [surveyed([{ ...CARD, x0: CARD.x0 + 37, x1: CARD.x1 + 37 }])]);
    expect(actorOf(moved, second).x - actorOf(moved, first).x).toBe(actorOf(stage, second).x - actorOf(stage, first).x);
    const squeezed = advance(menagerie, stage, [surveyed([{ ...CARD, x1: 500 + shoulders(first, second) + widthOf(second) / 2 - 3 }])]);
    expect(overlapping(squeezed)).toEqual([]);
    expect(squeezed.actors.every((actor) => !actor.leaving)).toBe(true);
    expect(actorOf(squeezed, second).x).toBe(500 + shoulders(first, second) - 3);
    expect(actorOf(squeezed, second).x - actorOf(squeezed, first).x).toBe(shoulders(first, second) + 2);
    const stacked = craft(stage, second, { x: 505, goal: 505 });
    const parted = advance(menagerie, stacked, [surveyed([{ ...CARD, x0: CARD.x0 + 1 }])]);
    expect(overlapping(parted)).toEqual([]);
  });

  it("comes down beside whoever stands where it lands, and is crowded out when the perch is full", () => {
    const [first, second] = company.strangers;
    const hover = hoverOf(kindOf(menagerie, first));
    let stage = staged(menagerie, [first, second], { surfaces: [FLOOR] });
    stage = craft(stage, second, { perch: FLOOR.id, x: 550, y: HEIGHT - hoverOf(kindOf(menagerie, second)), goal: 550, activity: "idle", since: stage.tick, until: stage.tick + FAR, blink: stage.tick + FAR, opacity: 1 });
    const aimed = 550 + shoulders(first, second) - 4;
    const falling: Partial<Actor> = { perch: null, x: aimed, y: HEIGHT - 120 - hover, vx: 0, vy: 0, goal: aimed, activity: "fall", since: stage.tick, until: stage.tick + FAR, opacity: 1 };
    const landed = run(menagerie, craft(stage, first, falling), 2 * SECOND, (after) => expect(overlapping(after)).toEqual([]));
    expect(actorOf(landed, first)).toMatchObject({ perch: FLOOR.id, leaving: false, y: HEIGHT - hover, x: 550 + shoulders(first, second) + 8 });
    const room = Math.max(widthOf(first), widthOf(second)) / 2 + 2;
    const full: Surface = { id: "ledge", x0: 550 - room, x1: 550 + room, y: HEIGHT };
    let cramped = advance(menagerie, craft(stage, first, { ...falling, x: 553, goal: 553 }), [surveyed([full])]);
    expect(actorOf(cramped, second)).toMatchObject({ perch: full.id, x: 550, leaving: false });
    let faded = false;
    cramped = run(menagerie, cramped, 2 * SECOND, (after) => {
      expect(overlapping(after)).toEqual([]);
      const lander = after.actors.find((actor) => actor.species === first);
      if (lander !== undefined && lander.activity === "land") {
        faded = true;
        expect(lander).toMatchObject({ leaving: true, perch: null });
      }
    });
    expect(faded).toBe(true);
    expect(cramped.actors.map((actor) => actor.species)).toEqual([second]);
  });

  it("leaves a crowd for a perch with room: a pet that feels like a hop where none is in reach walks off and arrives anew elsewhere", () => {
    let relocated = 0;
    let hops = 0;
    for (let seed = 1; seed <= sampled(1, 4, 8); seed++) {
      let stage = run(menagerie, staged(menagerie, everyone, { seed, mode: "lively", surfaces: [FLOOR] }), 2 * SECOND);
      expect(stage.actors.every((actor) => actor.perch === FLOOR.id)).toBe(true);
      stage = advance(menagerie, stage, [surveyed([CARD, FLOOR])]);
      run(menagerie, stage, sampled(60, 240, 240) * SECOND, (after, before) => {
        expect(overlapping(after)).toEqual([]);
        for (const actor of after.actors) {
          if (actor.activity === "hop") hops++;
          if (actor.perch === CARD.id && !before.actors.some((earlier) => earlier.species === actor.species)) relocated++;
        }
      });
    }
    expect(hops).toBe(0);
    expect(relocated).toBeGreaterThan(0);
  });
});

describe.each(COMPANIES)("hopping between perches in $name", (company) => {
  const menagerie = company.menagerie;
  const species = company.walker;
  const kind = kindOf(menagerie, species);
  const low: Surface = { id: "low", x0: 300, x1: 500, y: 400 };
  const high: Surface = { id: "high", x0: 520, x1: 800, y: 380 };

  /** 🪜️ How many hops between the two shelves a lonely pet takes under a keep-out, and whether it ever covered it: in five lively minutes for each of three seeds, of eight at the exhaustive level, and at the fundamental level in one minute for each of four (the fourth seed is the one that hops that soon). It starts on the lower shelf, beside the upper one (a newcomer by itself would arrive on the upper one, wherever on it). */
  function hops(keepouts: readonly Rect[]): { readonly count: number; readonly covered: number } {
    let count = 0;
    let covered = 0;
    for (let seed = 1; seed <= sampled(4, 3, 8); seed++) {
      const arrived = staged(menagerie, [species], { seed, mode: "lively", surfaces: [low, high], keepouts });
      expect(actorOf(arrived, species).perch).toBe(high.id);
      run(menagerie, craft(arrived, species, { perch: low.id, x: low.x1 - kind.size.width, y: low.y, goal: low.x1 - kind.size.width }), sampled(60, 300, 300) * SECOND, (after, before) => {
        const actor = after.actors.find((candidate) => candidate.species === species);
        if (actor === undefined) return;
        if (actor.activity === "hop" && before.actors.find((candidate) => candidate.species === species)?.activity !== "hop") count++;
        for (const keepout of keepouts) if (covers(menagerie, actor, keepout)) covered++;
      });
    }
    return { count, covered };
  }

  it("steps between two levels by hopping, and never when the arc would pass in front of something that must stay free", () => {
    expect(hops([]).count).toBeGreaterThan(0);
    const bar: Rect = { x: 0, y: 0, width: WIDTH, height: high.y - kind.size.height - 2 };
    expect(hops([bar])).toEqual({ count: 0, covered: 0 });
    const far: Rect = { x: 0, y: 0, width: WIDTH, height: 40 };
    const free = hops([far]);
    expect(free.count).toBeGreaterThan(0);
    expect(free.covered).toBe(0);
  });
});

describe.each(COMPANIES)("turning round in $name", (company) => {
  const menagerie = company.menagerie;
  const species = company.walker;
  const kind = kindOf(menagerie, species);

  /** 🪞️ The factor a drawing is squeezed by across, `step` ticks into a turn of eight: from −1 (still the way it faced) through 0 (a line) to 1. */
  function squeezeAt(step: number): number {
    const amount = step / 8;
    return 2 * (amount * amount * (3 - 2 * amount)) - 1;
  }

  it("turns towards its goal before it sets out: it faces the goal at once, its drawing follows over eight ticks on the spot, squeezed through a line, then the walk", () => {
    const walk = kind.repertoire.walk![0]!;
    const idle = kind.repertoire.idle![0]!;
    for (const facing of [1, -1] as const) {
      const turned = facing === 1 ? -1 : 1;
      const before = alone(company, species, { activity: "walk", goal: 550 - facing * 120, facing, clip: walk });
      let stage = run(menagerie, before, 1);
      expect(actorOf(stage, species)).toMatchObject({ x: 550, facing: turned, faced: before.tick + 9, activity: "walk" });
      const squeezes: number[] = [];
      for (let step = 0; step < 8; step++) {
        const actor = actorOf(stage, species);
        expect([actor.x, actor.facing, actor.activity]).toEqual([550, turned, "walk"]);
        const frame = frameOf(menagerie, stage);
        expect(frame.rate).toBe(64);
        expect(frame.actors[0]!.facing).toBe(turned);
        const squeeze = squeezeAt(step);
        const standing = frameOf(menagerie, craft(stage, species, { activity: "idle", clip: idle, goal: 550, faced: stage.tick, facing: squeeze < 0 ? facing : turned })).actors[0]!.bones;
        squeezes.push(squeeze);
        frame.actors[0]!.bones.forEach((number, index) => expect(number).toBeCloseTo(standing[index]! * (index % 2 === 0 ? squeeze : 1), 9));
        stage = run(menagerie, stage, 1);
      }
      expect(squeezes[0]).toBe(-1);
      expect(squeezes[4]).toBe(0);
      expect(squeezes[7]).toBeGreaterThan(0.9);
      expect(actorOf(stage, species)).toMatchObject({ x: 550, facing: turned, faced: stage.tick, activity: "walk", since: stage.tick });
      const walking = run(menagerie, stage, 3);
      expect((actorOf(walking, species).x - 550) * facing).toBeLessThan(0);
      expect(actorOf(walking, species).x).toBeCloseTo(550 - facing * 3 * (kind.locomotion.speed / SECOND), 9);
    }
  });

  it("sets out at once when it faces its goal already", () => {
    const stage = run(menagerie, alone(company, species, { activity: "walk", goal: 700, facing: 1 }), 1);
    expect(actorOf(stage, species)).toMatchObject({ x: 550 + kind.locomotion.speed / SECOND, facing: 1 });
  });

  it("turns towards its partner when they meet and away from it when it sulks, with the pupils on the side it is drawn", () => {
    const [first, second] = company.rivals;
    let stage = staged(menagerie, [first, second], { mode: "lively", surfaces: [CARD] });
    stage = seated(menagerie, stage, first, 450, { partner: second, facing: -1, gaze: { x: 0.5, y: 0, vx: 0, vy: 0 } });
    stage = seated(menagerie, stage, second, 450 + (kindOf(menagerie, first).size.width + kindOf(menagerie, second).size.width) / 2 + 2, { partner: first, facing: 1 });
    const early = frameOf(menagerie, run(menagerie, stage, 2)).actors.find((actor) => actor.species === first)!;
    const late = frameOf(menagerie, run(menagerie, stage, 6)).actors.find((actor) => actor.species === first)!;
    expect([early.facing, late.facing]).toEqual([1, 1]);
    if (early.eyes.length > 0) expect(Math.sign(early.eyes[0]!.x)).toBe(-Math.sign(late.eyes[0]!.x));
    const met = run(menagerie, stage, 9);
    expect([actorOf(met, first).facing, actorOf(met, second).facing]).toEqual([1, -1]);
    let sulking = craft(craft(met, first, { activity: "sulk", since: met.tick, until: met.tick + FAR }), second, { activity: "sulk", since: met.tick, until: met.tick + FAR });
    sulking = run(menagerie, sulking, 9);
    expect([actorOf(sulking, first).facing, actorOf(sulking, second).facing]).toEqual([-1, 1]);
  });

  it("turns back from where its drawing is when it is asked to turn again in the middle of a turn", () => {
    const plain: Menagerie = { ...menagerie, species: menagerie.species.map((entry) => ({ ...entry, repertoire: {}, clips: [] })) };
    let stage = advance(plain, alone({ ...company, menagerie: plain }, species, { facing: 1 }), [{ kind: "poked", x: 530, y: CARD.y - kind.size.height / 2 }]);
    expect(actorOf(stage, species)).toMatchObject({ facing: -1, faced: stage.tick + 8, activity: "greet" });
    stage = run(plain, stage, 3);
    const before = frameOf(plain, stage).actors[0]!;
    expect(before.bones[0]).toBe(squeezeAt(3));
    stage = advance(plain, stage, [{ kind: "poked", x: 570, y: CARD.y - kind.size.height / 2 }]);
    expect(actorOf(stage, species)).toMatchObject({ facing: 1, faced: stage.tick + 3 });
    const after = frameOf(plain, stage).actors[0]!;
    expect(after.bones[0]! * after.facing).toBeCloseTo(before.bones[0]! * before.facing, 12);
    stage = run(plain, stage, 3);
    expect(frameOf(plain, stage).actors[0]!.bones[0]).toBe(1);
  });
});

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
      run(menagerie, advance(menagerie, rested(sated), [{ kind: "pointed", x, y: level }]), 2 * SECOND, (after) => seen.push(actorOf(after, species).facing));
      return seen;
    };
    expect(new Set(facings(550 - half - 11))).toEqual(new Set([1]));
    expect(new Set(facings(550 + 300))).toEqual(new Set([1]));
    const behind = facings(550 - half - 13);
    expect(behind[0]).toBe(-1);
    expect(new Set(behind)).toEqual(new Set([-1]));
    let stage = advance(menagerie, rested(sated), [{ kind: "pointed", x: 100, y: level }]);
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
      stage = advance(menagerie, stage, [{ kind: "pointed", x: tick % 4 < 2 ? 100 : 1000, y: level }, ticked()]);
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
    const pointedAt = (stage: Stage): Stage => run(menagerie, advance(menagerie, stage, [{ kind: "pointed", x: 100, y: level }]), 2 * SECOND);
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
    let stage = advance(menagerie, rested(sated), [{ kind: "pointed", x: 1100, y: level }]);
    const tilts: number[] = [];
    stage = run(menagerie, stage, SECOND, (after) => tilts.push(tilt(after)));
    const gaze = actorOf(stage, species).gaze;
    expect(gaze.x).toBeGreaterThan(0.8);
    for (let step = 1; step < tilts.length; step++) expect(Math.abs(tilts[step]! - tilts[step - 1]!)).toBeLessThan(0.02);
    expect(Math.abs(tilts.at(-1)!)).toBeGreaterThan(0.05);
    const pose = restPose(kind).map((rest, index) => (index === bone ? { ...rest, x: rest.x + 1.5 * gaze.x, y: rest.y + gaze.y, rotation: rest.rotation + 5 * gaze.x } : rest));
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
    const beside = { kind: "pointed", x: 550 + half + 20, y: level } as const;
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

  it("does not perk up for a pointer that keeps moving, rests on it, is far away, or in a time of concentration", () => {
    const curious = rested(eager);
    let moving = curious;
    for (let tick = 0; tick < 2 * SECOND; tick++) {
      moving = advance(menagerie, moving, [{ kind: "pointed", x: 550 + half + 20 + (tick % 20), y: level }, ticked()]);
      expect(actorOf(moving, species).activity).toBe("idle");
    }
    const resting = (x: number, y: number, quiet = false): Activity => actorOf(run(menagerie, advance(menagerie, curious, [{ kind: "hushed", quiet }, { kind: "pointed", x, y }]), 40), species).activity;
    expect(resting(550 + half + 20, level)).toBe("greet");
    expect(resting(550, level)).toBe("idle");
    expect(resting(550 + 4 * kind.size.height, level)).toBe("idle");
    expect(resting(550 + half + 20, level, true)).toBe("idle");
  });

  it("jumps over the time in which a resting pointer changes nothing, and stops for the tick it has lingered and the tick an actor may turn again", () => {
    const plain: Menagerie = { ...menagerie, species: menagerie.species.map((entry) => ({ ...entry, repertoire: {}, clips: [] })) };
    for (const [needs, x] of [[eager, 550 + half + 20], [sated, 100], [sated, 550 + half + 20]] as const) {
      const pointed = advance(plain, rested(needs, plain), [{ kind: "pointed", x, y: level }]);
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
    const lingering = advance(plain, rested(eager, plain), [{ kind: "pointed", x: 550 + half + 20, y: level }]);
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

describe.each(COMPANIES)("the pointer's courtesy in $name", (company) => {
  const menagerie = company.menagerie;
  const species = company.walker;
  const kind = kindOf(menagerie, species);
  const middle = CARD.y - kind.size.height / 2;

  it("turns see-through while the pointer rests on it, eased, and whole again when the pointer leaves", () => {
    let stage = advance(menagerie, alone(company, species), [{ kind: "pointed", x: 550, y: middle }]);
    expect(frameOf(menagerie, stage).rate).toBe(64);
    let previous = 1;
    stage = run(menagerie, stage, 21, (after) => {
      const opacity = actorOf(after, species).opacity;
      expect(opacity).toBeLessThan(previous);
      expect(previous - opacity).toBeLessThanOrEqual(1 / 32);
      expect(opacity).toBeGreaterThanOrEqual(0.35);
      expect(frameOf(menagerie, after).actors[0]!.opacity).toBe(opacity);
      previous = opacity;
    });
    expect(previous).toBe(0.35);
    stage = run(menagerie, stage, 10 * SECOND);
    expect(actorOf(stage, species)).toMatchObject({ opacity: 0.35, activity: "idle", x: 550 });
    expect(frameOf(menagerie, stage).rate).toBe(32);
    stage = run(menagerie, advance(menagerie, stage, [{ kind: "pointed", x: 550 + kind.size.width, y: middle }]), 11);
    expect(actorOf(stage, species).opacity).toBe(1);
    stage = run(menagerie, advance(menagerie, stage, [{ kind: "pointed", x: 550, y: middle }]), 30);
    expect(actorOf(stage, species).opacity).toBe(0.35);
    stage = run(menagerie, advance(menagerie, stage, [{ kind: "unpointed" }]), 11);
    expect(actorOf(stage, species).opacity).toBe(1);
  });

  it("reaches as far as its box and 4 px around it, no farther", () => {
    const stage = alone(company, species);
    const shy = (x: number, y: number): boolean => actorOf(run(menagerie, advance(menagerie, stage, [{ kind: "pointed", x, y }]), 30), species).opacity < 1;
    const half = kind.size.width / 2;
    expect([shy(550 + half + 3, middle), shy(550 - half - 3, middle), shy(550, CARD.y - kind.size.height - 3), shy(550, CARD.y + 3)]).toEqual([true, true, true, true]);
    expect([shy(550 + half + 5, middle), shy(550 - half - 5, middle), shy(550, CARD.y - kind.size.height - 5), shy(550, CARD.y + 5)]).toEqual([false, false, false, false]);
  });

  it("does not keep a resting stage awake: time is still jumped while the pointer rests on a pet", () => {
    const stage = run(menagerie, advance(menagerie, alone(company, species), [{ kind: "pointed", x: 550, y: middle }]), 6 * SECOND);
    expect(actorOf(stage, species).opacity).toBe(0.35);
    const started = performance.now();
    const later = advance(menagerie, stage, [ticked(FAR - 10 * SECOND)]);
    expect(performance.now() - started).toBeLessThan(200);
    expect(later.actors).toEqual(stage.actors);
  });

  it("is see-through at once on a still stage, where nothing eases, and leaves the actor as it is", () => {
    const still = advance(menagerie, alone(company, species), [{ kind: "tuned", mode: "still" }]);
    const pointed = advance(menagerie, still, [{ kind: "pointed", x: 550, y: middle }, ticked(100)]);
    expect(pointed.actors).toEqual(still.actors);
    expect(frameOf(menagerie, pointed)).toMatchObject({ rate: 0, wake: null });
    expect(frameOf(menagerie, pointed).actors[0]!.opacity).toBe(0.35);
    expect(frameOf(menagerie, advance(menagerie, pointed, [{ kind: "pointed", x: 100, y: 100 }])).actors[0]!.opacity).toBe(1);
  });

  it("lets a newcomer under the pointer fade in only as far as see-through", () => {
    let stage = staged(menagerie, [species], { surfaces: [FLOOR] });
    const actor = actorOf(stage, species);
    stage = run(menagerie, advance(menagerie, stage, [{ kind: "pointed", x: actor.x, y: actor.y - 5 }]), 40, (after) => expect(actorOf(after, species).opacity).toBeLessThanOrEqual(0.35));
    expect(actorOf(stage, species).opacity).toBe(0.35);
  });
});

describe.each(COMPANIES)("determinism in $name", (company) => {
  const menagerie = company.menagerie;
  const everyone = menagerie.species.map((kind) => kind.id);

  /** 📽️ The frames of a little story: an arrival, a pointer, a vanishing card, a poke, a quiet time. */
  function story(seed: number, ticks: number): Frame[] {
    const frames: Frame[] = [];
    let stage = staged(menagerie, everyone, { seed, mode: "lively" });
    for (let tick = 1; tick <= ticks; tick++) {
      const events: StageEvent[] = [];
      if (tick === 200) events.push({ kind: "pointed", x: 400, y: 300 });
      if (tick === 900) events.push(surveyed([FLOOR]));
      if (tick === 1300) events.push({ kind: "poked", x: 640, y: 700 });
      if (tick === 1500) events.push(surveyed([CARD, FLOOR]), { kind: "hushed", quiet: true });
      if (tick === 2400) events.push({ kind: "hushed", quiet: false }, { kind: "summoned", species: everyone.slice(1) });
      stage = advance(menagerie, stage, [...events, ticked()]);
      frames.push(frameOf(menagerie, stage));
    }
    return frames;
  }

  it("yields the same frames for the same seed and events, and another story for another seed", () => {
    const ticks = sampled(320, 3000, 3000);
    for (const seed of sampled([42], [42], [42, 142, 242])) {
      const first = story(seed, ticks);
      expect(story(seed, ticks)).toEqual(first);
      const other = story(seed + 1, ticks);
      expect(other[0]!.actors.map((actor) => actor.x)).not.toEqual(first[0]!.actors.map((actor) => actor.x));
      expect(JSON.stringify(other.at(-1))).not.toBe(JSON.stringify(first.at(-1)));
    }
  });

  it("ends in the same stage however the ticks are cut", () => {
    for (const mode of ["calm", "lively"] as const) {
      let stage = staged(menagerie, everyone, { seed: 8, mode });
      let chunked = stage;
      const chunks = sampled([1, 2, 3, 64, 7, 500, 5, 63, 129], [1, 2, 3, 64, 7, 1000, 5, 5000, 63, 129, 20000], [1, 2, 3, 64, 7, 1000, 5, 5000, 63, 129, 20000, 9, 50000, 65]);
      for (const chunk of chunks) {
        stage = run(menagerie, stage, chunk);
        chunked = advance(menagerie, chunked, [ticked(chunk)]);
        expect(chunked, `${mode} after a chunk of ${chunk}`).toEqual(stage);
      }
      const events: StageEvent[] = [{ kind: "pointed", x: 500, y: 350 }, { kind: "glanced", points: [{ x: 100, y: 100 }] }];
      const tail = sampled(500, 3000, 3000);
      stage = run(menagerie, advance(menagerie, stage, events), tail);
      chunked = advance(menagerie, chunked, [...events, ticked((2 * tail) / 5), ticked((3 * tail) / 5)]);
      expect(chunked).toEqual(stage);
    }
  });

  it("jumps over a quiet hour of sleep without stepping through it", () => {
    let stage = staged(menagerie, everyone, { seed: 1 });
    for (const species of everyone) stage = craft(stage, species, { activity: "sleep", until: stage.tick + FAR, opacity: 1, mood: moodOf("sleep") });
    const started = performance.now();
    const later = advance(menagerie, stage, [ticked(FAR - 1)]);
    expect(performance.now() - started).toBeLessThan(200);
    expect(later.tick).toBe(stage.tick + FAR - 1);
    expect(later.actors).toEqual(stage.actors);
    expect(advance(menagerie, later, [ticked(1)]).actors.every((actor) => actor.activity !== "sleep")).toBe(true);
  });

  it("never changes the stage or the events it is given", () => {
    let stage = frozen(staged(menagerie, everyone, { seed: 5, mode: "lively" }));
    for (let round = 0; round < sampled(12, 40, 160); round++) {
      const events = frozen<StageEvent[]>([{ kind: "pointed", x: 10 * round, y: 300 }, ticked(97), surveyed(round % 7 === 3 ? [FLOOR] : [CARD, FLOOR]), { kind: "poked", x: 30 * round, y: 700 }, ticked(31)]);
      const before = JSON.stringify(stage);
      const next = advance(menagerie, stage, events);
      expect(JSON.stringify(stage)).toBe(before);
      stage = frozen(next);
    }
    expect(advance(menagerie, stage, [])).toBe(stage);
  });

  it("draws for every actor from its own stream: who else is on stage does not change an actor's first decisions", () => {
    const species = company.walker;
    const lonely = staged(menagerie, [species], { seed: 12 });
    const crowded = staged(menagerie, everyone, { seed: 12 });
    expect(actorOf(crowded, species).until).toBe(actorOf(lonely, species).until);
    expect(actorOf(crowded, species).blink).toBe(actorOf(lonely, species).blink);
    expect(actorOf(crowded, species).draws).toBe(actorOf(lonely, species).draws);
  });
});

describe.each(COMPANIES)("the rate and the wake of $name", (company) => {
  const menagerie = company.menagerie;
  const everyone = menagerie.species.map((kind) => kind.id);
  const species = company.walker;

  it("asks for 64 ticks per second while anyone fades, walks, falls or lands", () => {
    expect(frameOf(menagerie, staged(menagerie, [species])).rate).toBe(64);
    expect(frameOf(menagerie, alone(company, species, { activity: "walk", goal: 700 })).rate).toBe(64);
    expect(frameOf(menagerie, alone(company, species, { activity: "fall", perch: null })).rate).toBe(64);
    expect(frameOf(menagerie, alone(company, species, { activity: "land" })).rate).toBe(64);
    expect(frameOf(menagerie, alone(company, species, { leaving: true })).rate).toBe(64);
  });

  it("asks for 32 while only loops, blinks, pupils or moods move, and 16 while everyone sleeps", () => {
    let stage = run(menagerie, alone(company, species, { clip: kindOf(menagerie, species).repertoire.idle![0]! }), 64);
    expect(frameOf(menagerie, stage)).toMatchObject({ rate: 32, wake: null });
    stage = run(menagerie, craft(stage, species, { activity: "sleep", since: stage.tick - 100, clip: kindOf(menagerie, species).repertoire.sleep![0]!, mood: moodOf("sleep") }), 1);
    expect(actorOf(stage, species).gaze.x).toBeGreaterThan(0);
    expect(frameOf(menagerie, stage).rate).toBe(32);
    stage = run(menagerie, stage, SECOND);
    expect(actorOf(stage, species).gaze).toEqual({ x: 0, y: 0, vx: 0, vy: 0 });
    expect(frameOf(menagerie, stage)).toMatchObject({ rate: 16, wake: null });
    expect(frameOf(menagerie, craft(stage, species, { mood: 0.5 })).rate).toBe(32);
  });

  it("names what every actor is doing, so a render target can show it", () => {
    for (const activity of ACTIVITIES) {
      const stage = alone(company, species, { activity });
      expect(frameOf(menagerie, stage).actors[0]!.activity).toBe(activity);
      expect(frameOf(menagerie, advance(menagerie, alone(company, species), [{ kind: "tuned", mode: "still" }])).actors[0]!.activity).toBe("idle");
    }
  });

  it("reports the highest rate anyone needs and draws the actors back to front", () => {
    let stage = run(menagerie, staged(menagerie, everyone, { seed: 3, mode: "lively" }), 30 * SECOND);
    let sorted = true;
    let finite = true;
    let moving = true;
    for (let tick = 0; tick < sampled(600, 2000, 10000); tick++) {
      stage = advance(menagerie, stage, [ticked()]);
      const frame = frameOf(menagerie, stage);
      if (frame.tick !== stage.tick || frame.wake !== null || frame.actors.length !== stage.actors.length || stage.actors.length < everyone.length - 1 || ![16, 32, 64].includes(frame.rate)) moving = false;
      if (stage.actors.some((actor) => actor.activity === "walk" || actor.activity === "hop" || actor.activity === "fall" || actor.activity === "land" || actor.opacity < 1) && frame.rate !== 64) moving = false;
      for (let index = 1; index < frame.actors.length; index++) {
        const lower = frame.actors[index - 1]!;
        const upper = frame.actors[index]!;
        if (!(lower.y < upper.y || (lower.y === upper.y && lower.species < upper.species))) sorted = false;
      }
      for (const drawn of frame.actors) if (drawn.bones.length !== 6 * kindOf(menagerie, drawn.species).bones.length || !drawn.bones.every((number) => Number.isFinite(number)) || !(Math.abs(drawn.mood) <= 1)) finite = false;
    }
    expect([sorted, finite, moving]).toEqual([true, true, true]);
  });

  it("reports rate 0 with the tick of the next change for a pet that has no loop, and wakes exactly then", () => {
    const plain: Menagerie = { ...menagerie, species: menagerie.species.map((kind) => ({ ...kind, repertoire: {}, clips: [] })) };
    let stage = advance(plain, staged(plain, [species]), [ticked(32)]);
    stage = run(plain, stage, 64);
    const actor = actorOf(stage, species);
    const frame = frameOf(plain, stage);
    expect(frame.rate).toBe(0);
    expect(frame.wake).toBe(Math.min(actor.blink > stage.tick ? actor.blink : actor.blink + BLINK_TICKS, actor.until));
    expect(frame.wake!).toBeGreaterThan(stage.tick);
    const woken = advance(plain, stage, [ticked(frame.wake! - stage.tick)]);
    expect(woken).toEqual(run(plain, stage, frame.wake! - stage.tick));
    expect(frameOf(plain, advance(plain, woken, [ticked(1)])).rate).toBeGreaterThan(0);
    const pointed = advance(plain, stage, [{ kind: "pointed", x: 100, y: 100 }]);
    expect(frameOf(plain, pointed).rate).toBe(32);
  });

  it("blends the clip of an activity in over its first eight ticks, on top of the idle loop that never stops", () => {
    const kind = kindOf(menagerie, species);
    const greet = kind.repertoire.greet![0]!;
    const idle = alone(company, species, { clip: kind.repertoire.idle![0]! });
    const greeting = craft(idle, species, { activity: "greet", clip: greet, since: idle.tick, until: idle.tick + 200, mood: moodOf("greet") });
    expect(frameOf(menagerie, greeting).actors[0]!.bones).toEqual(frameOf(menagerie, idle).actors[0]!.bones);
    expect(frameOf(menagerie, greeting).rate).toBe(64);
    const later = { ...greeting, tick: greeting.tick + 30 };
    expect(frameOf(menagerie, later).actors[0]!.bones).not.toEqual(frameOf(menagerie, { ...idle, tick: idle.tick + 30 }).actors[0]!.bones);
    const ending = { ...greeting, tick: greeting.tick + 200 };
    expect(frameOf(menagerie, ending).actors[0]!.bones).toEqual(frameOf(menagerie, { ...idle, tick: idle.tick + 200 }).actors[0]!.bones);
  });
});

describe("the arithmetic of the module", () => {
  it("uses nothing the Rust twin cannot reproduce bit for bit, and never the console", () => {
    const source = readFileSync(resolve(HERE, "../../🟦️.ts"), "utf8");
    expect(source).not.toMatch(/Math\.(sin|cos|tan|atan2|exp|pow|hypot|log|random)\b/);
    expect(source).not.toMatch(/\b(Date|performance|console)\b/);
  });
});
