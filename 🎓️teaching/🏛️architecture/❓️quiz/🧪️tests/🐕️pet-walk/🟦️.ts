/** 🐕️ The pets of the site in a real browser, as the second round made them (design-v2 §22): decoration that lives on
 * the cards and answers the learner's hand. A click on a pet says hello, the next clicks ask for a trick and then for a
 * purr; a pet picked up hangs in the hand, and let go high above the page it opens its parachute, glides and lands;
 * circling the pointer round a pet changes its state; during a lively minute in which the learner drags pets over each
 * other and throws them, no two pets' bodies ever overlap; a control under a pet still receives its click; the "Show
 * pets" switch and the play group of the settings work by keyboard; a pet whose topic is a task of a quiz's page pushes a
 * copy of that task out of its place while the learner stays still, and is thrown off the moment the learner points at
 * the task or focuses it, which is back at once; on a quiz's page, whose cards leave the footer line as the only perch,
 * a climber raises its ladder against the lowest card and climbs on up its wall, and on the home screen a grappler on
 * the footer line reels itself up a rope to a card once the hand holds whoever sat there. What the first round asked still holds: the pets
 * show up on the home screen within seconds, hidden from assistive technology and never a hit target of their own,
 * they stand on what the cards show, the cast follows the learner and rests during a run, the learner's choice stops
 * them (`still`) or removes them (`off`), a device that asks for reduced motion gets still pets that answer no hand
 * until the learner chooses, one that forces its own colours gets none and is told why, a phone shows at most two small
 * ones, and every device stays clean throughout: no console error, no failed request, no policy violation.
 *
 * Every wait waits for a state the page shows: what a pet's depiction says it does (`data-pet-activity`), what carries
 * it (`data-pet-footing`) and the state it shows (`data-pet-state`), the hand's cursor on the document element
 * (`data-pet-cursor`), the client's own attributes. The depiction does not expose the solid body the stage keeps apart,
 * so the spec computes it: the size box of the species, mapped onto the screen through the drawing's own transform
 * (place, facing, size and tilt about the pivot). That box is the stage's body less its margin of 4 px on every side and
 * less what a canopy, a held gear or a floater's hover add, so two disjoint bodies leave these boxes at least 8 px apart;
 * the spec allows half a pixel for the transform's rounding to two decimals. The specs that wait for minutes of the
 * pets' time — calm pets that walk after a choice, a ladder and a rope — set the client's test seam `data-pets-tempo="8"` before the client
 * starts, and the mischief spec `data-pets-tempo="1"` (a prank within a minute, a copy out long enough for the hand to
 * reach it); everything else the learner's hand does runs at the wall clock, because the hand's timing (a hold, the
 * leak of a pet's warmth, the speed of a circle) is the learner's.
 * @see ../../../🐾️pets/🔣️.json — the ensemble whose casts, gear and states are expected on screen
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learner these specs drive
 * @see ../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🐾️pets/🟦️.tsx — the quiz's glue to the pets
 * @see ../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/🤏️grasp/🟦️.ts — how a press reaches a pet
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html — why `still` and `off` exist
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pointer-gestures.html — why the settings offer every deed of the hand */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { Page } from "@playwright/test";
import { QUIZZES, answerRun, arrive, card, enter, expect, identify, pane, playQuiz, primary, readIntroduction, screen, submitRun, test, way, type Device, type Locale, type Text } from "../../🎭️e2e/🚶️learner/🟦️.ts";

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
  readonly states: readonly { readonly id: string }[];
  readonly tricks: readonly { readonly id: string; readonly cues: readonly string[]; readonly from?: readonly string[]; readonly to?: string }[];
}

const MENAGERIE = resolve(dirname(fileURLToPath(import.meta.url)), "../../../🐾️pets");
const ENSEMBLE = JSON.parse(readFileSync(resolve(MENAGERIE, "🔣️.json"), "utf8")) as { readonly casts: readonly Cast[]; readonly species: readonly string[] };
const CASTS = ENSEMBLE.casts;
const SPECIES = ENSEMBLE.species.map((path) => JSON.parse(readFileSync(resolve(MENAGERIE, path), "utf8")) as Kind);
const KINDS: Readonly<Record<string, Kind>> = Object.fromEntries(SPECIES.map((kind) => [kind.id, kind]));

/** 📐️ The box of every species, as the page's instruments take it. */
const BOXES: Readonly<Record<string, { readonly width: number; readonly height: number }>> = Object.fromEntries(SPECIES.map((kind) => [kind.id, kind.size]));

/** 🎈️ How high a species floats above what it stands on. */
function hoverOf(id: string): number {
  const kind = KINDS[id];
  return kind?.locomotion.gait === "float" ? (kind.locomotion.hover ?? 0) : 0;
}

/** 🃏️ The parts of a card of the screen in front: its tab and its body. */
const CARD_PARTS = '#quiz-main [data-card] [data-slot="window-chrome-chip-cap"], #quiz-main [data-card] [data-slot="window-chrome-body-surface"]';

/** 🧱️ What pets stand on in the quiz, as the client names it for its pet layer: the parts of every card, and the
 * footer line. */
const EDGES = `${CARD_PARTS}, .quiz-app > footer`;

/** 🕹️ What a press on a pet never takes from the learner in the quiz: the controls of the layer (links, buttons, form
 * fields, labels, ARIA widgets, focusable elements) and what the quiz adds (the grip of an item, the zones an item is
 * dropped on, the cards of the overview). A pointer over one of them is over a control, where circling is not followed. */
const CONTROLS = 'a[href], button, input, select, textarea, summary, label, [role="button"], [role="link"], [role="checkbox"], [role="radio"], [role="tab"], [role="menuitem"], [role="option"], [role="slider"], [role="switch"], [tabindex]:not([tabindex="-1"]), [contenteditable]:not([contenteditable="false"]), [data-quiz-grip], [data-quiz-drop], [data-layered-card]';

/** 🏡️ The scene of every screen that belongs to no quiz. */
const HOME = "home";

/** 🎚️ What a learner may choose for the pets, in the order the preferences offer it. */
const CHOICES = ["off", "still", "calm", "lively"] as const;

/** 🏷️ The name of the pets' choice in the preferences (`quiz.preferences.pets`). */
const PETS_LABEL: Text = { en: "Pets", de: "Tierchen" };

/** 🔘️ The name of the switch on the footer of every screen (`quiz.preferences.petsShown`). */
const SWITCH_LABEL: Text = { en: "Show pets", de: "Tierchen anzeigen" };

/** 🪧️ How the preferences begin the line that names the pets on stage (`quiz.preferences.petsCast`). */
const CAST_LINE: Text = { en: "Here right now: ", de: "Gerade hier: " };

/** 🐢️ What the preferences say to a learner whose device asks for reduced motion and who has not chosen how lively the
 * pets are (`quiz.preferences.petsReduced`). */
const REDUCED_NOTE: Text = { en: "Your device asks for less motion, so the pets stay still until you choose.", de: "Dein Gerät bittet um weniger Bewegung, deshalb bleiben die Tierchen reglos, bis du selbst wählst." };

/** 🔲️ What the preferences say to a learner whose device forces its own colours (`quiz.preferences.petsForced`). */
const FORCED_NOTE: Text = { en: "Your device uses its own contrast colours, so no pets show.", de: "Dein Gerät verwendet eigene Kontrastfarben, deshalb erscheinen keine Tierchen." };

/** 🤹️ The name of the group of the settings that plays with the pets on stage (`quiz.preferences.petsPlayWith`). */
const PLAY_LABEL: Text = { en: "Play with the pets", de: "Mit den Tierchen spielen" };

/** 🎾️ What the settings can ask a pet for, the name of its button and what the status line says once it was asked (`quiz.preferences.pets{Hello,Trick,Pet,Toss}` and `…Said`). */
const DEEDS = {
  hello: { button: { en: "Hello", de: "Hallo" }, said: { en: "{{name}} says hello.", de: "{{name}} sagt Hallo." } },
  trick: { button: { en: "Trick", de: "Kunststück" }, said: { en: "{{name}} does a trick.", de: "{{name}} macht ein Kunststück." } },
  pet: { button: { en: "Pet", de: "Streicheln" }, said: { en: "{{name}} is petted.", de: "{{name}} wird gestreichelt." } },
  toss: { button: { en: "Toss", de: "Hochwerfen" }, said: { en: "{{name}} is tossed up.", de: "{{name}} wird hochgeworfen." } },
} as const satisfies Readonly<Record<string, { readonly button: Text; readonly said: Text }>>;

/** ⏩️ The client's test seam: the attribute of the document root that makes the pets' time pass faster. */
const TEMPO = "data-pets-tempo";

/** 🐎️ How many times faster than the wall clock the pets' time passes where a spec waits for minutes of it. */
const HASTE = 8;

/** 🐌️ How many times faster the pets' time passes in the mischief spec: not at all. A prank still comes within a minute (a lively stage waits 45 s from its start and between two pranks, and 12 s for a still learner), and its copy stays out for some ten seconds — the hand has to reach it while it is out, also when four browsers share a busy machine and a step of the spec takes a second. */
const MISCHIEF_HASTE = 1;

/** 🔥️ The quiz whose page the mischief spec idles on: in its cast two climbers (radiatory of its core, insuly of its rotation) have a task of its page among their grounds and reach a wall beside its rows — the pets of physics that cover its tasks float or only own a parachute. */
const MISCHIEF_QUIZ = "heating";

/** 🖐️ The attribute of the document element on which the hand shows its cursor: `grab` over a pet it can pick up, `grabbing` while it holds one. */
const CURSOR = "data-pet-cursor";

/** ⏱️ How long the first pet may take to show once the home screen is there. On an idle machine it is there within two
 * seconds; the gate runs four browsers against one cold dev server, which first has to transform the lazy chunk. */
const APPEAR_MS = 30_000;

/** 🚶️ How long a change of cast may take: the pets that leave walk off and fade, the new ones fade in. */
const RECAST_MS = 45_000;

/** 💬️ How long a pet may take to answer the hand or a deed: a frame or two, and a trick of at most three seconds. */
const ANSWER_MS = 6_000;

/** 🪂️ How long a pet let go high above the page may take to land: a parachute sinks at 20 to 45 px a second. */
const LANDING_MS = 60_000;

/** 🧮️ How many animation frames a stillness is watched for: a second and a half on an idle machine. Frames, not
 * milliseconds: what must not happen is a write into the layer, and a frame is the only time one can happen. */
const STILL_FRAMES = 90;

/** 🕰️ How long a walk may be waited for at eight times the pace. */
const WALK_MS = 120_000;

/** ⚡️ The lively minute in which no two pets may ever overlap, in milliseconds of the wall clock. */
const LIVELY_MS = 60_000;

/** 🤏️ How far, in pixels of the screen, two bodies may reach into each other before they overlap: the rounding of a
 * drawing's place to two decimals. */
const OVERLAP_SLACK = 0.5;

/** 🧗️ How high above the edge it falls on a pet is let go of in the parachute spec, at most, in pixels. */
const DROP_HEIGHT = 420;

/** 📏️ How tall, at least, the body of the card a pet falls in front of must be, in pixels: the overview at 1440 × 900
 * fills its cells with cards whose bodies are 34 to 84 px tall, and a pet let go at rest `RELEASE_DEPTH` under the top
 * of the tallest keeps the middle of its body in front of it for some ten frames before it has fallen past. */
const FRONT_LEAST = 56;

/** 🪶️ How far under the top of that card's body the hand lets the pet go, in pixels: it hangs from its scruff, so the
 * middle of its body is a little lower still. */
const RELEASE_DEPTH = 6;

/** 🍩️ How far beyond half the larger side of its box the pointer circles a pet, in pixels: inside the band in which
 * the stage follows a circle (from 8 px beyond half the larger side to 3.4 sides from the middle). */
const CIRCLE_BEYOND = 14;

/** 🧭️ How many corners one lap of a circle has: the pointer jumps from corner to corner, an eighth of a turn at a time. */
const CIRCLE_POINTS = 8;

/** ⏲️ How long one lap of a circle takes by the page's clock, in milliseconds: a quarter turn in a quarter of a second,
 * the pace of a hand — between the scribble (four quarter turns a second) and the stall (a quarter turn in more than
 * 0.625 s) the stage refuses as a circle —, held by the clock the stage keeps time by, however long a move takes. */
const CIRCLE_LAP_MS = 1000;

/** 🌀️ How many laps the spec draws round a pet: five quarter turns make a circle, and the first lap begins anew while the pointer comes in. */
const CIRCLE_LAPS = 3;

/** 🪝️ How many of its own heights the rope of a grappling gun reaches at most (`ROPE_LONG` of the pets' climbing): on the
 * home screen at 1440 × 900 those are the tops of the bottom row of cards over the footer line, which pets of every
 * kind like to sit on. */
const ROPE_REACH = 8;

/** 🔁️ How long the gear spec waits for a rope while the hand holds whoever sat on the perch in a rope's reach, before it
 * lets go and clears the way again, in milliseconds of the wall clock: two minutes of the pets' time at eight times the
 * pace, in which a grappler on the footer line feels like going up a time or two — and in which a newcomer of the
 * cast's rotation or a pet with a ladder may take the perch first. */
const ROPE_ROUND_MS = 15_000;

/** ⌛️ How long the gear spec tries for a rope in all, round after round. */
const ROPE_MS = 360_000;

/** 🧩️ What the page's instruments say of one pet: what its depiction says it does, what carries it, the state it shows,
 * how visible it is, and the box of its body on the screen. */
interface Body {
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
interface Instruments {
  readonly bodies: () => Body[];
  readonly trails: Record<string, string[]>;
  readonly overlaps: (milliseconds: number, slack: number) => Promise<Watched>;
}

/** 🧰️ In the page: installs the spec's instruments on `window.petWalk`, once per pet layer — `bodies()` (every pet as
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
async function instrumented(page: Page): Promise<void> {
  await expect(page.locator(".pet-layer svg.pet")).not.toHaveCount(0, { timeout: APPEAR_MS });
  await page.evaluate(instrument, BOXES);
}

/** 🩻️ Every pet of `page` as the instruments see it. */
async function bodies(page: Page): Promise<readonly Body[]> {
  return page.evaluate(() => (window as unknown as { petWalk: Instruments }).petWalk.bodies());
}

/** 🧵️ The changes of what `id` did since the instruments were installed, as `activity/footing/state`. */
async function trailOf(page: Page, id: string): Promise<readonly string[]> {
  return page.evaluate((species) => [...((window as unknown as { petWalk: Instruments }).petWalk.trails[species] ?? [])], id);
}

/** 🪜️ How many of `wanted` a trail shows in that order, each matched against the start of an entry (`activity/footing/state`). */
function inOrder(trail: readonly string[], wanted: readonly string[]): number {
  let found = 0;
  for (const line of trail) if (found < wanted.length && line.startsWith(wanted[found]!)) found += 1;
  return found;
}

/** 📍️ The middle of a body on the screen. */
function middleOf(body: Body): { readonly x: number; readonly y: number } {
  return { x: (body.left + body.right) / 2, y: (body.top + body.bottom) / 2 };
}

/** 🔍️ In the page: for every point, whether a control of the quiz lies under it (the pet layer takes no pointer, so the element under a point is the page's) or it lies outside the window. */
function overControls(question: { readonly points: readonly { readonly x: number; readonly y: number }[]; readonly controls: string }): boolean[] {
  return question.points.map((point) => point.x < 1 || point.y < 1 || point.x > window.innerWidth - 1 || point.y > window.innerHeight - 1 || document.elementFromPoint(point.x, point.y)?.closest(question.controls) != null);
}

/** 🙋️ A pet the learner's hand can reach now, among those `wanted` picks: it stands idle on a perch, wholly visible, and no control lies under the middle of its body. Throws while there is none, for `toPass`. */
async function reachable(page: Page, wanted: (body: Body) => boolean = () => true): Promise<Body> {
  const shown = (await bodies(page)).filter((body) => body.activity === "idle" && body.footing === "perch" && body.opacity > 0.99 && wanted(body));
  const covered = await page.evaluate(overControls, { points: shown.map(middleOf), controls: CONTROLS });
  const free = shown.filter((_, index) => !covered[index]);
  if (free.length === 0) throw new Error(`no pet the hand can reach among ${shown.map((body) => body.id).join(", ") || "nobody"}`);
  return free[0]!;
}

/** 🔄️ The state circling a pet one way (`circle`, clockwise on the screen) or the other (`countercircle`) leaves it in, as its species authors it: the first trick that cue sets off in `state` — its `to`, or one rung along the states its `from` lists (every state when it lists none), up for a trick cued by circling clockwise and down otherwise —, or `state` when no trick answers. */
function circled(kind: Kind, state: string, cue: "circle" | "countercircle"): string {
  const trick = kind.tricks.find((candidate) => candidate.cues.includes(cue) && (candidate.from === undefined || candidate.from.includes(state)));
  const states = kind.states.map((entry) => entry.id);
  if (trick === undefined) return state;
  if (trick.to !== undefined) return states.includes(trick.to) ? trick.to : state;
  const listed = (trick.from ?? []).filter((rung) => states.includes(rung));
  const rungs = listed.length > 0 ? listed : states;
  const index = rungs.indexOf(state);
  const next = index + (trick.cues.includes("circle") ? 1 : -1);
  return index < 0 || next < 0 || next >= rungs.length ? state : rungs[next]!;
}

/** ⭕️ The points of a circle round a body, `laps` times, clockwise on the screen for `circle` and the other way for `countercircle`. */
function circleRound(body: Body, cue: "circle" | "countercircle", laps: number): { readonly x: number; readonly y: number }[] {
  const middle = middleOf(body);
  const radius = Math.max(body.right - body.left, body.bottom - body.top) / 2 + CIRCLE_BEYOND;
  const way = cue === "circle" ? 1 : -1;
  return Array.from({ length: laps * CIRCLE_POINTS + 1 }, (_, step) => ({ x: middle.x + radius * Math.cos((way * step * 2 * Math.PI) / CIRCLE_POINTS), y: middle.y + radius * Math.sin((way * step * 2 * Math.PI) / CIRCLE_POINTS) }));
}

/** 🏝️ In the page: a place on the footer line with room for a pet and a circle round it — no part of a card and no pet
 * within `reach` pixels to either side and `room` pixels above the line, no control on the line below it — the one
 * nearest the middle of the window, or `null`. */
function floorSpot(question: { readonly parts: string; readonly controls: string; readonly reach: number; readonly room: number }): { readonly x: number; readonly top: number } | null {
  const footer = document.querySelector(".quiz-app > footer");
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
async function carry(page: Page, body: Body, to: { readonly x: number; readonly y: number }): Promise<void> {
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

/** 🛬️ Waits until the pet `id` stands on a perch again, wholly visible. */
async function expectPerched(page: Page, id: string, timeout: number): Promise<void> {
  await expect.poll(async () => (await bodies(page)).some((body) => body.id === id && body.footing === "perch" && body.opacity > 0.99 && !["hang", "tumble", "fall", "glide"].includes(body.activity)), { message: `${id} stands on a perch again`, timeout }).toBe(true);
}

function castOf(scene: string): Cast {
  const cast = CASTS.find((candidate) => candidate.scene === scene);
  if (cast === undefined) throw new Error(`the ensemble has no cast for ${scene}`);
  return cast;
}

function layer(device: Device): ReturnType<Device["page"]["locator"]> {
  return device.page.locator(".pet-layer");
}

function pets(device: Device): ReturnType<Device["page"]["locator"]> {
  return device.page.locator(".pet-layer svg.pet");
}

/** 🐾️ The species on screen, by id. */
async function shownPets(device: Device): Promise<readonly string[]> {
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
function framesPass(frames: number): Promise<void> {
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
function paceTo(deadline: number): Promise<void> {
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
function watchStill(watch: { readonly frames: number; readonly places: boolean }): Promise<boolean> {
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
async function expectMotionless(device: Device): Promise<void> {
  await expect(async () => expect(await device.page.evaluate(watchStill, { frames: STILL_FRAMES, places: false }), "nothing is written into the pet layer").toBe(true)).toPass({ timeout: RECAST_MS });
}

/** 🏃️ Checks that the pets on screen are alive: the drawing changes. */
async function expectMoving(device: Device): Promise<void> {
  const first = await drawing(device);
  await expect.poll(() => drawing(device)).not.toBe(first);
}

/** 🧷️ In the page: which properties of a pet may transition and how many transitions and animations the browser runs
 * inside the pet layer. Frames are written by script; under reduced motion the quiz gives every element a transition
 * of a hundredth of a millisecond, which would start one with every write into a pet. */
function layerTransitions(): { property: string; running: number } {
  const pet = document.querySelector(".pet-layer svg.pet");
  const inLayer = (animation: Animation): boolean => (animation.effect as KeyframeEffect | null)?.target?.closest(".pet-layer") != null;
  return { property: pet === null ? "" : getComputedStyle(pet).transitionProperty, running: document.getAnimations().filter(inLayer).length };
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
async function expectWalk(device: Device): Promise<void> {
  const walked = await device.page.evaluate(watchWalk, WALK_MS);
  expect(walked, "a pet whose feet move while it walks").not.toBeNull();
  expect(Math.abs(walked!.to - walked!.from)).toBeGreaterThanOrEqual(4);
}

/** 🎭️ Waits until the pets on screen are those of `scene` — its core and the visitors of its rotation: nobody of
 * another cast is left, nobody is there twice, and — where `present` — somebody of its cast is there. Never the whole
 * cast: how many of it show, and how soon, depends on the room the screen offers and on how busy the machine is. */
async function expectCast(device: Device, scene: string, present: boolean): Promise<void> {
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
async function takenTargets(device: Device): Promise<readonly string[]> {
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
async function enterWithPets(device: Device): Promise<void> {
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
async function standing(page: Page): Promise<readonly Standing[]> {
  return page.locator(".pet-layer svg.pet").evaluateAll((drawn) =>
    drawn.map((pet) => {
      const place = /translate\(([-\d.]+)px, ([-\d.]+)px\) scale\(([-\d.]+), ([-\d.]+)\)/u.exec((pet as SVGElement).style.transform);
      return { id: pet.getAttribute("data-pet") ?? "", x: Number(place?.[1]), y: Number(place?.[2]), size: Number(place?.[4]), opacity: Number((pet as SVGElement).style.opacity || "1"), footing: pet.getAttribute("data-pet-footing") ?? "" };
    }),
  );
}

/** ✂️ In the page: what one sees of the elements `selector` matches — each box cut down to what its scrolling and
 * clipping ancestors let through, without anything inert, hidden or scrolled away — and whether its top edge shows. */
function seenBoxes(selector: string): { left: number; top: number; right: number; bottom: number; topped: boolean }[] {
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
async function misplaced(page: Page): Promise<readonly string[]> {
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
function hasten({ attribute, tempo }: { readonly attribute: string; readonly tempo: number }): void {
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
async function choosePets(device: Device, choice: (typeof CHOICES)[number]): Promise<void> {
  const choices = pane(device.page, "prefs").getByRole("group", { name: PETS_LABEL[device.locale], exact: true }).getByRole("button");
  await choices.nth(CHOICES.indexOf(choice)).click();
  await expect(choices.nth(CHOICES.indexOf(choice))).toHaveAttribute("aria-pressed", "true");
  await expect(device.page.locator(".quiz-app")).toHaveAttribute("data-pets", choice);
}

/** 📛️ The line the preferences would show for the pets on screen right now: their names in the order of the ensemble. */
async function castLine(device: Device): Promise<string> {
  const shown = await shownPets(device);
  return `${CAST_LINE[device.locale]}${SPECIES.filter((kind) => shown.includes(kind.id)).map((kind) => kind.name[device.locale]).join(" · ")}`;
}

/** 🗣️ What the status line of the play group says once a deed was asked of the pet called `name`. */
function said(deed: keyof typeof DEEDS, name: string, locale: Locale): string {
  return DEEDS[deed].said[locale].replace("{{name}}", name);
}

test("pets live on the home screen as decoration: hidden from assistive technology, never a hit target of their own, standing on what the cards show", async ({ device }) => {
  const learner = await device("en");
  await enterWithPets(learner);
  await expect(learner.page.locator(".quiz-app")).toHaveAttribute("data-pets", "calm");
  await expect(layer(learner)).toHaveCount(1);
  await expect(layer(learner)).toHaveAttribute("aria-hidden", "true");
  await expect(layer(learner)).toHaveCSS("pointer-events", "none");
  await expect(layer(learner).locator("a, button, input, select, textarea, summary, [tabindex], [role], [id], style, script")).toHaveCount(0);
  await expectCast(learner, HOME, true);
  await expectMoving(learner);
  for (const pet of await pets(learner).all()) {
    await expect(pet).toHaveAttribute("data-pet-activity", /^[a-z]+$/u);
    await expect(pet).toHaveAttribute("data-pet-footing", /^[a-z]+$/u);
    await expect(pet).toHaveAttribute("data-pet-state", /^[a-z0-9-]+$/u);
  }
  expect(await takenTargets(learner)).toEqual([]);
  await expect.poll(() => misplaced(learner.page), { timeout: RECAST_MS }).toEqual([]);

  await primary(card(learner.page, "board")).click();
  await expect(pane(learner.page, "board")).toHaveAttribute("data-opened", "");
  expect(await takenTargets(learner)).toEqual([]);
  await way(learner.page, "overview").click();
  await expect(pane(learner.page, "board")).not.toHaveAttribute("data-opened", "");
  await expect(pets(learner)).not.toHaveCount(0, { timeout: APPEAR_MS });
  expect(await learner.page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth)).toBeLessThanOrEqual(0);
});

test("a click on a pet says hello, the next one asks for a trick and the ones after it for a purr, and nothing beneath the pet acts", async ({ device }) => {
  const learner = await device("en");
  const page = learner.page;
  await enterWithPets(learner);
  await instrumented(page);
  const tried = new Set<string>();
  await expect(async () => {
    const pet = await reachable(page, (body) => !tried.has(body.id));
    tried.add(pet.id);
    const click = async (): Promise<void> => {
      const now = (await bodies(page)).find((body) => body.id === pet.id);
      expect(now, `${pet.id} is still on stage`).toBeDefined();
      const middle = middleOf(now!);
      await page.mouse.click(middle.x, middle.y);
    };
    const since = Math.max((await trailOf(page, pet.id)).length - 1, 0);
    const answered = (wanted: readonly string[]): Promise<number> => trailOf(page, pet.id).then((trail) => inOrder(trail.slice(since), wanted));
    const middle = middleOf(pet);
    await page.mouse.move(middle.x, middle.y);
    await expect(page.locator("html")).toHaveAttribute(CURSOR, "grab", { timeout: ANSWER_MS });
    await click();
    await expect.poll(() => answered(["greet"]), { message: `${pet.id} says hello`, timeout: ANSWER_MS }).toBe(1);
    await click();
    await expect.poll(() => answered(["greet", "trick"]), { message: `${pet.id} does a trick`, timeout: ANSWER_MS }).toBe(2);
    for (let more = 0; more < 4 && (await answered(["greet", "trick", "purr"])) < 3; more++) {
      await click();
      await expect.poll(() => answered(["greet", "trick", "purr"]), { timeout: 400 }).toBe(3).catch(() => undefined);
    }
    expect(await answered(["greet", "trick", "purr"]), `${pet.id}: ${(await trailOf(page, pet.id)).join(" → ")}`).toBe(3);
    test.info().annotations.push({ type: "trail", description: `${pet.id}: ${(await trailOf(page, pet.id)).slice(since).join(" → ")}` });
  }).toPass({ timeout: RECAST_MS });
  expect(await page.evaluate(() => window.location.hash), "a press on a pet is the pet's: no card under it opened").toBe("");
});

test("a pet picked up hangs in the hand, and let go high above the page it opens its parachute, glides down and lands", async ({ device }) => {
  const learner = await device("en");
  const page = learner.page;
  await enterWithPets(learner);
  await instrumented(page);
  const parachuted = (body: Body): boolean => KINDS[body.id]?.gear.includes("parachute") === true;
  let pet: Body | undefined;
  await expect(async () => {
    pet = await reachable(page, parachuted);
  })
    .toPass({ timeout: RECAST_MS })
    .catch(async () => {
      await page.evaluate(() => (window.location.hash = "heating"));
      await expect(pane(page, "heating")).toHaveAttribute("data-opened", "");
      await instrumented(page);
      await expect(async () => {
        pet = await reachable(page, parachuted);
      }).toPass({ timeout: RECAST_MS });
    });
  const held = pet!;
  const edges = (await page.evaluate(seenBoxes, EDGES)).filter((edge) => edge.topped);
  const top = 70;
  const width = page.viewportSize()?.width ?? 1440;
  const columns = Array.from({ length: Math.floor((width - 160) / 8) }, (_, index) => 80 + index * 8).map((x) => ({ x, floor: Math.min(...edges.filter((edge) => edge.left <= x && edge.right >= x && edge.top > top + 40).map((edge) => edge.top)) }));
  const column = columns.reduce((best, candidate) => (candidate.floor > best.floor && candidate.floor < Infinity ? candidate : best));
  const release = { x: column.x, y: Math.max(top, column.floor - DROP_HEIGHT) };
  expect(column.floor - release.y, "a fall long enough for a parachute").toBeGreaterThan(200);

  await carry(page, held, release);
  await expect(page.locator("html")).toHaveAttribute(CURSOR, "grabbing");
  await expect.poll(() => trailOf(page, held.id).then((trail) => inOrder(trail, ["hang/hand"])), { message: `${held.id} hangs in the hand`, timeout: ANSWER_MS }).toBe(1);
  await page.mouse.up();
  await expect.poll(() => trailOf(page, held.id).then((trail) => inOrder(trail, ["hang/hand", "glide/chute"])), { message: `${held.id} opens its parachute`, timeout: ANSWER_MS }).toBe(2);
  await expectPerched(page, held.id, LANDING_MS);
  test.info().annotations.push({ type: "trail", description: `${held.id} let go at ${release.x}, ${release.y} above ${column.floor}: ${(await trailOf(page, held.id)).join(" → ")}` });
  expect(inOrder(await trailOf(page, held.id), ["hang/hand", "glide/chute", "idle/perch"]) >= 2, (await trailOf(page, held.id)).join(" → ")).toBe(true);
  await expect(page.locator("html")).not.toHaveAttribute(CURSOR, "grabbing");
});

test("no two pets ever overlap during a lively minute in which the learner drags them over each other and throws them", async ({ device }) => {
  const learner = await device("en");
  const page = learner.page;
  await enterWithPets(learner);
  await page.evaluate(() => (window.location.hash = "prefs"));
  await expect(pane(page, "prefs")).toHaveAttribute("data-opened", "");
  await choosePets(learner, "lively");
  await page.keyboard.press("Escape");
  await expect(pane(page, "prefs")).not.toHaveAttribute("data-opened", "");
  await instrumented(page);
  const watching = page.evaluate(({ milliseconds, slack }) => (window as unknown as { petWalk: Instruments }).petWalk.overlaps(milliseconds, slack), { milliseconds: LIVELY_MS, slack: OVERLAP_SLACK });
  watching.catch(() => undefined);
  const started = Date.now();
  const thrown: string[] = [];
  while (Date.now() - started < LIVELY_MS - 15_000) {
    const pet = await reachable(page).catch(() => undefined);
    if (pet === undefined) {
      await page.evaluate(framesPass, 30);
      continue;
    }
    const others = (await bodies(page)).filter((body) => body.id !== pet.id && body.opacity > 0);
    const over = others.length === 0 ? { x: 720, y: 300 } : middleOf(others[thrown.length % others.length]!);
    await carry(page, pet, over);
    const fling = { x: Math.min(Math.max(over.x + (thrown.length % 2 === 0 ? 160 : -160), 40), 1400), y: Math.max(over.y - 160, 60) };
    for (let step = 1; step <= 4; step++) await page.mouse.move(over.x + ((fling.x - over.x) * step) / 4, over.y + ((fling.y - over.y) * step) / 4);
    await page.mouse.up();
    thrown.push(pet.id);
    await expectPerched(page, pet.id, ANSWER_MS * 2).catch(() => undefined);
  }
  const watched = await watching;
  test.info().annotations.push({ type: "watched", description: `${watched.frames} frames, at most ${watched.most} pets on one, ${watched.overlaps.length} overlaps; thrown: ${thrown.join(", ")}` });
  expect(thrown.length, `pets thrown: ${thrown.join(", ")}`).toBeGreaterThanOrEqual(3);
  expect(watched.frames, "frames watched").toBeGreaterThan(300);
  expect(watched.most, "pets on one frame").toBeGreaterThanOrEqual(2);
  expect(watched.overlaps.map(({ frame, pair: [one, other] }) => `frame ${frame}: ${one.id} ${one.activity}/${one.footing} [${one.left.toFixed(1)} ${one.top.toFixed(1)} ${one.right.toFixed(1)} ${one.bottom.toFixed(1)}] × ${other.id} ${other.activity}/${other.footing} [${other.left.toFixed(1)} ${other.top.toFixed(1)} ${other.right.toFixed(1)} ${other.bottom.toFixed(1)}]`)).toEqual([]);
});

test("circling the pointer round a pet changes its state", async ({ device }) => {
  const learner = await device("en");
  const page = learner.page;
  const quiz = QUIZZES[1]!.id;
  await enterWithPets(learner);
  await page.evaluate((id) => (window.location.hash = id), quiz);
  await expect(pane(page, quiz)).toHaveAttribute("data-opened", "");
  await expectCast(learner, quiz, true);
  await instrumented(page);
  const tried = new Set<string>();
  const plan = async (): Promise<{ readonly body: Body; readonly cue: "circle" | "countercircle"; readonly becomes: string; readonly path: { readonly x: number; readonly y: number }[] } | undefined> => {
    const shown = (await bodies(page)).filter((body) => body.activity === "idle" && body.footing === "perch" && body.opacity > 0.99 && !tried.has(body.id));
    const plans = shown.flatMap((body) =>
      (["circle", "countercircle"] as const)
        .map((cue) => ({ body, cue, becomes: circled(KINDS[body.id]!, body.state, cue), path: circleRound(body, cue, CIRCLE_LAPS) }))
        .filter((candidate) => candidate.becomes !== candidate.body.state)
        .slice(0, 1),
    );
    for (const candidate of plans) if (!(await page.evaluate(overControls, { points: candidate.path, controls: CONTROLS })).includes(true)) return candidate;
    return undefined;
  };
  await expect(async () => {
    let chosen = await plan();
    if (chosen === undefined) {
      const spot = await page.evaluate(floorSpot, { parts: CARD_PARTS, controls: CONTROLS, reach: 70, room: 140 });
      expect(spot, "a place on the footer line with room for a circle").not.toBeNull();
      const pet = await reachable(page, (body) => !tried.has(body.id));
      await carry(page, pet, { x: spot!.x, y: spot!.top - 100 });
      await page.mouse.up();
      await expectPerched(page, pet.id, LANDING_MS);
      await expect.poll(async () => (await bodies(page)).find((body) => body.id === pet.id)?.activity, { message: `${pet.id} rests on the footer line`, timeout: ANSWER_MS }).toBe("idle");
      chosen = await plan();
      if (chosen === undefined) throw new Error(`no pet could be circled clear of the controls, not even ${pet.id} on the footer line at ${spot!.x}`);
    }
    const { body, cue, becomes, path } = chosen;
    tried.add(body.id);
    const since = Math.max((await trailOf(page, body.id)).length - 1, 0);
    await page.mouse.move(path[0]!.x, path[0]!.y);
    const begun = await page.evaluate(() => performance.now());
    for (const [index, point] of path.entries()) {
      await page.mouse.move(point.x, point.y);
      await page.evaluate(paceTo, begun + (index * CIRCLE_LAP_MS) / CIRCLE_POINTS);
    }
    await page.mouse.move(path[0]!.x + 200, Math.max(path[0]!.y - 200, 4), { steps: 3 });
    await expect
      .poll(async () => ({ state: (await bodies(page)).find((candidate) => candidate.id === body.id)?.state, trail: (await trailOf(page, body.id)).slice(since) }), { message: `${body.id} circled ${cue} from ${body.state}`, timeout: ANSWER_MS })
      .toMatchObject({ state: becomes, trail: expect.arrayContaining([expect.stringMatching(/^trick\//u)]) });
    test.info().annotations.push({ type: "trail", description: `${body.id} circled ${cue}: ${(await trailOf(page, body.id)).slice(since).join(" → ")}` });
  }).toPass({ timeout: 2 * RECAST_MS });
});

test("a control under a pet still receives its click: the card a falling pet passes in front of opens, and the pet is not picked up", async ({ device }) => {
  const learner = await device("en");
  const page = learner.page;
  await enterWithPets(learner);
  await instrumented(page);
  const fronts = await Promise.all(QUIZZES.map(async (quiz) => ({ id: quiz.id, box: await page.locator(`[data-layered-card="${quiz.id}"] [data-slot="window-chrome-body-surface"]`).boundingBox() })));
  const { id: target, box } = fronts.reduce((tallest, candidate) => ((candidate.box?.height ?? 0) > (tallest.box?.height ?? 0) ? candidate : tallest));
  const front = box!;
  expect(front.height, "a card tall enough for the middle of a pet to pass in front of it").toBeGreaterThan(FRONT_LEAST);
  await page.evaluate((id) => {
    const host = window as unknown as { petWalkClicked?: number };
    host.petWalkClicked = 0;
    document.querySelector(`[data-layered-card="${id}"]`)!.addEventListener("click", () => (host.petWalkClicked = (host.petWalkClicked ?? 0) + 1));
  }, target);
  let pet: Body | undefined;
  await expect(async () => {
    pet = await reachable(page, (body) => KINDS[body.id]?.gear.includes("parachute") === true).catch(() => reachable(page));
  }).toPass({ timeout: RECAST_MS });
  const fallen = pet!;
  await carry(page, fallen, { x: front.x + front.width / 2, y: front.y + RELEASE_DEPTH });
  await page.mouse.up();
  let pressed: { readonly x: number; readonly y: number; readonly footing: string } | undefined;
  await expect
    .poll(
      async () => {
        const now = (await bodies(page)).find((body) => body.id === fallen.id);
        if (now === undefined || !["air", "chute"].includes(now.footing)) return false;
        const middle = middleOf(now);
        const inside = middle.x > front.x + 4 && middle.x < front.x + front.width - 4 && middle.y > front.y + 4 && middle.y < front.y + front.height - 4;
        const under = await page.evaluate(([x, y, id]) => {
          const element = document.elementFromPoint(x, y);
          return element?.closest(`[data-layered-card="${id}"]`) != null && element.closest("a[href], button, input, select, textarea, summary, label") === null;
        }, [middle.x, middle.y, target] as const);
        if (!inside || !under) return false;
        await page.mouse.click(middle.x, middle.y);
        pressed = { ...middle, footing: now.footing };
        return true;
      },
      { message: `${fallen.id} falls in front of the card of ${target}`, timeout: ANSWER_MS, intervals: [20] },
    )
    .toBe(true);
  await expect(pane(page, target)).toHaveAttribute("data-opened", "");
  expect(await page.evaluate(() => (window as unknown as { petWalkClicked?: number }).petWalkClicked), `the card took the click at ${pressed?.x}, ${pressed?.y} in front of ${fallen.id} (${pressed?.footing})`).toBe(1);
  const after = await trailOf(page, fallen.id);
  test.info().annotations.push({ type: "trail", description: `${fallen.id} in front of ${target} at ${pressed?.x}, ${pressed?.y} (${pressed?.footing}): ${after.join(" → ")}` });
  expect(after.filter((line) => line.startsWith("hang/hand")), `${fallen.id} was picked up once, by the carry: ${after.join(" → ")}`).toHaveLength(1);
  await expect(page.locator("html")).not.toHaveAttribute(CURSOR, "grabbing");
});

test("the settings let the learner play with a pet on stage by keyboard: a hello, a trick, a stroke and a toss", async ({ device }) => {
  const learner = await device("de");
  const page = learner.page;
  await enterWithPets(learner);
  await page.evaluate(() => (window.location.hash = "prefs"));
  await expect(pane(page, "prefs")).toHaveAttribute("data-opened", "");
  await instrumented(page);
  const group = pane(page, "prefs").getByRole("group", { name: PLAY_LABEL[learner.locale], exact: true });
  await expect(group).toBeVisible();
  const status = group.getByRole("status");
  const tried = new Set<string>();
  await expect(async () => {
    const pet = await reachable(page, (body) => !tried.has(body.id));
    tried.add(pet.id);
    const name = KINDS[pet.id]!.name[learner.locale];
    const player = group.getByRole("group", { name, exact: true });
    const ask = async (deed: keyof typeof DEEDS, key: "Enter" | "Space"): Promise<void> => {
      const button = player.getByRole("button", { name: DEEDS[deed].button[learner.locale], exact: true });
      await button.focus();
      await expect(button).toBeFocused();
      await page.keyboard.press(key);
      await expect(button).toBeFocused();
      await expect(status).toHaveText(said(deed, name, learner.locale));
    };
    const since = Math.max((await trailOf(page, pet.id)).length - 1, 0);
    const answered = (wanted: readonly string[]): Promise<number> => trailOf(page, pet.id).then((trail) => inOrder(trail.slice(since), wanted));
    await ask("hello", "Enter");
    await expect.poll(() => answered(["greet"]), { message: `${pet.id} says hello`, timeout: ANSWER_MS }).toBe(1);
    await ask("trick", "Space");
    await expect.poll(() => answered(["greet", "trick"]), { message: `${pet.id} does a trick`, timeout: ANSWER_MS }).toBe(2);
    await ask("pet", "Enter");
    await expect.poll(() => answered(["greet", "trick", "purr"]), { message: `${pet.id} purrs`, timeout: ANSWER_MS }).toBe(3);
    await ask("toss", "Space");
    await expect.poll(() => answered(["greet", "trick", "purr", "tumble/air"]), { message: `${pet.id} is tossed up`, timeout: ANSWER_MS }).toBe(4);
    await expectPerched(page, pet.id, LANDING_MS);
  }).toPass({ timeout: 2 * RECAST_MS });
});

test("the cast follows the learner: a quiz's page and run show its species, the home screen its own again", async ({ device }) => {
  const learner = await device("de");
  await enterWithPets(learner);
  for (const quiz of QUIZZES) {
    await learner.page.evaluate((id) => (window.location.hash = id), quiz.id);
    await expect(pane(learner.page, quiz.id)).toHaveAttribute("data-opened", "");
    await expectCast(learner, quiz.id, true);
    expect(await takenTargets(learner)).toEqual([]);
    await learner.page.keyboard.press("Escape");
    await expect(pane(learner.page, quiz.id)).not.toHaveAttribute("data-opened", "");
    await expectCast(learner, HOME, true);
  }

  const played = QUIZZES[0]!;
  await playQuiz(learner, played.id);
  await expectCast(learner, played.id, false);
  await expect(async () => {
    if ((await shownPets(learner)).length > 0) expect(await learner.page.evaluate(watchStill, { frames: 2 * STILL_FRAMES, places: true }), "pets stay where they are while a run is on screen").toBe(true);
  }).toPass({ timeout: RECAST_MS });
  expect(await takenTargets(learner)).toEqual([]);
  await expect(screen(learner.page, "run")).toBeVisible();
});

test("the learner's choice stops the pets, removes them and brings them back, and is kept", async ({ device }) => {
  const learner = await device("en");
  await enterWithPets(learner);
  const app = learner.page.locator(".quiz-app");
  const preferences = pane(learner.page, "prefs");
  const choices = preferences.getByRole("group", { name: PETS_LABEL[learner.locale], exact: true }).getByRole("button");
  const choose = (choice: (typeof CHOICES)[number]): Promise<void> => choosePets(learner, choice);
  await learner.page.evaluate(() => (window.location.hash = "prefs"));
  await expect(preferences).toHaveAttribute("data-opened", "");
  await expect(choices).toHaveCount(CHOICES.length);
  await expect(choices.nth(CHOICES.indexOf("calm"))).toHaveAttribute("aria-pressed", "true");
  await expectMoving(learner);
  await expect(async () => expect(await preferences.locator("p", { hasText: CAST_LINE[learner.locale] }).innerText()).toBe(await castLine(learner))).toPass({ timeout: RECAST_MS });

  await choose("still");
  await expect(pets(learner)).not.toHaveCount(0, { timeout: APPEAR_MS });
  await expectMotionless(learner);
  await expect(preferences.getByRole("group", { name: PLAY_LABEL[learner.locale], exact: true }), "still pets do not play").toHaveCount(0);

  await choose("off");
  await expect(layer(learner)).toHaveCount(0);
  await expect(learner.page.locator("svg.pet")).toHaveCount(0);
  await expect(preferences.locator("p", { hasText: CAST_LINE[learner.locale] })).toHaveCount(0);

  await learner.page.reload();
  await expect(preferences).toHaveAttribute("data-opened", "");
  await expect(app).toHaveAttribute("data-pets", "off");
  await expect(choices.nth(CHOICES.indexOf("off"))).toHaveAttribute("aria-pressed", "true");
  await expect(layer(learner)).toHaveCount(0);

  await choose("lively");
  await expect(pets(learner)).not.toHaveCount(0, { timeout: APPEAR_MS });
  await expect(layer(learner)).toHaveAttribute("aria-hidden", "true");
  await expectMoving(learner);
});

test("a device that asks for reduced motion decides the default only: its learner gets still pets that no hand moves until they choose, and calm ones that walk and answer from then on, also after a reload", async ({ device }) => {
  const learner = await device("de");
  const page = learner.page;
  const app = page.locator(".quiz-app");
  const preferences = pane(page, "prefs");
  const note = preferences.getByText(REDUCED_NOTE[learner.locale], { exact: true });
  const choices = preferences.getByRole("group", { name: PETS_LABEL[learner.locale], exact: true }).getByRole("button");
  const playing = preferences.getByRole("group", { name: PLAY_LABEL[learner.locale], exact: true });
  const pressed = (choice: (typeof CHOICES)[number]): Promise<void> => expect(choices.nth(CHOICES.indexOf(choice))).toHaveAttribute("aria-pressed", "true");
  await page.addInitScript(hasten, { attribute: TEMPO, tempo: HASTE });
  await page.emulateMedia({ reducedMotion: "reduce" });
  await enterWithPets(learner);
  await expect(page.locator("html")).toHaveAttribute(TEMPO, "8");
  await expect(app).toHaveAttribute("data-pets", "still");
  await expectCast(learner, HOME, true);
  await instrumented(page);
  await expectMotionless(learner);
  const still = await reachable(page);
  const middle = middleOf(still);
  await page.mouse.move(middle.x, middle.y, { steps: 5 });
  await expect(page.locator("html"), "the hand offers no still pet").not.toHaveAttribute(CURSOR, /.+/u);
  await page.mouse.click(middle.x, middle.y);
  await page.mouse.move(900, 600, { steps: 5 });
  await expectMotionless(learner);
  expect(await trailOf(page, still.id), `${still.id} answers no click`).toEqual([`idle/perch/${still.state}`]);
  expect(await takenTargets(learner)).toEqual([]);
  await page.evaluate(() => (window.location.hash = "prefs"));
  await expect(preferences).toHaveAttribute("data-opened", "");
  await expect(note).toBeVisible();
  await pressed("still");
  await expect(playing).toHaveCount(0);

  await page.emulateMedia({ reducedMotion: "no-preference" });
  await expect(note).toHaveCount(0);
  await expect(app).toHaveAttribute("data-pets", "calm");
  await pressed("calm");
  await expectMoving(learner);
  await page.emulateMedia({ reducedMotion: "reduce" });
  await expect(note).toBeVisible();
  await expect(app).toHaveAttribute("data-pets", "still");
  await pressed("still");
  await expectMotionless(learner);

  await choosePets(learner, "calm");
  await expect(note).toHaveCount(0);
  await expect(playing).toBeVisible();
  await page.mouse.move(4, 4);
  await expectWalk(learner);
  expect(await page.evaluate(layerTransitions), "the quiz shortens every transition for this device: a frame of the pets must not start one").toEqual({ property: "none", running: 0 });

  await page.reload();
  await expect(preferences).toHaveAttribute("data-opened", "");
  await expect(app).toHaveAttribute("data-pets", "calm");
  await pressed("calm");
  await expect(note).toHaveCount(0);
  await expect(pets(learner)).not.toHaveCount(0, { timeout: APPEAR_MS });
  await expectWalk(learner);
  expect(await page.evaluate(() => window.matchMedia("(prefers-reduced-motion: reduce)").matches), "the device still asks for reduced motion").toBe(true);
});

test("a device that forces its own colours shows no pets, and the preferences say why", async ({ device }) => {
  const learner = await device("de");
  const note = pane(learner.page, "prefs").getByText(FORCED_NOTE[learner.locale], { exact: true });
  await learner.page.emulateMedia({ forcedColors: "active" });
  await enter(learner, { kind: "anonymous" });
  await expect(learner.page.locator(".quiz-app")).toHaveAttribute("data-pets", "calm");
  await learner.page.evaluate(() => (window.location.hash = "prefs"));
  await expect(pane(learner.page, "prefs")).toHaveAttribute("data-opened", "");
  await expect(note).toBeVisible();
  await expect(layer(learner)).toHaveCount(1, { timeout: APPEAR_MS });
  await expect(pets(learner)).toHaveCount(0);
  await expect(pane(learner.page, "prefs").locator("p", { hasText: CAST_LINE[learner.locale] })).toHaveCount(0);

  await learner.page.emulateMedia({ forcedColors: "none" });
  await expect(note).toHaveCount(0);
  await expect(pets(learner)).not.toHaveCount(0, { timeout: APPEAR_MS });
  await expect(pane(learner.page, "prefs").locator("p", { hasText: CAST_LINE[learner.locale] })).toHaveCount(1, { timeout: RECAST_MS });
});

test("a switch on the footer of every screen hides the pets and brings them back as lively as they were, by keyboard", async ({ device }) => {
  const learner = await device("de");
  const page = learner.page;
  const app = page.locator(".quiz-app");
  const toggle = page.locator(".quiz-app > footer").getByRole("checkbox", { name: SWITCH_LABEL[learner.locale], exact: true });
  const press = async (shown: boolean, choice: (typeof CHOICES)[number]): Promise<void> => {
    await toggle.focus();
    await expect(toggle).toBeFocused();
    await page.keyboard.press("Space");
    await expect(toggle).toBeChecked({ checked: shown });
    await expect(app).toHaveAttribute("data-pets", choice);
    await expect(layer(learner)).toHaveCount(shown ? 1 : 0);
    expect(await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth)).toBeLessThanOrEqual(0);
  };

  await arrive(learner);
  await expect(toggle).toBeChecked();
  await press(false, "off");
  await press(true, "calm");
  await readIntroduction(learner);
  await expect(toggle).toBeChecked();
  await identify(learner, { kind: "anonymous" });
  await expect(page.locator("[data-layered-overview]")).toBeVisible();
  await expect(toggle).toBeChecked();
  await expect(pets(learner)).not.toHaveCount(0, { timeout: APPEAR_MS });

  await page.evaluate(() => (window.location.hash = "prefs"));
  await expect(pane(page, "prefs")).toHaveAttribute("data-opened", "");
  await choosePets(learner, "lively");
  await press(false, "off");
  await expect(pane(page, "prefs").getByRole("group", { name: PETS_LABEL[learner.locale], exact: true }).getByRole("button").nth(CHOICES.indexOf("off"))).toHaveAttribute("aria-pressed", "true");
  await page.keyboard.press("Escape");
  await expect(pane(page, "prefs")).not.toHaveAttribute("data-opened", "");

  const quiz = QUIZZES[0]!;
  await playQuiz(learner, quiz.id);
  await expect(toggle).not.toBeChecked();
  await press(true, "lively");
  await press(false, "off");
  await answerRun(learner, quiz, "perfect");
  await submitRun(learner);
  await expect(screen(page, "results")).toBeVisible();
  await expect(toggle).not.toBeChecked();
  await press(true, "lively");

  await page.reload();
  await expect(app).toHaveAttribute("data-pets", "lively");
  await expect(toggle).toBeChecked();
});

test("a pet lifts a copy of a task of a quiz's page out of its place, beside its transparent original; the original is back at once on hover and on focus, and the pet is thrown off", async ({ device }) => {
  const learner = await device("en");
  const page = learner.page;
  await page.addInitScript(hasten, { attribute: TEMPO, tempo: MISCHIEF_HASTE });
  await enterWithPets(learner);
  await page.evaluate(() => (window.location.hash = "prefs"));
  await expect(pane(page, "prefs")).toHaveAttribute("data-opened", "");
  await choosePets(learner, "lively");
  await page.evaluate((id) => (window.location.hash = id), MISCHIEF_QUIZ);
  await expect(pane(page, MISCHIEF_QUIZ)).toHaveAttribute("data-opened", "");
  await instrumented(page);
  const lifted = page.locator("[data-pet-prop]").and(page.locator('[style*="opacity: 0"]'));
  const copy = layer(learner).locator(':scope > [inert][aria-hidden="true"]');
  for (const reclaim of ["hover", "focus"] as const) {
    await page.mouse.move(4, 4);
    await expect(lifted, `a task row lifted, to be taken back by ${reclaim}`).toHaveCount(1, { timeout: 2 * RECAST_MS });
    await expect(copy).toHaveCount(1);
    const original = await lifted.boundingBox();
    expect(original, "the original keeps its box").not.toBeNull();
    await expect.poll(() => copy.boundingBox().then((shown) => (shown === null ? 0 : Math.hypot(shown.x - original!.x, shown.y - original!.y))), { message: "the copy stands out of the original's place", timeout: ANSWER_MS }).toBeGreaterThan(1);
    const [key, pets] = await Promise.all([lifted.getAttribute("data-pet-prop"), bodies(page)]);
    const pusher = pets.find((body) => body.activity === "push");
    expect(pusher, "a pet pushes the copy").toBeDefined();
    const pushed = (await trailOf(page, pusher!.id)).length - 1;
    if (reclaim === "hover") await page.mouse.move(original!.x + original!.width / 2, original!.y + original!.height / 2);
    else await lifted.evaluate((element) => (element.matches("a[href], button, input, select, textarea, [tabindex]") ? (element as HTMLElement) : (element.querySelector<HTMLElement>("a[href], button, input, select, textarea, [tabindex]") ?? Object.assign(element as HTMLElement, { tabIndex: -1 }))).focus());
    await expect(lifted, `the original is back at once on ${reclaim}`).toHaveCount(0, { timeout: ANSWER_MS });
    await expect(copy).toHaveCount(0, { timeout: ANSWER_MS });
    expect(key, "a task row of the opened quiz's page").toMatch(new RegExp(`^${MISCHIEF_QUIZ}/[^/]+$`));
    await expect.poll(async () => (await trailOf(page, pusher!.id)).slice(pushed).join(" → "), { message: `${pusher!.id} is thrown off`, timeout: ANSWER_MS }).toMatch(/^push\/[a-z]+\/[^ ]* → tumble\/air\//u);
    test.info().annotations.push({ type: "mischief", description: `${reclaim}: ${pusher!.id} pushed ${key}, then ${(await trailOf(page, pusher!.id)).slice(pushed).join(" → ")}` });
    if (reclaim === "focus") await page.evaluate(() => (document.activeElement as HTMLElement | null)?.blur());
  }
});

test("a pet that cannot hop to a perch above raises its ladder or shoots its grapple there: on a quiz's page a climber raises its ladder against the lowest card of the column, and on the home screen a grappler on the footer line reels itself up a rope to a card", async ({ device }) => {
  test.setTimeout(WALK_MS + ROPE_MS + 4 * RECAST_MS);
  const learner = await device("en");
  const page = learner.page;
  await page.addInitScript(hasten, { attribute: TEMPO, tempo: HASTE });
  await enterWithPets(learner);
  await page.evaluate(() => (window.location.hash = "prefs"));
  await expect(pane(page, "prefs")).toHaveAttribute("data-opened", "");
  await choosePets(learner, "lively");
  await page.keyboard.press("Escape");
  await expect(pane(page, "prefs")).not.toHaveAttribute("data-opened", "");
  const climber = (id: string): boolean => ["climb", "ladder"].every((gear) => KINDS[id]?.gear.includes(gear) === true);
  const gutters = QUIZZES.find((quiz) => castOf(quiz.id).core.some(climber));
  expect(gutters, "a quiz whose cast has a climber with a ladder in its core").toBeDefined();
  await page.evaluate((id) => (window.location.hash = id), gutters!.id);
  await expect(pane(page, gutters!.id)).toHaveAttribute("data-opened", "");
  await expectCast(learner, gutters!.id, true);
  await instrumented(page);
  const carried = (footing: string): Promise<readonly string[]> => page.evaluate((wanted) => Object.entries((window as unknown as { petWalk: Instruments }).petWalk.trails).flatMap(([id, trail]) => (trail.some((line) => line.split("/")[1] === wanted) ? [id] : [])), footing);
  await expect.poll(() => carried("ladder"), { message: `a pet on a ladder on the page of ${gutters!.id}`, timeout: WALK_MS }).not.toEqual([]);
  await page.keyboard.press("Escape");
  await expect(pane(page, gutters!.id)).not.toHaveAttribute("data-opened", "");
  await expectCast(learner, HOME, true);
  await instrumented(page);
  const grappling = (body: Body): boolean => KINDS[body.id]?.gear.includes("grapple") === true;
  const moved: string[] = [];
  const held: string[] = [];
  await expect(async () => {
    if ((await carried("rope")).length > 0) return;
    const footer = (await page.locator(".quiz-app > footer").boundingBox())!;
    const raised = (body: Body): number => footer.y - body.bottom - hoverOf(body.id);
    for (const body of (await bodies(page)).filter((candidate) => grappling(candidate) && candidate.footing === "perch" && raised(candidate) > 2)) {
      const pet = await reachable(page, (candidate) => candidate.id === body.id).catch(() => undefined);
      const spot = await page.evaluate(floorSpot, { parts: CARD_PARTS, controls: CONTROLS, reach: 70, room: 140 });
      if (pet === undefined || spot === null) continue;
      await carry(page, pet, { x: spot.x, y: spot.top - 100 });
      await page.mouse.up();
      moved.push(pet.id);
      await expectPerched(page, pet.id, LANDING_MS).catch(() => undefined);
    }
    const shown = (await bodies(page)).filter((body) => body.opacity > 0.99);
    const grounded = shown.filter((body) => grappling(body) && body.footing === "perch" && raised(body) <= 2);
    expect(grounded.length, `a grappler on the footer line among ${shown.map((body) => `${body.id} ${body.footing}`).join(", ")}`).toBeGreaterThan(0);
    const reach = Math.max(...grounded.map((body) => ROPE_REACH * KINDS[body.id]!.size.height));
    const sitting = (body: Body): boolean => !grappling(body) && raised(body) > 2 && raised(body) <= reach;
    const until = Date.now() + ROPE_ROUND_MS;
    let holding = false;
    try {
      while (Date.now() < until && (await carried("rope")).length === 0) {
        const sitter = await reachable(page, sitting).catch(() => undefined);
        if (sitter !== undefined) {
          if (holding) await page.mouse.up();
          await carry(page, sitter, { x: 50, y: footer.y / 2 });
          holding = true;
          held.push(sitter.id);
        }
        await page.evaluate(framesPass, 30);
      }
    } finally {
      if (holding) await page.mouse.up();
    }
    expect(await carried("rope"), "a grappler on the footer line reels itself up a rope").not.toEqual([]);
  }).toPass({ timeout: ROPE_MS, intervals: [0] });
  const [laddered, roped] = await Promise.all([carried("ladder"), carried("rope")]);
  test.info().annotations.push({ type: "gear", description: `on a ladder: ${laddered.join(", ")}; on a rope: ${roped.join(", ")}; taken to the footer line by the hand: ${moved.join(", ") || "nobody"}; held off a perch in reach of a rope: ${held.join(", ") || "nobody"}` });
  for (const id of roped) test.info().annotations.push({ type: "trail", description: `${id}: ${(await trailOf(page, id)).join(" → ")}` });
  expect(roped.every((id) => KINDS[id]?.gear.includes("grapple") === true), `only grapplers are on a rope: ${roped.join(", ")}`).toBe(true);
  expect(laddered.every((id) => KINDS[id]!.gear.some((gear) => gear !== "parachute")), `only pets with gear are on a ladder: ${laddered.join(", ")}`).toBe(true);
});

test("a phone shows at most two small pets, on edges it shows and never on a control", async ({ browser }, info) => {
  const context = await browser.newContext({ viewport: { width: 375, height: 812 }, isMobile: true, hasTouch: true, baseURL: info.project.use.baseURL, locale: "en-GB" });
  const page = await context.newPage();
  const problems: string[] = [];
  page.on("console", (message) => {
    if (message.type() === "error") problems.push(`console error: ${message.text()}`);
  });
  page.on("pageerror", (error) => problems.push(`page error: ${error.message}`));
  const phone: Device = { context, page, locale: "en", problems, expectingFailures: async () => 0 };
  const few = async (): Promise<void> => {
    const shown = await standing(page);
    expect(shown.length, "a newcomer waits until whoever it replaces is gone").toBeLessThanOrEqual(2);
    for (const pet of shown) expect(pet.size).toBe(0.8);
  };
  await enter(phone, { kind: "anonymous" });
  await expect(page.locator("[data-layered-overview]")).toHaveAttribute("data-mode", "list");
  await expect(pets(phone)).not.toHaveCount(0, { timeout: APPEAR_MS });
  await expect.poll(() => misplaced(page), { timeout: RECAST_MS }).toEqual([]);
  await few();

  const quiz = QUIZZES[0]!;
  await page.evaluate((id) => (window.location.hash = id), quiz.id);
  await expect(pane(page, quiz.id)).toHaveAttribute("data-opened", "");
  await expectCast(phone, quiz.id, false);
  await expect(pets(phone)).not.toHaveCount(0, { timeout: RECAST_MS });
  await expect.poll(() => misplaced(page), { timeout: RECAST_MS }).toEqual([]);
  await few();

  await page.keyboard.press("Escape");
  await expect(pane(page, quiz.id)).not.toHaveAttribute("data-opened", "");
  await card(page, quiz.id).scrollIntoViewIfNeeded();
  await playQuiz(phone, quiz.id);
  await expect.poll(() => misplaced(page), { timeout: RECAST_MS }).toEqual([]);
  await few();
  expect(await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth)).toBeLessThanOrEqual(0);
  await context.close();
  expect(problems).toEqual([]);
});
