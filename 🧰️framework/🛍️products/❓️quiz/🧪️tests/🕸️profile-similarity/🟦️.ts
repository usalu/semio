/** 🕸️ Subject adapter of the profile-similarity case: `scoreTask` of `@semio-tech/quiz` for every committed classification answer.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/📏️scoring/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { type Answer, type SheetTask, type Task, scoreTask } from "../../📦️packages/🟦️typescript/🟦️.ts";

const VECTORS = "shared://🕸️profile-similarity/🔣️.json";

type Vector = { readonly id: string; readonly task: string; readonly sheetTask: SheetTask; readonly answer?: Answer };
type Degraded = { readonly id: string; readonly task: Task; readonly sheetTask: SheetTask; readonly answer?: Answer };
type Vectors = { readonly tasks: readonly Task[]; readonly vectors: readonly Vector[]; readonly timed: readonly Vector[]; readonly degraded: readonly Degraded[] };

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
}

/** 🗃️ Every committed answer of one vector group — or its absence on a timed sheet task — credited against its task and sheet task. */
function credited(ctx: AdapterContext, group: "vectors" | "timed"): Record<string, unknown> {
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
    credits: { subject: (ctx) => ({ projection: credited(ctx, "vectors") }) },
    timed: { subject: (ctx) => ({ projection: credited(ctx, "timed") }) },
    degraded: { subject: (ctx) => ({ projection: degraded(ctx) }) },
  },
});
