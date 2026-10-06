/** ⛰️ Subject adapter of the challenge-rules case: the rule table, `points`, `reach`, `misses`, `taskSeconds`, `acted` and `hintsOf` of `@semio-tech/quiz` — the hints over three vector groups (compare hints, classification hints, hints on the authored quizzes).
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/⛰️challenge/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { type Answer, type Challenge, type Scale, type SheetTask, type Task, type TaskKind, acted, challengeMeets, challengeRank, challengeRules, hintsOf, misses, points, reach, taskSeconds } from "../../📦️packages/🟦️typescript/🟦️.ts";

const VECTORS = "shared://⛰️challenge-rules/🔣️.json";

type Vectors = {
  readonly rules: { readonly challenges: readonly Challenge[] };
  readonly points: readonly { readonly id: string; readonly score: number; readonly challenge: Challenge }[];
  readonly reaches: readonly { readonly id: string; readonly values: readonly number[]; readonly scale: Scale }[];
  readonly misses: readonly { readonly id: string; readonly values: readonly number[]; readonly scale: Scale; readonly truth: number; readonly value: number }[];
  readonly seconds: readonly { readonly id: string; readonly kind: TaskKind; readonly items: number; readonly dimensions: number }[];
  readonly instants: readonly { readonly id: string; readonly at: number; readonly floor: number; readonly now: number }[];
  readonly tasks: readonly Task[];
} & { readonly [group in HintGroup]: readonly HintVector[] };

type HintGroup = "hints" | "classificationHints" | "quizHints";
type HintVector = { readonly id: string; readonly task: string; readonly sheetTask: SheetTask; readonly answer?: Answer };

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.inputBytes(VECTORS))) as Vectors;
}

/** 💡️ The hints of every committed task, sheet task and answer of one vector group. */
function hinted(ctx: AdapterContext, group: HintGroup) {
  const committed = vectors(ctx);
  const tasks = new Map(committed.tasks.map((task) => [task.id, task]));
  return { projection: Object.fromEntries(committed[group].map((vector) => [vector.id, hintsOf(tasks.get(vector.task)!, vector.sheetTask, vector.answer)])) };
}

/** ♾️ A reach as the vectors carry it: `null` where it is unbounded. */
function bounded(found: number): number | null {
  return Number.isFinite(found) ? found : null;
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    rules: {
      subject: (ctx) => {
        const committed = vectors(ctx);
        const challenges = committed.rules.challenges;
        return {
          projection: {
            table: Object.fromEntries(challenges.map((challenge) => [challenge, challengeRules(challenge)])),
            ranks: Object.fromEntries(challenges.map((challenge) => [challenge, challengeRank(challenge)])),
            meets: Object.fromEntries(challenges.map((challenge) => [challenge, Object.fromEntries(challenges.map((least) => [least, challengeMeets(challenge, least)]))])),
            points: Object.fromEntries(committed.points.map((vector) => [vector.id, points(vector.score, vector.challenge)])),
          },
        };
      },
    },
    reach: {
      subject: (ctx) => {
        const committed = vectors(ctx);
        return {
          projection: {
            reaches: Object.fromEntries(committed.reaches.map((vector) => [vector.id, bounded(reach(vector.values, vector.scale))])),
            misses: Object.fromEntries(
              committed.misses.map((vector) => {
                const found = reach(vector.values, vector.scale);
                return [vector.id, { reach: bounded(found), miss: misses(vector.value, vector.truth, vector.scale, found) }];
              }),
            ),
          },
        };
      },
    },
    clock: {
      subject: (ctx) => {
        const committed = vectors(ctx);
        return {
          projection: {
            seconds: Object.fromEntries(committed.seconds.map((vector) => [vector.id, taskSeconds(vector.kind, vector.items, vector.dimensions)])),
            instants: Object.fromEntries(committed.instants.map((vector) => [vector.id, acted(vector.at, vector.floor, vector.now)])),
          },
        };
      },
    },
    hints: { subject: (ctx) => hinted(ctx, "hints") },
    "classification-hints": { subject: (ctx) => hinted(ctx, "classificationHints") },
    "quiz-hints": { subject: (ctx) => hinted(ctx, "quizHints") },
  },
});
