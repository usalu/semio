/** 🐾️ Pets in the quiz: small animated companions that fit the topic on screen. This module is the quiz's glue to the
 * pets product, the half of it the client needs before any pet came — which liveliness the learner chose, the switch
 * that shows and hides them on every screen, the topics the quiz marks, the lazy loading of the species, of the layer
 * that draws them and of the other half of the glue, and what the preferences and the client read of the pets once
 * they came. The other half (`./🎪️stage/🟦️.tsx`) comes with the pets: the scene of the step on screen, the names of
 * the pets on stage, the layer as the quiz sets it up — what pets stand on, what of the quiz a press never takes from
 * the learner, the topics they may play with — and the deeds the settings ask the pets on stage for.
 *
 * The quiz knows no species. A site hands its menagerie in as a {@link CoursePetsSource}; the menagerie, the render
 * target (`@semio-tech/pets-react`) and the other half of the glue are fetched together, and only once the learner's
 * choice is not `off`, so a learner without pets never downloads them. Pets are decoration: a load that fails leaves
 * the quiz as it is without a word, a layer that throws is taken away (React itself reports what it caught), and the
 * layer never takes the keyboard or a place in the accessibility tree; a press is a pet's only where nothing of the
 * quiz acts on it.
 *
 * The learner decides whether the pets answer clicks and can be picked up (`petsPlay`) and whether they may play with
 * the page (`petsMischief`); still pets do neither. A run stays a time of concentration: the layer hears that it is
 * quiet and the pets' own stage decides what they may do then — the glue only says what the learner allows. Pets play
 * with the page only on lifted copies of what the quiz marks with a topic key among the grounds of the menagerie
 * (`data-pet-prop`: `<quiz>`, `<quiz>/<task>`, `<quiz>/<task>/<item>`, composed by {@link petProp}): the tasks of an
 * opened quiz's page, the items of a run that are no table rows and the true order of a sorting's results. The cards of
 * the overview carry their quiz as `data-pet-topic` instead, since a card is never lifted. The settings offer the play
 * of the hand without a pointer ({@link PetsPlay}).
 *
 * Pets start lively. A device that asks for reduced motion does not hold them still — some devices report that hint
 * without the learner having asked for it (every browser in a Remote Desktop session does). The learner chooses off,
 * still, calm or lively, and that choice is what the layer gets.
 *
 * @see ./🎪️stage/🟦️.tsx — the half of the glue that comes with the pets
 * @see ../../../../../🐾️pets/README.md — the pets product: menageries, stages and frames
 * @see ../🎛️preferences/🟦️.tsx — where the learner chooses how lively they are, and where that they chose is stored
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html — why `still`, `off` and the switch exist
 * @see https://www.w3.org/TR/mediaqueries-5/#prefers-reduced-motion — the device's hint
 */

import { createContext, useCallback, useContext, useEffect, useLayoutEffect, useMemo, useRef, useState, type ReactElement, type ReactNode } from "react";
import { useMediaQuery } from "@semio-tech/ui-react/chrome";
import type { Menagerie, Slug } from "@semio-tech/pets";
import type { PetLayerHandle } from "@semio-tech/pets-react";
import type { CourseLocale, CourseText } from "../🌐️i18n/🟦️.ts";
import type { CourseState } from "../🧭️session/🧮️state/🟦️.ts";
import type { PetDeed, PetPlay, CoursePetsStage, StagedPets } from "./🎪️stage/🟦️.tsx";

//#region 🎚️Choice
/** 🎚️ What a learner may choose for the pets: none at all, motionless, slightly active or busy. */
export const PET_CHOICES = ["off", "still", "calm", "lively"] as const;

/** 🔢️ One of {@link PET_CHOICES}. */
export type PetChoice = (typeof PET_CHOICES)[number];

/** 🌡️ How lively pets are while they show: every {@link PetChoice} but `off`. */
export type PetLiveliness = Exclude<PetChoice, "off">;

/** 🧘️ The liveliness the pets get: the learner's `choice`, lively until they choose otherwise. A device that asks for
 * reduced motion does not hold them still. Forced colours are not its concern: the layer shows nobody under them,
 * whatever its mode. */
export function effectivePetMode(choice: PetChoice): PetChoice {
  return choice;
}

/** 🔀️ What the pets switch makes of a choice: `off` when the pets are hidden, and when they are shown again the
 * liveliness they had before (`before`). */
export function switchedPets(shown: boolean, before: PetLiveliness): PetChoice {
  return shown ? before : "off";
}
//#endregion 🎚️Choice

//#region 🗺️Topics
/** 🔑️ The topic key of `part` within `topic` (`<quiz>` or `<quiz>/<task>`): `<topic>/<part>`, one of the grounds a
 * species may name; none without a topic. A key says what an element is about, never a value or an answer. */
export function petProp(topic: string | undefined, part: string): string | undefined {
  return topic === undefined ? undefined : `${topic}/${part}`;
}

const PetTopicContext = createContext<string | undefined>(undefined);
type PetQuestion = { readonly run: string; readonly task: string };
const PetQuestionContext = createContext<(owner: symbol, question: PetQuestion | undefined) => void>(() => undefined);

/** ❓️ Registers this screen's question before paint and clears only its own registration on departure. */
export function usePetQuestion(run: string, task: string | undefined): void {
  const setQuestion = useContext(PetQuestionContext);
  const owner = useRef(Symbol("course-pet-question"));
  useLayoutEffect(() => {
    setQuestion(owner.current, task === undefined ? undefined : { run, task });
    return () => setQuestion(owner.current, undefined);
  }, [setQuestion, run, task]);
}

/** 🧭️ Names the topic of everything inside it — the task of a run or of results, as `<quiz>/<task>` — so the items it
 * shows mark themselves with their own key ({@link petProp}); it adds no element. */
export function PetTopic(props: { readonly topic: string | undefined; readonly children: ReactNode }): ReactElement {
  return <PetTopicContext.Provider value={props.topic}>{props.children}</PetTopicContext.Provider>;
}

/** 🧩️ The topic named around the caller ({@link PetTopic}); none outside one, where nothing marks itself. */
export function usePetTopic(): string | undefined {
  return useContext(PetTopicContext);
}
//#endregion 🗺️Topics

//#region 🚚️Loading
/** 🎪️ Where a site's menagerie comes from: called at most once per mounted attempt (a development build that mounts
 * twice asks twice and drops the first answer), and only when pets are wanted. */
export type CoursePetsSource = () => Promise<Menagerie>;

/** 📦️ The render target as a chunk of its own (with its stylesheet), fetched when pets are first wanted. */
const renderTarget: CoursePetsStage = () => import("@semio-tech/pets-react");

/** 🎒️ The half of the glue that comes with the pets (`./🎪️stage/🟦️.tsx`). */
type CoursePetsHalf = typeof import("./🎪️stage/🟦️.tsx");

/** 🧳️ The half of the glue that comes with the pets as a chunk of its own, fetched beside the render target. */
const courseHalf = (): Promise<CoursePetsHalf> => import("./🎪️stage/🟦️.tsx");

/** 🎟️ What the layer shows once the pets came: the species, the render target and the half of the glue that came with
 * them, the scene of the step on screen, how lively the pets are, whether a run asks for quiet, what the learner allows
 * (`play`, `mischief`), the handle the settings ask for deeds by, and where the layer says who is on stage. */
export interface ShownPets extends StagedPets {
  readonly half: CoursePetsHalf;
}

type LoadedPets = Pick<ShownPets, "menagerie" | "target" | "half">;

/** 🗒️ What the client knows about its pets: whether the site has any (`offered`), whether the device asks for reduced
 * motion and thereby holds the pets of a learner who has not chosen still (`reduced`), whether it forces its own colours, under which the layer
 * shows nobody (`forced`), what the layer draws once it is fetched (`shown`), the names of the pets on stage right
 * now, in the learner's language (`names`), and how the settings play with them (`playing`: only while they may). */
interface CoursePetsNotes {
  readonly offered: boolean;
  readonly reduced: boolean;
  readonly forced: boolean;
  readonly shown: ShownPets | undefined;
  readonly names: readonly string[];
  readonly playing: PetPlay | undefined;
}

const NO_NAMES: readonly string[] = [];
const NO_SPECIES: readonly Slug[] = [];
const NO_PETS: CoursePetsNotes = { offered: false, reduced: false, forced: false, shown: undefined, names: NO_NAMES, playing: undefined };
const CoursePetsContext = createContext<CoursePetsNotes>(NO_PETS);

/** 🚚️ Fetches the menagerie of `source`, the render target and the half of the glue that comes with the pets once the
 * learner's `choice` asks for pets, and makes them — with the scene of the step in `state` — what {@link CoursePets}
 * draws, while {@link usePetCast} names, in `locale`, whoever the layer says is on stage. The layer's mode is that choice ({@link effectivePetMode}); `chosen` records that the learner stated it and does not change the mode. Nothing is
 * fetched without a source or while the choice is `off`; a fetch that fails is forgotten (choosing pets again tries anew) and one that outlives the provider or the
 * wish is dropped. A run is a time of concentration: the pets rest while one is on screen. Results are not: a learner who reads
 * a score is done concentrating, the pets live again, and the switch on that screen hides them for whoever minds.
 * Whether the pets answer clicks and can be picked up (`play`) and whether they may play with the page (`mischief`) is
 * what the learner allows, handed on as it is while they are calm or lively and switched off while they are still; in
 * a run the pets' stage decides what of it fits a time of concentration. While play is allowed and somebody is on
 * stage, {@link PetsPlay} asks the pets for deeds through the layer's handle. `stage` replaces the render target in
 * tests. */
export function CoursePetsProvider(props: {
  readonly source: CoursePetsSource | undefined;
  readonly choice: PetChoice;
  readonly chosen: boolean;
  readonly play: boolean;
  readonly mischief: boolean;
  readonly state: Pick<CourseState, "step" | "runs" | "course" | "learnerView" | "learner" | "awards">;
  readonly locale: CourseLocale;
  readonly stage?: CoursePetsStage;
  readonly children: ReactNode;
}): ReactElement {
  const { source, choice, play, mischief, state, locale, stage = renderTarget } = props;
  const reducedMotion = useMediaQuery("(prefers-reduced-motion: reduce)");
  const forcedColors = useMediaQuery("(forced-colors: active)");
  const mode = effectivePetMode(choice);
  const [loaded, setLoaded] = useState<LoadedPets | undefined>(undefined);
  const [onStage, setOnStage] = useState<readonly Slug[]>(NO_SPECIES);
  const [questions, setQuestions] = useState<ReadonlyMap<symbol, PetQuestion>>(() => new Map());
  const setQuestion = useCallback((owner: symbol, question: PetQuestion | undefined) => setQuestions((previous) => {
    const held = previous.get(owner);
    if (held?.run === question?.run && held?.task === question?.task) return previous;
    const next = new Map(previous);
    if (question === undefined) next.delete(owner);
    else next.set(owner, question);
    return next;
  }), []);
  const handle = useRef<PetLayerHandle>(null);
  const ask = useCallback((species: Slug, deed: PetDeed) => handle.current?.play(species, deed), []);
  const wanted = mode !== "off" && loaded === undefined;
  useEffect(() => {
    if (!wanted || source === undefined) return;
    let dropped = false;
    Promise.resolve()
      .then(() => Promise.all([source(), stage(), courseHalf()]))
      .then(
        ([menagerie, target, half]) => {
          if (!dropped) setLoaded({ menagerie, target, half });
        },
        () => undefined,
      );
    return () => {
      dropped = true;
    };
  }, [wanted, source, stage]);
  const currentRun = state.step.screen === "run" ? state.step.run : undefined;
  const question = [...questions.values()].find((candidate) => candidate.run === currentRun);
  const scene = loaded?.half.petScene(state.step, state.runs, state.course, question?.task);
  const quiet = state.step.screen === "run";
  const offered = source !== undefined;
  const lively = mode === "calm" || mode === "lively";
  const shiny = useMemo(() => loaded?.half.shinyPetSpecies(loaded.menagerie, state) ?? NO_SPECIES, [loaded, state.course, state.learnerView, state.learner, state.runs, state.awards]);
  const notes = useMemo<CoursePetsNotes>(() => {
    const shown = loaded === undefined || scene === undefined || mode === "off" ? undefined : { ...loaded, scene, mode, quiet, play: lively && play, mischief: lively && mischief, shiny, handle, onCast: setOnStage };
    const players = shown === undefined ? [] : shown.half.petPlayers(shown.menagerie, onStage, locale);
    return {
      offered,
      reduced: offered && reducedMotion,
      forced: offered && forcedColors,
      shown,
      names: players.length === 0 ? NO_NAMES : players.map((player) => player.name),
      playing: shown?.play === true && players.length > 0 ? { players, play: ask } : undefined,
    };
  }, [offered, reducedMotion, forcedColors, loaded, mode, scene, quiet, lively, play, mischief, shiny, ask, onStage, locale]);
  return <PetQuestionContext.Provider value={setQuestion}><CoursePetsContext.Provider value={notes}>{props.children}</CoursePetsContext.Provider></PetQuestionContext.Provider>;
}

/** 🙌️ "Play with the pets", the row of the settings that plays with the pets on stage without a pointer (`row` and
 * `name`: the settings' classes of a row and of its name), drawn by the half of the glue that came with the pets;
 * nothing while the pets are off, still or not fetched, play is not allowed, nobody is on stage or the site has no
 * pets. */
export function PetsPlay(props: { readonly text: CourseText; readonly row: string; readonly name: string }): ReactElement | null {
  const { shown, playing } = useContext(CoursePetsContext);
  return shown === undefined || playing === undefined ? null : <shown.half.PetsPlayground {...props} playing={playing} />;
}

/** 🪧️ The names of the pets on stage right now, in the learner's language; none while no pets show. */
export function usePetCast(): readonly string[] {
  return useContext(CoursePetsContext).names;
}

/** 🐢️ Whether the device asks for reduced motion on a site that has pets. The hint does not change them: they stay as the learner chose, lively until then. */
export function usePetsReduced(): boolean {
  return useContext(CoursePetsContext).reduced;
}

/** 🔲️ Whether the device forces its own colours on a site that has pets: a pet's palette would be lost, so the layer
 * shows nobody whatever the learner chose, and the preferences say so. */
export function usePetsForced(): boolean {
  return useContext(CoursePetsContext).forced;
}
//#endregion 🚚️Loading

//#region 🔘️Switch
/** 🔘️ The switch that shows and hides the pets, for every screen on which they move (WCAG 2.2.2, pause, stop, hide):
 * a native checkbox with its label, so the keyboard operates it and its name is the label's. Nothing on a site
 * without pets. The client puts it on its footer line and stores what it says with the preferences. */
export function PetsSwitch(props: { readonly shown: boolean; readonly label: string; readonly onChange: (shown: boolean) => void }): ReactElement | null {
  const { offered } = useContext(CoursePetsContext);
  if (!offered) return null;
  return (
    <label className="course-target flex w-fit cursor-pointer items-center gap-single text-xs text-muted-foreground" data-pets-switch="">
      <input type="checkbox" className="course-check" checked={props.shown} onChange={(event) => props.onChange(event.target.checked)} />
      {props.label}
    </label>
  );
}
//#endregion 🔘️Switch

//#region 🫧️Layer
/** 🫧️ The pets of the scene on screen, once {@link CoursePetsProvider} fetched them: the layer as the half of the glue
 * that came with them sets it up for the quiz. Nothing while pets are off, not fetched yet or could not be fetched. */
export function CoursePets(): ReactElement | null {
  const pets = useContext(CoursePetsContext).shown;
  return pets === undefined ? null : <pets.half.CoursePetLayer pets={pets} />;
}
//#endregion 🫧️Layer
