/** 📏️ Subject adapter of the sorting-concordance case: `scoreTask` of `@semio-tech/quiz` for every committed sorting answer.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/📏️scoring/🟦️.ts
 */
/** 📏️ Subject adapter of the sorting-concordance case: `scoreTask` of `@semio-tech/quiz` for every committed sorting answer.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/📏️scoring/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDEA\uFE0Ftest/\uD83D\uDD0C\uFE0Fadapter/\uD83D\uDFE6\uFE0F.ts";
import { type Answer, type SheetTask, type Task, scoreTask } from "../../📦️packages/🟦️typescript/🟦️.ts";

const VECTORS = "shared://📏️sorting-concordance/🔣️.json";

type Vector = { readonly id: string; readonly task: string; readonly sheetTask: SheetTask; readonly answer: Answer };
type Degraded = { readonly id: string; readonly task: Task; readonly sheetTask: SheetTask; readonly answer: Answer };
type Vectors = { readonly tasks: readonly Task[]; readonly vectors: readonly Vector[]; readonly rankVectors: readonly Vector[] };

/** 🗃️ Every committed answer of one vector group scored against its task and sheet task. */
function scored(ctx: AdapterContext, group: "vectors" | "rankVectors"): Record<string, unknown> {
  const committed = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
  const tasks = new Map(committed.tasks.map((task) => [task.id, task]));
  return Object.fromEntries(committed[group].map((vector) => [vector.id, scoreTask(tasks.get(vector.task)!, vector.sheetTask, vector.answer)]));
}

/** 🩹️ Every committed input that bypasses validation, scored — `null` where the core scores none. */
function degraded(ctx: AdapterContext): Record<string, unknown> {
  const committed = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as { readonly degraded: readonly Degraded[] };
  return Object.fromEntries(committed.degraded.map((vector) => [vector.id, scoreTask(vector.task, vector.sheetTask, vector.answer) ?? null]));
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    scores: { subject: (ctx) => ({ projection: scored(ctx, "vectors") }) },
    "rank-weights": { subject: (ctx) => ({ projection: scored(ctx, "rankVectors") }) },
    degraded: { subject: (ctx) => ({ projection: degraded(ctx) }) },
  },
});
