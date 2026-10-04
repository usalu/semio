/** 🐾️ Pets in the quiz: small animated companions that fit the topic on screen. This module is the quiz's glue to the
 * pets product, the half of it the client needs before any pet came — which liveliness the learner chose, the switch
 * that shows and hides them on every screen, the topics the quiz marks, the lazy loading of the species, of the layer
 * that draws them and of the other half of the glue, and what the preferences and the client read of the pets once
 * they came. The other half (`./🎪️stage/🟦️.tsx`) comes with the pets: the scene of the step on screen, the names of
 * the pets on stage, the layer as the quiz sets it up — what pets stand on, what of the quiz a press never takes from
 * the learner, the topics they may play with — and the deeds the settings ask the pets on stage for.
 *
 * The quiz knows no species. A site hands its menagerie in as a {@link QuizPetsSource}; the menagerie, the render
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
 * A device that asks for reduced motion decides the default only: its learner gets motionless pets until they choose
 * a liveliness or use the switch, and what a learner chose holds on every device. (A remote desktop session reports
 * reduced motion in every browser; a learner who asks for moving pets there gets them.)
 *
 * @see ./🎪️stage/🟦️.tsx — the half of the glue that comes with the pets
 * @see ../../../../../🐾️pets/README.md — the pets product: menageries, stages and frames
 * @see ../🎛️preferences/🟦️.tsx — where the learner chooses how lively they are, and where that they chose is stored
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html — why `still`, `off` and the switch exist
 * @see https://www.w3.org/TR/mediaqueries-5/#prefers-reduced-motion — the device's hint
 */

import { createContext, useCallback, useContext, useEffect, useMemo, useRef, useState, type ComponentType, type ReactElement, type ReactNode, type RefObject } from "react";
import { useMediaQuery } from "@semio-tech/ui-react/chrome";
import type { Menagerie, PetMode, Slug } from "@semio-tech/pets";
import type { PetLayerHandle, PetLayerProps } from "@semio-tech/pets-react";
import type { QuizLocale, QuizText } from "../🌐️i18n/🟦️.ts";
import type { QuizState } from "../🧭️session/🟦️.ts";
import type { PetDeed } from "./🎪️stage/🟦️.tsx";

//#region 🎚️Choice
/** 🎚️ What a learner may choose for the pets: none at all, motionless, slightly active or busy. */
export const PET_CHOICES = ["off", "still", "calm", "lively"] as const;

/** 🔢️ One of {@link PET_CHOICES}. */
export type PetChoice = (typeof PET_CHOICES)[number];

/** 🌡️ How lively pets are while they show: every {@link PetChoice} but `off`. */
export type PetLiveliness = Exclude<PetChoice, "off">;

/** 🧘️ The liveliness the pets get. A choice the learner made (`chosen`) holds as it is, whatever the device asks for.
 * Until the learner chooses, the device decides the default: motionless on one that asks for reduced motion, else the
 * `choice` the preferences start with. Pets that are off stay off either way. Forced colours are not its concern: the
 * layer shows nobody under them, whatever its mode. */
export function effectivePetMode(choice: PetChoice, chosen: boolean, reducedMotion: boolean): PetChoice {
  return choice === "off" || chosen || !reducedMotion ? choice : "still";
}

/** 🔀️ What the pets switch makes of a choice: `off` when the pets are hidden, and when they are shown again the
 * liveliness they had before (`before`). */
export function switchedPets(shown: boolean, before: PetLiveliness): PetChoice {
  return shown ? before : "off";
}
//#endregion 🎚️Choice

//#region 🎾️Play
/** 🐕️ A pet on stage: its species and its display name in the learner's language. */
export interface PetPlayer {
  readonly species: Slug;
  readonly name: string;
}

/** 🤹️ What the settings need to play with the pets without a pointer: who is on stage, in the order the layer names
 * them, and how to ask one of them for a deed (whether and how it answers is its stage's to decide). */
export interface PetPlay {
  readonly players: readonly PetPlayer[];
  readonly play: (species: Slug, deed: PetDeed) => void;
}
//#endregion 🎾️Play

//#region 🗺️Topics
/** 🔑️ The topic key of `part` within `topic` (`<quiz>` or `<quiz>/<task>`): `<topic>/<part>`, one of the grounds a
 * species may name; none without a topic. A key says what an element is about, never a value or an answer. */
export function petProp(topic: string | undefined, part: string): string | undefined {
  return topic === undefined ? undefined : `${topic}/${part}`;
}

const PetTopicContext = createContext<string | undefined>(undefined);

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
export type QuizPetsSource = () => Promise<Menagerie>;

/** 🎭️ Where the render target comes from: its pet layer and what that layer keeps clear of by itself. */
export type QuizPetsStage = () => Promise<{ readonly PetLayer: ComponentType<PetLayerProps>; readonly PET_KEEPOUTS: string }>;

/** 📦️ The render target as a chunk of its own (with its stylesheet), fetched when pets are first wanted. */
const renderTarget: QuizPetsStage = () => import("@semio-tech/pets-react");

/** 🎒️ The half of the glue that comes with the pets (`./🎪️stage/🟦️.tsx`). */
type QuizPetsHalf = typeof import("./🎪️stage/🟦️.tsx");

/** 🧳️ The half of the glue that comes with the pets as a chunk of its own, fetched beside the render target. */
const quizHalf = (): Promise<QuizPetsHalf> => import("./🎪️stage/🟦️.tsx");

/** 🎟️ What the layer shows once the pets came: the species, the render target and the half of the glue that came with
 * them, the scene of the step on screen, how lively the pets are, whether a run asks for quiet, what the learner allows
 * (`play`, `mischief`), the handle the settings ask for deeds by, and where the layer says who is on stage. */
export interface ShownPets {
  readonly menagerie: Menagerie;
  readonly target: Awaited<ReturnType<QuizPetsStage>>;
  readonly half: QuizPetsHalf;
  readonly scene: string;
  readonly mode: PetMode;
  readonly quiet: boolean;
  readonly play: boolean;
  readonly mischief: boolean;
  readonly handle: RefObject<PetLayerHandle | null>;
  readonly onCast: (species: readonly Slug[]) => void;
}

type LoadedPets = Pick<ShownPets, "menagerie" | "target" | "half">;

/** 🗒️ What the client knows about its pets: whether the site has any (`offered`), whether the device asks for reduced
 * motion and thereby holds the pets of a learner who has not chosen still (`reduced`), whether it forces its own colours, under which the layer
 * shows nobody (`forced`), what the layer draws once it is fetched (`shown`), the names of the pets on stage right
 * now, in the learner's language (`names`), and how the settings play with them (`playing`: only while they may). */
interface QuizPetsNotes {
  readonly offered: boolean;
  readonly reduced: boolean;
  readonly forced: boolean;
  readonly shown: ShownPets | undefined;
  readonly names: readonly string[];
  readonly playing: PetPlay | undefined;
}

const NO_NAMES: readonly string[] = [];
const NO_SPECIES: readonly Slug[] = [];
const NO_PETS: QuizPetsNotes = { offered: false, reduced: false, forced: false, shown: undefined, names: NO_NAMES, playing: undefined };
const QuizPetsContext = createContext<QuizPetsNotes>(NO_PETS);

/** 🚚️ Fetches the menagerie of `source`, the render target and the half of the glue that comes with the pets once the
 * learner's `choice` asks for pets, and makes them — with the scene of the step in `state` — what {@link QuizPets}
 * draws, while {@link usePetCast} names, in `locale`, whoever the layer says is on stage. Whether the learner made
 * that choice (`chosen`) decides what a device that asks for reduced motion gets ({@link effectivePetMode}). Nothing is
 * fetched without a source or while the choice is `off`; a fetch that fails is forgotten (choosing pets again tries anew) and one that outlives the provider or the
 * wish is dropped. A run is a time of concentration: the pets rest while one is on screen. Results are not: a learner who reads
 * a score is done concentrating, the pets live again, and the switch on that screen hides them for whoever minds.
 * Whether the pets answer clicks and can be picked up (`play`) and whether they may play with the page (`mischief`) is
 * what the learner allows, handed on as it is while they are calm or lively and switched off while they are still; in
 * a run the pets' stage decides what of it fits a time of concentration. While play is allowed and somebody is on
 * stage, {@link PetsPlay} asks the pets for deeds through the layer's handle. `stage` replaces the render target in
 * tests. */
export function QuizPetsProvider(props: {
  readonly source: QuizPetsSource | undefined;
  readonly choice: PetChoice;
  readonly chosen: boolean;
  readonly play: boolean;
  readonly mischief: boolean;
  readonly state: Pick<QuizState, "step" | "runs" | "catalog">;
  readonly locale: QuizLocale;
  readonly stage?: QuizPetsStage;
  readonly children: ReactNode;
}): ReactElement {
  const { source, choice, chosen, play, mischief, state, locale, stage = renderTarget } = props;
  const reducedMotion = useMediaQuery("(prefers-reduced-motion: reduce)");
  const forcedColors = useMediaQuery("(forced-colors: active)");
  const mode = effectivePetMode(choice, chosen, reducedMotion);
  const [loaded, setLoaded] = useState<LoadedPets | undefined>(undefined);
  const [onStage, setOnStage] = useState<readonly Slug[]>(NO_SPECIES);
  const handle = useRef<PetLayerHandle>(null);
  const ask = useCallback((species: Slug, deed: PetDeed) => handle.current?.play(species, deed), []);
  const wanted = mode !== "off" && loaded === undefined;
  useEffect(() => {
    if (!wanted || source === undefined) return;
    let dropped = false;
    Promise.resolve()
      .then(() => Promise.all([source(), stage(), quizHalf()]))
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
  const scene = loaded?.half.petScene(state.step, state.runs, state.catalog);
  const quiet = state.step.screen === "run";
  const offered = source !== undefined;
  const lively = mode === "calm" || mode === "lively";
  const notes = useMemo<QuizPetsNotes>(() => {
    const shown = loaded === undefined || scene === undefined || mode === "off" ? undefined : { ...loaded, scene, mode, quiet, play: lively && play, mischief: lively && mischief, handle, onCast: setOnStage };
    const players = shown === undefined ? [] : shown.half.petPlayers(shown.menagerie, onStage, locale);
    return {
      offered,
      reduced: offered && reducedMotion,
      forced: offered && forcedColors,
      shown,
      names: players.length === 0 ? NO_NAMES : players.map((player) => player.name),
      playing: shown?.play === true && players.length > 0 ? { players, play: ask } : undefined,
    };
  }, [offered, reducedMotion, forcedColors, loaded, mode, scene, quiet, lively, play, mischief, ask, onStage, locale]);
  return <QuizPetsContext.Provider value={notes}>{props.children}</QuizPetsContext.Provider>;
}

/** 🙌️ "Play with the pets", the row of the settings that plays with the pets on stage without a pointer (`row` and
 * `name`: the settings' classes of a row and of its name), drawn by the half of the glue that came with the pets;
 * nothing while the pets are off, still or not fetched, play is not allowed, nobody is on stage or the site has no
 * pets. */
export function PetsPlay(props: { readonly text: QuizText; readonly row: string; readonly name: string }): ReactElement | null {
  const { shown, playing } = useContext(QuizPetsContext);
  return shown === undefined || playing === undefined ? null : <shown.half.PetsPlayground {...props} playing={playing} />;
}

/** 🪧️ The names of the pets on stage right now, in the learner's language; none while no pets show. */
export function usePetCast(): readonly string[] {
  return useContext(QuizPetsContext).names;
}

/** 🐢️ Whether the device asks for reduced motion on a site that has pets: the pets then stay still until the learner
 * chooses how lively they are, and the preferences say so. */
export function usePetsReduced(): boolean {
  return useContext(QuizPetsContext).reduced;
}

/** 🔲️ Whether the device forces its own colours on a site that has pets: a pet's palette would be lost, so the layer
 * shows nobody whatever the learner chose, and the preferences say so. */
export function usePetsForced(): boolean {
  return useContext(QuizPetsContext).forced;
}
//#endregion 🚚️Loading

//#region 🔘️Switch
/** 🔘️ The switch that shows and hides the pets, for every screen on which they move (WCAG 2.2.2, pause, stop, hide):
 * a native checkbox with its label, so the keyboard operates it and its name is the label's. Nothing on a site
 * without pets. The client puts it on its footer line and stores what it says with the preferences. */
export function PetsSwitch(props: { readonly shown: boolean; readonly label: string; readonly onChange: (shown: boolean) => void }): ReactElement | null {
  const { offered } = useContext(QuizPetsContext);
  if (!offered) return null;
  return (
    <label className="quiz-target flex w-fit cursor-pointer items-center gap-single text-xs text-muted-foreground" data-pets-switch="">
      <input type="checkbox" className="quiz-check" checked={props.shown} onChange={(event) => props.onChange(event.target.checked)} />
      {props.label}
    </label>
  );
}
//#endregion 🔘️Switch

//#region 🫧️Layer
/** 🫧️ The pets of the scene on screen, once {@link QuizPetsProvider} fetched them: the layer as the half of the glue
 * that came with them sets it up for the quiz. Nothing while pets are off, not fetched yet or could not be fetched. */
export function QuizPets(): ReactElement | null {
  const pets = useContext(QuizPetsContext).shown;
  return pets === undefined ? null : <pets.half.QuizPetLayer pets={pets} />;
}
//#endregion 🫧️Layer
