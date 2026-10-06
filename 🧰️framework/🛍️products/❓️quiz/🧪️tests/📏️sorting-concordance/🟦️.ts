/** 📏️ Subject adapter of the sorting-concordance case: `scoreTask` of `@semio-tech/quiz` for every committed sorting answer.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/📏️scoring/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { type Answer, type SheetTask, type Task, scoreTask } from "../../📦️packages/🟦️typescript/🟦️.ts";

const VECTORS = "shared://📏️sorting-concordance/🔣️.json";

type Vector = { readonly id: string; readonly task: string; readonly sheetTask: SheetTask; readonly answer?: Answer };
type Degraded = { readonly id: string; readonly task: Task; readonly sheetTask: SheetTask; readonly answer?: Answer };
type Vectors = { readonly tasks: readonly Task[]; readonly vectors: readonly Vector[]; readonly rankVectors: readonly Vector[]; readonly guessed: readonly Vector[]; readonly degraded: readonly Degraded[] };

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.inputBytes(VECTORS))) as Vectors;
}

/** 🗃️ Every committed answer of one vector group — or its absence on a timed sheet task — scored against its task and sheet task. */
function scored(ctx: AdapterContext, group: "vectors" | "rankVectors" | "guessed"): Record<string, unknown> {
  const committed = vectors(ctx);
  const tasks = new Map(committed.tasks.map((task) => [task.id, task]));
  return Object.fromEntries(committed[group].map((vector) => [vector.id, scoreTask(tasks.get(vector.task)!, vector.sheetTask, vector.answer) ?? null]));
}

/** 🩹️ Every committed input that bypasses validation, scored — `null` where the core scores none. */
function degraded(ctx: AdapterContext): Record<string, unknown> {
  return Object.fromEntries(vectors(ctx).degraded.map((vector) => [vector.id, scoreTask(vector.task, vector.sheetTask, vector.answer) ?? null]));
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    scores: { subject: (ctx) => ({ projection: scored(ctx, "vectors") }) },
    "rank-weights": { subject: (ctx) => ({ projection: scored(ctx, "rankVectors") }) },
    guessed: { subject: (ctx) => ({ projection: scored(ctx, "guessed") }) },
    degraded: { subject: (ctx) => ({ projection: degraded(ctx) }) },
  },
});
