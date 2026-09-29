/** 🏆️ The leaderboard: every learner with a submitted run in one sortable table — rank, learner, total, best score per
 * quiz, badges, runs and last activity — with the current learner highlighted. Rows never carry learner ids (an
 * anonymous id is a credential), only their public tags; the own row is the one whose tag matches the tag of the own
 * id. It is ephemeral shared state, polled while the page is visible and refreshed when it becomes visible again.
 */

import { useEffect, useId, useState, type ReactElement } from "react";
import { learnerTag, type CatalogView, type LeaderboardRow } from "@semio-tech/quiz";
import { localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { formatInstant, formatPoints, formatScore } from "../📏️quantity/🟦️.ts";
import { learnerName } from "../🪪️identity/🟦️.tsx";
import type { QuizSession, QuizState } from "../🧭️session/🟦️.ts";

/** ⏱️ How often the leaderboard is asked for while visible. */
export const LEADERBOARD_POLL_MS = 10_000;

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

/** 🔁️ Calls `refresh` now, every `intervalMs` while the document is visible, and whenever it becomes visible. */
export function usePolling(refresh: () => Promise<void>, intervalMs: number): void {
  useEffect(() => {
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
  }, [refresh, intervalMs]);
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

/** 🏆️ The leaderboard screen. */
export function LeaderboardScreen(props: { readonly session: QuizSession; readonly state: QuizState; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { session, state, text, locale } = props;
  const scope = useId();
  const [sort, setSort] = useState<LeaderboardSort>({ key: "rank", direction: "ascending" });
  usePolling(session.refreshLeaderboard, LEADERBOARD_POLL_MS);
  const mine = state.learner === undefined ? undefined : learnerTag(state.learner.id);
  const name = (row: LeaderboardRow): string => learnerName(row.identity, row.tag, text);
  const board = state.leaderboard;
  const shown = columns(state.catalog, text, locale);
  const toggle = (key: LeaderboardKey): void =>
    setSort((present) => (present.key === key ? { key, direction: present.direction === "ascending" ? "descending" : "ascending" } : { key, direction: key === "rank" || key === "learner" ? "ascending" : "descending" }));
  const badgeLabel = (id: string): string => {
    const badge = state.catalog?.badges.find((candidate) => candidate.id === id);
    return badge === undefined ? id : `${badge.emoji} ${localized(badge.label, locale)}`;
  };

  return (
    <section className="quiz-leaderboard">
      <h1 id={`${scope}-title`} tabIndex={-1}>
        {text("quiz.leaderboard.title")}
      </h1>
      {board === undefined ? (
        <p role="status">{text("quiz.app.fetching")}</p>
      ) : board.board.rows.length === 0 ? (
        <p>{text("quiz.leaderboard.empty")}</p>
      ) : (
        <>
          <p id={`${scope}-description`} className="quiz-muted">
            {text("quiz.leaderboard.caption")}
          </p>
          <div className="quiz-table-scroll" role="region" aria-labelledby={`${scope}-title`} aria-describedby={`${scope}-description`} tabIndex={0}>
            <table className="quiz-table quiz-leaderboard-table" aria-labelledby={`${scope}-title`} aria-describedby={`${scope}-description`}>
              <thead>
                <tr>
                  {shown.map((column) => (
                    <th key={column.key} scope="col" aria-sort={sort.key === column.key ? sort.direction : undefined}>
                      <button type="button" className="quiz-sort" onClick={() => toggle(column.key)} title={text("quiz.leaderboard.sort", { column: column.label })}>
                        {column.label}
                        <span aria-hidden="true">{sort.key === column.key ? (sort.direction === "ascending" ? " ▲" : " ▼") : " ↕"}</span>
                      </button>
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {sortLeaderboard(board.board.rows, sort, name, locale).map((row) => (
                  <tr key={row.tag} className={row.tag === mine ? "quiz-me" : undefined} aria-current={row.tag === mine ? "true" : undefined}>
                    <td>{row.rank}</td>
                    <th scope="row">
                      {name(row)}
                      {row.tag === mine ? <strong className="quiz-me-tag"> ({text("quiz.leaderboard.you")})</strong> : null}
                    </th>
                    <td>{formatPoints(row.total, locale)}</td>
                    {(state.catalog?.quizzes ?? []).map((quiz) => (
                      <td key={quiz.id}>{row.best[quiz.id] === undefined ? "–" : formatScore(row.best[quiz.id] ?? 0, locale)}</td>
                    ))}
                    <td>
                      <span className="quiz-badge-count">{row.badges.length}</span>
                      {row.badges.length === 0 ? null : <span className="quiz-badge-list"> {row.badges.map(badgeLabel).join(", ")}</span>}
                    </td>
                    <td>{row.runs}</td>
                    <td>{formatInstant(row.lastActivity, locale)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </>
      )}
      {board === undefined ? null : <p className="quiz-muted">{text("quiz.leaderboard.updated", { time: formatInstant(board.at, locale) })}</p>}
    </section>
  );
}
