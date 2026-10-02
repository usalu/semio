/** 🃏️ Subject adapter of the sheet-assembly case: `sheetOf` of `@semio-tech/quiz` for every committed quiz and seed.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🃏️sheet/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDEA\uFE0Ftest/\uD83D\uDD0C\uFE0Fadapter/\uD83D\uDFE6\uFE0F.ts";
import { type Quiz, sheetOf } from "../../📦️packages/🟦️typescript/🟦️.ts";

const VECTORS = "shared://🃏️sheet-assembly/🔣️.json";

type Vectors = { readonly quizzes: readonly Quiz[]; readonly sheets: readonly { readonly id: string; readonly quiz: string; readonly seed: number }[] };

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    sheets: {
      subject: (ctx) => {
        const committed = vectors(ctx);
        const quizzes = new Map(committed.quizzes.map((quiz) => [quiz.id, quiz]));
        return { projection: Object.fromEntries(committed.sheets.map((vector) => [vector.id, sheetOf(quizzes.get(vector.quiz)!, vector.seed)])) };
      },
    },
  },
});
