/** 🧰️ The shared kit of the pet specs: the ensemble as authored, the sizes, labels, timings and tolerances the specs agree on, the page instrumentation that watches the pets move, and the checks every spec reuses.
 *
 * @see ../🟦️.ts — the home screen and hand specs, which describe what these specs expect of the pets
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { Page } from "@playwright/test";
import { enter, expect, pane, type Device, type Locale, type Text } from "../../../🎭️e2e/🚶️learner/🟦️.ts";

/** 🎟️ A cast as the ensemble authors it. */
interface Cast {
  readonly scene: string;
  readonly core: readonly string[];
  readonly rotation: readonly string[];
}

/** 🧬️ What the spec needs of a species as authored: its names, its box, how it gets around, its gear and what its states and tricks are. */
interface Kind {
  readonly id: string;
  readonly name: Text;
  readonly size: { readonly width: number; readonly height: number };
  readonly locomotion: { readonly gait: string; readonly hover?: number };
  readonly gear: readonly string[];
  readonly canopy?: { readonly kind: string; readonly d?: string };
  readonly states: readonly { readonly id: string }[];
  readonly tricks: readonly { readonly id: string; readonly cues: readonly string[]; readonly from?: readonly string[]; readonly to?: string }[];
}

const MENAGERIE = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../🐾️pets");
const ENSEMBLE = JSON.parse(readFileSync(resolve(MENAGERIE, "🔣️.json"), "utf8")) as { readonly casts: readonly Cast[]; readonly species: readonly string[] };
const CASTS = ENSEMBLE.casts;
const SPECIES = ENSEMBLE.species.map((path) => JSON.parse(readFileSync(resolve(MENAGERIE, path), "utf8")) as Kind);
export const KINDS: Readonly<Record<string, Kind>> = Object.fromEntries(SPECIES.map((kind) => [kind.id, kind]));

/** 📐️ The box of every species, as the page's instruments take it. */
const BOXES: Readonly<Record<string, { readonly width: number; readonly height: number }>> = Object.fromEntries(SPECIES.map((kind) => [kind.id, kind.size]));

/** 🎈️ How high a species floats above what it stands on. */
export function hoverOf(id: string): number {
  const kind = KINDS[id];
  return kind?.locomotion.gait === "float" ? (kind.locomotion.hover ?? 0) : 0;
}

/** 🃏️ The parts of a card of the screen in front: its tab and its body. */
export const CARD_PARTS = '#course-main [data-card] [data-slot="window-chrome-chip-cap"], #course-main [data-card] [data-slot="window-chrome-body-surface"]';

/** 🧱️ What pets stand on in the quiz, as the client names it for its pet layer: the parts of every card, and the
 * footer line. */
export const EDGES = `${CARD_PARTS}, .course-app > footer`;

/** 🕹️ What a press on a pet never takes from the learner in the quiz: the controls of the layer (links, buttons, form
 * fields, labels, ARIA widgets, focusable elements) and what the quiz adds (the grip of an item, the zones an item is
 * dropped on, the cards of the overview). A pointer over one of them is over a control, where circling is not followed. */
export const CONTROLS = 'a[href], button, input, select, textarea, summary, label, [role="button"], [role="link"], [role="checkbox"], [role="radio"], [role="tab"], [role="menuitem"], [role="option"], [role="slider"], [role="switch"], [tabindex]:not([tabindex="-1"]), [contenteditable]:not([contenteditable="false"]), [data-course-grip], [data-course-drop], [data-layered-card]';

/** 🏡️ The scene of every screen that belongs to no quiz. */
export const HOME = "home";

/** 🎚️ What a learner may choose for the pets, in the order the preferences offer it. */
export const CHOICES = ["off", "still", "calm", "lively"] as const;

/** 🏷️ The name of the pets' choice in the preferences (`course.preferences.pets`). */
export const PETS_LABEL: Text = { en: "Pets", de: "Tierchen" };

/** 🔘️ The name of the switch on the footer of every screen (`course.preferences.petsShown`). */
export const SWITCH_LABEL: Text = { en: "Show pets", de: "Tierchen anzeigen" };

/** 🪧️ How the preferences begin the line that names the pets on stage (`course.preferences.petsCast`). */
export const CAST_LINE: Text = { en: "Here right now: ", de: "Gerade hier: " };

/** 🐢️ The sentence the preferences used to show for reduced motion. They no longer do: the device does not choose for the learner. */
export const REDUCED_NOTE: Text = { en: "Your device asks for less motion, so the pets stay still until you choose.", de: "Dein Gerät bittet um weniger Bewegung, deshalb bleiben die Tierchen reglos, bis du selbst wählst." };

/** 🔲️ What the preferences say to a learner whose device forces its own colours (`course.preferences.petsForced`). */
export const FORCED_NOTE: Text = { en: "Your device uses its own contrast colours, so no pets show.", de: "Dein Gerät verwendet eigene Kontrastfarben, deshalb erscheinen keine Tierchen." };

/** 🤹️ The name of the group of the settings that plays with the pets on stage (`course.preferences.petsPlayWith`). */
export const PLAY_LABEL: Text = { en: "Play with the pets", de: "Mit den Tierchen spielen" };

/** 🎾️ The settings' deeds, their bilingual button names and their polite request announcements. */
export const DEEDS = {
  hello: { button: { en: "Hello", de: "Hallo" }, said: { en: "{{name}} says hello.", de: "{{name}} sagt Hallo." } },
  trick: { button: { en: "Trick", de: "Kunststück" }, said: { en: "{{name}} does a trick.", de: "{{name}} macht ein Kunststück." } },
  pet: { button: { en: "Pet", de: "Streicheln" }, said: { en: "{{name}} is petted.", de: "{{name}} wird gestreichelt." } },
  toss: { button: { en: "Toss", de: "Hochwerfen" }, said: { en: "{{name}} is tossed up.", de: "{{name}} wird hochgeworfen." } },
  circle: { button: { en: "Circle clockwise", de: "Im Uhrzeigersinn kreisen" }, said: { en: "You circle {{name}} clockwise.", de: "Du umkreist {{name}} im Uhrzeigersinn." } },
  countercircle: { button: { en: "Circle counterclockwise", de: "Gegen den Uhrzeigersinn kreisen" }, said: { en: "You circle {{name}} counterclockwise.", de: "Du umkreist {{name}} gegen den Uhrzeigersinn." } },
  stroke: { button: { en: "Stroke", de: "Streichen" }, said: { en: "You stroke {{name}}.", de: "Du streichst über {{name}}." } },
  shake: { button: { en: "Shake", de: "Schütteln" }, said: { en: "You shake {{name}}.", de: "Du schüttelst {{name}}." } },
} as const satisfies Readonly<Record<string, { readonly button: Text; readonly said: Text }>>;

/** ⏩️ The client's test seam: the attribute of the document root that makes the pets' time pass faster. */
export const TEMPO = "data-pets-tempo";

/** 🐎️ How many times faster than the wall clock the pets' time passes where a spec waits for minutes of it. */
export const HASTE = 8;

/** 🐌️ Keeps the lifted copy out long enough for the hand to reach it; the cooldown and physical approach run at wall-clock speed. */
export const MISCHIEF_HASTE = 1;

/** 🔥️ The quiz whose page the mischief spec idles on: in its cast two climbers (radiatory of its core, insuly of its rotation) have a task of its page among their grounds and reach a wall beside its rows — the pets of physics that cover its tasks float or only own a parachute. */
export const MISCHIEF_QUIZ = "heating";

/** 🖐️ The attribute of the document element on which the hand shows its cursor: `grab` over a pet it can pick up, `grabbing` while it holds one. */
export const CURSOR = "data-pet-cursor";

/** 🌅️ How long the first pet may take to show once the home screen is there. On an idle machine it is there within two
 * seconds; the gate runs four browsers against one cold dev server, which first has to transform the lazy chunk. */
export const APPEAR_MS = 30_000;

/** 🚶️ How long a change of cast may take: the pets that leave walk off and fade, the new ones fade in. */
export const RECAST_MS = 45_000;

/** 💬️ How long a pet may take to answer the hand or a deed: a frame or two, and a trick of at most three seconds. */
export const ANSWER_MS = 6_000;

/** 🪂️ How long a pet let go high above the page may take to land: a parachute sinks at 20 to 45 px a second. */
export const LANDING_MS = 60_000;

/** 🧮️ How many animation frames a stillness is watched for: a second and a half on an idle machine. Frames, not
 * milliseconds: what must not happen is a write into the layer, and a frame is the only time one can happen. */
export const STILL_FRAMES = 90;

/** 🕰️ How long a walk may be waited for at eight times the pace. */
export const WALK_MS = 120_000;

/** ⚡️ The lively minute in which no two pets may ever overlap, in milliseconds of the wall clock. */
export const LIVELY_MS = 60_000;

/** 🤏️ How far, in pixels of the screen, two bodies may reach into each other before they overlap: the rounding of a
 * drawing's place to two decimals. */
export const OVERLAP_SLACK = 0.5;

/** 🧗️ How high above the edge it falls on a pet is let go of in the parachute spec, at most, in pixels. */
export const DROP_HEIGHT = 420;

/** 📏️ How tall, at least, the body of the card a pet falls in front of must be, in pixels: the overview at 1440 × 900
 * fills its cells with cards whose bodies are 34 to 84 px tall, and a pet let go at rest `RELEASE_DEPTH` under the top
 * of the tallest keeps the middle of its body in front of it for some ten frames before it has fallen past. A body
 * laid out at that least height can measure a fraction of a pixel under it. */
export const FRONT_LEAST = 56;

/** 🪶️ How far under the top of that card's body the hand lets the pet go, in pixels: it hangs from its scruff, so the
 * middle of its body is a little lower still. */
export const RELEASE_DEPTH = 6;

/** 🍩️ How far beyond half the larger side of its box the pointer circles a pet, in pixels: inside the band in which
 * the stage follows a circle (from 8 px beyond half the larger side to 3.4 sides from the middle). */
const CIRCLE_BEYOND = 14;

/** 🧭️ How many corners one lap of a circle has: the pointer jumps from corner to corner, an eighth of a turn at a time. */
export const CIRCLE_POINTS = 8;

/** ⏲️ How long one lap of a circle takes by the page's clock, in milliseconds: a quarter turn in a quarter of a second,
 * the pace of a hand — between the scribble (four quarter turns a second) and the stall (a quarter turn in more than
 * 0.625 s) the stage refuses as a circle —, held by the clock the stage keeps time by, however long a move takes. */
export const CIRCLE_LAP_MS = 1000;

/** 🌀️ How many laps the spec draws round a pet: five quarter turns make a circle, and the first lap begins anew while the pointer comes in. */
export const CIRCLE_LAPS = 3;

/** 🪝️ How many of its own heights the rope of a grappling gun reaches at most (`ROPE_LONG` of the pets' climbing): on the
 * home screen at 1440 × 900 those are the tops of the bottom row of cards over the footer line, which pets of every
 * kind like to sit on. */
export const ROPE_REACH = 8;

/** 🔁️ How long the gear spec waits for a rope while the hand holds whoever sat on the perch in a rope's reach, before it
 * lets go and clears the way again, in milliseconds of the wall clock: two minutes of the pets' time at eight times the
 * pace, in which a grappler on the footer line feels like going up a time or two — and in which a newcomer of the
 * cast's rotation or a pet with a ladder may take the perch first. */
export const ROPE_ROUND_MS = 15_000;

/** ⌛️ How long the gear spec tries for a rope in all, round after round. */
export const ROPE_MS = 360_000;

/** 🧩️ What the page's instruments say of one pet: what its depiction says it does, what carries it, the state it shows,
 * how visible it is, and the box of its body on the screen. */
export interface Body {
  readonly id: string;
  readonly activity: string;
  readonly footing: string;
  readonly state: string;
  readonly opacity: number;
  readonly left: number;
  readonly top: number;
  readonly right: number;
  readonly bottom: number;
}

/** 🚨️ Two pets whose bodies reached into each other on one frame, as the instruments saw them. */
interface Overlap {
  readonly frame: number;
  readonly pair: readonly [Body, Body];
}

/** 📈️ What the instruments saw while they watched for overlaps: how many frames, the most pets on one frame, and the first overlaps. */
interface Watched {
  readonly frames: number;
  readonly most: number;
  readonly overlaps: readonly Overlap[];
}

/** 🔭️ The instruments the spec installs in a page (`window.petWalk`). */
export interface Instruments {
  readonly bodies: () => Body[];
  readonly trails: Record<string, string[]>;
  readonly overlaps: (milliseconds: number, slack: number) => Promise<Watched>;
}

/** 🔩️ In the page: installs the spec's instruments on `window.petWalk`, once per pet layer — `bodies()` (every pet as
 * its depiction shows it, with the box of its body: the size box of its species mapped through the drawing's own
 * transform onto the screen, the axis-aligned box of its four corners), `trails` (per pet, every change of
 * `activity/footing/state` since the instruments were installed, recorded by a mutation observer, so nothing between two
 * polls is lost) and `overlaps(milliseconds, slack)` (every animation frame for that long, every two visible pets whose
 * bodies reach more than `slack` pixels into each other both ways). */
function instrument(boxes: Readonly<Record<string, { readonly width: number; readonly height: number }>>): void {
  const host = window as unknown as { petWalk?: Instruments; petWalkLayer?: Element };
  const layer = document.querySelector(".pet-layer");
  if (layer === null || host.petWalkLayer === layer) return;
  const bodies = (): Body[] =>
    [...document.querySelectorAll<SVGSVGElement>(".pet-layer svg.pet")].flatMap((pet) => {
      const id = pet.getAttribute("data-pet") ?? "";
      const box = boxes[id];
      const place = pet.getScreenCTM();
      if (box === undefined || place === null) return [];
      const corners = [[-box.width / 2, -box.height], [box.width / 2, -box.height], [box.width / 2, 0], [-box.width / 2, 0]] as const;
      const xs = corners.map(([x, y]) => place.a * x + place.c * y + place.e);
      const ys = corners.map(([x, y]) => place.b * x + place.d * y + place.f);
      return [{ id, activity: pet.getAttribute("data-pet-activity") ?? "", footing: pet.getAttribute("data-pet-footing") ?? "", state: pet.getAttribute("data-pet-state") ?? "", opacity: Number(pet.style.opacity || "1"), left: Math.min(...xs), top: Math.min(...ys), right: Math.max(...xs), bottom: Math.max(...ys) }];
    });
  const trails: Record<string, string[]> = {};
  const note = (pet: Element): void => {
    const id = pet.getAttribute("data-pet") ?? "";
    const line = `${pet.getAttribute("data-pet-activity") ?? ""}/${pet.getAttribute("data-pet-footing") ?? ""}/${pet.getAttribute("data-pet-state") ?? ""}`;
    const trail = (trails[id] ??= []);
    if (trail.at(-1) !== line) trail.push(line);
  };
  for (const pet of layer.querySelectorAll("svg.pet")) note(pet);
  new MutationObserver((records) => {
    for (const record of records) {
      const target = record.target as Element;
      if (target.matches("svg.pet")) note(target);
      for (const added of record.addedNodes) if (added instanceof Element && added.matches("svg.pet")) note(added);
    }
  }).observe(layer, { subtree: true, childList: true, attributes: true, attributeFilter: ["data-pet-activity", "data-pet-footing", "data-pet-state"] });
  const overlaps = (milliseconds: number, slack: number): Promise<Watched> =>
    new Promise((resolve) => {
      const started = performance.now();
      const found: Overlap[] = [];
      let frames = 0;
      let most = 0;
      const look = (): void => {
        frames += 1;
        const shown = bodies().filter((body) => body.opacity > 0);
        most = Math.max(most, shown.length);
        for (const [index, one] of shown.entries()) {
          for (const other of shown.slice(index + 1)) {
            const across = Math.min(one.right, other.right) - Math.max(one.left, other.left);
            const upright = Math.min(one.bottom, other.bottom) - Math.max(one.top, other.top);
            if (across > slack && upright > slack && found.length < 20) found.push({ frame: frames, pair: [one, other] });
          }
        }
        if (performance.now() - started >= milliseconds) resolve({ frames, most, overlaps: found });
        else window.requestAnimationFrame(look);
      };
      window.requestAnimationFrame(look);
    });
  host.petWalkLayer = layer;
  host.petWalk = { bodies, trails, overlaps };
}

/** 🔌️ Installs the instruments in `page` once its pet layer is there. */
export async function instrumented(page: Page): Promise<void> {
  await expect(page.locator(".pet-layer svg.pet")).not.toHaveCount(0, { timeout: APPEAR_MS });
  await page.evaluate(instrument, BOXES);
}

/** 🩻️ Every pet of `page` as the instruments see it. */
export async function bodies(page: Page): Promise<readonly Body[]> {
  return page.evaluate(() => (window as unknown as { petWalk: Instruments }).petWalk.bodies());
}

/** 🧵️ The changes of what `id` did since the instruments were installed, as `activity/footing/state`. */
export async function trailOf(page: Page, id: string): Promise<readonly string[]> {
  return page.evaluate((species) => [...((window as unknown as { petWalk: Instruments }).petWalk.trails[species] ?? [])], id);
}

/** 🪜️ How many of `wanted` a trail shows in that order, each matched against the start of an entry (`activity/footing/state`). */
export function inOrder(trail: readonly string[], wanted: readonly string[]): number {
  let found = 0;
  for (const line of trail) if (found < wanted.length && line.startsWith(wanted[found]!)) found += 1;
  return found;
}

/** 📍️ The middle of a body on the screen. */
export function middleOf(body: Body): { readonly x: number; readonly y: number } {
  return { x: (body.left + body.right) / 2, y: (body.top + body.bottom) / 2 };
}

/** 🔍️ In the page: for every point, whether a control of the quiz lies under it (the pet layer takes no pointer, so the element under a point is the page's) or it lies outside the window. */
export function overControls(question: { readonly points: readonly { readonly x: number; readonly y: number }[]; readonly controls: string }): boolean[] {
  return question.points.map((point) => point.x < 1 || point.y < 1 || point.x > window.innerWidth - 1 || point.y > window.innerHeight - 1 || document.elementFromPoint(point.x, point.y)?.closest(question.controls) != null);
}

/** 🙋️ A pet the learner's hand can reach now, among those `wanted` picks: it stands idle on a perch, wholly visible, and no control lies under the middle of its body. Throws while there is none, for `toPass`. */
export async function reachable(page: Page, wanted: (body: Body) => boolean = () => true): Promise<Body> {
  const shown = (await bodies(page)).filter((body) => body.activity === "idle" && body.footing === "perch" && body.opacity > 0.99 && wanted(body));
  const covered = await page.evaluate(overControls, { points: shown.map(middleOf), controls: CONTROLS });
  const free = shown.filter((_, index) => !covered[index]);
  if (free.length === 0) throw new Error(`no pet the hand can reach among ${shown.map((body) => body.id).join(", ") || "nobody"}`);
  return free[0]!;
}

/** 🔄️ The state a circling or shaking hand leaves, as authored: the first eligible trick's `to`, or one rung along its `from` states (all states when absent), up for a clockwise trick and down otherwise; an unanswered cue keeps the state. */
export function circled(kind: Kind, state: string, cue: "circle" | "countercircle" | "shake"): string {
  const trick = kind.tricks.find((candidate) => candidate.cues.includes(cue) && (candidate.from === undefined || candidate.from.includes(state)));
  const states = kind.states.map((entry) => entry.id);
  if (trick === undefined) return state;
  if (trick.to !== undefined) return states.includes(trick.to) ? trick.to : state;
  if (!trick.cues.includes("circle") && !trick.cues.includes("countercircle")) return state;
  const listed = (trick.from ?? []).filter((rung) => states.includes(rung));
  const rungs = listed.length > 0 ? listed : states;
  const index = rungs.indexOf(state);
  const next = index + (trick.cues.includes("circle") ? 1 : -1);
  return index < 0 || next < 0 || next >= rungs.length ? state : rungs[next]!;
}

/** ⭕️ The points of a circle round a body, `laps` times, clockwise on the screen for `circle` and the other way for `countercircle`. */
export function circleRound(body: Body, cue: "circle" | "countercircle", laps: number): { readonly x: number; readonly y: number }[] {
  const middle = middleOf(body);
  const radius = Math.max(body.right - body.left, body.bottom - body.top) / 2 + CIRCLE_BEYOND;
  const way = cue === "circle" ? 1 : -1;
  return Array.from({ length: laps * CIRCLE_POINTS + 1 }, (_, step) => ({ x: middle.x + radius * Math.cos((way * step * 2 * Math.PI) / CIRCLE_POINTS), y: middle.y + radius * Math.sin((way * step * 2 * Math.PI) / CIRCLE_POINTS) }));
}

/** 🏝️ In the page: a place on the footer line with room for a pet and a circle round it — no part of a card and no pet
 * within `reach` pixels to either side and `room` pixels above the line, no control on the line below it — the one
 * nearest the middle of the window, or `null`. */
export function floorSpot(question: { readonly parts: string; readonly controls: string; readonly reach: number; readonly room: number }): { readonly x: number; readonly top: number } | null {
  const footer = document.querySelector(".course-app > footer");
  if (footer === null) return null;
  const top = footer.getBoundingClientRect().top;
  const blocks = [...[...document.querySelectorAll(question.parts)].map((part) => part.getBoundingClientRect()), ...(window as unknown as { petWalk: Instruments }).petWalk.bodies()];
  const spots: number[] = [];
  for (let x = question.reach + 10; x <= window.innerWidth - question.reach - 10; x += 10) {
    if (blocks.some((block) => block.right > x - question.reach && block.left < x + question.reach && block.bottom > top - question.room && block.top < top)) continue;
    const below = Array.from({ length: 9 }, (_, step) => document.elementFromPoint(x - question.reach + (step * question.reach) / 4, top + 8));
    if (below.some((element) => element?.closest(question.controls) != null)) continue;
    spots.push(x);
  }
  if (spots.length === 0) return null;
  const middle = window.innerWidth / 2;
  return { x: spots.reduce((best, x) => (Math.abs(x - middle) < Math.abs(best - middle) ? x : best)), top };
}

/** 🤲️ Picks up the pet whose body is `body` at its middle and carries it to `to` along a straight line, a frame per step, then holds it still there for ten frames, so that its last samples say it does not move; the button stays down. */
export async function carry(page: Page, body: Body, to: { readonly x: number; readonly y: number }): Promise<void> {
  const from = middleOf(body);
  await page.mouse.move(from.x, from.y);
  await expect(page.locator("html")).toHaveAttribute(CURSOR, "grab", { timeout: ANSWER_MS });
  await page.mouse.down();
  for (let step = 1; step <= 24; step++) {
    await page.mouse.move(from.x + ((to.x - from.x) * step) / 24, from.y + ((to.y - from.y) * step) / 24);
    await page.evaluate(framesPass, 1);
  }
  for (let step = 0; step < 10; step++) {
    await page.mouse.move(to.x, to.y);
    await page.evaluate(framesPass, 1);
  }
}

/** ⏱️ How much of the pets' time one look of {@link eventually} lets pass, in milliseconds. */
export const STAGE_LOOK_MS = 250;

/** 🚦️ In the page: waits until the pets' own clock has run `milliseconds` and answers how long it ran. The clock of the pet layer hands out at most eight ticks (125 ms) per animation frame and drops the rest, so on a loaded host the pets' time falls behind the wall clock; summing the frames the way the pacer does measures the time the pets lived, whatever the host did. */
export function stagePasses(wait: { readonly milliseconds: number; readonly attribute: string }): Promise<number> {
  return new Promise((resolve) => {
    const tempo = Math.min(Math.max(Number(document.documentElement.getAttribute(wait.attribute) ?? "1") || 1, 1 / 8), 8);
    let ran = 0;
    let last = performance.now();
    const next = (time: number): void => {
      ran += Math.min(Math.max(0, time - last) * tempo, 125);
      last = Math.max(last, time);
      if (ran >= wait.milliseconds) resolve(ran);
      else window.requestAnimationFrame(next);
    };
    window.requestAnimationFrame(next);
  });
}

/** 👀️ Looks until `done` accepts what `probe` sees, for at most `stageMs` milliseconds of the pets' time ({@link stagePasses}), not of the wall clock: the budget is what an idle host spends in the same wall time, and a loaded host does not shorten it. A wait for something a pet does is declared this way; nothing sleeps. */
export async function eventually<T>(page: Page, probe: () => Promise<T>, done: (seen: T) => boolean, wait: { readonly stageMs: number; readonly message: string }): Promise<T> {
  let seen = await probe();
  for (let ran = 0; !done(seen); ) {
    expect(ran, `${wait.message} (last seen: ${JSON.stringify(seen)})`).toBeLessThan(wait.stageMs);
    ran += await page.evaluate(stagePasses, { milliseconds: STAGE_LOOK_MS, attribute: TEMPO });
    seen = await probe();
  }
  return seen;
}

/** 🛬️ Waits, for `stageMs` milliseconds of the pets' time, until the pet `id` stands on a perch again, wholly visible. */
export async function expectPerched(page: Page, id: string, stageMs: number): Promise<void> {
  await eventually(page, async () => (await bodies(page)).some((body) => body.id === id && body.footing === "perch" && body.opacity > 0.99 && !["hang", "tumble", "fall", "glide"].includes(body.activity)), (perched) => perched, { stageMs, message: `${id} stands on a perch again` });
}

export function castOf(scene: string): Cast {
  const cast = CASTS.find((candidate) => candidate.scene === scene);
  if (cast === undefined) throw new Error(`the ensemble has no cast for ${scene}`);
  return cast;
}

export function layer(device: Device): ReturnType<Device["page"]["locator"]> {
  return device.page.locator(".pet-layer");
}

export function pets(device: Device): ReturnType<Device["page"]["locator"]> {
  return device.page.locator(".pet-layer svg.pet");
}

/** 🐾️ The species on screen, by id. */
export async function shownPets(device: Device): Promise<readonly string[]> {
  return pets(device).evaluateAll((drawn) => drawn.map((pet) => pet.getAttribute("data-pet") ?? ""));
}

/** 🎞️ Everything the last frame wrote: where every pet stands, how visible it is, and every bone, pupil, lid and mouth. */
async function drawing(device: Device): Promise<string> {
  const frames = await pets(device).evaluateAll((drawn) =>
    drawn.map((pet) => [pet.getAttribute("data-pet"), (pet as SVGElement).style.transform, (pet as SVGElement).style.opacity, ...[...pet.querySelectorAll("*")].map((node) => ["transform", "d", "cx", "cy"].map((name) => node.getAttribute(name) ?? "").join(","))].join("|")),
  );
  return frames.join("\n");
}

/** ⏳️ In the page: waits for `frames` animation frames. */
export function framesPass(frames: number): Promise<void> {
  return new Promise((resolve) => {
    let left = frames;
    const next = (): void => {
      left -= 1;
      if (left <= 0) resolve();
      else window.requestAnimationFrame(next);
    };
    window.requestAnimationFrame(next);
  });
}

/** 🥁️ In the page: waits, frame by frame, until the page's clock — the clock the stage keeps time by — reads `deadline` milliseconds; at once when it already does. */
export function paceTo(deadline: number): Promise<void> {
  return new Promise((resolve) => {
    const next = (): void => {
      if (performance.now() >= deadline) resolve();
      else window.requestAnimationFrame(next);
    };
    next();
  });
}

/** 🧊️ In the page: whether pets are on screen and, for `frames` animation frames, nothing is written into their layer —
 * with `places` only, whether every pet keeps its place (it may breathe, blink and look around where it stands). */
export function watchStill(watch: { readonly frames: number; readonly places: boolean }): Promise<boolean> {
  return new Promise((resolve) => {
    const stage = document.querySelector(".pet-layer");
    const drawn = [...document.querySelectorAll<SVGSVGElement>(".pet-layer svg.pet")];
    if (stage === null || drawn.length === 0) return resolve(false);
    const stood = drawn.map((pet) => pet.style.transform);
    let written = false;
    const observer = new MutationObserver(() => {
      written = true;
    });
    if (!watch.places) observer.observe(stage, { subtree: true, attributes: true, childList: true, characterData: true });
    let left = watch.frames;
    const next = (): void => {
      left -= 1;
      const now = [...document.querySelectorAll<SVGSVGElement>(".pet-layer svg.pet")];
      const moved = watch.places ? now.length !== drawn.length || now.some((pet, index) => pet !== drawn[index] || pet.style.transform !== stood[index]) : written || observer.takeRecords().length > 0;
      if (moved || left <= 0) {
        observer.disconnect();
        resolve(!moved);
      } else window.requestAnimationFrame(next);
    };
    window.requestAnimationFrame(next);
  });
}

/** 🗿️ Checks that pets are on screen and do not move at all: frame after frame, nothing is written into their layer. */
export async function expectMotionless(device: Device): Promise<void> {
  await expect(async () => expect(await device.page.evaluate(watchStill, { frames: STILL_FRAMES, places: false }), "nothing is written into the pet layer").toBe(true)).toPass({ timeout: RECAST_MS });
}

/** 🏃️ Checks that the pets on screen are alive: the drawing changes. */
export async function expectMoving(device: Device): Promise<void> {
  const first = await drawing(device);
  await expect.poll(() => drawing(device)).not.toBe(first);
}

/** 👟️ In the page: waits up to `limit` milliseconds for a pet whose feet move at least four pixels across while its
 * depiction says it walks, and answers who walked from where to where, or `null`. */
function watchWalk(limit: number): Promise<{ id: string; from: number; to: number } | null> {
  return new Promise((resolve) => {
    const started = performance.now();
    const setOut = new Map<string, number>();
    const look = (): void => {
      for (const pet of document.querySelectorAll<SVGSVGElement>(".pet-layer svg.pet")) {
        const id = pet.getAttribute("data-pet") ?? "";
        const x = Number(/translate\(([-\d.]+)px/u.exec(pet.style.transform)?.[1]);
        if (pet.getAttribute("data-pet-activity") !== "walk") {
          setOut.delete(id);
          continue;
        }
        const from = setOut.get(id);
        if (from === undefined) setOut.set(id, x);
        else if (Math.abs(x - from) >= 4) return resolve({ id, from, to: x });
      }
      if (performance.now() - started > limit) return resolve(null);
      window.requestAnimationFrame(look);
    };
    look();
  });
}

/** 🥾️ Checks that a pet walks: its feet move at least four pixels across while its depiction says it walks. */
export async function expectWalk(device: Device): Promise<void> {
  const walked = await device.page.evaluate(watchWalk, WALK_MS);
  expect(walked, "a pet whose feet move while it walks").not.toBeNull();
  expect(Math.abs(walked!.to - walked!.from)).toBeGreaterThanOrEqual(4);
}

/** 🎭️ Waits until the pets on screen are those of `scene` — its core and the visitors of its rotation: nobody of
 * another cast is left, nobody is there twice, and — where `present` — somebody of its cast is there. Never the whole
 * cast: how many of it show, and how soon, depends on the room the screen offers and on how busy the machine is. */
export async function expectCast(device: Device, scene: string, present: boolean): Promise<void> {
  const cast = castOf(scene);
  const allowed = [...cast.core, ...cast.rotation];
  await expect
    .poll(
      async () => {
        const shown = await shownPets(device);
        return { strangers: shown.filter((id) => !allowed.includes(id)), absent: present && shown.length === 0, twice: shown.length - new Set(shown).size };
      },
      { timeout: RECAST_MS },
    )
    .toEqual({ strangers: [], absent: false, twice: 0 });
}

/** 🎯️ The controls, cards and pets of the page whose place would hand a click to the pet layer: the element under the
 * middle of each must never belong to it. */
export async function takenTargets(device: Device): Promise<readonly string[]> {
  return device.page.evaluate(() => {
    const under = (x: number, y: number): boolean => x >= 0 && y >= 0 && x < window.innerWidth && y < window.innerHeight && document.elementFromPoint(x, y)?.closest(".pet-layer") != null;
    const taken: string[] = [];
    for (const target of document.querySelectorAll("[data-overview-card], a[href], button, input, select, summary")) {
      if (target.closest("[inert], [hidden]") !== null) continue;
      const box = target.getBoundingClientRect();
      if (box.width > 0 && box.height > 0 && under(box.left + box.width / 2, box.top + box.height / 2)) taken.push(`${target.tagName.toLowerCase()} ${(target.textContent ?? "").trim().slice(0, 40)}`);
    }
    for (const pet of document.querySelectorAll(".pet-layer svg.pet")) {
      const boxes = [...pet.children].map((part) => part.getBoundingClientRect()).filter((box) => box.width > 0 && box.height > 0);
      if (boxes.length === 0) continue;
      const [left, right, top, bottom] = [Math.min(...boxes.map((box) => box.left)), Math.max(...boxes.map((box) => box.right)), Math.min(...boxes.map((box) => box.top)), Math.max(...boxes.map((box) => box.bottom))];
      if (under((left + right) / 2, (top + bottom) / 2)) taken.push(`pet ${pet.getAttribute("data-pet")}`);
    }
    return taken;
  });
}

/** 🚪️ Enters the site anonymously and waits for the first pets on the home screen. */
export async function enterWithPets(device: Device): Promise<void> {
  await enter(device, { kind: "anonymous" });
  await expect(pets(device)).not.toHaveCount(0, { timeout: APPEAR_MS });
}

/** 🧍️ A pet as painted: its feet, the size it is drawn at, how visible it is and what carries it. */
interface Standing {
  readonly id: string;
  readonly x: number;
  readonly y: number;
  readonly size: number;
  readonly opacity: number;
  readonly footing: string;
}

/** 🗺️ Where every pet of a page stands. */
export async function standing(page: Page): Promise<readonly Standing[]> {
  return page.locator(".pet-layer svg.pet").evaluateAll((drawn) =>
    drawn.map((pet) => {
      const place = /translate\(([-\d.]+)px, ([-\d.]+)px\) scale\(([-\d.]+), ([-\d.]+)\)/u.exec((pet as SVGElement).style.transform);
      return { id: pet.getAttribute("data-pet") ?? "", x: Number(place?.[1]), y: Number(place?.[2]), size: Number(place?.[4]), opacity: Number((pet as SVGElement).style.opacity || "1"), footing: pet.getAttribute("data-pet-footing") ?? "" };
    }),
  );
}

/** ✂️ In the page: what one sees of the elements `selector` matches — each box cut down to what its scrolling and
 * clipping ancestors let through, without anything inert, hidden or scrolled away — and whether its top edge shows. */
export function seenBoxes(selector: string): { left: number; top: number; right: number; bottom: number; topped: boolean }[] {
  return [...document.querySelectorAll(selector)].flatMap((element) => {
    if (element.closest("[inert], [hidden], .pet-layer") !== null) return [];
    const box = element.getBoundingClientRect();
    let [left, top, right, bottom] = [Math.max(box.left, 0), Math.max(box.top, 0), Math.min(box.right, window.innerWidth), Math.min(box.bottom, window.innerHeight)];
    for (let node = element.parentElement; node !== null && node !== document.body; node = node.parentElement) {
      const style = getComputedStyle(node);
      if (style.overflowX === "visible" && style.overflowY === "visible") continue;
      const clip = node.getBoundingClientRect();
      [left, top, right, bottom] = [Math.max(left, clip.left), Math.max(top, clip.top), Math.min(right, clip.right), Math.min(bottom, clip.bottom)];
    }
    return right > left && bottom > top ? [{ left, top, right, bottom, topped: top <= box.top }] : [];
  });
}

/** 👣️ What is wrong with where the pets of a page stand that stand on a perch, one line each; none when every such pet
 * that is whole stands with its feet on an edge the screen shows (the top of a tab, of a body or of the footer; a
 * floater its hover above it), no two of them on one edge overlap, and none covers a control or stands in front of a
 * card. Pets in the hand, in the air, on a wall, a ladder, a rope or another pet's head stand on no edge by design. */
export async function misplaced(page: Page): Promise<readonly string[]> {
  const shown = (await standing(page)).filter((pet) => pet.opacity > 0.99 && pet.footing === "perch");
  const edges = (await page.evaluate(seenBoxes, EDGES)).filter((edge) => edge.topped);
  const kept = await page.evaluate(seenBoxes, `a[href], button, input, select, textarea, summary, ${CARD_PARTS}`);
  const wrong: string[] = [];
  for (const pet of shown) {
    const kind = KINDS[pet.id];
    if (kind === undefined) {
      wrong.push(`${pet.id} is no species of the ensemble`);
      continue;
    }
    const feet = pet.y + hoverOf(pet.id) * pet.size;
    const half = (kind.size.width * pet.size) / 2;
    if (!edges.some((edge) => Math.abs(edge.top - feet) <= 1.5 && pet.x >= edge.left - 1 && pet.x <= edge.right + 1)) wrong.push(`${pet.id} at ${pet.x}, ${pet.y} stands on no edge`);
    for (const other of shown) {
      const mate = KINDS[other.id];
      if (mate === undefined || other.id <= pet.id) continue;
      if (Math.abs(other.y + hoverOf(other.id) * other.size - feet) <= 1.5 && Math.abs(other.x - pet.x) < ((kind.size.width + mate.size.width) / 2) * pet.size - 1) wrong.push(`${pet.id} stands in ${other.id}`);
    }
    for (const box of kept) if (Math.min(pet.x + half, box.right) - Math.max(pet.x - half, box.left) > 1 && Math.min(pet.y, box.bottom) - Math.max(pet.y - kind.size.height * pet.size, box.top) > 1) wrong.push(`${pet.id} at ${pet.x}, ${pet.y} covers something it must keep clear of`);
  }
  return wrong;
}

/** 🏎️ In the page, before any of its scripts: sets the client's tempo seam (`attribute`) to `tempo` on the document
 * root — as soon as the root exists, which it does not yet when an init script runs. */
export function hasten({ attribute, tempo }: { readonly attribute: string; readonly tempo: number }): void {
  const mark = (): boolean => {
    document.documentElement?.setAttribute(attribute, String(tempo));
    return document.documentElement !== null;
  };
  if (mark()) return;
  const watch = new MutationObserver(() => {
    if (mark()) watch.disconnect();
  });
  watch.observe(document, { childList: true });
}

/** 🎛️ Chooses a liveliness on the preferences page, which must be open: the choice is pressed and the client carries it. */
export async function choosePets(device: Device, choice: (typeof CHOICES)[number]): Promise<void> {
  const choices = pane(device.page, "prefs").getByRole("group", { name: PETS_LABEL[device.locale], exact: true }).getByRole("button");
  await choices.nth(CHOICES.indexOf(choice)).click();
  await expect(choices.nth(CHOICES.indexOf(choice))).toHaveAttribute("aria-pressed", "true");
  await expect(device.page.locator(".course-app")).toHaveAttribute("data-pets", choice);
}

/** 📛️ The line the preferences would show for the pets on screen right now: their names in the order of the ensemble. */
export async function castLine(device: Device): Promise<string> {
  const shown = await shownPets(device);
  return `${CAST_LINE[device.locale]}${SPECIES.filter((kind) => shown.includes(kind.id)).map((kind) => kind.name[device.locale]).join(" · ")}`;
}

/** 🗣️ What the status line of the play group says once a deed was asked of the pet called `name`. */
export function said(deed: keyof typeof DEEDS, name: string, locale: Locale): string {
  return DEEDS[deed].said[locale].replace("{{name}}", name);
}
