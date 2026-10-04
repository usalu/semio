/** 🎥️ Unit suite of the projection: every field the second round adds to a frame, checked on frames of stages written directly — the face a feeling gives (mouth, lids, posture), the look of a state (its clip over the idle loop channel by channel, blended in from the former look, kept under replacing clips, the tint changing in the middle of the blend), the tilt and pivot and where the rig is placed so that the feet stay where the stage has them, the tools, the body, the particles of every plume (anchored at their bone, capped, gone on a still stage), standing ladders, the lifted copy, who is held, and the rate and wake all of these ask for.
 *
 * Two menageries are played: the sample menagerie of the schema-conformance vectors and the troupe of the stage-trace
 * vectors. gl-matrix (`mat2d`) is the independent judge of every placement: it builds the transform a render target
 * applies (translate, mirror, turn about the pivot) and the matrices of the bones, and the frame must agree with it.
 *
 * @see ../../🟦️.ts — the projection under test
 * @see ../../../🎪️stage/🟦️.ts — the façade whose stages it projects
 * @see https://glmatrix.net/docs/module-mat2d.html — the oracle of the placements
 */
import { mat2d, vec2 } from "gl-matrix";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { TICKS_PER_SECOND, type Actor, type ActorFrame, type Clip, type Emitter, type Fixture, type Frame, type Menagerie, type Pitch, type Point, type Slug, type Species, type Stage, type StageEvent, type Surface, type Wall } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { lidAt } from "../../../🎞️animation/🟦️.ts";
import { STAGE_CAP, capped, emitterEnds, emitterKey, particlesOf, type Particle } from "../../../✨️effects/🟦️.ts";
import { atRest, faceOf, impulse, settled, spiritsOf } from "../../../💗️feeling/🟦️.ts";
import { HOOK_RETURN, HOOK_SPEED, MUZZLE_FORWARD, MUZZLE_HEIGHT, RUNG_SPACING, hookStep, hookTicks } from "../../../🧗️climbing/🟦️.ts";
import { LIFT_RETURNS, LIFT_TICKS, liftAt } from "../../../🪄️mischief/🟦️.ts";
import { CHUTE_ROD, canopyOf, chuteOf, hangOf } from "../../../🪢️swing/🟦️.ts";
import { PUFF_TICKS } from "../../../🚶️locomotion/🟦️.ts";
import { WALL_LEAN, extentAt, extentOf } from "../../../📏️spacing/🟦️.ts";
import { restPose, solveRig, type Pose } from "../../../🦴️rig/🟦️.ts";
import { advance, openStage } from "../../../🎪️stage/🟦️.ts";
import { AIM_RISE, BRISK, CARRY_LEAN, CARRY_LENGTH, LADDER_FADE, STATE_BLEND, frameOf } from "../../🟦️.ts";

const HERE = dirname(fileURLToPath(import.meta.url));
const SECOND = TICKS_PER_SECOND;
const WIDTH = 1280;
const HEIGHT = 720;
const FLOOR: Surface = { id: "floor", x0: 0, x1: WIDTH, y: HEIGHT };
const CARD: Surface = { id: "card", x0: 300, x1: 800, y: 400 };
const FAR = 100000000;
const SPOT = 550;

/** 🧫️ One committed fixture of the product. */
function fixture<T>(name: string): T {
  return JSON.parse(readFileSync(resolve(HERE, "../../../../🧫️fixtures", name, "🔣️.json"), "utf8")) as T;
}

type Company = { readonly name: string; readonly menagerie: Menagerie; readonly walker: Slug; readonly other: Slug };

const SAMPLE: Company = { name: "the sample menagerie", menagerie: fixture<{ menagerie: Menagerie }>("🧬️schema-conformance").menagerie, walker: "blobby", other: "hoppy" };
const TROUPE: Company = { name: "the trace troupe", menagerie: fixture<{ menagerie: Menagerie }>("🎪️stage-trace").menagerie, walker: "mossy", other: "sparky" };
const COMPANIES = [SAMPLE, TROUPE];

/** 🗺️ A survey of the stage. */
function surveyed(surfaces: readonly Surface[], walls: readonly Wall[] = [], fixtures: readonly Fixture[] = []): StageEvent {
  return { kind: "surveyed", width: WIDTH, height: HEIGHT, surfaces, keepouts: [], walls, fixtures };
}

/** 🧬️ A species of a menagerie. */
function kindOf(menagerie: Menagerie, id: Slug): Species {
  return menagerie.species.find((species) => species.id === id)!;
}

/** 🎈️ The hover of a species. */
function hoverOf(kind: Species): number {
  return kind.locomotion.gait === "float" ? (kind.locomotion.hover ?? 0) : 0;
}

/** 🧸️ The actor of a species on a stage. */
function actorOf(stage: Stage, species: Slug): Actor {
  return stage.actors.find((candidate) => candidate.species === species)!;
}

/** 🖼️ The frame of the actor of a species. */
function drawnOf(frame: Frame, species: Slug): ActorFrame {
  return frame.actors.find((candidate) => candidate.species === species)!;
}

/** ✍️ A stage in which the actor of a species is written anew. */
function craft(stage: Stage, species: Slug, changes: Partial<Actor>): Stage {
  return { ...stage, actors: stage.actors.map((actor) => (actor.species === species ? { ...actor, ...changes } : actor)) };
}

/** 🪑️ A stage on which the species stand whole, idle and at rest on the card, 120 px apart from `SPOT` on, for as long as the test runs, without a blink in sight. */
function seated(menagerie: Menagerie, species: readonly Slug[], mode: "calm" | "lively" | "still" = "calm", walls: readonly Wall[] = []): Stage {
  let stage = advance(menagerie, openStage(5), [{ kind: "tuned", mode: mode === "still" ? "calm" : mode }, surveyed([CARD, FLOOR], walls), { kind: "summoned", species }]);
  species.forEach((id, place) => {
    const kind = kindOf(menagerie, id);
    const x = SPOT + 120 * place - 120;
    stage = craft(stage, id, { perch: CARD.id, footing: "perch", host: null, x, y: CARD.y - hoverOf(kind), vx: 0, vy: 0, tilt: 0, goal: x, activity: "idle", clip: null, since: stage.tick, until: stage.tick + FAR, blink: stage.tick + FAR, partner: null, opacity: 1, leaving: false, feeling: atRest(kind.mood, stage.tick), gaze: { x: 0, y: 0, vx: 0, vy: 0 }, emitters: [], hang: null, chute: null, rope: null });
  });
  return mode === "still" ? advance(menagerie, stage, [{ kind: "tuned", mode: "still" }]) : stage;
}

/** 😴️ The stage four seconds later, its pupils come to rest where its actors look. */
function rested(menagerie: Menagerie, stage: Stage): Stage {
  return advance(menagerie, stage, [{ kind: "ticked", ticks: 4 * SECOND }]);
}

/** 🧱️ The menagerie with every species bare of clips: no idle loop, no activity clip — a stage of them rests completely (rate 0) unless something else moves. */
function bare(menagerie: Menagerie): Menagerie {
  return { ...menagerie, species: menagerie.species.map((entry) => ({ ...entry, clips: [], repertoire: {} })) };
}

/** 🛤️ A looping clip of one second that holds the bone `body` on constant values per channel. */
function holding(id: string, values: Readonly<Partial<Record<"x" | "y" | "rotation" | "scaleX" | "scaleY", number>>>): Clip {
  return { id, seconds: 1, loop: true, tracks: Object.entries(values).map(([channel, value]) => ({ bone: "body", channel: channel as "x", keys: [{ at: 0, value: value! }, { at: 1, value: value! }] })) };
}

/** 🎨️ The menagerie with the walker made a test piece: content at rest, an idle loop that holds the body 2 px up and 10° round, a dozing clip that moves nothing visible, and three states — at rest without a look, `glow-a` whose look turns the body 30° and `glow-b` whose look turns it 60° and shifts it 3 px. */
function lookingMenagerie(company: Company): Menagerie {
  const clips = [holding("still-breath", { y: -2, rotation: 10 }), holding("look-a", { rotation: 30 }), holding("look-b", { rotation: 60, x: 3 }), { id: "nap", seconds: 1, loop: true, tracks: [{ bone: "root", channel: "x" as const, keys: [{ at: 0, value: 0 }, { at: 1, value: 0 }] }] }];
  const name = { en: "State", de: "Zustand" };
  return {
    ...company.menagerie,
    species: company.menagerie.species.map((entry) =>
      entry.id !== company.walker
        ? entry
        : {
            ...entry,
            mood: "content",
            clips,
            repertoire: { idle: ["still-breath"], sleep: ["nap"] },
            states: [
              { id: "resting", name },
              { id: "glow-a", name, clip: "look-a" },
              { id: "glow-b", name, clip: "look-b" },
            ],
          },
    ),
  };
}

/** 🦴️ The pose of the test piece with the body at `y`, turned `rotation` and shifted `x`. */
function bodyPose(kind: Species, x: number, y: number, rotation: number): Pose {
  return restPose(kind).map((bone, index) => (kind.bones[index]!.id === "body" ? { ...bone, x, y, rotation } : bone));
}

/** 🔢️ A gl-matrix matrix in doubles (gl-matrix builds single floats by default, which round to 1e-5 px on a 1280 px stage). */
function precise(values: readonly number[] = [1, 0, 0, 1, 0, 0]): mat2d {
  return new Float64Array(values) as unknown as mat2d;
}

/** 📌️ A gl-matrix vector in doubles. */
function spot(): vec2 {
  return new Float64Array(2) as unknown as vec2;
}

/** 🧮️ The transform a render target applies to the drawing of an actor frame (gl-matrix): placed at its `x`, `y`, mirrored with its facing, turned by its tilt about its pivot. */
function placement(drawn: ActorFrame): mat2d {
  const matrix = precise();
  mat2d.translate(matrix, matrix, [drawn.x, drawn.y]);
  mat2d.scale(matrix, matrix, [drawn.facing, 1]);
  mat2d.translate(matrix, matrix, [drawn.pivot.x, drawn.pivot.y]);
  mat2d.rotate(matrix, matrix, drawn.facing * drawn.tilt * 2 * Math.PI);
  mat2d.translate(matrix, matrix, [-drawn.pivot.x, -drawn.pivot.y]);
  return matrix;
}

/** 📍️ Where a point of the rig of an actor frame lands on the stage (gl-matrix). */
function landed(drawn: ActorFrame, x: number, y: number): Point {
  const point = vec2.transformMat2d(spot(), [x, y], placement(drawn));
  return { x: point[0]!, y: point[1]! };
}

/** 💫️ A plume of an emitter that began at `since` and runs while its cause lasts. */
function plume(emitter: Slug, since: number, until: number | null = null): Actor["emitters"][number] {
  return { emitter, since, until };
}

/** ✨️ The menagerie in which the walker carries the given emitters (the troupe has none of its own). */
function emitting(company: Company, emitters: readonly Emitter[]): Menagerie {
  return { ...company.menagerie, species: company.menagerie.species.map((entry) => (entry.id === company.walker ? { ...entry, emitters: [...entry.emitters.filter((own) => !emitters.some((added) => added.id === own.id)), ...emitters] } : entry)) };
}

/** 🌧️ A test emitter on the walker's body: `motion` at `speed` px/s, 12 alive, 1 s each. */
function emitter(id: Slug, motion: Emitter["motion"], speed: number, count = 12): Emitter {
  return { id, bone: "body", x: 3, y: -12, shape: { kind: "ellipse", cx: 0, cy: 0, rx: 1.5, ry: 1.5 }, fill: "accent", stroke: "none", motion, count, life: 1, speed, spread: 0.25 };
}

describe.each(COMPANIES)("the face of a feeling in $name", (company) => {
  const menagerie = company.menagerie;
  const species = company.walker;
  const kind = kindOf(menagerie, species);

  it("shows the mood felt at the tick with its intensity, bends the mouth with its spirits and lowers the lids to the mood's resting height, the blink on top", () => {
    const stage = seated(menagerie, [species]);
    const moods = ["grumpy", "sad", "happy", "sleepy", "scared"] as const;
    for (const mood of moods) {
      const feeling = impulse(atRest(kind.mood, stage.tick), mood, 0.9, stage.tick);
      const ages = sampled([0, 200, 4000], [0, 64, 128, 200, 1000, 4000, 9000], [0, 32, 64, 96, 128, 160, 200, 500, 1000, 2000, 4000, 9000, 20000]);
      for (const age of ages) {
        const later = { ...craft(stage, species, { feeling, blink: stage.tick + age - 5 }), tick: stage.tick + age };
        const drawn = drawnOf(frameOf(menagerie, later), species);
        const felt = settled(feeling, kind.mood, later.tick);
        const face = faceOf(felt);
        expect([drawn.mood, drawn.intensity, drawn.spirits]).toEqual([felt.mood, felt.intensity, spiritsOf(felt)]);
        expect(drawn.spirits).toBe(face.bend);
        for (const eye of drawn.eyes) expect(eye.lid).toBe(face.lid + (1 - face.lid) * lidAt(5));
      }
    }
  });

  it("sinks or lifts the bone that carries the first eye by the posture of the mood, shuts the lids asleep and shows the resting face, open-eyed and upright, on a still stage", () => {
    const plain = bare(menagerie);
    const stage = { ...seated(plain, [species]), quiet: true };
    const sad = impulse(atRest(kind.mood, stage.tick), "sad", 1, stage.tick);
    const drop = faceOf(sad).drop;
    expect(drop).toBe(2);
    const eye = kind.bones.findIndex((bone) => bone.id === kind.face.eyes[0]!.bone);
    const pose = restPose(kind).map((bone, index) => (index === eye ? { ...bone, y: bone.y + drop } : bone));
    expect(drawnOf(frameOf(plain, craft(stage, species, { feeling: sad })), species).bones).toEqual(solveRig(kind, pose));
    const asleep = drawnOf(frameOf(plain, craft(stage, species, { feeling: sad, activity: "sleep" })), species);
    for (const look of asleep.eyes) expect(look.lid).toBe(1);
    const still = drawnOf(frameOf(plain, advance(plain, craft(stage, species, { feeling: sad }), [{ kind: "tuned", mode: "still" }])), species);
    const resting = atRest(kind.mood, 0);
    expect([still.mood, still.intensity, still.spirits]).toEqual([kind.mood, resting.intensity, spiritsOf(resting)]);
    for (const look of still.eyes) expect(look.lid).toBe(0);
    expect(still.bones).toEqual(solveRig(kind, restPose(kind)));
  });
});

describe.each(COMPANIES)("the look of a state in $name", (company) => {
  const menagerie = lookingMenagerie(company);
  const species = company.walker;
  const kind = kindOf(menagerie, species);
  const at = (stage: Stage, changes: Partial<Actor>): ActorFrame => drawnOf(frameOf(menagerie, { ...craft(stage, species, changes), quiet: true }), species);

  it("takes every channel its clip keys from that clip and leaves every other channel to the idle loop", () => {
    const stage = seated(menagerie, [species]);
    expect(at(stage, {}).bones).toEqual(solveRig(kind, bodyPose(kind, 0, -2, 10)));
    const since = stage.tick - STATE_BLEND;
    const drawn = at(stage, { state: "glow-a", stateSince: since, former: "resting" });
    expect(drawn.bones).toEqual(solveRig(kind, bodyPose(kind, 0, -2, 30)));
    expect(drawn.state).toBe("glow-a");
  });

  it("blends in from the former look channel by channel and changes the state it names in the middle of the blend", () => {
    const stage = seated(menagerie, [species]);
    const half = at(stage, { state: "glow-a", stateSince: stage.tick - STATE_BLEND / 2, former: "glow-b" });
    expect(half.bones).toEqual(solveRig(kind, bodyPose(kind, 1.5, -2, 45)));
    expect(half.state).toBe("glow-a");
    expect(at(stage, { state: "glow-a", stateSince: stage.tick - STATE_BLEND / 2 + 1, former: "glow-b" }).state).toBe("glow-b");
    const quarter = at(stage, { state: "resting", stateSince: stage.tick - STATE_BLEND / 4, former: "glow-a" });
    const share = 0.25 * 0.25 * (3 - 2 * 0.25);
    expect(quarter.bones).toEqual(solveRig(kind, bodyPose(kind, 0, -2, 30 + (10 - 30) * share)));
    let previous: readonly number[] | null = null;
    for (let tick = 0; tick <= STATE_BLEND + 2; tick++) {
      const bones = at({ ...stage, tick: stage.tick + tick }, { state: "glow-b", stateSince: stage.tick, former: "glow-a" }).bones;
      if (previous !== null) for (let entry = 0; entry < bones.length; entry++) expect(Math.abs(bones[entry]! - previous[entry]!)).toBeLessThan(1);
      previous = bones;
    }
  });

  it("blends from the state that gave way when a lasting state has run out, whatever the actor was last told", () => {
    const lasting: Menagerie = { ...menagerie, species: menagerie.species.map((entry) => (entry.id !== species ? entry : { ...entry, states: entry.states.map((state) => (state.id === "glow-b" ? { ...state, lasts: 1, then: "glow-a" } : state)) })) };
    const stage = seated(lasting, [species]);
    const drawn = drawnOf(frameOf(lasting, { ...craft(stage, species, { state: "glow-b", stateSince: stage.tick - SECOND - STATE_BLEND / 2, former: "resting" }), quiet: true }), species);
    expect(drawn.bones).toEqual(solveRig(kind, bodyPose(kind, 1.5, -2, 45)));
  });

  it("keeps its look under a clip that lays the idle loop to rest, and shows it at rest on a still stage", () => {
    const stage = seated(menagerie, [species]);
    const asleep = at(stage, { state: "glow-a", stateSince: stage.tick - 100, former: "resting", activity: "sleep", clip: "nap", since: stage.tick - 100, until: stage.tick + 100 });
    expect(asleep.bones).toEqual(solveRig(kind, bodyPose(kind, 0, 0, 30)));
    const still = advance(menagerie, craft(stage, species, { state: "glow-a", stateSince: stage.tick, former: "glow-b" }), [{ kind: "tuned", mode: "still" }]);
    expect(drawnOf(frameOf(menagerie, still), species).bones).toEqual(solveRig(kind, bodyPose(kind, 0, 0, 30)));
  });

  it("lands without a jump: a landing begins with the idle loop at rest, where the flight left it", () => {
    const plainLanding: Menagerie = { ...menagerie, species: menagerie.species.map((entry) => (entry.id !== species ? entry : { ...entry, repertoire: { ...entry.repertoire, land: ["nap"], hop: ["nap"] } })) };
    const stage = seated(plainLanding, [species]);
    const flying = drawnOf(frameOf(plainLanding, { ...craft(stage, species, { activity: "hop", clip: "nap", since: stage.tick - 30, until: stage.tick + 1 }), quiet: true }), species);
    const landing = drawnOf(frameOf(plainLanding, { ...craft(stage, species, { activity: "land", clip: "nap", since: stage.tick, until: stage.tick + 20 }), quiet: true }), species);
    expect(landing.bones).toEqual(flying.bones);
    const ending = drawnOf(frameOf(plainLanding, { ...craft(stage, species, { activity: "land", clip: "nap", since: stage.tick - 20, until: stage.tick }), quiet: true }), species);
    expect(ending.bones).toEqual(solveRig(kind, bodyPose(kind, 0, -2, 10)));
  });
});

describe.each(COMPANIES)("tilt, pivot and placement in $name", (company) => {
  const menagerie = company.menagerie;
  const species = company.walker;
  const kind = kindOf(menagerie, species);

  it("turns the drawing by the actor's tilt about its scruff and places the rig so that its feet stay where the stage has them (gl-matrix)", () => {
    const stage = seated(menagerie, [species]);
    const count = sampled(24, 400, 4000);
    for (let index = 0; index < count; index++) {
      const tilt = ((index * 0.6180339887498949) % 1) - 0.5;
      const facing = index % 2 === 0 ? 1 : -1;
      const feet = { x: 400 + ((index * 37) % 300), y: 300 + ((index * 53) % 200) };
      const drawn = drawnOf(frameOf(menagerie, craft(stage, species, { tilt, facing, x: feet.x, y: feet.y })), species);
      expect([drawn.tilt, drawn.pivot]).toEqual([tilt, { x: 0, y: -kind.grip }]);
      const foot = landed(drawn, 0, 0);
      expect(Math.abs(foot.x - feet.x) + Math.abs(foot.y - feet.y)).toBeLessThan(1e-9);
      const scruff = landed(drawn, 0, -kind.grip);
      expect(Math.abs(scruff.x - (feet.x + kind.grip * Math.sin(2 * Math.PI * tilt))) + Math.abs(scruff.y - (feet.y - kind.grip * Math.cos(2 * Math.PI * tilt)))).toBeLessThan(1e-9);
    }
    const upright = drawnOf(frameOf(menagerie, stage), species);
    expect([upright.x, upright.y, upright.tilt]).toEqual([actorOf(stage, species).x, actorOf(stage, species).y, 0]);
  });

  it("leans a climber towards the wall of its pitch, its body with it, and gives the lean up as its feet leave the line it clings to — over the rim onto the top", () => {
    const wall: Wall = { id: "card-left", surface: CARD.id, side: -1, x: CARD.x0, y0: CARD.y, y1: CARD.y + 200 };
    const stage = seated(menagerie, [species], "calm", [wall]);
    const pitch: Pitch = { wall: wall.id, surface: wall.surface, side: wall.side, x: wall.x, y0: wall.y0, y1: wall.y1 };
    const cling = { footing: "wall" as const, perch: null, pitch, x: wall.x + (wall.side * kind.size.width) / 2, y: CARD.y + 120, tilt: 0 };
    const leaning = drawnOf(frameOf(menagerie, craft(stage, species, { ...cling, activity: "climb" })), species);
    expect(leaning.tilt).toBe(WALL_LEAN);
    const leant = extentAt(species, kind, { x: cling.x, y: cling.y }, WALL_LEAN, null);
    expect(leaning.body).toEqual({ x: leant.x0, y: leant.y0, width: leant.x1 - leant.x0, height: leant.y1 - leant.y0 });
    const sine = Math.sin(2 * Math.PI * WALL_LEAN);
    const cosine = Math.cos(2 * Math.PI * WALL_LEAN);
    for (const [across, up] of [[-kind.size.width / 2, -kind.size.height], [kind.size.width / 2, -kind.size.height], [kind.size.width / 2, 0], [-kind.size.width / 2, 0]] as const) {
      const corner = { x: cling.x + across * cosine - up * sine, y: cling.y + across * sine + up * cosine };
      expect(corner.x >= leaning.body.x && corner.x <= leaning.body.x + leaning.body.width && corner.y >= leaning.body.y && corner.y <= leaning.body.y + leaning.body.height).toBe(true);
    }
    expect(drawnOf(frameOf(menagerie, craft(stage, species, { ...cling, pitch: null, activity: "climb", facing: -1 })), species).tilt).toBe(0);
    const right: Wall = { ...wall, id: "card-right", side: 1, x: CARD.x1 };
    const other = seated(menagerie, [species], "calm", [right]);
    expect(drawnOf(frameOf(menagerie, craft(other, species, { ...cling, pitch: { ...pitch, wall: right.id, side: 1, x: right.x }, x: right.x + kind.size.width / 2, activity: "climb" })), species).tilt).toBe(-WALL_LEAN);
    const mantle = drawnOf(frameOf(menagerie, craft(stage, species, { ...cling, x: cling.x + kind.size.width / 4, activity: "mantle" })), species);
    expect(mantle.tilt).toBe(WALL_LEAN * 0.5);
    expect(drawnOf(frameOf(menagerie, craft(stage, species, { ...cling, x: cling.x + kind.size.width / 2 + 8, activity: "mantle" })), species).tilt).toBe(0);
    expect(drawnOf(frameOf(menagerie, advance(menagerie, craft(stage, species, { ...cling, activity: "climb" }), [{ kind: "tuned", mode: "still" }])), species).tilt).toBe(0);
  });

  it("turns the clip of a climber with the distance it climbs: a climber that rests holds its pose whole and still, and asks for no more than a loop does", () => {
    const wall: Wall = { id: "card-left", surface: CARD.id, side: -1, x: CARD.x0, y0: CARD.y, y1: CARD.y + 200 };
    const pitch: Pitch = { wall: wall.id, surface: wall.surface, side: wall.side, x: wall.x, y0: wall.y0, y1: wall.y1 };
    const stage = seated(menagerie, [species], "calm", [wall]);
    const clip = kind.repertoire.climb?.[0] ?? null;
    expect(clip).not.toBeNull();
    const resting = craft(stage, species, { footing: "wall", perch: null, pitch, x: wall.x - kind.size.width / 2, y: CARD.y + 120, activity: "climb", clip, since: stage.tick - 3, until: stage.tick + FAR, faced: stage.tick - 100 });
    const now = drawnOf(frameOf(menagerie, resting), species).bones;
    const later = drawnOf(frameOf(menagerie, { ...resting, tick: resting.tick + 7, actors: resting.actors.map((actor) => ({ ...actor, blink: resting.tick + FAR })) }), species).bones;
    expect(later).toEqual(now);
    const climbed = drawnOf(frameOf(menagerie, craft(resting, species, { y: CARD.y + 120 - 0.3 * kind.size.height })), species).bones;
    expect(climbed).not.toEqual(now);
    expect(frameOf(menagerie, resting).rate).toBeLessThan(64);
    expect(frameOf(menagerie, { ...resting, trips: [{ owner: species, from: resting.tick, steps: [{ x: resting.actors[0]!.x, y: CARD.y + 120, footing: "wall", perch: null, activity: "climb", facing: 1, hold: 0 }], pitches: [pitch], ladder: null, ending: "wall", landing: "", grip: 100 }] }).rate).toBe(64);
  });

  it("hangs a pet on a rope from its hands", () => {
    const stage = seated(menagerie, [species]);
    const drawn = drawnOf(frameOf(menagerie, craft(stage, species, { footing: "rope", perch: null })), species);
    expect(drawn.pivot).toEqual({ x: MUZZLE_FORWARD * kind.size.width, y: 0 - MUZZLE_HEIGHT * kind.size.height });
  });
});

describe.each(COMPANIES)("tools, body and the hand in $name", (company) => {
  const menagerie = company.menagerie;
  const species = company.walker;
  const kind = kindOf(menagerie, species);

  it("opens the parachute as far as the stage has it, canopy and cords leaning as the cords lean", () => {
    const stage = seated(menagerie, [species]);
    const feet = { x: 500, y: 200 };
    const cords = CHUTE_ROD * kind.size.height;
    for (const lean of [-0.6, -0.2, 0, 0.3, 0.7]) {
      const canopy = { ...canopyOf(feet, 0, 120, chuteOf(kind.size.height)), x: feet.x + cords * lean, y: feet.y - cords * Math.sqrt(1 - lean * lean) };
      const drawn = drawnOf(frameOf(menagerie, craft(stage, species, { footing: "chute", perch: null, x: feet.x, y: feet.y, chute: { since: stage.tick - 10, open: 0.75, opening: 2, canopy } })), species);
      expect(drawn.tools.length).toBe(1);
      const tool = drawn.tools[0]!;
      expect(tool.kind).toBe("chute");
      if (tool.kind !== "chute") continue;
      expect(tool.open).toBe(0.75);
      expect(Math.abs(tool.sway - Math.asin(lean) / (2 * Math.PI))).toBeLessThan(2e-6);
    }
  });

  it("pays the rope out to the flying hook with the gun aimed at the edge, then holds the taut rope to the hook", () => {
    const stage = seated(menagerie, [species]);
    const muzzle = { x: SPOT + 12, y: CARD.y - 20 };
    const hook = { x: SPOT + 140, y: CARD.y - 230 };
    const shot = { surface: "card", facing: 1 as const, muzzle, hook, length: Math.sqrt(128 * 128 + 210 * 210), reel: "swing" as const };
    const flight = hookTicks(muzzle, hook, HOOK_SPEED);
    expect(flight).toBeGreaterThan(4);
    const rope = { shot, since: stage.tick - 3, length: shot.length, hand: muzzle, before: muzzle, caught: true };
    const flying = drawnOf(frameOf(menagerie, craft(stage, species, { activity: "aim", rope })), species).tools;
    const tip = hookStep(muzzle, hook, HOOK_SPEED, 3);
    expect(flying.slice(0, 2)).toEqual([
      { kind: "rope", x: tip.x, y: tip.y, slack: 0 },
      { kind: "hook", x: tip.x, y: tip.y },
    ]);
    const gun = flying[2]!;
    expect(gun.kind).toBe("gun");
    if (gun.kind === "gun") expect(Math.abs(gun.aim - Math.atan2(hook.y - muzzle.y, hook.x - muzzle.x) / (2 * Math.PI))).toBeLessThan(2e-6);
    const bitten = drawnOf(frameOf(menagerie, craft(stage, species, { footing: "rope", perch: null, activity: "reel", rope: { ...rope, since: stage.tick - flight } })), species);
    expect(bitten.tools).toEqual([
      { kind: "rope", x: hook.x, y: hook.y, slack: 0 },
      { kind: "hook", x: hook.x, y: hook.y },
    ]);
    const back = hookStep(hook, muzzle, HOOK_RETURN, 2);
    const missed = drawnOf(frameOf(menagerie, craft(stage, species, { activity: "aim", rope: { ...rope, since: stage.tick - flight - 2, caught: false } })), species).tools;
    expect(missed.slice(0, 2)).toEqual([
      { kind: "rope", x: back.x, y: back.y, slack: 0 },
      { kind: "hook", x: back.x, y: back.y },
    ]);
    expect(missed[2]!.kind).toBe("gun");
  });

  it("carries its ladder leaning the way it faces and raises its gun when it aims without a shot; empty-handed otherwise", () => {
    const stage = seated(menagerie, [species]);
    for (const facing of [1, -1] as const) {
      expect(drawnOf(frameOf(menagerie, craft(stage, species, { activity: "carry", facing })), species).tools).toEqual([{ kind: "ladder", lean: facing * CARRY_LEAN, length: CARRY_LENGTH * kind.size.height }]);
      expect(drawnOf(frameOf(menagerie, craft(stage, species, { activity: "aim", facing })), species).tools).toEqual([{ kind: "gun", aim: facing > 0 ? -AIM_RISE : AIM_RISE - 0.5 }]);
    }
    expect(drawnOf(frameOf(menagerie, stage), species).tools).toEqual([]);
  });

  it("gives as its body the box the stage keeps clear, and names the actor in the learner's hand", () => {
    const stage = seated(menagerie, [species, company.other]);
    const grip = { x: SPOT - 80, y: CARD.y - 200 };
    const held = craft(stage, species, { footing: "hand", perch: null, activity: "hang", x: grip.x + 9, y: grip.y + kind.grip - 1, tilt: -0.05, hang: hangOf({ x: grip.x, y: grip.y + kind.grip }, kind.grip) });
    const frame = frameOf(menagerie, held);
    expect(frame.held).toBe(species);
    for (const drawn of frame.actors) {
      const extent = extentOf(actorOf(held, drawn.species), kindOf(menagerie, drawn.species));
      expect(drawn.body).toEqual({ x: extent.x0, y: extent.y0, width: extent.x1 - extent.x0, height: extent.y1 - extent.y0 });
    }
    expect(frameOf(menagerie, stage).held).toBeNull();
    expect(frameOf(menagerie, advance(menagerie, held, [{ kind: "tuned", mode: "still" }])).held).toBeNull();
  });
});

describe.each(COMPANIES)("particles in $name", (company) => {
  const species = company.walker;
  const brisk = emitter("sparks", "burst", 80);
  const slow = emitter("motes", "rise", BRISK / 2);
  const menagerie = emitting(company, [brisk, slow]);
  const kind = kindOf(menagerie, species);

  it("lets every plume emit at its bone as drawn — mirrored, turned and placed like the actor (gl-matrix) — and keys it by seed, species, emitter and begin", () => {
    const stage = seated(menagerie, [species]);
    const count = sampled(6, 40, 200);
    for (let index = 0; index < count; index++) {
      const facing = index % 2 === 0 ? 1 : -1;
      const tilt = index % 3 === 0 ? 0 : ((index * 0.37) % 0.4) - 0.2;
      const since = stage.tick - (index % 7) * 5;
      const posed = craft(stage, species, { facing, tilt, emitters: [plume("sparks", since)] });
      const frame = frameOf(menagerie, posed);
      const drawn = drawnOf(frame, species);
      const bone = kind.bones.findIndex((entry) => entry.id === brisk.bone) * 6;
      const local = vec2.transformMat2d(spot(), [brisk.x, brisk.y], precise(drawn.bones.slice(bone, bone + 6)));
      const origin = landed(drawn, local[0]!, local[1]!);
      const stream = menagerie.species.findIndex((entry) => entry.id === species);
      const place = kind.emitters.findIndex((entry) => entry.id === brisk.id);
      const expected = particlesOf(brisk, origin, facing, since, null, stage.tick, emitterKey(stage.seed, stream, place, since));
      expect(frame.particles.length).toBe(expected.length);
      frame.particles.forEach((particle, at) => {
        const twin = expected[at]!;
        expect([particle.species, particle.emitter]).toEqual([species, brisk.id]);
        expect(Math.abs(particle.x - twin.x) + Math.abs(particle.y - twin.y)).toBeLessThan(1e-9);
        expect([particle.scale, particle.rotation, particle.opacity]).toEqual([twin.scale, twin.rotation, twin.opacity]);
      });
    }
  });

  it("fades the particles with their actor, keeps the youngest 160 of a crowded stage and shows none on a still stage", () => {
    const stage = seated(menagerie, [species]);
    const faded = frameOf(menagerie, craft(stage, species, { opacity: 0.5, emitters: [plume("sparks", stage.tick - 4)] }));
    const whole = frameOf(menagerie, craft(stage, species, { emitters: [plume("sparks", stage.tick - 4)] }));
    expect(faded.particles.map((particle) => particle.opacity)).toEqual(whole.particles.map((particle) => particle.opacity * 0.5));
    const crowded = emitting(company, [emitter("rain", "fall", 120, 32)]);
    const plumes = Array.from({ length: 8 }, (_, index) => plume("rain", stage.tick - 80 - 13 * index));
    const busy = craft(seated(crowded, [species]), species, { emitters: plumes });
    const frame = frameOf(crowded, busy);
    const crowdKind = kindOf(crowded, species);
    const all: Particle[] = [];
    const drawn = drawnOf(frame, species);
    const bone = crowdKind.bones.findIndex((entry) => entry.id === "body") * 6;
    const local = vec2.transformMat2d(spot(), [3, -12], precise(drawn.bones.slice(bone, bone + 6)));
    const origin = landed(drawn, local[0]!, local[1]!);
    const stream = crowded.species.findIndex((entry) => entry.id === species);
    const place = crowdKind.emitters.findIndex((entry) => entry.id === "rain");
    for (const one of plumes) all.push(...particlesOf(crowdKind.emitters[place]!, origin, drawn.facing, one.since, null, busy.tick, emitterKey(busy.seed, stream, place, one.since)));
    expect(all.length).toBeGreaterThan(STAGE_CAP);
    expect(frame.particles.length).toBe(STAGE_CAP);
    const youngest = [...all].sort((one, two) => one.age - two.age)[STAGE_CAP - 1]!.age;
    const kept = capped(all, STAGE_CAP);
    expect(Math.max(...kept.map((particle) => particle.age))).toBe(youngest);
    expect(frame.particles.map((particle) => particle.x)).toEqual(kept.map((particle) => particle.x));
    const still = advance(crowded, busy, [{ kind: "tuned", mode: "still" }]);
    expect(frameOf(crowded, still).particles).toEqual([]);
  });

  it("stops showing a plume once it has ended (`emitterEnds`) and skips one of an emitter the species does not have", () => {
    const stage = seated(menagerie, [species]);
    const ends = emitterEnds(brisk, stage.tick - 70, stage.tick - 69)!;
    expect(ends).toBeLessThanOrEqual(stage.tick);
    expect(frameOf(menagerie, craft(stage, species, { emitters: [plume("sparks", stage.tick - 70, stage.tick - 69), plume("missing", stage.tick - 2)] })).particles).toEqual([]);
  });
});

describe.each(COMPANIES)("ladders and the lifted copy in $name", (company) => {
  const menagerie = company.menagerie;
  const species = company.walker;

  it("draws a standing ladder from foot to top with one rung per 10.5 px, fading in after it stood up and out before it is taken away", () => {
    const stage = seated(menagerie, [species]);
    const ladder = { owner: species, wall: "card-left", surface: "floor", side: -1 as const, foot: { x: 250, y: 720 }, top: { x: 300, y: 407 }, since: stage.tick - LADDER_FADE / 2, until: stage.tick + 1000, rider: null };
    const length = Math.sqrt(50 * 50 + 313 * 313);
    const fading = frameOf(menagerie, { ...stage, ladders: [ladder] }).ladders;
    expect(fading).toEqual([{ x0: 250, y0: 720, x1: 300, y1: 407, rungs: Math.floor(length / RUNG_SPACING), opacity: 0.5 }]);
    expect(frameOf(menagerie, { ...stage, ladders: [{ ...ladder, since: stage.tick - 100 }] }).ladders[0]!.opacity).toBe(1);
    expect(frameOf(menagerie, { ...stage, ladders: [{ ...ladder, until: stage.tick + LADDER_FADE / 2 }] }).ladders[0]!.opacity).toBe(0.5);
    expect(frameOf(menagerie, { ...stage, ladders: [{ ...ladder, since: stage.tick }] }).ladders).toEqual([]);
    expect(frameOf(menagerie, { ...stage, ladders: [{ ...ladder, until: stage.tick }] }).ladders).toEqual([]);
    const still = advance(menagerie, { ...stage, ladders: [ladder] }, [{ kind: "tuned", mode: "still" }]);
    expect(frameOf(menagerie, still).ladders[0]!.opacity).toBe(1);
  });

  it("shows the dust where a pet vanished, spreading from 0 towards 1 over `PUFF_TICKS`, at rate 64, and none on a still stage", () => {
    const stage = seated(bare(menagerie), [species]);
    const puff = { x: 300, y: 405, width: 40, height: 30, tick: stage.tick - PUFF_TICKS / 4 };
    const frame = frameOf(bare(menagerie), { ...stage, puffs: [puff] });
    expect([frame.puffs, frame.rate]).toEqual([[{ x: 300, y: 405, width: 40, height: 30, phase: 0.25 }], 64]);
    expect(frameOf(bare(menagerie), { ...stage, puffs: [{ ...puff, tick: stage.tick - PUFF_TICKS }] }).puffs).toEqual([]);
    expect(frameOf(bare(menagerie), { ...advance(bare(menagerie), stage, [{ kind: "tuned", mode: "still" }]), puffs: [puff] }).puffs).toEqual([]);
  });

  it("shows the lifted copy as `liftAt` has it for as long as the lift lasts, and never on a still stage", () => {
    const stage = seated(menagerie, [species]);
    const count = sampled(16, 96, 688);
    for (let index = 0; index < count; index++) {
      const age = Math.floor((index * LIFT_TICKS) / count);
      const lift = { fixture: "task-3", pusher: species, since: stage.tick - age, side: (index % 2 === 0 ? 1 : -1) as 1 | -1, room: 40, span: 300, unit: 0.25 };
      const copy = liftAt(lift.since, stage.tick, lift.side, lift.room, lift.span, lift.unit);
      expect(frameOf(menagerie, { ...stage, lift }).lifts).toEqual([{ fixture: "task-3", dx: copy.dx, dy: copy.dy, tilt: copy.tilt, opacity: copy.opacity }]);
    }
    expect(frameOf(menagerie, { ...stage, lift: { fixture: "task-3", pusher: species, since: stage.tick - LIFT_TICKS, side: 1, room: 40, span: 300, unit: 0.25 } }).lifts).toEqual([]);
    expect(frameOf(menagerie, { ...stage, lift: { fixture: "task-3", pusher: species, since: stage.tick + 1, side: 1, room: 40, span: 300, unit: 0.25 } }).lifts).toEqual([]);
    const still = advance(menagerie, stage, [{ kind: "tuned", mode: "still" }]);
    expect(frameOf(menagerie, { ...still, lift: { fixture: "task-3", pusher: species, since: still.tick - 50, side: 1, room: 40, span: 300, unit: 0.25 } }).lifts).toEqual([]);
  });
});

describe.each(COMPANIES)("rate and wake in $name", (company) => {
  const species = company.walker;
  const menagerie = bare(emitting(company, [emitter("sparks", "burst", 80), emitter("motes", "rise", BRISK / 2), emitter("haze", "drift", BRISK)]));

  it("rests completely while nothing moves, and keeps the rate up while a plume, a ladder or the lifted copy moves", () => {
    const stage = rested(menagerie, seated(menagerie, [species]));
    const resting = frameOf(menagerie, stage);
    expect(resting.rate).toBe(0);
    expect(frameOf(menagerie, craft(stage, species, { emitters: [plume("sparks", stage.tick - 3)] })).rate).toBe(64);
    expect(frameOf(menagerie, craft(stage, species, { emitters: [plume("haze", stage.tick - 3)] })).rate).toBe(64);
    expect(frameOf(menagerie, craft(stage, species, { emitters: [plume("motes", stage.tick - 3)] })).rate).toBe(32);
    const ended = craft(stage, species, { emitters: [plume("motes", stage.tick - 300, stage.tick - 200)] });
    expect([frameOf(menagerie, ended).rate, frameOf(menagerie, ended).particles]).toEqual([0, []]);
    const ladder = { owner: species, wall: "card-left", surface: "floor", side: -1 as const, foot: { x: 250, y: 720 }, top: { x: 300, y: 407 }, since: stage.tick - 2, until: stage.tick + 1000, rider: null };
    expect(frameOf(menagerie, { ...stage, ladders: [ladder] }).rate).toBe(64);
    const standing = frameOf(menagerie, { ...stage, ladders: [{ ...ladder, since: stage.tick - 100 }] });
    expect(standing.rate).toBe(0);
    expect(standing.wake).toBeLessThanOrEqual(ladder.until - LADDER_FADE);
    const lift = { fixture: "task-3", pusher: species, since: stage.tick - 30, side: 1 as const, room: 40, span: 300, unit: 0.5 };
    expect(frameOf(menagerie, { ...stage, lift }).rate).toBe(64);
    const holding = frameOf(menagerie, { ...stage, lift: { ...lift, since: stage.tick - 200 } });
    expect(holding.rate).toBe(0);
    expect(holding.wake).toBeLessThanOrEqual(stage.tick - 200 + LIFT_RETURNS);
  });

  it("wakes for a plume that is still to begin, and shows nothing at all, at rate 0 without a wake, on a still stage", () => {
    const stage = rested(menagerie, seated(menagerie, [species]));
    const later = frameOf(menagerie, craft(stage, species, { emitters: [plume("sparks", stage.tick + 40)] }));
    expect(later.rate).toBe(0);
    expect(later.wake).toBeLessThanOrEqual(stage.tick + 40);
    const busy = craft(stage, species, { emitters: [plume("sparks", stage.tick - 3)] });
    const still = frameOf(menagerie, advance(menagerie, { ...busy, lift: { fixture: "task-3", pusher: species, since: busy.tick - 30, side: 1, room: 40, span: 300, unit: 0.5 } }, [{ kind: "tuned", mode: "still" }]));
    expect([still.rate, still.wake, still.particles, still.lifts]).toEqual([0, null, [], []]);
  });

  it("changes nothing it is given and draws the same frame twice", () => {
    const stage = craft(seated(menagerie, [species]), species, { emitters: [plume("sparks", 0), plume("motes", 1)], tilt: 0.05 });
    const copy = structuredClone(stage);
    const first = frameOf(menagerie, Object.freeze(stage));
    expect(frameOf(menagerie, stage)).toEqual(first);
    expect(stage).toEqual(copy);
  });
});
