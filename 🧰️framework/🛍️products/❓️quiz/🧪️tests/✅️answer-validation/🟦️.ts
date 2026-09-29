/** ✅️ Subject adapter of the answer-validation case: `answerRejection` and `answerComplete` of `@semio-tech/quiz`.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/✅️validation/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { type Answer, type SheetTask, answerComplete, answerRejection } from "../../📦️packages/🟦️typescript/🟦️.ts";

const VECTORS = "shared://✅️answer-validation/🔣️.json";

type Vectors = { readonly sheetTasks: readonly SheetTask[]; readonly vectors: readonly { readonly id: string; readonly sheetTask: string; readonly answer?: Answer }[] };

/** ⚖️ The rejection, and completeness for an answer that is valid or absent. */
function verdict(sheetTask: SheetTask, answer: Answer | undefined): { rejection: string | null; complete?: boolean } {
  const rejection = answer === undefined ? undefined : answerRejection(sheetTask, answer);
  return rejection !== undefined ? { rejection } : { rejection: null, complete: answerComplete(sheetTask, answer) };
}

/** 🗃️ Every committed answer judged against its sheet task. */
function verdicts(ctx: AdapterContext): Record<string, unknown> {
  const committed = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
  const tasks = new Map(committed.sheetTasks.map((task) => [task.id, task]));
  return Object.fromEntries(committed.vectors.map((vector) => [vector.id, verdict(tasks.get(vector.sheetTask)!, vector.answer)]));
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: { verdicts: { subject: (ctx) => ({ projection: verdicts(ctx) }) } },
});
