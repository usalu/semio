/** 👁️ The read side: the solution-free catalog, a learner's runs and bests, one run with its sheet, the leaderboard, and the crowd of a quiz.
 *
 * Id-keyed maps are emitted with their keys in code point order, like the Rust twin's `BTreeMap`s.
 *
 * @see ../../README.md — the views and the leaderboard ordering
 * @see ./🦀️.rs — the Rust twin
 */
import type { Catalog, CatalogView, CrowdCount, CrowdItem, CrowdTask, CrowdView, Leaderboard, LeaderboardRow, LearnerView, Quiz, RunResult, RunView, Score, TaskResult } from "../../🧬️schema/🟦️.ts";
import { fnv1a32 } from "../🎲️randomness/🟦️.ts";
import { sheetOf } from "../🃏️sheet/🟦️.ts";
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
    quizzes: quizzes.map((quiz) => ({ id: quiz.id, emoji: quiz.emoji, title: quiz.title, description: quiz.description, tasks: quiz.tasks.map((task) => ({ id: task.id, kind: task.kind, title: task.title })) })),
    badges: catalog.badges.map((badge) => ({ id: badge.id, emoji: badge.emoji, label: badge.label, description: badge.description })),
  };
}

/** 📬️ The submitted runs of a learner in submission order (submittedAt, ties by start order). */
function submittedRuns(state: LearnerState): (RunState & { readonly result: NonNullable<RunState["result"]>; readonly submittedAt: number })[] {
  return state.runs.filter((run): run is RunState & { readonly result: NonNullable<RunState["result"]>; readonly submittedAt: number } => run.status === "submitted" && run.result !== undefined && run.submittedAt !== undefined).sort((left, right) => left.submittedAt - right.submittedAt);
}

/** 🥇️ The best submitted score per quiz id and when the last best was raised. */
function bests(state: LearnerState): { readonly best: Record<string, Score>; readonly reachedAt: number | undefined } {
  const best: Record<string, Score> = {};
  let reachedAt: number | undefined;
  for (const run of submittedRuns(state)) {
    if (Object.hasOwn(best, run.quiz) && best[run.quiz]! >= run.result.score) continue;
    best[run.quiz] = run.result.score;
    reachedAt = run.submittedAt;
  }
  return { best: sortedRecord(best), reachedAt };
}

/** ➕️ The sum of the best scores over the catalog quizzes in catalog order, in points (score × 100). */
function total(best: Readonly<Record<string, Score>>, catalog: CatalogView): number {
  return catalog.quizzes.reduce((sum, quiz) => (Object.hasOwn(best, quiz.id) ? sum + best[quiz.id]! * 100 : sum), 0);
}

/** 👤️ A registered learner's runs (newest first), badges, bests and total; `undefined` before registration. */
export function learnerView(state: LearnerState, catalog: CatalogView): LearnerView | undefined {
  if (!state.identity) return undefined;
  const { best } = bests(state);
  const runs = [...state.runs].reverse().map((run) => ({ run: run.run, quiz: run.quiz, status: run.status, ...(run.result ? { score: run.result.score } : {}), startedAt: run.startedAt, ...(run.submittedAt !== undefined ? { submittedAt: run.submittedAt } : {}) }));
  return { learner: state.learner, identity: state.identity, runs, badges: state.badges, best, total: total(best, catalog) };
}

/** 🏃️ One run with the sheet rebuilt from its seed and the loaded quiz; `undefined` when the run or its quiz is unknown. */
export function runView(state: LearnerState, run: string, quizzes: Readonly<Record<string, LoadedQuiz>>): RunView | undefined {
  const found = state.runs.find((candidate) => candidate.run === run);
  if (!found || !Object.hasOwn(quizzes, found.quiz)) return undefined;
  return {
    run: found.run,
    learner: state.learner,
    quiz: found.quiz,
    status: found.status,
    sheet: sheetOf(quizzes[found.quiz]!.quiz, found.seed),
    answers: sortedRecord(found.answers),
    ...(found.result ? { result: found.result } : {}),
    startedAt: found.startedAt,
    ...(found.submittedAt !== undefined ? { submittedAt: found.submittedAt } : {}),
  };
}

/** 🏷️ The public, non-reversible tag of a learner: FNV-1a of the id as 8 lowercase hex digits; a client finds its own leaderboard row by computing it from its own id. */
export function learnerTag(learner: string): string {
  return fnv1a32(learner).toString(16).padStart(8, "0");
}

/** 🏆️ Every learner with a submitted run, ordered by total ↓, badge count ↓, reachedAt ↑, learner id ↑ and ranked by position; rows carry the learner tag, never the id. */
export function leaderboard(states: readonly LearnerState[], catalog: CatalogView): Leaderboard {
  const rows = states.flatMap((state): { readonly learner: string; readonly row: Omit<LeaderboardRow, "rank"> }[] => {
    const { best, reachedAt } = bests(state);
    if (!state.identity || reachedAt === undefined) return [];
    return [{ learner: state.learner, row: { tag: learnerTag(state.learner), identity: state.identity, total: total(best, catalog), reachedAt, best, badges: state.badges.map((award) => award.badge), runs: submittedRuns(state).length, lastActivity: state.lastActivity ?? reachedAt } }];
  });
  rows.sort((left, right) => right.row.total - left.row.total || right.row.badges.length - left.row.badges.length || left.row.reachedAt - right.row.reachedAt || compareCodePoints(left.learner, right.learner));
  return { rows: rows.map(({ row }, index) => ({ rank: index + 1, ...row })) };
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

/** 👪️ What the learners answered in the submitted results of one quiz: classification counts per assigned category, sorting mean normalized position (position / (len − 1), 0 for a single item) summed in result order, matching counts per assigned value key per dimension, counts keys ascending by code point; tasks, dimensions and items in definition order, unanswered items left out, results of other quizzes ignored. */
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
        return [{ task: task.id, kind: task.kind, items }];
      }
      case "sorting": {
        const answered = answeredTasks(runs, task.id, "sorting");
        const items = task.items.flatMap((item): CrowdItem[] => {
          const positions = answered.flatMap((result) => {
            const found = result.items.find((candidate) => candidate.item === item.id);
            return found === undefined ? [] : [result.items.length > 1 ? found.position / (result.items.length - 1) : 0];
          });
          return positions.length === 0 ? [] : [{ item: item.id, answers: positions.length, meanPosition: positions.reduce((sum, position) => sum + position, 0) / positions.length }];
        });
        return [{ task: task.id, kind: task.kind, items }];
      }
      case "matching": {
        const answered = answeredTasks(runs, task.id, "matching");
        return task.dimensions.map((dimension): CrowdTask => {
          const results = answered.flatMap((result) => result.dimensions.find((candidate) => candidate.dimension === dimension.id) ?? []);
          const items = task.items.flatMap((item): CrowdItem[] => {
            const assigned = results.flatMap((result) => {
              const found = result.items.find((candidate) => candidate.item === item.id);
              return found === undefined ? [] : [valueKey(found.assigned)];
            });
            return assigned.length === 0 ? [] : [{ item: item.id, answers: assigned.length, counts: counted(assigned) }];
          });
          return { task: task.id, kind: task.kind, dimension: dimension.id, items };
        });
      }
    }
  });
  return { quiz: quiz.id, runs: runs.length, tasks };
}
