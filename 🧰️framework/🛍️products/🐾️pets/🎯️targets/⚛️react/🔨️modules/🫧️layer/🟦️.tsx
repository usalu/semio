/** 🫧️ The pet layer: one static, decorative `<div class="pet-layer">` over the page in which a cast of a menagerie lives — the only impure part of the pets target.
 *
 * React renders the empty layer once. A single effect then starts the show: it opens a stage of the core, feeds it
 * events (the survey of the page, the pointer, the learner's hand, the host's wishes, the ticks of the pacer) and paints
 * every frame into one inline SVG per actor. A change of props becomes an event; React never renders per frame. The
 * layer is `aria-hidden`, holds nothing that takes focus, takes no pointer events (`🎨️.css`, and a running show says so
 * on the element itself in case the stylesheet is missing) — the learner's hand reaches the pets through listeners on
 * the window (`../🤏️grasp/🟦️.ts`), and only on a device whose pointer is coarse, while play is permitted, do small pads
 * over the grounded pets take touches, so that a finger can drag a pet instead of scrolling the page —, never changes
 * layout or scroll, makes no sound, no request and no console output, and runs nothing at all while the document is
 * hidden or forced colours are active. A menagerie the validation of the core rejects is never shown: what reaches the
 * page (a palette as custom properties, shapes as attributes) is only what the schema allows. A fault inside the show
 * ends the show silently: pets are decoration and must never break their host.
 *
 * @see ../📡️survey/🟦️.ts — what the layer sees of the page
 * @see ../🤏️grasp/🟦️.ts — how the learner's hand reaches the pets
 * @see ../⏲️pacing/🟦️.ts — how it keeps time
 * @see ../🖌️depiction/🟦️.ts — how an actor is drawn
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html — why the host offers `still` and "off"
 * @see https://www.w3.org/WAI/WCAG22/Understanding/keyboard.html — why a host offers the deeds of the hand without a pointer (`play` of the handle)
 * @see https://www.w3.org/WAI/ARIA/apg/practices/hiding-semantics/ — decorative content
 */

//#region 🔌️Adapters
import { useEffect, useImperativeHandle, useMemo, useRef, useSyncExternalStore, type ReactElement, type Ref } from "react";
import { ROTATION_STREAM, TICKS_PER_SECOND, advance, castOf, frameOf, menagerieIssues, openStage, randomBetween, type ActorFrame, type Cast, type Deed, type Frame, type Menagerie, type PetMode, type Point, type Pointed, type Rect, type Slug, type Species, type Stage, type StageEvent, type Summoned, type Surveyed, type Unpointed } from "@semio-tech/pets";
import { depict, paint, type Depiction } from "../🖌️depiction/🟦️.ts";
import { paintEffects, paintPuffs, retireEffects, stageEffects, stockEffects, strikeEffects, tintEffects } from "../✨️effects/🟦️.ts";
import { equip, paintLadders, paintTools, rackLadders, type Equipment } from "../🧰️gear/🟦️.ts";
import { liftFixtures } from "../🪞️lifting/🟦️.ts";
import { FRAME_TICKS, createPacer, documentHidden, watchVisibility, type Pace } from "../⏲️pacing/🟦️.ts";
import { PET_PRESS_CONTROLS, stageBox, survey, watchPointer, watchSurvey } from "../📡️survey/🟦️.ts";
import { watchGrasp, type Grasped } from "../🤏️grasp/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔖️Defaults
/** 🏠️ The scene whose cast stands in for a scene the menagerie does not know. */
export const PET_HOME_SCENE = "home";

/** 🥞️ Where the layer sits in the stacking order unless the host says otherwise: above cards, below dialogs. */
export const PET_LAYER_Z_INDEX = 35;

/** 📱️ The viewport width in pixels below which a stage is narrow: fewer and smaller pets. */
export const PET_NARROW_WIDTH = 768;

/** 💻️ The viewport width in pixels below which a stage is of medium width. */
export const PET_MEDIUM_WIDTH = 1024;

/** 🧱️ How far from the left and the right edge of the layer every surface ends, in pixels at pet size 1. */
export const PET_EDGE_INSET = 6;

/** 🔁️ The shortest and the longest time in seconds a rotation of the cast stays on stage. */
export const PET_ROTATION_SECONDS = [60, 120] as const;

/** 👆️ The media query of a device whose primary pointer is coarse: there a finger that drags a pet needs a pad that keeps the page from scrolling. */
export const PET_COARSE_POINTER = "(pointer: coarse)";

/** 🖱️ The media query of a device whose primary pointer is fine: only there do pets play with the page (design-v2 §20) — a learner takes a lifted element back by merely pointing at it, which a finger cannot do — so the layer tells the stage that mischief is permitted only while this matches (and while the host permits it). */
export const PET_FINE_POINTER = "(pointer: fine)";

const SURVEY_TICKS = 8;
const PATROL_TICKS = TICKS_PER_SECOND;
const FORCED_COLORS = "(forced-colors: active)";
const OFFSTAGE = "[inert], [hidden]";
const PRESS_KINDS: ReadonlySet<string> = new Set(["pressed", "dragged", "released", "cancelled"]);

/** 🧮️ The most actors a stage of `width` pixels holds unless the host says otherwise: 2 when narrow, 4 at medium width, else 6. */
export function petCapacity(width: number): number {
  return width < PET_NARROW_WIDTH ? 2 : width < PET_MEDIUM_WIDTH ? 4 : 6;
}

/** 🔍️ The size pets are drawn at on a stage of `width` pixels unless the host says otherwise: 0.8 when narrow, else 1. */
export function petScale(width: number): number {
  return width < PET_NARROW_WIDTH ? 0.8 : 1;
}

/** 🎟️ The cast of `scene`; an unknown scene falls back to {@link PET_HOME_SCENE}, then to nothing. */
export function petCast(menagerie: Menagerie, scene: string): Cast | null {
  return menagerie.casts.find((cast) => cast.scene === scene) ?? menagerie.casts.find((cast) => cast.scene === PET_HOME_SCENE) ?? null;
}
//#endregion 🔖️Defaults

//#region 🔖️Scenery
/** 🎭️ What a frame shows of a stage besides the actors' own drawings, in the layer's order: the copies of lifted fixtures and the ladders that stand lie behind every actor (`behind` is the last of them in the layer, `null` while there is none), the tools an actor holds live inside its drawing (`actor`, after the depiction painted it), the dust where pets vanished and the particles in flight lie in front of every actor (`stage` paints ladders, dust and particles). `retire` forgets a species that left the stage, `strike` takes everything away. */
export type Scenery = {
  readonly behind: () => Element | null;
  readonly actor: (depiction: Depiction, actor: ActorFrame) => void;
  readonly stage: (frame: Pick<Frame, "ladders" | "particles" | "puffs">, size: number) => void;
  readonly retire: (species: Slug) => void;
  readonly strike: () => void;
};

/** 🎠️ The scenery of the pet layer `host` for the species of `kinds`, whose copies of lifted fixtures (`copies`, put first in the layer by the lifting) it keeps in front of: it joins the drawings of the depiction, the gear and the effects without any of them knowing another.
 *
 * Built lazily: the rack of standing ladders goes into the layer (behind the actors, after the copies) with the first
 * ladder, the root of the particles and the dust (after everything) with the first particle or puff, so a stage
 * without any adds no element. An actor is equipped on its first frame — its tools, every one hidden, before and after
 * its parts; the pools of its species' emitters stocked —, and the particles take the tint of its state whenever the
 * state changes. Every painter writes only what changed and nothing at all while nothing lives.
 *
 * @see ../🧰️gear/🟦️.ts — the tools and the ladders
 * @see ../✨️effects/🟦️.ts — the particles and the dust
 */
export function stageScenery(host: Element, kinds: ReadonlyMap<Slug, Species>, copies: ReadonlySet<Element>): Scenery {
  const page = host.ownerDocument;
  const rack = rackLadders(page);
  const effects = stageEffects(page);
  const equipped = new WeakMap<Depiction, Equipment>();
  const states = new Map<Slug, Slug>();
  const front = (): Element | null => {
    let child = host.firstElementChild;
    while (child !== null && copies.has(child)) child = child.nextElementSibling;
    return child;
  };
  return {
    behind: () => {
      let last: Element | null = null;
      for (let child = host.firstElementChild; child !== null && (copies.has(child) || child === rack.element); child = child.nextElementSibling) last = child;
      return last;
    },
    actor: (depiction, actor) => {
      let equipment = equipped.get(depiction);
      if (equipment === undefined) {
        equipment = equip(depiction.species, page);
        depiction.element.prepend(equipment.back);
        depiction.element.append(equipment.front);
        stockEffects(effects, depiction.species);
        equipped.set(depiction, equipment);
      }
      paintTools(equipment, actor);
      if (states.get(actor.species) === actor.state) return;
      states.set(actor.species, actor.state);
      tintEffects(effects, depiction.species, depiction.species.states.find((state) => state.id === actor.state)?.tint);
    },
    stage: (frame, size) => {
      if (frame.ladders.length > 0 && rack.element.parentNode !== host) host.insertBefore(rack.element, front());
      paintLadders(rack, frame.ladders, size);
      if ((frame.particles.length > 0 || frame.puffs.length > 0) && effects.element.parentNode !== host) host.append(effects.element);
      paintPuffs(effects, frame.puffs, size);
      paintEffects(effects, kinds, frame.particles, size);
    },
    retire: (species) => {
      retireEffects(effects, species);
      states.delete(species);
    },
    strike: () => {
      rack.element.remove();
      strikeEffects(effects);
      states.clear();
    },
  };
}
//#endregion 🔖️Scenery

//#region 🔖️Show
/** 🎚️ Everything the host may change while a show runs. */
type Direction = Pick<PetLayerProps, "scene" | "mode" | "quiet" | "play" | "mischief" | "capacity" | "surfaces" | "keepouts" | "controls" | "walls" | "props" | "glances" | "scale" | "zIndex" | "onCast">;

/** 🎬️ A running show: `direct` hands it the host's current wishes, `play` a deed the learner asked a pet for without a pointer, `stop` ends it and leaves nothing behind. */
type Show = { readonly direct: (direction: Direction) => void; readonly play: (species: Slug, deed: Deed) => void; readonly stop: () => void };

/** 🩹️ A touch pad over one grounded pet and the placement last written to it. */
type Pad = { readonly element: HTMLElement; placed: string };

/** 🟰️ Whether two surveys say the same; a survey that changed nothing is not worth an event. */
function sameSurvey(one: Surveyed | null, other: Surveyed): boolean {
  if (one === null || one.width !== other.width || one.height !== other.height || one.surfaces.length !== other.surfaces.length || one.keepouts.length !== other.keepouts.length || one.walls.length !== other.walls.length || one.fixtures.length !== other.fixtures.length) return false;
  for (const [index, surface] of other.surfaces.entries()) {
    const known = one.surfaces[index]!;
    if (known.id !== surface.id || known.x0 !== surface.x0 || known.x1 !== surface.x1 || known.y !== surface.y) return false;
  }
  for (const [index, box] of other.keepouts.entries()) {
    const known = one.keepouts[index]!;
    if (known.x !== box.x || known.y !== box.y || known.width !== box.width || known.height !== box.height) return false;
  }
  for (const [index, wall] of other.walls.entries()) {
    const known = one.walls[index]!;
    if (known.id !== wall.id || known.surface !== wall.surface || known.side !== wall.side || known.x !== wall.x || known.y0 !== wall.y0 || known.y1 !== wall.y1) return false;
  }
  for (const [index, fixture] of other.fixtures.entries()) {
    const known = one.fixtures[index]!;
    if (known.id !== fixture.id || known.key !== fixture.key || known.x !== fixture.x || known.y !== fixture.y || known.width !== fixture.width || known.height !== fixture.height) return false;
  }
  return true;
}

/** 📏️ A survey as the stage gets it. Pets are authored in pixels at size 1, so a stage that draws them at `size` lives in pixels ÷ `size`: every distance of the simulation (headroom, spacing, speed, the reach of a hop) then shrinks or grows with the pets. And every surface ends {@link PET_EDGE_INSET} short of the left and the right edge of the layer, which clips what it draws: an arm, a ray or a foot that reaches a little beyond the box of its pet is never cut off. */
function staged(seen: Surveyed, size: number): Surveyed {
  const width = seen.width / size;
  return {
    kind: "surveyed",
    width,
    height: seen.height / size,
    surfaces: seen.surfaces.map((surface) => ({ id: surface.id, x0: Math.max(surface.x0 / size, PET_EDGE_INSET), x1: Math.min(surface.x1 / size, width - PET_EDGE_INSET), y: surface.y / size })),
    keepouts: size === 1 ? seen.keepouts : seen.keepouts.map((box) => ({ x: box.x / size, y: box.y / size, width: box.width / size, height: box.height / size })),
    walls: size === 1 ? seen.walls : seen.walls.map((wall) => ({ id: wall.id, surface: wall.surface, side: wall.side, x: wall.x / size, y0: wall.y0 / size, y1: wall.y1 / size })),
    fixtures: size === 1 ? seen.fixtures : seen.fixtures.map((fixture) => ({ id: fixture.id, key: fixture.key, x: fixture.x / size, y: fixture.y / size, width: fixture.width / size, height: fixture.height / size })),
  };
}

/** 📍️ Whether two lists name the same points in the same order. */
function samePoints(one: readonly Point[], other: readonly Point[]): boolean {
  return one.length === other.length && other.every((point, index) => one[index]!.x === point.x && one[index]!.y === point.y);
}

/** 🎯️ Whether a point lies in a box, its edges included. */
function within(box: Rect, x: number, y: number): boolean {
  return x >= box.x && x <= box.x + box.width && y >= box.y && y <= box.y + box.height;
}

/** 🎪️ Starts a show of `menagerie` in the layer element `host`: opens a stage seeded with `seed`, watches the page, the
 * pointer, the learner's hand and the visibility of the document, and paints one depiction per actor of every frame
 * into `host`.
 *
 * A step happens inside an animation frame and is the only place the stage advances. It reads first (a survey when the
 * last one went stale: at once when the page moves or nothing runs, otherwise at most every eight ticks; while actors
 * travel the page is measured every eight ticks anyway — but not because a pet is held, which keeps the stage at full
 * rate for as long as the learner holds it —, while they only stand once a second), folds the ticks that passed, the
 * fixtures given back since the last step (before the survey: the learner who took one back points at it or focuses
 * it, and a survey has no fixture under the pointer or the focus — told first, the stage throws its pusher off instead
 * of ending the prank quietly), the survey and then every other waiting event into the stage, and writes last (the
 * frame). The stage lives in the pets' own units — pixels ÷ the size they are drawn at ({@link staged}) —, so
 * surveys, the pointer, the hand and glances are divided by that size on the way in and the feet of every actor multiplied by it on the way out. A still stage hears
 * nothing of the pointer — nothing on it answers the pointer — and the hand takes nothing on it. Of the hand's events the stage gets every press, release and cancellation, the latest
 * drag of a frame, and one stir and one scroll at a time. The pacer hands out at most eight ticks while the stage runs
 * and every tick slept after a rest, which the stage jumps over; its clock runs `tempo` times as fast as the wall clock
 * (held between an eighth and eight: beyond eight ticks a frame time would be dropped). The rotation of the cast
 * advances every 60 to 120 seconds of stage time, never while the stage is `still` or quiet. The show rests — no frame,
 * no timer, no survey, the pets where they are, a held pet called off — while the document is hidden and while the
 * layer itself is inert or hidden (a host that opens a modal dialog makes everything else inert: the pets wait behind
 * it instead of falling off surfaces that seem to have vanished).
 *
 * Beside the actors a frame shows the tools they hold, the ladders that stand and the particles in flight
 * ({@link stageScenery}) and the copies of the fixtures pets lifted (`../🪞️lifting/🟦️.ts`): a fixture the learner
 * reaches for comes back in the same task and the stage hears `reclaimed`; every lift is given back as well when the
 * show rests, the scene changes, the stage turns still, mischief is no longer permitted or the show ends, and a lifted
 * fixture whose box moved under a survey is given back with that survey. Mischief is permitted to the stage only
 * where the primary pointer is fine ({@link PET_FINE_POINTER}); when it stops being fine, every lift is given back and
 * the stage hears that mischief is no longer permitted.
 *
 * Touch pads: on a device whose primary pointer is coarse ({@link PET_COARSE_POINTER}), while play is permitted and
 * the stage is not still, every visible pet that stands on a perch gets a pad — a `div.pet-pad` over its solid box
 * down to its feet, which takes the touch and keeps the page from panning (`🎨️.css`) — so that a finger can pick it
 * up. Grounded pets stand on perches cleared of controls, so a pad never covers one; a pet in the air, in the hand, on
 * a rope, a ladder or a wall has none.
 */
function startShow(host: HTMLElement, menagerie: Menagerie, seed: number, tempo: number, first: Direction): Show {
  const page = host.ownerDocument;
  const view = page.defaultView;
  if (view === null) return { direct: () => {}, play: () => {}, stop: () => {} };
  const haste = Math.min(Math.max(tempo, 1 / FRAME_TICKS), FRAME_TICKS);
  const kinds = new Map<Slug, Species>(menagerie.species.map((species) => [species.id, species]));
  const depictions = new Map<Slug, Depiction>();
  const pads = new Map<Slug, Pad>();
  const coarse = typeof view.matchMedia === "function" ? view.matchMedia(PET_COARSE_POINTER) : null;
  const fine = typeof view.matchMedia === "function" ? view.matchMedia(PET_FINE_POINTER) : null;
  let direction = first;
  let stage: Stage = advance(menagerie, openStage(seed), [
    { kind: "tuned", mode: first.mode },
    { kind: "hushed", quiet: first.quiet === true },
    { kind: "permitted", play: first.play !== false, mischief: first.mischief !== false && fine?.matches !== false },
  ]);
  let frame: Frame | null = null;
  let waiting: StageEvent[] = [];
  let stale: "now" | "soon" | null = "now";
  let surveyed: Surveyed | null = null;
  let surveyedSize = 1;
  let surveyedAt = 0;
  let glanced: readonly Point[] = [];
  let origin: Point = { x: 0, y: 0 };
  let width = 0;
  let epoch = 0;
  let rotateAt: number | null = null;
  let wished: string | null = null;
  let summoned: string | null = null;
  let redraw = true;
  let told = "";
  let toldTo: PetLayerProps["onCast"] = undefined;
  let padding: HTMLElement | null = null;
  let ended = false;
  let pointer: Point | null = null;
  const lifting = liftFixtures(host, {
    props: () => direction.props,
    quiet: () => direction.quiet === true,
    reclaimed: (fixture) => {
      waiting.push({ kind: "reclaimed", fixture });
      pacer.wake();
    },
  });
  const scenery = stageScenery(host, kinds, lifting.copies);

  const capacity = (): number => direction.capacity ?? petCapacity(width);
  const scale = (): number => direction.scale ?? petScale(width);
  const controls = (): string => (direction.controls === undefined ? PET_PRESS_CONTROLS : `${PET_PRESS_CONTROLS}, ${direction.controls}`);
  const resting = (): boolean => documentHidden(view) || host.closest(OFFSTAGE) !== null;
  const playable = (): boolean => direction.play !== false && direction.mode !== "still" && !ended && !resting();

  const summon = (): Summoned | null => {
    const wish = `${direction.scene} ${capacity()} ${epoch}`;
    if (wish === wished) return null;
    wished = wish;
    const cast = petCast(menagerie, direction.scene);
    const species = cast === null ? [] : castOf(cast, capacity(), epoch, seed);
    const wanted = species.join(" ");
    if (wanted === summoned) return null;
    summoned = wanted;
    return { kind: "summoned", species };
  };

  const measure = (tick: number): StageEvent[] => {
    const box = stageBox(page, host);
    const seen = survey(page, { surfaces: direction.surfaces, keepouts: direction.keepouts, walls: direction.walls, props: direction.props, pointer, frame: host });
    lifting.measured();
    const events: StageEvent[] = [];
    origin = { x: box.x, y: box.y };
    width = seen.width;
    stale = null;
    surveyedAt = tick;
    const size = scale();
    if (size !== surveyedSize || !sameSurvey(surveyed, seen)) {
      surveyed = seen;
      surveyedSize = size;
      events.push(staged(seen, size));
    }
    const points = direction.glances?.().map((point) => ({ x: (point.x - box.x) / size, y: (point.y - box.y) / size })) ?? [];
    if (!samePoints(glanced, points)) {
      glanced = points;
      events.push({ kind: "glanced", points });
    }
    return events;
  };

  const pad = (next: Frame, size: number): void => {
    const shown = new Set<Slug>();
    if (coarse?.matches === true && playable()) {
      for (const actor of next.actors) {
        const height = Math.min(actor.body.height, actor.y - actor.body.y);
        if (actor.footing !== "perch" || actor.opacity <= 0 || height <= 0 || actor.body.width <= 0) continue;
        if (padding === null) {
          padding = page.createElement("div");
          padding.className = "pet-pads";
          host.append(padding);
        }
        let known = pads.get(actor.species);
        if (known === undefined) {
          const element = page.createElement("div");
          element.className = "pet-pad";
          padding.append(element);
          known = { element, placed: "" };
          pads.set(actor.species, known);
        }
        const placed = `${Math.round(actor.body.x * size * 100) / 100} ${Math.round(actor.body.y * size * 100) / 100} ${Math.round(actor.body.width * size * 100) / 100} ${Math.round(height * size * 100) / 100}`;
        if (placed !== known.placed) {
          const [x, y, wide, high] = placed.split(" ");
          known.element.style.transform = `translate(${x}px, ${y}px)`;
          known.element.style.width = `${wide}px`;
          known.element.style.height = `${high}px`;
          known.placed = placed;
        }
        shown.add(actor.species);
      }
    }
    for (const [species, known] of pads) {
      if (shown.has(species)) continue;
      known.element.remove();
      pads.delete(species);
    }
  };

  const show = (next: Frame): void => {
    const size = scale();
    const drawn = new Set<Slug>();
    lifting.realise(next.lifts, origin, size);
    scenery.stage(next, size);
    let before: Element | null = scenery.behind();
    for (const actor of next.actors) {
      const kind = kinds.get(actor.species);
      if (kind === undefined) continue;
      let depiction = depictions.get(actor.species);
      if (depiction === undefined) {
        depiction = depict(kind, page);
        depictions.set(actor.species, depiction);
      }
      paint(depiction, size === 1 ? actor : { ...actor, x: actor.x * size, y: actor.y * size }, size);
      scenery.actor(depiction, actor);
      const place: Element | null = before === null ? host.firstElementChild : before.nextElementSibling;
      if (depiction.element !== place) host.insertBefore(depiction.element, place);
      before = depiction.element;
      drawn.add(actor.species);
    }
    for (const [id, depiction] of depictions) {
      if (drawn.has(id)) continue;
      depiction.element.remove();
      depictions.delete(id);
      scenery.retire(id);
    }
    pad(next, size);
    grasp.frame(next.held !== null);
    announce(menagerie.species.filter((species) => drawn.has(species.id)).map((species) => species.id));
  };

  const announce = (cast: readonly Slug[]): void => {
    const listener = direction.onCast;
    const names = cast.join(" ");
    if (names === told && listener === toldTo) return;
    const news = names !== told || names !== "";
    told = names;
    toldTo = listener;
    if (news) listener?.(cast);
  };

  const step = (ticks: number): Pace => {
    const events: StageEvent[] = [];
    if (ticks > 0) events.push({ kind: "ticked", ticks });
    events.push(...waiting.filter((known) => known.kind === "reclaimed"));
    waiting = waiting.filter((known) => known.kind !== "reclaimed");
    const tick = stage.tick + ticks;
    const rate = frame?.rate ?? 0;
    const since = tick - surveyedAt;
    const holding = (frame?.held ?? null) !== null;
    if (stale === "now" || (stale === "soon" && (rate === 0 || since >= SURVEY_TICKS)) || (rate === 64 && !holding && since >= SURVEY_TICKS) || (rate > 0 && since >= PATROL_TICKS)) events.push(...measure(tick));
    events.push(...waiting);
    waiting = [];
    if (direction.mode !== "still" && direction.quiet !== true) {
      if (rotateAt !== null && tick >= rotateAt) epoch += 1;
      if (rotateAt === null || tick >= rotateAt) rotateAt = tick + Math.floor(randomBetween([seed, ROTATION_STREAM, epoch], PET_ROTATION_SECONDS[0], PET_ROTATION_SECONDS[1]) * TICKS_PER_SECOND);
    }
    const cast = summon();
    if (cast !== null) events.push(cast);
    let shown = frame;
    if (events.length > 0 || shown === null) {
      stage = advance(menagerie, stage, events);
      shown = frameOf(menagerie, stage);
      frame = shown;
    }
    if (events.length > 0 || redraw) show(shown);
    redraw = false;
    return shown;
  };

  const pacer = createPacer(
    (ticks) => {
      try {
        return step(ticks);
      } catch {
        stop();
        return { tick: 0, rate: 0, wake: null };
      }
    },
    {
      now: () => view.performance.now() * haste,
      requestFrame: (callback) => view.requestAnimationFrame((time) => callback(time * haste)),
      cancelFrame: (handle) => view.cancelAnimationFrame(handle),
      setTimer: (callback, milliseconds) => view.setTimeout(callback, milliseconds / haste),
      clearTimer: (handle) => view.clearTimeout(handle as number),
    },
  );

  const hit = (x: number, y: number): boolean => {
    const size = scale();
    return frame?.actors.some((actor) => actor.opacity > 0 && within(actor.body, x / size, y / size)) ?? false;
  };

  const point = (event: Pointed | Unpointed): boolean => {
    if (direction.mode === "still") return false;
    const size = scale();
    const staged: Pointed | Unpointed = event.kind === "unpointed" ? event : { kind: event.kind, x: event.x / size, y: event.y / size, over: event.over };
    waiting = waiting.filter((known) => known.kind !== "pointed" && known.kind !== "unpointed");
    waiting.push(staged);
    return true;
  };

  const hand = (event: Grasped): void => {
    if (ended || (direction.mode === "still" && event.kind !== "cancelled")) return;
    const size = scale();
    if (event.kind === "dragged") {
      let last = waiting.length - 1;
      while (last >= 0 && !PRESS_KINDS.has(waiting[last]!.kind)) last -= 1;
      if (last >= 0 && waiting[last]!.kind === "dragged") waiting.splice(last, 1);
    } else if ((event.kind === "stirred" || event.kind === "scrolled") && waiting.some((known) => known.kind === event.kind)) return;
    waiting.push("x" in event ? { ...event, x: event.x / size, y: event.y / size } : event);
    if (frame !== null && frame.actors.length > 0) pacer.wake();
  };

  const grasp = watchGrasp(view, hand, { origin: () => origin, controls, takes: (x, y) => playable() && hit(x, y), now: () => view.performance.now() });

  const rest = (hidden = false): void => {
    if (hidden || resting()) {
      grasp.cancel();
      lifting.release();
      pacer.hide();
    } else pacer.show();
  };

  const repointed = (): void => {
    if (fine?.matches === false) lifting.release();
    waiting.push({ kind: "permitted", play: direction.play !== false, mischief: direction.mischief !== false && fine?.matches !== false });
    pacer.wake();
  };
  fine?.addEventListener?.("change", repointed);

  const unwatch = [
    () => fine?.removeEventListener?.("change", repointed),
    watchSurvey(
      view,
      page,
      (urgent) => {
        if (urgent || stale === null) stale = urgent ? "now" : "soon";
        rest();
        pacer.wake();
      },
      { ignore: host },
    ),
    watchPointer(
      view,
      (event) => {
        pointer = event.kind === "pointed" ? { x: event.x + origin.x, y: event.y + origin.y } : null;
        if (point(event) && frame !== null && frame.actors.length > 0) pacer.wake();
      },
      { origin: () => origin, controls },
    ),
    watchVisibility(view, rest),
    grasp.stop,
  ];

  function stop(): void {
    if (ended) return;
    ended = true;
    pacer.stop();
    for (const end of unwatch) end();
    lifting.end();
    scenery.strike();
    for (const depiction of depictions.values()) depiction.element.remove();
    depictions.clear();
    pads.clear();
    padding?.remove();
    padding = null;
    host.style.removeProperty("z-index");
    host.style.removeProperty("pointer-events");
    announce([]);
  }

  host.style.zIndex = String(first.zIndex ?? PET_LAYER_Z_INDEX);
  host.style.pointerEvents = "none";
  rest();
  pacer.wake();

  return {
    direct: (next) => {
      if (ended) return;
      const before = direction;
      direction = next;
      if (next.zIndex !== before.zIndex) host.style.zIndex = String(next.zIndex ?? PET_LAYER_Z_INDEX);
      const hushed = (next.quiet === true) !== (before.quiet === true);
      const permitted = (next.play !== false) !== (before.play !== false) || (next.mischief !== false) !== (before.mischief !== false);
      const resurvey = next.surfaces !== before.surfaces || next.keepouts !== before.keepouts || next.walls !== before.walls || next.props !== before.props;
      if (next.mode !== before.mode) {
        if (next.mode === "still") waiting = waiting.filter((known) => known.kind !== "pointed" && known.kind !== "unpointed" && known.kind !== "stirred" && known.kind !== "scrolled");
        waiting.push({ kind: "tuned", mode: next.mode });
      }
      if (hushed) waiting.push({ kind: "hushed", quiet: next.quiet === true });
      if (next.scene !== before.scene || (next.mode === "still" && before.mode !== "still") || (next.mischief === false && before.mischief !== false)) lifting.release();
      if (!playable()) grasp.cancel();
      if (permitted) waiting.push({ kind: "permitted", play: next.play !== false, mischief: next.mischief !== false && fine?.matches !== false });
      if (next.mode !== before.mode || hushed) rotateAt = null;
      if (resurvey || next.scale !== before.scale) stale = "now";
      else if ((next.glances === undefined) !== (before.glances === undefined) && stale === null) stale = "soon";
      if (next.scale !== before.scale || next.onCast !== before.onCast || permitted || next.mode !== before.mode) redraw = true;
      if (next.mode !== before.mode || hushed || permitted || stale !== null || redraw || next.scene !== before.scene || next.capacity !== before.capacity) pacer.wake();
    },
    play: (species, deed) => {
      if (ended) return;
      waiting.push({ kind: "played", species, deed });
      pacer.wake();
    },
    stop,
  };
}
//#endregion 🔖️Show

//#region 🔖️Layer
/** 🕹️ What a host can ask of a running layer by its `ref`: `play` hands the stage a deed the learner asked a pet on stage for without a pointer — `hello`, `trick`, `pet` or `toss` from a "Play with the pets" group of the host's settings, the keyboard equivalent of the hand (design-v2 §14.5). A deed before the show runs, after it ended, under forced colours or for a species not on stage does nothing; whether a pet answers is the stage's to decide (a still stage or play that is not permitted answers nothing). */
export interface PetLayerHandle {
  readonly play: (species: Slug, deed: Deed) => void;
}

/** 🎛️ What a host tells the pet layer: the menagerie; the scene whose cast is on stage (an unknown scene falls back to
 * `home`, then to nothing); the liveliness (`off` is expressed by not rendering the layer); whether a time of
 * concentration runs; whether the learner lets the pets be played with — they answer clicks and can be picked up
 * (`play`, true when absent) — and lets them play with the page (`mischief`, true when absent), both handed to the
 * stage as `permitted`; the most actors at once (by viewport width when absent: below 768 px 2, below 1024 px 4, else
 * 6); the selector of the elements whose top edge carries pets (`[data-pet-surface]`), of what pets must not cover
 * (interactive controls, text blocks, `[data-pet-keepout]`), of more controls a press on a pet never takes away from
 * besides links, buttons, form fields, labels, ARIA widgets and focusable elements (`controls`, such as the host's drag
 * grips), of more elements whose sides pets may climb besides those of the surfaces (`walls`) and of the elements a
 * pet may play with (`props`, `[data-pet-prop]`); other things worth a look, in viewport pixels, sampled with every
 * survey; the size pets are drawn at (1; 0.8 below 768 px); the seed of the stage (a random one per mount); the
 * stacking order (35); who wants to know which species are on stage right now (`onCast`: called, in the order of the
 * menagerie, whenever that set changes — also with nobody when the show ends — and once for a new listener while
 * somebody is on stage); how fast the pets' time passes (`tempo`: 1 is the wall clock, less is slow motion, more — up
 * to eight — is for whoever has to see in seconds what takes minutes, such as a test of an encounter; a new tempo
 * begins a new show); and `ref`, which receives the {@link PetLayerHandle} of the layer. */
export interface PetLayerProps {
  readonly menagerie: Menagerie;
  readonly scene: string;
  readonly mode: PetMode;
  readonly quiet?: boolean;
  readonly play?: boolean;
  readonly mischief?: boolean;
  readonly capacity?: number;
  readonly surfaces?: string;
  readonly keepouts?: string;
  readonly controls?: string;
  readonly walls?: string;
  readonly props?: string;
  readonly glances?: () => readonly Point[];
  readonly scale?: number;
  readonly seed?: number;
  readonly zIndex?: number;
  readonly onCast?: (species: readonly Slug[]) => void;
  readonly tempo?: number;
  readonly ref?: Ref<PetLayerHandle>;
}

/** 🎨️ Whether the system forces its own colours on the page, where a pet's palette would be lost. */
function forcedColors(): boolean {
  return typeof window !== "undefined" && typeof window.matchMedia === "function" && window.matchMedia(FORCED_COLORS).matches;
}

/** 👂️ Calls `change` whenever forced colours are switched on or off and returns the function that ends the watch. */
function watchForcedColors(change: () => void): () => void {
  if (typeof window === "undefined" || typeof window.matchMedia !== "function") return () => {};
  const query = window.matchMedia(FORCED_COLORS);
  query.addEventListener("change", change);
  return () => query.removeEventListener("change", change);
}

/** 🐾️ A cast of pets living on the page: decoration that stands on the top edges of the host's surfaces, looks at the
 * pointer and at each other, answers the learner's hand where nothing of the page lies beneath it, and never gets in
 * the way. Renders one static element; everything else happens outside React. Under forced colours, and for a
 * menagerie with issues ({@link menagerieIssues}), the layer stays empty and runs nothing. When the layer goes or
 * play is switched off, a pet the learner holds is called off and the hand lets go of everything. */
export function PetLayer(props: PetLayerProps): ReactElement {
  const { menagerie, scene, mode, quiet, play, mischief, capacity, surfaces, keepouts, controls, walls, props: fixtures, glances, scale, seed, zIndex, onCast, tempo = 1, ref } = props;
  const host = useRef<HTMLDivElement>(null);
  const show = useRef<Show | null>(null);
  const forced = useSyncExternalStore(watchForcedColors, forcedColors, () => false);
  const sound = useMemo(() => menagerieIssues(menagerie).length === 0, [menagerie]);
  useImperativeHandle(ref, () => ({ play: (species, deed) => show.current?.play(species, deed) }), []);
  useEffect(() => {
    if (forced || !sound || host.current === null) return;
    const started = startShow(host.current, menagerie, seed ?? Math.floor(Math.random() * 0x1_0000_0000), tempo, { scene, mode, quiet, play, mischief, capacity, surfaces, keepouts, controls, walls, props: fixtures, glances, scale, zIndex, onCast });
    show.current = started;
    return () => {
      started.stop();
      show.current = null;
    };
  }, [menagerie, sound, seed, tempo, forced]);
  useEffect(() => {
    show.current?.direct({ scene, mode, quiet, play, mischief, capacity, surfaces, keepouts, controls, walls, props: fixtures, glances, scale, zIndex, onCast });
  }, [scene, mode, quiet, play, mischief, capacity, surfaces, keepouts, controls, walls, fixtures, glances, scale, zIndex, onCast]);
  return <div ref={host} className="pet-layer" aria-hidden="true" />;
}
//#endregion 🔖️Layer
