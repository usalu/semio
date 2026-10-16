/** 🧠️ Unit suite of the stage, determinism and mind: determinism, the rate and the wake, and the mind of a pet (clicks, gestures, moods).
 *
 * @see ../🧰️stage-helpers/🟦️.ts — the shared kit; ../../🟦️.ts — the façade under test
 */
import { describe, expect, it } from "vitest";
import { ACTIVITIES, type Actor, type Frame, type Menagerie, type Slug, type Stage, type StageEvent } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { BLINK_TICKS } from "../../../🎞️animation/🟦️.ts";
import { atRest, settled } from "../../../💗️feeling/🟦️.ts";
import { shaken } from "../../../👀️attention/🟦️.ts";
import { draftOf, indexOf, sealed } from "../../../📝️draft/🟦️.ts";
import { STATE_BLEND } from "../../../🎥️projection/🟦️.ts";
import { advance, frameOf } from "../../🟦️.ts";
import { SECOND, FLOOR, CARD, FAR, COMPANIES, ticked, surveyed, clicked, kindOf, actorOf, staged, run, craft, seated, alone } from "../🧰️stage-helpers/🟦️.ts";

/** 🧊️ An object and everything in it, frozen. */
function frozen<T>(value: T): T {
  if (typeof value === "object" && value !== null && !Object.isFrozen(value)) {
    Object.freeze(value);
    for (const inner of Object.values(value)) frozen(inner);
  }
  return value;
}

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
