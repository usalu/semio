/** 🐾️ The species page of the stories gallery: every scene of a species on a light and a dark ground, its clips, states, tricks and gear, and the gallery of all species. */
import { useEffect, useMemo, useRef, useState, type ReactElement } from "react";
import { ACTIVITIES, MOODS, MUZZLE_FORWARD, MUZZLE_HEIGHT, type Activity, type Clip, type Emitter, type Menagerie, type Slug, type Species } from "@semio-tech/pets";
import { CHUTE_DOME, CHUTE_RISE, CHUTE_SPAN, canopyRim } from "@semio-tech/pets-react";
import { AIM_RISE, CARRY_LEAN, CARRY_LENGTH } from "@semio-tech/pets/modules/projection";
import { WALL_LEAN } from "@semio-tech/pets/modules/spacing";
import { clipOf } from "@semio-tech/pets/modules/draft";
import "../🎨️.css";
import { GETTING_AROUND, type Labels, type Language, type Way, WAYS } from "../🔤️language-and-menagerie/🟦️.ts";
import { CLIP_ZOOM, type Controls, GROUNDS, openStudio, type Scene, Specimen, type Studio, StudioContext, TONES } from "../🎬️studio/🟦️.tsx";
import { Choice, Slider, Toggle } from "../🎚️controls/🟦️.tsx";

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
export function Gallery({ menagerie, language, text }: { readonly menagerie: Menagerie; readonly language: Language; readonly text: Labels }): ReactElement {
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
