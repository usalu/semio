/** 🌲️ Home groups general pages, quiz topics and individual quizzes into nested layered overviews. Each card opens its
 * full page or another grid using the same glass, pointer parallax, hover reveal and zoom. A single page address derives
 * every opened ancestor. Hovering a card's explore control zooms the grid immediately behind it. Every grid behind that follows the pointer without zooming. Only the deepest active grid handles input; background previews remain live and inert.
 * @see ./🗺️pages/🟦️.ts — course-derived page hierarchy
 * @see ../🚏️navigation/🟦️.tsx — history and parent navigation
 */
import { createContext, useCallback, useContext, useEffect, useId, useLayoutEffect, useMemo, useRef, useState, type CSSProperties, type ReactElement, type ReactNode } from "react";
import { LayeredOverview, UI_MOBILE_MAX_WIDTH_PX, UI_TABLET_MAX_WIDTH_PX, stripGrid, useMediaQuery, type IconName, type LayeredCardState, type LayeredCell, type LayeredDirection, type LayeredMode, type LayeredPane } from "@semio-tech/ui-react/chrome";
import { type CourseLocale, type CourseText } from "../🌐️i18n/🟦️.ts";
import { failureProblem, thrownProblem } from "../🪪️identity/🟦️.tsx";
import { LEADERBOARD_POLL_MS, LeaderboardCard, LeaderboardPage, usePolling } from "../🏆️leaderboard/🟦️.tsx";
import { BadgesCard, BadgesPage } from "../🏅️badges/🟦️.tsx";
import { PreferencesCard, PreferencesPage, challengeOf, textScale, withChallenge, type CoursePreferences } from "../🎛️preferences/🟦️.tsx";
import { LearnerCard, LearnerPage } from "../📇️profile/🟦️.tsx";
import { QuizCardView, QuizPage, type Act } from "../📖️quiz-page/🟦️.tsx";
import { BodyButton, CardAction, CardIcon, Glyph, PageFrame, ProblemNote, CourseCard, type Problem } from "../🪟️chrome/🟦️.tsx";
import { isHomeGrid, openedChild, pageLabel, pagesWithin, parentPage, previewCamera, type PreviewCamera } from "./🗺️pages/🟦️.ts";
export { addressPages, homePages, isHomeGrid, openedChild, pageLabel, pagesWithin, parentPage, previewCamera } from "./🗺️pages/🟦️.ts";
export type { PreviewCamera } from "./🗺️pages/🟦️.ts";
import { CardPresence } from "../👥️presence/🟦️.tsx";
import { PRESENCE_ANCHORS } from "../👥️presence/👁️view/🟦️.ts";
import { usePresenceFollow } from "../👀️follow/🟦️.tsx";
import { HOME_PAGES, type CourseState } from "../🧭️session/🧮️state/🟦️.ts";
import { type CourseSession } from "../🧭️session/🟦️.ts";

/** 🗺️ The grid of an overview: three columns or two on a tablet. */
export type HomeGrid = "desktop" | "tablet";

/** 🏡️ How the overview is laid out: its grid, and whether its cards stand fixed on it (`strip`) or the learner swipes through it
 * one page at a time (`swipe`). */
export interface HomeLayout {
  readonly grid: HomeGrid;
  readonly mode: LayeredMode;
}

/** 🔎️ The layout the matched media queries of {@link homeLayoutQueries} give: the tablet grid between the breakpoints, the desktop grid
 * on either side of them, swiping below the mobile breakpoint and wherever the grid's cards
 * would not fit their cells. */
export function homeLayout(matched: { readonly narrow: boolean; readonly medium: boolean; readonly short: { readonly [G in HomeGrid]: boolean } }): HomeLayout {
  const grid: HomeGrid = matched.medium ? "tablet" : "desktop";
  return { grid, mode: matched.narrow || matched.short[grid] ? "swipe" : "strip" };
}



/** 📍️ The cell of every page, row by row in three columns on desktop or two on tablets. */
export function homeCells(pages: readonly string[], grid: HomeGrid): Readonly<Record<string, LayeredCell>> {
  const columns = grid === "desktop" ? 3 : 2;
  const cells: Record<string, LayeredCell> = {};
  let column = 0;
  let row = 0;
  for (const page of pages) {
    cells[page] = { column, row };
    column += 1;
    if (column === columns) {
      column = 0;
      row += 1;
    }
  }
  return cells;
}

/** 📏️ Equal track weights shared by each level of the overview. */
export const HOME_GRID_TRACKS = { columns: [1, 1, 1], rows: [1, 1] } as const;

/** 🔳️ The CSS track list of `count` tracks of the card grid (`minmax(0, 1fr) …`); a track without a
 * weight weighs 1. */
export function homeTrackTemplate(weights: readonly number[] | undefined, count: number): string {
  return Array.from({ length: count }, (_, index) => `minmax(0, ${weights?.[index] ?? 1}fr)`).join(" ");
}

/** 🧱️ The height (px at the normal text size) a row of the grid needs so that the tallest card — in either language, in
 * the narrowest column of its layout — fits its cell. */
export const HOME_GRID_ROW_HEIGHT_PX = 160;

/** 🎩️ The height (px at the normal text size) of what stands above the overview: the navigation bar. */
export const HOME_CHROME_HEIGHT_PX = 56;

/** 🪜️ The least viewport height (px at the normal text size) at which every card of `pages` fits its cell in `grid`:
 * the navigation bar and a grid whose lowest row is {@link HOME_GRID_ROW_HEIGHT_PX} high. Below it the learner swipes
 * through the grid, as on a phone — a phone held sideways, a small window, a strongly zoomed page, large text. */
export function homeGridMinHeight(grid: HomeGrid, pages: readonly string[]): number {
  const rows = Math.max(0, ...Object.values(homeCells(pages, grid)).map((cell) => cell.row + 1));
  return HOME_CHROME_HEIGHT_PX + HOME_GRID_ROW_HEIGHT_PX * rows;
}

/** 📐️ The media queries that choose the layout of `pages` at a text `scale`: the design system's breakpoints and the
 * least height of either grid, each multiplied by the scale — text 1.5 times as large leaves a viewport two thirds as
 * wide and as high. */
export function homeLayoutQueries(scale: number, pages: readonly string[]): { readonly narrow: string; readonly medium: string; readonly short: { readonly [G in HomeGrid]: string } } {
  const narrow = Math.floor(UI_MOBILE_MAX_WIDTH_PX * scale);
  const short = (grid: HomeGrid): string => `(max-height: ${Math.floor(homeGridMinHeight(grid, pages) * scale) - 1}px)`;
  return {
    narrow: `(max-width: ${narrow}px)`,
    medium: `(min-width: ${narrow + 1}px) and (max-width: ${Math.floor(UI_TABLET_MAX_WIDTH_PX * scale)}px)`,
    short: { desktop: short("desktop"), tablet: short("tablet") },
  };
}

interface HomeProps {
  readonly introductionPage?: string;
  readonly session: CourseSession;
  readonly state: CourseState;
  readonly text: CourseText;
  readonly locale: CourseLocale;
  readonly preferences: CoursePreferences;
  readonly onPreferences: (preferences: CoursePreferences) => void;
}

/** 🧭️ What the hint of a page one swipe away is called, by the direction it lies in. */
const NEIGHBOUR_TEXTS = { up: "course.home.neighbourUp", left: "course.home.neighbourLeft", right: "course.home.neighbourRight", down: "course.home.neighbourDown" } as const satisfies { readonly [D in LayeredDirection]: string };

interface PreviewPointerBus {
  current: PreviewCamera | undefined;
  publish(camera: PreviewCamera | undefined): void;
  subscribe(listener: (camera: PreviewCamera | undefined) => void): () => void;
}

const PreviewPointerContext = createContext<PreviewPointerBus | null>(null);

/** 🖱️ One pointer shared by every nested overview. The active grid publishes it; the grids behind follow it. */
function createPreviewPointerBus(): PreviewPointerBus {
  let current: PreviewCamera | undefined;
  const listeners = new Set<(camera: PreviewCamera | undefined) => void>();
  return {
    get current() {
      return current;
    },
    publish(camera) {
      if (camera?.x === current?.x && camera?.y === current?.y) return;
      current = camera;
      for (const listener of listeners) listener(camera);
    },
    subscribe(listener) {
      listeners.add(listener);
      return () => listeners.delete(listener);
    },
  };
}

/** 📺️ The pointer inherited from the active grid. The publisher, and an ancestor that already has a page open, ignore it. */
function useInheritedPreview(ignore: boolean): PreviewCamera | undefined {
  const bus = useContext(PreviewPointerContext);
  const [camera, setCamera] = useState<PreviewCamera | undefined>(undefined);
  useLayoutEffect(() => {
    if (bus === null || ignore) return;
    setCamera(bus.current);
    return bus.subscribe(setCamera);
  }, [bus, ignore]);
  return ignore ? undefined : camera;
}

const ICONS: { readonly [page: string]: IconName } ={ [HOME_PAGES.learner]: "user", [HOME_PAGES.quizzes]: "layout-grid", [HOME_PAGES.leaderboard]: "list-ordered", [HOME_PAGES.badges]: "award", [HOME_PAGES.preferences]: "settings" };



/** 🗃️ A container card, or a topic placeholder with its own full page. */
function CollectionCard(props: HomeProps & { readonly page: string; readonly revealed?: boolean; readonly onOpen?: () => void; readonly placeholder?: boolean }): ReactElement {
  const { page, state, locale, text, revealed, onOpen, placeholder } = props;
  const id = useId();
  const topic = state.course?.topics?.find((candidate) => candidate.id === page);
  const label = pageLabel(page, state, locale, text) ?? page;
  const count = topic?.quizzes.length ?? state.course?.quizzes.length ?? 0;
  return (
    <CourseCard
      id={id}
      card={placeholder ? `topic-page:${page}` : page === HOME_PAGES.quizzes ? "quizzes" : `topic:${page}`}
      anchor={PRESENCE_ANCHORS.home(page === HOME_PAGES.quizzes ? "quizzes" : `topic:${page}`)}
      topic={topic?.id}
      href={onOpen === undefined ? undefined : `#${page}`}
      onOpen={onOpen}
      revealed={revealed}
      icon={topic === undefined ? <CardIcon icon="layout-grid" /> : <Glyph emoji={topic.emoji} className="text-sm" />}
      title={label}
      footerRight={onOpen === undefined ? undefined : <CardAction primary onClick={onOpen}>{text("course.home.explore")}</CardAction>}
    >
      <p className="m-0 text-xs leading-normal text-foreground">
        {text(page === HOME_PAGES.quizzes ? "course.home.quizzesDescription" : count > 0 ? "course.home.topicQuizzes" : "course.home.comingSoon", { count })}
      </p>
    </CourseCard>
  );
}

interface HomeOverviewProps extends HomeProps {
  readonly container?: string;
  readonly enabled: boolean;
  readonly busy: boolean;
  readonly act: Act;
}

/** 🥪️ Each container uses the same layered overview; only the current grid handles input. */
function HomeOverview(props: HomeOverviewProps): ReactElement | null {
  const { session, state, text, locale, preferences, onPreferences, container, enabled, busy, act } = props;
  const course = state.course;
  const follow = usePresenceFollow();
  const pages = useMemo(() => course === undefined ? [] : pagesWithin(container, course), [course, container]);
  const queries = homeLayoutQueries(textScale(preferences.textSize), pages);
  const narrow = useMediaQuery(queries.narrow);
  const medium = useMediaQuery(queries.medium);
  const short = { desktop: useMediaQuery(queries.short.desktop), tablet: useMediaQuery(queries.short.tablet) };
  const selected = state.step.screen === "home" ? state.step.page : undefined;
  const activeContainer = course === undefined || isHomeGrid(selected, course) ? selected : selected === undefined ? undefined : parentPage(selected, course);
  const activeGrid = enabled && activeContainer === container;
  const interactive = activeGrid && follow.following === undefined;
  const measuredLayout = homeLayout({ narrow, medium, short });
  const layout = activeGrid && follow.following !== undefined ? follow.presentation?.layout ?? measuredLayout : measuredLayout;
  useEffect(() => {
    if (interactive) follow.onLayout?.(measuredLayout);
  }, [interactive, measuredLayout.grid, measuredLayout.mode, follow.onLayout]);
  const opened = enabled && course !== undefined ? openedChild(container, selected, course) : null;
  const root = useRef<HTMLDivElement>(null);
  const previous = useRef({ opened, interactive: container === undefined && interactive });
  useLayoutEffect(() => {
    const before = previous.current;
    previous.current = { opened, interactive };
    if (!interactive) return;
    if (opened === null && before.opened !== null) {
      root.current?.querySelector<HTMLElement>(`:scope > [data-layered-overview] > [data-layered-overlay] [data-layered-card="${before.opened}"] a[href]`)?.focus({ preventScroll: true });
    } else if (!before.interactive && opened === null) root.current?.querySelector<HTMLElement>(":scope > [data-layered-overview] > [data-layered-overlay]")?.focus({ preventScroll: true });
  }, [opened, interactive]);
  useEffect(() => {
    if (!interactive || opened !== null || container === undefined || course === undefined) return;
    const onKey = (event: KeyboardEvent) => {
      if (event.key !== "Escape" || event.defaultPrevented) return;
      event.preventDefault();
      const page = parentPage(container, course);
      session.open(page === undefined ? { screen: "home" } : { screen: "home", page });
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [interactive, opened, container, course, session]);
  const previewPointer = useContext(PreviewPointerContext);
  const followedCamera = activeGrid && follow.following !== undefined ? follow.presentation?.camera : undefined;
  const inheritedCamera = useInheritedPreview(activeGrid || opened !== null);
  const camera = previewCamera(activeGrid, followedCamera, inheritedCamera);
  useLayoutEffect(() => {
    if (previewPointer === null || !activeGrid || follow.following === undefined) return;
    previewPointer.publish(followedCamera);
  }, [previewPointer, activeGrid, follow.following, followedCamera]);
  if (course === undefined) return null;
  const common = { session, state, text, locale };
  const panes: readonly LayeredPane[] = pages.map((page) => {
    const label = pageLabel(page, state, locale, text) ?? page;
    return {
      id: page,
      label,
      shortLabel: pageLabel(page, state, locale, text, true) ?? label,
      icon: ICONS[page],
      render: (pane) => {
        if (isHomeGrid(page, course)) return <HomeOverview {...props} container={page} enabled={enabled && pane.opened} />;
        const view = { opened: enabled && pane.opened, revealed: pane.revealed };
        const quiz = course.quizzes.find((candidate) => candidate.id === page);
        if (quiz !== undefined) return <QuizPage {...common} quiz={quiz} busy={busy} act={act} view={view} others={preferences.others} challenge={challengeOf(preferences, quiz.id)} onChallenge={(challenge) => onPreferences(withChallenge(preferences, quiz.id, challenge))} />;
        if (course.topics?.some((topic) => topic.id === page)) return <PageFrame page={page}><CollectionCard {...props} page={page} placeholder /></PageFrame>;
        switch (page) {
          case HOME_PAGES.learner:
            return <LearnerPage {...common} busy={busy} act={act} view={view} />;
          case HOME_PAGES.leaderboard:
            return <LeaderboardPage {...common} view={view} />;
          case HOME_PAGES.badges:
            return <BadgesPage state={state} text={text} locale={locale} view={view} />;
          default:
            return <PreferencesPage course={course.id} preferences={preferences} locale={locale} text={text} onChange={onPreferences} view={view} />;
        }
      },
    };
  });
  const cells = homeCells(pages, layout.grid);
  const strip = stripGrid(Object.values(cells));
  const tracks = layout.grid === "desktop" ? HOME_GRID_TRACKS : undefined;
  const cardOf = (pane: LayeredPane, card: LayeredCardState): ReactNode => {
    const shown = { revealed: card.revealed, onOpen: card.open };
    const quiz = course.quizzes.find((candidate) => candidate.id === pane.id);
    if (quiz !== undefined) return <QuizCardView {...common} {...shown} quiz={quiz} busy={busy} act={act} challenge={challengeOf(preferences, quiz.id)} />;
    if (pane.id === HOME_PAGES.quizzes || course.topics?.some((topic) => topic.id === pane.id)) return <CollectionCard {...props} page={pane.id} {...shown} />;
    switch (pane.id) {
      case HOME_PAGES.learner:
        return <LearnerCard {...common} {...shown} />;
      case HOME_PAGES.leaderboard:
        return <LeaderboardCard {...common} {...shown} />;
      case HOME_PAGES.badges:
        return <BadgesCard state={state} text={text} locale={locale} {...shown} />;
      default:
        return <PreferencesCard text={text} {...shown} />;
    }
  };
  const renderCard = (pane: LayeredPane, card: LayeredCardState): ReactNode => {
    const cell = cells[pane.id];
    const quiz = course.quizzes.some((candidate) => candidate.id === pane.id);
    const collection = isHomeGrid(pane.id, course);
    const key = quiz ? `quiz:${pane.id}` : collection ? pane.id === HOME_PAGES.quizzes ? "quizzes" : `topic:${pane.id}` : pane.id === HOME_PAGES.preferences ? "preferences" : pane.id;
    const contained = new Set(collection ? pagesWithin(pane.id, course).flatMap((page) => isHomeGrid(page, course) ? [page, ...pagesWithin(page, course)] : [page]) : []);
    const activity = <CardPresence anchor={PRESENCE_ANCHORS.home(key)} text={text} accepts={(place, presentation) => quiz ? place.quiz === pane.id : collection ? (place.quiz !== undefined && contained.has(place.quiz)) || (place.screen === "home" && presentation?.home !== undefined && (presentation.home === pane.id || contained.has(presentation.home))) : place.screen === (pane.id === HOME_PAGES.leaderboard ? "leaderboard" : pane.id === HOME_PAGES.preferences ? "preferences" : pane.id)} />;
    const alias = container === undefined ? `course.home.${pane.id}` : undefined;
    return card.mode === "strip" && cell !== undefined ? (
      <div data-element-alias={alias} className="course-home-cell" style={{ gridColumn: cell.column + 1, gridRow: cell.row + 1 }}><div className="course-home-card">{cardOf(pane, card)}{activity}</div></div>
    ) : <div data-element-alias={alias} className="course-home-entry course-home-card">{cardOf(pane, card)}{activity}</div>;
  };
  return (
    <div ref={root} data-page={container} data-course-grid={container ?? "root"} data-grid-active={activeGrid ? "" : undefined} className="relative h-full min-h-0 w-full">
      <LayeredOverview
        panes={panes}
        cells={cells}
        renderCard={renderCard}
        overlayClassName="course-home-grid"
        overlayStyle={{ "--course-home-columns": homeTrackTemplate(tracks?.columns, strip.columns), "--course-home-rows": homeTrackTemplate(tracks?.rows, strip.rows) } as CSSProperties}
        mode={layout.mode}
        routing="none"
        interactive={interactive}
        openedId={opened}
        camera={camera}
        onCameraChange={interactive ? (point) => { previewPointer?.publish(point); follow.onCamera(point); } : undefined}
        swipeId={activeGrid && container === undefined && props.introductionPage !== undefined ? props.introductionPage : activeGrid && follow.following !== undefined ? follow.presentation?.swipe : undefined}
        onSwipeIdChange={interactive ? follow.onSwipe : undefined}
        revealedId={activeGrid && follow.following !== undefined ? follow.presentation?.reveal ?? null : undefined}
        revealedBy={activeGrid && follow.following !== undefined ? follow.presentation?.revealedBy : undefined}
        onRevealedIdChange={interactive ? (page, source) => follow.onRevealed(page ?? undefined, source ?? undefined) : undefined}
        onOpenedIdChange={(page) => session.open(page === null ? container === undefined ? { screen: "home" } : { screen: "home", page: container } : { screen: "home", page })}
        lifecycle={{ budget: pages.length, warmStartMs: 0, warmIntervalMs: 0 }}
        labels={{
          grid: container === undefined ? text("course.home.cards") : pageLabel(container, state, locale, text) ?? container,
          waiting: (pane) => text("course.home.pageWaiting", { page: pane.label }),
          failed: (pane) => text("course.home.pageFailed", { page: pane.label }),
          neighbour: (pane, direction) => text(NEIGHBOUR_TEXTS[direction], { page: pane.label }),
        }}
      />
    </div>
  );
}

/** 🏠️ The home screen, with a layered grid at every level of the course's topic tree. */
export function HomeScreen(props: HomeProps): ReactElement | null {
  const { session, state, text } = props;
  const previewPointer = useRef<PreviewPointerBus | null>(null);
  if (previewPointer.current === null) previewPointer.current = createPreviewPointerBus();
  const [pending, setPending] = useState<AbortController | undefined>(undefined);
  const [problem, setProblem] = useState<Problem | undefined>(undefined);
  const quizIds = state.course?.quizzes.map((quiz) => quiz.id).join(" ") ?? "";
  const refreshLeaderboard = useCallback(() => session.refreshLeaderboard(), [session, state.learner?.id]);
  usePolling(refreshLeaderboard, LEADERBOARD_POLL_MS);
  const { submissions } = state;
  useEffect(() => {
    for (const quiz of quizIds.split(" ").filter(Boolean)) void session.refreshCrowd(quiz);
  }, [quizIds, submissions, session]);
  useEffect(() => () => pending?.abort(), [pending]);
  if (state.course === undefined) return null;
  const act: Act = (run) => {
    if (state.learner === undefined) { session.open({ screen: "identity" }); return; }
    if (pending !== undefined) return;
    const controller = new AbortController();
    setProblem(undefined);
    setPending(controller);
    run(controller.signal)
      .then((failure) => failure !== undefined && setProblem(failureProblem(failure, text)))
      .catch((thrown: unknown) => setProblem(thrownProblem(thrown, controller.signal, text)))
      .finally(() => setPending((current) => (current === controller ? undefined : current)));
  };
  const selected = state.step.screen === "home" ? state.step.page : undefined;
  const heading = selected === undefined ? text("course.home.title") : pageLabel(selected, state, props.locale, text) ?? text("course.home.title");
  return (
    <div className="course-home flex min-h-0 flex-1 flex-col">
      <h1 tabIndex={-1} className="sr-only">{heading}</h1>
      {problem === undefined ? null : <ProblemNote problem={problem} text={text} className="m-double mb-0" />}
      {pending === undefined ? null : (
        <div role="status" className="flex flex-wrap items-center gap-double px-double pt-double text-sm">
          <progress aria-label={text("course.home.starting")} className="course-progress" />
          <span>{text("course.home.starting")}</span>
          <BodyButton onClick={() => pending.abort()}>{text("course.run.cancel")}</BodyButton>
        </div>
      )}
      <div className="relative min-h-0 flex-1">
        <PreviewPointerContext.Provider value={previewPointer.current}>
          <HomeOverview {...props} enabled busy={pending !== undefined} act={act} />
        </PreviewPointerContext.Provider>
      </div>
    </div>
  );
}
