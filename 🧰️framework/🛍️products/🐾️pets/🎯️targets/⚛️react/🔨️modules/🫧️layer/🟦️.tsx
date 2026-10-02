/** 🫧️ The pet layer: one static, decorative `<div class="pet-layer">` over the page in which a cast of a menagerie lives — the only impure part of the pets target.
 *
 * React renders the empty layer once. A single effect then starts the show: it opens a stage of the core, feeds it
 * events (the survey of the page, the pointer, the host's wishes, the ticks of the pacer) and paints every frame into
 * one inline SVG per actor. A change of props becomes an event; React never renders per frame. The layer is
 * `aria-hidden`, holds nothing that takes focus, never takes pointer events (`🎨️.css`, and a running show says so on
 * the element itself in case the stylesheet is missing), never changes layout or scroll, makes no sound, no request
 * and no console output, and runs nothing at all while the document is hidden or forced colours are active. A
 * menagerie the validation of the core rejects is never shown: what reaches the page (a palette as custom properties,
 * shapes as attributes) is only what the schema allows. A fault inside the show ends the show silently: pets are
 * decoration and must never break their host.
 *
 * @see ../📡️survey/🟦️.ts — what the layer sees of the page
 * @see ../⏲️pacing/🟦️.ts — how it keeps time
 * @see ../🖌️depiction/🟦️.ts — how an actor is drawn
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html — why the host offers `still` and "off"
 * @see https://www.w3.org/WAI/ARIA/apg/practices/hiding-semantics/ — decorative content
 */

//#region 🔌️Adapters
import { useEffect, useMemo, useRef, useSyncExternalStore, type ReactElement } from "react";
import { ROTATION_STREAM, TICKS_PER_SECOND, advance, castOf, frameOf, menagerieIssues, openStage, randomBetween, type Cast, type Frame, type Menagerie, type PetMode, type Point, type Pointed, type Poked, type Slug, type Species, type Stage, type StageEvent, type Summoned, type Surveyed, type Unpointed } from "@semio-tech/pets";
import { depict, paint, type Depiction } from "../🖌️depiction/🟦️.ts";
import { FRAME_TICKS, createPacer, documentHidden, watchVisibility, type Pace } from "../⏲️pacing/🟦️.ts";
import { stageBox, survey, watchPointer, watchSurvey } from "../📡️survey/🟦️.ts";
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

const SURVEY_TICKS = 8;
const PATROL_TICKS = TICKS_PER_SECOND;
const POINTER_REACH = 8;
const FORCED_COLORS = "(forced-colors: active)";
const OFFSTAGE = "[inert], [hidden]";

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

//#region 🔖️Show
/** 🎚️ Everything the host may change while a show runs. */
type Direction = Pick<PetLayerProps, "scene" | "mode" | "quiet" | "capacity" | "surfaces" | "keepouts" | "glances" | "scale" | "zIndex" | "onCast">;

/** 🎬️ A running show: `direct` hands it the host's current wishes, `stop` ends it and leaves nothing behind. */
type Show = { readonly direct: (direction: Direction) => void; readonly stop: () => void };

/** 🟰️ Whether two surveys say the same; a survey that changed nothing is not worth an event. */
function sameSurvey(one: Surveyed | null, other: Surveyed): boolean {
  if (one === null || one.width !== other.width || one.height !== other.height || one.surfaces.length !== other.surfaces.length || one.keepouts.length !== other.keepouts.length) return false;
  for (const [index, surface] of other.surfaces.entries()) {
    const known = one.surfaces[index]!;
    if (known.id !== surface.id || known.x0 !== surface.x0 || known.x1 !== surface.x1 || known.y !== surface.y) return false;
  }
  for (const [index, box] of other.keepouts.entries()) {
    const known = one.keepouts[index]!;
    if (known.x !== box.x || known.y !== box.y || known.width !== box.width || known.height !== box.height) return false;
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
  };
}

/** 📍️ Whether two lists name the same points in the same order. */
function samePoints(one: readonly Point[], other: readonly Point[]): boolean {
  return one.length === other.length && other.every((point, index) => one[index]!.x === point.x && one[index]!.y === point.y);
}

/** 🎪️ Starts a show of `menagerie` in the layer element `host`: opens a stage seeded with `seed`, watches the page, the
 * pointer and the visibility of the document, and paints one depiction per actor of every frame into `host`.
 *
 * A step happens inside an animation frame and is the only place the stage advances. It reads first (a survey when the
 * last one went stale: at once when the page moves or nothing runs, otherwise at most every eight ticks; while actors
 * travel the page is measured every eight ticks anyway, while they only stand once a second), folds the ticks that
 * passed and then every waiting event into the stage, and writes last (the frame). The stage lives in the pets' own
 * units — pixels ÷ the size they are drawn at ({@link staged}) —, so surveys, the pointer and glances are divided by
 * that size on the way in and the feet of every actor multiplied by it on the way out. A still stage hears of the
 * pointer only while it rests on or beside a pet (which turns see-through); every other move leaves it asleep. The
 * pacer hands out at most eight ticks while the stage runs and every tick slept after a rest, which the stage jumps
 * over; its clock runs `tempo` times as fast as the wall clock (held between an eighth and eight: beyond eight ticks
 * a frame time would be dropped). The rotation of the cast advances every 60 to 120 seconds of stage time, never
 * while the stage is `still` or quiet. The show rests — no frame, no timer, no survey, the pets where they are —
 * while the document is hidden and while the layer itself is inert or hidden (a host that opens a modal dialog makes
 * everything else inert: the pets wait behind it instead of falling off surfaces that seem to have vanished).
 */
function startShow(host: HTMLElement, menagerie: Menagerie, seed: number, tempo: number, first: Direction): Show {
  const page = host.ownerDocument;
  const view = page.defaultView;
  if (view === null) return { direct: () => {}, stop: () => {} };
  const haste = Math.min(Math.max(tempo, 1 / FRAME_TICKS), FRAME_TICKS);
  const kinds = new Map<Slug, Species>(menagerie.species.map((species) => [species.id, species]));
  const depictions = new Map<Slug, Depiction>();
  let direction = first;
  let stage: Stage = advance(menagerie, openStage(seed), [
    { kind: "tuned", mode: first.mode },
    { kind: "hushed", quiet: first.quiet === true },
  ]);
  let frame: Frame | null = null;
  let waiting: StageEvent[] = [];
  let stale: "now" | "soon" | null = "now";
  let surveyed: Surveyed | null = null;
  let surveyedSize = 1;
  let surveyedAt = 0;
  let touched = false;
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
  let ended = false;

  const capacity = (): number => direction.capacity ?? petCapacity(width);
  const scale = (): number => direction.scale ?? petScale(width);

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
    const seen = survey(page, { surfaces: direction.surfaces, keepouts: direction.keepouts, frame: host });
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

  const show = (next: Frame): void => {
    const size = scale();
    const drawn = new Set<Slug>();
    let before: Element | null = null;
    for (const actor of next.actors) {
      const kind = kinds.get(actor.species);
      if (kind === undefined) continue;
      let depiction = depictions.get(actor.species);
      if (depiction === undefined) {
        depiction = depict(kind, page);
        depictions.set(actor.species, depiction);
      }
      paint(depiction, size === 1 ? actor : { ...actor, x: actor.x * size, y: actor.y * size }, size);
      const place: Element | null = before === null ? host.firstElementChild : before.nextElementSibling;
      if (depiction.element !== place) host.insertBefore(depiction.element, place);
      before = depiction.element;
      drawn.add(actor.species);
    }
    for (const [id, depiction] of depictions) {
      if (drawn.has(id)) continue;
      depiction.element.remove();
      depictions.delete(id);
    }
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
    const tick = stage.tick + ticks;
    const rate = frame?.rate ?? 0;
    const since = tick - surveyedAt;
    if (stale === "now" || (stale === "soon" && (rate === 0 || since >= SURVEY_TICKS)) || (rate === 64 && since >= SURVEY_TICKS) || (rate > 0 && since >= PATROL_TICKS)) events.push(...measure(tick));
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
    return (
      frame?.actors.some((actor) => {
        const kind = kinds.get(actor.species);
        if (kind === undefined) return false;
        const reach = (kind.size.width * size) / 2;
        return x >= actor.x * size - reach && x <= actor.x * size + reach && y <= actor.y * size && y >= (actor.y - kind.size.height) * size;
      }) ?? false
    );
  };

  const near = (x: number, y: number): boolean => {
    const size = scale();
    return (
      frame?.actors.some((actor) => {
        const kind = kinds.get(actor.species);
        if (kind === undefined) return false;
        const reach = (kind.size.width * size) / 2 + POINTER_REACH;
        return x >= actor.x * size - reach && x <= actor.x * size + reach && y <= actor.y * size + POINTER_REACH && y >= (actor.y - kind.size.height) * size - POINTER_REACH;
      }) ?? false
    );
  };

  const point = (event: Pointed | Unpointed | Poked): boolean => {
    const size = scale();
    const staged = event.kind === "unpointed" ? event : { kind: event.kind, x: event.x / size, y: event.y / size };
    if (direction.mode === "still") {
      const touches = event.kind !== "unpointed" && near(event.x, event.y);
      if (event.kind === "poked" || (!touches && !touched)) return false;
      touched = touches;
    }
    if (event.kind !== "poked") waiting = waiting.filter((known) => known.kind !== "pointed" && known.kind !== "unpointed");
    waiting.push(staged);
    return true;
  };

  const rest = (hidden = false): void => {
    if (hidden || documentHidden(view) || host.closest(OFFSTAGE) !== null) pacer.hide();
    else pacer.show();
  };

  const unwatch = [
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
        if (point(event) && frame !== null && frame.actors.length > 0) pacer.wake();
      },
      { origin: () => origin, hit },
    ),
    watchVisibility(view, rest),
  ];

  function stop(): void {
    if (ended) return;
    ended = true;
    pacer.stop();
    for (const end of unwatch) end();
    for (const depiction of depictions.values()) depiction.element.remove();
    depictions.clear();
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
      const resurvey = next.surfaces !== before.surfaces || next.keepouts !== before.keepouts;
      if (next.mode !== before.mode) {
        if (next.mode === "still") waiting = waiting.filter((known) => known.kind !== "pointed" && known.kind !== "unpointed" && known.kind !== "poked");
        waiting.push({ kind: "tuned", mode: next.mode });
      }
      if (hushed) waiting.push({ kind: "hushed", quiet: next.quiet === true });
      if (next.mode !== before.mode || hushed) rotateAt = null;
      if (next.mode !== before.mode) touched = false;
      if (resurvey || next.scale !== before.scale) stale = "now";
      else if ((next.glances === undefined) !== (before.glances === undefined) && stale === null) stale = "soon";
      if (next.scale !== before.scale || next.onCast !== before.onCast) redraw = true;
      if (next.mode !== before.mode || hushed || stale !== null || redraw || next.scene !== before.scene || next.capacity !== before.capacity) pacer.wake();
    },
    stop,
  };
}
//#endregion 🔖️Show

//#region 🔖️Layer
/** 🎛️ What a host tells the pet layer: the menagerie; the scene whose cast is on stage (an unknown scene falls back to
 * `home`, then to nothing); the liveliness (`off` is expressed by not rendering the layer); whether a time of
 * concentration runs; the most actors at once (by viewport width when absent: below 768 px 2, below 1024 px 4, else 6);
 * the selector of the elements whose top edge carries pets (`[data-pet-surface]`) and of what pets must not cover
 * (interactive controls, text blocks, `[data-pet-keepout]`); other things worth a look, in viewport pixels, sampled with
 * every survey; the size pets are drawn at (1; 0.8 below 768 px); the seed of the stage (a random one per mount); the
 * stacking order (35); who wants to know which species are on stage right now (`onCast`: called, in the order of
 * the menagerie, whenever that set changes — also with nobody when the show ends — and once for a new listener while
 * somebody is on stage); and how fast the pets' time passes (`tempo`: 1 is the wall clock, less is slow motion, more —
 * up to eight — is for whoever has to see in seconds what takes minutes, such as a test of an encounter; a new tempo
 * begins a new show). */
export interface PetLayerProps {
  readonly menagerie: Menagerie;
  readonly scene: string;
  readonly mode: PetMode;
  readonly quiet?: boolean;
  readonly capacity?: number;
  readonly surfaces?: string;
  readonly keepouts?: string;
  readonly glances?: () => readonly Point[];
  readonly scale?: number;
  readonly seed?: number;
  readonly zIndex?: number;
  readonly onCast?: (species: readonly Slug[]) => void;
  readonly tempo?: number;
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
 * pointer and at each other, and never gets in the way. Renders one static element; everything else happens outside
 * React. Under forced colours, and for a menagerie with issues ({@link menagerieIssues}), the layer stays empty and
 * runs nothing. */
export function PetLayer(props: PetLayerProps): ReactElement {
  const { menagerie, scene, mode, quiet, capacity, surfaces, keepouts, glances, scale, seed, zIndex, onCast, tempo = 1 } = props;
  const host = useRef<HTMLDivElement>(null);
  const show = useRef<Show | null>(null);
  const forced = useSyncExternalStore(watchForcedColors, forcedColors, () => false);
  const sound = useMemo(() => menagerieIssues(menagerie).length === 0, [menagerie]);
  useEffect(() => {
    if (forced || !sound || host.current === null) return;
    const started = startShow(host.current, menagerie, seed ?? Math.floor(Math.random() * 0x1_0000_0000), tempo, { scene, mode, quiet, capacity, surfaces, keepouts, glances, scale, zIndex, onCast });
    show.current = started;
    return () => {
      started.stop();
      show.current = null;
    };
  }, [menagerie, sound, seed, tempo, forced]);
  useEffect(() => {
    show.current?.direct({ scene, mode, quiet, capacity, surfaces, keepouts, glances, scale, zIndex, onCast });
  }, [scene, mode, quiet, capacity, surfaces, keepouts, glances, scale, zIndex, onCast]);
  return <div ref={host} className="pet-layer" aria-hidden="true" />;
}
//#endregion 🔖️Layer
