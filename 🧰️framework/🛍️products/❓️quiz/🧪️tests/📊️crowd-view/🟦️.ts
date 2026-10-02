/** 📊️ Subject adapter of the crowd-view case: `crowdView` of `@semio-tech/quiz` for every committed quiz and set of results.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/👁️views/🟦️.ts
 */
/** 📊️ Subject adapter of the crowd-view case: `crowdView` of `@semio-tech/quiz` for every committed quiz and set of results.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/👁️views/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDEA\uFE0Ftest/\uD83D\uDD0C\uFE0Fadapter/\uD83D\uDFE6\uFE0F.ts";
import { type Quiz, type RunResult, crowdView } from "../../📦️packages/🟦️typescript/🟦️.ts";

const VECTORS = "shared://📊️crowd-view/🔣️.json";

type Vectors = { readonly quizzes: readonly Quiz[]; readonly vectors: readonly { readonly id: string; readonly quiz: string; readonly results: readonly RunResult[] }[] };

/** 🗃️ The crowd view of every committed vector. */
function crowds(ctx: AdapterContext): Record<string, unknown> {
  const committed = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
  const quizzes = new Map(committed.quizzes.map((quiz) => [quiz.id, quiz]));
  return Object.fromEntries(committed.vectors.map((vector) => [vector.id, crowdView(quizzes.get(vector.quiz)!, vector.results)]));
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: { crowds: { subject: (ctx) => ({ projection: crowds(ctx) }) } },
});
