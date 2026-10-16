/** 📏️ Unit suite of the stage, spacing: keeping their distance, hopping between perches and turning round.
 *
 * @see ../🧰️stage-helpers/🟦️.ts — the shared kit; ../../🟦️.ts — the façade under test
 */
import { describe, expect, it } from "vitest";
import { type Actor, type Menagerie, type Rect, type Slug, type Stage, type Surface } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { hail } from "../../../👀️attention/🟦️.ts";
import { draftOf, indexOf, sealed } from "../../../📝️draft/🟦️.ts";
import { advance, frameOf } from "../../🟦️.ts";
import { SECOND, WIDTH, HEIGHT, FLOOR, CARD, FAR, COMFORT, COMPANIES, ticked, surveyed, kindOf, hoverOf, hopOf, actorOf, staged, collisions, run, craft, seated, alone, covers } from "../🧰️stage-helpers/🟦️.ts";

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
