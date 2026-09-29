/** 🏠️ Home: the layered overview of semio-tech play. Behind a glass layer lie the real pages of every card — the
 * learner's profile, each quiz's page, the introduction, the full leaderboard, the badges and the preferences — and the
 * cards float above it; hovering or focusing a card shows its page clear, and its heading, a click on it or the page's
 * hash (`#board`) opens the page full size. In reading order — which is the DOM, focus and visual order at every width
 * — the learner, the first quiz, how it works, the second quiz, the leaderboard, the third quiz, the badges, the fourth
 * quiz and the preferences; further quizzes follow. At rest the overview is a live grid of every page — each scaled
 * into its cell with the others inside it, the leaderboard polled while home shows, the crowd of every quiz asked for —
 * and each card sits compact in its page's cell, the card layer sharing the grid's tracks: three columns from 1024 px
 * (the leaderboard in the larger centre cell), two from 768 px (the leaderboard alone on its row) and a list of sections
 * below, the design system's own breakpoints. Pages are pure views over session state: showing, revealing or keeping
 * one live never runs a command.
 *
 * @see ../../🎨️.css — `.quiz-home-grid`
 * @see ../../../../../../../🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx — the layered overview
 * @see ../../../../../../../🔨️modules/🖱️ui/📱️device/🟦️.ts — `UI_MOBILE_MEDIA_QUERY`, `UI_TABLET_MEDIA_QUERY`
 */

import { useEffect, useState, type ReactElement, type ReactNode } from "react";
import { learnerTag } from "@semio-tech/quiz";
import { LayeredOverview, UI_MOBILE_MEDIA_QUERY, UI_TABLET_MEDIA_QUERY, useMediaQuery, type IconName, type LayeredCardState, type LayeredCell, type LayeredPane } from "@semio-tech/ui-react/chrome";
import { localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { failureMessage, learnerName, thrownMessage } from "../🪪️identity/🟦️.tsx";
import { LEADERBOARD_POLL_MS, LeaderboardCard, LeaderboardPage, usePolling } from "../🏆️leaderboard/🟦️.tsx";
import { BadgesCard, BadgesPage } from "../🏅️badges/🟦️.tsx";
import { IntroductionCard, IntroductionPage } from "../👋️introduction/🟦️.tsx";
import { PreferencesCard, PreferencesPage, type QuizPreferences } from "../🎛️preferences/🟦️.tsx";
import { LearnerCard, LearnerPage } from "../📇️profile/🟦️.tsx";
import { QuizCardView, QuizPage, type Act } from "../📖️quiz-page/🟦️.tsx";
import { BodyButton } from "../🪟️chrome/🟦️.tsx";
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

/** 📏️ The track weights of the desktop grid: the leaderboard's column and row the larger ones, so every page lies
 * exactly behind its card (the card layer takes the same tracks from the overview). */
export const HOME_GRID_TRACKS = { columns: [1, 1.5, 1], rows: [1, 1.4, 1] } as const;

interface HomeProps {
  readonly session: QuizSession;
  readonly state: QuizState;
  readonly text: QuizText;
  readonly locale: QuizLocale;
  readonly preferences: QuizPreferences;
  readonly onPreferences: (preferences: QuizPreferences) => void;
}

const ICONS: { readonly [page: string]: IconName } = { [HOME_PAGES.learner]: "user", [HOME_PAGES.introduction]: "info", [HOME_PAGES.leaderboard]: "list-ordered", [HOME_PAGES.badges]: "award", [HOME_PAGES.preferences]: "settings" };

/** 🏠️ The home screen: the layered overview, its page opened as `state.step.page`. */
export function HomeScreen(props: HomeProps): ReactElement | null {
  const { session, state, text, locale, preferences, onPreferences } = props;
  const [pending, setPending] = useState<AbortController | undefined>(undefined);
  const [error, setError] = useState<string | undefined>(undefined);
  const mobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const tablet = useMediaQuery(UI_TABLET_MEDIA_QUERY);
  usePolling(session.refreshLeaderboard, LEADERBOARD_POLL_MS);
  const quizIds = state.catalog?.quizzes.map((quiz) => quiz.id).join(" ") ?? "";
  useEffect(() => {
    for (const quiz of quizIds.split(" ").filter(Boolean)) void session.refreshCrowd(quiz);
  }, [quizIds, session]);
  useEffect(() => () => pending?.abort(), [pending]);
  const { catalog, learner } = state;
  if (catalog === undefined || learner === undefined) return null;
  const layout: HomeLayout = mobile ? "list" : tablet ? "tablet" : "desktop";
  const busy = pending !== undefined;
  const act: Act = (run) => {
    if (pending !== undefined) return;
    const controller = new AbortController();
    setError(undefined);
    setPending(controller);
    run(controller.signal)
      .then((failure) => failure !== undefined && setError(failureMessage(failure, text)))
      .catch((thrown: unknown) => setError(thrownMessage(thrown, controller.signal, text)))
      .finally(() => setPending((current) => (current === controller ? undefined : current)));
  };
  const pages = homePages(catalog.quizzes.map((quiz) => quiz.id));
  const label = (page: string): string => {
    const quiz = catalog.quizzes.find((candidate) => candidate.id === page);
    if (quiz !== undefined) return localized(quiz.title, locale);
    switch (page) {
      case HOME_PAGES.learner:
        return learnerName(learner.identity, learnerTag(learner.id), text);
      case HOME_PAGES.introduction:
        return text("quiz.home.howItWorks");
      case HOME_PAGES.leaderboard:
        return text("quiz.leaderboard.title");
      case HOME_PAGES.badges:
        return text("quiz.home.badges");
      default:
        return text("quiz.preferences.title");
    }
  };
  const common = { session, state, text, locale };
  const panes: readonly LayeredPane[] = pages.map((page) => ({
    id: page,
    label: label(page),
    icon: ICONS[page],
    render: (pane) => {
      const view = { opened: pane.opened, revealed: pane.revealed };
      const quiz = catalog.quizzes.find((candidate) => candidate.id === page);
      if (quiz !== undefined) return <QuizPage {...common} quiz={quiz} busy={busy} act={act} view={view} showAnswers={preferences.showAnswers} />;
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
  const renderCard = (pane: LayeredPane, card: LayeredCardState): ReactNode => {
    const cell = cells[pane.id];
    const body = cardOf(pane, card);
    return card.mode === "strip" && cell !== undefined ? (
      <div className="quiz-home-cell" style={{ gridColumn: cell.column + 1, gridRow: cell.row + 1 }}>
        {body}
      </div>
    ) : (
      body
    );
  };
  const cardOf = (pane: LayeredPane, card: LayeredCardState): ReactNode => {
    const shown = { revealed: card.revealed, onOpen: card.open };
    const quiz = catalog.quizzes.find((candidate) => candidate.id === pane.id);
    if (quiz !== undefined) return <QuizCardView {...common} {...shown} quiz={quiz} busy={busy} act={act} />;
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
      {error === undefined ? null : (
        <p role="alert" className="quiz-alert m-double mb-0 border border-normal px-double py-single text-sm font-semibold">
          {error}
        </p>
      )}
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
          rest="grid"
          gridTracks={layout === "desktop" ? HOME_GRID_TRACKS : undefined}
          renderCard={renderCard}
          overlayClassName="quiz-home-grid"
          mode={layout === "list" ? "list" : "strip"}
          routing="hash"
          openedId={opened}
          onOpenedIdChange={(page) => session.open(page === null ? { screen: "home" } : { screen: "home", page })}
          lifecycle={{ budget: pages.length, suspendIdleMs: Number.POSITIVE_INFINITY, suspendOffscreenMs: Number.POSITIVE_INFINITY, suspendHiddenMs: Number.POSITIVE_INFINITY }}
          labels={{
            grid: text("quiz.home.cards"),
            overview: text("quiz.nav.home"),
            waiting: (pane) => text("quiz.home.pageWaiting", { page: pane.label }),
            failed: (pane) => text("quiz.home.pageFailed", { page: pane.label }),
          }}
        />
      </div>
    </div>
  );
}
