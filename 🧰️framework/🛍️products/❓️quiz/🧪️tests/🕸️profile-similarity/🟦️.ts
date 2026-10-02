/** 🕸️ Subject adapter of the profile-similarity case: `scoreTask` of `@semio-tech/quiz` for every committed classification answer.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/📏️scoring/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDEA\uFE0Ftest/\uD83D\uDD0C\uFE0Fadapter/\uD83D\uDFE6\uFE0F.ts";
import { type Answer, type SheetTask, type Task, scoreTask } from "../../📦️packages/🟦️typescript/🟦️.ts";

const VECTORS = "shared://🕸️profile-similarity/🔣️.json";

type Degraded = { readonly id: string; readonly task: Task; readonly sheetTask: SheetTask; readonly answer: Answer };
type Vectors = { readonly tasks: readonly Task[]; readonly vectors: readonly { readonly id: string; readonly task: string; readonly sheetTask: SheetTask; readonly answer: Answer }[] };

/** 🗃️ Every committed answer credited against its task and sheet task. */
function credits(ctx: AdapterContext): Record<string, unknown> {
  const committed = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
  const tasks = new Map(committed.tasks.map((task) => [task.id, task]));
  return Object.fromEntries(committed.vectors.map((vector) => [vector.id, scoreTask(tasks.get(vector.task)!, vector.sheetTask, vector.answer)]));
}

/** 🩹️ Every committed input that bypasses validation, scored — `null` where the core scores none. */
function degraded(ctx: AdapterContext): Record<string, unknown> {
  const committed = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as { readonly degraded: readonly Degraded[] };
  return Object.fromEntries(committed.degraded.map((vector) => [vector.id, scoreTask(vector.task, vector.sheetTask, vector.answer) ?? null]));
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    credits: { subject: (ctx) => ({ projection: credits(ctx) }) },
    degraded: { subject: (ctx) => ({ projection: degraded(ctx) }) },
  },
});
