/** 📖️ The stories gallery of `@semio-tech/pets-react`: a dev page where the owner and the artists judge pets in motion, never part of a release.
 *
 * Two views of one menagerie. *Species* is a page per species, every scene on a light and on a dark ground: at rest
 * and with every clip looping; every state in its tint, with the overlay clip of its look over the breathing idle loop
 * and the particles of its emitter; every trick (its clip replayed, the particles it throws, in the tint of the state
 * it starts from) and the purr; and every clip of getting around and being handled with what goes with it — hanging
 * in the hand with the drawing swinging about the scruff, tumbling, gliding under the species' canopy, aiming the
 * grappling gun, reeling on a rope to its hook, climbing and sliding against a wall line, mantling over a corner,
 * climbing a standing ladder, carrying a ladder, pushing a block, being dizzy, shrugging and scooting. Poses come from
 * the core (`sampleClip`, `solveRig`, `faceOf`, `particlesOf`), the drawing from the product (`depict`, `paint` and the
 * layer's scenery for tools, ladders and particles); mood, lids, mirror, speed and size apply to every specimen and the
 * pupils follow the pointer. Only specimens in view are built and painted.
 *
 * *Sandbox* is a page of mock cards — side walls with gutters between them and a page gutter around them, a task card
 * whose rows a pet may play with (`data-pet-prop`, keyed with the grounds of the cast) — on which the cast of a scene
 * lives through the real pet layer, with the host's switches (scene, mode, quiet, play, mischief, capacity, scale, seed,
 * tempo), the learner's hand, the deeds of the keyboard and an inspector. Through the director the dev server puts
 * between the layer and the core (`../🏗️builder/🌐️vite/🟦️.ts`) the inspector reads every frame — poofs, overlapping
 * bodies (there must never be one), particles, rate — and folds forced states, tricks, moods and activities into the
 * stage by the stage's own rules; footings are reached the way the learner reaches them, by pressing, carrying and
 * letting go of a pet. The last scene of every species page is the poof, the dust it leaves where it vanishes.
 *
 * The document hands over whatever the menagerie file exports; the first valid menagerie found in it is shown. Every
 * label exists in English and German, and the page starts in the browser's language.
 *
 * @see ./🌐️.html — the document
 * @see ../🏗️builder/🌐️vite/🟦️.ts — the dev server, the menagerie module and the director
 * @see ../🔨️modules/🖌️depiction/🟦️.ts — the renderer under judgement
 * @see ../🔨️modules/🫧️layer/🟦️.tsx — the layer and its scenery
 */

import { StrictMode, createContext, useContext, useEffect, useMemo, useRef, useState, useSyncExternalStore, type ReactElement, type ReactNode } from "react";
import { createRoot } from "react-dom/client";
import { ACTIVITIES, DEEDS, MENAGERIE_SCHEMA, MOODS, MUZZLE_FORWARD, MUZZLE_HEIGHT, PET_MODES, TICKS_PER_SECOND, clipTicks, emitterKey, faceOf, lidAt, lifeTicks, lookOffset, menagerieIssues, overlaps, particlesOf, pupilReach, restPose, sampleClip, solveRig, type Activity, type ActorFrame, type BonePose, type Clip, type Deed, type Emitter, type Footing, type Frame, type Issue, type LadderFrame, type Menagerie, type Mood, type PetMode, type Point, type Pose, type Slug, type Species, type Stage, type StageEvent, type ToolFrame, type Turns } from "@semio-tech/pets";
import { CHUTE_DOME, CHUTE_RISE, CHUTE_SPAN, PetLayer, canopyRim, depict, paint, petCast, stageScenery, type EffectParticle, type PetLayerHandle } from "@semio-tech/pets-react";
import { AIM_RISE, CARRY_LEAN, CARRY_LENGTH } from "../../../🔨️modules/🎥️projection/🟦️.ts";
import { WALL_LEAN } from "../../../🔨️modules/📏️spacing/🟦️.ts";
import { PURR_TICKS, hail, shrug } from "../../../🔨️modules/👀️attention/🟦️.ts";
import { PUFF_TICKS } from "../../../🔨️modules/🚶️locomotion/🟦️.ts";
import { clipAt, clipOf, draftOf, enter, indexOf, perform, purr, release, sealed, settle, shift, trickOf, type Draft } from "../../../🔨️modules/📝️draft/🟦️.ts";
import "./🎨️.css";

//#region 🔖️Language
const LABELS = {
  en: {
    title: "Pet stories",
    species: "Species",
    sandbox: "Sandbox",
    language: "Language",
    dark: "Dark page",
    mood: "Mood",
    intensity: "Intensity",
    lid: "Lid",
    blinking: "Blink",
    mirrored: "Face left",
    speed: "Speed",
    zoom: "Size",
    rest: "rest",
    light: "light ground",
    darkGround: "dark ground",
    loop: "loop",
    once: "once",
    unused: "no activity",
    gait: "gait",
    hover: "hover",
    grip: "grip",
    reach: "reach",
    gear: "gear",
    none: "none",
    canopy: "own canopy",
    rests: "rests",
    counts: "states · tricks · emitters",
    clips: "At rest and every clip",
    states: "States: tint, look and particles",
    tricks: "Tricks and the purr",
    around: "Getting around and being handled",
    noClips: "No clip in the repertoire for",
    missingClip: "missing clip",
    missingEmitter: "missing emitter",
    tint: "tint",
    noTint: "no tint",
    lasts: "lasts",
    purr: "purr",
    missing: "No file at",
    empty: "No valid menagerie in",
    issues: "Issues",
    scene: "Scene",
    mode: "Mode",
    off: "off",
    quiet: "Quiet",
    play: "Play",
    mischief: "Mischief",
    capacity: "Capacity",
    scale: "Scale",
    seed: "Seed",
    reseed: "New seed",
    tempo: "Tempo",
    scroll: "Scroll the cards",
    rearrange: "Rearrange the cards",
    remove: "Take a card away",
    restore: "Put the card back",
    card: "Card",
    cardText: "Pets walk along the top edge of a card and climb its sides. Text and controls are kept free.",
    action: "A button",
    note: "A note on the edge",
    shelf: "A scrolling shelf",
    task: "Task",
    taskText: "Rows a pet may play with:",
    noProps: "The species of this scene name no grounds.",
    hand: "The hand",
    handHint: "Press on a pet and move to pick it up by its scruff; let go to throw it — from high up its parachute opens. Escape gives it back. Click a pet: hello, then its tricks, then a purr, then enough. Circle the pointer round a pet, clockwise or counter-clockwise, or stroke back and forth over it.",
    inspect: "Inspect a pet",
    pet: "Pet",
    nobody: "Nobody on stage yet.",
    state: "State",
    trick: "Trick",
    activity: "Activity",
    footing: "Footing",
    enter: "Enter",
    perform: "Perform",
    feel: "Feel",
    act: "Do it",
    deeds: "Deeds",
    deedHello: "Hello",
    deedTrick: "Trick",
    deedPet: "Pet it",
    deedToss: "Toss",
    pickUp: "Pick up (hand)",
    drop: "Drop (air)",
    dropHigh: "Drop from high (chute)",
    dropOnHead: "Drop on a neighbour (head)",
    routes: "Walls, ladders and ropes are taken by the pets on their own routes.",
    readout: "Live",
    tick: "tick",
    rate: "rate",
    steps: "frames per second",
    poofs: "poofs",
    overlaps: "overlaps now",
    overlapped: "frames with an overlap",
    frames: "frames",
    particles: "particles alive",
    puffs: "dust clouds",
    ladders: "ladders standing",
    lifts: "lifted fixtures",
    held: "held",
    unseen: "The stage has not drawn a frame yet.",
    applied: "applied",
    refused: "not now: it is not on a perch",
  },
  de: {
    title: "Tierchengeschichten",
    species: "Arten",
    sandbox: "Sandkasten",
    language: "Sprache",
    dark: "Dunkle Seite",
    mood: "Stimmung",
    intensity: "Stärke",
    lid: "Lid",
    blinking: "Blinzeln",
    mirrored: "Nach links schauen",
    speed: "Tempo",
    zoom: "Größe",
    rest: "Ruhe",
    light: "heller Grund",
    darkGround: "dunkler Grund",
    loop: "Schleife",
    once: "einmal",
    unused: "keine Aktivität",
    gait: "Gangart",
    hover: "Schwebehöhe",
    grip: "Griff",
    reach: "Reichweite",
    gear: "Ausrüstung",
    none: "keine",
    canopy: "eigener Schirm",
    rests: "ruht",
    counts: "Zustände · Kunststücke · Quellen",
    clips: "In Ruhe und jeder Clip",
    states: "Zustände: Tönung, Aussehen und Partikel",
    tricks: "Kunststücke und das Schnurren",
    around: "Unterwegs und in der Hand",
    noClips: "Kein Clip im Repertoire für",
    missingClip: "fehlender Clip",
    missingEmitter: "fehlende Quelle",
    tint: "Tönung",
    noTint: "keine Tönung",
    lasts: "hält",
    purr: "Schnurren",
    missing: "Keine Datei unter",
    empty: "Keine gültige Menagerie in",
    issues: "Befunde",
    scene: "Szene",
    mode: "Modus",
    off: "aus",
    quiet: "Ruhe",
    play: "Spielen",
    mischief: "Unfug",
    capacity: "Plätze",
    scale: "Maßstab",
    seed: "Startwert",
    reseed: "Neuer Startwert",
    tempo: "Zeitraffer",
    scroll: "Karten rollen",
    rearrange: "Karten umstellen",
    remove: "Eine Karte wegnehmen",
    restore: "Karte zurücklegen",
    card: "Karte",
    cardText: "Tierchen laufen auf der Oberkante einer Karte und klettern an ihren Seiten. Text und Bedienelemente bleiben frei.",
    action: "Ein Knopf",
    note: "Eine Notiz auf der Kante",
    shelf: "Ein rollendes Regal",
    task: "Aufgabe",
    taskText: "Zeilen, mit denen ein Tierchen spielen darf:",
    noProps: "Die Arten dieser Szene nennen keine Themen.",
    hand: "Die Hand",
    handHint: "Drück auf ein Tierchen und beweg den Zeiger, um es am Nackenfell hochzuheben; lass los, um es zu werfen — von weit oben öffnet sich sein Fallschirm. Escape gibt es zurück. Klick ein Tierchen an: Hallo, dann seine Kunststücke, dann ein Schnurren, dann ist es genug. Kreise mit dem Zeiger um ein Tierchen, im oder gegen den Uhrzeigersinn, oder streichle hin und her darüber.",
    inspect: "Ein Tierchen untersuchen",
    pet: "Tierchen",
    nobody: "Noch niemand auf der Bühne.",
    state: "Zustand",
    trick: "Kunststück",
    activity: "Tätigkeit",
    footing: "Halt",
    enter: "Eintreten",
    perform: "Vorführen",
    feel: "Fühlen",
    act: "Tun",
    deeds: "Taten",
    deedHello: "Hallo",
    deedTrick: "Kunststück",
    deedPet: "Streicheln",
    deedToss: "Werfen",
    pickUp: "Hochheben (Hand)",
    drop: "Fallen lassen (Luft)",
    dropHigh: "Von weit oben fallen lassen (Fallschirm)",
    dropOnHead: "Auf einen Nachbarn fallen lassen (Kopf)",
    routes: "Wände, Leitern und Seile nehmen die Tierchen auf eigenen Wegen.",
    readout: "Live",
    tick: "Tick",
    rate: "Rate",
    steps: "Bilder pro Sekunde",
    poofs: "Verpuffungen",
    overlaps: "Überlappungen jetzt",
    overlapped: "Bilder mit Überlappung",
    frames: "Bilder",
    particles: "lebende Partikel",
    puffs: "Staubwolken",
    ladders: "stehende Leitern",
    lifts: "angehobene Elemente",
    held: "gehalten",
    unseen: "Die Bühne hat noch kein Bild gezeichnet.",
    applied: "angewandt",
    refused: "nicht jetzt: es sitzt nicht auf einem Platz",
  },
} as const;

/** 🗣️ The languages of the gallery, English first. */
type Language = keyof typeof LABELS;

/** 🏷️ Every label of the gallery in one language. */
type Labels = { readonly [label in keyof (typeof LABELS)["en"]]: string };

const LANGUAGES = Object.keys(LABELS) as Language[];

/** 🧗️ The activities of getting around and being handled that the species pages show with what goes with them, in the order of the contract; `ladder` is the climb on a standing ladder. */
const GETTING_AROUND = ["hang", "tumble", "glide", "aim", "reel", "climb", "mantle", "slide", "carry", "push", "dizzy", "shrug", "scoot"] as const satisfies readonly Activity[];

/** 🛤️ A way of getting around that a species page shows, the climb on a standing ladder and the poof. */
type Way = (typeof GETTING_AROUND)[number] | "ladder" | "poof";

const WAYS: { readonly [language in Language]: { readonly [way in Way]: string } } = {
  en: {
    hang: "held at the scruff: the whole drawing swings about the grip (dot)",
    tumble: "thrown: the whole drawing turns about its middle",
    glide: "under its open parachute, hanging from the grip",
    aim: "with the grappling gun at its side",
    reel: "on the rope from its hands to the hook on a ledge",
    climb: "at a wall on the side it faces, leaning in",
    ladder: "on a ladder that stands against a wall",
    mantle: "over the corner of a wall onto its top",
    slide: "down a wall on the side it faces",
    carry: "with its ladder on its back",
    push: "against a block it shoves out of its stack",
    dizzy: "after a hard landing or a shake",
    shrug: "after a miss, or when it has had enough",
    scoot: "making room for a neighbour",
    poof: "the last resort: it vanishes in a puff of dust and arrives anew",
  },
  de: {
    hang: "am Nackenfell gehalten: die ganze Zeichnung schwingt um den Griff (Punkt)",
    tumble: "geworfen: die ganze Zeichnung dreht sich um ihre Mitte",
    glide: "unter dem offenen Fallschirm, am Griff hängend",
    aim: "mit der Seilpistole an der Seite",
    reel: "am Seil von den Händen zum Haken an einer Kante",
    climb: "an einer Wand auf der Seite, in die es schaut, angelehnt",
    ladder: "auf einer Leiter, die an einer Wand lehnt",
    mantle: "über die Ecke einer Wand auf ihre Oberkante",
    slide: "eine Wand hinunter, auf der Seite, in die es schaut",
    carry: "mit seiner Leiter auf dem Rücken",
    push: "gegen einen Block, den es aus seinem Stapel schiebt",
    dizzy: "nach einer harten Landung oder einem Schütteln",
    shrug: "nach einem Fehlschuss, oder wenn es genug hat",
    scoot: "macht einem Nachbarn Platz",
    poof: "der letzte Ausweg: es verschwindet in einer Staubwolke und kommt neu an",
  },
};

/** 🌍️ The first language the browser asks for that the gallery speaks; English when it asks for neither. */
function preferredLanguage(): Language {
  return navigator.languages.map((tag) => tag.slice(0, 2).toLowerCase()).find((tag): tag is Language => tag in LABELS) ?? "en";
}
//#endregion 🔖️Language

//#region 🔖️Menagerie
const SEARCH_DEPTH = 8;

/** 🎪️ What the menagerie file holds: the outermost document that calls itself a menagerie and passes validation — an export of a module, a JSON document or a member of one, such as the sample of the conformance vectors — and otherwise the issues of the outermost one that fails. */
function menagerieOf(source: unknown): { readonly menagerie: Menagerie | null; readonly issues: readonly Issue[] } {
  let issues: readonly Issue[] = [];
  let level: unknown[] = [source];
  for (let depth = 0; depth < SEARCH_DEPTH && level.length > 0; depth++) {
    const inner: unknown[] = [];
    for (const value of level) {
      if (typeof value !== "object" || value === null) continue;
      if ((value as { readonly schema?: unknown }).schema === MENAGERIE_SCHEMA && Object.prototype.toString.call(value) !== "[object Module]") {
        const found = menagerieIssues(value);
        if (found.length === 0) return { menagerie: value as Menagerie, issues: [] };
        if (issues.length === 0) issues = found;
      }
      inner.push(...Object.values(value));
    }
    level = inner;
  }
  return { menagerie: null, issues };
}
//#endregion 🔖️Menagerie

//#region 🔖️Studio
/** 🎚️ What the controls of the species view set for every specimen: the mood and how strongly it shows, how far the lids are down besides, whether the eyes blink, whether the pets face left, how fast the gallery's clock runs. */
type Controls = { readonly mood: Mood; readonly intensity: number; readonly lid: number; readonly blinking: boolean; readonly mirrored: boolean; readonly speed: number };

/** 🖌️ One specimen as the studio paints it: `paint` draws it at a tick of the gallery's clock for the pointer and the controls, its stage lying at `corner` in the viewport; `end` takes its drawing away. */
type Painting = { readonly paint: (ticks: number, pointer: Point | null, corner: Point, controls: Controls) => void; readonly end: () => void };

/** 🎬️ The clock of the species view: a stage that entered gets its painting built when it first comes into view, and painted on every animation frame while it is in view. */
type Studio = { readonly enter: (stage: Element, build: () => Painting) => () => void };

/** 🎞️ What a specimen shows: its clip (replayed after a pause when it does not loop; the rest pose without one) or, for a state, its look (`look`: the state's overlay clip over the breathing idle loop, `null` for the idle loop alone); the state whose tint it wears, the activity and the footing it names; the emitter whose particles it throws, running with the scene or once per replay; how high its feet are above the ground and the room it needs ahead, behind and above them (pet pixels); the tilt of its drawing over time and the pivot it turns about; the tools it holds and the ladders that stand beside it, over time and from where its feet are; what it leans on, hangs from or pushes (`prop`, with the wall `wall` pixels ahead of the feet, the ledge of a rope's hook at `anchor`); and whether it vanishes in a puff now and then (`poof`). */
type Scene = {
  readonly key: string;
  readonly caption: string;
  readonly clip: Clip | null;
  readonly look?: Clip | null;
  readonly state?: Slug;
  readonly activity?: Activity;
  readonly footing?: Footing;
  readonly emitter?: Emitter;
  readonly plume?: "running" | "once";
  readonly lift?: number;
  readonly room?: { readonly ahead?: number; readonly behind?: number; readonly up?: number };
  readonly tilt?: (ticks: number, facing: 1 | -1) => Turns;
  readonly pivot?: Point;
  readonly tools?: (ticks: number, feet: Point, facing: 1 | -1) => readonly ToolFrame[];
  readonly ladders?: (feet: Point, facing: 1 | -1) => readonly LadderFrame[];
  readonly prop?: "wall" | "corner" | "block" | "ledge" | "hand";
  readonly wall?: number;
  readonly anchor?: Point;
  readonly poof?: boolean;
};

/** 📐️ Where a specimen of a scene is drawn, in pet pixels: the size of its stage, where its feet are and where its ground line runs. */
type Layout = { readonly width: number; readonly height: number; readonly feet: Point; readonly ground: number };

const StudioContext = createContext<Studio | null>(null);
const MARGIN = 12;
const DOWN = 8;
const CLIP_ZOOM = 0.625;
const GAZE_REACH = 24;
const REPLAY_PAUSE = 40;
const BLINK_EVERY = 160;
const TRICK_TICKS = 2 * TICKS_PER_SECOND;
const POOF_SHOWN = 96;
const POOF_CYCLE = POOF_SHOWN + PUFF_TICKS + 16;
const SPARK_SEED = 1;
const GROUNDS = ["light", "dark"] as const;
const TONES = ["body", "accent", "detail"] as const;
const ORIGIN: Point = { x: 0, y: 0 };
const NO_COPIES: ReadonlySet<Element> = new Set();

/** ⏱️ Opens the clock: one animation frame loop, one pointer listener and one visibility observer for every specimen; the stages in view are measured first and painted after, so painting never forces a layout between two measurements. */
function openStudio(controls: () => Controls): { readonly studio: Studio; readonly close: () => void } {
  const builders = new Map<Element, () => Painting>();
  const built = new Map<Element, Painting>();
  const seen = new Set<Element>();
  const watcher = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) {
        if (entry.isIntersecting) seen.add(entry.target);
        else seen.delete(entry.target);
      }
    },
    { rootMargin: "96px" },
  );
  let pointer: Point | null = null;
  let ticks = 0;
  let before = performance.now();
  const point = (event: PointerEvent): void => {
    pointer = { x: event.clientX, y: event.clientY };
  };
  const unpoint = (): void => {
    pointer = null;
  };
  const frame = (now: number): void => {
    const current = controls();
    ticks += ((now - before) / 1000) * TICKS_PER_SECOND * current.speed;
    before = now;
    const shown: (readonly [Painting, Point])[] = [];
    for (const stage of seen) {
      let painting = built.get(stage);
      const build = builders.get(stage);
      if (painting === undefined && build !== undefined) {
        painting = build();
        built.set(stage, painting);
      }
      if (painting === undefined) continue;
      const box = pointer === null ? null : stage.getBoundingClientRect();
      shown.push([painting, box === null ? ORIGIN : { x: box.left, y: box.top }]);
    }
    for (const [painting, corner] of shown) painting.paint(Math.floor(ticks), pointer, corner, current);
    handle = requestAnimationFrame(frame);
  };
  let handle = requestAnimationFrame(frame);
  window.addEventListener("pointermove", point, { passive: true });
  document.documentElement.addEventListener("pointerleave", unpoint, { passive: true });
  return {
    studio: {
      enter(stage, build) {
        builders.set(stage, build);
        watcher.observe(stage);
        return () => {
          const painting = built.get(stage);
          built.delete(stage);
          builders.delete(stage);
          seen.delete(stage);
          watcher.unobserve(stage);
          painting?.end();
        };
      },
    },
    close() {
      cancelAnimationFrame(handle);
      watcher.disconnect();
      window.removeEventListener("pointermove", point);
      document.documentElement.removeEventListener("pointerleave", unpoint);
      for (const painting of built.values()) painting.end();
      built.clear();
    },
  };
}

/** 📏️ Where `scene` puts a specimen of `species` facing `facing`: its room ahead of it lies on the side it faces. */
function layoutOf(species: Species, scene: Scene, facing: 1 | -1): Layout {
  const { width, height } = species.size;
  const hover = species.locomotion.hover ?? 0;
  const lift = scene.lift ?? 0;
  const ahead = Math.max(width / 2 + MARGIN, scene.room?.ahead ?? 0);
  const behind = Math.max(width / 2 + MARGIN, scene.room?.behind ?? 0);
  const up = Math.max(height + hover + lift + MARGIN, scene.room?.up ?? 0);
  return { width: ahead + behind, height: up + DOWN, feet: { x: facing > 0 ? behind : ahead, y: up - hover - lift }, ground: up };
}

/** 🫁️ The pose a species breathes in at `ticks`: its first idle clip looping, else its rest pose. */
function breathing(species: Species, ticks: number): Pose {
  const idle = clipOf(species, species.repertoire.idle?.[0] ?? null);
  return idle === null ? restPose(species) : sampleClip(species, idle, ticks);
}

/** 🎭️ The look of a state over `under`: every channel of a bone the state's clip keys takes the clip's value at `ticks`, every other one keeps breathing — as the stage draws a state (design-v2 §24.1). */
function lookedOver(species: Species, under: Pose, look: Clip | null, ticks: number): Pose {
  if (look === null) return under;
  const over = sampleClip(species, look, ticks);
  const pose: { -readonly [channel in keyof BonePose]: BonePose[channel] }[] = under.map((bone) => ({ ...bone }));
  for (const track of look.tracks) {
    const index = species.bones.findIndex((bone) => bone.id === track.bone);
    if (index >= 0) pose[index]![track.channel] = over[index]![track.channel];
  }
  return pose;
}

/** 🫠️ A pose whose head sinks by `drop` pixels — the bone of the first eye —, as a mood carries it. */
function drooped(species: Species, pose: Pose, drop: number): Pose {
  const eye = species.face.eyes[0];
  if (eye === undefined || drop === 0) return pose;
  return pose.map((bone, index) => (species.bones[index]!.id === eye.bone ? { ...bone, y: bone.y + drop } : bone));
}

/** 🤸️ The pose of a scene at `ticks`: a state's look over the breathing loop, else its clip (a clip that does not loop replays after a pause), else the rest pose. */
function poseOf(species: Species, scene: Scene, ticks: number): Pose {
  if (scene.look !== undefined) return lookedOver(species, breathing(species, ticks), scene.look, ticks);
  const clip = scene.clip;
  if (clip === null) return restPose(species);
  return sampleClip(species, clip, clip.loop ? ticks : ticks % (clipTicks(clip) + REPLAY_PAUSE));
}

/** 🖼️ The frame of a specimen at `ticks`: its pose under the face of the chosen mood, pupils drawn towards the pointer, lids from the mood, the lid control and the blink; its tilt, tools, state, activity and footing from the scene. `feet` is where it stands in the units of its stage (pet pixels), `corner` where that stage is in the viewport, `zoom` how large it is drawn. */
function specimenFrame(species: Species, scene: Scene, ticks: number, feet: Point, corner: Point, pointer: Point | null, controls: Controls, zoom: number): ActorFrame {
  const facing: 1 | -1 = controls.mirrored ? -1 : 1;
  const face = faceOf({ mood: controls.mood, intensity: controls.intensity, since: 0 });
  const bones = solveRig(species, drooped(species, poseOf(species, scene, ticks), face.drop));
  const blink = controls.blinking ? lidAt(ticks % BLINK_EVERY) : 0;
  const lid = face.lid + (1 - face.lid) * Math.max(controls.lid, blink);
  const target = pointer === null ? null : { x: ((pointer.x - corner.x) / zoom - feet.x) * facing, y: (pointer.y - corner.y) / zoom - feet.y };
  const eyes = species.face.eyes.map((eye) => {
    const bone = species.bones.findIndex((candidate) => candidate.id === eye.bone) * 6;
    if (target === null || bone < 0) return { x: 0, y: 0, lid };
    const centre = { x: bones[bone]! * eye.x + bones[bone + 2]! * eye.y + bones[bone + 4]!, y: bones[bone + 1]! * eye.x + bones[bone + 3]! * eye.y + bones[bone + 5]! };
    const look = lookOffset(centre, target, GAZE_REACH);
    const reach = pupilReach(eye);
    return { x: look.x * reach, y: look.y * reach, lid };
  });
  const { width, height } = species.size;
  return {
    species: species.id,
    x: feet.x,
    y: feet.y,
    facing,
    activity: scene.activity ?? "idle",
    opacity: 1,
    bones,
    eyes,
    footing: scene.footing ?? "perch",
    state: scene.state ?? species.states[0]?.id ?? "",
    mood: controls.mood,
    intensity: controls.intensity,
    spirits: face.bend,
    tilt: scene.tilt?.(ticks, facing) ?? 0,
    pivot: scene.pivot ?? { x: 0, y: -species.grip },
    tools: scene.tools?.(ticks, feet, facing) ?? [],
    body: { x: feet.x - width / 2, y: feet.y - height, width, height },
  };
}

/** 🎇️ When the emitter of a scene runs at `ticks`, as `[since, until]` of each run whose particles may still be alive: a trick throws its particles once per replay of its clip, a running burst once per its life and a pause, every other running emitter streams from the start. */
function plumesOf(scene: Scene, emitter: Emitter, ticks: number): readonly (readonly [number, number | null])[] {
  const replayed = scene.plume === "once";
  if (!replayed && emitter.motion !== "burst") return [[0, null]];
  const span = replayed ? (scene.clip === null || scene.clip.loop ? TRICK_TICKS : clipTicks(scene.clip)) : 0;
  const cycle = (replayed ? span : lifeTicks(emitter)) + REPLAY_PAUSE;
  const round = Math.floor(ticks / cycle);
  return [round - 1, round].filter((each) => each >= 0).map((each) => [each * cycle, each * cycle + span] as const);
}

/** ✨️ The particles of a scene's emitter at `ticks`, from the emitter's point on its bone in `frame` (mirrored with the facing), in the units of the stage. */
function sparksOf(species: Species, scene: Scene, frame: ActorFrame, ticks: number): EffectParticle[] {
  const emitter = scene.emitter;
  if (emitter === undefined) return [];
  const bone = Math.max(0, species.bones.findIndex((candidate) => candidate.id === emitter.bone)) * 6;
  const matrix = frame.bones;
  const origin = { x: frame.x + frame.facing * (matrix[bone]! * emitter.x + matrix[bone + 2]! * emitter.y + matrix[bone + 4]!), y: frame.y + matrix[bone + 1]! * emitter.x + matrix[bone + 3]! * emitter.y + matrix[bone + 5]! };
  const index = species.emitters.indexOf(emitter);
  return plumesOf(scene, emitter, ticks).flatMap(([since, until]) =>
    particlesOf(emitter, origin, frame.facing, since, until, ticks, emitterKey(SPARK_SEED, 0, index, since)).map((particle) => ({ species: species.id, emitter: emitter.id, x: particle.x, y: particle.y, scale: particle.scale, rotation: particle.rotation, opacity: particle.opacity })),
  );
}

/** 💨️ The dust of a scene that poofs, at `ticks`: the pet shows for {@link POOF_SHOWN} ticks, then vanishes in a puff over its body that clears in `PUFF_TICKS`, and arrives anew a little later; nothing for any other scene. */
function puffsOf(species: Species, scene: Scene, feet: Point, ticks: number): readonly { readonly x: number; readonly y: number; readonly width: number; readonly height: number; readonly phase: number }[] {
  const at = ticks % POOF_CYCLE;
  if (scene.poof !== true || at < POOF_SHOWN || at >= POOF_SHOWN + PUFF_TICKS) return [];
  const { width, height } = species.size;
  return [{ x: feet.x, y: feet.y - height / 2, width, height, phase: (at - POOF_SHOWN) / PUFF_TICKS }];
}

/** 🏗️ Builds the drawing of a specimen in its stage `host`: the depiction, and — for a scene with tools, ladders, particles or a poof — the scenery of the pet layer, which equips it and draws ladders, dust and particles beside it. */
function drawing(host: HTMLElement, species: Species, scene: Scene, zoom: number): Painting {
  const depiction = depict(species, host.ownerDocument);
  host.append(depiction.element);
  const scenery = scene.tools !== undefined || scene.ladders !== undefined || scene.emitter !== undefined || scene.poof === true ? stageScenery(host, new Map([[species.id, species]]), NO_COPIES) : null;
  return {
    paint: (ticks, pointer, corner, controls) => {
      const facing: 1 | -1 = controls.mirrored ? -1 : 1;
      const { feet } = layoutOf(species, scene, facing);
      const frame = specimenFrame(species, scene, ticks, feet, corner, pointer, controls, zoom);
      const vanished = scene.poof === true && ticks % POOF_CYCLE >= POOF_SHOWN;
      paint(depiction, { ...frame, x: frame.x * zoom, y: frame.y * zoom, opacity: vanished ? 0 : 1 }, zoom);
      if (scenery === null) return;
      scenery.actor(depiction, frame);
      scenery.stage({ ladders: scene.ladders?.(feet, facing) ?? [], particles: sparksOf(species, scene, frame, ticks), puffs: puffsOf(species, scene, feet, ticks) }, zoom);
    },
    end: () => {
      scenery?.strike();
      depiction.element.remove();
    },
  };
}

/** 🧱️ What a specimen leans on, hangs from or pushes, in the pet pixels of its stage: a wall line, the corner of a wall, a block, the ledge of a rope's hook (all behind the pet), the hand that holds it (in front of the pet, where its scruff may lie inside the drawing). */
function Prop({ species, scene, layout, facing, zoom }: { readonly species: Species; readonly scene: Scene; readonly layout: Layout; readonly facing: 1 | -1; readonly zoom: number }): ReactElement | null {
  const { feet, ground, width, height } = layout;
  const ahead = (distance: number): number => feet.x + facing * distance;
  const side = species.size.width / 2 + 1;
  let shape: ReactElement | null = null;
  if (scene.prop === "wall") shape = <path d={`M ${ahead(scene.wall ?? side)} 0 V ${ground}`} />;
  else if (scene.prop === "corner") shape = <path d={`M ${ahead(side)} ${ground} V ${feet.y - species.size.height * 0.55} H ${facing > 0 ? width : 0}`} />;
  else if (scene.prop === "block") shape = <rect className="solid" x={facing > 0 ? ahead(side) : ahead(side) - 20} y={feet.y - 16} width={20} height={16} rx={3} />;
  else if (scene.prop === "ledge" && scene.anchor !== undefined) shape = <path d={`M ${ahead(scene.anchor.x) - 16} ${feet.y + scene.anchor.y} H ${ahead(scene.anchor.x) + 16}`} />;
  else if (scene.prop === "hand") shape = <circle className="hand" cx={feet.x + facing * (scene.pivot?.x ?? 0)} cy={feet.y + (scene.pivot?.y ?? -species.grip)} r={2.5} />;
  if (shape === null) return null;
  return (
    <svg className={scene.prop === "hand" ? "specimen-decor specimen-front" : "specimen-decor"} viewBox={`0 0 ${width} ${height}`} width={width * zoom} height={height * zoom} aria-hidden="true">
      {shape}
    </svg>
  );
}

/** 🧸️ One species in one scene on a stage of its own: its declared box, the ground it stands on (or hovers or hangs above), what it leans on, and the drawing the studio builds and paints while it is in view. */
function Specimen({ species, scene, zoom, facing }: { readonly species: Species; readonly scene: Scene; readonly zoom: number; readonly facing: 1 | -1 }): ReactElement {
  const studio = useContext(StudioContext);
  const stage = useRef<HTMLDivElement>(null);
  const layout = layoutOf(species, scene, facing);
  const { width, height } = species.size;
  useEffect(() => {
    const host = stage.current;
    if (!host || !studio) return undefined;
    return studio.enter(host, () => drawing(host, species, scene, zoom));
  }, [species, scene, zoom, studio]);
  return (
    <figure className="specimen" data-scene={scene.key}>
      <div ref={stage} className="specimen-stage" style={{ width: layout.width * zoom, height: layout.height * zoom }}>
        <div className="specimen-ground" style={{ top: layout.ground * zoom }} />
        <div className="specimen-box" style={{ left: (layout.feet.x - width / 2) * zoom, top: (layout.feet.y - height) * zoom, width: width * zoom, height: height * zoom }} />
        <Prop species={species} scene={scene} layout={layout} facing={facing} zoom={zoom} />
      </div>
      <figcaption>{scene.caption}</figcaption>
    </figure>
  );
}
//#endregion 🔖️Studio

//#region 🔖️Controls
/** 🎛️ A labelled range input that shows its value (or what `shown` calls it). */
function Slider({ label, value, min, max, step, shown, onChange }: { readonly label: string; readonly value: number; readonly min: number; readonly max: number; readonly step: number; readonly shown?: string; readonly onChange: (value: number) => void }): ReactElement {
  return (
    <label className="control">
      <span>{label}</span>
      <input type="range" value={value} min={min} max={max} step={step} onChange={(event) => onChange(Number(event.target.value))} />
      <output>{shown ?? value}</output>
    </label>
  );
}

/** ☑️ A labelled checkbox. */
function Toggle({ label, checked, onChange }: { readonly label: string; readonly checked: boolean; readonly onChange: (checked: boolean) => void }): ReactElement {
  return (
    <label className="control">
      <input type="checkbox" checked={checked} onChange={(event) => onChange(event.target.checked)} />
      <span>{label}</span>
    </label>
  );
}

/** 🔽️ A labelled select over `options` (value and what it is called). */
function Choice<Value extends string>({ label, value, options, onChange }: { readonly label: string; readonly value: Value; readonly options: readonly (readonly [Value, string])[]; readonly onChange: (value: Value) => void }): ReactElement {
  return (
    <label className="control">
      <span>{label}</span>
      <select value={value} onChange={(event) => onChange(event.target.value as Value)}>
        {options.map(([option, name]) => (
          <option key={option} value={option}>
            {name}
          </option>
        ))}
      </select>
    </label>
  );
}

/** 🔘️ A plain button. */
function Action({ label, onPress, pressed, disabled }: { readonly label: string; readonly onPress: () => void; readonly pressed?: boolean; readonly disabled?: boolean }): ReactElement {
  return (
    <button type="button" className="action" aria-pressed={pressed} disabled={disabled} onClick={onPress}>
      {label}
    </button>
  );
}
//#endregion 🔖️Controls

//#region 🔖️Species
/** 🎟️ The activities that play each clip of a species, by clip id. */
function clipRoles(species: Species): ReadonlyMap<string, readonly string[]> {
  const roles = new Map<string, string[]>();
  for (const activity of ACTIVITIES) for (const clip of species.repertoire[activity] ?? []) roles.set(clip, [...(roles.get(clip) ?? []), activity]);
  return roles;
}

/** 🎥️ A clip as a caption: its id, length and whether it loops. */
function clipCaption(clip: Clip, text: Labels): string {
  return `${clip.id} · ${clip.seconds} s · ${clip.loop ? text.loop : text.once}`;
}

/** 🧨️ An emitter as a caption: its id, motion and how many particles it keeps alive. */
function emitterCaption(emitter: Emitter): string {
  return `${emitter.id} (${emitter.motion} ×${emitter.count})`;
}

/** 📼️ The species at rest and with every clip looping, captioned with the activities that play it. */
function clipScenes(species: Species, text: Labels): Scene[] {
  const roles = clipRoles(species);
  return [{ key: "rest", caption: text.rest, clip: null }, ...species.clips.map((clip) => ({ key: `clip-${clip.id}`, caption: `${clipCaption(clip, text)} · ${roles.get(clip.id)?.join(", ") ?? text.unused}`, clip }))];
}

/** 🌈️ Every state of the species: its tint, its look over the breathing idle loop and the particles of its emitter, captioned with how long it lasts and what it gives way to. */
function stateScenes(species: Species, language: Language, text: Labels): Scene[] {
  const { width, height } = species.size;
  return species.states.map((state) => {
    const look = clipOf(species, state.clip ?? null);
    const emitter = species.emitters.find((entry) => entry.id === state.emitter);
    const tint = state.tint;
    const parts = [
      `${state.name[language]} · ${state.id}`,
      tint === undefined ? text.noTint : `${text.tint} ${TONES.filter((tone) => tint[tone] !== undefined).join("+")}`,
      look !== null ? clipCaption(look, text) : state.clip === undefined ? null : `${text.missingClip} ${state.clip}`,
      emitter !== undefined ? emitterCaption(emitter) : state.emitter === undefined ? null : `${text.missingEmitter} ${state.emitter}`,
      state.lasts === undefined ? null : `${text.lasts} ${state.lasts} s → ${state.then ?? species.states[0]?.id ?? ""}`,
    ];
    return { key: `state-${state.id}`, caption: parts.filter((part) => part !== null).join(" · "), clip: null, look, state: state.id, emitter, plume: "running", room: { up: height + (species.locomotion.hover ?? 0) + 34, ahead: width / 2 + 26, behind: width / 2 + 26 } } satisfies Scene;
  });
}

/** 🪄️ Every trick of the species — its clip replayed with the particles it throws, in the tint of the first state it starts from —, then the purr. */
function trickScenes(species: Species, language: Language, text: Labels): Scene[] {
  const { width, height } = species.size;
  const room = { up: height + (species.locomotion.hover ?? 0) + 34, ahead: width / 2 + 30, behind: width / 2 + 30 };
  const emitterOf = (id: Slug | undefined): Emitter | undefined => species.emitters.find((entry) => entry.id === id);
  const shown = (clip: Clip | null, id: Slug): string => (clip === null ? `${text.missingClip} ${id}` : clipCaption(clip, text));
  const tricks = species.tricks.map((trick) => {
    const clip = clipOf(species, trick.clip);
    const emitter = emitterOf(trick.emitter);
    const route = trick.from === undefined && trick.to === undefined ? null : `${trick.from === undefined ? "" : `${trick.from.join("/")} `}→${trick.to === undefined ? "" : ` ${trick.to}`}`;
    const caption = [`${trick.name[language]} · ${trick.id} [${trick.cues.join(", ")}]`, route, shown(clip, trick.clip), emitter === undefined ? null : emitterCaption(emitter)].filter((part) => part !== null).join(" · ");
    return { key: `trick-${trick.id}`, caption, clip, state: trick.from?.[0], activity: "trick", emitter, plume: "once", room } satisfies Scene;
  });
  const clip = clipOf(species, species.purr.clip);
  const emitter = emitterOf(species.purr.emitter);
  const caption = [text.purr, shown(clip, species.purr.clip), emitter === undefined ? null : emitterCaption(emitter)].filter((part) => part !== null).join(" · ");
  return [...tricks, { key: "purr", caption, clip, activity: "purr", emitter, plume: "running", room }];
}

/** 🪂️ Every clip of getting around and being handled with what goes with it — gear from the frame's tools, ladders, walls, a block, a ledge, the hand —, and the activities the species has no clip for. */
function gearScenes(species: Species, language: Language, text: Labels): { readonly scenes: readonly Scene[]; readonly missing: readonly Activity[] } {
  const { width, height } = species.size;
  const grip = species.grip;
  const [rimLeft, rimRight] = canopyRim(species);
  const canopyUp = grip + CHUTE_RISE * height + (species.canopy === undefined ? CHUTE_DOME * height * 1.1 : height * 0.8) + 6;
  const canopySide = Math.max(Math.abs(rimLeft.x), Math.abs(rimRight.x), species.canopy === undefined ? CHUTE_SPAN * width : 0) + 10;
  const spin = Math.hypot(width, height) / 2;
  const swing = (ticks: number, period: number): number => Math.sin((2 * Math.PI * ticks) / period);
  const anchor = { x: width * 0.75, y: -(grip + height * 0.9) };
  const scenes: Scene[] = [];
  const missing: Activity[] = [];
  for (const activity of GETTING_AROUND) {
    const ids = species.repertoire[activity] ?? [];
    if (ids.length === 0) missing.push(activity);
    for (const id of ids) {
      const clip = clipOf(species, id);
      const named = (way: Way): string => `${activity} · ${clip === null ? `${text.missingClip} ${id}` : clipCaption(clip, text)} — ${WAYS[language][way]}`;
      const base = { key: `${activity}-${id}`, caption: named(activity), clip, activity };
      if (activity === "hang") scenes.push({ ...base, footing: "hand", lift: 18, prop: "hand", room: { ahead: width * 0.8, behind: width * 0.8, up: height + 44 }, tilt: (ticks) => (20 / 360) * swing(ticks, 96) });
      else if (activity === "tumble") scenes.push({ ...base, footing: "air", lift: spin - height / 2 + 6, pivot: { x: 0, y: -height / 2 }, room: { ahead: spin + 6, behind: spin + 6, up: 2 * spin + 12 }, tilt: (ticks) => ticks / 96 });
      else if (activity === "glide") scenes.push({ ...base, footing: "chute", lift: 24, room: { ahead: canopySide, behind: canopySide, up: 24 + canopyUp }, tools: (ticks) => [{ kind: "chute", open: 1, sway: 0.025 * swing(ticks, 128) }], tilt: (ticks) => -0.015 * swing(ticks, 128) });
      else if (activity === "aim") scenes.push({ ...base, room: { ahead: width / 2 + 28 }, tools: (ticks, _feet, facing) => [{ kind: "gun", aim: (facing > 0 ? -AIM_RISE : AIM_RISE - 0.5) + 0.02 * facing * swing(ticks, 160) }] });
      else if (activity === "reel") {
        scenes.push({
          ...base,
          footing: "rope",
          lift: 20,
          prop: "ledge",
          anchor,
          pivot: { x: MUZZLE_FORWARD * width, y: -MUZZLE_HEIGHT * height },
          room: { ahead: anchor.x + 18, up: 20 - anchor.y + 14 },
          tilt: (ticks, facing) => facing * 0.02 * swing(ticks, 112),
          tools: (_ticks, feet, facing) => [
            { kind: "rope", x: feet.x + facing * anchor.x, y: feet.y + anchor.y, slack: 0 },
            { kind: "hook", x: feet.x + facing * anchor.x, y: feet.y + anchor.y },
          ],
        });
      } else if (activity === "climb" || activity === "slide") {
        scenes.push({ ...base, footing: "wall", lift: 22, prop: "wall", room: { up: height + 56 }, tilt: (_ticks, facing) => facing * WALL_LEAN });
        if (activity === "climb" && species.gear.includes("ladder")) {
          const length = Math.hypot(0.55 * height, 1.8 * height);
          scenes.push({
            ...base,
            key: `${base.key}-ladder`,
            caption: named("ladder"),
            footing: "ladder",
            lift: height * 0.5,
            prop: "wall",
            wall: height * 0.25 + 3,
            room: { up: height * 1.8 + 8, behind: height * 0.3 + 10, ahead: height * 0.25 + 16 },
            ladders: (feet, facing) => [{ x0: feet.x - facing * height * 0.3, y0: feet.y + height * 0.5, x1: feet.x + facing * height * 0.25, y1: feet.y - height * 1.3, rungs: Math.max(1, Math.floor(length / 10.5)), opacity: 1 }],
          });
        }
      } else if (activity === "mantle") scenes.push({ ...base, footing: "wall", prop: "corner", room: { ahead: width / 2 + 32, up: height + 24 } });
      else if (activity === "carry") scenes.push({ ...base, room: { ahead: height + 10, behind: height + 10 }, tools: (_ticks, _feet, facing) => [{ kind: "ladder", lean: facing * CARRY_LEAN, length: CARRY_LENGTH * height }] });
      else if (activity === "push") scenes.push({ ...base, prop: "block", room: { ahead: width / 2 + 30 } });
      else scenes.push(base);
    }
  }
  scenes.push({ key: "poof", caption: `poof — ${WAYS[language].poof}`, clip: null, look: null, poof: true, room: { ahead: width / 2 + 20, behind: width / 2 + 20, up: height + (species.locomotion.hover ?? 0) + 28 } });
  return { scenes, missing };
}

/** 📚️ One page of the gallery: who the species is, then its sections — at rest and every clip, its states, its tricks and the purr, getting around —, each on a light and on a dark ground. */
function SpeciesPage({ species, language, text, zoom, facing }: { readonly species: Species; readonly language: Language; readonly text: Labels; readonly zoom: number; readonly facing: 1 | -1 }): ReactElement {
  const sections = useMemo(() => {
    const around = gearScenes(species, language, text);
    return [
      { id: "clips", title: text.clips, scenes: clipScenes(species, text), note: null },
      { id: "states", title: text.states, scenes: stateScenes(species, language, text), note: null },
      { id: "tricks", title: text.tricks, scenes: trickScenes(species, language, text), note: null },
      { id: "around", title: text.around, scenes: around.scenes, note: around.missing.length === 0 ? null : `${text.noClips}: ${around.missing.join(", ")}` },
    ];
  }, [species, language, text]);
  const { gait, speed, hover } = species.locomotion;
  return (
    <section className="story" id={species.id} data-species={species.id}>
      <header className="story-head">
        <h2>{species.name[language]}</h2>
        <p>
          <code>{species.id}</code> · {species.thing[language]} · {species.size.width} × {species.size.height} px · {text.gait} {gait}, {speed} px/s{hover === undefined ? "" : ` · ${text.hover} ${hover} px`} · {text.grip} {species.grip} px · {text.reach} {species.reach} px · {text.gear} {species.gear.length === 0 ? text.none : species.gear.join(", ")}
          {species.canopy === undefined ? "" : ` · ${text.canopy}`} · {text.rests} {species.mood} · {text.counts} {species.states.length} · {species.tricks.length} · {species.emitters.length}
          {TONES.map((tone) => (
            <span key={tone} className="swatch" title={`${tone} ${species.palette[tone]}`} style={{ background: species.palette[tone] }} />
          ))}
        </p>
      </header>
      {sections.map((section) => (
        <section key={section.id} className="story-section" data-section={section.id}>
          <h3>{section.title}</h3>
          {section.note === null ? null : <p className="story-note">{section.note}</p>}
          {GROUNDS.map((ground) => (
            <div key={ground} className={`ground ground-${ground}`} data-ground={ground} aria-label={ground === "light" ? text.light : text.darkGround}>
              {section.scenes.map((scene) => (
                <Specimen key={scene.key} species={species} scene={scene} zoom={scene.key === "rest" ? zoom : zoom * CLIP_ZOOM} facing={facing} />
              ))}
            </div>
          ))}
        </section>
      ))}
    </section>
  );
}

/** 🔗️ The species the address names (`#<id>`), the first one when it names none the menagerie has. */
function hashed(menagerie: Menagerie): Species | null {
  const id = decodeURIComponent(window.location.hash.slice(1));
  return menagerie.species.find((species) => species.id === id) ?? menagerie.species[0] ?? null;
}

/** 🗂️ The species view: the controls, a roster to choose a species from, and the page of the chosen one. */
function Gallery({ menagerie, language, text }: { readonly menagerie: Menagerie; readonly language: Language; readonly text: Labels }): ReactElement {
  const [controls, setControls] = useState<Controls>({ mood: "content", intensity: 0.5, lid: 0, blinking: false, mirrored: false, speed: 1 });
  const [zoom, setZoom] = useState(4);
  const [studio, setStudio] = useState<Studio | null>(null);
  const [chosen, setChosen] = useState<Species | null>(() => hashed(menagerie));
  const latest = useRef(controls);
  useEffect(() => {
    latest.current = controls;
  }, [controls]);
  useEffect(() => {
    const opened = openStudio(() => latest.current);
    setStudio(opened.studio);
    return opened.close;
  }, []);
  useEffect(() => {
    const choose = (): void => setChosen(hashed(menagerie));
    choose();
    window.addEventListener("hashchange", choose);
    return () => window.removeEventListener("hashchange", choose);
  }, [menagerie]);
  const change = (patch: Partial<Controls>): void => setControls((current) => ({ ...current, ...patch }));
  const facing: 1 | -1 = controls.mirrored ? -1 : 1;
  const roster = useMemo(() => new Map<Slug, Scene>(menagerie.species.map((species) => [species.id, { key: "roster", caption: species.id, clip: null }])), [menagerie]);
  return (
    <StudioContext.Provider value={studio}>
      <form className="controls" onSubmit={(event) => event.preventDefault()}>
        <Choice label={text.mood} value={controls.mood} options={MOODS.map((mood) => [mood, mood] as const)} onChange={(mood) => change({ mood })} />
        <Slider label={text.intensity} value={controls.intensity} min={0} max={1} step={0.05} onChange={(intensity) => change({ intensity })} />
        <Slider label={text.lid} value={controls.lid} min={0} max={1} step={0.05} onChange={(lid) => change({ lid })} />
        <Toggle label={text.blinking} checked={controls.blinking} onChange={(blinking) => change({ blinking })} />
        <Toggle label={text.mirrored} checked={controls.mirrored} onChange={(mirrored) => change({ mirrored })} />
        <Slider label={text.speed} value={controls.speed} min={0} max={2} step={0.25} onChange={(speed) => change({ speed })} />
        <Slider label={text.zoom} value={zoom} min={2} max={8} step={1} onChange={setZoom} />
      </form>
      <nav className="roster ground-light" aria-label={text.species}>
        {menagerie.species.map((species) => (
          <a key={species.id} href={`#${species.id}`} title={species.name[language]} aria-current={species.id === chosen?.id ? "page" : undefined}>
            <Specimen species={species} scene={roster.get(species.id)!} zoom={1} facing={facing} />
          </a>
        ))}
      </nav>
      {chosen === null ? null : <SpeciesPage key={chosen.id} species={chosen} language={language} text={text} zoom={zoom} facing={facing} />}
    </StudioContext.Provider>
  );
}
//#endregion 🔖️Species

//#region 🔖️Director
/** 🎙️ What the dev server hands the gallery (`pets-stories:director`): the two functions of the core the pet layer calls while it is served there. The gallery may wrap them; whatever they hold is what the layer runs. */
export type Director = { advance: (menagerie: Menagerie, stage: Stage, events: readonly StageEvent[]) => Stage; frameOf: (menagerie: Menagerie, stage: Stage) => Frame };

/** 🧲️ A change the inspector folds into the stage between two frames, for one species on stage: enter a state, perform a trick, feel a mood at an intensity, or take up an activity. */
type Force = { readonly kind: "state"; readonly species: Slug; readonly state: Slug } | { readonly kind: "trick"; readonly species: Slug; readonly trick: Slug } | { readonly kind: "mood"; readonly species: Slug; readonly mood: Mood; readonly intensity: number } | { readonly kind: "activity"; readonly species: Slug; readonly activity: Activity };

/** 🧾️ What the inspector last read off the layer's frames: the tick, the rate the stage asks for and the frames it drew in the last second; the actors as drawn and how many particles, dust clouds, standing ladders and lifted fixtures there are, who is held; how many pets vanished in a puff, which bodies overlap now (there must never be any), in how many of the frames drawn since the show began any did; and the last force and whether the stage took it. */
type Reading = {
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
type Inspector = { readonly read: () => Reading | null; readonly listen: (listener: () => void) => () => void; readonly force: (force: Force) => void };

/** 🧍️ The activities a pet on a perch can be set to on its own, without a partner, a motion or gear. */
const STANDING = ["idle", "fidget", "sleep", "greet", "trick", "purr", "dizzy", "shrug", "push"] as const satisfies readonly Activity[];
const HOLD_TICKS = 2 * TICKS_PER_SECOND;
const READ_MILLISECONDS = 200;
const NO_READING = (): Reading | null => null;
const NO_LISTENING = (): (() => void) => () => {};

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
function useInspector(director: Director | null): Inspector | null {
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
async function carry(from: Point, to: Point, steps: number, hold: number): Promise<void> {
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

//#region 🔖️Sandbox
const PROP_ROWS = 5;

/** 🗝️ Up to five topic keys for the rows pets may play with: the grounds of the species the scene casts, one of each species in turn, so every pet that can play has a row of its own. */
function propKeys(menagerie: Menagerie, scene: string): readonly string[] {
  const cast = petCast(menagerie, scene);
  const grounds = (cast === null ? [] : [...cast.core, ...cast.rotation]).map((id) => menagerie.species.find((species) => species.id === id)?.grounds ?? []);
  const keys: string[] = [];
  for (let rank = 0; keys.length < PROP_ROWS && grounds.some((list) => rank < list.length); rank++) {
    for (const list of grounds) {
      const key = list[rank];
      if (key !== undefined && !keys.includes(key) && keys.length < PROP_ROWS) keys.push(key);
    }
  }
  return keys;
}

/** 🃏️ A mock card: its top edge carries pets, its sides are walls, its text and controls are kept free. */
function Card({ title, text, className = "", children }: { readonly title: string; readonly text: Labels; readonly className?: string; readonly children?: ReactNode }): ReactElement {
  return (
    <article className={`card ${className}`} data-pet-surface="">
      <h3>{title}</h3>
      <p>{text.cardText}</p>
      {children}
    </article>
  );
}

/** 🔢️ The actors of a frame in the order of the menagerie, so lists of them hold still while the actors move back and forth in the frame's drawing order. */
function inMenagerieOrder(menagerie: Menagerie, actors: readonly ActorFrame[]): readonly ActorFrame[] {
  const rank = (actor: ActorFrame): number => menagerie.species.findIndex((species) => species.id === actor.species);
  return [...actors].sort((one, other) => rank(one) - rank(other));
}

/** 📟️ The live reading of the stage: the numbers (overlaps stand out when there is one) and every actor as drawn, in the order of the menagerie, each row carrying what a tool needs to find it (`data-species`, `data-footing`, `data-activity`, `data-state`, `data-mood`, and `data-body`: its solid box in viewport pixels). */
function Readout({ menagerie, reading, text, scale }: { readonly menagerie: Menagerie; readonly reading: Reading | null; readonly text: Labels; readonly scale: number }): ReactElement {
  if (reading === null) return <p className="readout-empty">{text.unseen}</p>;
  const numbers = [
    ["tick", reading.tick],
    ["rate", reading.rate],
    ["steps", reading.steps],
    ["poofs", reading.poofs],
    ["overlaps", reading.overlaps.length],
    ["overlapped", reading.overlapped],
    ["frames", reading.frames],
    ["particles", reading.particles],
    ["puffs", reading.puffs],
    ["ladders", reading.ladders],
    ["lifts", reading.lifts],
  ] as const;
  const box = (actor: ActorFrame): string => [actor.body.x, actor.body.y, actor.body.width, actor.body.height].map((value) => Math.round(value * scale * 10) / 10).join(" ");
  return (
    <div className="readout">
      <dl>
        {numbers.map(([name, value]) => (
          <div key={name} data-reading={name} data-value={value} className={name === "overlaps" && value > 0 ? "alarm" : undefined}>
            <dt>{text[name]}</dt>
            <dd>{value}</dd>
          </div>
        ))}
        <div data-reading="held" data-value={reading.held ?? ""}>
          <dt>{text.held}</dt>
          <dd>{reading.held ?? "—"}</dd>
        </div>
      </dl>
      {reading.overlaps.length === 0 ? null : <p className="alarm">{reading.overlaps.join(", ")}</p>}
      <table>
        <thead>
          <tr>
            <th>{text.pet}</th>
            <th>{text.footing}</th>
            <th>{text.activity}</th>
            <th>{text.state}</th>
            <th>{text.mood}</th>
          </tr>
        </thead>
        <tbody>
          {inMenagerieOrder(menagerie, reading.actors).map((actor) => (
            <tr key={actor.species} data-species={actor.species} data-footing={actor.footing} data-activity={actor.activity} data-state={actor.state} data-mood={actor.mood} data-body={box(actor)}>
              <td>{actor.species}</td>
              <td>{actor.footing}</td>
              <td>{actor.activity}</td>
              <td>{actor.state}</td>
              <td>
                {actor.mood} {actor.intensity.toFixed(2)}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

/** 🩺️ The inspector's panel for one pet on stage: force its state, a trick, a mood, an activity (through the director), carry it into a footing with the gallery's own pointer, or ask it for a deed (through the layer's handle). */
function Inspect({ menagerie, reading, inspector, handle, text, scale }: { readonly menagerie: Menagerie; readonly reading: Reading | null; readonly inspector: Inspector | null; readonly handle: { readonly current: PetLayerHandle | null }; readonly text: Labels; readonly scale: number }): ReactElement {
  const actors = inMenagerieOrder(menagerie, reading?.actors ?? []);
  const [chosen, setChosen] = useState<Slug>("");
  const [state, setState] = useState<Slug>("");
  const [trick, setTrick] = useState<Slug>("");
  const [mood, setMood] = useState<Mood>("happy");
  const [intensity, setIntensity] = useState(0.8);
  const [activity, setActivity] = useState<Activity>("dizzy");
  const [busy, setBusy] = useState(false);
  const actor = actors.find((candidate) => candidate.species === chosen) ?? actors[0] ?? null;
  const kind = actor === null ? null : (menagerie.species.find((species) => species.id === actor.species) ?? null);
  if (actor === null || kind === null) return <p className="readout-empty">{text.nobody}</p>;
  const stateShown = kind.states.some((entry) => entry.id === state) ? state : (kind.states[0]?.id ?? "");
  const trickShown = kind.tricks.some((entry) => entry.id === trick) ? trick : (kind.tricks[0]?.id ?? "");
  const centre = (frame: ActorFrame): Point => ({ x: (frame.body.x + frame.body.width / 2) * scale, y: (frame.body.y + frame.body.height / 2) * scale });
  const carried = (to: (from: Point) => Point, steps: number, hold: number): void => {
    const from = centre(actor);
    setBusy(true);
    void carry(from, to(from), steps, hold).finally(() => setBusy(false));
  };
  const neighbour = actors.filter((other) => other.species !== actor.species && other.footing === "perch").sort((one, other) => Math.abs(one.x - actor.x) - Math.abs(other.x - actor.x))[0];
  const deeds: { readonly [deed in Deed]: string } = { hello: text.deedHello, trick: text.deedTrick, pet: text.deedPet, toss: text.deedToss };
  const news = reading?.news ?? null;
  return (
    <div className="inspect">
      <Choice label={text.pet} value={actor.species} options={actors.map((candidate) => [candidate.species, candidate.species] as const)} onChange={setChosen} />
      <p className="inspect-status" data-inspected={actor.species}>
        {actor.footing} · {actor.activity} · {actor.state} · {actor.mood} {actor.intensity.toFixed(2)}
      </p>
      <div className="inspect-row">
        <Choice label={text.state} value={stateShown} options={kind.states.map((entry) => [entry.id, entry.id] as const)} onChange={setState} />
        <Action label={text.enter} disabled={stateShown === ""} onPress={() => inspector?.force({ kind: "state", species: actor.species, state: stateShown })} />
      </div>
      <div className="inspect-row">
        <Choice label={text.trick} value={trickShown} options={kind.tricks.map((entry) => [entry.id, entry.id] as const)} onChange={setTrick} />
        <Action label={text.perform} disabled={trickShown === ""} onPress={() => inspector?.force({ kind: "trick", species: actor.species, trick: trickShown })} />
      </div>
      <div className="inspect-row">
        <Choice label={text.mood} value={mood} options={MOODS.map((entry) => [entry, entry] as const)} onChange={setMood} />
        <Slider label={text.intensity} value={intensity} min={0} max={1} step={0.05} onChange={setIntensity} />
        <Action label={text.feel} onPress={() => inspector?.force({ kind: "mood", species: actor.species, mood, intensity })} />
      </div>
      <div className="inspect-row">
        <Choice label={text.activity} value={activity} options={STANDING.map((entry) => [entry, entry] as const)} onChange={setActivity} />
        <Action label={text.act} onPress={() => inspector?.force({ kind: "activity", species: actor.species, activity })} />
      </div>
      <div className="inspect-row" role="group" aria-label={text.footing}>
        <Action label={text.pickUp} disabled={busy} onPress={() => carried((from) => ({ x: from.x, y: from.y - 40 }), 8, 128)} />
        <Action label={text.drop} disabled={busy} onPress={() => carried((from) => ({ x: from.x, y: from.y - 120 }), 10, 12)} />
        <Action label={text.dropHigh} disabled={busy} onPress={() => carried((from) => ({ x: from.x, y: Math.max(document.querySelector(".workbench")?.getBoundingClientRect().top ?? 0, 0) + 48 }), 24, 16)} />
        <Action label={text.dropOnHead} disabled={busy || neighbour === undefined} onPress={() => carried(() => (neighbour === undefined ? centre(actor) : { x: (neighbour.body.x + neighbour.body.width / 2) * scale, y: (neighbour.body.y - actor.body.height * 0.7 - 24) * scale }), 16, 16)} />
      </div>
      <p className="inspect-note">{text.routes}</p>
      <div className="inspect-row" role="group" aria-label={text.deeds}>
        {DEEDS.map((deed) => (
          <Action key={deed} label={deeds[deed]} onPress={() => handle.current?.play(actor.species, deed)} />
        ))}
      </div>
      {news === null ? null : (
        <p className="inspect-news" data-news={news.done ? "applied" : "refused"}>
          {news.force.species} ← {news.force.kind} {news.force.kind === "state" ? news.force.state : news.force.kind === "trick" ? news.force.trick : news.force.kind === "mood" ? `${news.force.mood} ${news.force.intensity}` : news.force.activity} · {news.tick} · {news.done ? text.applied : text.refused}
        </p>
      )}
    </div>
  );
}

/** 🏖️ The sandbox: mock terrain with walls, gutters and rows to play with; the switches of the pet layer and ways to disturb the terrain; the hand, the inspector and the live reading beside it. */
function Sandbox({ menagerie, text, director }: { readonly menagerie: Menagerie; readonly text: Labels; readonly director: Director | null }): ReactElement {
  const scenes = menagerie.casts.map((cast) => cast.scene);
  const [scene, setScene] = useState(scenes[0] ?? "home");
  const [mode, setMode] = useState<PetMode | "off">("lively");
  const [quiet, setQuiet] = useState(false);
  const [play, setPlay] = useState(true);
  const [mischief, setMischief] = useState(true);
  const [capacity, setCapacity] = useState(5);
  const [scale, setScale] = useState(1);
  const [seed, setSeed] = useState(1);
  const [pace, setPace] = useState(0);
  const [arrangement, setArrangement] = useState(0);
  const [taken, setTaken] = useState(false);
  const shelf = useRef<HTMLDivElement>(null);
  const handle = useRef<PetLayerHandle>(null);
  const inspector = useInspector(director);
  const reading = useSyncExternalStore(inspector?.listen ?? NO_LISTENING, inspector?.read ?? NO_READING);
  const keys = useMemo(() => propKeys(menagerie, scene), [menagerie, scene]);
  const tempo = 2 ** pace;
  const scroll = (): void => {
    const pane = shelf.current;
    if (pane) pane.scrollTop = pane.scrollTop + pane.clientHeight >= pane.scrollHeight - 1 ? 0 : pane.scrollTop + 72;
  };
  return (
    <>
      <form className="controls" onSubmit={(event) => event.preventDefault()}>
        <Choice label={text.scene} value={scene} options={scenes.map((name) => [name, name] as const)} onChange={setScene} />
        <Choice label={text.mode} value={mode} options={[["off", text.off] as const, ...PET_MODES.map((name) => [name, name] as const)]} onChange={setMode} />
        <Toggle label={text.quiet} checked={quiet} onChange={setQuiet} />
        <Toggle label={text.play} checked={play} onChange={setPlay} />
        <Toggle label={text.mischief} checked={mischief} onChange={setMischief} />
        <Slider label={text.capacity} value={capacity} min={1} max={8} step={1} onChange={setCapacity} />
        <Slider label={text.scale} value={scale} min={0.6} max={2} step={0.1} onChange={setScale} />
        <Slider label={text.tempo} value={pace} min={-3} max={3} step={1} shown={`×${tempo}`} onChange={setPace} />
        <Slider label={text.seed} value={seed} min={1} max={99} step={1} onChange={setSeed} />
        <Action label={text.reseed} onPress={() => setSeed(1 + Math.floor(Math.random() * 99))} />
        <Action label={text.scroll} onPress={scroll} />
        <Action label={text.rearrange} onPress={() => setArrangement((current) => current + 1)} />
        <Action label={taken ? text.restore : text.remove} pressed={taken} onPress={() => setTaken((current) => !current)} />
      </form>
      <div className="workbench">
        <div className={`room room-${arrangement % 2}`}>
          <Card title={`${text.card} 1`} text={text} className="card-low">
            <button type="button" className="tab">
              {text.action}
            </button>
          </Card>
          <Card title={`${text.card} 2`} text={text} className="card-high">
            <span className="note" data-pet-keepout="">
              {text.note}
            </span>
          </Card>
          {taken ? null : <Card title={`${text.card} 3`} text={text} className="card-wide" />}
          <article className="card card-task" data-pet-surface="">
            <h3>{text.task}</h3>
            <p>{keys.length === 0 ? text.noProps : text.taskText}</p>
            <ul className="props">
              {keys.map((key) => (
                <li key={key} className="prop" data-pet-prop={key}>
                  <code>{key}</code>
                </li>
              ))}
            </ul>
          </article>
          <div className="shelf" ref={shelf} aria-label={text.shelf}>
            {[4, 5, 6, 7, 8].map((number) => (
              <Card key={number} title={`${text.card} ${number}`} text={text} />
            ))}
          </div>
        </div>
        <aside className="bench" aria-label={text.inspect}>
          <section>
            <h2>{text.hand}</h2>
            <p className="hand-hint">{text.handHint}</p>
          </section>
          <section>
            <h2>{text.inspect}</h2>
            <Inspect menagerie={menagerie} reading={reading} inspector={inspector} handle={handle} text={text} scale={scale} />
          </section>
          <section>
            <h2>{text.readout}</h2>
            <Readout menagerie={menagerie} reading={reading} text={text} scale={scale} />
          </section>
        </aside>
      </div>
      {mode === "off" ? null : <PetLayer ref={handle} key={seed} menagerie={menagerie} scene={scene} mode={mode} quiet={quiet} play={play} mischief={mischief} capacity={capacity} scale={scale} seed={seed} tempo={tempo} />}
    </>
  );
}
//#endregion 🔖️Sandbox

//#region 🔖️Page
/** 🏛️ The gallery: its header with the view, language and theme switches, then the chosen view of the menagerie found in `source`. */
function Stories({ source, origin, director }: { readonly source: unknown; readonly origin: string; readonly director: Director | null }): ReactElement {
  const [language, setLanguage] = useState<Language>(preferredLanguage);
  const [dark, setDark] = useState(() => window.matchMedia("(prefers-color-scheme: dark)").matches);
  const [view, setView] = useState<"species" | "sandbox">("species");
  const { menagerie, issues } = useMemo(() => menagerieOf(source), [source]);
  const text: Labels = LABELS[language];
  useEffect(() => {
    document.documentElement.lang = language;
    document.title = text.title;
  }, [language, text]);
  useEffect(() => {
    document.documentElement.classList.toggle("dark", dark);
  }, [dark]);
  return (
    <main className="stories">
      <header className="masthead">
        <h1>
          {text.title}
          {menagerie ? ` · ${menagerie.title[language]}` : ""}
        </h1>
        <code>{origin}</code>
        <Action label={text.species} pressed={view === "species"} onPress={() => setView("species")} />
        <Action label={text.sandbox} pressed={view === "sandbox"} onPress={() => setView("sandbox")} />
        <Choice label={text.language} value={language} options={LANGUAGES.map((name) => [name, name] as const)} onChange={setLanguage} />
        <Toggle label={text.dark} checked={dark} onChange={setDark} />
      </header>
      {menagerie ? null : (
        <div className="notice">
          <p>
            {source === null ? text.missing : text.empty} <code>{origin}</code>
          </p>
          {issues.length === 0 ? null : (
            <>
              <p>{text.issues}</p>
              <ul>
                {issues.map((issue, index) => (
                  <li key={index}>
                    <code>{issue.path}</code> · {issue.code}
                  </li>
                ))}
              </ul>
            </>
          )}
        </div>
      )}
      {menagerie && view === "species" ? <Gallery menagerie={menagerie} language={language} text={text} /> : null}
      {menagerie && view === "sandbox" ? <Sandbox menagerie={menagerie} text={text} director={director} /> : null}
    </main>
  );
}

/** 🚀️ Mounts the gallery into `root` for whatever the menagerie file at `origin` exports (`null` when there is no such file), with the director of the dev server between the pet layer and the core (none: the sandbox shows no reading and forces nothing). */
export function mountStories(root: HTMLElement | null, source: unknown, origin: string, director: Director | null = null): void {
  if (!root) return;
  createRoot(root).render(
    <StrictMode>
      <Stories source={source} origin={origin} director={director} />
    </StrictMode>,
  );
}
//#endregion 🔖️Page
