/** 🐕️ The pets of the site in a real browser: they show up on the home screen within seconds, as decoration that is
 * hidden from assistive technology and never takes a click from a card or a control; the cast follows the learner
 * (the species of the quiz whose page or run is on screen, the home cast everywhere else) and rests during a run; the
 * learner's preference stops them (`still`) or removes them (`off`), a switch on the footer of every screen hides them
 * and brings them back, a device that asks for reduced motion decides the default only — its learner gets them
 * motionless and is told why until they choose, and calm pets they chose walk, also after a reload —, and a learner
 * whose device forces its own colours and therefore shows none is told why as well. They stand on
 * what the cards show — the tab, the edge of the body beside it, the footer line —, also for a learner who has just
 * arrived, keep their distance from each other and from a task, turn see-through while the pointer rests on them, and
 * on a phone at most two small ones show up. And they are alive the way the owner asked for it: their pupils follow
 * the pointer across the window and they turn round to it, they blink, they walk, and they meet each other. Every
 * device stays clean throughout: no console error, no failed request, no policy violation.
 *
 * An encounter takes a minute or two of real time (a first one 52 to 89 s in four lively sessions), too long and too
 * uncertain for a gate that runs every spec three times, and so does the first walk of calm pets. The two specs that
 * wait for a walk — lively pets that walk and meet, and the calm pets of a learner who chose them on a device that
 * asks for reduced motion — therefore set the client's test seam `data-pets-tempo="8"` on the document root before the
 * client starts: the pets' time then passes eight ticks a frame instead of about one. Nothing else differs, and no
 * other spec sets it.
 * @see ../../../🐾️pets/🔣️.json — the ensemble whose casts are expected on screen
 * @see ../../🎭️e2e/🚶️learner/🟦️.ts — the learner these specs drive
 * @see ../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🐾️pets/🟦️.tsx — the quiz's glue to the pets
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html — why `still` and `off` exist */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { Page } from "@playwright/test";
import { QUIZZES, answerRun, arrive, card, enter, expect, identify, pane, playQuiz, primary, readIntroduction, screen, submitRun, test, way, type Device, type Text } from "../../🎭️e2e/🚶️learner/🟦️.ts";

/** 🎟️ A cast as the ensemble authors it. */
interface Cast {
  readonly scene: string;
  readonly core: readonly string[];
  readonly rotation: readonly string[];
}

/** 📦️ What the walk needs of a species as authored: the box of its body and how high it floats above what it stands on. */
interface Build {
  readonly width: number;
  readonly height: number;
  readonly hover: number;
}

const MENAGERIE = resolve(dirname(fileURLToPath(import.meta.url)), "../../../🐾️pets");
const ENSEMBLE = JSON.parse(readFileSync(resolve(MENAGERIE, "🔣️.json"), "utf8")) as { readonly casts: readonly Cast[]; readonly species: readonly string[] };
const CASTS = ENSEMBLE.casts;
const SPECIES = ENSEMBLE.species.map((path) => JSON.parse(readFileSync(resolve(MENAGERIE, path), "utf8")) as { readonly id: string; readonly name: Text; readonly size: { readonly width: number; readonly height: number }; readonly locomotion: { readonly gait: string; readonly hover?: number } });
const BUILDS: Readonly<Record<string, Build>> = Object.fromEntries(SPECIES.map((species) => [species.id, { width: species.size.width, height: species.size.height, hover: species.locomotion.gait === "float" ? (species.locomotion.hover ?? 0) : 0 }]));

/** 🃏️ The parts of a card of the screen in front: its tab and its body. */
const CARD_PARTS = '#quiz-main [data-card] [data-slot="window-chrome-chip-cap"], #quiz-main [data-card] [data-slot="window-chrome-body-surface"]';

/** 🧱️ What pets stand on in the quiz, as the client names it for its pet layer: the parts of every card, and the
 * footer line. */
const EDGES = `${CARD_PARTS}, .quiz-app > footer`;

/** 🫥️ How visible a pet is while the pointer rests on it. */
const SEE_THROUGH = 0.35;

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

/** ⏩️ The client's test seam: the attribute of the document root that makes the pets' time pass faster. */
const TEMPO = "data-pets-tempo";

/** 🤝️ What two pets do when they meet. */
const ENCOUNTERS = ["greet", "cuddle", "squabble"] as const;

/** ⏱️ How long the first pet may take to show once the home screen is there. On an idle machine it is there within two
 * seconds; the gate runs four browsers against one cold dev server, which first has to transform the lazy chunk. */
const APPEAR_MS = 30_000;

/** 🚶️ How long a change of cast may take: the pets that leave walk off and fade, the new ones fade in. */
const RECAST_MS = 45_000;

/** 👁️ How long a blink may be waited for: every pet blinks every two to six seconds of its own time, which a busy
 * machine stretches. */
const BLINK_MS = 30_000;

/** 🧮️ How many animation frames a stillness is watched for: a second and a half on an idle machine. Frames, not
 * milliseconds: what must not happen is a write into the layer, and a frame is the only time one can happen. */
const STILL_FRAMES = 90;

/** 🕰️ How long a walk and an encounter may be waited for at eight times the pace: a first encounter took 52 to 89 s of
 * the pets' time in four lively sessions, seven to twelve seconds here. */
const MEET_MS = 120_000;

/** 📏️ How far apart, in pixels, the feet of two pets may be that act together: they come together on one edge, or act
 * across the gap between two neighbouring ones. */
const MEET_REACH = 700;

/** 🦶️ How far, in pixels, the bottom of a drawing may lie from the edge its pet stands on: a box the browser lays out
 * leaves the stroke out (a leg is a line three pixels thick, so its box ends a pixel and a half above the edge), and
 * the lean after the pointer lifts a foot by a fraction of a pixel. */
const FEET_SLACK = 3;

/** 🔭️ How far, in pixels of the screen, a pupil leaves the middle of its white at least — measured along the line from
 * the eye to the pointer, wherever the pointer is — when its pet looks at a pointer 150 px away or more: the gaze is
 * then at least four fifths of the way out, and a pupil travels 1.5 to 2.05 px to the outline at full size. Across
 * alone it may be far less: a pet on the footer line looks mostly up at a pointer in the middle of the window's edge. */
const PUPIL_TRAVEL = 0.8;

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

/** 🥾️ Checks that a pet walks: its feet move at least four pixels across while its depiction says it walks. */
async function expectWalk(device: Device): Promise<void> {
  const walked = await device.page.evaluate(watchWalk, MEET_MS);
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

/** 🧍️ A pet as painted: its feet, the size it is drawn at and how visible it is. */
interface Standing {
  readonly id: string;
  readonly x: number;
  readonly y: number;
  readonly size: number;
  readonly opacity: number;
}

/** 🗺️ Where every pet of a page stands. */
async function standing(page: Page): Promise<readonly Standing[]> {
  return page.locator(".pet-layer svg.pet").evaluateAll((drawn) =>
    drawn.map((pet) => {
      const place = /translate\(([-\d.]+)px, ([-\d.]+)px\) scale\(([-\d.]+), ([-\d.]+)\)/u.exec((pet as SVGElement).style.transform);
      return { id: pet.getAttribute("data-pet") ?? "", x: Number(place?.[1]), y: Number(place?.[2]), size: Number(place?.[4]), opacity: Number((pet as SVGElement).style.opacity || "1") };
    }),
  );
}

/** 🖼️ A pet as drawn: what its depiction says it does, the box of its drawing (the union of its parts), where its feet
 * are across, the way it is drawn (1 as authored, −1 mirrored), how visible it is, where the white of its first eye
 * is, where the pupil sits relative to the middle of that white — in pixels of the screen, so left is negative
 * whichever way the pet faces — and how far its lids leave its eyes open (1 open). */
interface Drawn {
  readonly id: string;
  readonly activity: string;
  readonly left: number;
  readonly top: number;
  readonly right: number;
  readonly bottom: number;
  readonly x: number;
  readonly flip: number;
  readonly opacity: number;
  readonly eye: { readonly x: number; readonly y: number } | null;
  readonly look: { readonly x: number; readonly y: number } | null;
  readonly open: number;
}

/** 🧲️ How far, in pixels of the screen, the first pupil of a pet has left the middle of its white towards a point: the
 * part of its offset that lies on the line from the eye to that point (negative when it looks away). */
function lookTowards(pet: Drawn, x: number, y: number): number {
  if (pet.eye === null || pet.look === null) return 0;
  const [across, down] = [x - pet.eye.x, y - pet.eye.y];
  const far = Math.hypot(across, down);
  return far === 0 ? 0 : (pet.look.x * across + pet.look.y * down) / far;
}

/** 🔬️ In the page: every pet as drawn, read from its depiction and from the boxes the browser lays out. */
function drawnPets(): Drawn[] {
  const middle = (element: Element): { x: number; y: number } => {
    const box = element.getBoundingClientRect();
    return { x: box.left + box.width / 2, y: box.top + box.height / 2 };
  };
  return [...document.querySelectorAll<SVGSVGElement>(".pet-layer svg.pet")].map((pet) => {
    const boxes = [...pet.children].map((part) => part.getBoundingClientRect()).filter((box) => box.width > 0 || box.height > 0);
    const place = /translate\(([-\d.]+)px, ([-\d.]+)px\) scale\(([-\d.]+), ([-\d.]+)\)/u.exec(pet.style.transform);
    const pupil = pet.querySelector(".pet-pupil");
    const white = pupil?.previousElementSibling ?? null;
    const squash = /scale\(1 ([\d.]+)\)/u.exec(pupil?.parentElement?.getAttribute("transform") ?? "");
    return {
      id: pet.getAttribute("data-pet") ?? "",
      activity: pet.getAttribute("data-pet-activity") ?? "",
      left: Math.min(...boxes.map((box) => box.left)),
      top: Math.min(...boxes.map((box) => box.top)),
      right: Math.max(...boxes.map((box) => box.right)),
      bottom: Math.max(...boxes.map((box) => box.bottom)),
      x: Number(place?.[1]),
      flip: Math.sign(Number(place?.[3])),
      opacity: Number(pet.style.opacity || "1"),
      eye: white === null ? null : middle(white),
      look: pupil === null || white === null ? null : { x: middle(pupil).x - middle(white).x, y: middle(pupil).y - middle(white).y },
      open: squash === null ? 1 : Number(squash[1]),
    };
  });
}

/** 🏎️ In the page, before any of its scripts: sets the client's tempo seam to eight on the document root — as soon as
 * the root exists, which it does not yet when an init script runs. */
function hasten(attribute: string): void {
  const mark = (): boolean => {
    document.documentElement?.setAttribute(attribute, "8");
    return document.documentElement !== null;
  };
  if (mark()) return;
  const watch = new MutationObserver(() => {
    if (mark()) watch.disconnect();
  });
  watch.observe(document, { childList: true });
}

/** 😑️ In the page: waits up to `limit` milliseconds for a blink — a lid that shuts and opens again — and answers whose
 * it was, or `null`. */
function watchBlink(limit: number): Promise<string | null> {
  return new Promise((resolve) => {
    const stage = document.querySelector(".pet-layer");
    if (stage === null) return resolve(null);
    const shut = new Set<Element>();
    const done = (id: string | null): void => {
      observer.disconnect();
      window.clearTimeout(timer);
      resolve(id);
    };
    const observer = new MutationObserver((records) => {
      for (const record of records) {
        const socket = record.target as Element;
        const squash = /scale\(1 ([\d.]+)\)/u.exec(socket.getAttribute("transform") ?? "");
        if (squash === null) continue;
        if (Number(squash[1]) < 0.5) shut.add(socket);
        else if (Number(squash[1]) === 1 && shut.has(socket)) return done(socket.closest("svg")?.getAttribute("data-pet") ?? "");
      }
    });
    const timer = window.setTimeout(() => done(null), limit);
    observer.observe(stage, { subtree: true, attributes: true, attributeFilter: ["transform"] });
  });
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

/** 🫂️ In the page: waits up to `limit` milliseconds for two pets whose depictions say at the same time that they
 * greet, cuddle or squabble, and answers who they are, what they do and how far apart their feet are, or `null`. */
function watchEncounter(limit: number): Promise<{ pair: string[]; activities: string[]; apart: number } | null> {
  return new Promise((resolve) => {
    const started = performance.now();
    const look = (): void => {
      const meeting = [...document.querySelectorAll<SVGSVGElement>(".pet-layer svg.pet")].filter((pet) => ["greet", "cuddle", "squabble"].includes(pet.getAttribute("data-pet-activity") ?? ""));
      if (meeting.length >= 2) {
        const feet = meeting.map((pet) => Number(/translate\(([-\d.]+)px/u.exec(pet.style.transform)?.[1]));
        return resolve({ pair: meeting.map((pet) => pet.getAttribute("data-pet") ?? ""), activities: meeting.map((pet) => pet.getAttribute("data-pet-activity") ?? ""), apart: Math.abs(feet[0]! - feet[1]!) });
      }
      if (performance.now() - started > limit) return resolve(null);
      window.requestAnimationFrame(look);
    };
    look();
  });
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

/** 👣️ What is wrong with where the pets of a page stand, one line each; none when every pet that is whole stands with
 * its feet on an edge the screen shows (the top of a tab, of a body or of the footer; a floater its hover above it),
 * no two bodies on one edge overlap, and no body covers a control or stands in front of a card. */
async function misplaced(page: Page): Promise<readonly string[]> {
  const shown = (await standing(page)).filter((pet) => pet.opacity > 0.99 || pet.opacity === SEE_THROUGH);
  const edges = (await page.evaluate(seenBoxes, EDGES)).filter((edge) => edge.topped);
  const kept = await page.evaluate(seenBoxes, `a[href], button, input, select, textarea, summary, ${CARD_PARTS}`);
  const wrong: string[] = [];
  for (const pet of shown) {
    const build = BUILDS[pet.id];
    if (build === undefined) {
      wrong.push(`${pet.id} is no species of the ensemble`);
      continue;
    }
    const feet = pet.y + build.hover * pet.size;
    const half = (build.width * pet.size) / 2;
    if (!edges.some((edge) => Math.abs(edge.top - feet) <= 1.5 && pet.x >= edge.left - 1 && pet.x <= edge.right + 1)) wrong.push(`${pet.id} at ${pet.x}, ${pet.y} stands on no edge`);
    for (const other of shown) {
      const mate = BUILDS[other.id];
      if (mate === undefined || other.id <= pet.id) continue;
      if (Math.abs(other.y + mate.hover * other.size - feet) <= 1.5 && Math.abs(other.x - pet.x) < ((build.width + mate.width) / 2) * pet.size - 1) wrong.push(`${pet.id} stands in ${other.id}`);
    }
    for (const box of kept) if (Math.min(pet.x + half, box.right) - Math.max(pet.x - half, box.left) > 1 && Math.min(pet.y, box.bottom) - Math.max(pet.y - build.height * pet.size, box.top) > 1) wrong.push(`${pet.id} at ${pet.x}, ${pet.y} covers something it must keep clear of`);
  }
  return wrong;
}

test("pets live on the home screen as decoration that never takes a click", async ({ device }) => {
  const learner = await device("en");
  await enterWithPets(learner);
  await expect(learner.page.locator(".quiz-app")).toHaveAttribute("data-pets", "calm");
  await expect(layer(learner)).toHaveCount(1);
  await expect(layer(learner)).toHaveAttribute("aria-hidden", "true");
  await expect(layer(learner)).toHaveCSS("pointer-events", "none");
  await expect(layer(learner).locator("a, button, input, select, textarea, summary, [tabindex], [role], [id], style, script")).toHaveCount(0);
  await expectCast(learner, HOME, true);
  await expectMoving(learner);
  expect(await takenTargets(learner)).toEqual([]);

  await primary(card(learner.page, "board")).click();
  await expect(pane(learner.page, "board")).toHaveAttribute("data-opened", "");
  expect(await takenTargets(learner)).toEqual([]);
  await way(learner.page, "overview").click();
  await expect(pane(learner.page, "board")).not.toHaveAttribute("data-opened", "");
  await expect(pets(learner)).not.toHaveCount(0, { timeout: APPEAR_MS });
  expect(await learner.page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth)).toBeLessThanOrEqual(0);
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
  return `${CAST_LINE[device.locale]}${SPECIES.filter((species) => shown.includes(species.id)).map((species) => species.name[device.locale]).join(" · ")}`;
}

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

test("a device that asks for reduced motion decides the default only: its learner gets motionless pets until they choose, and calm ones that walk from then on, also after a reload", async ({ device }) => {
  const learner = await device("de");
  const page = learner.page;
  const app = page.locator(".quiz-app");
  const preferences = pane(page, "prefs");
  const note = preferences.getByText(REDUCED_NOTE[learner.locale], { exact: true });
  const choices = preferences.getByRole("group", { name: PETS_LABEL[learner.locale], exact: true }).getByRole("button");
  const pressed = (choice: (typeof CHOICES)[number]): Promise<void> => expect(choices.nth(CHOICES.indexOf(choice))).toHaveAttribute("aria-pressed", "true");
  await page.addInitScript(hasten, TEMPO);
  await page.emulateMedia({ reducedMotion: "reduce" });
  await enterWithPets(learner);
  await expect(page.locator("html")).toHaveAttribute(TEMPO, "8");
  await expect(app).toHaveAttribute("data-pets", "still");
  await expectCast(learner, HOME, true);
  await expectMotionless(learner);
  await page.mouse.move(400, 300, { steps: 5 });
  await page.mouse.move(900, 600, { steps: 5 });
  await expectMotionless(learner);
  expect(await takenTargets(learner)).toEqual([]);
  await page.evaluate(() => (window.location.hash = "prefs"));
  await expect(preferences).toHaveAttribute("data-opened", "");
  await expect(note).toBeVisible();
  await pressed("still");

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

test("pets stand on what the cards show, keep their distance and stay beside a task", async ({ device }) => {
  const learner = await device("en");
  await enterWithPets(learner);
  await expect.poll(() => misplaced(learner.page), { timeout: RECAST_MS }).toEqual([]);
  await learner.page.reload();
  await expect(learner.page.locator("[data-layered-overview]")).toBeVisible();
  await expect(pets(learner)).not.toHaveCount(0, { timeout: APPEAR_MS });
  await expect.poll(() => misplaced(learner.page), { timeout: RECAST_MS }).toEqual([]);
  const footer = await learner.page.locator(".quiz-app > footer").evaluate((line) => line.getBoundingClientRect().top);
  await expect
    .poll(async () => (await standing(learner.page)).filter((pet) => pet.opacity > 0.99 && pet.y < footer - 40).length, { message: "a returning learner finds pets on the cards, not only on the footer line", timeout: RECAST_MS })
    .toBeGreaterThan(0);
  for (let look = 0; look < 10; look++) {
    await learner.page.evaluate(framesPass, 30);
    expect((await misplaced(learner.page)).filter((line) => line.includes(" stands in "))).toEqual([]);
  }

  const played = QUIZZES[0]!;
  await playQuiz(learner, played.id);
  await expectCast(learner, played.id, false);
  await expect.poll(() => misplaced(learner.page), { timeout: RECAST_MS }).toEqual([]);
  for (let look = 0; look < 6; look++) {
    await learner.page.evaluate(framesPass, 30);
    expect((await misplaced(learner.page)).filter((line) => !line.includes(" stands on no edge")), "nobody stands in anybody or covers the task, also while the screen still settles").toEqual([]);
  }
  await expect.poll(() => misplaced(learner.page), { timeout: RECAST_MS }).toEqual([]);
  expect(await takenTargets(learner)).toEqual([]);
});

test("a pet turns see-through while the pointer rests on it and whole again when the pointer leaves", async ({ device }) => {
  const learner = await device("en");
  await enterWithPets(learner);
  await expectCast(learner, HOME, true);
  let chosen = "";
  await expect(async () => {
    const pet = (await standing(learner.page)).find((candidate) => candidate.opacity > 0.99);
    expect(pet).toBeDefined();
    chosen = pet!.id;
    await learner.page.mouse.move(pet!.x, pet!.y - (BUILDS[chosen]!.height * pet!.size) / 2, { steps: 4 });
    await expect.poll(async () => (await standing(learner.page)).find((candidate) => candidate.id === chosen)?.opacity, { timeout: 5_000 }).toBe(SEE_THROUGH);
  }).toPass({ timeout: RECAST_MS });
  expect((await standing(learner.page)).filter((pet) => pet.id !== chosen && pet.opacity === SEE_THROUGH)).toEqual([]);
  await learner.page.mouse.move(5, 5, { steps: 4 });
  await expect.poll(async () => (await standing(learner.page)).find((candidate) => candidate.id === chosen)?.opacity ?? 1).toBe(1);
  expect(await takenTargets(learner)).toEqual([]);
});

test("a learner who has just arrived finds the pets standing on the cards: the bottom of a drawing meets the top of a tab or of a body", async ({ device }) => {
  const learner = await device("en");
  await enterWithPets(learner);
  const page = learner.page;
  const onCards = async (): Promise<readonly string[]> => {
    const tops = (await page.evaluate(seenBoxes, CARD_PARTS)).filter((part) => part.topped);
    return (await page.evaluate(drawnPets)).filter((pet) => pet.opacity > 0.99 && BUILDS[pet.id]?.hover === 0 && tops.some((part) => pet.x >= part.left && pet.x <= part.right && Math.abs(pet.bottom - part.top) <= FEET_SLACK)).map((pet) => pet.id);
  };
  const onFooter = async (): Promise<number> => {
    const line = await page.locator(".quiz-app > footer").evaluate((footer) => footer.getBoundingClientRect().top);
    return (await standing(page)).filter((pet) => Math.abs(pet.y + (BUILDS[pet.id]?.hover ?? 0) * pet.size - line) <= 1.5).length;
  };
  await expect.poll(async () => (await onCards()).length, { message: "walkers whose drawing ends on the top edge of a tab or of a body", timeout: APPEAR_MS }).toBeGreaterThanOrEqual(2);
  await expect.poll(onFooter, { message: "the company does not stay parked on the footer line", timeout: APPEAR_MS }).toBeLessThanOrEqual(2);
  await expect.poll(() => misplaced(page), { timeout: RECAST_MS }).toEqual([]);
});

test("pets that stand still follow the pointer across the window with their pupils and turn round to it", async ({ device }) => {
  const learner = await device("en");
  await enterWithPets(learner);
  await expectCast(learner, HOME, true);
  const page = learner.page;
  const attentive = async (x: number, y: number, side: 1 | -1): Promise<string> => {
    const watching = (await page.evaluate(drawnPets)).filter((pet) => pet.activity === "idle" && pet.opacity > 0.99 && pet.look !== null && Math.abs(pet.x - x) > 150);
    if (watching.length === 0) return "nobody stands still";
    return watching.filter((pet) => !(lookTowards(pet, x, y) >= PUPIL_TRAVEL && pet.look!.x * side > 0 && pet.flip === side)).map((pet) => `${pet.id} looks ${lookTowards(pet, x, y).toFixed(2)} towards the pointer (${pet.look!.x.toFixed(2)} across) and faces ${pet.flip}`).join("; ");
  };
  let nudge = 0;
  for (const [x, side] of [[8, -1], [1432, 1], [8, -1]] as const) {
    await expect(async () => {
      nudge = (nudge + 1) % 5;
      const y = 440 + 4 * nudge;
      await page.mouse.move(x, y, { steps: 6 });
      await expect.poll(() => attentive(x, y, side), { message: `every pet that stands still looks at the pointer at ${x} and faces it`, timeout: 3_000, intervals: [100] }).toBe("");
    }).toPass({ timeout: RECAST_MS });
  }
  await expect(async () => {
    nudge = (nudge + 1) % 5;
    const x = 700 + 4 * nudge;
    await page.mouse.move(x, 4, { steps: 6 });
    await expect
      .poll(async () => (await page.evaluate(drawnPets)).filter((pet) => pet.activity === "idle" && pet.opacity > 0.99 && pet.look !== null && pet.top > 200 && Math.abs(pet.x - 710) < 200 && !(lookTowards(pet, x, 4) >= PUPIL_TRAVEL && pet.look.y < 0)).map((pet) => pet.id), { message: "pets under the pointer look up", timeout: 3_000, intervals: [100] })
      .toEqual([]);
  }).toPass({ timeout: RECAST_MS });
  expect(await takenTargets(learner)).toEqual([]);
});

test("pets blink: within a few seconds a lid shuts and opens again", async ({ device }) => {
  const learner = await device("en");
  await enterWithPets(learner);
  expect((await learner.page.evaluate(drawnPets)).every((pet) => pet.open > 0 && pet.open <= 1)).toBe(true);
  const blinked = await learner.page.evaluate(watchBlink, BLINK_MS);
  expect(blinked, `a blink within ${BLINK_MS} ms`).not.toBeNull();
  expect(await shownPets(learner)).toContain(blinked);
});

test("lively pets walk along their edges and meet each other", async ({ device }) => {
  const learner = await device("en");
  const page = learner.page;
  await page.addInitScript(hasten, TEMPO);
  await enterWithPets(learner);
  await expect(page.locator("html")).toHaveAttribute(TEMPO, "8");
  await page.evaluate(() => (window.location.hash = "prefs"));
  await expect(pane(page, "prefs")).toHaveAttribute("data-opened", "");
  await choosePets(learner, "lively");
  await page.keyboard.press("Escape");
  await expect(pane(page, "prefs")).not.toHaveAttribute("data-opened", "");
  await page.mouse.move(4, 4);
  await expectCast(learner, HOME, false);

  await expectWalk(learner);

  const met = await page.evaluate(watchEncounter, MEET_MS);
  expect(met, "two pets that greet, cuddle or squabble at the same time").not.toBeNull();
  expect(new Set(met!.pair).size).toBe(2);
  for (const activity of met!.activities) expect(ENCOUNTERS).toContain(activity);
  expect(met!.apart).toBeLessThan(MEET_REACH);
  expect(await takenTargets(learner)).toEqual([]);
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
  await expect(pets(learner)).not.toHaveCount(0, { timeout: RECAST_MS });
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
