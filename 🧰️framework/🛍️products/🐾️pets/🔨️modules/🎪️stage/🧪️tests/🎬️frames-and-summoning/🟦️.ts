/** 🎬️ Unit suite of the stage, frames: a time of concentration, a still stage, summoning and leaving, the learner's hand, what an actor arrives with and what a frame shows.
 *
 * @see ../🧰️stage-helpers/🟦️.ts — the shared kit; ../../🟦️.ts — the façade under test
 */
import { describe, expect, it } from "vitest";
import { type Fixture, type Slug, type Stage, type StageEvent, type Wall } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { restPose, solveRig } from "../../../🦴️rig/🟦️.ts";
import { MODE_LIMITS } from "../../../🧠️behavior/🟦️.ts";
import { atRest, settled, spiritsOf } from "../../../💗️feeling/🟦️.ts";
import { COLD, noHover } from "../../../👆️gesture/🟦️.ts";
import { extentOf } from "../../../📏️spacing/🟦️.ts";
import { advance, frameOf } from "../../🟦️.ts";
import { SECOND, WIDTH, HEIGHT, FLOOR, CARD, FAR, COMPANIES, ticked, surveyed, clicked, kindOf, hoverOf, actorOf, staged, run, craft, seated, alone } from "../🧰️stage-helpers/🟦️.ts";

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
