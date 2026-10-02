/** 🐾️ Pets in the quiz: small animated companions that fit the topic on screen. This module is the quiz's glue to the
 * pets product — which liveliness the learner chose, the switch that shows and hides them on every screen, which scene
 * the step on screen belongs to, the lazy loading of the species and of the layer that draws them, and the names of
 * the pets on stage for the preferences.
 *
 * The quiz knows no species. A site hands its menagerie in as a {@link QuizPetsSource}; the menagerie and the render
 * target (`@semio-tech/pets-react`) are fetched together, and only once the learner's choice is not `off`, so a learner
 * without pets never downloads them. Pets are decoration: a load that fails leaves the quiz as it is without a word, a
 * layer that throws is taken away (React itself reports what it caught), and the layer never takes the pointer, the
 * keyboard or a place in the accessibility tree.
 *
 * A device that asks for reduced motion decides the default only: its learner gets motionless pets until they choose
 * a liveliness or use the switch, and what a learner chose holds on every device. (A remote desktop session reports
 * reduced motion in every browser; a learner who asks for moving pets there gets them.)
 *
 * @see ../../../../../🐾️pets/README.md — the pets product: menageries, stages and frames
 * @see ../🎛️preferences/🟦️.tsx — where the learner chooses how lively they are, and where that they chose is stored
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html — why `still`, `off` and the switch exist
 * @see https://www.w3.org/TR/mediaqueries-5/#prefers-reduced-motion — the device's hint
 */

import { Component, createContext, useContext, useEffect, useMemo, useState, type ComponentType, type ReactElement, type ReactNode } from "react";
import { useMediaQuery } from "@semio-tech/ui-react/chrome";
import type { Menagerie, PetMode, Point, Slug } from "@semio-tech/pets";
import type { PetLayerProps } from "@semio-tech/pets-react";
import { localized, type QuizLocale } from "../🌐️i18n/🟦️.ts";
import type { QuizState, QuizStep } from "../🧭️session/🟦️.ts";

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

//#region 🎬️Scene
/** 🏡️ The scene of every screen that belongs to no quiz. */
export const PET_HOME_SCENE = "home";

/** 🎬️ The scene whose cast fits the step on screen: the id of the quiz while its page is opened, it is being played or
 * its results show; {@link PET_HOME_SCENE} on the overview, its other pages, the introduction and the identity step. */
export function petScene(step: QuizStep, runs: QuizState["runs"], catalog: QuizState["catalog"]): string {
  if (step.screen === "run" || step.screen === "results") return runs[step.run]?.quiz ?? PET_HOME_SCENE;
  if (step.screen !== "home" || step.page === undefined) return PET_HOME_SCENE;
  return catalog?.quizzes.some((quiz) => quiz.id === step.page) === true ? step.page : PET_HOME_SCENE;
}

/** 📛️ The display names of `species` in `locale`, in the order given; a species the menagerie does not know has none. */
export function petNames(menagerie: Menagerie, species: readonly Slug[], locale: QuizLocale): readonly string[] {
  return species.flatMap((id) => {
    const known = menagerie.species.find((entry) => entry.id === id);
    return known === undefined ? [] : [localized(known.name, locale)];
  });
}
//#endregion 🎬️Scene

//#region 🗺️Stage
/** 🪵️ What pets stand on in the quiz: what one sees of every card of the screen — its title tab, which stands up on
 * the left, and the edge of its body beside the tab, one tab height lower (the glass behind the body, which the window
 * chrome lays out once it has measured its silhouette) — and the footer line of the client, which is the floor. The box
 * of a card as a whole starts with the row of the tab, most of which is empty: a pet on that edge would hover. The
 * cards of the pages behind the overview are inert and the cards of a dialog lie outside `main`, so neither carries a
 * pet. */
export const QUIZ_PET_SURFACES = '#quiz-main [data-card] [data-slot="window-chrome-chip-cap"], #quiz-main [data-card] [data-slot="window-chrome-body-surface"], .quiz-app > footer';

/** 🚧️ What pets keep clear of in the quiz besides what the layer avoids anyway: the items a learner drags, the zones
 * they are dropped on, and the navigation bar as a whole — its mark and its title are neither controls nor text
 * blocks, and a pet on the topmost card of a page would stand in front of them. */
export const QUIZ_PET_KEEPOUTS = "[data-quiz-item], [data-quiz-drop], .quiz-app > header";

/** ⏩️ The attribute of the document root that makes the pets' time pass faster: how many times as fast as the wall
 * clock (the layer holds it between an eighth and eight). A test seam: nothing in the client ever sets it. The
 * end-to-end proofs that wait for a walk or for two pets that meet set it before the client starts, because that takes
 * a minute or two of real time. */
export const QUIZ_PETS_TEMPO = "data-pets-tempo";

/** ⏱️ How fast the pets' time passes: what {@link QUIZ_PETS_TEMPO} says on the document root, 1 without it. */
export function petsTempo(): number {
  const said = Number(document.documentElement.getAttribute(QUIZ_PETS_TEMPO) ?? "");
  return said > 0 && said < Infinity ? said : 1;
}

/** 👀️ Where the other learners point right now, in viewport pixels: the tips of their cursor marks. None while their
 * cursors are switched off or nobody else is on the page, because the presence layer does not exist then. */
export function peerGlances(): readonly Point[] {
  return [...document.querySelectorAll<HTMLElement>("[data-presence-layer] .quiz-peer:not([hidden])")].map((mark) => {
    const box = mark.getBoundingClientRect();
    return { x: box.left, y: box.top };
  });
}
//#endregion 🗺️Stage

//#region 🚚️Loading
/** 🎪️ Where a site's menagerie comes from: called at most once per mounted attempt (a development build that mounts
 * twice asks twice and drops the first answer), and only when pets are wanted. */
export type QuizPetsSource = () => Promise<Menagerie>;

/** 🎭️ Where the render target comes from: its pet layer and what that layer keeps clear of by itself. */
export type QuizPetsStage = () => Promise<{ readonly PetLayer: ComponentType<PetLayerProps>; readonly PET_KEEPOUTS: string }>;

/** 📦️ The render target as a chunk of its own (with its stylesheet), fetched when pets are first wanted. */
const renderTarget: QuizPetsStage = () => import("@semio-tech/pets-react");

interface LoadedPets {
  readonly menagerie: Menagerie;
  readonly Layer: ComponentType<PetLayerProps>;
  readonly keepouts: string;
}

interface ShownPets extends LoadedPets {
  readonly scene: string;
  readonly mode: PetMode;
  readonly quiet: boolean;
  readonly onCast: (species: readonly Slug[]) => void;
}

/** 🗒️ What the client knows about its pets: whether the site has any (`offered`), whether the device asks for reduced
 * motion and thereby holds the pets of a learner who has not chosen still (`reduced`), whether it forces its own colours, under which the layer
 * shows nobody (`forced`), what the layer draws once it is fetched (`shown`) and the names of the pets on stage right
 * now, in the learner's language (`names`). */
interface QuizPetsNotes {
  readonly offered: boolean;
  readonly reduced: boolean;
  readonly forced: boolean;
  readonly shown: ShownPets | undefined;
  readonly names: readonly string[];
}

const NO_NAMES: readonly string[] = [];
const NO_SPECIES: readonly Slug[] = [];
const NO_PETS: QuizPetsNotes = { offered: false, reduced: false, forced: false, shown: undefined, names: NO_NAMES };
const QuizPetsContext = createContext<QuizPetsNotes>(NO_PETS);

/** 🚚️ Fetches the menagerie of `source` and the render target once the learner's `choice` asks for pets, and makes
 * them — with the scene of the step in `state` — what {@link QuizPets} draws, while {@link usePetCast} names, in
 * `locale`, whoever the layer says is on stage. Whether the learner made that choice (`chosen`) decides what a device
 * that asks for reduced motion gets ({@link effectivePetMode}). Nothing is fetched without a source or while the choice
 * is `off`; a fetch that fails is forgotten (choosing pets again tries anew) and one that outlives the provider or the
 * wish is dropped. A run is a time of concentration: the pets rest while one is on screen. Results are not: a learner who reads
 * a score is done concentrating, the pets live again, and the switch on that screen hides them for whoever minds.
 * `stage` replaces the render target in tests. */
export function QuizPetsProvider(props: {
  readonly source: QuizPetsSource | undefined;
  readonly choice: PetChoice;
  readonly chosen: boolean;
  readonly state: Pick<QuizState, "step" | "runs" | "catalog">;
  readonly locale: QuizLocale;
  readonly stage?: QuizPetsStage;
  readonly children: ReactNode;
}): ReactElement {
  const { source, choice, chosen, state, locale, stage = renderTarget } = props;
  const reducedMotion = useMediaQuery("(prefers-reduced-motion: reduce)");
  const forcedColors = useMediaQuery("(forced-colors: active)");
  const mode = effectivePetMode(choice, chosen, reducedMotion);
  const [loaded, setLoaded] = useState<LoadedPets | undefined>(undefined);
  const [onStage, setOnStage] = useState<readonly Slug[]>(NO_SPECIES);
  const wanted = mode !== "off" && loaded === undefined;
  useEffect(() => {
    if (!wanted || source === undefined) return;
    let dropped = false;
    Promise.resolve()
      .then(() => Promise.all([source(), stage()]))
      .then(
        ([menagerie, target]) => {
          if (!dropped) setLoaded({ menagerie, Layer: target.PetLayer, keepouts: `${target.PET_KEEPOUTS}, ${QUIZ_PET_KEEPOUTS}` });
        },
        () => undefined,
      );
    return () => {
      dropped = true;
    };
  }, [wanted, source, stage]);
  const scene = petScene(state.step, state.runs, state.catalog);
  const quiet = state.step.screen === "run";
  const offered = source !== undefined;
  const notes = useMemo<QuizPetsNotes>(() => {
    const shown = loaded === undefined || mode === "off" ? undefined : { ...loaded, scene, mode, quiet, onCast: setOnStage };
    return { offered, reduced: offered && reducedMotion, forced: offered && forcedColors, shown, names: shown === undefined ? NO_NAMES : petNames(shown.menagerie, onStage, locale) };
  }, [offered, reducedMotion, forcedColors, loaded, mode, scene, quiet, onStage, locale]);
  return <QuizPetsContext.Provider value={notes}>{props.children}</QuizPetsContext.Provider>;
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
/** 🧯️ Keeps a pet layer that throws from taking the quiz down with it: the pets vanish, everything else stays. */
class PetBoundary extends Component<{ readonly children: ReactNode }, { readonly failed: boolean }> {
  override state = { failed: false };

  static getDerivedStateFromError(): { readonly failed: boolean } {
    return { failed: true };
  }

  override render(): ReactNode {
    return this.state.failed ? null : this.props.children;
  }
}

/** 🫧️ The pets of the scene on screen, once {@link QuizPetsProvider} fetched them: one decorative layer over the whole
 * client that stands on its cards, keeps clear of what a learner reads, operates and drags, glances at the other
 * learners' cursors and says who is on stage. Nothing while pets are off, not fetched yet or could not be fetched. */
export function QuizPets(): ReactElement | null {
  const pets = useContext(QuizPetsContext).shown;
  if (pets === undefined) return null;
  const { Layer } = pets;
  return (
    <PetBoundary>
      <Layer menagerie={pets.menagerie} scene={pets.scene} mode={pets.mode} quiet={pets.quiet} surfaces={QUIZ_PET_SURFACES} keepouts={pets.keepouts} glances={peerGlances} onCast={pets.onCast} tempo={petsTempo()} />
    </PetBoundary>
  );
}
//#endregion 🫧️Layer
