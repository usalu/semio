/** 🧾️ Subject adapter of the learner-lifecycle case: `@semio-tech/quiz` replays every committed sequence through its deciders and plays the site catalog.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🧾️lifecycle/🟦️.ts
 */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { type AdapterContext, defineTestAdapter } from "../../../../\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDEA\uFE0Ftest/\uD83D\uDD0C\uFE0Fadapter/\uD83D\uDFE6\uFE0F.ts";
import {
  type Answer,
  type Catalog,
  type Command,
  type Decision,
  type Event,
  type IdentifyLearnerCommand,
  type LearnerState,
  type Limits,
  type LoadedQuiz,
  type Quiz,
  type SheetTask,
  type Task,
  DEFAULT_LIMITS,
  catalogView,
  decideHandle,
  decideLearner,
  emptyHandleState,
  emptyLearnerState,
  evolveHandle,
  evolveLearner,
  learnerView,
  registrationRejection,
  runSeed,
  runView,
  sheetOf,
} from "../../📦️packages/🟦️typescript/🟦️.ts";

const VECTORS = "shared://🧾️learner-lifecycle/🔣️.json";

type Step<C> = { readonly command: C; readonly now: number; readonly revisions?: Readonly<Record<string, string>> };
type Sequence = { readonly id: string; readonly learner: string; readonly given: readonly Event[]; readonly limits?: Limits; readonly steps: readonly Step<Command>[]; readonly views?: { readonly runs: readonly string[] } };
type Play = { readonly id: string; readonly learner: string; readonly runs: readonly { readonly run: string; readonly quiz: string; readonly flaw?: string }[] };
type Vectors = {
  readonly catalog: Catalog;
  readonly quizzes: readonly Quiz[];
  readonly revisions: Readonly<Record<string, string>>;
  readonly limits: Limits;
  readonly registrations: readonly { readonly id: string; readonly key: string; readonly steps: readonly Step<IdentifyLearnerCommand>[] }[];
  readonly quotas: readonly { readonly id: string; readonly learners: number; readonly limits: Limits }[];
  readonly learners: readonly Sequence[];
  readonly malformed: { readonly registrations: Vectors["registrations"]; readonly learners: readonly Sequence[] };
  readonly site: { readonly catalog: string; readonly plays: readonly Play[] };
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
}

/** 📚️ The loaded quizzes at the given revisions. */
function loaded(committed: Vectors, revisions: Readonly<Record<string, string>>): Record<string, LoadedQuiz> {
  return Object.fromEntries(committed.quizzes.map((quiz) => [quiz.id, { quiz, revision: revisions[quiz.id]! }]));
}

/** 🎞️ Folds the given events, then decides and folds every step under the sequence's caps. */
function replay(committed: Vectors, sequence: Sequence): { decisions: Decision[]; state: LearnerState; quizzes: Record<string, LoadedQuiz> } {
  let state = sequence.given.reduce(evolveLearner, emptyLearnerState(sequence.learner));
  let revisions = { ...committed.revisions };
  const decisions: Decision[] = [];
  for (const step of sequence.steps) {
    revisions = { ...revisions, ...step.revisions };
    const decision = decideLearner(state, step.command, { now: step.now, catalog: committed.catalog, quizzes: loaded(committed, revisions), limits: sequence.limits ?? committed.limits });
    decisions.push(decision);
    if ("events" in decision) state = decision.events.reduce(evolveLearner, state);
  }
  return { decisions, state, quizzes: loaded(committed, revisions) };
}

/** 💯️ The answer that scores a sheet task 1: every item in its category, ascending by value (ties in definition order), every item on a card of its own value. */
function perfectAnswer(task: Task, sheetTask: SheetTask): Answer {
  const presented = sheetTask.items.map((item) => item.id);
  if (task.kind === "classification") return { kind: "classification", assignments: Object.fromEntries(presented.map((id) => [id, task.items.find((item) => item.id === id)!.category])) };
  if (task.kind === "sorting") {
    const position = (id: string): number => task.items.findIndex((item) => item.id === id);
    return { kind: "sorting", order: [...presented].sort((left, right) => task.items[position(left)]!.value - task.items[position(right)]!.value || position(left) - position(right)) };
  }
  if (sheetTask.kind !== "matching") throw new Error(`${task.id} is not presented as a matching task`);
  const assignments: Record<string, Record<string, number>> = {};
  for (const dimension of sheetTask.dimensions) {
    const free = dimension.cards.map((_, card) => card);
    assignments[dimension.id] = {};
    for (const id of presented) {
      const value = task.items.find((item) => item.id === id)!.values[dimension.id];
      const card = free.splice(free.findIndex((candidate) => dimension.cards[candidate] === value), 1)[0]!;
      assignments[dimension.id]![id] = card;
    }
  }
  return { kind: "matching", assignments };
}

/** 🩹️ The perfect answer with exactly one mistake: the first item in the first wrong category, the smallest and the largest item exchanged, or the cards of the first item and of the first later item of another value exchanged in the first dimension. */
function flawedAnswer(task: Task, sheetTask: SheetTask): Answer {
  const answer = perfectAnswer(task, sheetTask);
  const presented = sheetTask.items.map((item) => item.id);
  const first = presented[0]!;
  if (task.kind === "classification" && sheetTask.kind === "classification" && answer.kind === "classification") {
    const correct = task.items.find((item) => item.id === first)!.category;
    return { ...answer, assignments: { ...answer.assignments, [first]: sheetTask.categories.find((category) => category.id !== correct)!.id } };
  }
  if (answer.kind === "sorting") return { ...answer, order: [answer.order[answer.order.length - 1]!, ...answer.order.slice(1, -1), answer.order[0]!] };
  if (task.kind !== "matching" || sheetTask.kind !== "matching" || answer.kind !== "matching") throw new Error(`${task.id} is not presented as its own kind`);
  const dimension = sheetTask.dimensions[0]!.id;
  const value = (id: string): number => task.items.find((item) => item.id === id)!.values[dimension]!;
  const other = presented.slice(1).find((id) => value(id) !== value(first))!;
  const cards = answer.assignments[dimension]!;
  return { ...answer, assignments: { ...answer.assignments, [dimension]: { ...cards, [first]: cards[other]!, [other]: cards[first]! } } };
}

/** 🎓️ The catalog at a repository path and its quizzes in catalog order. */
function siteQuizzes(repoRoot: string, path: string): { catalog: Catalog; quizzes: Quiz[] } {
  const catalog = JSON.parse(readFileSync(join(repoRoot, path), "utf8")) as Catalog;
  return { catalog, quizzes: catalog.quizzes.map((quiz) => JSON.parse(readFileSync(join(repoRoot, dirname(path), quiz), "utf8")) as Quiz) };
}

/** 🎮️ One registered learner playing the committed runs: per run its score and the badges it awards, and every badge held at the end. */
function play(catalog: Catalog, quizzes: readonly Quiz[], scenario: Play): { scores: number[]; awards: string[][]; held: string[] } {
  const context = { catalog, quizzes: Object.fromEntries(quizzes.map((quiz) => [quiz.id, { quiz, revision: "0".repeat(64) }])), limits: DEFAULT_LIMITS };
  let state = evolveLearner(emptyLearnerState(scenario.learner), { type: "learner-registered", learner: scenario.learner, identity: { kind: "anonymous" }, at: 0 });
  let now = 0;
  const decided = (command: Command): readonly Event[] => {
    now += 1;
    const decision = decideLearner(state, command, { ...context, now });
    if (!("events" in decision)) throw new Error(`site/${scenario.id}: ${command.type} is refused with ${decision.rejection}`);
    state = decision.events.reduce(evolveLearner, state);
    return decision.events;
  };
  const scores: number[] = [];
  const awards: string[][] = [];
  scenario.runs.forEach((run, number) => {
    const quiz = quizzes.find((candidate) => candidate.id === run.quiz)!;
    const id = (): string => ((BigInt(number) << 64n) + BigInt(now + 1)).toString(16).padStart(32, "0");
    decided({ type: "start-run", id: id(), learner: scenario.learner, run: run.run, quiz: run.quiz });
    for (const sheetTask of sheetOf(quiz, runSeed(run.run)).tasks) {
      const task = quiz.tasks.find((candidate) => candidate.id === sheetTask.id)!;
      decided({ type: "record-answer", id: id(), learner: scenario.learner, run: run.run, task: sheetTask.id, answer: (sheetTask.id === run.flaw ? flawedAnswer : perfectAnswer)(task, sheetTask) });
    }
    const events = decided({ type: "submit-run", id: id(), learner: scenario.learner, run: run.run });
    scores.push(events.flatMap((event) => (event.type === "run-submitted" ? [event.result.score] : []))[0]!);
    awards.push(events.flatMap((event) => (event.type === "badge-awarded" ? [event.badge] : [])));
  });
  return { scores, awards, held: state.badges.map((award) => award.badge) };
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    registrations: {
      subject: (ctx) => {
        const committed = vectors(ctx);
        const handles = Object.fromEntries(
          [...committed.registrations, ...committed.malformed.registrations].map((sequence) => {
            let state = emptyHandleState(sequence.key);
            return [
              sequence.id,
              sequence.steps.map((step) => {
                const decision = decideHandle(state, step.command, step.now);
                if ("events" in decision) state = decision.events.reduce(evolveHandle, state);
                return decision;
              }),
            ];
          }),
        );
        return { projection: { handles, quotas: Object.fromEntries(committed.quotas.map((vector) => [vector.id, registrationRejection(vector.learners, vector.limits) ?? null])) } };
      },
    },
    "learner-decisions": {
      subject: (ctx) => {
        const committed = vectors(ctx);
        return { projection: Object.fromEntries([...committed.learners, ...committed.malformed.learners].map((sequence) => [sequence.id, replay(committed, sequence).decisions])) };
      },
    },
    "learner-views": {
      subject: (ctx) => {
        const committed = vectors(ctx);
        const view = catalogView(committed.catalog, committed.quizzes);
        return {
          projection: Object.fromEntries(
            [...committed.learners, ...committed.malformed.learners].flatMap((sequence) => {
              if (sequence.views === undefined) return [];
              const { state, quizzes } = replay(committed, sequence);
              return [[sequence.id, { learner: learnerView(state, view) ?? null, runs: Object.fromEntries(sequence.views.runs.map((run) => [run, runView(state, run, quizzes) ?? null])) }]];
            }),
          ),
        };
      },
    },
    "site-catalog": {
      subject: (ctx) => {
        const site = vectors(ctx).site;
        const { catalog, quizzes } = siteQuizzes(ctx.repoRoot, site.catalog);
        return { projection: Object.fromEntries(site.plays.map((scenario) => [scenario.id, play(catalog, quizzes, scenario)])) };
      },
    },
  },
});
