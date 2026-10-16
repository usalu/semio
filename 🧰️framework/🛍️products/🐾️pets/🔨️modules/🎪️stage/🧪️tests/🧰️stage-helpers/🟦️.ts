/** 🎪️ Shared kit of the unit suite of the stage (the topics sit in the sibling folders `🚪️arrival-and-gaze`, `🚶️walking-and-encounters`, `🎬️frames-and-summoning`, `📏️distance-and-turning`, `👁️attention-and-gait`, `🧠️determinism-and-mind`, `🦾️body-and-gear` and `🪄️mischief-and-arithmetic`): the troupe, the meadow and the event builders. The suite checks: everything the owner asked of the pets, checked on the fold itself — slightly active by default, eyes that follow the pointer while standing still, blinking, walking along perches and riding a surface that moves, falling when a perch vanishes and landing, encounters that are affectionate for friends and small disputes (squabble → sulk → mending) for rivals, quiet and still, summon and leave, the events of the learner's hand, what an actor arrives with and stands on, determinism, and the rate, the wake and the fields a frame reports.
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
import { expect } from "vitest";
import { TICKS_PER_SECOND, type Actor, type Fixture, type Menagerie, type PetMode, type Rect, type Slug, type Species, type Stage, type StageEvent, type Surface, type Wall } from "../../../../🧬️schema/🟦️.ts";
import { atRest } from "../../../💗️feeling/🟦️.ts";
import { overlaps } from "../../../🚧️clearance/🟦️.ts";
import { bodiesOf } from "../../../📏️spacing/🟦️.ts";
import { advance, openStage } from "../../🟦️.ts";

export const HERE = dirname(fileURLToPath(import.meta.url));

export const SECOND = TICKS_PER_SECOND;

export const WIDTH = 1280;

export const HEIGHT = 720;

export const FLOOR: Surface = { id: "floor", x0: 0, x1: WIDTH, y: HEIGHT };

export const CARD: Surface = { id: "card", x0: 300, x1: 800, y: 400 };

export const FAR = 100000000;

export const COMFORT = 8;

/** 🧫️ One committed fixture of the product. */
function fixture<T>(name: string): T {
  return JSON.parse(readFileSync(resolve(HERE, "../../../../🧪️tests/🧫️fixtures", name, "🔣️.json"), "utf8")) as T;
}

type Company = { readonly name: string; readonly menagerie: Menagerie; readonly walker: Slug; readonly hopper: Slug; readonly floater: Slug; readonly friends: readonly [Slug, Slug]; readonly rivals: readonly [Slug, Slug]; readonly strangers: readonly [Slug, Slug] };

export const SAMPLE: Company = { name: "the sample menagerie", menagerie: fixture<{ menagerie: Menagerie }>("🧬️schema-conformance").menagerie, walker: "blobby", hopper: "hoppy", floater: "floaty", friends: ["blobby", "hoppy"], rivals: ["blobby", "floaty"], strangers: ["hoppy", "floaty"] };

const TROUPE: Company = { name: "the trace troupe", menagerie: fixture<{ menagerie: Menagerie }>("🎪️stage-trace").menagerie, walker: "mossy", hopper: "sparky", floater: "misty", friends: ["mossy", "sparky"], rivals: ["mossy", "thorny"], strangers: ["mossy", "pebble"] };

export const COMPANIES = [SAMPLE, TROUPE];

/** ⏭️ Time passing. */
export function ticked(ticks = 1): StageEvent {
  return { kind: "ticked", ticks };
}

/** 🗺️ A survey of the stage. */
export function surveyed(surfaces: readonly Surface[], keepouts: readonly Rect[] = [], width = WIDTH, height = HEIGHT, walls: readonly Wall[] = [], fixtures: readonly Fixture[] = []): StageEvent {
  return { kind: "surveyed", width, height, surfaces, keepouts, walls, fixtures };
}

/** 👆️ A click of the learner's hand at a position: a press and its release. */
export function clicked(x: number, y: number): StageEvent[] {
  return [
    { kind: "pressed", x, y, pointer: "mouse" },
    { kind: "released", x, y },
  ];
}

/** 🧬️ A species of a menagerie. */
export function kindOf(menagerie: Menagerie, id: Slug): Species {
  return menagerie.species.find((species) => species.id === id)!;
}

/** 🎈️ The hover of a species. */
export function hoverOf(kind: Species): number {
  return kind.locomotion.gait === "float" ? (kind.locomotion.hover ?? 0) : 0;
}

/** 🦘️ How far one hop carries a species that walks in hops (its speed × the length of its hop clip), 0 for every other gait. */
export function hopOf(kind: Species): number {
  if (kind.locomotion.gait !== "hop") return 0;
  const clip = kind.clips.find((candidate) => candidate.id === (kind.repertoire.walk ?? kind.repertoire.hop ?? [])[0]);
  return clip === undefined ? 0 : (kind.locomotion.speed * Math.max(Math.floor(clip.seconds * SECOND + 0.5), 1)) / SECOND;
}

/** 🧸️ The actor of a species on a stage. */
export function actorOf(stage: Stage, species: Slug): Actor {
  const actor = stage.actors.find((candidate) => candidate.species === species);
  if (actor === undefined) throw new Error(`${species} is not on stage`);
  return actor;
}

/** 🎬️ A stage on which species were summoned onto surveyed surfaces. */
export function staged(menagerie: Menagerie, species: readonly Slug[], options: { seed?: number; mode?: PetMode; surfaces?: readonly Surface[]; keepouts?: readonly Rect[] } = {}): Stage {
  return advance(menagerie, openStage(options.seed ?? 1), [{ kind: "tuned", mode: options.mode ?? "calm" }, surveyed(options.surfaces ?? [CARD, FLOOR], options.keepouts ?? []), { kind: "summoned", species }]);
}

/** 🚧️ Every pair of actors whose bodies overlap on a stage (`overlaps` of their bodies): the invariant of the stage says there is none at the end of any tick. */
export function collisions(menagerie: Menagerie, stage: Stage): string[] {
  return overlaps(bodiesOf(stage.actors, stage.actors.map((actor) => kindOf(menagerie, actor.species)))).map((pair) => `${pair.first}+${pair.second} at tick ${stage.tick}`);
}

/** 🧱️ Holds a stage to the invariant: it fails, naming the pairs, when bodies overlap (an assertion only then, so that a check after every tick stays cheap). */
export function heldApart(menagerie: Menagerie, stage: Stage): void {
  const found = collisions(menagerie, stage);
  if (found.length > 0) expect(found).toEqual([]);
}

/** 🏃️ A stage after `ticks` single ticks, every one of them held to the invariant ({@link heldApart}); `watch` sees every stage with the one before it. */
export function run(menagerie: Menagerie, stage: Stage, ticks: number, watch?: (after: Stage, before: Stage) => void): Stage {
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
export function craft(stage: Stage, species: Slug, changes: Partial<Actor>): Stage {
  return { ...stage, actors: stage.actors.map((actor) => (actor.species === species ? { ...actor, ...changes } : actor)) };
}

/** 🪑️ A stage in which an actor stands whole and idle at `x` on the card for as long as the test runs, without a blink in sight. */
export function seated(menagerie: Menagerie, stage: Stage, species: Slug, x: number, changes: Partial<Actor> = {}): Stage {
  return craft(stage, species, { perch: CARD.id, x, y: CARD.y - hoverOf(kindOf(menagerie, species)), vx: 0, vy: 0, goal: x, activity: "idle", since: stage.tick, until: stage.tick + FAR, blink: stage.tick + FAR, partner: null, opacity: 1, leaving: false, feeling: atRest(kindOf(menagerie, species).mood, stage.tick), gaze: { x: 0, y: 0, vx: 0, vy: 0 }, ...changes });
}

/** 🧍️ A stage with one actor seated on the card. */
export function alone(company: Company, species: Slug, changes: Partial<Actor> = {}): Stage {
  return seated(company.menagerie, staged(company.menagerie, [species]), species, 550, changes);
}

/** 📦️ Whether the box of a grounded actor touches a keep-out. */
export function covers(menagerie: Menagerie, actor: Actor, keepout: Rect): boolean {
  const kind = kindOf(menagerie, actor.species);
  return Math.max(actor.x - kind.size.width / 2, keepout.x) < Math.min(actor.x + kind.size.width / 2, keepout.x + keepout.width) && Math.max(actor.y - kind.size.height, keepout.y) < Math.min(actor.y, keepout.y + keepout.height);
}
