/** 🏆️ Subject adapter of the leaderboard case: `@semio-tech/course` folds every committed stream and answers with its views.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/👁️views/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "@semio-tech/framework/test/adapter";
import {
  LEADERBOARD_PERIODS,
  type Course,
  type CourseView,
  type Event,
  type Leaderboard,
  type LeaderboardPeriod,
  type LeaderboardRow,
  type LearnerState,
  type Quiz,
  type Transcript,
  courseView,
  emptyLearnerState,
  evolveLearner,
  leaderboard,
  learnerView,
  periodWindow,
  transcript,
} from "../../📦️packages/🟦️typescript/🟦️.ts";

const VECTORS = "shared://🏆️leaderboard/🔣️.json";

type Caller = { readonly id: string; readonly learner?: string };
type Board = { readonly id: string; readonly period: LeaderboardPeriod; readonly quiz?: string; readonly at: number };
type Vectors = {
  readonly course: Course;
  readonly quizzes: readonly Quiz[];
  readonly courses: readonly { readonly id: string; readonly course: Course; readonly quizzes: readonly Quiz[] }[];
  readonly windows: readonly { readonly id: string; readonly at: number }[];
  readonly vectors: readonly { readonly id: string; readonly learners: readonly { readonly learner: string; readonly events: readonly Event[] }[]; readonly callers: readonly Caller[]; readonly boards: readonly Board[] }[];
  readonly crowd: { readonly transcripts: readonly Transcript[]; readonly board: Board; readonly cuts: readonly { readonly id: string; readonly learners: number; readonly callers: readonly Caller[] }[] };
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.inputBytes(VECTORS))) as Vectors;
}

/** 🗂️ Every learner of a vector folded from its committed events. */
function folded(vector: Vectors["vectors"][number]): LearnerState[] {
  return vector.learners.map((learner) => learner.events.reduce(evolveLearner, emptyLearnerState(learner.learner)));
}

/** 🙋️ One leaderboard as every committed caller is answered, keyed by the caller's label. */
function asked(transcripts: readonly Transcript[], view: CourseView, board: Board, callers: readonly Caller[]): Record<string, Leaderboard> {
  return Object.fromEntries(callers.map((caller) => [caller.id, leaderboard(transcripts, view, board, board.at, caller.learner)]));
}

/** 🪧️ A leaderboard reduced to what a cut decides: `rank:tag` of every row, the number of ranked learners and `rank:tag` of the own row. */
function outline(board: Leaderboard): { rows: string[]; learners: number; own: string | null } {
  const place = (row: LeaderboardRow): string => `${row.rank}:${row.tag}`;
  return { rows: board.rows.map(place), learners: board.learners, own: board.own ? place(board.own) : null };
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "course-views": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).courses.map((vector) => [vector.id, courseView(vector.course, vector.quizzes)])) }) },
    "learner-views": {
      subject: (ctx) => {
        const committed = vectors(ctx);
        const view = courseView(committed.course, committed.quizzes);
        return { projection: Object.fromEntries(committed.vectors.map((vector) => [vector.id, Object.fromEntries(folded(vector).map((state) => [state.learner, learnerView(state, view) ?? null]))])) };
      },
    },
    windows: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).windows.map((vector) => [vector.id, Object.fromEntries(LEADERBOARD_PERIODS.map((period) => [period, periodWindow(period, vector.at) ?? null]))])) }) },
    rankings: {
      subject: (ctx) => {
        const committed = vectors(ctx);
        const view = courseView(committed.course, committed.quizzes);
        return {
          projection: Object.fromEntries(
            committed.vectors.map((vector) => {
              const transcripts = folded(vector).flatMap((state) => transcript(state) ?? []);
              return [vector.id, Object.fromEntries(vector.boards.map((board) => [board.id, asked(transcripts, view, board, vector.callers)]))];
            }),
          ),
        };
      },
    },
    cuts: {
      subject: (ctx) => {
        const committed = vectors(ctx);
        const view = courseView(committed.course, committed.quizzes);
        const { crowd } = committed;
        return { projection: Object.fromEntries(crowd.cuts.map((vector) => [vector.id, Object.fromEntries(Object.entries(asked(crowd.transcripts.slice(0, vector.learners), view, crowd.board, vector.callers)).map(([caller, board]) => [caller, outline(board)]))])) };
      },
    },
  },
});
