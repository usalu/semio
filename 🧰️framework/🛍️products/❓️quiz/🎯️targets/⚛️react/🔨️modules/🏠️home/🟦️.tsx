/** 🏠️ Home: the layered overview of semio-tech play and the mit-bestand demonstrator, two layers. In front, fixed cards
 * on a grid; behind one glass, the real pages of those cards — the learner's profile, each quiz's page, the
 * introduction, the full leaderboard, the badges and the preferences — each the size of the screen, side by side on one
 * strip. While the mouse is between the cards the strip pans under the glass with it; hovering or focusing a card glides
 * the strip to its page and shows it clear, and its heading, a click on it or the page's
 * hash (`#board`) opens the page full size — the step of the session, which the client's address follows — until Escape
 * or the navbar's way back to the overview closes it. In reading order — which is the DOM, focus and visual order at every width
 * — the learner, the first quiz, how it works, the second quiz, the leaderboard, the third quiz, the badges, the fourth
 * quiz and the preferences; further quizzes follow. Every page is live behind the glass — the others inside it, the
 * leaderboard the learner looks at polled while home shows, the crowd of every quiz asked for again whenever its answer
 * says that a run was submitted. Pan and glide run whatever the device says about motion, as on play.
 * Each card sits compact in its cell of the card grid, whose cells are those of the strip: three columns from 1024 px
 * (the leaderboard in the larger centre cell), two from 768 px (the leaderboard alone on its row) and a list of sections
 * below, the design system's own breakpoints — measured in the learner's text size, so larger text gets the layout of
 * the narrower viewport it leaves, and a viewport too short for every card to fit its cell (three rows of cells on a
 * desktop, five on a tablet) gets the list as well. Should a card still be taller than its cell, it scrolls inside the
 * cell and never lies over the card below. In the list every card is as wide as the screen leaves it, up to a width
 * that reads (`.quiz-home-entry`). Pages are pure views over session state: showing, revealing or keeping one
 * live never runs a command.
 *
 * @see ../../🎨️.css — `.quiz-home-grid`
 * @see ../🚏️navigation/🟦️.tsx — the navbar's ways and the address that names the opened page
 * @see ../../../../../../../🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx — the layered overview
 * @see ../../../../../../../🔨️modules/🖱️ui/📱️device/🟦️.ts — `UI_MOBILE_MEDIA_QUERY`, `UI_TABLET_MEDIA_QUERY`
 */

import { useEffect, useMemo, useState, type CSSProperties, type ReactElement, type ReactNode } from "react";
import { learnerTag } from "@semio-tech/quiz";
import { LayeredOverview, UI_MOBILE_MAX_WIDTH_PX, UI_TABLET_MAX_WIDTH_PX, stripGrid, useMediaQuery, type IconName, type LayeredCardState, type LayeredCell, type LayeredPane } from "@semio-tech/ui-react/chrome";
import { localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { failureProblem, learnerName, thrownProblem } from "../🪪️identity/🟦️.tsx";
import { LEADERBOARD_POLL_MS, LeaderboardCard, LeaderboardPage, usePolling } from "../🏆️leaderboard/🟦️.tsx";
import { BadgesCard, BadgesPage } from "../🏅️badges/🟦️.tsx";
import { IntroductionCard, IntroductionPage } from "../👋️introduction/🟦️.tsx";
import { PreferencesCard, PreferencesPage, challengeOf, textScale, withChallenge, type QuizPreferences } from "../🎛️preferences/🟦️.tsx";
import { LearnerCard, LearnerPage } from "../📇️profile/🟦️.tsx";
import { QuizCardView, QuizPage, type Act } from "../📖️quiz-page/🟦️.tsx";
import { BodyButton, ProblemNote, type Problem } from "../🪟️chrome/🟦️.tsx";
import { HOME_PAGES, type QuizSession, type QuizState } from "../🧭️session/🟦️.ts";

/** 🗺️ How the overview is laid out: three columns, two columns (the leaderboard on a row of its own) or a list. */
export type HomeLayout = "desktop" | "tablet" | "list";

/** 🧩️ The pages of home in reading order for a catalog with `quizzes`: the ring of nine around the leaderboard — the
 * learner, quiz 1, the introduction, quiz 2, the leaderboard, quiz 3, the badges, quiz 4, the preferences — where
 * missing quizzes leave no gap, then every quiz beyond the fourth. */
export function homePages(quizzes: readonly string[]): readonly string[] {
  const quiz = (index: number): readonly string[] => (index < quizzes.length ? [quizzes[index]!] : []);
  return [HOME_PAGES.learner, ...quiz(0), HOME_PAGES.introduction, ...quiz(1), HOME_PAGES.leaderboard, ...quiz(2), HOME_PAGES.badges, ...quiz(3), HOME_PAGES.preferences, ...quizzes.slice(4)];
}

/** 📍️ The cell of every page on the strip for a layout, matching where its card sits on the overview: row by row in
 * three columns on desktop; in two columns on tablets, where the leaderboard takes a row of its own. */
export function homeCells(pages: readonly string[], layout: Exclude<HomeLayout, "list">): Readonly<Record<string, LayeredCell>> {
  const columns = layout === "desktop" ? 3 : 2;
  const cells: Record<string, LayeredCell> = {};
  let column = 0;
  let row = 0;
  for (const page of pages) {
    const alone = layout === "tablet" && page === HOME_PAGES.leaderboard;
    if (alone && column !== 0) {
      column = 0;
      row += 1;
    }
    cells[page] = { column, row };
    column += 1;
    if (alone || column === columns) {
      column = 0;
      row += 1;
    }
  }
  return cells;
}

/** 📏️ The track weights of the desktop card grid: the leaderboard's column and row are the larger ones. */
export const HOME_GRID_TRACKS = { columns: [1, 1.5, 1], rows: [1, 1.4, 1] } as const;

/** 🔳️ The CSS track list of `count` tracks of the card grid (`minmax(0, 1fr) minmax(0, 1.5fr) …`); a track without a
 * weight weighs 1. */
export function homeTrackTemplate(weights: readonly number[] | undefined, count: number): string {
  return Array.from({ length: count }, (_, index) => `minmax(0, ${weights?.[index] ?? 1}fr)`).join(" ");
}

/** 🧱️ The height (px at the normal text size) a row of the grid needs so that the tallest card — in either language, in
 * the narrowest column of its layout — fits its cell. */
export const HOME_GRID_ROW_HEIGHT_PX = 160;

/** 🎩️ The height (px at the normal text size) of what stands above the overview: the navigation bar. */
export const HOME_CHROME_HEIGHT_PX = 56;

/** 🪜️ The least viewport height (px at the normal text size) at which every card of `pages` fits its cell in `layout`:
 * the navigation bar and a grid whose lowest row is {@link HOME_GRID_ROW_HEIGHT_PX} high. Below it the overview is a
 * list, as on a phone — a phone held sideways, a small window, a strongly zoomed page, large text. */
export function homeGridMinHeight(layout: Exclude<HomeLayout, "list">, pages: readonly string[]): number {
  const rows = Math.max(0, ...Object.values(homeCells(pages, layout)).map((cell) => cell.row + 1));
  const weights = Array.from({ length: rows }, (_, row) => (layout === "desktop" ? (HOME_GRID_TRACKS.rows[row] ?? 1) : 1));
  return Math.ceil(HOME_CHROME_HEIGHT_PX + (HOME_GRID_ROW_HEIGHT_PX * weights.reduce((sum, weight) => sum + weight, 0)) / Math.min(...weights));
}

/** 📐️ The media queries that choose the layout of `pages` at a text `scale`: the design system's breakpoints and the
 * least height of either grid, each multiplied by the scale — text 1.5 times as large leaves a viewport two thirds as
 * wide and as high. */
export function homeLayoutQueries(scale: number, pages: readonly string[]): { readonly narrow: string; readonly medium: string; readonly short: { readonly [L in Exclude<HomeLayout, "list">]: string } } {
  const narrow = Math.floor(UI_MOBILE_MAX_WIDTH_PX * scale);
  const short = (layout: Exclude<HomeLayout, "list">): string => `(max-height: ${Math.floor(homeGridMinHeight(layout, pages) * scale) - 1}px)`;
  return {
    narrow: `(max-width: ${narrow}px)`,
    medium: `(min-width: ${narrow + 1}px) and (max-width: ${Math.floor(UI_TABLET_MAX_WIDTH_PX * scale)}px)`,
    short: { desktop: short("desktop"), tablet: short("tablet") },
  };
}

interface HomeProps {
  readonly session: QuizSession;
  readonly state: QuizState;
  readonly text: QuizText;
  readonly locale: QuizLocale;
  readonly preferences: QuizPreferences;
  readonly onPreferences: (preferences: QuizPreferences) => void;
}

const ICONS: { readonly [page: string]: IconName } = { [HOME_PAGES.learner]: "user", [HOME_PAGES.introduction]: "info", [HOME_PAGES.leaderboard]: "list-ordered", [HOME_PAGES.badges]: "award", [HOME_PAGES.preferences]: "settings" };

/** 🏷️ What a page of home is called: a quiz by its title, the learner's page by the learner's name, the others by what
 * they hold; nothing for a page home does not have. */
export function pageLabel(page: string, state: Pick<QuizState, "catalog" | "learner">, locale: QuizLocale, text: QuizText): string | undefined {
  const quiz = state.catalog?.quizzes.find((candidate) => candidate.id === page);
  if (quiz !== undefined) return localized(quiz.title, locale);
  switch (page) {
    case HOME_PAGES.learner:
      return state.learner === undefined ? undefined : learnerName(state.learner.identity, learnerTag(state.learner.id), text);
    case HOME_PAGES.introduction:
      return text("quiz.home.howItWorks");
    case HOME_PAGES.leaderboard:
      return text("quiz.leaderboard.title");
    case HOME_PAGES.badges:
      return text("quiz.home.badges");
    case HOME_PAGES.preferences:
      return text("quiz.preferences.title");
    default:
      return undefined;
  }
}

/** 🏠️ The home screen: the layered overview, its page opened as `state.step.page`. */
export function HomeScreen(props: HomeProps): ReactElement | null {
  const { session, state, text, locale, preferences, onPreferences } = props;
  const [pending, setPending] = useState<AbortController | undefined>(undefined);
  const [problem, setProblem] = useState<Problem | undefined>(undefined);
  const quizIds = state.catalog?.quizzes.map((quiz) => quiz.id).join(" ") ?? "";
  const pages = useMemo(() => homePages(quizIds.split(" ").filter(Boolean)), [quizIds]);
  const queries = homeLayoutQueries(textScale(preferences.textSize), pages);
  const narrow = useMediaQuery(queries.narrow);
  const medium = useMediaQuery(queries.medium);
  const short = { desktop: useMediaQuery(queries.short.desktop), tablet: useMediaQuery(queries.short.tablet) };
  usePolling(session.refreshLeaderboard, LEADERBOARD_POLL_MS);
  const { submissions } = state;
  useEffect(() => {
    for (const quiz of quizIds.split(" ").filter(Boolean)) void session.refreshCrowd(quiz);
  }, [quizIds, submissions, session]);
  useEffect(() => () => pending?.abort(), [pending]);
  const { catalog, learner } = state;
  if (catalog === undefined || learner === undefined) return null;
  const wide = medium ? "tablet" : "desktop";
  const layout: HomeLayout = narrow || short[wide] ? "list" : wide;
  const busy = pending !== undefined;
  const act: Act = (run) => {
    if (pending !== undefined) return;
    const controller = new AbortController();
    setProblem(undefined);
    setPending(controller);
    run(controller.signal)
      .then((failure) => failure !== undefined && setProblem(failureProblem(failure, text)))
      .catch((thrown: unknown) => setProblem(thrownProblem(thrown, controller.signal, text)))
      .finally(() => setPending((current) => (current === controller ? undefined : current)));
  };
  const common = { session, state, text, locale };
  const panes: readonly LayeredPane[] = pages.map((page) => ({
    id: page,
    label: pageLabel(page, state, locale, text) ?? page,
    icon: ICONS[page],
    render: (pane) => {
      const view = { opened: pane.opened, revealed: pane.revealed };
      const quiz = catalog.quizzes.find((candidate) => candidate.id === page);
      if (quiz !== undefined) return <QuizPage {...common} quiz={quiz} busy={busy} act={act} view={view} others={preferences.others} challenge={challengeOf(preferences, quiz.id)} onChallenge={(challenge) => onPreferences(withChallenge(preferences, quiz.id, challenge))} />;
      switch (page) {
        case HOME_PAGES.learner:
          return <LearnerPage {...common} busy={busy} act={act} view={view} />;
        case HOME_PAGES.introduction:
          return <IntroductionPage catalog={catalog} locale={locale} view={view} />;
        case HOME_PAGES.leaderboard:
          return <LeaderboardPage {...common} view={view} />;
        case HOME_PAGES.badges:
          return <BadgesPage state={state} text={text} locale={locale} view={view} />;
        default:
          return <PreferencesPage preferences={preferences} locale={locale} text={text} onChange={onPreferences} view={view} />;
      }
    },
  }));
  const cells = homeCells(pages, layout === "tablet" ? "tablet" : "desktop");
  const strip = stripGrid(Object.values(cells));
  const tracks: { readonly columns?: readonly number[]; readonly rows?: readonly number[] } = layout === "desktop" ? HOME_GRID_TRACKS : {};
  const renderCard = (pane: LayeredPane, card: LayeredCardState): ReactNode => {
    const cell = cells[pane.id];
    const body = cardOf(pane, card);
    return card.mode === "strip" && cell !== undefined ? (
      <div className="quiz-home-cell" style={{ gridColumn: cell.column + 1, gridRow: cell.row + 1 }}>
        {body}
      </div>
    ) : (
      <div className="quiz-home-entry">{body}</div>
    );
  };
  const cardOf = (pane: LayeredPane, card: LayeredCardState): ReactNode => {
    const shown = { revealed: card.revealed, onOpen: card.open };
    const quiz = catalog.quizzes.find((candidate) => candidate.id === pane.id);
    if (quiz !== undefined) return <QuizCardView {...common} {...shown} quiz={quiz} busy={busy} act={act} challenge={challengeOf(preferences, quiz.id)} />;
    switch (pane.id) {
      case HOME_PAGES.learner:
        return <LearnerCard {...common} {...shown} />;
      case HOME_PAGES.introduction:
        return <IntroductionCard catalog={catalog} text={text} locale={locale} {...shown} />;
      case HOME_PAGES.leaderboard:
        return <LeaderboardCard {...common} {...shown} />;
      case HOME_PAGES.badges:
        return <BadgesCard state={state} text={text} locale={locale} {...shown} />;
      default:
        return <PreferencesCard text={text} {...shown} />;
    }
  };
  const opened = state.step.screen === "home" && state.step.page !== undefined && pages.includes(state.step.page) ? state.step.page : null;

  return (
    <div className="quiz-home flex min-h-0 flex-1 flex-col">
      <h1 tabIndex={-1} className="sr-only">
        {text("quiz.home.title")}
      </h1>
      {problem === undefined ? null : <ProblemNote problem={problem} text={text} className="m-double mb-0" />}
      {pending === undefined ? null : (
        <div role="status" className="flex flex-wrap items-center gap-double px-double pt-double text-sm">
          <progress aria-label={text("quiz.home.starting")} className="quiz-progress" />
          <span>{text("quiz.home.starting")}</span>
          <BodyButton onClick={() => pending.abort()}>{text("quiz.run.cancel")}</BodyButton>
        </div>
      )}
      <div className="relative min-h-0 flex-1">
        <LayeredOverview
          panes={panes}
          cells={cells}
          renderCard={renderCard}
          overlayClassName="quiz-home-grid"
          overlayStyle={{ "--quiz-home-columns": homeTrackTemplate(tracks.columns, strip.columns), "--quiz-home-rows": homeTrackTemplate(tracks.rows, strip.rows) } as CSSProperties}
          mode={layout === "list" ? "list" : "strip"}
          routing="none"
          openedId={opened}
          onOpenedIdChange={(page) => session.open(page === null ? { screen: "home" } : { screen: "home", page })}
          lifecycle={{ budget: pages.length, warmStartMs: 0, warmIntervalMs: 0 }}
          labels={{
            grid: text("quiz.home.cards"),
            waiting: (pane) => text("quiz.home.pageWaiting", { page: pane.label }),
            failed: (pane) => text("quiz.home.pageFailed", { page: pane.label }),
          }}
        />
      </div>
    </div>
  );
}
