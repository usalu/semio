/** 🧾️ Subject adapter of the learner-lifecycle case: `@semio-tech/quiz` replays every committed sequence through its deciders.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🧾️lifecycle/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import {
  type Catalog,
  type Command,
  type Decision,
  type Event,
  type IdentifyLearnerCommand,
  type LearnerState,
  type LoadedQuiz,
  type Quiz,
  catalogView,
  decideLearner,
  decideRoster,
  emptyLearnerState,
  emptyRosterState,
  evolveLearner,
  evolveRoster,
  learnerView,
  normalizeHandle,
  runView,
} from "../../📦️packages/🟦️typescript/🟦️.ts";

const VECTORS = "shared://🧾️learner-lifecycle/🔣️.json";

type Step<C> = { readonly command: C; readonly now: number; readonly revisions?: Readonly<Record<string, string>> };
type Sequence = { readonly id: string; readonly learner: string; readonly given: readonly Event[]; readonly steps: readonly Step<Exclude<Command, IdentifyLearnerCommand>>[]; readonly views?: { readonly runs: readonly string[] } };
type Vectors = {
  readonly catalog: Catalog;
  readonly quizzes: readonly Quiz[];
  readonly revisions: Readonly<Record<string, string>>;
  readonly handles: readonly { readonly id: string; readonly handle: string }[];
  readonly roster: readonly { readonly id: string; readonly steps: readonly Step<IdentifyLearnerCommand>[] }[];
  readonly learners: readonly Sequence[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
}

/** 📚️ The loaded quizzes at the given revisions. */
function loaded(committed: Vectors, revisions: Readonly<Record<string, string>>): Record<string, LoadedQuiz> {
  return Object.fromEntries(committed.quizzes.map((quiz) => [quiz.id, { quiz, revision: revisions[quiz.id]! }]));
}

/** 🎞️ Folds the given events, then decides and folds every step. */
function replay(committed: Vectors, sequence: Sequence): { decisions: Decision[]; state: LearnerState; quizzes: Record<string, LoadedQuiz> } {
  let state = sequence.given.reduce(evolveLearner, emptyLearnerState(sequence.learner));
  let revisions = { ...committed.revisions };
  const decisions: Decision[] = [];
  for (const step of sequence.steps) {
    revisions = { ...revisions, ...step.revisions };
    const decision = decideLearner(state, step.command, { now: step.now, catalog: committed.catalog, quizzes: loaded(committed, revisions) });
    decisions.push(decision);
    if ("events" in decision) state = decision.events.reduce(evolveLearner, state);
  }
  return { decisions, state, quizzes: loaded(committed, revisions) };
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    handles: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).handles.map((vector) => [vector.id, normalizeHandle(vector.handle) ?? null])) }) },
    roster: {
      subject: (ctx) => ({
        projection: Object.fromEntries(
          vectors(ctx).roster.map((sequence) => {
            let state = emptyRosterState();
            return [
              sequence.id,
              sequence.steps.map((step) => {
                const decision = decideRoster(state, step.command, step.now);
                if ("events" in decision) state = decision.events.reduce(evolveRoster, state);
                return decision;
              }),
            ];
          }),
        ),
      }),
    },
    "learner-decisions": {
      subject: (ctx) => {
        const committed = vectors(ctx);
        return { projection: Object.fromEntries(committed.learners.map((sequence) => [sequence.id, replay(committed, sequence).decisions])) };
      },
    },
    "learner-views": {
      subject: (ctx) => {
        const committed = vectors(ctx);
        const view = catalogView(committed.catalog, committed.quizzes);
        return {
          projection: Object.fromEntries(
            committed.learners.flatMap((sequence) => {
              if (sequence.views === undefined) return [];
              const { state, quizzes } = replay(committed, sequence);
              return [[sequence.id, { learner: learnerView(state, view) ?? null, runs: Object.fromEntries(sequence.views.runs.map((run) => [run, runView(state, run, quizzes) ?? null])) }]];
            }),
          ),
        };
      },
    },
  },
});
