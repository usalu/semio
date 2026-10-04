/** 🏆️ The leaderboards: four of them — today's, this week's, this month's and the all-time one — each of every quiz or
 * of one category (a quiz of the catalog), one shown at a time. The learner chooses the period and the category with
 * two segmented choices, and the choice holds for the page and the card alike. The page shows the top learners with a
 * submitted run on the chosen leaderboard in a table that sorts by any column on a click on its heading — rank,
 * learner, total, best run per quiz as points with its challenge, badges, runs and last submission —, what the leaderboard counts (the window of
 * its period in the learner's own time zone), how many learners are ranked in all, and the current learner's own row
 * with its real rank, highlighted inside the top and set apart below it otherwise; the card at the centre of the home
 * grid shows its excerpt (the first rows and the own row), sortable the same way. Rows never carry learner ids (an
 * anonymous id is a credential), only their public tags; the own row is the one the proctor returns for the asking
 * learner. The leaderboards are ephemeral shared state: the one looked at is polled while the page is visible — at a
 * jittered interval, backing off while the proctor is busy or away — and never announced to screen readers. They are
 * for fun: names are not verified, and the page says so.
 *
 * @see ../../../../🧬️schema/🔣️.json — `Leaderboard { period, quiz?, window?, rows, learners, submissions, own? }`
 */

import { useEffect, useId, useState, type ReactElement } from "react";
import { Icon, overviewCardChipClass } from "@semio-tech/ui-react/chrome";
import { LEADERBOARD_PERIODS, learnerTag, roomScope, type CatalogView, type Leaderboard, type LeaderboardPeriod, type LeaderboardRow, type Slug } from "@semio-tech/quiz";
import { localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { formatClock, formatInstant, formatPoints } from "../📏️quantity/🟦️.ts";
import { CHALLENGE_LABELS } from "../⛰️challenge/🟦️.tsx";
import { learnerName } from "../🪪️identity/🟦️.tsx";
import { CARD_CHIP_WRAP, CardAction, CardIcon, Missing, PageFrame, QuizCard, Records, Segments, TABLE, cn, textPresentation, type PaneView, type Segment } from "../🪟️chrome/🟦️.tsx";
import { HOME_PAGES, shownLeaderboard, type HeldLeaderboard, type QuizSession, type QuizState, type RefreshOutcome } from "../🧭️session/🟦️.ts";
import { OnlineMark, PRESENCE_ANCHORS, PanePeers, usePresenceView } from "../👥️presence/🟦️.tsx";

/** ⏱️ How often the leaderboard is asked for while visible and the proctor answers. */
export const LEADERBOARD_POLL_MS = 10_000;

/** ⏱️ The longest pause between two polls while the proctor does not answer. */
export const POLL_BACKOFF_MAX_MS = 300_000;

/** 🔢️ How many rows the leaderboard card shows before the own row. */
export const BOARD_EXCERPT_SIZE = 5;

/** 🔑️ A sortable column: a fixed one or the best run of one quiz, by its points. */
export type LeaderboardKey = "rank" | "learner" | "total" | "badges" | "runs" | "last-activity" | `best:${string}`;

/** ↕️ The order of the table. */
export interface LeaderboardSort {
  readonly key: LeaderboardKey;
  readonly direction: "ascending" | "descending";
}

/** ↕️ The order every table starts in: by rank, the best first. */
export const RANK_ORDER: LeaderboardSort = { key: "rank", direction: "ascending" };

/** ↕️ The order once the heading of column `key` is chosen: the other way round when the table is in that column's
 * order already; else rank and learner ascending and every other column — where more is better — descending. */
export function nextSort(present: LeaderboardSort, key: LeaderboardKey): LeaderboardSort {
  if (present.key === key) return { key, direction: present.direction === "ascending" ? "descending" : "ascending" };
  return { key, direction: key === "rank" || key === "learner" ? "ascending" : "descending" };
}

function sortValue(row: LeaderboardRow, key: LeaderboardKey, name: (row: LeaderboardRow) => string): number | string {
  switch (key) {
    case "rank":
      return row.rank;
    case "learner":
      return name(row);
    case "total":
      return row.total;
    case "badges":
      return row.badges.length;
    case "runs":
      return row.runs;
    case "last-activity":
      return row.lastActivity;
    default:
      return row.best[key.slice("best:".length)]?.points ?? -1;
  }
}

/** ↕️ `rows` in the order of `sort`, ties by rank; learner names compare in `locale`. */
export function sortLeaderboard(rows: readonly LeaderboardRow[], sort: LeaderboardSort, name: (row: LeaderboardRow) => string, locale: QuizLocale): readonly LeaderboardRow[] {
  const collator = new Intl.Collator(locale);
  return [...rows].sort((left, right) => {
    const a = sortValue(left, sort.key, name);
    const b = sortValue(right, sort.key, name);
    const order = typeof a === "number" && typeof b === "number" ? a - b : collator.compare(String(a), String(b));
    return (sort.direction === "ascending" ? order : -order) || left.rank - right.rank;
  });
}

/** 🙋️ The learner's own row of a leaderboard: the one the proctor returned for the asker, else the row of the top that
 * carries the tag `mine`. */
export function ownRow(board: Pick<Leaderboard, "rows" | "own">, mine: string | undefined): LeaderboardRow | undefined {
  return board.own ?? (mine === undefined ? undefined : board.rows.find((row) => row.tag === mine));
}

/** ✂️ The rows the leaderboard card shows: the first `size` by rank, and the own row apart when it ranks lower —
 * wherever it ranks, also far below the rows the proctor sends. */
export function boardExcerpt(board: Pick<Leaderboard, "rows" | "own">, mine: string | undefined, size = BOARD_EXCERPT_SIZE): { readonly top: readonly LeaderboardRow[]; readonly own: LeaderboardRow | undefined } {
  const top = [...board.rows].sort((left, right) => left.rank - right.rank).slice(0, size);
  const own = ownRow(board, mine);
  return { top, own: own === undefined || top.some((row) => row.tag === own.tag) ? undefined : own };
}

/** ⏳️ How long to wait before the next poll: the interval ± 10 % while the proctor answers (`random` in [0, 1] spreads
 * the clients of one room); after `failures` unanswered polls in a row at least the interval and at least what the
 * proctor asked for, plus a random share of an exponentially growing ceiling capped at {@link POLL_BACKOFF_MAX_MS}. */
export function pollDelay(intervalMs: number, failures: number, retryAfterMs: number | undefined, random: number): number {
  if (failures <= 0) return intervalMs + ((2 * random - 1) * intervalMs) / 10;
  const ceiling = Math.min(POLL_BACKOFF_MAX_MS, intervalMs * 2 ** failures);
  return Math.max(retryAfterMs ?? 0, intervalMs) + random * (ceiling - intervalMs);
}

/** 🔁️ While `enabled`: calls `refresh` now, again {@link pollDelay} after each answer while the document is visible,
 * and at once when it becomes visible again — unless it is backing off, which a look at the tab never cuts short. */
export function usePolling(refresh: () => Promise<RefreshOutcome>, intervalMs: number, enabled = true): void {
  useEffect(() => {
    if (!enabled) return;
    let stopped = false;
    let asking = false;
    let failures = 0;
    let timer: ReturnType<typeof setTimeout> | undefined;
    const hidden = (): boolean => typeof document !== "undefined" && document.visibilityState === "hidden";
    const settle = (outcome: RefreshOutcome): void => {
      asking = false;
      if (stopped) return;
      failures = outcome.answered ? 0 : failures + 1;
      timer = setTimeout(ask, pollDelay(intervalMs, failures, outcome.answered ? undefined : outcome.retryAfterMs, Math.random()));
    };
    const ask = (): void => {
      timer = undefined;
      if (stopped || asking || hidden()) return;
      asking = true;
      refresh().then(settle, () => settle({ answered: false }));
    };
    const seen = (): void => {
      if (hidden() || asking || (timer !== undefined && failures > 0)) return;
      clearTimeout(timer);
      ask();
    };
    ask();
    document.addEventListener("visibilitychange", seen);
    return () => {
      stopped = true;
      clearTimeout(timer);
      document.removeEventListener("visibilitychange", seen);
    };
  }, [refresh, intervalMs, enabled]);
}

/** 🏷️ One column of a leaderboard table. */
export interface LeaderboardColumn {
  readonly key: LeaderboardKey;
  readonly label: string;
}

/** 🏛️ The columns of the leaderboard page: rank, learner, total, the best run of every quiz (its points with its challenge), badges, runs and last
 * submission — or, on the leaderboard of the one quiz `only`, its points in place of the total and the bests, which
 * would say the same. */
export function leaderboardColumns(catalog: CatalogView | undefined, text: QuizText, locale: QuizLocale, only?: Slug): readonly LeaderboardColumn[] {
  return [
    { key: "rank", label: text("quiz.leaderboard.rank") },
    { key: "learner", label: text("quiz.leaderboard.learner") },
    ...(only === undefined ? [{ key: "total" as const, label: text("quiz.leaderboard.total") }, ...(catalog?.quizzes ?? []).map((quiz) => ({ key: `best:${quiz.id}` as const, label: localized(quiz.title, locale) }))] : [{ key: "total" as const, label: text("quiz.leaderboard.points") }]),
    { key: "badges", label: text("quiz.leaderboard.badges") },
    { key: "runs", label: text("quiz.leaderboard.runs") },
    { key: "last-activity", label: text("quiz.leaderboard.lastActivity") },
  ];
}

const PERIOD_TEXT = { daily: "quiz.leaderboard.daily", weekly: "quiz.leaderboard.weekly", monthly: "quiz.leaderboard.monthly", "all-time": "quiz.leaderboard.allTime" } as const satisfies Record<LeaderboardPeriod, string>;

/** 🗂️ The choice of every quiz among the categories; no quiz id can be it. */
const EVERY_QUIZ = "*";

const CELL = "px-single py-single align-top";
const NUMBER = "quiz-nowrap tabular-nums";
const EDGE = "border-b border-normal px-single py-single text-left align-top";

/** 🏷️ The name of a category: the emoji and title of its quiz. */
function categoryLabel(quiz: CatalogView["quizzes"][number], locale: QuizLocale): string {
  return `${textPresentation(quiz.emoji)} ${localized(quiz.title, locale)}`;
}

/** 🗂️ The choice of the leaderboard to look at: its period and — with `categories` — its category, every quiz or one. */
function BoardChoices(props: { readonly session: QuizSession; readonly state: QuizState; readonly text: QuizText; readonly locale: QuizLocale; readonly categories?: boolean }): ReactElement {
  const { session, state, text, locale } = props;
  const { board } = state;
  const periods: readonly Segment<LeaderboardPeriod>[] = LEADERBOARD_PERIODS.map((period) => ({ value: period, label: text(PERIOD_TEXT[period]) }));
  const quizzes: readonly Segment<string>[] = [{ value: EVERY_QUIZ, label: text("quiz.leaderboard.everyQuiz") }, ...(state.catalog?.quizzes ?? []).map((quiz) => ({ value: quiz.id, label: categoryLabel(quiz, locale) }))];
  return (
    <div data-board-choices="" className="flex flex-wrap items-start gap-double">
      <Segments label={text("quiz.leaderboard.period")} options={periods} value={board.period} onChange={(period) => session.chooseBoard({ ...board, period })} />
      {props.categories ? <Segments label={text("quiz.leaderboard.category")} options={quizzes} value={board.quiz ?? EVERY_QUIZ} onChange={(quiz) => session.chooseBoard(quiz === EVERY_QUIZ ? { period: board.period } : { period: board.period, quiz })} /> : null}
    </div>
  );
}

/** ↕️ The heading of a column: a button that puts the table in its order, and the other way round when chosen again. */
function SortHeading(props: { readonly column: LeaderboardColumn; readonly sort: LeaderboardSort; readonly onSort: (key: LeaderboardKey) => void; readonly text: QuizText; readonly className: string; readonly quiet?: boolean; readonly end?: boolean }): ReactElement {
  const { column, sort } = props;
  const chosen = sort.key === column.key;
  return (
    <th {...TABLE.column} aria-sort={chosen ? sort.direction : undefined} className={props.className}>
      <button
        type="button"
        className={cn("quiz-target inline-flex cursor-pointer items-center gap-single border-0 bg-transparent p-0 text-left", props.quiet ? "font-medium text-inherit" : "font-semibold text-foreground", props.end && "flex-row-reverse")}
        onClick={() => props.onSort(column.key)}
        title={props.text("quiz.leaderboard.sort", { column: column.label })}
      >
        {column.label}
        <Icon icon={!chosen ? "chevrons-up-down" : sort.direction === "ascending" ? "chevron-up" : "chevron-down"} size="small" className={cn("shrink-0", !chosen && "text-muted-foreground")} />
      </button>
    </th>
  );
}

/** 🕰️ Where the standings shown come from: the time the proctor's answer last changed here — or, for a leaderboard the
 * device made up by itself while the proctor was away, that it holds this device alone. */
function Updated(props: { readonly board: HeldLeaderboard; readonly text: QuizText; readonly locale: QuizLocale; readonly long?: boolean }): ReactElement {
  const { board, text } = props;
  const time = props.long ? formatInstant(board.at, props.locale) : formatClock(board.at, props.locale);
  return (
    <span data-board-local={board.local ? "" : undefined} className={cn(overviewCardChipClass, CARD_CHIP_WRAP, "text-muted-foreground")}>
      {board.local ? text("quiz.leaderboard.local") : text("quiz.leaderboard.updated", { time })}
    </span>
  );
}

function Gap(props: { readonly span: number }): ReactElement {
  return (
    <tr aria-hidden="true" className="border-t border-normal text-muted-foreground">
      <td colSpan={props.span} data-cell="note" className={cn(CELL, "text-center")}>
        ⋯
      </td>
    </tr>
  );
}

function BoardRow(props: { readonly row: LeaderboardRow; readonly mine: boolean; readonly name: string; readonly online: number | undefined; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { row, mine, name, online, text, locale } = props;
  return (
    <tr {...TABLE.row} className={cn("border-t border-normal", mine && "quiz-me")} aria-current={mine ? "true" : undefined}>
      <td {...TABLE.cell} data-cell="lead" className={cn(CELL, NUMBER)}>
        {row.rank}
      </td>
      <th {...TABLE.name} data-cell="name" className={cn(CELL, "quiz-name text-left", mine ? "font-semibold" : "font-normal")}>
        {name}
        {mine ? <span className="font-normal text-muted-foreground"> ({text("quiz.leaderboard.you")})</span> : null}
        {online === undefined ? null : <OnlineMark colour={online} text={text} />}
      </th>
      <td {...TABLE.cell} data-cell="lead" className={cn(CELL, NUMBER, "text-right")}>
        {formatPoints(row.total, locale)}
      </td>
      <td {...TABLE.cell} data-label={text("quiz.leaderboard.badges")} className={cn(CELL, NUMBER, "text-right")}>
        {row.badges.length}
      </td>
    </tr>
  );
}

/** 🏆️ The leaderboard card at the centre of the overview: the choice of the period, then the top rows of the chosen
 * leaderboard — in the order of the column heading chosen last — a gap, and the own row; the category chosen on the
 * page is named. Its heading and "Full leaderboard" open the leaderboard page (`#board`). */
export function LeaderboardCard(props: { readonly session: QuizSession; readonly state: QuizState; readonly text: QuizText; readonly locale: QuizLocale; readonly revealed: boolean; readonly onOpen: () => void }): ReactElement {
  const { session, state, text, locale, revealed, onOpen } = props;
  const id = useId();
  const [sort, setSort] = useState<LeaderboardSort>(RANK_ORDER);
  const board = shownLeaderboard(state);
  const mine = state.learner === undefined ? undefined : learnerTag(state.learner.id);
  const { top, own } = boardExcerpt(board?.board ?? { rows: [] }, mine);
  const name = (row: LeaderboardRow): string => learnerName(row.identity, row.tag, text);
  const head = cn(CELL, "quiz-nowrap");
  const category = state.catalog?.quizzes.find((quiz) => quiz.id === state.board.quiz);
  const shown: readonly (LeaderboardColumn & { readonly end?: boolean })[] = [
    { key: "rank", label: text("quiz.leaderboard.rank") },
    { key: "learner", label: text("quiz.leaderboard.learner") },
    { key: "total", label: text(category === undefined ? "quiz.leaderboard.total" : "quiz.leaderboard.points"), end: true },
    { key: "badges", label: text("quiz.leaderboard.badges"), end: true },
  ];
  const { colours } = usePresenceView();
  return (
    <QuizCard
      id={id}
      card="board"
      anchor={PRESENCE_ANCHORS.home("board")}
      href={`#${HOME_PAGES.leaderboard}`}
      onOpen={onOpen}
      revealed={revealed}
      icon={<CardIcon icon="list-ordered" />}
      title={text("quiz.leaderboard.title")}
      footerLeft={board === undefined ? undefined : <Updated board={board} text={text} locale={locale} />}
      footerRight={
        <CardAction primary onClick={onOpen}>
          {text("quiz.leaderboard.full")}
        </CardAction>
      }
    >
      <BoardChoices session={session} state={state} text={text} locale={locale} />
      {category === undefined ? null : (
        <p data-board-category="" className="m-0 text-xs text-muted-foreground">
          {text("quiz.leaderboard.category")}: {categoryLabel(category, locale)}
        </p>
      )}
      {board === undefined ? (
        <p className="m-0 text-xs text-muted-foreground">{text("quiz.app.fetching")}</p>
      ) : top.length === 0 ? (
        <p className="m-0 text-xs text-muted-foreground">{text(state.board.period === "all-time" && state.board.quiz === undefined ? "quiz.leaderboard.empty" : "quiz.leaderboard.emptyScope")}</p>
      ) : (
        <Records fold={19} className="min-h-0 flex-1">
          <table {...TABLE.table} data-head="sort" className="quiz-fold w-full border-collapse text-sm">
            <caption className="sr-only">{text("quiz.leaderboard.topCaption")}</caption>
            <thead {...TABLE.group}>
              <tr {...TABLE.row} className="text-left text-muted-foreground">
                {shown.map((column) => (
                  <SortHeading key={column.key} column={column} sort={sort} onSort={(key) => setSort((present) => nextSort(present, key))} text={text} className={cn(head, column.end && "text-right")} quiet end={column.end} />
                ))}
              </tr>
            </thead>
            <tbody {...TABLE.group}>
              {sortLeaderboard(top, sort, name, locale).map((row) => (
                <BoardRow key={row.tag} row={row} mine={row.tag === mine} name={name(row)} online={colours.get(row.tag)} text={text} locale={locale} />
              ))}
              {own === undefined ? null : (
                <>
                  <Gap span={4} />
                  <BoardRow row={own} mine name={name(own)} online={colours.get(own.tag)} text={text} locale={locale} />
                </>
              )}
            </tbody>
          </table>
        </Records>
      )}
    </QuizCard>
  );
}

function PageRow(props: { readonly row: LeaderboardRow; readonly mine: boolean; readonly state: QuizState; readonly bests: boolean; readonly online: number | undefined; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { row, mine, state, online, text, locale } = props;
  const badgeLabel = (id: string): string => {
    const badge = state.catalog?.badges.find((candidate) => candidate.id === id);
    return badge === undefined ? id : `${textPresentation(badge.emoji)} ${localized(badge.label, locale)}`;
  };
  return (
    <tr {...TABLE.row} className={mine ? "quiz-me" : undefined} aria-current={mine ? "true" : undefined}>
      <td {...TABLE.cell} data-cell="lead" className={cn(EDGE, NUMBER)}>
        {row.rank}
      </td>
      <th {...TABLE.name} data-cell="name" className={cn(EDGE, "quiz-name", mine ? "font-semibold" : "font-normal")}>
        {learnerName(row.identity, row.tag, text)}
        {mine ? <span className="font-normal text-muted-foreground"> ({text("quiz.leaderboard.you")})</span> : null}
        {online === undefined ? null : <OnlineMark colour={online} text={text} />}
      </th>
      <td {...TABLE.cell} data-cell="lead" className={cn(EDGE, NUMBER, "quiz-board-total")}>
        {formatPoints(row.total, locale)}
      </td>
      {(props.bests ? (state.catalog?.quizzes ?? []) : []).map((quiz) => {
        const best = row.best[quiz.id];
        return (
          <td key={quiz.id} {...TABLE.cell} data-label={localized(quiz.title, locale)} className={cn(EDGE, NUMBER)}>
            {best === undefined ? <Missing label={text("quiz.leaderboard.noBest")} /> : text("quiz.leaderboard.best", { points: formatPoints(best.points, locale), challenge: text(CHALLENGE_LABELS[best.challenge]) })}
          </td>
        );
      })}
      <td {...TABLE.cell} data-label={text("quiz.leaderboard.badges")} className={EDGE}>
        <span className="quiz-nowrap tabular-nums">{row.badges.length}</span>
        {row.badges.length === 0 ? null : <span className="text-muted-foreground"> {row.badges.map(badgeLabel).join(", ")}</span>}
      </td>
      <td {...TABLE.cell} data-label={text("quiz.leaderboard.runs")} className={cn(EDGE, NUMBER)}>
        {row.runs}
      </td>
      <td {...TABLE.cell} data-label={text("quiz.leaderboard.lastActivity")} className={cn(EDGE, "quiz-nowrap")}>
        {formatInstant(row.lastActivity, locale)}
      </td>
    </tr>
  );
}

/** 🏆️ The leaderboard page behind the centre card: the choice of the period and the category, then the chosen
 * leaderboard — what it counts, the number of ranked learners, the top learners in one table that sorts by the column
 * heading chosen last, the own row below it when it ranks lower — kept up to date by home's polling, with the others on
 * this page inside it. A column the chosen leaderboard does not show orders nothing: the table is by rank then. */
export function LeaderboardPage(props: { readonly session: QuizSession; readonly state: QuizState; readonly text: QuizText; readonly locale: QuizLocale; readonly view: PaneView }): ReactElement {
  const { session, state, text, locale, view } = props;
  const scope = useId();
  const [chosen, setSort] = useState<LeaderboardSort>(RANK_ORDER);
  const mine = state.learner === undefined ? undefined : learnerTag(state.learner.id);
  const name = (row: LeaderboardRow): string => learnerName(row.identity, row.tag, text);
  const board = shownLeaderboard(state);
  const shown = leaderboardColumns(state.catalog, text, locale, state.board.quiz);
  const sort = shown.some((column) => column.key === chosen.key) ? chosen : RANK_ORDER;
  const { colours } = usePresenceView();
  const rows = board?.board.rows ?? [];
  const own = board === undefined ? undefined : ownRow(board.board, mine);
  const apart = own !== undefined && !rows.some((row) => row.tag === own.tag) ? own : undefined;
  const counted = board?.board.window;
  const bests = state.board.quiz === undefined;

  return (
    <PageFrame page={HOME_PAGES.leaderboard} wide overlay={<PanePeers scope={state.catalog === undefined ? undefined : roomScope(state.catalog.id, { screen: "leaderboard" })} opened={view.opened} />}>
      <QuizCard
        id={`${scope}-title`}
        card="leaderboard"
        anchor={PRESENCE_ANCHORS.leaderboard}
        icon={<CardIcon icon="list-ordered" />}
        title={text("quiz.leaderboard.title")}
        footerLeft={board === undefined ? undefined : <Updated board={board} text={text} locale={locale} long />}
      >
        <BoardChoices session={session} state={state} text={text} locale={locale} categories />
        {counted === undefined ? null : (
          <p data-board-window="" className="m-0 text-sm text-muted-foreground">
            {text("quiz.leaderboard.window", { from: formatInstant(counted.from, locale), until: formatInstant(counted.until, locale) })}
          </p>
        )}
        {board === undefined ? (
          <p role={view.opened ? "status" : undefined} className="m-0 text-sm">
            {text("quiz.app.fetching")}
          </p>
        ) : rows.length === 0 ? (
          <p data-board-empty="" className="m-0 text-sm">
            {text(state.board.period === "all-time" && bests ? "quiz.leaderboard.empty" : "quiz.leaderboard.emptyScope")}
          </p>
        ) : (
          <>
            <p id={`${scope}-description`} className="m-0 text-sm text-muted-foreground">
              {text("quiz.leaderboard.caption")}
            </p>
            <p data-board-count="" className="m-0 flex flex-wrap gap-x-double text-sm">
              <span className="quiz-nowrap">{text("quiz.leaderboard.learners", { count: board.board.learners })}</span>
              {board.board.learners > rows.length ? <span className="quiz-nowrap text-muted-foreground">{text("quiz.leaderboard.shown", { count: rows.length })}</span> : null}
            </p>
            <Records fold={bests ? 80 : 44} role="region" aria-labelledby={`${scope}-title`} aria-describedby={`${scope}-description`} tabIndex={0}>
              <table {...TABLE.table} data-head="sort" className="quiz-fold w-full border-collapse text-sm" aria-labelledby={`${scope}-title`} aria-describedby={`${scope}-description`}>
                <thead {...TABLE.group}>
                  <tr {...TABLE.row}>
                    {shown.map((column) => (
                      <SortHeading key={column.key} column={column} sort={sort} onSort={(key) => setSort(nextSort(sort, key))} text={text} className={cn(EDGE, "quiz-nowrap border-b-2")} />
                    ))}
                  </tr>
                </thead>
                <tbody {...TABLE.group}>
                  {sortLeaderboard(rows, sort, name, locale).map((row) => (
                    <PageRow key={row.tag} row={row} mine={row.tag === mine} state={state} bests={bests} online={colours.get(row.tag)} text={text} locale={locale} />
                  ))}
                </tbody>
                {apart === undefined ? null : (
                  <tbody {...TABLE.group} data-board-own="">
                    <Gap span={shown.length} />
                    <tr {...TABLE.row}>
                      <th role="rowheader" scope="rowgroup" colSpan={shown.length} data-cell="note" className={cn(EDGE, "text-xs font-semibold text-muted-foreground")}>
                        {text("quiz.leaderboard.own")}
                      </th>
                    </tr>
                    <PageRow row={apart} mine state={state} bests={bests} online={colours.get(apart.tag)} text={text} locale={locale} />
                  </tbody>
                )}
              </table>
            </Records>
            <p className="m-0 text-xs text-muted-foreground">{text("quiz.leaderboard.forFun")}</p>
          </>
        )}
      </QuizCard>
    </PageFrame>
  );
}
