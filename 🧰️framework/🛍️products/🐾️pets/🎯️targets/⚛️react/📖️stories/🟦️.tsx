/** 📖️ The stories gallery of `@semio-tech/pets-react`: a dev page for judging rigs and behaviour by eye, never part of a release.
 *
 * Two views of one menagerie. *Species* shows every species large on a light and on a dark ground — at rest and with
 * every clip looping, drawn by the product's own depiction from poses of the core (`sampleClip`, `solveRig`); mood, lid
 * and mirror controls apply to all of them and the pupils follow the pointer. *Sandbox* is a page of mock cards
 * (`data-pet-surface`) with buttons and text as keep-outs, on which a cast lives through the pet layer; the terrain can
 * be scrolled, rearranged and taken away, and the layer's inputs are switches.
 *
 * The document hands over whatever the menagerie file exports; the first valid menagerie found in it is shown. Every
 * label exists in English and German, and the page starts in the browser's language.
 *
 * @see ./🌐️.html — the document
 * @see ../🏗️builder/🌐️vite/🟦️.ts — the dev server and the menagerie module
 * @see ../🔨️modules/🖌️depiction/🟦️.ts — the renderer under judgement
 */

import { StrictMode, createContext, useContext, useEffect, useMemo, useRef, useState, type ReactElement, type ReactNode } from "react";
import { createRoot } from "react-dom/client";
import { ACTIVITIES, MENAGERIE_SCHEMA, PET_MODES, TICKS_PER_SECOND, clipTicks, lidAt, lookOffset, menagerieIssues, pupilReach, restPose, sampleClip, solveRig, type ActorFrame, type Clip, type Issue, type Menagerie, type PetMode, type Point, type Species } from "@semio-tech/pets";
import { PetLayer, depict, paint } from "@semio-tech/pets-react";
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
    missing: "No file at",
    empty: "No valid menagerie in",
    issues: "Issues",
    scene: "Scene",
    mode: "Mode",
    off: "off",
    quiet: "Quiet",
    capacity: "Capacity",
    scale: "Scale",
    seed: "Seed",
    reseed: "New seed",
    poke: "Poke a pet",
    scroll: "Scroll the cards",
    rearrange: "Rearrange the cards",
    remove: "Take a card away",
    restore: "Put the card back",
    card: "Card",
    cardText: "Pets walk along the top edge of a card. Text and controls are kept free.",
    action: "A button",
    note: "A note on the edge",
    shelf: "A scrolling shelf",
  },
  de: {
    title: "Tierchengeschichten",
    species: "Arten",
    sandbox: "Sandkasten",
    language: "Sprache",
    dark: "Dunkle Seite",
    mood: "Stimmung",
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
    missing: "Keine Datei unter",
    empty: "Keine gültige Menagerie in",
    issues: "Befunde",
    scene: "Szene",
    mode: "Modus",
    off: "aus",
    quiet: "Ruhe",
    capacity: "Plätze",
    scale: "Maßstab",
    seed: "Startwert",
    reseed: "Neuer Startwert",
    poke: "Ein Tierchen anstupsen",
    scroll: "Karten rollen",
    rearrange: "Karten umstellen",
    remove: "Eine Karte wegnehmen",
    restore: "Karte zurücklegen",
    card: "Karte",
    cardText: "Tierchen laufen auf der Oberkante einer Karte. Text und Bedienelemente bleiben frei.",
    action: "Ein Knopf",
    note: "Eine Notiz auf der Kante",
    shelf: "Ein rollendes Regal",
  },
} as const;

/** 🗣️ The languages of the gallery, English first. */
type Language = keyof typeof LABELS;

/** 🏷️ Every label of the gallery in one language. */
type Labels = { readonly [label in keyof (typeof LABELS)["en"]]: string };

const LANGUAGES = Object.keys(LABELS) as Language[];

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
/** 🎚️ What the controls of the species view set for every specimen. */
type Controls = { readonly mood: number; readonly lid: number; readonly blinking: boolean; readonly mirrored: boolean; readonly speed: number };

/** 🖌️ Paints one specimen at a tick of the gallery's clock. */
type Painter = (ticks: number, pointer: Point | null, controls: Controls) => void;

/** 🎬️ The clock of the species view: a stage that entered is painted on every animation frame while it is on screen. */
type Studio = { readonly enter: (stage: Element, painter: Painter) => () => void };

const StudioContext = createContext<Studio | null>(null);
const MARGIN = 12;
const CLIP_ZOOM = 0.625;
const GAZE_REACH = 24;
const REPLAY_PAUSE = 40;
const BLINK_EVERY = 160;
const GROUNDS = ["light", "dark"] as const;

/** ⏱️ Opens the clock: one animation frame loop, one pointer listener and one visibility observer for every specimen. */
function openStudio(controls: () => Controls): { readonly studio: Studio; readonly close: () => void } {
  const painters = new Map<Element, Painter>();
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
    for (const stage of seen) painters.get(stage)?.(Math.floor(ticks), pointer, current);
    handle = requestAnimationFrame(frame);
  };
  let handle = requestAnimationFrame(frame);
  window.addEventListener("pointermove", point, { passive: true });
  document.documentElement.addEventListener("pointerleave", unpoint, { passive: true });
  return {
    studio: {
      enter(stage, painter) {
        painters.set(stage, painter);
        watcher.observe(stage);
        painter(Math.floor(ticks), pointer, controls());
        return () => {
          painters.delete(stage);
          seen.delete(stage);
          watcher.unobserve(stage);
        };
      },
    },
    close() {
      cancelAnimationFrame(handle);
      watcher.disconnect();
      window.removeEventListener("pointermove", point);
      document.documentElement.removeEventListener("pointerleave", unpoint);
    },
  };
}

/** 🖼️ The frame of a specimen: the pose of `clip` at `ticks` (the rest pose without a clip; a clip that does not loop is replayed after a pause), pupils drawn towards the pointer, lids and mood from the controls. `feet` is where the specimen stands on its stage, `corner` where that stage is in the viewport. */
function specimenFrame(species: Species, clip: Clip | null, ticks: number, feet: Point, corner: Point, pointer: Point | null, controls: Controls, scale: number): ActorFrame {
  const bones = solveRig(species, clip ? sampleClip(species, clip, clip.loop ? ticks : ticks % (clipTicks(clip) + REPLAY_PAUSE)) : restPose(species));
  const facing = controls.mirrored ? -1 : 1;
  const lid = Math.max(controls.lid, controls.blinking ? lidAt(ticks % BLINK_EVERY) : 0);
  const eyes = species.face.eyes.map((eye) => {
    const bone = species.bones.findIndex((candidate) => candidate.id === eye.bone) * 6;
    if (!pointer || bone < 0) return { x: 0, y: 0, lid };
    const centre = { x: bones[bone]! * eye.x + bones[bone + 2]! * eye.y + bones[bone + 4]!, y: bones[bone + 1]! * eye.x + bones[bone + 3]! * eye.y + bones[bone + 5]! };
    const look = lookOffset(centre, { x: ((pointer.x - corner.x - feet.x) / scale) * facing, y: (pointer.y - corner.y - feet.y) / scale }, GAZE_REACH);
    const reach = pupilReach(eye);
    return { x: look.x * reach, y: look.y * reach, lid };
  });
  return { species: species.id, x: feet.x, y: feet.y, facing, activity: "idle", opacity: 1, bones, eyes, mood: controls.mood };
}

/** 🧸️ One species on a stage: its declared box, the ground it stands on (or hovers above) and the depiction, painted by the studio. */
function Specimen({ species, clip, scale, caption }: { readonly species: Species; readonly clip: Clip | null; readonly scale: number; readonly caption: string }): ReactElement {
  const studio = useContext(StudioContext);
  const stage = useRef<HTMLDivElement>(null);
  const hover = species.locomotion.hover ?? 0;
  const width = (species.size.width + 2 * MARGIN) * scale;
  const ground = (species.size.height + hover + MARGIN) * scale;
  useEffect(() => {
    const host = stage.current;
    if (!host || !studio) return undefined;
    const depiction = depict(species, host.ownerDocument);
    const feet = { x: width / 2, y: ground - hover * scale };
    host.append(depiction.element);
    const leave = studio.enter(host, (ticks, pointer, controls) => {
      const box = host.getBoundingClientRect();
      paint(depiction, specimenFrame(species, clip, ticks, feet, { x: box.left, y: box.top }, pointer, controls, scale), scale);
    });
    return () => {
      leave();
      depiction.element.remove();
    };
  }, [species, clip, scale, studio, width, ground, hover]);
  return (
    <figure className="specimen">
      <div ref={stage} className="specimen-stage" style={{ width, height: ground + (MARGIN * scale) / 2 }}>
        <div className="specimen-ground" style={{ top: ground }} />
        <div className="specimen-box" style={{ left: MARGIN * scale, top: ground - (hover + species.size.height) * scale, width: species.size.width * scale, height: species.size.height * scale }} />
      </div>
      <figcaption>{caption}</figcaption>
    </figure>
  );
}
//#endregion 🔖️Studio

//#region 🔖️Controls
/** 🎛️ A labelled range input that shows its value. */
function Slider({ label, value, min, max, step, onChange }: { readonly label: string; readonly value: number; readonly min: number; readonly max: number; readonly step: number; readonly onChange: (value: number) => void }): ReactElement {
  return (
    <label className="control">
      <span>{label}</span>
      <input type="range" value={value} min={min} max={max} step={step} onChange={(event) => onChange(Number(event.target.value))} />
      <output>{value}</output>
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
function Action({ label, onPress, pressed }: { readonly label: string; readonly onPress: () => void; readonly pressed?: boolean }): ReactElement {
  return (
    <button type="button" className="action" aria-pressed={pressed} onClick={onPress}>
      {label}
    </button>
  );
}
//#endregion 🔖️Controls

//#region 🔖️Species
/** 🎭️ The activities that play each clip of a species, by clip id. */
function clipRoles(species: Species): ReadonlyMap<string, readonly string[]> {
  const roles = new Map<string, string[]>();
  for (const activity of ACTIVITIES) for (const clip of species.repertoire[activity] ?? []) roles.set(clip, [...(roles.get(clip) ?? []), activity]);
  return roles;
}

/** 🧬️ One species: who it is, then at rest and with every clip looping, once per ground. */
function SpeciesStory({ species, language, text, zoom }: { readonly species: Species; readonly language: Language; readonly text: Labels; readonly zoom: number }): ReactElement {
  const roles = useMemo(() => clipRoles(species), [species]);
  const { gait, speed, hover } = species.locomotion;
  return (
    <section className="story" id={species.id}>
      <header className="story-head">
        <h2>{species.name[language]}</h2>
        <p>
          <code>{species.id}</code> · {species.thing[language]} · {species.size.width} × {species.size.height} px · {text.gait} {gait}, {speed} px/s{hover === undefined ? "" : ` · ${text.hover} ${hover} px`}
          {(["body", "accent", "detail"] as const).map((paintName) => (
            <span key={paintName} className="swatch" title={`${paintName} ${species.palette[paintName]}`} style={{ background: species.palette[paintName] }} />
          ))}
        </p>
      </header>
      {GROUNDS.map((ground) => (
        <div key={ground} className={`ground ground-${ground}`}>
          <Specimen species={species} clip={null} scale={zoom} caption={`${text.rest} · ${ground === "light" ? text.light : text.darkGround}`} />
          {species.clips.map((clip) => (
            <Specimen key={clip.id} species={species} clip={clip} scale={zoom * CLIP_ZOOM} caption={`${clip.id} · ${clip.seconds} s · ${clip.loop ? text.loop : text.once} · ${roles.get(clip.id)?.join(", ") ?? text.unused}`} />
          ))}
        </div>
      ))}
    </section>
  );
}

/** 🗂️ The species view: the controls, a roster to jump from, and every species. */
function Gallery({ menagerie, language, text }: { readonly menagerie: Menagerie; readonly language: Language; readonly text: Labels }): ReactElement {
  const [controls, setControls] = useState<Controls>({ mood: 0.4, lid: 0, blinking: false, mirrored: false, speed: 1 });
  const [zoom, setZoom] = useState(4);
  const [studio, setStudio] = useState<Studio | null>(null);
  const latest = useRef(controls);
  useEffect(() => {
    latest.current = controls;
  }, [controls]);
  useEffect(() => {
    const opened = openStudio(() => latest.current);
    setStudio(opened.studio);
    return opened.close;
  }, []);
  const change = (patch: Partial<Controls>): void => setControls((current) => ({ ...current, ...patch }));
  return (
    <StudioContext.Provider value={studio}>
      <form className="controls" onSubmit={(event) => event.preventDefault()}>
        <Slider label={text.mood} value={controls.mood} min={-1} max={1} step={0.1} onChange={(mood) => change({ mood })} />
        <Slider label={text.lid} value={controls.lid} min={0} max={1} step={0.05} onChange={(lid) => change({ lid })} />
        <Toggle label={text.blinking} checked={controls.blinking} onChange={(blinking) => change({ blinking })} />
        <Toggle label={text.mirrored} checked={controls.mirrored} onChange={(mirrored) => change({ mirrored })} />
        <Slider label={text.speed} value={controls.speed} min={0} max={2} step={0.25} onChange={(speed) => change({ speed })} />
        <Slider label={text.zoom} value={zoom} min={2} max={8} step={1} onChange={setZoom} />
      </form>
      <nav className="roster ground-light" aria-label={text.species}>
        {menagerie.species.map((species) => (
          <a key={species.id} href={`#${species.id}`} title={species.name[language]}>
            <Specimen species={species} clip={null} scale={1} caption={species.id} />
          </a>
        ))}
      </nav>
      {menagerie.species.map((species) => (
        <SpeciesStory key={species.id} species={species} language={language} text={text} zoom={zoom} />
      ))}
    </StudioContext.Provider>
  );
}
//#endregion 🔖️Species

//#region 🔖️Sandbox
/** 👆️ Taps the stage on a pet chosen at random: a pointer event on the page at the pet's body, as a finger would send it. */
function pokeSomePet(): void {
  const pets = [...document.querySelectorAll(".pet-layer .pet")];
  const pet = pets[Math.floor(Math.random() * pets.length)];
  if (!pet) return;
  const feet = pet.getBoundingClientRect();
  document.body.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true, clientX: feet.left, clientY: feet.top - 12, pointerType: "touch", isPrimary: true }));
}

/** 🃏️ A mock card: its top edge carries pets, its text and controls are kept free. */
function Card({ title, text, className = "", children }: { readonly title: string; readonly text: Labels; readonly className?: string; readonly children?: ReactNode }): ReactElement {
  return (
    <article className={`card ${className}`} data-pet-surface="">
      <h3>{title}</h3>
      <p>{text.cardText}</p>
      {children}
    </article>
  );
}

/** 🏖️ The sandbox: mock terrain, the switches of the pet layer and ways to disturb the terrain. */
function Sandbox({ menagerie, text }: { readonly menagerie: Menagerie; readonly text: Labels }): ReactElement {
  const scenes = menagerie.casts.map((cast) => cast.scene);
  const [scene, setScene] = useState(scenes[0] ?? "home");
  const [mode, setMode] = useState<PetMode | "off">("calm");
  const [quiet, setQuiet] = useState(false);
  const [capacity, setCapacity] = useState(4);
  const [scale, setScale] = useState(1);
  const [seed, setSeed] = useState(1);
  const [arrangement, setArrangement] = useState(0);
  const [taken, setTaken] = useState(false);
  const shelf = useRef<HTMLDivElement>(null);
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
        <Slider label={text.capacity} value={capacity} min={1} max={8} step={1} onChange={setCapacity} />
        <Slider label={text.scale} value={scale} min={0.6} max={2} step={0.1} onChange={setScale} />
        <Slider label={text.seed} value={seed} min={1} max={99} step={1} onChange={setSeed} />
        <Action label={text.reseed} onPress={() => setSeed(1 + Math.floor(Math.random() * 99))} />
        <Action label={text.poke} onPress={pokeSomePet} />
        <Action label={text.scroll} onPress={scroll} />
        <Action label={text.rearrange} onPress={() => setArrangement((current) => current + 1)} />
        <Action label={taken ? text.restore : text.remove} pressed={taken} onPress={() => setTaken((current) => !current)} />
      </form>
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
        <div className="shelf" ref={shelf} aria-label={text.shelf}>
          {[4, 5, 6, 7, 8].map((number) => (
            <Card key={number} title={`${text.card} ${number}`} text={text} />
          ))}
        </div>
      </div>
      {mode === "off" ? null : <PetLayer key={seed} menagerie={menagerie} scene={scene} mode={mode} quiet={quiet} capacity={capacity} scale={scale} seed={seed} />}
    </>
  );
}
//#endregion 🔖️Sandbox

//#region 🔖️Page
/** 🏛️ The gallery: its header with the view, language and theme switches, then the chosen view of the menagerie found in `source`. */
function Stories({ source, origin }: { readonly source: unknown; readonly origin: string }): ReactElement {
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
      {menagerie && view === "sandbox" ? <Sandbox menagerie={menagerie} text={text} /> : null}
    </main>
  );
}

/** 🚀️ Mounts the gallery into `root` for whatever the menagerie file at `origin` exports (`null` when there is no such file). */
export function mountStories(root: HTMLElement | null, source: unknown, origin: string): void {
  if (!root) return;
  createRoot(root).render(
    <StrictMode>
      <Stories source={source} origin={origin} />
    </StrictMode>,
  );
}
//#endregion 🔖️Page
