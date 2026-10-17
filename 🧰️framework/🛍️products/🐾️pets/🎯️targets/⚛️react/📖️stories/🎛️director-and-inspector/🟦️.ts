/** 🎛️ The director and inspector of the stories sandbox: the forced states, tricks and moods folded into the stage, the inspector that reads every frame and the synthetic pointer that carries a pet. */
import { useEffect, useState } from "react";
import { TICKS_PER_SECOND, clipTicks, overlaps, type Activity, type ActorFrame, type Frame, type Menagerie, type Mood, type Point, type Slug, type Stage, type StageEvent } from "@semio-tech/pets";
import { PURR_TICKS, hail, shrug } from "@semio-tech/pets/modules/attention";
import { clipAt, clipOf, draftOf, enter, indexOf, perform, purr, release, sealed, settle, shift, trickOf, type Draft } from "@semio-tech/pets/modules/draft";
import "../🎨️.css";

//#region 🔖️Director
/** 🎙️ What the dev server hands the gallery (`pets-stories:director`): the two functions of the core the pet layer calls while it is served there. The gallery may wrap them; whatever they hold is what the layer runs. */
export type Director = { advance: (menagerie: Menagerie, stage: Stage, events: readonly StageEvent[]) => Stage; frameOf: (menagerie: Menagerie, stage: Stage) => Frame };

/** 🧲️ A change the inspector folds into the stage between two frames, for one species on stage: enter a state, perform a trick, feel a mood at an intensity, or take up an activity. */
type Force = { readonly kind: "state"; readonly species: Slug; readonly state: Slug } | { readonly kind: "trick"; readonly species: Slug; readonly trick: Slug } | { readonly kind: "mood"; readonly species: Slug; readonly mood: Mood; readonly intensity: number } | { readonly kind: "activity"; readonly species: Slug; readonly activity: Activity };

/** 🧾️ What the inspector last read off the layer's frames: the tick, the rate the stage asks for and the frames it drew in the last second; the actors as drawn and how many particles, dust clouds, standing ladders and lifted fixtures there are, who is held; how many pets vanished in a puff, which bodies overlap now (there must never be any), in how many of the frames drawn since the show began any did; and the last force and whether the stage took it. */
export type Reading = {
  readonly tick: number;
  readonly rate: number;
  readonly steps: number;
  readonly actors: readonly ActorFrame[];
  readonly particles: number;
  readonly puffs: number;
  readonly ladders: number;
  readonly lifts: number;
  readonly held: Slug | null;
  readonly poofs: number;
  readonly overlaps: readonly string[];
  readonly overlapped: number;
  readonly frames: number;
  readonly news: { readonly force: Force; readonly done: boolean; readonly tick: number } | null;
};

/** 🔬️ The gallery's view into the layer's stage: the latest reading (published at most five times a second), who wants to hear of a new one, and forces to fold in at the next step. */
export type Inspector = { readonly read: () => Reading | null; readonly listen: (listener: () => void) => () => void; readonly force: (force: Force) => void };

/** 🧍️ The activities a pet on a perch can be set to on its own, without a partner, a motion or gear. */
export const STANDING = ["idle", "fidget", "sleep", "greet", "trick", "purr", "dizzy", "shrug", "push"] as const satisfies readonly Activity[];
const HOLD_TICKS = 2 * TICKS_PER_SECOND;
const READ_MILLISECONDS = 200;
export const NO_READING = (): Reading | null => null;
export const NO_LISTENING = (): (() => void) => () => {};

/** 🎯️ An actor on a perch takes up `activity` at tick `now` by the stage's own rules: it lets go of its partner; idle comes to rest, greet greets, purr purrs, shrug shrugs, trick performs its first trick; every other activity plays a clip of it once (or for two seconds when the clip loops), standing where it is. */
function actOut(draft: Draft, index: number, activity: Activity, now: number): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  release(draft, index, now);
  if (activity === "idle") return settle(draft, index, now);
  if (activity === "greet") return hail(draft, index, body.x + body.facing, now);
  if (activity === "purr") return purr(draft, index, PURR_TICKS, now);
  if (activity === "shrug") return shrug(draft, index, body.x + body.facing, now);
  if (activity === "trick") {
    const trick = kind.tricks[0];
    if (trick !== undefined) perform(draft, index, trick, now);
    return;
  }
  shift(draft, index, activity, now);
  body.clip = clipAt(kind, activity, 0);
  const clip = clipOf(kind, body.clip);
  body.until = now + (clip !== null && !clip.loop ? clipTicks(clip) : HOLD_TICKS);
  body.goal = body.x;
  body.vx = 0;
  body.vy = 0;
}

/** 🪛️ The stage after `force`, folded in by the stage's own rules on a draft of it: a state is entered at once (its emitters follow), a mood is felt from now on and decays as moods do; a trick or an activity only for a pet that stands on a perch (`done` is false otherwise, and for a species not on stage or a trick it does not have). */
function forced(menagerie: Menagerie, stage: Stage, force: Force): { readonly stage: Stage; readonly done: boolean } {
  const draft = draftOf(menagerie, stage);
  const index = indexOf(draft.actors, force.species);
  if (index < 0) return { stage, done: false };
  const body = draft.actors[index]!;
  const now = draft.tick;
  const grounded = body.footing === "perch" && body.perch !== null && !body.leaving;
  if (force.kind === "state") enter(draft, index, force.state, now, now);
  else if (force.kind === "mood") body.feeling = { mood: force.mood, intensity: force.intensity, since: now };
  else if (!grounded) return { stage, done: false };
  else if (force.kind === "trick") {
    const trick = trickOf(draft.kinds[index]!, force.trick);
    if (trick === null) return { stage, done: false };
    release(draft, index, now);
    perform(draft, index, trick, now);
  } else actOut(draft, index, force.activity, now);
  return { stage: sealed(draft), done: true };
}

/** 🔭️ Opens the inspector on `director`: its `advance` folds the forces waiting into the stage after every step, its `frameOf` reads every frame — counting the frames since the show began (a stage of another seed or an earlier tick begins a new count) and those in which bodies overlap by the stage's own invariant (`overlaps`) —; `close` gives the director its functions back. */
function openInspector(director: Director): { readonly inspector: Inspector; readonly close: () => void } {
  const { advance, frameOf } = director;
  const listeners = new Set<() => void>();
  const stepped: number[] = [];
  let waiting: Force[] = [];
  let latest: Reading | null = null;
  let published: Reading | null = null;
  let news: Reading["news"] = null;
  let seed = Number.NaN;
  let last = -1;
  let frames = 0;
  let overlapped = 0;
  let timer: number | null = null;
  const publish = (): void => {
    timer = null;
    published = latest;
    for (const listener of listeners) listener();
  };
  director.advance = (menagerie, stage, events) => {
    let next = advance(menagerie, stage, events);
    for (const force of waiting) {
      const result = forced(menagerie, next, force);
      next = result.stage;
      news = { force, done: result.done, tick: next.tick };
    }
    waiting = [];
    return next;
  };
  director.frameOf = (menagerie, stage) => {
    const frame = frameOf(menagerie, stage);
    if (stage.seed !== seed || stage.tick < last) {
      seed = stage.seed;
      frames = 0;
      overlapped = 0;
      stepped.length = 0;
    }
    last = stage.tick;
    const now = performance.now();
    stepped.push(now);
    while (stepped.length > 0 && stepped[0]! < now - 1000) stepped.shift();
    const pairs = overlaps(frame.actors.map((actor) => ({ owner: actor.species, extent: { x0: actor.body.x, y0: actor.body.y, x1: actor.body.x + actor.body.width, y1: actor.body.y + actor.body.height } })));
    frames += 1;
    if (pairs.length > 0) overlapped += 1;
    latest = { tick: frame.tick, rate: frame.rate, steps: stepped.length, actors: frame.actors, particles: frame.particles.length, puffs: frame.puffs.length, ladders: frame.ladders.length, lifts: frame.lifts.length, held: frame.held, poofs: stage.poofs, overlaps: pairs.map((pair) => `${pair.first}+${pair.second}`), overlapped, frames, news };
    if (timer === null) timer = window.setTimeout(publish, READ_MILLISECONDS);
    return frame;
  };
  return {
    inspector: {
      read: () => published,
      listen: (listener) => {
        listeners.add(listener);
        return () => listeners.delete(listener);
      },
      force: (force) => {
        waiting = [...waiting, force];
      },
    },
    close: () => {
      director.advance = advance;
      director.frameOf = frameOf;
      if (timer !== null) window.clearTimeout(timer);
      listeners.clear();
    },
  };
}

/** 🪝️ The inspector of `director` while the component lives (none without a director: the page was not served by the gallery's dev server). */
export function useInspector(director: Director | null): Inspector | null {
  const [inspector, setInspector] = useState<Inspector | null>(null);
  useEffect(() => {
    if (director === null) return undefined;
    const opened = openInspector(director);
    setInspector(opened.inspector);
    return () => {
      opened.close();
      setInspector(null);
    };
  }, [director]);
  return inspector;
}

const SYNTHETIC_POINTER = 47;

/** ✋️ Plays one event of a pointer of the gallery's own (a mouse with its own id) at `point` in the viewport, on whatever lies there: the learner's hand of the layer hears it at the window like a real one. */
function pointerAt(type: "pointerdown" | "pointermove" | "pointerup", point: Point, buttons: number): void {
  const target = document.elementFromPoint(point.x, point.y) ?? document.body;
  target.dispatchEvent(new PointerEvent(type, { bubbles: true, cancelable: true, composed: true, clientX: point.x, clientY: point.y, pointerId: SYNTHETIC_POINTER, pointerType: "mouse", isPrimary: true, button: type === "pointermove" ? -1 : 0, buttons }));
}

/** ⏭️ The next animation frame. */
function nextFrame(): Promise<void> {
  return new Promise((done) => requestAnimationFrame(() => done()));
}

/** 🧺️ Presses at `from`, carries the press to `to` over `steps` frames, holds it there for `hold` frames and lets go — the way a learner picks a pet up, carries it and drops it. */
export async function carry(from: Point, to: Point, steps: number, hold: number): Promise<void> {
  pointerAt("pointerdown", from, 1);
  for (let step = 1; step <= steps; step++) {
    await nextFrame();
    pointerAt("pointermove", { x: from.x + ((to.x - from.x) * step) / steps, y: from.y + ((to.y - from.y) * step) / steps }, 1);
  }
  for (let rest = 0; rest < hold; rest++) {
    await nextFrame();
    pointerAt("pointermove", to, 1);
  }
  await nextFrame();
  pointerAt("pointerup", to, 0);
}
//#endregion 🔖️Director
