/** 🪄️ Unit suite of the stage, mischief: a pet pushes the copy of a thing, and the arithmetic of the module.
 *
 * @see ../🧰️stage-helpers/🟦️.ts — the shared kit; ../../🟦️.ts — the façade under test
 */
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { type Fixture, type Menagerie, type PetMode, type Slug, type Stage, type StageEvent, type Surface, type Wall } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { followersOf } from "../../../🧠️behavior/🟦️.ts";
import { atRest, settled } from "../../../💗️feeling/🟦️.ts";
import { GRIP_BUDGET } from "../../../🧗️climbing/🟦️.ts";
import { LIFT_BRACE, LIFT_LIMIT, LIFT_RETURNS, LIFT_SHOVE, LIFT_TICKS, LIFT_WOBBLE, MISCHIEF_COOLDOWN_CALM, MISCHIEF_COOLDOWN_LIVELY, MISCHIEF_PATIENCE, MISCHIEF_PATIENCE_QUIET, THROW_LIFT, THROW_SPEED, THROW_SPREAD, liftAt } from "../../../🪄️mischief/🟦️.ts";
import { MISCHIEF_STREAM, randomWords, unitOf } from "../../../🎲️randomness/🟦️.ts";
import { prankTick, prospectsOf } from "../../../🗓️schedule/🟦️.ts";
import { HARD_LANDING } from "../../../🪢️swing/🟦️.ts";
import { advance, frameOf, openStage } from "../../🟦️.ts";
import { HERE, SECOND, WIDTH, HEIGHT, FLOOR, FAR, COMPANIES, ticked, surveyed, clicked, kindOf, hoverOf, actorOf, heldApart, run, craft } from "../🧰️stage-helpers/🟦️.ts";

/** 📜️ The source texts of a part of the stage: its own module file, or the module file of each child folder when the part is split into children. */
function sourcesOf(part: string): string[] {
  const directory = resolve(HERE, "../../..", part);
  const own = resolve(directory, "🟦️.ts");
  if (existsSync(own)) return [readFileSync(own, "utf8")];
  return readdirSync(directory, { withFileTypes: true })
    .filter((entry) => entry.isDirectory() && existsSync(resolve(directory, entry.name, "🟦️.ts")))
    .map((entry) => readFileSync(resolve(directory, entry.name, "🟦️.ts"), "utf8"));
}

const PARTS = ["🎪️stage", "📝️draft", "📏️spacing", "🗓️schedule", "👀️attention", "🚶️locomotion", "💞️sociability", "🎯️choice", "👥️population", "🕰️clock", "🎥️projection", "🧗️climbing", "🪄️mischief"];

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

  /** 🎮️ A stage over the task card, permitted to play with the page, at tick `OPEN` (every mode's first cooldown is over, and the learner has been still since tick 0), on which every one of `who` stands idle at its place on the card's top — or on the ledge, when the place lies on it — for as long as the test runs. */
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
      for (const source of sourcesOf(part)) {
        expect(source, part).not.toMatch(/Math\.(sin|cos|tan|atan2|exp|pow|hypot|log|random)\b/);
        expect(source, part).not.toMatch(/\b(Date|performance|console)\b/);
      }
    }
  });
});
