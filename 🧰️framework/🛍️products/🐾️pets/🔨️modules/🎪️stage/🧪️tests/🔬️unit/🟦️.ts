/** 🎪️ Unit suite of the stage: everything the owner asked of the pets, checked on the fold itself — slightly active by default, eyes that follow the pointer while standing still, blinking, walking along perches and riding a surface that moves, falling when a perch vanishes and landing, encounters that are affectionate for friends and small disputes (squabble → sulk → mending) for rivals, quiet and still, summon and leave, the events of the learner's hand, what an actor arrives with and stands on, determinism, and the rate, the wake and the fields a frame reports.
 *
 * Two menageries are played: the sample menagerie of the schema-conformance vectors (a walker, a hopper, a floater)
 * and the troupe of the stage-trace vectors (five blobs). Where a test needs a precise situation it writes the state
 * of an actor directly — the stage is a pure fold over plain data, so any valid state is a fair starting point.
 *
 * @see ../../🟦️.ts — the façade under test; the modules it folds are the `PARTS` whose arithmetic the last block reads
 * @see ../../../../🧫️fixtures/🧬️schema-conformance/🔣️.json — the sample menagerie
 * @see ../../../../🧫️fixtures/🎪️stage-trace/🔣️.json — the trace troupe
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { ACTIVITIES, TICKS_PER_SECOND, type Activity, type Actor, type Fixture, type Frame, type Menagerie, type PetMode, type Rect, type Slug, type Species, type Stage, type StageEvent, type Surface, type Wall } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { BLINK_TICKS } from "../../../🎞️animation/🟦️.ts";
import { perchAt } from "../../../🏞️terrain/🟦️.ts";
import { lookOffset, restPose, solveRig } from "../../../🦴️rig/🟦️.ts";
import { menagerieIssues } from "../../../✅️validation/🟦️.ts";
import { ENCOUNTERS, MODE_LIMITS, followersOf } from "../../../🧠️behavior/🟦️.ts";
import { atRest, faceOf, settled, spiritsOf } from "../../../💗️feeling/🟦️.ts";
import { COLD, IDLE, noHover, noShaking } from "../../../👆️gesture/🟦️.ts";
import { hail, shaken } from "../../../👀️attention/🟦️.ts";
import { draftOf, indexOf, sealed } from "../../../📝️draft/🟦️.ts";
import { overlaps } from "../../../🚧️clearance/🟦️.ts";
import { WALL_LEAN, bodiesOf, extentOf } from "../../../📏️spacing/🟦️.ts";
import { STATE_BLEND } from "../../../🎥️projection/🟦️.ts";
import { BOUNCE, setOut, type Venture } from "../../../🚶️locomotion/🟦️.ts";
import { GRIP_BUDGET, LADDER_IDLE, ROPE_MISS_CHANCE, SLIP_PUSH } from "../../../🧗️climbing/🟦️.ts";
import { LIFT_BRACE, LIFT_LIMIT, LIFT_RETURNS, LIFT_SHOVE, LIFT_TICKS, LIFT_WOBBLE, MISCHIEF_COOLDOWN_CALM, MISCHIEF_COOLDOWN_LIVELY, MISCHIEF_PATIENCE, MISCHIEF_PATIENCE_QUIET, THROW_LIFT, THROW_SPEED, THROW_SPREAD, liftAt } from "../../../🪄️mischief/🟦️.ts";
import { MISCHIEF_STREAM, randomWords, unitOf } from "../../../🎲️randomness/🟦️.ts";
import { prankTick, prospectsOf } from "../../../🗓️schedule/🟦️.ts";
import { HARD_LANDING } from "../../../🪢️swing/🟦️.ts";
import { advance, frameOf, openStage } from "../../🟦️.ts";

const HERE = dirname(fileURLToPath(import.meta.url));
const SECOND = TICKS_PER_SECOND;
const WIDTH = 1280;
const HEIGHT = 720;
const FLOOR: Surface = { id: "floor", x0: 0, x1: WIDTH, y: HEIGHT };
const CARD: Surface = { id: "card", x0: 300, x1: 800, y: 400 };
const FAR = 100000000;
const COMFORT = 8;
const PARTS = ["🎪️stage", "📝️draft", "📏️spacing", "🗓️schedule", "👀️attention", "🚶️locomotion", "💞️sociability", "🎯️choice", "👥️population", "🕰️clock", "🎥️projection", "🧗️climbing", "🪄️mischief"];

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
function surveyed(surfaces: readonly Surface[], keepouts: readonly Rect[] = [], width = WIDTH, height = HEIGHT, walls: readonly Wall[] = [], fixtures: readonly Fixture[] = []): StageEvent {
  return { kind: "surveyed", width, height, surfaces, keepouts, walls, fixtures };
}

/** 👆️ A click of the learner's hand at a position: a press and its release. */
function clicked(x: number, y: number): StageEvent[] {
  return [
    { kind: "pressed", x, y, pointer: "mouse" },
    { kind: "released", x, y },
  ];
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

/** 🚧️ Every pair of actors whose bodies overlap on a stage (`overlaps` of their bodies): the invariant of the stage says there is none at the end of any tick. */
function collisions(menagerie: Menagerie, stage: Stage): string[] {
  return overlaps(bodiesOf(stage.actors, stage.actors.map((actor) => kindOf(menagerie, actor.species)))).map((pair) => `${pair.first}+${pair.second} at tick ${stage.tick}`);
}

/** 🧱️ Holds a stage to the invariant: it fails, naming the pairs, when bodies overlap (an assertion only then, so that a check after every tick stays cheap). */
function heldApart(menagerie: Menagerie, stage: Stage): void {
  const found = collisions(menagerie, stage);
  if (found.length > 0) expect(found).toEqual([]);
}

/** 🖼️ Every pair of visible actors whose drawings reach into each other by more than half a pixel both ways, as the browser spec `🐕️pet-walk` measures a drawing: the size box of its species with its feet at the actor's feet, turned by the frame's tilt about the feet (where the turn about the pivot carries the rig's feet), the axis-aligned box of its corners. Bodies that are apart keep their drawings apart only when a body holds its drawing. */
function drawnClashes(menagerie: Menagerie, stage: Stage): string[] {
  const boxes = frameOf(menagerie, stage)
    .actors.filter((shown) => shown.opacity > 0)
    .map((shown) => {
      const actor = actorOf(stage, shown.species);
      const size = kindOf(menagerie, shown.species).size;
      const sine = Math.sin(2 * Math.PI * shown.tilt);
      const cosine = Math.cos(2 * Math.PI * shown.tilt);
      const corners = [[-size.width / 2, -size.height], [size.width / 2, -size.height], [size.width / 2, 0], [-size.width / 2, 0]] as const;
      const xs = corners.map(([x, y]) => actor.x + x * cosine - y * sine);
      const ys = corners.map(([x, y]) => actor.y + x * sine + y * cosine);
      return { species: shown.species, x0: Math.min(...xs), y0: Math.min(...ys), x1: Math.max(...xs), y1: Math.max(...ys) };
    });
  const clashes: string[] = [];
  for (let one = 0; one < boxes.length; one++) {
    for (let other = one + 1; other < boxes.length; other++) {
      const [a, b] = [boxes[one]!, boxes[other]!];
      if (Math.min(a.x1, b.x1) - Math.max(a.x0, b.x0) > 0.5 && Math.min(a.y1, b.y1) - Math.max(a.y0, b.y0) > 0.5) clashes.push(`${a.species}+${b.species} drawn at tick ${stage.tick}`);
    }
  }
  return clashes;
}

/** 🏃️ A stage after `ticks` single ticks, every one of them held to the invariant ({@link heldApart}); `watch` sees every stage with the one before it. */
function run(menagerie: Menagerie, stage: Stage, ticks: number, watch?: (after: Stage, before: Stage) => void): Stage {
  let current = stage;
  for (let tick = 0; tick < ticks; tick++) {
    const next = advance(menagerie, current, [ticked()]);
    heldApart(menagerie, next);
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
  return craft(stage, species, { perch: CARD.id, x, y: CARD.y - hoverOf(kindOf(menagerie, species)), vx: 0, vy: 0, goal: x, activity: "idle", since: stage.tick, until: stage.tick + FAR, blink: stage.tick + FAR, partner: null, opacity: 1, leaving: false, feeling: atRest(kindOf(menagerie, species).mood, stage.tick), gaze: { x: 0, y: 0, vx: 0, vy: 0 }, ...changes });
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

  it("scoots back onto its perch when the perch shrinks, and steps aside for a keep-out, never jumping; it falls when its perch went farther than its own width away", () => {
    const stage = alone(company, species);
    const scoots = (after: Stage, before: Stage): void => {
      expect(Math.abs(actorOf(after, species).x - actorOf(before, species).x)).toBeLessThanOrEqual((1.5 * kind.locomotion.speed) / SECOND + 1e-9);
    };
    const shrunk = advance(menagerie, stage, [surveyed([{ ...CARD, x1: 560 }, FLOOR])]);
    expect(actorOf(shrunk, species)).toMatchObject({ x: 550, goal: 560 - kind.size.width / 2, perch: CARD.id, activity: "scoot", footing: "perch" });
    let seated: Actor | null = null;
    run(menagerie, shrunk, 2 * SECOND, (after, before) => {
      scoots(after, before);
      if (seated === null && actorOf(after, species).activity !== "scoot") seated = actorOf(after, species);
    });
    expect(seated).toMatchObject({ x: 560 - kind.size.width / 2, perch: CARD.id, activity: "idle", opacity: 1 });
    const keepout: Rect = { x: 520, y: 380, width: 60, height: 20 };
    const blocked = run(menagerie, advance(menagerie, stage, [surveyed([CARD, FLOOR], [keepout])]), 2 * SECOND, scoots);
    expect(covers(menagerie, actorOf(blocked, species), keepout)).toBe(false);
    expect(actorOf(blocked, species).perch).toBe(CARD.id);
    expect(Math.abs(actorOf(blocked, species).x - 550)).toBeLessThanOrEqual(30 + kind.size.width);
    const lost = advance(menagerie, stage, [surveyed([CARD, FLOOR], [{ x: 400, y: 380, width: 400, height: 20 }])]);
    expect(actorOf(lost, species)).toMatchObject({ footing: "air", activity: "fall", perch: null, x: 550 });
    const shoved = run(menagerie, lost, 8 * SECOND);
    expect(actorOf(shoved, species)).toMatchObject({ perch: FLOOR.id, opacity: 1, footing: "perch" });
  });
});

describe.each(COMPANIES)("falling and landing in $name", (company) => {
  const menagerie = company.menagerie;

  it.each([company.walker, company.floater])("lets %s fall when its perch vanishes, open its parachute on the way and float down to the floor, softly", (species) => {
    const kind = kindOf(menagerie, species);
    expect(kind.gear).toContain("parachute");
    let stage = advance(menagerie, alone(company, species), [surveyed([FLOOR])]);
    expect(actorOf(stage, species)).toMatchObject({ activity: "fall", footing: "air", perch: null, x: 550, vy: 0 });
    let opened = -1;
    let landed: Actor | null = null;
    let touch = 0;
    stage = run(menagerie, stage, 8 * SECOND, (after, before) => {
      const actor = actorOf(after, species);
      const earlier = actorOf(before, species);
      if (landed !== null || earlier.footing === "perch") return;
      expect(frameOf(menagerie, before).rate).toBe(64);
      if (opened < 0 && actor.footing === "chute") {
        opened = after.tick;
        expect(actor).toMatchObject({ activity: "glide", chute: { since: after.tick } });
      }
      if (actor.footing !== "perch") return;
      landed = actor;
      touch = (actor.y - earlier.y) * SECOND;
    });
    expect(opened).toBeGreaterThan(0);
    expect(landed).toMatchObject({ activity: "idle", footing: "perch", perch: FLOOR.id, y: HEIGHT - hoverOf(kind), chute: null, tilt: 0 });
    expect(Math.abs(landed!.x - 550)).toBeLessThan(kind.size.width);
    expect(touch).toBeLessThanOrEqual(600);
    expect(actorOf(stage, species).perch).toBe(FLOOR.id);
  });

  it("lands hard without a parachute: it falls straight, lands with its landing clip and is dizzy after a long fall", () => {
    const species = company.walker;
    const bare: Menagerie = { ...menagerie, species: menagerie.species.map((kind) => (kind.id === species ? { ...kind, gear: [] } : kind)) };
    let stage = advance(bare, alone({ ...company, menagerie: bare }, species), [surveyed([FLOOR])]);
    let fallen = 0;
    let landed: Actor | null = null;
    stage = run(bare, stage, 4 * SECOND, (after, before) => {
      const actor = actorOf(after, species);
      if (actorOf(before, species).activity === "fall" && actor.activity === "fall") {
        fallen++;
        expect(actor).toMatchObject({ x: 550, footing: "air", chute: null });
      }
      if (landed === null && actor.activity === "land") landed = actor;
    });
    expect(landed).toMatchObject({ footing: "perch", perch: FLOOR.id, x: 550, y: HEIGHT - hoverOf(kindOf(bare, species)) });
    expect(landed!.vy).toBeGreaterThan(600);
    expect(fallen).toBeGreaterThan(20);
    expect(fallen).toBeLessThan(50);
    const dizzy = run(bare, advance(bare, alone({ ...company, menagerie: bare }, species), [surveyed([FLOOR])]), 2 * SECOND);
    expect(["dizzy", "idle"]).toContain(actorOf(dizzy, species).activity);
    let wasDizzy = false;
    run(bare, advance(bare, alone({ ...company, menagerie: bare }, species), [surveyed([FLOOR])]), 4 * SECOND, (after) => {
      if (actorOf(after, species).activity === "dizzy") wasDizzy = true;
    });
    expect(wasDizzy).toBe(true);
  });

  it("never lands harder than a hard landing when it owns a parachute, from any height, thrown up, down or sideways", () => {
    const species = company.walker;
    const kind = kindOf(menagerie, species);
    for (const height of sampled([150, 400], [104, 150, 220, 400, 600], [60, 80, 104, 120, 150, 220, 300, 400, 500, 600])) {
      for (const [vx, vy] of sampled([[0, 0], [300, -400]], [[0, 0], [0, -400], [300, -400], [-500, 100], [200, 300]], [[0, 0], [0, -520], [0, -400], [300, -400], [-500, 100], [200, 300], [640, 0], [-200, -500]])) {
        let stage = staged(menagerie, [species], { surfaces: [FLOOR] });
        stage = craft(stage, species, { footing: "air", perch: null, activity: "tumble", x: 640, y: HEIGHT - height, vx, vy, goal: 640, until: stage.tick + 32, opacity: 1 });
        let landing = 0;
        run(menagerie, stage, 12 * SECOND, (after, before) => {
          const now = actorOf(after, species);
          const earlier = actorOf(before, species);
          if (earlier.footing !== "perch" && now.footing === "perch" && landing === 0) landing = (now.y - earlier.y) * SECOND;
        });
        if (landing > 600) expect({ height, vx, vy, landing }).toEqual({ height, vx, vy, landing: 600 });
        expect(kind.gear).toContain("parachute");
      }
    }
  });

  it("brakes when it is thrown down at the ground too close to it for its parachute to open, and lands no harder than a hard landing all the same", () => {
    const species = company.walker;
    for (const height of sampled([30], [10, 30, 50], [2, 10, 20, 30, 40, 50, 55])) {
      for (const vy of sampled([900], [650, 900], [610, 650, 900, 1200, 2000])) {
        let stage = staged(menagerie, [species], { surfaces: [FLOOR] });
        stage = craft(stage, species, { footing: "air", perch: null, activity: "tumble", x: 640, y: HEIGHT - height, vx: 0, vy, goal: 640, until: stage.tick + 32, opacity: 1 });
        let landing = 0;
        run(menagerie, stage, 2 * SECOND, (after, before) => {
          const now = actorOf(after, species);
          const earlier = actorOf(before, species);
          if (earlier.footing !== "perch" && now.footing === "perch" && landing === 0) landing = (now.y - earlier.y) * SECOND;
        });
        expect(landing, `${height} px above the ground at ${vy} px/s`).toBeGreaterThan(0);
        expect(landing, `${height} px above the ground at ${vy} px/s`).toBeLessThanOrEqual(600);
      }
    }
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

  it("is gone in a puff the stage counts when it falls out of the stage, and arrives anew once a perch exists", () => {
    const species = company.walker;
    const before = advance(menagerie, alone(company, species), [surveyed([])]);
    let stage = run(menagerie, before, 3 * SECOND);
    expect(stage.actors).toEqual([]);
    expect(stage.wanted).toEqual([species]);
    expect(stage.poofs).toBe(before.poofs + 1);
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

  /** 💑️ A stage on which two species stand on the card, a comfortable gap and 2 px apart, each waiting for the other. */
  function waiting(pair: readonly [Slug, Slug], draws: number): Stage {
    const apart = (kindOf(menagerie, pair[0]).size.width + kindOf(menagerie, pair[1]).size.width) / 2 + COMFORT + 2;
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
    const next = one.activity === "cuddle" ? "purr" : "idle";
    expect([actorOf(after, first).activity, actorOf(after, second).activity]).toEqual([next, next]);
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
      if (one.activity === "squabble") expect(spiritsOf(settled(one.feeling, kindOf(menagerie, first).mood, after.tick))).toBeLessThan(0.3);
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
          const reach = Math.max(kindOf(menagerie, one.species).reach + kindOf(menagerie, two.species).reach, COMFORT);
          expect(apart).toBeGreaterThanOrEqual(COMFORT);
          expect(apart).toBeLessThanOrEqual(reach + Math.max(hopOf(kindOf(menagerie, one.species)), hopOf(kindOf(menagerie, two.species))) + 1);
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
      for (const species of everyone) stage = craft(stage, species, { needs: { energy: 0.45, sociability: 0.5, curiosity: 0.5 } });
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
    stage = advance(menagerie, stage, [{ kind: "pointed", over: "free", x: 10, y: 10 }, ticked(20), { kind: "tuned", mode: "still" }]);
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
    const later = advance(menagerie, stage, [ticked(123456789), ...clicked(stage.actors[0]!.x, stage.actors[0]!.y - 10), { kind: "pointed", over: "free", x: 600, y: 300 }, ticked(1000)]);
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

describe.each(COMPANIES)("the learner's hand in $name", (company) => {
  const menagerie = company.menagerie;
  const everyone = menagerie.species.map((kind) => kind.id);
  const species = company.walker;
  it("lets a reclaimed fixture change nothing while no fixture is lifted", () => {
    for (const quiet of [false, true]) {
      const stage = advance(menagerie, alone(company, species), [{ kind: "hushed", quiet }]);
      expect(advance(menagerie, stage, [{ kind: "reclaimed", fixture: "card" }])).toEqual(stage);
    }
  });

  it("keeps what the learner permits and the ticks of the learner's last input and of the last scroll", () => {
    const stage = advance(menagerie, run(menagerie, alone(company, species), 10), [{ kind: "permitted", play: true, mischief: false }, { kind: "stirred" }]);
    expect([stage.play, stage.mischief, stage.stirred, stage.scrolled]).toEqual([true, false, stage.tick, 0]);
    const later = advance(menagerie, run(menagerie, stage, 5), [{ kind: "scrolled" }, { kind: "permitted", play: false, mischief: true }]);
    expect([later.play, later.mischief, later.stirred, later.scrolled]).toEqual([false, true, stage.tick, stage.tick + 5]);
    expect(later.actors).toEqual(run(menagerie, stage, 5).actors);
  });

  it("keeps the walls and the fixtures a survey brings, cuts the pitches of the walls, and nothing else of the stage changes with them", () => {
    const walls: Wall[] = [
      { id: "card-left", surface: CARD.id, side: -1, x: CARD.x0, y0: CARD.y, y1: CARD.y + 120 },
      { id: "card-right", surface: CARD.id, side: 1, x: CARD.x1, y0: CARD.y, y1: CARD.y + 120 },
    ];
    const fixtures: Fixture[] = [{ id: "task-1", key: "quiz/task", x: 320, y: 420, width: 200, height: 30 }];
    const plain = staged(menagerie, everyone, { seed: 4 });
    const walled = advance(menagerie, plain, [surveyed([CARD, FLOOR], [], WIDTH, HEIGHT, walls, fixtures)]);
    expect([walled.walls, walled.fixtures]).toEqual([walls, fixtures]);
    expect(walled.pitches.map((pitch) => pitch.wall)).toEqual(["card-left", "card-right"]);
    for (const pitch of walled.pitches) expect(walls.some((wall) => wall.id === pitch.wall && wall.surface === pitch.surface && wall.x === pitch.x && wall.side === pitch.side && wall.y0 === pitch.y0 && wall.y1 === pitch.y1)).toBe(true);
    expect({ ...walled, walls: [], fixtures: [], pitches: [] }).toEqual(advance(menagerie, plain, [surveyed([CARD, FLOOR])]));
  });
});

describe.each(COMPANIES)("what an actor arrives with and stands on in $name", (company) => {
  const menagerie = company.menagerie;
  const everyone = menagerie.species.map((kind) => kind.id);

  it("arrives on its perch, at rest in the resting mood and in the first state of its species, untouched and empty-handed", () => {
    const stage = staged(menagerie, everyone, { seed: 21 });
    expect(stage.actors.length).toBeGreaterThan(0);
    for (const actor of stage.actors) {
      const kind = kindOf(menagerie, actor.species);
      expect(actor).toMatchObject({ footing: "perch", feeling: atRest(kind.mood, stage.tick), state: kind.states[0]!.id, stateSince: stage.tick, trick: null, warmth: COLD, hover: noHover(0), hang: null, chute: null, rope: null, emitters: [] });
    }
  });

  it("stands on a perch exactly while it has one, whatever happens to the stage", () => {
    let stage = staged(menagerie, everyone, { seed: 13, mode: "lively" });
    let airborne = 0;
    const strays: string[] = [];
    const steady = (after: Stage): void => {
      for (const actor of after.actors) {
        if ((actor.footing === "perch") !== (actor.perch !== null)) strays.push(`${actor.species} at ${after.tick} on ${actor.perch} by ${actor.footing}`);
        if (actor.perch === null) airborne++;
      }
    };
    const happenings: Record<number, StageEvent[]> = {
      20: [surveyed([FLOOR])],
      30: [surveyed([CARD, FLOOR])],
      45: [{ kind: "summoned", species: everyone.slice(0, 2) }],
      50: [{ kind: "summoned", species: everyone }],
      70: [{ kind: "tuned", mode: "still" }],
      72: [{ kind: "tuned", mode: "lively" }],
    };
    for (let second = 0; second < sampled(90, 240, 240); second++) {
      stage = advance(menagerie, stage, happenings[second] ?? []);
      steady(stage);
      stage = run(menagerie, stage, SECOND, steady);
    }
    expect(strays).toEqual([]);
    expect(airborne).toBeGreaterThan(0);
  });
});

describe.each(COMPANIES)("what a frame shows of every actor in $name", (company) => {
  const menagerie = company.menagerie;
  const everyone = menagerie.species.map((kind) => kind.id);

  it("draws it upright about its grip and empty-handed, with its footing, its state, the mood it feels with its intensity and its spirits, and the box the stage keeps clear as its body", () => {
    let stage = run(menagerie, staged(menagerie, everyone, { seed: 17, mode: "lively" }), 20 * SECOND);
    const moods = new Set<string>();
    for (let tick = 0; tick < sampled(400, 2000, 2000); tick++) {
      stage = advance(menagerie, stage, [ticked()]);
      const frame = frameOf(menagerie, stage);
      expect([frame.ladders, frame.particles, frame.lifts, frame.held]).toEqual([[], [], [], null]);
      for (const drawn of frame.actors) {
        const actor = actorOf(stage, drawn.species);
        const kind = kindOf(menagerie, drawn.species);
        const felt = settled(actor.feeling, kind.mood, stage.tick);
        moods.add(drawn.mood);
        expect(drawn).toMatchObject({
          footing: actor.footing,
          state: actor.state,
          spirits: spiritsOf(felt),
          intensity: felt.intensity,
          mood: felt.mood,
          tilt: 0,
          pivot: { x: 0, y: -kind.grip },
          tools: [],
        });
        const extent = extentOf(actor, kind);
        expect(drawn.body).toEqual({ x: extent.x0, y: extent.y0, width: extent.x1 - extent.x0, height: extent.y1 - extent.y0 });
      }
    }
    expect(moods.size).toBeGreaterThan(0);
  });
});

describe.each(COMPANIES)("keeping their distance in $name", (company) => {
  const menagerie = company.menagerie;
  const everyone = menagerie.species.map((kind) => kind.id);
  const widthOf = (id: Slug): number => kindOf(menagerie, id).size.width;
  const shoulders = (one: Slug, two: Slug): number => (widthOf(one) + widthOf(two)) / 2;

  /** 👥️ Every pair of actors whose bodies overlap, wherever they are. */
  function overlapping(stage: Stage): string[] {
    return collisions(menagerie, stage);
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
      const newcomers: string[] = [];
      stage = run(menagerie, stage, 5 * SECOND, (after) => newcomers.push(...after.actors.filter((actor) => !arrived.includes(actor.species)).map((actor) => `${actor.species} at ${after.tick}`)));
      expect(newcomers).toEqual([]);
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

  it("stops a walker before a neighbour: bodies never closer than the comfortable gap, and nobody is walked through", () => {
    const [first, second] = company.strangers;
    for (const side of [1, -1] as const) {
      let stage = staged(menagerie, [first, second], { mode: "lively", surfaces: [CARD] });
      stage = seated(menagerie, stage, second, 550);
      stage = seated(menagerie, stage, first, 550 - side * 150, { activity: "walk", goal: 550 + side * 150, facing: side, clip: kindOf(menagerie, first).repertoire.walk?.[0] ?? kindOf(menagerie, first).repertoire.hop?.[0] ?? null });
      let stopped: Actor | null = null;
      run(menagerie, stage, 20 * SECOND, (after, before) => {
        if (stopped !== null) return;
        const walker = actorOf(after, first);
        expect((550 - walker.x) * side).toBeGreaterThanOrEqual(shoulders(first, second) + COMFORT);
        if (actorOf(before, first).activity === "walk" && walker.activity !== "walk") stopped = walker;
      });
      expect(stopped).not.toBeNull();
      expect(stopped!.activity).toBe("idle");
      expect((550 - stopped!.x) * side).toBeLessThan(shoulders(first, second) + COMFORT + Math.max(hopOf(kindOf(menagerie, first)), 1));
      expect(stopped!.goal).toBe(stopped!.x);
    }
  });

  it("never chooses a goal beyond or inside a neighbour, and never lets two bodies overlap however lively the stage is", () => {
    const narrow: Surface = { ...CARD, x0: 200, x1: 200 + needOf(everyone) + 3 * widest };
    const trouble: string[] = [];
    let walks = 0;
    for (let seed = 1; seed <= sampled(2, 6, 12); seed++) {
      run(menagerie, staged(menagerie, everyone, { seed, mode: "lively", surfaces: [narrow] }), sampled(60, 300, 300) * SECOND, (after, before) => {
        for (const actor of after.actors) {
          const earlier = before.actors.find((candidate) => candidate.species === actor.species);
          if (earlier === undefined || actor.activity !== "walk" || earlier.activity === "walk") continue;
          walks++;
          for (const other of after.actors) {
            if (other.species === actor.species || other.species === actor.partner || other.perch !== actor.perch) continue;
            if ((other.x - earlier.x) * (other.x - actor.goal) <= 0) trouble.push(`${actor.species} aims past ${other.species} at tick ${after.tick}`);
            if (Math.abs(other.x - actor.goal) < shoulders(actor.species, other.species) + COMFORT) trouble.push(`${actor.species} aims into ${other.species} at tick ${after.tick}`);
          }
        }
      });
    }
    expect(trouble).toEqual([]);
    expect(walks).toBeGreaterThan(5);
  });

  it("seats everyone anew when a perch shrinks — scooting, never jumping —, lets whoever does not fit fall off its end or vanish in a puff, and lets it return when there is room", () => {
    for (let seed = 1; seed <= sampled(2, 6, 24); seed++) {
      const wide = run(menagerie, staged(menagerie, everyone, { seed, surfaces: [FLOOR] }), 32);
      const shrunk = advance(menagerie, wide, [surveyed([STRIP])]);
      expect(overlapping(shrunk)).toEqual([]);
      const staying = shrunk.actors.filter((actor) => actor.footing === "perch");
      const gone = everyone.filter((species) => !shrunk.actors.some((actor) => actor.species === species));
      expect(staying.length).toBeLessThan(everyone.length);
      expect(needOf(staying.map((actor) => actor.species))).toBeLessThanOrEqual(STRIP.x1);
      for (const actor of staying) expect(Math.abs(actor.goal - actorOf(wide, actor.species).x)).toBeLessThanOrEqual(2 * widthOf(actor.species) + widest + COMFORT);
      expect(shrunk.poofs - wide.poofs).toBe(shrunk.puffs.length - wide.puffs.length);
      for (const species of gone) expect(shrunk.puffs.some((puff) => puff.tick === shrunk.tick && Math.abs(puff.x - actorOf(wide, species).x) < 1e-9)).toBe(true);
      for (const actor of shrunk.actors) if (actor.footing !== "perch") expect(actor).toMatchObject({ footing: "air", activity: "fall", partner: null });
      const later = run(menagerie, shrunk, 6 * SECOND, (after, before) => {
        for (const actor of after.actors) {
          const earlier = before.actors.find((candidate) => candidate.species === actor.species);
          if (earlier !== undefined && earlier.activity === "scoot") expect(Math.abs(actor.x - earlier.x)).toBeLessThanOrEqual((1.5 * kindOf(menagerie, actor.species).locomotion.speed) / SECOND + 1e-9);
        }
      });
      for (const actor of staying) expect(later.actors.map((candidate) => candidate.species)).toContain(actor.species);
      expect(later.actors.length).toBeGreaterThanOrEqual(1);
      expect(later.actors.length).toBeLessThan(everyone.length);
      for (const actor of later.actors) {
        expect(actor).toMatchObject({ perch: STRIP.id, footing: "perch", leaving: false });
        expect(actor.x).toBeGreaterThanOrEqual(widthOf(actor.species) / 2);
        expect(actor.x).toBeLessThanOrEqual(STRIP.x1 - widthOf(actor.species) / 2);
      }
      expect(later.wanted).toEqual(everyone);
      const again = advance(menagerie, shrunk, [surveyed([STRIP])]);
      expect(again.actors.filter((actor) => actor.footing === "perch")).toEqual(staying);
      const widened = run(menagerie, advance(menagerie, later, [surveyed([FLOOR])]), SECOND);
      expect(widened.actors.map((actor) => actor.species)).toEqual(everyone);
    }
  });

  it("keeps neighbours as close as they were when a ride only moves them, seats them apart when it squeezes them, and lets whoever a ride carried into somebody vanish", () => {
    const [first, second] = company.friends;
    let stage = staged(menagerie, [first, second], { surfaces: [CARD] });
    stage = seated(menagerie, seated(menagerie, stage, first, 500), second, 500 + shoulders(first, second) + COMFORT + 2);
    const moved = advance(menagerie, stage, [surveyed([{ ...CARD, x0: CARD.x0 + 37, x1: CARD.x1 + 37 }])]);
    expect(actorOf(moved, second).x - actorOf(moved, first).x).toBe(actorOf(stage, second).x - actorOf(stage, first).x);
    const edge = 500 + shoulders(first, second) + widthOf(second) / 2 + 3;
    const squeezed = run(menagerie, advance(menagerie, stage, [surveyed([{ ...CARD, x1: edge }])]), 2 * SECOND);
    expect(squeezed.actors.every((actor) => !actor.leaving && actor.footing === "perch")).toBe(true);
    expect(actorOf(squeezed, second).x).toBeLessThanOrEqual(edge - widthOf(second) / 2);
    expect(actorOf(squeezed, second).x - actorOf(squeezed, first).x).toBeGreaterThanOrEqual(shoulders(first, second) + COMFORT);
    const stacked = craft(stage, second, { x: 505, goal: 505 });
    const parted = advance(menagerie, stacked, [surveyed([{ ...CARD, x0: CARD.x0 + 1 }])]);
    expect(overlapping(parted)).toEqual([]);
    expect(parted.poofs).toBe(stacked.poofs + 1);
    expect(parted.puffs.length).toBe(1);
    expect(parted.actors.filter((actor) => actor.opacity === 0).length).toBe(1);
  });

  it("comes down beside whoever stands where it lands — on its head first, sliding off —, and vanishes when the perch is full", () => {
    const [first, second] = company.strangers;
    const hover = hoverOf(kindOf(menagerie, first));
    let stage = staged(menagerie, [first, second], { surfaces: [FLOOR] });
    stage = craft(stage, second, { perch: FLOOR.id, x: 550, y: HEIGHT - hoverOf(kindOf(menagerie, second)), goal: 550, activity: "idle", since: stage.tick, until: stage.tick + FAR, blink: stage.tick + FAR, opacity: 1 });
    const aimed = 550 + shoulders(first, second) - 4;
    const falling: Partial<Actor> = { perch: null, footing: "air", x: aimed, y: HEIGHT - 120 - hover, vx: 0, vy: 0, goal: aimed, activity: "fall", since: stage.tick, until: stage.tick + 32, opacity: 1 };
    let rode = false;
    const landed = run(menagerie, craft(stage, first, falling), 4 * SECOND, (after) => {
      if (actorOf(after, first).footing === "head") rode = true;
    });
    expect(rode).toBe(true);
    expect(actorOf(landed, first)).toMatchObject({ perch: FLOOR.id, footing: "perch", leaving: false, y: HEIGHT - hover });
    expect(Math.abs(actorOf(landed, first).x - 550)).toBeGreaterThanOrEqual(shoulders(first, second) + COMFORT);
    const room = Math.max(widthOf(first), widthOf(second)) / 2 + 2;
    const full: Surface = { id: "ledge", x0: 550 - room, x1: 550 + room, y: HEIGHT };
    let cramped = advance(menagerie, craft(stage, first, { ...falling, x: 553, goal: 553 }), [surveyed([full])]);
    expect(actorOf(cramped, second)).toMatchObject({ perch: full.id, x: 550, leaving: false });
    cramped = run(menagerie, cramped, 4 * SECOND);
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

  /** 🪜️ How many hops between the two shelves a lonely pet takes under a keep-out, and whether it ever covered it: in five lively minutes for each of three seeds, of eight at the exhaustive level, and at the fundamental level in one minute for seeds 4 and 7 (seed 7 is the one that hops that soon in both companies). It starts on the lower shelf, beside the upper one (a newcomer by itself would arrive on the upper one, wherever on it). */
  function hops(keepouts: readonly Rect[]): { readonly count: number; readonly covered: number } {
    let count = 0;
    let covered = 0;
    for (const seed of sampled([4, 7], [1, 2, 3], [1, 2, 3, 4, 5, 6, 7, 8])) {
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
    stage = seated(menagerie, stage, second, 450 + (kindOf(menagerie, first).size.width + kindOf(menagerie, second).size.width) / 2 + COMFORT + 2, { partner: first, facing: 1 });
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
    const greeted = (before: Stage, x: number): Stage => {
      const draft = draftOf(plain, before);
      hail(draft, indexOf(draft.actors, species), x, draft.tick);
      return sealed(draft);
    };
    let stage = greeted(alone({ ...company, menagerie: plain }, species, { facing: 1 }), 530);
    expect(actorOf(stage, species)).toMatchObject({ facing: -1, faced: stage.tick + 8, activity: "greet" });
    stage = run(plain, stage, 3);
    const before = frameOf(plain, stage).actors[0]!;
    expect(before.bones[0]).toBe(squeezeAt(3));
    stage = greeted(stage, 570);
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

describe.each(COMPANIES)("determinism in $name", (company) => {
  const menagerie = company.menagerie;
  const everyone = menagerie.species.map((kind) => kind.id);

  /** 📽️ The frames of a little story: an arrival, a pointer, a vanishing card, a click, a quiet time. */
  function story(seed: number, ticks: number): Frame[] {
    const frames: Frame[] = [];
    let stage = staged(menagerie, everyone, { seed, mode: "lively" });
    for (let tick = 1; tick <= ticks; tick++) {
      const events: StageEvent[] = [];
      if (tick === 200) events.push({ kind: "pointed", over: "free", x: 400, y: 300 });
      if (tick === 900) events.push(surveyed([FLOOR]));
      if (tick === 1300) events.push(...clicked(640, 700));
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
      const events: StageEvent[] = [{ kind: "pointed", over: "free", x: 500, y: 350 }, { kind: "glanced", points: [{ x: 100, y: 100 }] }];
      const tail = sampled(500, 3000, 3000);
      stage = run(menagerie, advance(menagerie, stage, events), tail);
      chunked = advance(menagerie, chunked, [...events, ticked((2 * tail) / 5), ticked((3 * tail) / 5)]);
      expect(chunked).toEqual(stage);
    }
  });

  it("jumps over a quiet hour of sleep without stepping through it", () => {
    let stage = staged(menagerie, everyone, { seed: 1 });
    for (const species of everyone) stage = craft(stage, species, { activity: "sleep", until: stage.tick + FAR, opacity: 1, feeling: atRest(kindOf(menagerie, species).mood, stage.tick) });
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
      const events = frozen<StageEvent[]>([{ kind: "pointed", over: "free", x: 10 * round, y: 300 }, ticked(97), surveyed(round % 7 === 3 ? [FLOOR] : [CARD, FLOOR]), ...clicked(30 * round, 700), { kind: "permitted", play: round % 2 === 0, mischief: true }, { kind: "stirred" }, ticked(31)]);
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
    stage = run(menagerie, craft(stage, species, { activity: "sleep", since: stage.tick - 100, clip: kindOf(menagerie, species).repertoire.sleep![0]! }), 1);
    expect(actorOf(stage, species).gaze.x).toBeGreaterThan(0);
    expect(frameOf(menagerie, stage).rate).toBe(32);
    stage = run(menagerie, stage, SECOND);
    expect(actorOf(stage, species).gaze).toEqual({ x: 0, y: 0, vx: 0, vy: 0 });
    expect(frameOf(menagerie, stage)).toMatchObject({ rate: 16, wake: null });
    expect(frameOf(menagerie, craft(stage, species, { feeling: { mood: "happy", intensity: 0.5, since: stage.tick } })).rate).toBe(32);
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
      for (const drawn of frame.actors) if (drawn.bones.length !== 6 * kindOf(menagerie, drawn.species).bones.length || !drawn.bones.every((number) => Number.isFinite(number)) || !(Math.abs(drawn.spirits) <= 1)) finite = false;
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
    const pointed = advance(plain, stage, [{ kind: "pointed", over: "free", x: 100, y: 100 }]);
    expect(frameOf(plain, pointed).rate).toBe(32);
  });

  it("blends the clip of an activity in over its first eight ticks, on top of the idle loop that never stops", () => {
    const kind = kindOf(menagerie, species);
    const greet = kind.repertoire.greet![0]!;
    const idle = alone(company, species, { clip: kind.repertoire.idle![0]! });
    const greeting = craft(idle, species, { activity: "greet", clip: greet, since: idle.tick, until: idle.tick + 200 });
    expect(frameOf(menagerie, greeting).actors[0]!.bones).toEqual(frameOf(menagerie, idle).actors[0]!.bones);
    expect(frameOf(menagerie, greeting).rate).toBe(64);
    const later = { ...greeting, tick: greeting.tick + 30 };
    expect(frameOf(menagerie, later).actors[0]!.bones).not.toEqual(frameOf(menagerie, { ...idle, tick: idle.tick + 30 }).actors[0]!.bones);
    const ending = { ...greeting, tick: greeting.tick + 200 };
    expect(frameOf(menagerie, ending).actors[0]!.bones).toEqual(frameOf(menagerie, { ...idle, tick: idle.tick + 200 }).actors[0]!.bones);
  });
});

describe.each(COMPANIES)("the mind of $name: clicks, gestures, moods, states and chemistry", (company) => {
  const menagerie = company.menagerie;
  const everyone = menagerie.species.map((kind) => kind.id);
  const species = company.walker;
  const kind = kindOf(menagerie, species);
  const resting = kind.states[0]!.id;
  const PLAY: StageEvent = { kind: "permitted", play: true, mischief: false };
  const ticksOf = (clip: Slug): number => {
    const found = kind.clips.find((entry) => entry.id === clip)!;
    return found.loop ? -1 : Math.max(Math.floor(found.seconds * SECOND + 0.5), 1);
  };
  const shown = (stage: Stage, who: Slug): string => {
    const felt = settled(actorOf(stage, who).feeling, kindOf(menagerie, who).mood, stage.tick);
    return felt.intensity > 0.25 ? felt.mood : "content";
  };
  const playing = (changes: Partial<Actor> = {}): Stage => advance(menagerie, alone(company, species, changes), [PLAY]);
  const centre = (stage: Stage): { x: number; y: number } => ({ x: actorOf(stage, species).x, y: actorOf(stage, species).y - kind.size.height / 2 });
  const circling = (stage: Stage, way: 1 | -1, turns = 2.5): Stage => {
    const { x, y } = centre(stage);
    const radius = 1.8 * Math.max(kind.size.width, kind.size.height);
    let current = stage;
    for (let tick = 0; tick <= turns * SECOND && actorOf(current, species).activity !== "trick"; tick++) {
      const angle = (way * 2 * Math.PI * tick) / SECOND;
      current = advance(menagerie, current, [{ kind: "pointed", over: "free", x: Math.round(4 * (x + radius * Math.cos(angle))) / 4, y: Math.round(4 * (y + radius * Math.sin(angle))) / 4 }, ticked()]);
    }
    return advance(menagerie, current, [{ kind: "unpointed" }]);
  };

  it("answers clicks half a second apart with a hello, its click tricks in order, a purr drawn out by every click, and then enough: a shrug and, until it has forgiven, a glance", () => {
    let stage = playing();
    const { x, y } = centre(stage);
    const offered = kind.tricks.filter((trick) => trick.cues.includes("click") && (trick.from === undefined || trick.from.includes(resting)));
    const tiers: string[] = [];
    const answers: string[] = [];
    let shrugged = 0;
    for (let click = 0; click < 10; click++) {
      stage = advance(menagerie, stage, clicked(x, y));
      const actor = actorOf(stage, species);
      tiers.push(actor.warmth.tier);
      answers.push(actor.activity === "trick" ? `trick:${actor.trick}` : actor.activity);
      if (actor.activity === "trick" && actor.since === stage.tick) expect(actor.until - actor.since).toBe(ticksOf(actor.clip!));
      if (actor.activity === "purr") expect(actor.until).toBeGreaterThanOrEqual(stage.tick + 192);
      if (click === 8) shrugged = stage.tick;
      stage = run(menagerie, stage, 32);
    }
    const trick = (index: number): string => (offered.length === 0 ? "greet" : `trick:${offered[index % offered.length]!.id}`);
    expect(tiers).toEqual(["hello", "trick", "trick", "purr", "purr", "purr", "purr", "purr", "enough", "enough"]);
    expect(answers).toEqual(["greet", trick(0), trick(1), "purr", "purr", "purr", "purr", "purr", "shrug", "shrug"]);
    expect(shown(stage, species)).toBe("grumpy");
    stage = run(menagerie, stage, shrugged + 512 - stage.tick);
    stage = advance(menagerie, stage, clicked(x, y));
    expect(actorOf(stage, species).warmth.tier).toBe("trick");
    expect(actorOf(stage, species).activity).toBe(offered.length === 0 ? "greet" : "trick");
  });

  it("ignores clicks on a still stage, while play is not permitted and for a pet nobody pressed on, and answers them in a time of concentration", () => {
    const stage = playing();
    const { x, y } = centre(stage);
    const before = actorOf(stage, species);
    expect(actorOf(advance(menagerie, stage, [{ kind: "permitted", play: false, mischief: false }, ...clicked(x, y)]), species).warmth).toEqual(before.warmth);
    expect(actorOf(advance(menagerie, stage, [{ kind: "tuned", mode: "still" }, ...clicked(x, y)]), species).warmth.heat).toBe(0);
    expect(actorOf(advance(menagerie, stage, clicked(x + 300, y)), species).warmth).toEqual(before.warmth);
    const hushed = advance(menagerie, stage, [{ kind: "hushed", quiet: true }, ...clicked(x, y)]);
    expect([actorOf(hushed, species).warmth.tier, actorOf(hushed, species).activity]).toEqual(["hello", "greet"]);
  });

  it("performs the trick that circling clockwise asks for and climbs a rung when it has played out; counter-clockwise climbs back down", () => {
    let stage = circling(playing(), 1);
    const up = kind.tricks.find((trick) => trick.cues.includes("circle") && (trick.from === undefined || trick.from.includes(resting)))!;
    const actor = actorOf(stage, species);
    expect([actor.activity, actor.trick, actor.state]).toEqual(["trick", up.id, resting]);
    stage = run(menagerie, stage, actor.until - stage.tick);
    const climbed = actorOf(stage, species).state;
    expect(climbed).not.toBe(resting);
    expect(kind.states.findIndex((state) => state.id === climbed)).toBeGreaterThan(0);
    const emitter = kind.states.find((state) => state.id === climbed)!.emitter;
    if (emitter !== undefined) expect(actorOf(stage, species).emitters).toContainEqual({ emitter, since: stage.tick, until: null });
    stage = run(menagerie, stage, 3 * SECOND);
    stage = circling(stage, -1);
    const down = actorOf(stage, species);
    expect(down.activity).toBe("trick");
    expect(kind.tricks.find((trick) => trick.id === down.trick)!.cues).toContain("countercircle");
    stage = run(menagerie, stage, down.until - stage.tick);
    expect(actorOf(stage, species).state).toBe(resting);
    expect(frameOf(menagerie, stage).actors[0]!.state).toBe(climbed);
    expect(frameOf(menagerie, run(menagerie, stage, STATE_BLEND / 2)).actors[0]!.state).toBe(resting);
  });

  it("gives way from a lasting state to the next when its time is up, the emitters following, whether time passes tick by tick or in one jump", () => {
    const lasting = kind.states.find((state) => state.lasts !== undefined)!;
    const stage = playing({ state: lasting.id, stateSince: 0 });
    const span = Math.floor(lasting.lasts! * SECOND + 0.5);
    const later = run(menagerie, { ...stage, actors: stage.actors.map((actor) => ({ ...actor, stateSince: stage.tick, emitters: lasting.emitter === undefined ? [] : [{ emitter: lasting.emitter, since: stage.tick, until: null }] })) }, span + 1);
    expect(actorOf(later, species).state).toBe(lasting.then);
    expect(actorOf(later, species).stateSince).toBe(stage.tick + span);
    const jumped = advance(menagerie, { ...stage, actors: stage.actors.map((actor) => ({ ...actor, stateSince: stage.tick })) }, [ticked(span + 1)]);
    expect(actorOf(jumped, species).state).toBe(lasting.then);
  });

  it("purrs when it is stroked, and keeps purring while the stroking goes on", () => {
    let stage = playing();
    const { x, y } = centre(stage);
    const strokes = kind.tricks.some((trick) => trick.cues.includes("stroke"));
    let purred = 0;
    for (let tick = 0; tick <= 4 * SECOND; tick++) {
      const across = Math.round(4 * (x + 0.5 * kind.size.width * Math.sin((2 * Math.PI * 2 * tick) / SECOND))) / 4;
      stage = advance(menagerie, stage, [{ kind: "pointed", over: "free", x: across, y }, ticked()]);
      if (actorOf(stage, species).activity === (strokes ? "trick" : "purr")) purred++;
    }
    expect(purred).toBeGreaterThan(SECOND);
    expect(shown(stage, species)).not.toBe("grumpy");
    const control = advance(menagerie, playing(), [{ kind: "pointed", over: "control", x, y }]);
    let ignored = control;
    for (let tick = 0; tick <= 2 * SECOND; tick++) ignored = advance(menagerie, ignored, [{ kind: "pointed", over: "control", x: Math.round(4 * (x + 0.5 * kind.size.width * Math.sin((2 * Math.PI * 2 * tick) / SECOND))) / 4, y }, ticked()]);
    expect(actorOf(ignored, species).activity).not.toBe("purr");
  });

  it("follows no gesture in a time of concentration, on a still stage or while play is not permitted", () => {
    for (const events of [[{ kind: "hushed", quiet: true }], [{ kind: "tuned", mode: "still" }], [{ kind: "permitted", play: false, mischief: false }]] as StageEvent[][]) {
      const stage = circling(advance(menagerie, playing(), events), 1);
      expect(actorOf(stage, species).activity, JSON.stringify(events)).not.toBe("trick");
    }
  });

  it("holds still while the pointer circles it: it sets off on no walk, no hop, no gear route and no encounter of its own until the circle is given up or times out — then it moves again", () => {
    let during = 0;
    let after = 0;
    const urge = (stage: Stage, tick: number): Stage => (tick >= 40 && tick % 8 === 0 && stage.actors.find((actor) => actor.species === species)?.activity === "idle" ? craft(stage, species, { until: stage.tick + 1 }) : stage);
    const moving = (stage: Stage): boolean => {
      const actor = stage.actors.find((candidate) => candidate.species === species);
      return actor === undefined || actor.leaving || actor.activity === "walk" || actor.activity === "hop" || stage.trips.length > 0 || actor.partner !== null;
    };
    for (let seed = 1; seed <= sampled(3, 6, 12); seed++) {
      let stage = advance(menagerie, seated(menagerie, staged(menagerie, [species], { seed, mode: "lively" }), species, 550, { needs: { energy: 1, sociability: 0.5, curiosity: 1 } }), [PLAY]);
      const { x, y } = centre(stage);
      const reach = Math.max(kind.size.width, kind.size.height) / 2 + 20;
      for (let tick = 0; tick < 12 * SECOND; tick++) {
        const swing: StageEvent[] = tick % 16 === 0 ? [{ kind: "pointed", over: "free", x: x + 0.7 * reach, y: y + (tick % 32 === 0 ? -0.7 : 0.7) * reach }] : [];
        stage = advance(menagerie, urge(stage, tick), [...swing, ticked()]);
        if (tick >= 40 && moving(stage)) during++;
      }
      expect(actorOf(stage, species).hover.circling.steps).toBeGreaterThan(0);
      stage = advance(menagerie, stage, [{ kind: "unpointed" }]);
      for (let tick = 40; tick < 12 * SECOND; tick++) {
        stage = advance(menagerie, urge(stage, tick), [ticked()]);
        if (moving(stage)) after++;
      }
    }
    expect(during).toBe(0);
    expect(after).toBeGreaterThan(0);
  });

  it("lets moods travel between neighbours, never above the mood they caught, and keeps every feeling inside [0, 1]", () => {
    let stage = staged(menagerie, everyone, { seed: 9, surfaces: [CARD] });
    const giver = company.friends[0];
    stage = craft(stage, giver, { feeling: { mood: "playful", intensity: 0.9, since: stage.tick } });
    let caught = 0;
    stage = run(menagerie, stage, 10 * SECOND, (after) => {
      for (const actor of after.actors) {
        const felt = settled(actor.feeling, kindOf(menagerie, actor.species).mood, after.tick);
        expect(felt.intensity).toBeGreaterThanOrEqual(0);
        expect(felt.intensity).toBeLessThanOrEqual(1);
        if (actor.species !== giver && felt.mood === "playful" && felt.intensity > 0.25) {
          caught++;
          expect(felt.intensity).toBeLessThanOrEqual(0.9);
        }
      }
    });
    expect(caught).toBeGreaterThan(0);
  });

  it("lets the chemistry of the menagerie act on its beat: a pet that has held its state long enough warms a friend near it and promises them a cuddle", () => {
    const reaction = menagerie.chemistry.find((entry) => entry.affinity !== undefined && entry.when.held !== undefined)!;
    const [warmer, friend] = company.friends;
    expect(reaction.when.species).toBe(warmer);
    let stage = staged(menagerie, [warmer, friend], { seed: 2, surfaces: [CARD] });
    stage = seated(menagerie, stage, warmer, 500, { state: reaction.when.state!, stateSince: stage.tick });
    stage = seated(menagerie, stage, friend, 560);
    const held = Math.floor(reaction.when.held! * SECOND + 0.5);
    const before = run(menagerie, stage, held - 2);
    expect(before.pledges).toEqual([]);
    const after = run(menagerie, before, 64);
    expect(after.pledges).toEqual([{ between: [friend, warmer], encounter: "cuddle", until: expect.any(Number) }]);
    expect(shown(after, friend)).toBe("happy");
    expect(after.coolings.map((cooling) => cooling.reaction)).toContain(reaction.id);
  });

  it("performs on a whim now and then in a lively mood, and never in a time of concentration", () => {
    const whims = (quiet: boolean): number => {
      let stage = advance(menagerie, staged(menagerie, everyone, { seed: 21, mode: "lively" }), [{ kind: "hushed", quiet }]);
      let count = 0;
      run(menagerie, stage, sampled(5, 8, 8) * 60 * SECOND, (after, previous) => {
        for (const actor of after.actors) {
          const earlier = previous.actors.find((entry) => entry.species === actor.species);
          if (actor.activity === "trick" && (earlier === undefined || earlier.activity !== "trick")) count++;
        }
        stage = after;
      });
      return count;
    };
    expect(whims(false)).toBeGreaterThan(0);
    expect(whims(true)).toBe(0);
  });

  it("answers the deeds of the keyboard: a hello, the next click trick, a purr — and nothing on a still stage", () => {
    const stage = playing();
    const offered = kind.tricks.filter((trick) => trick.cues.includes("click") && (trick.from === undefined || trick.from.includes(resting)));
    expect(actorOf(advance(menagerie, stage, [{ kind: "played", species, deed: "hello" }]), species).activity).toBe("greet");
    expect(actorOf(advance(menagerie, stage, [{ kind: "played", species, deed: "pet" }]), species).activity).toBe("purr");
    const tricked = actorOf(advance(menagerie, stage, [{ kind: "played", species, deed: "trick" }]), species);
    if (offered.length > 0) expect([tricked.activity, tricked.trick]).toEqual(["trick", offered[0]!.id]);
    const still = advance(menagerie, stage, [{ kind: "tuned", mode: "still" }]);
    expect(advance(menagerie, still, [{ kind: "played", species, deed: "hello" }])).toEqual(still);
  });

  it("feels a shake of the hand: frightened, and the shake trick of its species happens to it without its clip", () => {
    const stage = playing();
    const draft = draftOf(menagerie, stage);
    const index = indexOf(draft.actors, species);
    shaken(draft, index, stage.tick);
    const after = sealed(draft);
    const trick = kind.tricks.find((entry) => entry.cues.includes("shake") && (entry.from === undefined || entry.from.includes(resting)));
    expect(actorOf(after, species).activity).toBe("idle");
    expect(shown(after, species)).toBe("scared");
    if (trick?.to !== undefined) expect([actorOf(after, species).state, actorOf(after, species).stateSince]).toEqual([trick.to, stage.tick]);
  });

  it("comes out the same however time is cut, with clicks, circles and beats", () => {
    const start = advance(menagerie, staged(menagerie, everyone, { seed: 13, mode: "lively" }), [PLAY]);
    const { x, y } = centre(start);
    const events: StageEvent[] = [...clicked(x, y), ticked(40), ...clicked(x, y), ticked(300)];
    let stepped = start;
    for (const event of events) stepped = event.kind === "ticked" ? run(menagerie, stepped, event.ticks) : advance(menagerie, stepped, [event]);
    expect(advance(menagerie, start, events)).toEqual(stepped);
  });
});

describe.each(COMPANIES)("the body of $name: footings, the hand, the parachute, heads and never colliding", (company) => {
  const menagerie = company.menagerie;
  const everyone = menagerie.species.map((kind) => kind.id);
  const species = company.walker;
  const kind = kindOf(menagerie, species);
  const PLAY: StageEvent = { kind: "permitted", play: true, mischief: false };
  const holding = (changes: Partial<Actor> = {}): Stage => advance(menagerie, alone(company, species, changes), [PLAY]);
  const centre = (stage: Stage): { x: number; y: number } => ({ x: actorOf(stage, species).x, y: actorOf(stage, species).y - kind.size.height / 2 });

  /** ✊️ A stage after a press on the walker and a drag of `dx`, `dy` pixels in `ticks` even steps, a tick after each. */
  function dragged(stage: Stage, dx: number, dy: number, ticks: number): Stage {
    const { x, y } = centre(stage);
    let current = advance(menagerie, stage, [{ kind: "pressed", x, y, pointer: "mouse" }]);
    for (let step = 1; step <= ticks; step++) {
      current = advance(menagerie, current, [{ kind: "dragged", x: x + (dx * step) / ticks, y: y + (dy * step) / ticks }, ticked()]);
      heldApart(menagerie, current);
    }
    return current;
  }

  it("picks a pet up by its scruff when a press moves beyond the slop — not for a click, not while play is not permitted, not on a still stage, and in a time of concentration too", () => {
    const stage = holding();
    const { x, y } = centre(stage);
    const nudged = advance(menagerie, stage, [{ kind: "pressed", x, y, pointer: "mouse" }, { kind: "dragged", x: x + 3, y }]);
    expect(actorOf(nudged, species)).toMatchObject({ footing: "perch", hang: null });
    const lifted = advance(menagerie, nudged, [{ kind: "dragged", x: x + 12, y }]);
    expect(actorOf(lifted, species)).toMatchObject({ footing: "hand", activity: "hang", perch: null, x: 550, y: CARD.y, hang: { grip: { x: 550, y: CARD.y - kind.grip, vx: 0, vy: 0 }, bob: { x: 550, y: CARD.y } } });
    expect([lifted.touched, lifted.origin, lifted.press.phase]).toEqual([species, { x: 550, y: CARD.y }, "lifted"]);
    expect(actorOf(advance(menagerie, stage, [...clicked(x, y)]), species).footing).toBe("perch");
    expect(actorOf(advance(menagerie, alone(company, species), [{ kind: "pressed", x, y, pointer: "mouse" }, { kind: "dragged", x: x + 40, y }]), species).footing).toBe("perch");
    expect(actorOf(advance(menagerie, stage, [{ kind: "tuned", mode: "still" }, { kind: "pressed", x, y, pointer: "mouse" }, { kind: "dragged", x: x + 40, y }]), species).footing).toBe("perch");
    expect(actorOf(advance(menagerie, stage, [{ kind: "hushed", quiet: true }, { kind: "pressed", x, y, pointer: "touch" }, { kind: "dragged", x: x + 40, y }]), species).footing).toBe("hand");
  });

  it("dangles under the pointer: its grip follows by a spring, its feet swing behind, its drawing tilts, and it asks for 64 ticks per second", () => {
    const held = dragged(holding(), 160, -80, 12);
    const actor = actorOf(held, species);
    expect(actor.footing).toBe("hand");
    expect(actor.tilt).toBeGreaterThan(0);
    expect(actor.hang!.grip.x).toBeLessThan(550 + 160);
    expect(frameOf(menagerie, held).rate).toBe(64);
    expect(held.trail.length).toBe(7);
    const rested = run(menagerie, held, 3 * SECOND);
    const still = actorOf(rested, species);
    expect(Math.abs(still.tilt)).toBeLessThan(0.002);
    expect(Math.abs(still.hang!.grip.x - (550 + 160))).toBeLessThan(0.5);
    expect(Math.abs(still.y - (CARD.y - kind.size.height / 2 - 80 + kind.grip))).toBeLessThan(0.5);
  });

  it("is pushed out of whoever it is dragged into, and never through it", () => {
    const [first, second] = company.friends;
    let stage = advance(menagerie, staged(menagerie, [first, second]), [PLAY]);
    stage = seated(menagerie, seated(menagerie, stage, first, 450), second, 650);
    const from = { x: 450, y: CARD.y - kindOf(menagerie, first).size.height / 2 };
    let current = advance(menagerie, stage, [{ kind: "pressed", x: from.x, y: from.y, pointer: "mouse" }]);
    for (let step = 1; step <= 3 * SECOND; step++) {
      current = advance(menagerie, current, [{ kind: "dragged", x: from.x + Math.min(step * 6, 300), y: from.y }, ticked()]);
      heldApart(menagerie, current);
    }
    expect(actorOf(current, first).footing).toBe("hand");
    expect(actorOf(current, first).x).toBeLessThan(650);
  });

  it("is thrown with the pointer's velocity: it tumbles, rights itself and lands, softly under its parachute where it would land hard", () => {
    const held = dragged(holding(), 120, -200, 8);
    const thrown = advance(menagerie, held, [{ kind: "released", x: centre(holding()).x + 120, y: centre(holding()).y - 200 }]);
    expect(actorOf(thrown, species)).toMatchObject({ footing: "air", activity: "tumble", hang: null });
    expect(actorOf(thrown, species).vx).toBeGreaterThan(0);
    expect(thrown.touched).toBeNull();
    let landing: Actor | null = null;
    let touch = 0;
    run(menagerie, thrown, 10 * SECOND, (after, before) => {
      const now = actorOf(after, species);
      if (landing === null && now.footing === "perch") {
        landing = now;
        touch = (now.y - actorOf(before, species).y) * SECOND;
      }
    });
    expect(landing).not.toBeNull();
    expect(landing!.tilt).toBe(0);
    if (kind.gear.includes("parachute")) expect(touch).toBeLessThanOrEqual(600);
  });

  it("glides back to where it was picked up when the pick-up is called off", () => {
    const held = dragged(holding(), 60, -120, 10);
    const cancelled = advance(menagerie, held, [{ kind: "cancelled" }]);
    expect(actorOf(cancelled, species)).toMatchObject({ footing: "air", activity: "glide", hang: null });
    expect(cancelled.courses.map((course) => course.owner)).toEqual([species]);
    expect(cancelled.origin).toBeNull();
    const back = run(menagerie, cancelled, 2 * SECOND);
    expect(actorOf(back, species)).toMatchObject({ footing: "perch", perch: CARD.id, x: 550, y: CARD.y - hoverOf(kind) });
    expect(back.courses).toEqual([]);
    expect(back.claims).toEqual([]);
  });

  it("is tossed to the top of the stage by a deed and let go there; with a parachute it floats down, without one it lands hard", () => {
    for (const geared of [true, false]) {
      const variant: Menagerie = geared ? menagerie : { ...menagerie, species: menagerie.species.map((entry) => (entry.id === species ? { ...entry, gear: [] } : entry)) };
      let stage = advance(variant, alone({ ...company, menagerie: variant }, species), [PLAY, { kind: "played", species, deed: "toss" }]);
      expect(actorOf(stage, species)).toMatchObject({ footing: "hand", activity: "hang" });
      expect(stage.origin).toBeNull();
      let highest = Infinity;
      let floated = false;
      let landed: Actor | null = null;
      stage = run(variant, stage, 12 * SECOND, (after) => {
        const now = actorOf(after, species);
        highest = Math.min(highest, now.y);
        if (now.footing === "chute") floated = true;
        if (landed === null && now.footing === "perch") landed = now;
      });
      expect(highest).toBeLessThan(CARD.y - 150);
      expect(floated).toBe(geared);
      expect(landed).not.toBeNull();
      expect(landed!.activity).toBe(geared ? "idle" : "land");
    }
    const quiet = advance(menagerie, alone(company, species), [{ kind: "played", species, deed: "toss" }]);
    expect(actorOf(quiet, species).footing).toBe("perch");
  });

  it("lets go of the press and gives a held pet back when play is no longer permitted, and lets a still stage take it away", () => {
    const held = dragged(holding(), 60, -100, 10);
    const withdrawn = advance(menagerie, held, [{ kind: "permitted", play: false, mischief: false }]);
    expect(actorOf(withdrawn, species).footing).toBe("air");
    expect([withdrawn.press.phase, withdrawn.touched]).toEqual(["idle", null]);
    const frozenStage = advance(menagerie, held, [{ kind: "tuned", mode: "still" }]);
    expect(actorOf(frozenStage, species)).toMatchObject({ footing: "perch", activity: "idle" });
    expect([frozenStage.press.phase, frozenStage.touched, frozenStage.trail]).toEqual(["idle", null, []]);
  });

  it("never lets two bodies overlap however the learner drags, throws and drops pets into each other while surveys move the cards", () => {
    const rounds = sampled(2, 6, 24);
    let poofs = 0;
    let throws = 0;
    for (let seed = 1; seed <= rounds; seed++) {
      let stage = advance(menagerie, run(menagerie, staged(menagerie, everyone, { seed, mode: "lively", surfaces: [CARD, { id: "shelf", x0: 700, x1: 1000, y: 250 }, FLOOR] }), SECOND), [PLAY]);
      let unit = seed * 7919;
      const next = (bound: number): number => {
        unit = (unit * 48271) % 2147483647;
        return (unit % 1000) / 1000 * bound;
      };
      for (let tick = 0; tick < sampled(600, 2400, 9600); tick++) {
        const events: StageEvent[] = [];
        if (stage.press.phase === "idle" && next(1) < 0.02 && stage.actors.length > 0) {
          const target = stage.actors[Math.floor(next(stage.actors.length))]!;
          events.push({ kind: "pressed", x: target.x, y: target.y - kindOf(menagerie, target.species).size.height / 2, pointer: "mouse" });
        } else if (stage.press.phase !== "idle") {
          const pointer = stage.pointer ?? { x: 640, y: 360 };
          if (next(1) < 0.03) {
            events.push({ kind: "released", x: pointer.x, y: pointer.y });
            throws++;
          } else if (next(1) < 0.005) events.push({ kind: "cancelled" });
          else {
            const other = stage.actors[Math.floor(next(stage.actors.length))];
            const goal = other === undefined ? { x: next(WIDTH), y: next(HEIGHT) } : { x: other.x, y: other.y - 20 };
            events.push({ kind: "dragged", x: pointer.x + Math.max(-14, Math.min(14, goal.x - pointer.x)), y: pointer.y + Math.max(-14, Math.min(14, goal.y - pointer.y)) });
          }
        }
        if (next(1) < 0.002) events.push(surveyed(next(1) < 0.5 ? [CARD, FLOOR] : [{ ...CARD, x0: 300 + next(100), x1: 600 + next(200), y: 400 + next(60) }, { id: "shelf", x0: 700, x1: 1000, y: 250 }, FLOOR]));
        stage = advance(menagerie, stage, [...events, ticked()]);
        heldApart(menagerie, stage);
      }
      poofs = poofs + stage.poofs;
    }
    expect(throws).toBeGreaterThan(0);
    expect(poofs).toBeLessThan(rounds * 20);
  });

  it("comes out the same however time is cut, with a pet in the hand, a throw, a parachute and a toss", () => {
    const start = advance(menagerie, staged(menagerie, everyone, { seed: 21, mode: "lively" }), [PLAY]);
    const { x, y } = { x: start.actors[0]!.x, y: start.actors[0]!.y - kindOf(menagerie, start.actors[0]!.species).size.height / 2 };
    const events: StageEvent[] = [{ kind: "pressed", x, y, pointer: "mouse" }, { kind: "dragged", x: x + 30, y: y - 40 }, ticked(5), { kind: "dragged", x: x + 60, y: y - 200 }, ticked(30), { kind: "released", x: x + 60, y: y - 200 }, ticked(200), { kind: "played", species: start.actors[start.actors.length - 1]!.species, deed: "toss" }, ticked(700)];
    let stepped = start;
    for (const event of events) stepped = event.kind === "ticked" ? run(menagerie, stepped, event.ticks) : advance(menagerie, stepped, [event]);
    expect(advance(menagerie, start, events)).toEqual(stepped);
  });
});

describe.each(COMPANIES)("gear of $name: walls, ladders, ropes and the edges of the stage", (company) => {
  type Card = { readonly id: string; readonly x0: number; readonly x1: number; readonly y0: number; readonly y1: number };
  const menagerie = company.menagerie;
  const walker = company.walker;
  const kind = kindOf(menagerie, walker);
  const TOWER: Card = { id: "tower", x0: 300, x1: 800, y0: 400, y1: 760 };
  const GROUND: Card = { id: "ground", x0: 0, x1: WIDTH, y0: 690, y1: 760 };
  const SHELF: Card = { id: "shelf", x0: 500, x1: 820, y0: 600, y1: 660 };
  const UPPER: Card = { id: "upper", x0: 400, x1: 880, y0: 550, y1: 600 };
  const LOWER: Card = { id: "lower", x0: 400, x1: 880, y0: 630, y1: 760 };

  /** 🃏️ A survey of cards: their tops are surfaces, their boxes keep-outs, their sides walls; with the floor unless said otherwise. */
  function carded(cards: readonly Card[], floor = true): StageEvent {
    const surfaces: Surface[] = cards.map((card) => ({ id: card.id, x0: card.x0, x1: card.x1, y: card.y0 }));
    const walls: Wall[] = cards.flatMap((card) => [
      { id: `${card.id}-left`, surface: card.id, side: -1 as const, x: card.x0, y0: card.y0, y1: card.y1 },
      { id: `${card.id}-right`, surface: card.id, side: 1 as const, x: card.x1, y0: card.y0, y1: card.y1 },
    ]);
    return surveyed(floor ? [...surfaces, FLOOR] : surfaces, cards.map((card) => ({ x: card.x0, y: card.y0, width: card.x1 - card.x0, height: card.y1 - card.y0 })), WIDTH, HEIGHT, walls);
  }

  /** 🎒️ The menagerie with the gear of one species replaced. */
  function equipped(species: Slug, gear: Species["gear"]): Menagerie {
    return { ...menagerie, species: menagerie.species.map((entry) => (entry.id === species ? { ...entry, gear } : entry)) };
  }

  /** 🧗️ A lively stage over `cards` on which one species stands idle at `x` on the perch of `surface` at height `y`, for as long as the test runs. */
  function standing(play: Menagerie, species: Slug, cards: readonly Card[], surface: string, x: number, y: number, floor = true): Stage {
    const stage = advance(play, openStage(3), [{ kind: "tuned", mode: "lively" }, carded(cards, floor), { kind: "summoned", species: [species] }]);
    return craft(stage, species, { perch: surface, x, y: y - hoverOf(kindOf(play, species)), vx: 0, vy: 0, goal: x, activity: "idle", since: stage.tick, until: stage.tick + FAR, blink: stage.tick + FAR, partner: null, opacity: 1, leaving: false, faced: stage.tick });
  }

  /** 🚩️ The stage after an actor set out for `venture` (`setOut` with the units `[place, height, miss]`), or `null` when no way there is clear. */
  function sent(play: Menagerie, stage: Stage, species: Slug, venture: Venture, units: readonly number[] = [0.5, 0.5, 0.9]): Stage | null {
    const draft = draftOf(play, stage);
    return setOut(draft, indexOf(draft.actors, species), [venture], 0, units, draft.tick) ? sealed(draft) : null;
  }

  /** 🧭️ A stage after `ticks` single ticks, every one of them held to the invariant, to the box of the stage (no body of an actor that does not leave outside it) and to what carries a climber (on a wall exactly while it has a pitch of a surveyed wall, on a ladder only while it rides one that stands); `story` lists the footings and activities of `species` as they change, and `watch` sees every stage with the one before it. */
  function journey(play: Menagerie, stage: Stage, ticks: number, species: Slug, watch?: (after: Stage, before: Stage) => void): { stage: Stage; story: string[] } {
    const story: string[] = [];
    const end = run(play, stage, ticks, (after, before) => {
      if (watch !== undefined) watch(after, before);
      const wrong: string[] = [];
      for (const actor of after.actors) {
        const size = kindOf(play, actor.species).size;
        if (!actor.leaving && (actor.x - size.width / 2 < 0 || actor.x + size.width / 2 > after.width || actor.y - size.height < 0 || actor.y > after.height)) wrong.push(`${actor.species} outside at ${actor.x},${actor.y}`);
        if ((actor.footing === "wall") !== (actor.pitch !== null) || (actor.pitch !== null && !after.walls.some((wall) => wall.id === actor.pitch!.wall))) wrong.push(`${actor.species} on ${actor.footing} with ${JSON.stringify(actor.pitch)}`);
        if (actor.footing === "ladder" && !after.ladders.some((ladder) => ladder.rider === actor.species)) wrong.push(`${actor.species} on no ladder`);
        if (actor.grip < 0 || actor.grip > GRIP_BUDGET) wrong.push(`${actor.species} grips ${actor.grip}`);
      }
      if (wrong.length > 0) expect(wrong, `tick ${after.tick}`).toEqual([]);
      const actor = after.actors.find((candidate) => candidate.species === species);
      const now = actor === undefined ? "gone" : `${actor.footing}/${actor.activity}`;
      if (story[story.length - 1] !== now) story.push(now);
    });
    return { stage: end, story };
  }

  /** 🪟️ The perch of a surface on a stage. */
  function perchOf(stage: Stage, surface: string): Venture {
    return { kind: "perch", perch: stage.perches.find((perch) => perch.surface === surface)! };
  }

  it("climbs a wall to a spot and rests there in its climbing pose, its grip running down, then climbs off again — apart, inside the stage and holding on every tick", () => {
    const stage = standing(menagerie, walker, [TOWER], "floor", 200, HEIGHT);
    const pitch = stage.pitches.find((entry) => entry.wall === "tower-left")!;
    const set = sent(menagerie, stage, walker, { kind: "wall", pitch, y: 650 })!;
    expect(set).not.toBeNull();
    const trip = set.trips[0]!;
    expect([trip.owner, trip.ending, trip.steps[trip.steps.length - 1]!.y, trip.steps[trip.steps.length - 1]!.footing]).toEqual([walker, "wall", 650, "wall"]);
    expect(sent(menagerie, stage, walker, { kind: "wall", pitch, y: 500 })).toBeNull();
    expect(set.claims.map((claim) => claim.owner)).toEqual([walker]);
    const { stage: rested, story } = journey(menagerie, set, trip.steps.length, walker);
    expect(story).toEqual(["perch/walk", "wall/climb"]);
    expect(actorOf(rested, walker)).toMatchObject({ footing: "wall", activity: "climb", perch: null, pitch, x: pitch.x - kind.size.width / 2, y: 650, grip: trip.grip });
    expect(actorOf(rested, walker).grip).toBeLessThan(GRIP_BUDGET);
    expect([rested.trips, rested.claims]).toEqual([[], []]);
    const frame = frameOf(menagerie, rested);
    expect(frame.actors[0]!.tilt).toBe(WALL_LEAN);
    expect(frame.rate).toBeLessThan(64);
    let landed: Actor | null = null;
    let lowest = GRIP_BUDGET;
    journey(menagerie, rested, 40 * SECOND, walker, (after) => {
      const actor = actorOf(after, walker);
      if (landed === null) lowest = Math.min(lowest, actor.grip);
      if (landed === null && actor.footing === "perch") landed = actor;
    });
    expect(landed).not.toBeNull();
    expect(landed!.grip).toBe(GRIP_BUDGET);
    expect(lowest).toBeLessThan(actorOf(rested, walker).grip);
  });

  it("is carried along by a wall that a survey moves, and thrown off with a fright by one that is taken away", () => {
    const stage = standing(menagerie, walker, [TOWER], "floor", 200, HEIGHT);
    const pitch = stage.pitches.find((entry) => entry.wall === "tower-left")!;
    const set = sent(menagerie, stage, walker, { kind: "wall", pitch, y: 650 })!;
    const rested = journey(menagerie, set, set.trips[0]!.steps.length, walker).stage;
    const moved = advance(menagerie, rested, [carded([{ ...TOWER, y0: 380, y1: 740 }])]);
    expect(actorOf(moved, walker)).toMatchObject({ footing: "wall", x: TOWER.x0 - kind.size.width / 2, y: 630, pitch: { wall: "tower-left", y0: 380 } });
    const gone = advance(menagerie, moved, [carded([])]);
    expect(actorOf(gone, walker)).toMatchObject({ footing: "air", activity: "fall", pitch: null, grip: GRIP_BUDGET, vx: 0 - SLIP_PUSH });
    expect(actorOf(gone, walker).feeling.mood).toBe("scared");
    expect(actorOf(journey(menagerie, gone, 5 * SECOND, walker).stage, walker)).toMatchObject({ footing: "perch", perch: "floor" });
  });

  it("holds the drawing of a climber that leans towards its wall inside its body: a pet dragged against its head from the side of the wall never reaches into its lean", () => {
    const hopper = company.hopper;
    const tall: Menagerie = { ...menagerie, species: menagerie.species.map((entry) => (entry.id === walker ? { ...entry, size: { width: 40, height: 60 }, grip: 51 } : entry)) };
    const climbing = kindOf(tall, walker);
    const stage = standing(tall, walker, [TOWER], "floor", 200, HEIGHT);
    const pitch = stage.pitches.find((entry) => entry.wall === "tower-left")!;
    const set = sent(tall, stage, walker, { kind: "wall", pitch, y: 650 })!;
    const rested = craft(journey(tall, set, set.trips[0]!.steps.length, walker).stage, walker, { until: FAR });
    let current = advance(tall, rested, [{ kind: "summoned", species: [walker, hopper] }, { kind: "permitted", play: true, mischief: false }]);
    current = craft(run(tall, current, SECOND), walker, { until: FAR });
    const held = actorOf(current, hopper);
    const size = kindOf(tall, hopper).size;
    const climber = actorOf(current, walker);
    current = advance(tall, current, [{ kind: "pressed", x: held.x, y: held.y - size.height / 2, pointer: "mouse" }]);
    const path: { x: number; y: number }[] = [];
    for (let step = 1; step <= 96; step++) path.push({ x: held.x + ((pitch.x + 60 - held.x) * step) / 96, y: held.y - size.height / 2 + ((climber.y - climbing.size.height - 40 - (held.y - size.height / 2)) * step) / 96 });
    for (let lower = 0; lower <= 80; lower += 2) for (let across = 60; across >= -10; across -= 2) path.push({ x: pitch.x + across, y: climber.y - climbing.size.height - 40 + lower });
    const reached: string[] = [];
    for (const point of path) {
      current = advance(tall, current, [{ kind: "dragged", x: point.x, y: point.y }, ticked()]);
      heldApart(tall, current);
      reached.push(...drawnClashes(tall, current));
    }
    expect(actorOf(current, hopper).footing).toBe("hand");
    expect(actorOf(current, walker)).toMatchObject({ footing: "wall", pitch: { wall: "tower-left" } });
    expect(reached.slice(0, 3)).toEqual([]);
  });

  it("raises its own ladder against a wall it cannot climb from below, climbs it and steps off onto the shelf, proud; the ladder stands on a while and is taken away", () => {
    const play = equipped(walker, ["climb", "ladder", "parachute"]);
    const stage = standing(play, walker, [GROUND, SHELF], "ground", 1000, GROUND.y0, false);
    const set = sent(play, stage, walker, perchOf(stage, "shelf"))!;
    expect(set).not.toBeNull();
    const trip = set.trips[0]!;
    expect(trip.ladder).toBe(walker);
    expect(set.ladders.map((ladder) => [ladder.owner, ladder.rider, ladder.since > set.tick])).toEqual([[walker, walker, true]]);
    const { stage: up, story } = journey(play, set, trip.steps.length, walker);
    expect(story).toEqual(["perch/carry", "ladder/climb", "ladder/mantle", "perch/idle"]);
    expect(actorOf(up, walker)).toMatchObject({ footing: "perch", perch: "shelf", y: SHELF.y0 });
    expect(actorOf(up, walker).feeling.mood).toBe("proud");
    const ladder = up.ladders[0]!;
    expect([ladder.rider, ladder.until]).toEqual([null, up.tick + LADDER_IDLE]);
    expect(frameOf(play, up).ladders.length).toBe(1);
    expect(journey(play, craft(up, walker, { until: up.tick + FAR }), LADDER_IDLE + 1, walker).stage.ladders).toEqual([]);
  });

  it("falls with a fright when an edge moves under its ladder and the ladder topples, and never lands hard with a parachute", () => {
    const play = equipped(walker, ["climb", "ladder", "parachute"]);
    const stage = standing(play, walker, [GROUND, SHELF], "ground", 1000, GROUND.y0, false);
    let climbing = sent(play, stage, walker, perchOf(stage, "shelf"))!;
    for (let tick = 0; tick < 30 * SECOND && (actorOf(climbing, walker).footing !== "ladder" || actorOf(climbing, walker).y > GROUND.y0 - 40); tick++) climbing = run(play, climbing, 1);
    expect(actorOf(climbing, walker).footing).toBe("ladder");
    const toppled = advance(play, climbing, [carded([GROUND, { ...SHELF, x0: 660, x1: 980 }], false)]);
    expect(toppled.ladders).toEqual([]);
    expect(actorOf(toppled, walker)).toMatchObject({ activity: "fall", pitch: null });
    expect(actorOf(toppled, walker).feeling.mood).toBe("scared");
    const { stage: down, story } = journey(play, toppled, 5 * SECOND, walker);
    expect(story).not.toContain("perch/dizzy");
    expect(actorOf(down, walker)).toMatchObject({ footing: "perch", perch: "ground" });
  });

  it("fires its grappling hook at an edge too high to hop to: a miss flies past, comes back and is shrugged off, grumpy; a hit reels it up onto the edge", () => {
    const hopper = company.hopper;
    const stage = standing(menagerie, hopper, [GROUND, SHELF], "ground", 1000, GROUND.y0, false);
    const missing = sent(menagerie, stage, hopper, perchOf(stage, "shelf"), [0.5, 0.5, ROPE_MISS_CHANCE / 2])!;
    expect(missing.trips[0]!.steps[missing.trips[0]!.steps.length - 1]!.activity).toBe("shrug");
    expect(actorOf(missing, hopper).rope!.caught).toBe(false);
    let hooked = false;
    const { stage: shrugged, story } = journey(menagerie, missing, missing.trips[0]!.steps.length, hopper, (after) => {
      if (frameOf(menagerie, after).actors[0]!.tools.some((tool) => tool.kind === "hook")) hooked = true;
    });
    expect(hooked).toBe(true);
    expect(story.filter((entry) => entry !== "perch/walk")).toEqual(["perch/aim", "perch/shrug", "perch/idle"]);
    expect(actorOf(shrugged, hopper)).toMatchObject({ footing: "perch", perch: "ground", rope: null });
    expect(actorOf(shrugged, hopper).feeling.mood).toBe("grumpy");
    const hitting = sent(menagerie, shrugged, hopper, perchOf(shrugged, "shelf"), [0.5, 0.5, 0.9])!;
    const { stage: up, story: climb } = journey(menagerie, hitting, hitting.trips[0]!.steps.length, hopper);
    expect(climb.filter((entry) => entry !== "perch/walk")).toEqual(["perch/aim", "rope/reel", "rope/mantle", "perch/idle"]);
    expect(actorOf(up, hopper)).toMatchObject({ footing: "perch", perch: "shelf", rope: null });
  });

  it("hooks the corner of the tab of a card of the home overview at 1440 × 900 from where the line clears the card, and hauls itself up straight beside the card — never swinging into it or below the footer — onto the tab", () => {
    const hopper = company.hopper;
    const play: Menagerie = { ...menagerie, species: menagerie.species.map((entry) => (entry.id === hopper ? { ...entry, size: { width: 48, height: 52 }, gear: ["grapple"] } : entry)) };
    const tab: Card = { id: "tab", x0: 659.8, x1: 780.2, y0: 631.8, y1: 654.2 };
    const card: Card = { id: "card", x0: 659.8, x1: 780.2, y0: 654.2, y1: 849.3 };
    const footer: Surface = { id: "footer", x0: 0, x1: 1440, y: 874 };
    const overview = surveyed([...[tab, card].map((part) => ({ id: part.id, x0: part.x0, x1: part.x1, y: part.y0 })), footer], [tab, card].map((part) => ({ x: part.x0, y: part.y0, width: part.x1 - part.x0, height: part.y1 - part.y0 })), 1440, 900);
    const opened = advance(play, openStage(3), [{ kind: "tuned", mode: "lively" }, overview, { kind: "summoned", species: [hopper] }]);
    const stage = craft(opened, hopper, { perch: footer.id, x: 600, y: footer.y, vx: 0, vy: 0, goal: 600, activity: "idle", since: opened.tick, until: opened.tick + FAR, blink: opened.tick + FAR, partner: null, opacity: 1, leaving: false, faced: opened.tick });
    const set = sent(play, stage, hopper, perchOf(stage, "tab"))!;
    expect(set).not.toBeNull();
    const rope = actorOf(set, hopper).rope!;
    const aim = set.trips[0]!.steps.find((step) => step.activity === "aim")!;
    expect(aim.x).toBeLessThan(600);
    expect([rope.shot.reel, rope.shot.hook.x - tab.x0 < kindOf(play, hopper).size.width / 2, rope.shot.muzzle.x < tab.x0]).toEqual(["zip", true, true]);
    expect(Math.max(...set.trips[0]!.steps.map((step) => step.y))).toBe(footer.y);
    const { stage: up, story } = journey(play, set, set.trips[0]!.steps.length, hopper);
    expect(story).toEqual(["perch/walk", "perch/aim", "rope/reel", "rope/mantle", "perch/idle"]);
    expect(actorOf(up, hopper)).toMatchObject({ footing: "perch", perch: "tab", rope: null });
  });

  it("crosses the gap between two cards of a column with a lunge from the lower wall to the upper one", () => {
    const play = equipped(walker, ["climb", "parachute"]);
    const stage = standing(play, walker, [UPPER, LOWER], "floor", 300, HEIGHT);
    const set = sent(play, stage, walker, perchOf(stage, "upper"))!;
    expect(set).not.toBeNull();
    expect(set.trips[0]!.pitches.map((pitch) => pitch.wall)).toEqual(["upper-left", "lower-left"]);
    const walls: string[] = [];
    const end = run(play, set, set.trips[0]!.steps.length, (after) => {
      const pitch = actorOf(after, walker).pitch;
      if (pitch !== null && walls[walls.length - 1] !== pitch.wall) walls.push(pitch.wall);
    });
    expect(walls).toEqual(["lower-left", "upper-left"]);
    expect(actorOf(end, walker)).toMatchObject({ footing: "perch", perch: "upper" });
  });

  it("gives its trip up when the learner's click turns it to something else on the way to its gear", () => {
    const stage = advance(menagerie, standing(menagerie, walker, [TOWER], "floor", 100, HEIGHT), [{ kind: "permitted", play: true, mischief: false }]);
    const pitch = stage.pitches.find((entry) => entry.wall === "tower-left")!;
    const walking = run(menagerie, sent(menagerie, stage, walker, { kind: "wall", pitch, y: 650 })!, 4);
    expect(actorOf(walking, walker).activity).toBe("walk");
    const actor = actorOf(walking, walker);
    const clickedOn = run(menagerie, advance(menagerie, walking, clicked(actor.x, actor.y - kind.size.height / 2)), 2);
    expect([clickedOn.trips, clickedOn.claims.filter((claim) => claim.owner === walker)]).toEqual([[], []]);
    expect(actorOf(clickedOn, walker).footing).toBe("perch");
  });

  it("bounces off the left, the right and the top edge of the stage with half its speed and a bump, and its body never leaves the stage", () => {
    for (const [x, y, vx, vy] of [
      [WIDTH - 60, 300, 1500, -100],
      [60, 300, -1500, -100],
      [640, 200, 0, -2400],
    ] as const) {
      const thrown = craft(alone(company, walker), walker, { footing: "air", perch: null, activity: "tumble", x, y, vx, vy, goal: x });
      let bounced: { before: Actor; after: Actor } | null = null;
      journey(menagerie, thrown, 4 * SECOND, walker, (after, before) => {
        const now = after.actors.find((actor) => actor.species === walker);
        const then = before.actors.find((actor) => actor.species === walker);
        if (bounced === null && now !== undefined && then !== undefined && now.footing === "air" && (now.vx * then.vx < 0 || (then.vy < 0 && now.vy > 0 && now.y === kind.size.height))) bounced = { before: then, after: now };
      });
      expect(bounced, `${x},${y}`).not.toBeNull();
      const { before, after } = bounced!;
      if (vx !== 0) expect([after.x, after.vx]).toEqual([vx > 0 ? WIDTH - kind.size.width / 2 : kind.size.width / 2, 0 - before.vx * BOUNCE]);
      else expect(after.y).toBe(kind.size.height);
      expect(after.feeling.mood).toBe("grumpy");
    }
  });

  it("is dizzy for a moment after a hard bump against an edge of the stage, and not after a soft one", () => {
    for (const [vx, dizzy] of [[1500, true], [300, false]] as const) {
      const thrown = craft(alone(company, walker), walker, { footing: "air", perch: null, activity: "tumble", x: WIDTH - 60, y: 670, vx, vy: -300, goal: WIDTH - 60 });
      const { story } = journey(menagerie, thrown, 3 * SECOND, walker);
      expect(story, `${vx}`).toContain("perch/land");
      expect(story.includes("perch/dizzy"), `${vx}`).toBe(dizzy);
    }
  });

  it("keeps every body apart, inside the stage and on what carries it on a lively page of cards whose sides are walls, while pets climb, rest on walls, raise ladders and fire ropes", () => {
    const everyone = menagerie.species.map((entry) => entry.id);
    const page: Card[] = [];
    for (const [column, x0, x1] of [["l", 40, 600], ["r", 680, 1240]] as const) for (const [row, y0, y1] of [[0, 60, 250], [1, 280, 470], [2, 500, 900]] as const) page.push({ id: `${column}${row}`, x0, x1, y0, y1 });
    let trips = 0;
    for (let seed = 1; seed <= sampled(2, 3, 6); seed++) {
      const stage = advance(menagerie, openStage(seed), [{ kind: "tuned", mode: "lively" }, carded(page), { kind: "summoned", species: everyone }]);
      journey(menagerie, stage, sampled(90, 180, 600) * SECOND, walker, (after, before) => {
        for (const trip of after.trips) if (!before.trips.some((entry) => entry.owner === trip.owner && entry.from === trip.from)) trips++;
      });
    }
    expect(trips).toBeGreaterThan(0);
  });

  it("keeps every body and every drawing apart while the panorama of a home overview at 1440 × 900 pans under pets that climb, rest on walls, raise ladders and fire ropes", () => {
    const everyone = menagerie.species.map((entry) => entry.id);
    const width = 1440;
    const height = 900;
    const footer: Surface = { id: "footer", x0: 0, x1: width, y: height - 26 };
    const cards: (Card & { readonly tab: number })[] = [];
    for (const [column, x0] of [120, 616, 1183].entries()) for (const [row, y0, y1] of [[0, 60, 180], [1, 290, 590], [2, 632, height - 46]] as const) cards.push({ id: `c${column}${row}`, x0, x1: x0 + 110 + 30 * row, y0, y1, tab: 60 + 10 * column });
    const panorama = (dx: number, dy: number): StageEvent => {
      const moved = cards.map((card) => ({ ...card, x0: card.x0 + dx, x1: card.x1 + dx, y0: card.y0 + dy, y1: card.y1 + dy }));
      const surfaces: Surface[] = moved.flatMap((card) => [{ id: `${card.id}-tab`, x0: card.x0, x1: card.x0 + card.tab, y: card.y0 - 22 }, { id: card.id, x0: card.x0, x1: card.x1, y: card.y0 }]);
      const walls: Wall[] = moved.flatMap((card) => [
        { id: `${card.id}-tab-left`, surface: `${card.id}-tab`, side: -1 as const, x: card.x0, y0: card.y0 - 22, y1: card.y0 },
        { id: `${card.id}-tab-right`, surface: `${card.id}-tab`, side: 1 as const, x: card.x0 + card.tab, y0: card.y0 - 22, y1: card.y0 },
        { id: `${card.id}-left`, surface: card.id, side: -1 as const, x: card.x0, y0: card.y0, y1: card.y1 },
        { id: `${card.id}-right`, surface: card.id, side: 1 as const, x: card.x1, y0: card.y0, y1: card.y1 },
      ]);
      const keepouts: Rect[] = moved.flatMap((card) => [
        { x: card.x0 - 4, y: card.y0 - 22, width: card.tab + 8, height: 22 },
        { x: card.x0 - 4, y: card.y0, width: card.x1 - card.x0 + 8, height: card.y1 - card.y0 },
      ]);
      return surveyed([...surfaces, footer], [...keepouts, { x: -4, y: footer.y, width: width + 8, height: 26 }], width, height, walls);
    };
    const panned = new Set<string>();
    const clashes: string[] = [];
    for (let seed = 1; seed <= sampled(1, 2, 4); seed++) {
      let state = seed * 2654435761;
      const next = (): number => {
        state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
        return state / 4294967296;
      };
      let [dx, dy] = [0, 0];
      const opened = run(menagerie, advance(menagerie, openStage(seed), [{ kind: "tuned", mode: "lively" }, panorama(dx, dy), { kind: "summoned", species: everyone }, { kind: "permitted", play: true, mischief: false }]), SECOND);
      const pitch = opened.pitches.find((entry) => entry.wall === "c02-left")!;
      let stage = craft(opened, walker, { footing: "wall", perch: null, pitch, x: pitch.x - kind.size.width / 2, y: 780, vx: 0, vy: 0, goal: pitch.x - kind.size.width / 2, activity: "climb", since: opened.tick, until: opened.tick + 20 * SECOND, grip: GRIP_BUDGET, opacity: 1, leaving: false, partner: null, rope: null });
      for (let tick = 0; tick < sampled(24, 90, 300) * SECOND; tick++) {
        const sweeping = tick % (6 * SECOND) < 2 * SECOND && tick % 4 === 0;
        if (sweeping) {
          dx = Math.max(-240, Math.min(240, dx + (next() - 0.5) * 9));
          dy = Math.max(-60, Math.min(60, dy + (next() - 0.5) * 5));
          for (const actor of stage.actors) panned.add(actor.footing);
        }
        stage = advance(menagerie, stage, sweeping ? [panorama(dx, dy), ticked()] : [ticked()]);
        heldApart(menagerie, stage);
        if (clashes.length < 3) clashes.push(...drawnClashes(menagerie, stage));
      }
    }
    expect(clashes).toEqual([]);
    expect([...panned].filter((footing) => footing === "wall" || footing === "ladder" || footing === "rope").length, [...panned].join(", ")).toBeGreaterThan(0);
  });

  it("sets out for nothing in a time of concentration, on a still stage or as a floater, and gives every trip up when the stage turns still", () => {
    const stage = standing(menagerie, walker, [TOWER], "floor", 200, HEIGHT);
    const pitch = stage.pitches.find((entry) => entry.wall === "tower-left")!;
    const quiet = advance(menagerie, stage, [{ kind: "hushed", quiet: true }]);
    const hushed = run(menagerie, { ...quiet, actors: quiet.actors.map((actor) => ({ ...actor, until: quiet.tick })) }, 60 * SECOND, (after) => {
      if (after.trips.length > 0) expect(after.trips).toEqual([]);
    });
    expect(hushed.trips).toEqual([]);
    const floater = standing(menagerie, company.floater, [TOWER], "floor", 200, HEIGHT);
    expect(sent(menagerie, floater, company.floater, { kind: "wall", pitch, y: 650 })).toBeNull();
    const set = sent(menagerie, stage, walker, { kind: "wall", pitch, y: 650 })!;
    const still = advance(menagerie, run(menagerie, set, 4), [{ kind: "tuned", mode: "still" }]);
    expect([still.trips, still.claims]).toEqual([[], []]);
    expect(actorOf(still, walker).footing).toBe("perch");
  });
});

describe.each(COMPANIES)("mischief of $name: a pet pushes the copy of a marked element of the page out of its stack", (company) => {
  type Card = { readonly id: string; readonly x0: number; readonly x1: number; readonly y0: number; readonly y1: number };
  const pusher = company.walker;
  const floater = company.floater;
  const KEY = "quiz/task";
  const menagerie = grounded(company.menagerie, { [pusher]: [KEY] });
  const kind = kindOf(menagerie, pusher);
  const TASKS: Card = { id: "tasks", x0: 300, x1: 980, y0: 300, y1: 460 };
  const LEDGE: Surface = { id: "ledge", x0: 120, x1: 290, y: 360 };
  const ROWS: readonly Fixture[] = [
    { id: "row-0", key: "quiz/other", x: 308, y: 306, width: 664, height: 28 },
    { id: "row-1", key: KEY, x: 308, y: 338, width: 664, height: 28 },
  ];
  const ROW = ROWS[1]!;
  const OPEN = MISCHIEF_COOLDOWN_CALM;
  const HOLD = LIFT_BRACE + LIFT_SHOVE + LIFT_WOBBLE;

  /** 🗝️ A menagerie with the grounds of some species replaced. */
  function grounded(play: Menagerie, grounds: Readonly<Record<Slug, readonly string[]>>): Menagerie {
    return { ...play, species: play.species.map((entry) => (grounds[entry.id] === undefined ? entry : { ...entry, grounds: [...grounds[entry.id]!] })) };
  }

  /** 📰️ A survey of the task card — its top a surface, its box a keep-out, its sides walls, its rows the marked elements — with a ledge beside its rows and the floor. */
  function page(fixtures: readonly Fixture[] = ROWS, width = WIDTH): StageEvent {
    const walls: Wall[] = [
      { id: "tasks-left", surface: TASKS.id, side: -1, x: TASKS.x0, y0: TASKS.y0, y1: TASKS.y1 },
      { id: "tasks-right", surface: TASKS.id, side: 1, x: TASKS.x1, y0: TASKS.y0, y1: TASKS.y1 },
    ];
    return surveyed([{ id: TASKS.id, x0: TASKS.x0, x1: TASKS.x1, y: TASKS.y0 }, LEDGE, { ...FLOOR, x1: width }], [{ x: TASKS.x0, y: TASKS.y0, width: TASKS.x1 - TASKS.x0, height: TASKS.y1 - TASKS.y0 }], width, HEIGHT, walls, fixtures);
  }

  /** 🪄️ A stage over the task card, permitted to play with the page, at tick `OPEN` (every mode's first cooldown is over, and the learner has been still since tick 0), on which every one of `who` stands idle at its place on the card's top — or on the ledge, when the place lies on it — for as long as the test runs. */
  function poised(play: Menagerie, who: readonly Slug[], places: readonly number[], options: { readonly mode?: PetMode; readonly fixtures?: readonly Fixture[]; readonly seed?: number } = {}): Stage {
    let stage: Stage = { ...advance(play, openStage(options.seed ?? 3), [{ kind: "tuned", mode: options.mode ?? "lively" }, page(options.fixtures), { kind: "summoned", species: who }, { kind: "permitted", play: true, mischief: true }]), tick: OPEN };
    who.forEach((species, at) => {
      const x = places[at]!;
      const surface = x <= LEDGE.x1 ? LEDGE : { id: TASKS.id, y: TASKS.y0 };
      stage = craft(stage, species, { footing: "perch", perch: surface.id, pitch: null, x, y: surface.y - hoverOf(kindOf(play, species)), vx: 0, vy: 0, goal: x, activity: "idle", since: OPEN, until: OPEN + FAR, blink: OPEN + FAR, faced: OPEN, partner: null, opacity: 1, leaving: false, feeling: atRest(kindOf(play, species).mood, OPEN), gaze: { x: 0, y: 0, vx: 0, vy: 0 } });
    });
    return stage;
  }

  /** 🛤️ The stage after single ticks until `done` holds (at most `limit` of them), every one held to the invariant and every change of activity to the activity graph. */
  function until(play: Menagerie, stage: Stage, done: (stage: Stage) => boolean, limit: number): Stage {
    let current = stage;
    const strays: string[] = [];
    for (let tick = 0; tick < limit && !done(current); tick++) {
      const next = advance(play, current, [ticked()]);
      heldApart(play, next);
      for (const actor of next.actors) {
        const was = current.actors.find((candidate) => candidate.species === actor.species);
        if (was !== undefined && was.activity !== actor.activity && !followersOf(was.activity).includes(actor.activity)) strays.push(`${actor.species} ${was.activity} → ${actor.activity} at ${next.tick}`);
      }
      current = next;
    }
    if (strays.length > 0) expect(strays).toEqual([]);
    return current;
  }

  /** 🧲️ A stage on which the pusher was picked at `OPEN + 1` and has pushed its copy for `age` ticks. */
  function pushing(age: number, play: Menagerie = menagerie): Stage {
    const picked = advance(play, poised(play, [pusher], [900]), [ticked()]);
    return until(play, picked, (current) => current.tick === picked.lift!.since + age, 40 * SECOND);
  }

  it("picks a pet whose grounds cover the key of a marked element, sends it over the rim to its post on the wall beside the element and pushes the copy out of its stack and home again — then it is playful", () => {
    const picked = advance(menagerie, poised(menagerie, [pusher], [900]), [ticked()]);
    const lift = picked.lift!;
    const units = randomWords([picked.seed, MISCHIEF_STREAM, picked.tick, 0], 2).map(unitOf);
    expect(lift).toEqual({ fixture: ROW.id, pusher, since: lift.since, side: -1, room: kind.size.width, span: ROW.width, unit: units[1] });
    expect(picked.trips.map((trip) => [trip.owner, trip.ending, trip.from + trip.steps.length])).toEqual([[pusher, "wall", lift.since]]);
    const posted = until(menagerie, picked, (current) => current.tick === lift.since, 30 * SECOND);
    expect(actorOf(posted, pusher)).toMatchObject({ footing: "wall", activity: "push", x: TASKS.x1 + kind.size.width / 2, y: ROW.y + ROW.height, facing: -1, until: lift.since + LIFT_TICKS + 1 });
    expect(actorOf(posted, pusher).pitch!.wall).toBe("tasks-right");
    let current = posted;
    const wrong: string[] = [];
    while (current.lift !== null) {
      const copy = liftAt(lift.since, current.tick, lift.side, lift.room, lift.span, lift.unit);
      const frame = frameOf(menagerie, current);
      if (JSON.stringify(frame.lifts) !== JSON.stringify([{ fixture: ROW.id, ...copy }]) || actorOf(current, pusher).activity !== "push") wrong.push(`tick ${current.tick}: ${JSON.stringify(frame.lifts)} by ${actorOf(current, pusher).activity}`);
      current = until(menagerie, current, () => false, 1);
    }
    expect(wrong).toEqual([]);
    expect([current.tick, current.rested, frameOf(menagerie, current).lifts]).toEqual([lift.since + LIFT_TICKS, lift.since + LIFT_TICKS, []]);
    expect(actorOf(current, pusher)).toMatchObject({ footing: "wall", activity: "climb" });
    expect(settled(actorOf(current, pusher).feeling, kind.mood, current.tick).mood).toBe("playful");
    expect(actorOf(until(menagerie, current, (later) => actorOf(later, pusher).footing === "perch", 20 * SECOND), pusher).footing).toBe("perch");
  });

  it("keeps the gates: the learner's consent, a stage 1024 px wide or more that is not still, a learner still for 12 s (30 s in a time of concentration; a move of the pointer, an input and a scroll all count), the cooldown of the mode after a prank, one prank at a time", () => {
    const ready = poised(menagerie, [pusher], [900]);
    expect(prankTick(menagerie, ready, ready.tick + 1)).toBe(ready.tick + 1);
    for (const [label, events] of [
      ["no consent", [{ kind: "permitted", play: true, mischief: false }]],
      ["a narrow stage", [page(ROWS, 1000)]],
      ["a still stage", [{ kind: "tuned", mode: "still" }]],
    ] as const) {
      const shut = advance(menagerie, ready, events);
      expect(prankTick(menagerie, shut, shut.tick + 1), label).toBe(-1);
      expect(advance(menagerie, shut, [ticked(sampled(4, 30, 120) * SECOND)]).lift, label).toBeNull();
    }
    for (const event of [{ kind: "pointed", over: "free", x: 100, y: 100 }, { kind: "stirred" }, { kind: "scrolled" }] as const) {
      const stirred = advance(menagerie, ready, [event]);
      expect(prankTick(menagerie, stirred, stirred.tick + 1), event.kind).toBe(stirred.tick + MISCHIEF_PATIENCE);
      const waited = advance(menagerie, stirred, [ticked(MISCHIEF_PATIENCE - 1)]);
      expect(waited.lift, event.kind).toBeNull();
      expect(advance(menagerie, waited, [ticked()]).lift, event.kind).not.toBeNull();
    }
    const quiet = advance(menagerie, ready, [{ kind: "hushed", quiet: true }, { kind: "stirred" }]);
    expect(prankTick(menagerie, quiet, quiet.tick + 1)).toBe(quiet.tick + MISCHIEF_PATIENCE_QUIET);
    for (const [mode, cooldown] of [["lively", MISCHIEF_COOLDOWN_LIVELY], ["calm", MISCHIEF_COOLDOWN_CALM]] as const) {
      const over = until(menagerie, advance(menagerie, poised(menagerie, [pusher], [900], { mode }), [ticked()]), (current) => current.lift === null, 40 * SECOND);
      const back = craft(over, pusher, { footing: "perch", perch: TASKS.id, pitch: null, x: 900, y: TASKS.y0, goal: 900, activity: "idle", until: over.tick + FAR });
      expect(prankTick(menagerie, back, back.tick + 1), mode).toBe(over.rested + cooldown);
    }
    const crowd = grounded(menagerie, { [floater]: [KEY] });
    let both = advance(crowd, poised(crowd, [pusher, floater], [900, 150]), [ticked()]);
    const first = both.lift!;
    let seen = 0;
    both = until(crowd, both, (current) => {
      if (current.lift !== null && current.lift.pusher !== first.pusher) seen++;
      return current.lift === null;
    }, 40 * SECOND);
    expect([seen, both.lift]).toEqual([0, null]);
  });

  it("picks by nothing but the topic and the draw of its own stream: elements no ground covers change nothing, every element that fits is picked now and then, and of an element only its key and its box are read", () => {
    const plain = advance(menagerie, poised(menagerie, [pusher], [900]), [ticked()]);
    const noise: Fixture[] = [
      { id: "noise-0", key: "quiz/tasks", x: 308, y: 380, width: 664, height: 28 },
      { id: "noise-1", key: "quiz", x: 308, y: 412, width: 664, height: 28 },
      { id: "noise-2", key: "elsewhere/task", x: 400, y: 600, width: 200, height: 28 },
    ];
    const noisy = advance(menagerie, poised(menagerie, [pusher], [900], { fixtures: [noise[0]!, ROWS[0]!, noise[1]!, ROW, noise[2]!] }), [ticked()]);
    expect({ ...noisy, fixtures: [] }).toEqual({ ...plain, fixtures: [] });
    const both = grounded(menagerie, { [pusher]: [KEY, "quiz/other"] });
    const picks = new Set<string>();
    for (let seed = 1; seed <= sampled(6, 16, 48); seed++) {
      const stage = poised(both, [pusher], [900], { seed });
      const prospects = prospectsOf(both, { ...stage, tick: stage.tick + 1 }, stage.tick + 1);
      expect(prospects.map((prospect) => prospect.fixture.id)).toEqual(["row-0", "row-1"]);
      const picked = advance(both, stage, [ticked()]);
      const unit = unitOf(randomWords([stage.seed, MISCHIEF_STREAM, picked.tick, 0], 1)[0]!);
      expect(picked.lift!.fixture).toBe(prospects[Math.floor(unit * 2)]!.fixture.id);
      picks.add(picked.lift!.fixture);
    }
    expect([...picks].sort()).toEqual(["row-0", "row-1"]);
    const reads = new Set<string>();
    const watched = ROWS.map((row) => new Proxy(row, { get: (target, field, receiver) => (reads.add(String(field)), Reflect.get(target, field, receiver)) }));
    const start = poised(menagerie, [pusher], [900], { fixtures: watched });
    reads.clear();
    const thrown = advance(menagerie, start, [ticked(), ticked(plain.lift!.since + HOLD - start.tick - 1), page(watched), { kind: "reclaimed", fixture: ROW.id }, ticked(SECOND)]);
    expect(actorOf(thrown, pusher).activity).not.toBe("push");
    expect([...reads].sort()).toEqual(["height", "id", "key", "width", "x", "y"]);
  });

  it("goes to its post with its gear: a climber over the rim to the wall beside the element, anybody along its perch to a perch beside it, a floater or a pet without climbing gear never to a wall — and slips over in a puff where its gear has no way there", () => {
    const ledged = grounded(menagerie, { [floater]: [KEY], [company.hopper]: [KEY] });
    const half = kindOf(ledged, floater).size.width / 2;
    const picked = advance(ledged, poised(ledged, [floater], [150]), [ticked()]);
    const end = picked.perches.find((perch) => perch.surface === LEDGE.id)!.x1;
    expect(picked.lift).toMatchObject({ pusher: floater, fixture: ROW.id, side: 1, room: Math.min(WIDTH - ROW.x - ROW.width, 2 * half) });
    expect(picked.trips.map((trip) => [trip.ending, trip.landing, trip.steps[trip.steps.length - 1]!.x])).toEqual([["perch", LEDGE.id, end - half]]);
    const posted = until(ledged, picked, (current) => current.tick === picked.lift!.since, 30 * SECOND);
    expect(actorOf(posted, floater)).toMatchObject({ footing: "perch", perch: LEDGE.id, activity: "push", x: end - half, facing: 1 });
    expect(frameOf(ledged, until(ledged, posted, (current) => current.tick === picked.lift!.since + HOLD, HOLD)).lifts[0]!.dx).toBeGreaterThan(0);
    for (const species of [floater, company.hopper]) expect(prospectsOf(ledged, { ...poised(ledged, [species], [600]), tick: OPEN + 1 }, OPEN + 1).map((prospect) => [prospect.post.kind, prospect.fixture.id]), species).toEqual([["perch", ROW.id]]);
    const low = craft(poised(menagerie, [pusher], [900]), pusher, { perch: FLOOR.id, x: 1100, goal: 1100, y: HEIGHT });
    const popped = advance(menagerie, low, [ticked()]);
    expect(popped.lift).toMatchObject({ pusher, fixture: ROW.id, since: popped.tick + 1, side: -1 });
    expect(popped.trips).toEqual([]);
    expect(popped.puffs.map((puff) => [puff.x, puff.tick])).toEqual([[1100, popped.tick]]);
    expect(popped.poofs).toBe(low.poofs + 1);
    expect(actorOf(popped, pusher)).toMatchObject({ footing: "wall", activity: "climb", x: TASKS.x1 + kind.size.width / 2, y: ROW.y + ROW.height, opacity: 0, facing: -1, grip: GRIP_BUDGET });
    const shown = until(menagerie, popped, (current) => actorOf(current, pusher).opacity === 1, SECOND);
    expect(actorOf(shown, pusher).activity).toBe("push");
    expect(frameOf(menagerie, until(menagerie, shown, (current) => current.tick === popped.lift!.since + HOLD, HOLD)).lifts[0]!.dx).toBeLessThan(0);
  });

  it("goes to its post beside a task row of a quiz's page as the site lays it out at 1440 × 900 with its gear, in two legs planned whole: from the footer far below it raises its ladder against the lowest card of the column, climbs from its exit up the wall line to the row and pushes — no puff", () => {
    const width = 1440;
    const parts: Card[] = [
      { id: "intro-tab", x0: 272, x1: 344.2, y0: 35.2, y1: 57.6 },
      { id: "intro", x0: 272, x1: 1168, y0: 57.6, y1: 298.5 },
      { id: "tasks-tab", x0: 272, x1: 402.8, y0: 328.9, y1: 351.3 },
      { id: "tasks", x0: 272, x1: 1168, y0: 351.3, y1: 409.6 },
      { id: "crowd-tab", x0: 272, x1: 448.1, y0: 416, y1: 438.3 },
      { id: "crowd", x0: 272, x1: 1168, y0: 438.3, y1: 469.7 },
    ];
    const footer: Surface = { id: "footer", x0: 0, x1: width, y: 874 };
    const rows: Fixture[] = [
      { id: "row-0", key: "quiz/other", x: 278.4, y: 351.3, width: 883.2, height: 27.6 },
      { id: "row-1", key: KEY, x: 278.4, y: 382, width: 883.2, height: 27.6 },
    ];
    const walls: Wall[] = parts.flatMap((part) => [
      { id: `${part.id}-left`, surface: part.id, side: -1 as const, x: part.x0, y0: part.y0, y1: part.y1 },
      { id: `${part.id}-right`, surface: part.id, side: 1 as const, x: part.x1, y0: part.y0, y1: part.y1 },
    ]);
    const quiz = surveyed([...parts.map((part) => ({ id: part.id, x0: part.x0, x1: part.x1, y: part.y0 })), footer], [...parts.map((part) => ({ x: part.x0, y: part.y0, width: part.x1 - part.x0, height: part.y1 - part.y0 })), { x: 0, y: footer.y, width, height: 26 }], width, 900, walls, rows);
    const play: Menagerie = { ...menagerie, species: menagerie.species.map((entry) => (entry.id === pusher ? { ...entry, size: { width: 54, height: 55 }, gear: ["climb", "ladder"] } : entry)) };
    const opened: Stage = { ...advance(play, openStage(3), [{ kind: "tuned", mode: "lively" }, quiz, { kind: "summoned", species: [pusher] }, { kind: "permitted", play: true, mischief: true }]), tick: OPEN };
    const low = craft(opened, pusher, { footing: "perch", perch: footer.id, pitch: null, x: 700, y: footer.y, vx: 0, vy: 0, goal: 700, activity: "idle", since: OPEN, until: OPEN + FAR, blink: OPEN + FAR, faced: OPEN, partner: null, opacity: 1, leaving: false, feeling: atRest(kindOf(play, pusher).mood, OPEN), gaze: { x: 0, y: 0, vx: 0, vy: 0 } });
    const picked = advance(play, low, [ticked()]);
    expect(picked.lift).toMatchObject({ pusher, fixture: "row-1" });
    expect([picked.puffs, picked.poofs]).toEqual([[], low.poofs]);
    expect(picked.trips.map((trip) => [trip.owner, trip.ladder, trip.ending])).toEqual([[pusher, pusher, "wall"]]);
    const story: string[] = [];
    const posted = until(play, picked, (current) => {
      const now = actorOf(current, pusher).footing;
      if (story[story.length - 1] !== now) story.push(now);
      return current.tick === picked.lift!.since;
    }, 60 * SECOND);
    expect(story).toEqual(["perch", "ladder", "wall"]);
    expect([posted.poofs, posted.puffs]).toEqual([low.poofs, []]);
    expect(actorOf(posted, pusher)).toMatchObject({ footing: "wall", activity: "push", y: 409.6 });
    expect(actorOf(posted, pusher).pitch!.wall).toMatch(/^tasks-(left|right)$/u);
  });

  it("throws its pusher off when the learner takes the element back: the copy is gone at once, the pusher tumbles away from the element and up, frightened, never lands hard with its parachute, and is sheepish once its fright has passed", () => {
    const at = pushing(HOLD + 200);
    for (const other of [{ kind: "reclaimed", fixture: "row-0" }, { kind: "reclaimed", fixture: "nowhere" }] as const) expect(advance(menagerie, at, [other])).toEqual(at);
    const thrown = advance(menagerie, at, [{ kind: "reclaimed", fixture: ROW.id }]);
    expect(frameOf(menagerie, thrown).lifts).toEqual([]);
    expect(thrown.rested).toBe(at.tick);
    const unit = unitOf(randomWords([at.seed, MISCHIEF_STREAM, at.tick, 1], 1)[0]!);
    expect(actorOf(thrown, pusher)).toMatchObject({ footing: "air", activity: "tumble", pitch: null, vx: THROW_SPEED + THROW_SPREAD * unit, vy: 0 - THROW_LIFT });
    expect(settled(actorOf(thrown, pusher).feeling, kind.mood, thrown.tick).mood).toBe("scared");
    expect(prankTick(menagerie, thrown, thrown.tick + 1)).toBe(-1);
    let touch = 0;
    let landed = -1;
    const over = until(menagerie, thrown, (current) => {
      for (const course of current.courses) if (course.owner === pusher) touch = Math.max(touch, course.touch);
      if (landed < 0 && actorOf(current, pusher).footing === "perch") landed = current.tick;
      return current.lift === null;
    }, 30 * SECOND);
    expect(touch).toBeLessThanOrEqual(HARD_LANDING);
    expect(landed).toBeGreaterThan(thrown.tick);
    expect(over.tick).toBeGreaterThanOrEqual(landed);
    expect(settled(actorOf(over, pusher).feeling, kind.mood, over.tick).mood).toBe("sad");
    const errand = advance(menagerie, poised(menagerie, [pusher], [900]), [ticked()]);
    expect(advance(menagerie, errand, [{ kind: "reclaimed", fixture: ROW.id }])).toEqual(errand);
  });

  it("puts the copy back by itself: home within 20 s, sliding home when its pusher is called away while the copy rests, gone at once when that happens on the copy's way out", () => {
    expect(LIFT_TICKS).toBeLessThanOrEqual(LIFT_LIMIT);
    const resting = pushing(HOLD + 100);
    const lift = resting.lift!;
    const called = until(menagerie, advance(menagerie, resting, [{ kind: "summoned", species: [] }]), () => false, 1);
    expect(called.lift).toEqual({ ...lift, since: called.tick - LIFT_RETURNS });
    expect(frameOf(menagerie, called).lifts[0]!.dx).toBe(liftAt(lift.since, resting.tick, lift.side, lift.room, lift.span, lift.unit).dx);
    const home = until(menagerie, called, (current) => current.lift === null, LIFT_TICKS);
    expect([home.tick, home.rested, frameOf(menagerie, home).lifts]).toEqual([called.tick - LIFT_RETURNS + LIFT_TICKS, called.tick - LIFT_RETURNS + LIFT_TICKS, []]);
    const shoving = pushing(LIFT_BRACE + 10);
    const vanished = until(menagerie, advance(menagerie, shoving, [{ kind: "summoned", species: [] }]), () => false, 1);
    expect([vanished.lift, vanished.rested, frameOf(menagerie, vanished).lifts]).toEqual([null, vanished.tick, []]);
  });

  it("ends the prank quietly when its element is gone or moved, when mischief is no longer permitted or the stage turns still, and calls the pet off when its way there is cut short", () => {
    const at = pushing(HOLD + 50);
    const quietly = (stage: Stage, label: string): void => {
      expect([stage.lift, stage.rested, frameOf(menagerie, stage).lifts], label).toEqual([null, at.tick, []]);
      expect(actorOf(stage, pusher), label).toMatchObject({ footing: "wall", activity: "climb" });
    };
    quietly(advance(menagerie, at, [page([ROWS[0]!])]), "gone");
    quietly(advance(menagerie, at, [page([ROWS[0]!, { ...ROW, x: ROW.x + 2 }])]), "moved");
    quietly(advance(menagerie, at, [{ kind: "permitted", play: true, mischief: false }]), "withdrawn");
    expect(advance(menagerie, at, [page([ROWS[0]!, { ...ROW, y: ROW.y + 0.25 }])]).lift).toEqual(at.lift);
    const still = advance(menagerie, at, [{ kind: "tuned", mode: "still" }]);
    expect([still.lift, actorOf(still, pusher).footing, frameOf(menagerie, still).lifts]).toEqual([null, "perch", []]);
    const picked = advance(menagerie, poised(menagerie, [pusher], [700]), [ticked()]);
    const walking = until(menagerie, picked, (current) => actorOf(current, pusher).activity === "walk" && actorOf(current, pusher).x > 720, 10 * SECOND);
    const actor = actorOf(walking, pusher);
    const called = until(menagerie, advance(menagerie, walking, clicked(actor.x, actor.y - kind.size.height / 2)), (current) => current.lift === null, 4);
    expect([called.lift, called.trips]).toEqual([null, []]);
    expect(called.rested).toBeGreaterThan(walking.tick);
  });

  it("ends in the same stage however the ticks are cut, through a prank, a reclaim and the landing, and wakes a resting stage at the tick the gates open", () => {
    const start = poised(menagerie, [pusher], [900]);
    const since = advance(menagerie, start, [ticked()]).lift!.since;
    const reclaim: StageEvent = { kind: "reclaimed", fixture: ROW.id };
    const one = advance(menagerie, until(menagerie, start, (current) => current.tick === since + HOLD + 80, 40 * SECOND), [reclaim]);
    const many = advance(menagerie, advance(menagerie, start, [ticked(3), ticked(17), ticked(since + HOLD + 80 - start.tick - 20)]), [reclaim]);
    expect(many).toEqual(one);
    expect(advance(menagerie, many, [ticked(7), ticked(30 * SECOND), ticked(1)])).toEqual(run(menagerie, one, 30 * SECOND + 8));
    const plain: Menagerie = { ...menagerie, species: menagerie.species.map((entry) => ({ ...entry, repertoire: {}, clips: [], emitters: [] })) };
    const resting = run(plain, advance(plain, craft(poised(plain, [pusher], [900]), pusher, { emitters: [] }), [{ kind: "stirred" }]), SECOND);
    expect(frameOf(plain, resting)).toMatchObject({ rate: 0, wake: resting.tick - SECOND + MISCHIEF_PATIENCE });
    const woken = advance(plain, resting, [ticked(MISCHIEF_PATIENCE - SECOND)]);
    expect(woken.lift).not.toBeNull();
    expect(woken).toEqual(run(plain, resting, MISCHIEF_PATIENCE - SECOND));
  });
});

describe("the arithmetic of the module", () => {
  it("uses nothing the Rust twin cannot reproduce bit for bit, and never the console, in any part of the stage", () => {
    for (const part of PARTS) {
      const source = readFileSync(resolve(HERE, "../../..", part, "🟦️.ts"), "utf8");
      expect(source, part).not.toMatch(/Math\.(sin|cos|tan|atan2|exp|pow|hypot|log|random)\b/);
      expect(source, part).not.toMatch(/\b(Date|performance|console)\b/);
    }
  });
});
