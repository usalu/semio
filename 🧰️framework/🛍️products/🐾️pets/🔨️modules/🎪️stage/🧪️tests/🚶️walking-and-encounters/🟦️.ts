/** 🚶️ Unit suite of the stage, movement: walking along perches and riding a surface, falling and landing, and encounters between friends and rivals.
 *
 * @see ../🧰️stage-helpers/🟦️.ts — the shared kit; ../../🟦️.ts — the façade under test
 */
import { describe, expect, it } from "vitest";
import { type Activity, type Actor, type Menagerie, type Rect, type Slug, type Stage, type Surface } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { ENCOUNTERS, MODE_LIMITS } from "../../../🧠️behavior/🟦️.ts";
import { settled, spiritsOf } from "../../../💗️feeling/🟦️.ts";
import { advance, frameOf } from "../../🟦️.ts";
import { SECOND, WIDTH, HEIGHT, FLOOR, CARD, COMFORT, COMPANIES, ticked, surveyed, kindOf, hoverOf, hopOf, actorOf, staged, run, craft, seated, alone, covers } from "../🧰️stage-helpers/🟦️.ts";

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
