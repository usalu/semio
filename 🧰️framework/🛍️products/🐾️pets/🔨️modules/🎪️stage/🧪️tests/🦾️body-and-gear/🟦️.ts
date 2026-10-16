/** 🦾️ Unit suite of the stage, body and gear: footings, the hand and the pointer, walls, ladders, ropes and what a pet holds.
 *
 * @see ../🧰️stage-helpers/🟦️.ts — the shared kit; ../../🟦️.ts — the façade under test
 */
import { describe, expect, it } from "vitest";
import { type Actor, type Menagerie, type Rect, type Slug, type Species, type Stage, type StageEvent, type Surface, type Wall } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { draftOf, indexOf, sealed } from "../../../📝️draft/🟦️.ts";
import { WALL_LEAN } from "../../../📏️spacing/🟦️.ts";
import { BOUNCE } from "../../../🚶️locomotion/📐️constants/🟦️.ts";
import { setOut } from "../../../🚶️locomotion/🧳️trips/🟦️.ts";
import { type Venture } from "../../../🚶️locomotion/🪜️outings/🟦️.ts";
import { GRIP_BUDGET, LADDER_IDLE, ROPE_MISS_CHANCE, SLIP_PUSH } from "../../../🧗️climbing/🟦️.ts";
import { advance, frameOf, openStage } from "../../🟦️.ts";
import { SECOND, WIDTH, HEIGHT, FLOOR, CARD, FAR, COMPANIES, ticked, surveyed, clicked, kindOf, hoverOf, actorOf, staged, heldApart, run, craft, seated, alone } from "../🧰️stage-helpers/🟦️.ts";

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
