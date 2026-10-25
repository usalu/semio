/** 🎬️ The studio of the stories page: the controls of a specimen, a scene and its layout, the poses and frames of a specimen, the particles it throws and the drawing of a specimen and its props. */
import { createContext, useContext, useEffect, useRef, type ReactElement } from "react";
import { TICKS_PER_SECOND, clipTicks, emitterKey, faceOf, lidAt, lifeTicks, lookOffset, particlesOf, pupilReach, restPose, sampleClip, solveRig, type Activity, type ActorFrame, type BonePose, type Clip, type Emitter, type Footing, type LadderFrame, type Mood, type Point, type Pose, type Slug, type Species, type ToolFrame, type Turns } from "@semio-tech/pets";
import { depict, paint, stageScenery, type EffectParticle } from "@semio-tech/pets-react";
import { PUFF_TICKS } from "@semio-tech/pets/modules/locomotion/constants";
import { clipOf } from "@semio-tech/pets/modules/draft";
import "../🎨️.css";

//#region 🔖️Studio
/** 🎚️ What the controls of the species view set for every specimen: the mood and how strongly it shows, how far the lids are down besides, whether the eyes blink, whether the pets face left, how fast the gallery's clock runs. */
export type Controls = { readonly mood: Mood; readonly intensity: number; readonly lid: number; readonly blinking: boolean; readonly mirrored: boolean; readonly speed: number };

/** 🖌️ One specimen as the studio paints it: `paint` draws it at a tick of the gallery's clock for the pointer and the controls, its stage lying at `corner` in the viewport; `end` takes its drawing away. */
type Painting = { readonly paint: (ticks: number, pointer: Point | null, corner: Point, controls: Controls) => void; readonly end: () => void };

/** ⏰️ The clock of the species view: a stage that entered gets its painting built when it first comes into view, and painted on every animation frame while it is in view. */
export type Studio = { readonly enter: (stage: Element, build: () => Painting) => () => void };

/** 🎞️ What a specimen shows: its clip (replayed after a pause when it does not loop; the rest pose without one) or, for a state, its look (`look`: the state's overlay clip over the breathing idle loop, `null` for the idle loop alone); the state whose tint it wears, the activity and the footing it names; the emitter whose particles it throws, running with the scene or once per replay; how high its feet are above the ground and the room it needs ahead, behind and above them (pet pixels); the tilt of its drawing over time and the pivot it turns about; the tools it holds and the ladders that stand beside it, over time and from where its feet are; what it leans on, hangs from or pushes (`prop`, with the wall `wall` pixels ahead of the feet, the ledge of a rope's hook at `anchor`); and whether it vanishes in a puff now and then (`poof`). */
export type Scene = {
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

export const StudioContext = createContext<Studio | null>(null);
const MARGIN = 12;
const DOWN = 8;
export const CLIP_ZOOM = 0.625;
const GAZE_REACH = 24;
const REPLAY_PAUSE = 40;
const BLINK_EVERY = 160;
const TRICK_TICKS = 2 * TICKS_PER_SECOND;
const POOF_SHOWN = 96;
const POOF_CYCLE = POOF_SHOWN + PUFF_TICKS + 16;
const SPARK_SEED = 1;
export const GROUNDS = ["light", "dark"] as const;
export const TONES = ["body", "accent", "detail"] as const;
const ORIGIN: Point = { x: 0, y: 0 };
const NO_COPIES: ReadonlySet<Element> = new Set();

/** ⏱️ Opens the clock: one animation frame loop, one pointer listener and one visibility observer for every specimen; the stages in view are measured first and painted after, so painting never forces a layout between two measurements. */
export function openStudio(controls: () => Controls): { readonly studio: Studio; readonly close: () => void } {
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
export function Specimen({ species, scene, zoom, facing }: { readonly species: Species; readonly scene: Scene; readonly zoom: number; readonly facing: 1 | -1 }): ReactElement {
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
