/** 🎪️ The pets on stage in the quiz: the half of the quiz's pets glue that matters only once pets came, and so comes
 * with them — a chunk of its own that the glue fetches beside the site's species and the render target the first time
 * a learner wants pets, so none of it weighs on the client's first script. It names the scene of the step on screen
 * and the pets on stage, sets the render target's layer up for the quiz — what pets stand on, what they keep clear of,
 * what of the quiz a press never takes from the learner, the topics they may play with, the other learners' glances
 * and the tempo seam — and draws "Play with the pets" in the settings.
 *
 * It imports no value of the pets product: the render target arrives as the glue hands it in, and the two names the
 * quiz shares with the product ({@link PET_HOME_SCENE}, {@link PET_DEEDS}) are written out, so this chunk and the
 * render target's share no module and load side by side, two chunks and no third; the suite holds the names equal.
 * Nothing runs when it is evaluated, so the package can re-export it without pulling it into the first script.
 *
 * @see ../🟦️.tsx — the glue: what the client needs before the pets come, and the fetch of this half
 * @see ../../../../../../🐾️pets/README.md — the pets product: menageries, stages and frames
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pointer-gestures.html — why every deed of the hand has a button
 * @see https://rollupjs.org/configuration-options/#treeshake-modulesideeffects — why evaluating it must do nothing
 */

import { Component, useId, type ComponentType, type ReactElement, type ReactNode, type RefObject } from "react";
import type { PetLayerHandle, PetLayerProps } from "@semio-tech/pets-react";
import type { Deed, Menagerie, PetMode, Point, Slug } from "@semio-tech/pets";
import { localized, type CourseLabelKey, type CourseLocale, type CourseText } from "../../🌐️i18n/🟦️.ts";
import { HOME_PAGES, type CourseState, type CourseStep } from "../../🧭️session/🧮️state/🟦️.ts";
import { BodyButton, useAnnouncement } from "../../🪟️chrome/🟦️.tsx";

//#region 🎬️Scene
/** 🏡️ The scene of every screen that belongs to no quiz: the render target's own name for it. */
export const PET_HOME_SCENE = "home";

/** 🎬️ The explicit scene of the active topic, quiz or question; general pages show the home ensemble. Questions use
 * only their authored identifiers, so no answer or result can affect their cast. */
export function petScene(step: CourseStep, runs: CourseState["runs"], course: CourseState["course"], question?: string): string {
  if (step.screen === "run" || step.screen === "results") {
    const quiz = runs[step.run]?.quiz;
    return quiz === undefined ? `run-${step.run}` : step.screen === "run" && question !== undefined ? `question-${quiz}-${question}` : quiz;
  }
  if (step.screen !== "home" || step.page === undefined) return PET_HOME_SCENE;
  if (course?.topics?.some((topic) => topic.id === step.page)) return `topic-${step.page}`;
  return course?.quizzes.some((quiz) => quiz.id === step.page) === true ? step.page : Object.values(HOME_PAGES).some((page) => page === step.page) ? PET_HOME_SCENE : `page-${step.page}`;
}

/** ✨️ Species associated with quizzes whose runs earned the current learner a badge, in menagerie order. */
export function shinyPetSpecies(menagerie: Menagerie, state: Pick<CourseState, "course" | "learnerView" | "learner" | "runs" | "awards">): readonly Slug[] {
  const view = state.learnerView?.learner === state.learner?.id ? state.learnerView : undefined;
  const awarded = new Set(view?.badges.map(award => award.run));
  const quizzes = new Set([
    ...view?.runs.filter(run => awarded.has(run.run)).map(run => run.quiz) ?? [],
    ...Object.values(state.runs).filter(run => run.learner === state.learner?.id && (state.awards[run.run]?.length ?? 0) > 0).map(run => run.quiz),
  ]);
  const scenes = new Set(state.course?.quizzes.filter(quiz => quizzes.has(quiz.id)).flatMap(quiz => [quiz.id, ...quiz.tasks.map(task => `question-${quiz.id}-${task.id}`)]));
  const shiny = new Set(menagerie.casts.filter(cast => scenes.has(cast.scene)).flatMap(cast => [...cast.core, ...cast.rotation]));
  return menagerie.species.filter(species => shiny.has(species.id)).map(species => species.id);
}

/** 👯️ The pets of `species` the menagerie knows, in the order given, each with its display name in `locale`; a species
 * the menagerie does not know is left out. */
export function petPlayers(menagerie: Menagerie, species: readonly Slug[], locale: CourseLocale): readonly PetPlayer[] {
  return species.flatMap((id) => {
    const known = menagerie.species.find((entry) => entry.id === id);
    return known === undefined ? [] : [{ species: id, name: localized(known.name, locale) }];
  });
}

/** 📛️ The display names of `species` in `locale`, in the order given; a species the menagerie does not know has none. */
export function petNames(menagerie: Menagerie, species: readonly Slug[], locale: CourseLocale): readonly string[] {
  return petPlayers(menagerie, species, locale).map((player) => player.name);
}
//#endregion 🎬️Scene

//#region 🎾️Play
/** 🎾️ What a learner can ask a pet on stage for without a pointer — a hello, a trick, a stroke, a toss into the air —
 * in the order the settings offer them: the stage's own deeds. */
export const PET_DEEDS = ["hello", "trick", "pet", "toss"] as const satisfies readonly Deed[];

/** 🕹️ One of {@link PET_DEEDS}. */
export type PetDeed = (typeof PET_DEEDS)[number];

/** 🏷️ The button of every {@link PetDeed} beside the name of a pet in "Play with the pets". */
export const PET_DEED_LABELS: { readonly [D in PetDeed]: CourseLabelKey } = { hello: "course.preferences.petsHello", trick: "course.preferences.petsTrick", pet: "course.preferences.petsPet", toss: "course.preferences.petsToss" };

/** 📣️ What the status line of "Play with the pets" says once the learner asked a pet for a {@link PetDeed} — what was
 * asked, never what the pet then does or what the pets do by themselves (`{{name}}` is the pet). */
export const PET_DEED_SAID: { readonly [D in PetDeed]: CourseLabelKey } = { hello: "course.preferences.petsHelloSaid", trick: "course.preferences.petsTrickSaid", pet: "course.preferences.petsPetSaid", toss: "course.preferences.petsTossSaid" };

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

/** 🎭️ Where the render target comes from: its pet layer and what that layer keeps clear of by itself. */
export type CoursePetsStage = () => Promise<{ readonly PetLayer: ComponentType<PetLayerProps>; readonly PET_KEEPOUTS: string }>;

/** 🎟️ What the layer of the quiz draws once the pets came: the species, the render target, the scene of the step on screen,
 * how lively the pets are, whether a run asks for quiet, what the learner allows (`play`, `mischief`), the handle the
 * settings ask for deeds by, and where the layer says who is on stage. */
export interface StagedPets {
  readonly menagerie: Menagerie;
  readonly target: Awaited<ReturnType<CoursePetsStage>>;
  readonly scene: string;
  readonly mode: PetMode;
  readonly quiet: boolean;
  readonly play: boolean;
  readonly mischief: boolean;
  readonly shiny: readonly Slug[];
  readonly handle: RefObject<PetLayerHandle | null>;
  readonly onCast: (species: readonly Slug[]) => void;
}
//#endregion 🎾️Play

//#region 🗺️Stage
/** 🪵️ What pets stand on in the quiz: what one sees of every card of the screen — its title tab, which stands up on
 * the left, and the edge of its body beside the tab, one tab height lower (the glass behind the body, which the window
 * chrome lays out once it has measured its silhouette) — and the footer line of the client, which is the floor. The box
 * of a card as a whole starts with the row of the tab, most of which is empty: a pet on that edge would hover. The
 * cards of the pages behind the overview are inert and the cards of a dialog lie outside `main`, so neither carries a
 * pet. A card the overview conceals while the pointer hovers another (`data-concealed` on its host) is unseen too:
 * the survey drops its edge, its sides and anything a pet was playing with on it, and whatever stood there falls. */
export const COURSE_PET_SURFACES = '#course-main [data-card] [data-slot="window-chrome-chip-cap"], #course-main [data-card] [data-slot="window-chrome-body-surface"], .course-app > footer';

/** 🚧️ What pets keep clear of in the quiz besides what the layer avoids anyway: the items a learner drags, the zones
 * they are dropped on, and the navigation bar as a whole — its mark and its title are neither controls nor text
 * blocks, and a pet on the topmost card of a page would stand in front of them. */
export const COURSE_PET_KEEPOUTS = "[data-course-item], [data-course-drop], .course-app > header";

/** 🖱️ What of the quiz acts on a press although the layer's own rules call it no control, so a press there is never a
 * pet's: the grip that starts the drag of an item (an `aria-hidden` span), the zones an item is dropped on, and the cards
 * of the overview, which open their page on a click anywhere but on a control. Their actions are buttons, which every
 * layer leaves to the learner anyway. */
export const COURSE_PET_CONTROLS = "[data-course-grip], [data-course-drop], [data-layered-card]";

/** 🧸️ What pets may play with in the quiz: every element marked with a topic key (`data-pet-prop`) — except the copy of
 * a row a drag carries, a deep clone of the row with all of its attributes. */
export const COURSE_PET_PROPS = "[data-pet-prop]:not(.course-drag-ghost *)";

/** ⏩️ The attribute of the document root that makes the pets' time pass faster: how many times as fast as the wall
 * clock (the layer holds it between an eighth and eight). A test seam: nothing in the client ever sets it. The
 * end-to-end proofs that wait for a walk or for two pets that meet set it before the client starts, because that takes
 * a minute or two of real time. */
export const COURSE_PETS_TEMPO = "data-pets-tempo";

/** ⏱️ How fast the pets' time passes: what {@link COURSE_PETS_TEMPO} says on the document root, 1 without it. */
export function petsTempo(): number {
  const said = Number(document.documentElement.getAttribute(COURSE_PETS_TEMPO) ?? "");
  return said > 0 && said < Infinity ? said : 1;
}

/** 👀️ Where the other learners point right now, in viewport pixels: the tips of their cursor marks. None while their
 * cursors are switched off or nobody else is on the page, because the presence layer does not exist then. */
export function peerGlances(): readonly Point[] {
  return [...document.querySelectorAll<HTMLElement>("[data-presence-layer] .course-peer:not([hidden])")].map((mark) => {
    const box = mark.getBoundingClientRect();
    return { x: box.left, y: box.top };
  });
}
//#endregion 🗺️Stage

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

/** 🫧️ The render target's layer as the quiz sets it up for the pets the glue shows: one decorative layer over the whole
 * client that stands on its cards ({@link COURSE_PET_SURFACES}), keeps clear of what a learner reads, operates and drags
 * ({@link COURSE_PET_KEEPOUTS} beside the layer's own), answers the learner's hand where nothing of the quiz acts on a press
 * ({@link COURSE_PET_CONTROLS}), plays with copies of the marked topics ({@link COURSE_PET_PROPS}), glances at the other
 * learners' cursors, says who is on stage and takes the deeds of the settings by its handle — as far as the learner
 * allows play and mischief. A layer that throws is taken away and the quiz stays. */
export function CoursePetLayer(props: { readonly pets: StagedPets }): ReactElement {
  const { pets } = props;
  const { PetLayer: Layer, PET_KEEPOUTS } = pets.target;
  return (
    <PetBoundary>
      <Layer
        key={pets.scene}
        ref={pets.handle}
        menagerie={pets.menagerie}
        scene={pets.scene}
        mode={pets.mode}
        quiet={pets.quiet}
        play={pets.play}
        mischief={pets.mischief}
        shiny={pets.shiny}
        surfaces={COURSE_PET_SURFACES}
        keepouts={`${PET_KEEPOUTS}, ${COURSE_PET_KEEPOUTS}`}
        controls={COURSE_PET_CONTROLS}
        props={COURSE_PET_PROPS}
        glances={peerGlances}
        onCast={pets.onCast}
        tempo={petsTempo()}
      />
    </PetBoundary>
  );
}
//#endregion 🫧️Layer

//#region 🤹️Playground
/** 🤹️ "Play with the pets", a row of the settings (`row`, with its name in `name`: the settings' own classes) that plays
 * with the pets on stage without a pointer (WCAG 2.1.1 keyboard, 2.5.1 pointer gestures, 2.5.7 dragging movements): one
 * group per pet, named by the pet, with a button for each deed — a hello, a trick, a stroke, a toss — and one polite
 * status line that says what the learner just asked for, and only that: nothing the pets do by themselves is spoken.
 * Focus stays on the pressed button. The names are a column, so every pet's buttons start at the same place and wrap
 * beside its name. */
export function PetsPlayground(props: { readonly playing: PetPlay; readonly text: CourseText; readonly row: string; readonly name: string }): ReactElement {
  const { playing, text } = props;
  const scope = useId();
  const { announcement, announce } = useAnnouncement();
  return (
    <div className={props.row}>
      <span className={props.name} aria-hidden="true">
        {text("course.preferences.petsPlayWith")}
      </span>
      <div role="group" aria-label={text("course.preferences.petsPlayWith")} className="grid min-w-0 grid-cols-[max-content_minmax(0,1fr)] items-center gap-single" data-pets-play="">
        {playing.players.map((player) => (
          <div key={player.species} role="group" aria-labelledby={`${scope}-${player.species}`} className="col-span-2 grid grid-cols-subgrid items-center" data-pets-player={player.species}>
            <span id={`${scope}-${player.species}`} className="text-sm font-semibold">
              {player.name}
            </span>
            <span className="flex min-w-0 flex-wrap gap-single">
              {PET_DEEDS.map((deed) => (
                <BodyButton
                  key={deed}
                  onClick={() => {
                    playing.play(player.species, deed);
                    announce(text(PET_DEED_SAID[deed], { name: player.name }));
                  }}
                >
                  {text(PET_DEED_LABELS[deed])}
                </BodyButton>
              ))}
            </span>
          </div>
        ))}
        <p role="status" aria-live="polite" aria-atomic="true" className="col-span-2 m-0 text-xs text-muted-foreground">
          {announcement.message === "" ? null : <span key={announcement.serial}>{announcement.message}</span>}
        </p>
      </div>
    </div>
  );
}
//#endregion 🤹️Playground
