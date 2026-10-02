/** 🏅️ Subject adapter of the badge-rules case: `earnedBadges` of `@semio-tech/quiz` for every committed set of results.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🏅️badges/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDEA\uFE0Ftest/\uD83D\uDD0C\uFE0Fadapter/\uD83D\uDFE6\uFE0F.ts";
import { type Badge, type Quiz, type RunResult, earnedBadges } from "../../📦️packages/🟦️typescript/🟦️.ts";

const VECTORS = "shared://🏅️badge-rules/🔣️.json";

type Vectors = { readonly quizzes: readonly Quiz[]; readonly badges: readonly Badge[]; readonly vectors: readonly { readonly id: string; readonly results: readonly RunResult[]; readonly held: readonly string[] }[] };

/** 🗃️ The newly earned badges of every committed vector. */
function awards(ctx: AdapterContext): Record<string, string[]> {
  const committed = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
  return Object.fromEntries(committed.vectors.map((vector) => [vector.id, earnedBadges(committed.badges, committed.quizzes, vector.results, vector.held)]));
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: { awards: { subject: (ctx) => ({ projection: awards(ctx) }) } },
});
