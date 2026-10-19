/** 🏟️ The sandbox page of the stories gallery: the mock cards, the readout and inspection panels and the sandbox in which the cast of a scene lives through the real pet layer. */
import { useMemo, useRef, useState, useSyncExternalStore, type ReactElement, type ReactNode } from "react";
import { DEEDS, MOODS, PET_MODES, type Activity, type ActorFrame, type Deed, type Menagerie, type Mood, type PetMode, type Point, type Slug } from "@semio-tech/pets";
import { PetLayer, petCast, type PetLayerHandle } from "@semio-tech/pets-react";
import "../🎨️.css";
import type { Labels } from "../🔤️language-and-menagerie/🟦️.ts";
import { Action, Choice, Slider, Toggle } from "../🎚️controls/🟦️.tsx";
import { carry, type Director, type Inspector, NO_LISTENING, NO_READING, type Reading, STANDING, useInspector } from "../🎛️director-and-inspector/🟦️.ts";

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
  const deeds: { readonly [deed in Deed]: string } = { hello: text.deedHello, trick: text.deedTrick, pet: text.deedPet, toss: text.deedToss, circle: text.deedCircle, countercircle: text.deedCountercircle, stroke: text.deedStroke, shake: text.deedShake };
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
export function Sandbox({ menagerie, text, director }: { readonly menagerie: Menagerie; readonly text: Labels; readonly director: Director | null }): ReactElement {
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
