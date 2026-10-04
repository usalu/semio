/** 👁️ The read side: the solution-free catalog, a learner's runs and bests, one run with its sheet, hints and opened tasks, the leaderboard, and the crowd of a quiz.
 *
 * Id-keyed maps are emitted with their keys in code point order, like the Rust twin's `BTreeMap`s. A best is the
 * submitted run of a quiz with the most points whatever its challenge, and totals sum those points; the crowd mixes
 * every challenge and tallies a guessed value under the nearest authored one.
 *
 * @see ../../README.md — the views and the leaderboard ordering
 * @see ./🦀️.rs — the Rust twin
 */
import {
  CROWD_SCORE_BINS,
  LEADERBOARD_TOP,
  type Answer,
  type Best,
  type Catalog,
  type CatalogView,
  type Challenge,
  type CrowdCount,
  type CrowdItem,
  type CrowdScores,
  type CrowdTask,
  type CrowdView,
  type Hint,
  type Id,
  type Identity,
  type Leaderboard,
  type LeaderboardPeriod,
  type LeaderboardRow,
  type LeaderboardWindow,
  type LearnerView,
  type MatchingTask,
  type Quiz,
  type RunResult,
  type RunView,
  type Scale,
  type Score,
  type Sheet,
  type Slug,
  type Task,
  type TaskResult,
  type Timestamp,
} from "../../🧬️schema/🟦️.ts";
import { challengeRules, hintsOf } from "../⛰️challenge/🟦️.ts";
import { fnv1a32 } from "../🎲️randomness/🟦️.ts";
import { scaled } from "../📏️scoring/🟦️.ts";
import { iconOf, sheetOf } from "../🃏️sheet/🟦️.ts";
import type { LearnerState, LoadedQuiz, RunState } from "../🧾️lifecycle/🟦️.ts";
import { compareCodePoints } from "../✅️validation/🟦️.ts";

/** 🗝️ The same record with its keys in code point order. */
function sortedRecord<T>(record: Readonly<Record<string, T>>): Record<string, T> {
  return Object.fromEntries(Object.entries(record).sort(([left], [right]) => compareCodePoints(left, right)));
}

/** 📚️ The catalog without paths and badge rules, its quizzes without solutions, in catalog order. */
export function catalogView(catalog: Catalog, quizzes: readonly Quiz[]): CatalogView {
  return {
    id: catalog.id,
    title: catalog.title,
    introduction: catalog.introduction,
    quizzes: quizzes.map((quiz) => ({ id: quiz.id, emoji: quiz.emoji, title: quiz.title, description: quiz.description, tasks: quiz.tasks.map((task) => ({ id: task.id, kind: task.kind, title: task.title, ...iconOf(task) })) })),
    badges: catalog.badges.map((badge) => ({ id: badge.id, emoji: badge.emoji, label: badge.label, description: badge.description })),
  };
}

/** 📨️ One submitted run as a standing counts it: its quiz, its challenge, its score, the points it earned and when it was submitted. */
export type TranscriptRun = { readonly quiz: Slug; readonly challenge: Challenge; readonly score: Score; readonly points: number; readonly at: Timestamp };

/** 🎗️ One badge as a standing counts it: the quiz of the run that earned it and when. */
export type TranscriptBadge = { readonly badge: Slug; readonly quiz: Slug; readonly at: Timestamp };

/** 📜️ What every standing of a learner is made of: the tag and identity, the submitted runs in submission order and the badges in award order. It carries the learner id, which breaks the last tie and finds the caller and never leaves the proctor. */
export type Transcript = { readonly learner: Id; readonly tag: string; readonly identity: Identity; readonly runs: readonly TranscriptRun[]; readonly badges: readonly TranscriptBadge[] };

/** 📬️ The submitted runs of a learner in submission order (submittedAt, ties by start order). */
function submittedRuns(state: LearnerState): TranscriptRun[] {
  return state.runs
    .filter((run): run is RunState & { readonly result: NonNullable<RunState["result"]>; readonly submittedAt: number } => run.status === "submitted" && run.result !== undefined && run.submittedAt !== undefined)
    .sort((left, right) => left.submittedAt - right.submittedAt)
    .map((run) => ({ quiz: run.quiz, challenge: run.result.challenge, score: run.result.score, points: run.result.points, at: run.submittedAt }));
}

/** 🥇️ The best run per quiz id over `runs` in submission order — the one with the most points, a later run replacing it only with strictly more — and when the last best was raised. */
function bests(runs: readonly TranscriptRun[]): { readonly best: Record<string, Best>; readonly reachedAt: number | undefined } {
  const best: Record<string, Best> = {};
  let reachedAt: number | undefined;
  for (const run of runs) {
    if (Object.hasOwn(best, run.quiz) && best[run.quiz]!.points >= run.points) continue;
    best[run.quiz] = { challenge: run.challenge, score: run.score, points: run.points };
    reachedAt = run.at;
  }
  return { best: sortedRecord(best), reachedAt };
}

/** ➕️ The sum of the points of the best runs over the catalog quizzes in catalog order. */
function total(best: Readonly<Record<string, Best>>, catalog: CatalogView): number {
  return catalog.quizzes.reduce((sum, quiz) => (Object.hasOwn(best, quiz.id) ? sum + best[quiz.id]!.points : sum), 0);
}

/** 👤️ A registered learner's runs (newest first, each with its challenge and, once submitted, its score and points), badges, bests and total; `undefined` before registration. */
export function learnerView(state: LearnerState, catalog: CatalogView): LearnerView | undefined {
  if (!state.identity) return undefined;
  const { best } = bests(submittedRuns(state));
  const runs = [...state.runs].reverse().map((run) => ({
    run: run.run,
    quiz: run.quiz,
    challenge: run.challenge,
    status: run.status,
    startedAt: run.startedAt,
    ...(run.result ? { score: run.result.score, points: run.result.points } : {}),
    ...(run.submittedAt !== undefined ? { submittedAt: run.submittedAt } : {}),
  }));
  return { learner: state.learner, identity: state.identity, runs, badges: state.badges, best, total: total(best, catalog) };
}

/** 💡️ The hints of a run per task id, only for tasks that have any: what each recorded answer earns from `hintsOf`, tasks in code point order. */
function runHints(quiz: Quiz, sheet: Sheet, answers: Readonly<Record<string, Answer>>): Record<string, Hint[]> {
  return sortedRecord(
    Object.fromEntries(
      sheet.tasks.flatMap((sheetTask) => {
        const task = quiz.tasks.find((candidate) => candidate.id === sheetTask.id);
        const hints = task && Object.hasOwn(answers, sheetTask.id) ? hintsOf(task, sheetTask, answers[sheetTask.id]) : [];
        return hints.length > 0 ? [[sheetTask.id, hints] as const] : [];
      }),
    ),
  );
}

/** 🏃️ One run with the sheet rebuilt from its seed and challenge and the loaded quiz; on a timed run when each opened task was opened, and while the run is open at a challenge that hints the hints of its answers, when there are any; `undefined` when the run or its quiz is unknown. */
export function runView(state: LearnerState, run: string, quizzes: Readonly<Record<string, LoadedQuiz>>): RunView | undefined {
  const found = state.runs.find((candidate) => candidate.run === run);
  if (!found || !Object.hasOwn(quizzes, found.quiz)) return undefined;
  const quiz = quizzes[found.quiz]!.quiz;
  const rules = challengeRules(found.challenge);
  const sheet = sheetOf(quiz, found.seed, found.challenge);
  const hints = found.status === "open" && rules.hints ? runHints(quiz, sheet, found.answers) : {};
  return {
    run: found.run,
    learner: state.learner,
    quiz: found.quiz,
    status: found.status,
    sheet,
    answers: sortedRecord(found.answers),
    ...(found.result ? { result: found.result } : {}),
    startedAt: found.startedAt,
    ...(found.submittedAt !== undefined ? { submittedAt: found.submittedAt } : {}),
    ...(rules.timed ? { opened: sortedRecord(found.opened) } : {}),
    ...(Object.keys(hints).length > 0 ? { hints } : {}),
  };
}

/** 🏷️ The public, non-reversible tag of a learner: FNV-1a of the id as 8 lowercase hex digits; a client finds its own leaderboard row by computing it from its own id. */
export function learnerTag(learner: string): string {
  return fnv1a32(learner).toString(16).padStart(8, "0");
}

/** 📃️ The transcript of a registered learner with a submitted run, `undefined` otherwise; it changes only when the learner submits a run, earns a badge or registers. A badge whose run is unknown is left out. */
export function transcript(state: LearnerState): Transcript | undefined {
  const runs = submittedRuns(state);
  if (!state.identity || runs.length === 0) return undefined;
  const badges = state.badges.flatMap((award): TranscriptBadge[] => {
    const run = state.runs.find((candidate) => candidate.run === award.run);
    return run ? [{ badge: award.badge, quiz: run.quiz, at: award.at }] : [];
  });
  return { learner: state.learner, tag: learnerTag(state.learner), identity: state.identity, runs, badges };
}

const DAY = 86_400_000;

/** 📆️ The day (days since the Unix epoch) on which the month of `day` begins and the one on which the next month does, in the proleptic Gregorian calendar: the years are counted from March inside their 400-year era, so the leap day is the last of a year.
 *
 * @see https://howardhinnant.github.io/date_algorithms.html#civil_from_days */
function monthOf(day: number): readonly [number, number] {
  const dayOfEra = (day + 719_468) % 146_097;
  const yearOfEra = Math.floor((dayOfEra - Math.floor(dayOfEra / 1460) + Math.floor(dayOfEra / 36_524) - Math.floor(dayOfEra / 146_096)) / 365);
  const dayOfYear = dayOfEra - (365 * yearOfEra + Math.floor(yearOfEra / 4) - Math.floor(yearOfEra / 100));
  const month = Math.floor((5 * dayOfYear + 2) / 153);
  const leap = yearOfEra % 4 === 3 && (yearOfEra % 100 !== 99 || yearOfEra === 399);
  const begins = (index: number): number => (index === 12 ? (leap ? 366 : 365) : Math.floor((153 * index + 2) / 5));
  return [day - dayOfYear + begins(month), day - dayOfYear + begins(month + 1)];
}

/** 🪟️ The window of `period` that contains the instant `at`: its day, its ISO week (from Monday) or its month in UTC — half-open, never starting before the epoch — and none for `all-time`. */
export function periodWindow(period: LeaderboardPeriod, at: Timestamp): LeaderboardWindow | undefined {
  const day = Math.floor(at / DAY);
  switch (period) {
    case "daily":
      return { from: day * DAY, until: (day + 1) * DAY };
    case "weekly": {
      const until = day + 7 - ((day + 3) % 7);
      return { from: Math.max(0, until - 7) * DAY, until: until * DAY };
    }
    case "monthly": {
      const [from, until] = monthOf(day);
      return { from: from * DAY, until: until * DAY };
    }
    case "all-time":
      return undefined;
  }
}

/** 🔭️ Which runs a leaderboard counts: those submitted inside `window` (every run without one), of `quiz` only when it names one. */
export type BoardScope = { readonly window?: LeaderboardWindow; readonly quiz?: Slug };

/** 🎯️ The scope of the leaderboard of `period` — of `quiz` only when it names one — at the instant `at`. */
export function boardScope(period: LeaderboardPeriod, quiz: Slug | undefined, at: Timestamp): BoardScope {
  const window = periodWindow(period, at);
  return { ...(window ? { window } : {}), ...(quiz === undefined ? {} : { quiz }) };
}

function counts(scope: BoardScope, quiz: Slug, at: Timestamp): boolean {
  return (scope.quiz === undefined || scope.quiz === quiz) && (scope.window === undefined || (scope.window.from <= at && at < scope.window.until));
}

/** 🧍️ One learner's unranked leaderboard row together with the learner id that breaks the last tie and finds the caller; the id never leaves the proctor. */
export type Standing = { readonly learner: Id } & Omit<LeaderboardRow, "rank">;

/** 📈️ The standing of a transcript in `scope`, made of the runs in scope only — their bests, their count, the last of them and the badges they earned; `undefined` when no run is in scope. */
export function standing(transcript: Transcript, catalog: CatalogView, scope: BoardScope = {}): Standing | undefined {
  const runs = transcript.runs.filter((run) => counts(scope, run.quiz, run.at));
  const { best, reachedAt } = bests(runs);
  const last = runs[runs.length - 1];
  if (reachedAt === undefined || last === undefined) return undefined;
  const badges = transcript.badges.filter((award) => counts(scope, award.quiz, award.at)).map((award) => award.badge);
  return { learner: transcript.learner, tag: transcript.tag, identity: transcript.identity, total: total(best, catalog), reachedAt, best, badges, runs: runs.length, lastActivity: last.at };
}

/** 🥈️ The order of the leaderboard: total ↓, badge count ↓, reachedAt ↑, learner id ↑. */
export function compareStandings(left: Standing, right: Standing): number {
  return right.total - left.total || right.badges.length - left.badges.length || left.reachedAt - right.reachedAt || compareCodePoints(left.learner, right.learner);
}

/** 🏆️ The leaderboard of `board` at the instant `at` over every transcript: the top {@link LEADERBOARD_TOP} rows by rank of the standings in scope, how many learners are ranked, how many runs were submitted in all, and the row of `caller` when that learner is ranked (also inside the top); rows carry the learner tag, never the id. */
export function leaderboard(transcripts: readonly Transcript[], catalog: CatalogView, board: { readonly period: LeaderboardPeriod; readonly quiz?: Slug }, at: Timestamp, caller?: Id): Leaderboard {
  const scope = boardScope(board.period, board.quiz, at);
  const ranked = transcripts
    .flatMap((candidate) => standing(candidate, catalog, scope) ?? [])
    .sort(compareStandings)
    .map(({ learner, ...row }, index) => ({ learner, row: { rank: index + 1, ...row } }));
  const own = caller === undefined ? undefined : ranked.find((entry) => entry.learner === caller)?.row;
  return {
    period: board.period,
    ...(scope.quiz === undefined ? {} : { quiz: scope.quiz }),
    ...(scope.window ? { window: scope.window } : {}),
    rows: ranked.slice(0, LEADERBOARD_TOP).map((entry) => entry.row),
    learners: ranked.length,
    submissions: transcripts.reduce((sum, candidate) => sum + candidate.runs.length, 0),
    ...(own ? { own } : {}),
  };
}

/** 🔣️ The key of a value in crowd counts: its ECMAScript `Number::toString` text, the shortest round-trip JSON number (`0.12`, `250`, `1e+21`, `1e-7`; `-0` → `0`) — Rust `value_key`. */
export function valueKey(value: number): string {
  return String(value);
}

/** 🎟️ Counts per distinct key, keys ascending by code point (category ids and value keys alike, so `120` precedes `15`). */
function counted(keys: readonly string[]): CrowdCount[] {
  const counts = new Map<string, number>();
  for (const key of keys) counts.set(key, (counts.get(key) ?? 0) + 1);
  return [...counts.entries()].sort(([left], [right]) => compareCodePoints(left, right)).map(([key, count]) => ({ key, count }));
}

/** 🔎️ Per result, its first result of the task with the task's kind. */
function answeredTasks<K extends TaskResult["kind"]>(results: readonly RunResult[], task: string, kind: K): Extract<TaskResult, { kind: K }>[] {
  return results.flatMap((result) => {
    const found = result.tasks.find((candidate) => candidate.task === task && candidate.kind === kind);
    return found ? [found as Extract<TaskResult, { kind: K }>] : [];
  });
}

/** 🪣️ The bin of a score among the {@link CROWD_SCORE_BINS}: a tenth of its whole percent ⌊score · 100 + ½⌋, rounded down, the last bin closed (`[90, 100]`). */
export function scoreBin(score: Score): number {
  return Math.max(0, Math.min(CROWD_SCORE_BINS - 1, Math.floor(Math.floor(score * 100 + 0.5) / 10)));
}

/** 📉️ How many of `scores` fall into each score bin. */
function binned(scores: readonly Score[]): CrowdScores {
  const bins = new Array<number>(CROWD_SCORE_BINS).fill(0);
  for (const score of scores) bins[scoreBin(score)]! += 1;
  return bins;
}

/** 🎪️ How many items a sheet of `task` presents: `draw` when it is set and smaller than the item count, else every item. */
export function presented(task: Task): number {
  return task.draw !== undefined && task.draw < task.items.length ? task.draw : task.items.length;
}

/** 🪑️ The place among `places` presented ones that `position` in a learner's order of `length` items counts for: `position · (places − 1) / (length − 1)` rounded half up in integer arithmetic — the position itself when `length` equals `places`, place 0 for an order of fewer than two items, never beyond the last place. */
export function placeBin(position: number, length: number, places: number): number {
  return length < 2 || places < 1 ? 0 : Math.min(places - 1, Math.floor((2 * position * (places - 1) + (length - 1)) / (2 * (length - 1))));
}

/** 📌️ The value among `values` nearest to a guess on `scale`, the smaller of two equally near; `undefined` among none — the rule a guess is tallied by in the crowd, for anyone who marks where it counts. */
export function nearestOf(values: Iterable<number>, scale: Scale, guess: number): number | undefined {
  const position = scaled(guess, scale);
  let nearest: number | undefined;
  let gap = Infinity;
  for (const value of values) {
    const distance = Math.abs(scaled(value, scale) - position);
    if (nearest !== undefined && !(distance < gap || (distance === gap && value < nearest))) continue;
    nearest = value;
    gap = distance;
  }
  return nearest;
}

/** 🧭️ The authored value of `dimension` nearest to a guess on the dimension's scale ({@link nearestOf}); `undefined` when no item of the task carries one. */
export function nearestValue(task: MatchingTask, dimension: Slug, scale: Scale, guess: number): number | undefined {
  return nearestOf(task.items.flatMap((item) => (Object.hasOwn(item.values, dimension) ? [item.values[dimension]!] : [])), scale, guess);
}

/** 👪️ What the learners answered and scored in the submitted results of one quiz, every challenge mixed: the run scores per score bin; per task the scores of the results that count for it (per dimension the dimension's scores for a matching), classification counts per assigned category, sorting mean normalized position (position / (len − 1), 0 for a single item) summed in result order beside the count per presented place ({@link placeBin}), matching counts per assigned value key per dimension — a guess (an item result that carries `miss`) under the authored value nearest to it ({@link nearestValue}) — counts keys ascending by code point; tasks, dimensions and items in definition order, unanswered items left out, results of other quizzes ignored. An item left unanswered counts nowhere, and a sorting nobody guessed in (every item a miss without a guess: no answer, or none that says anything) adds its score only. */
export function crowdView(quiz: Quiz, results: readonly RunResult[]): CrowdView {
  const runs = results.filter((result) => result.quiz === quiz.id);
  const tasks = quiz.tasks.flatMap((task): CrowdTask[] => {
    switch (task.kind) {
      case "classification": {
        const answered = answeredTasks(runs, task.id, "classification");
        const items = task.items.flatMap((item): CrowdItem[] => {
          const assigned = answered.flatMap((result) => result.items.find((candidate) => candidate.item === item.id)?.assigned ?? []);
          return assigned.length === 0 ? [] : [{ item: item.id, answers: assigned.length, counts: counted(assigned) }];
        });
        return [{ task: task.id, kind: task.kind, scores: binned(answered.map((result) => result.score)), items }];
      }
      case "sorting": {
        const answered = answeredTasks(runs, task.id, "sorting");
        const ordered = answered.filter((result) => !result.items.every((item) => item.miss === true && item.guess === undefined));
        const count = presented(task);
        const items = task.items.flatMap((item): CrowdItem[] => {
          const orders = ordered.flatMap((result) => {
            const found = result.items.find((candidate) => candidate.item === item.id);
            return found === undefined ? [] : [{ position: found.position, length: result.items.length }];
          });
          if (orders.length === 0) return [];
          const places = new Array<number>(count).fill(0);
          for (const { position, length } of orders) if (count > 0) places[placeBin(position, length, count)]! += 1;
          return [{ item: item.id, answers: orders.length, meanPosition: orders.reduce((sum, { position, length }) => sum + (length > 1 ? position / (length - 1) : 0), 0) / orders.length, places }];
        });
        return [{ task: task.id, kind: task.kind, scores: binned(answered.map((result) => result.score)), items }];
      }
      case "matching": {
        const answered = answeredTasks(runs, task.id, "matching");
        return task.dimensions.map((dimension): CrowdTask => {
          const results = answered.flatMap((result) => result.dimensions.find((candidate) => candidate.dimension === dimension.id) ?? []);
          const items = task.items.flatMap((item): CrowdItem[] => {
            const assigned = results.flatMap((result) => {
              const found = result.items.find((candidate) => candidate.item === item.id);
              if (found?.assigned === undefined) return [];
              const value = found.miss === undefined ? found.assigned : nearestValue(task, dimension.id, dimension.quantity.scale, found.assigned);
              return value === undefined ? [] : [valueKey(value)];
            });
            return assigned.length === 0 ? [] : [{ item: item.id, answers: assigned.length, counts: counted(assigned) }];
          });
          return { task: task.id, kind: task.kind, dimension: dimension.id, scores: binned(results.map((result) => result.score)), items };
        });
      }
    }
  });
  return { quiz: quiz.id, runs: runs.length, scores: binned(runs.map((result) => result.score)), tasks };
}
