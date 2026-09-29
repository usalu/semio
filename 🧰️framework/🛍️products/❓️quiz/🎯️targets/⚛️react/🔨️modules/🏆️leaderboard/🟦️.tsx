/** 🏆️ The leaderboard: every learner with a submitted run in one sortable table — rank, learner, total, best score per
 * quiz, badges, runs and last activity — with the current learner highlighted, and its excerpt at the centre of the
 * home grid (the top rows and the own row). Rows never carry learner ids (an anonymous id is a credential), only their
 * public tags; the own row is the one whose tag matches the tag of the own id. It is ephemeral shared state, polled
 * while the page is visible and refreshed when it becomes visible again, never announced to screen readers.
 */

import { useEffect, useId, useState, type ReactElement } from "react";
import { Icon, overviewCardChipClass } from "@semio-tech/ui-react/chrome";
import { learnerTag, roomScope, type CatalogView, type LeaderboardRow } from "@semio-tech/quiz";
import { localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { formatClock, formatInstant, formatPoints, formatScore } from "../📏️quantity/🟦️.ts";
import { learnerName } from "../🪪️identity/🟦️.tsx";
import { CardAction, CardIcon, PageFrame, QuizCard, cn, textPresentation, type PaneView } from "../🪟️chrome/🟦️.tsx";
import { HOME_PAGES, type QuizSession, type QuizState } from "../🧭️session/🟦️.ts";
import { OnlineMark, PRESENCE_ANCHORS, PanePeers, usePresenceView } from "../👥️presence/🟦️.tsx";

/** ⏱️ How often the leaderboard is asked for while visible. */
export const LEADERBOARD_POLL_MS = 10_000;

/** 🔢️ How many rows the leaderboard card shows before the own row. */
export const BOARD_EXCERPT_SIZE = 5;

/** 🔑️ A sortable column: a fixed one or the best score of one quiz. */
export type LeaderboardKey = "rank" | "learner" | "total" | "badges" | "runs" | "last-activity" | `best:${string}`;

/** ↕️ The order of the table. */
export interface LeaderboardSort {
  readonly key: LeaderboardKey;
  readonly direction: "ascending" | "descending";
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
      return row.best[key.slice("best:".length)] ?? -1;
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

/** ✂️ The rows the leaderboard card shows: the first `size` by rank, and the own row apart when it ranks lower. */
export function boardExcerpt(rows: readonly LeaderboardRow[], mine: string | undefined, size = BOARD_EXCERPT_SIZE): { readonly top: readonly LeaderboardRow[]; readonly own: LeaderboardRow | undefined } {
  const ranked = [...rows].sort((left, right) => left.rank - right.rank);
  const top = ranked.slice(0, size);
  const own = mine === undefined || top.some((row) => row.tag === mine) ? undefined : ranked.find((row) => row.tag === mine);
  return { top, own };
}

/** 🔁️ While `enabled`: calls `refresh` now, every `intervalMs` while the document is visible, and whenever it becomes
 * visible. */
export function usePolling(refresh: () => Promise<void>, intervalMs: number, enabled = true): void {
  useEffect(() => {
    if (!enabled) return;
    const tick = (): void => {
      if (typeof document === "undefined" || document.visibilityState !== "hidden") void refresh();
    };
    tick();
    const timer = setInterval(tick, intervalMs);
    document.addEventListener("visibilitychange", tick);
    return () => {
      clearInterval(timer);
      document.removeEventListener("visibilitychange", tick);
    };
  }, [refresh, intervalMs, enabled]);
}

function columns(catalog: CatalogView | undefined, text: QuizText, locale: QuizLocale): readonly { readonly key: LeaderboardKey; readonly label: string }[] {
  return [
    { key: "rank", label: text("quiz.leaderboard.rank") },
    { key: "learner", label: text("quiz.leaderboard.learner") },
    { key: "total", label: text("quiz.leaderboard.total") },
    ...(catalog?.quizzes ?? []).map((quiz) => ({ key: `best:${quiz.id}` as const, label: localized(quiz.title, locale) })),
    { key: "badges", label: text("quiz.leaderboard.badges") },
    { key: "runs", label: text("quiz.leaderboard.runs") },
    { key: "last-activity", label: text("quiz.leaderboard.lastActivity") },
  ];
}

const CELL = "px-single py-single align-top";
const NUMBER = "quiz-nowrap tabular-nums";

function Updated(props: { readonly at: number; readonly text: QuizText; readonly locale: QuizLocale; readonly long?: boolean }): ReactElement {
  const time = props.long ? formatInstant(props.at, props.locale) : formatClock(props.at, props.locale);
  return <span className={cn(overviewCardChipClass, "text-muted-foreground")}>{props.text("quiz.leaderboard.updated", { time })}</span>;
}

function BoardRow(props: { readonly row: LeaderboardRow; readonly mine: boolean; readonly name: string; readonly online: number | undefined; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { row, mine, name, online, text, locale } = props;
  return (
    <tr className={cn("border-t border-normal", mine && "quiz-me")} aria-current={mine ? "true" : undefined}>
      <td className={cn(CELL, NUMBER)}>{row.rank}</td>
      <th scope="row" className={cn(CELL, "quiz-name text-left", mine ? "font-semibold" : "font-normal")}>
        {name}
        {mine ? <span className="font-normal text-muted-foreground"> ({text("quiz.leaderboard.you")})</span> : null}
        {online === undefined ? null : <OnlineMark colour={online} text={text} />}
      </th>
      <td className={cn(CELL, NUMBER, "text-right")}>{formatPoints(row.total, locale)}</td>
      <td className={cn(CELL, NUMBER, "text-right")}>{row.badges.length}</td>
    </tr>
  );
}

/** 🏆️ The leaderboard card at the centre of the overview: the top rows, a gap, and the own row. Its heading and "Full
 * leaderboard" open the leaderboard page (`#board`). */
export function LeaderboardCard(props: { readonly session: QuizSession; readonly state: QuizState; readonly text: QuizText; readonly locale: QuizLocale; readonly revealed: boolean; readonly onOpen: () => void }): ReactElement {
  const { session, state, text, locale, revealed, onOpen } = props;
  const id = useId();
  const board = state.leaderboard;
  const mine = state.learner === undefined ? undefined : learnerTag(state.learner.id);
  const { top, own } = boardExcerpt(board?.board.rows ?? [], mine);
  const name = (row: LeaderboardRow): string => learnerName(row.identity, row.tag, text);
  const head = cn(CELL, "quiz-nowrap font-medium");
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
      footerLeft={board === undefined ? undefined : <Updated at={board.at} text={text} locale={locale} />}
      footerRight={
        <CardAction primary onClick={onOpen}>
          {text("quiz.leaderboard.full")}
        </CardAction>
      }
    >
      {board === undefined ? (
        <p className="m-0 text-xs text-muted-foreground">{text("quiz.app.fetching")}</p>
      ) : top.length === 0 ? (
        <p className="m-0 text-xs text-muted-foreground">{text("quiz.leaderboard.empty")}</p>
      ) : (
        <div className="min-h-0 flex-1 overflow-auto">
          <table className="w-full border-collapse text-sm">
            <caption className="sr-only">{text("quiz.leaderboard.topCaption")}</caption>
            <thead>
              <tr className="text-left text-muted-foreground">
                <th scope="col" className={head}>
                  {text("quiz.leaderboard.rank")}
                </th>
                <th scope="col" className={head}>
                  {text("quiz.leaderboard.learner")}
                </th>
                <th scope="col" className={cn(head, "text-right")}>
                  {text("quiz.leaderboard.total")}
                </th>
                <th scope="col" className={cn(head, "text-right")}>
                  {text("quiz.leaderboard.badges")}
                </th>
              </tr>
            </thead>
            <tbody>
              {top.map((row) => (
                <BoardRow key={row.tag} row={row} mine={row.tag === mine} name={name(row)} online={colours.get(row.tag)} text={text} locale={locale} />
              ))}
              {own === undefined ? null : (
                <>
                  <tr aria-hidden="true" className="border-t border-normal text-muted-foreground">
                    <td colSpan={4} className={cn(CELL, "text-center")}>
                      ⋯
                    </td>
                  </tr>
                  <BoardRow row={own} mine name={name(own)} online={colours.get(own.tag)} text={text} locale={locale} />
                </>
              )}
            </tbody>
          </table>
        </div>
      )}
    </QuizCard>
  );
}

/** 🏆️ The leaderboard page behind the centre card: every learner in one sortable table, kept up to date by home's
 * polling, with the others on this page inside it. */
export function LeaderboardPage(props: { readonly session: QuizSession; readonly state: QuizState; readonly text: QuizText; readonly locale: QuizLocale; readonly view: PaneView }): ReactElement {
  const { session, state, text, locale, view } = props;
  const scope = useId();
  const [sort, setSort] = useState<LeaderboardSort>({ key: "rank", direction: "ascending" });
  const mine = state.learner === undefined ? undefined : learnerTag(state.learner.id);
  const name = (row: LeaderboardRow): string => learnerName(row.identity, row.tag, text);
  const board = state.leaderboard;
  const shown = columns(state.catalog, text, locale);
  const { colours } = usePresenceView();
  const toggle = (key: LeaderboardKey): void =>
    setSort((present) => (present.key === key ? { key, direction: present.direction === "ascending" ? "descending" : "ascending" } : { key, direction: key === "rank" || key === "learner" ? "ascending" : "descending" }));
  const badgeLabel = (id: string): string => {
    const badge = state.catalog?.badges.find((candidate) => candidate.id === id);
    return badge === undefined ? id : `${textPresentation(badge.emoji)} ${localized(badge.label, locale)}`;
  };
  const edge = "border-b border-normal px-single py-single text-left align-top";

  return (
    <PageFrame page={HOME_PAGES.leaderboard} wide overlay={<PanePeers scope={state.catalog === undefined ? undefined : roomScope(state.catalog.id, { screen: "leaderboard" })} opened={view.opened} />}>
      <QuizCard
        id={`${scope}-title`}
        card="leaderboard"
        anchor={PRESENCE_ANCHORS.leaderboard}
        icon={<CardIcon icon="list-ordered" />}
        title={text("quiz.leaderboard.title")}
        footerLeft={board === undefined ? undefined : <Updated at={board.at} text={text} locale={locale} long />}
      >
        {board === undefined ? (
          <p role={view.opened ? "status" : undefined} className="m-0 text-sm">
            {text("quiz.app.fetching")}
          </p>
        ) : board.board.rows.length === 0 ? (
          <p className="m-0 text-sm">{text("quiz.leaderboard.empty")}</p>
        ) : (
          <>
            <p id={`${scope}-description`} className="m-0 text-sm text-muted-foreground">
              {text("quiz.leaderboard.caption")}
            </p>
            <div className="max-w-full overflow-x-auto" role="region" aria-labelledby={`${scope}-title`} aria-describedby={`${scope}-description`} tabIndex={0}>
              <table className="w-full border-collapse text-sm" aria-labelledby={`${scope}-title`} aria-describedby={`${scope}-description`}>
                <thead>
                  <tr>
                    {shown.map((column) => (
                      <th key={column.key} scope="col" aria-sort={sort.key === column.key ? sort.direction : undefined} className={cn(edge, "quiz-nowrap border-b-2")}>
                        <button
                          type="button"
                          className="quiz-target inline-flex cursor-pointer items-center gap-single border-0 bg-transparent p-0 text-left font-semibold text-foreground"
                          onClick={() => toggle(column.key)}
                          title={text("quiz.leaderboard.sort", { column: column.label })}
                        >
                          {column.label}
                          <Icon icon={sort.key !== column.key ? "chevrons-up-down" : sort.direction === "ascending" ? "chevron-up" : "chevron-down"} size="small" className={cn("shrink-0", sort.key !== column.key && "text-muted-foreground")} />
                        </button>
                      </th>
                    ))}
                  </tr>
                </thead>
                <tbody>
                  {sortLeaderboard(board.board.rows, sort, name, locale).map((row) => (
                    <tr key={row.tag} className={row.tag === mine ? "quiz-me" : undefined} aria-current={row.tag === mine ? "true" : undefined}>
                      <td className={cn(edge, NUMBER)}>{row.rank}</td>
                      <th scope="row" className={cn(edge, "quiz-name", row.tag === mine ? "font-semibold" : "font-normal")}>
                        {name(row)}
                        {row.tag === mine ? <span className="font-normal text-muted-foreground"> ({text("quiz.leaderboard.you")})</span> : null}
                        {colours.get(row.tag) === undefined ? null : <OnlineMark colour={colours.get(row.tag) ?? 0} text={text} />}
                      </th>
                      <td className={cn(edge, NUMBER)}>{formatPoints(row.total, locale)}</td>
                      {(state.catalog?.quizzes ?? []).map((quiz) => (
                        <td key={quiz.id} className={cn(edge, NUMBER)}>
                          {row.best[quiz.id] === undefined ? "–" : formatScore(row.best[quiz.id] ?? 0, locale)}
                        </td>
                      ))}
                      <td className={edge}>
                        <span className="quiz-nowrap tabular-nums">{row.badges.length}</span>
                        {row.badges.length === 0 ? null : <span className="text-muted-foreground"> {row.badges.map(badgeLabel).join(", ")}</span>}
                      </td>
                      <td className={cn(edge, NUMBER)}>{row.runs}</td>
                      <td className={cn(edge, "quiz-nowrap")}>{formatInstant(row.lastActivity, locale)}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </>
        )}
      </QuizCard>
    </PageFrame>
  );
}
