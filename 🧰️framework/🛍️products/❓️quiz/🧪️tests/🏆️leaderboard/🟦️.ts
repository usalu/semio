/** 🏆️ Subject adapter of the leaderboard case: `@semio-tech/quiz` folds every committed stream and answers with its views.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/👁️views/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { type Catalog, type Event, type LearnerState, type Quiz, catalogView, emptyLearnerState, evolveLearner, leaderboard, learnerView } from "../../📦️packages/🟦️typescript/🟦️.ts";

const VECTORS = "shared://🏆️leaderboard/🔣️.json";

type Vectors = {
  readonly catalog: Catalog;
  readonly quizzes: readonly Quiz[];
  readonly catalogs: readonly { readonly id: string; readonly catalog: Catalog; readonly quizzes: readonly Quiz[] }[];
  readonly vectors: readonly { readonly id: string; readonly learners: readonly { readonly learner: string; readonly events: readonly Event[] }[] }[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
}

/** 🗂️ Every learner of a vector folded from its committed events. */
function folded(vector: Vectors["vectors"][number]): LearnerState[] {
  return vector.learners.map((learner) => learner.events.reduce(evolveLearner, emptyLearnerState(learner.learner)));
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "catalog-views": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).catalogs.map((vector) => [vector.id, catalogView(vector.catalog, vector.quizzes)])) }) },
    "learner-views": {
      subject: (ctx) => {
        const committed = vectors(ctx);
        const view = catalogView(committed.catalog, committed.quizzes);
        return { projection: Object.fromEntries(committed.vectors.map((vector) => [vector.id, Object.fromEntries(folded(vector).map((state) => [state.learner, learnerView(state, view) ?? null]))])) };
      },
    },
    rankings: {
      subject: (ctx) => {
        const committed = vectors(ctx);
        const view = catalogView(committed.catalog, committed.quizzes);
        return { projection: Object.fromEntries(committed.vectors.map((vector) => [vector.id, leaderboard(folded(vector), view)])) };
      },
    },
  },
});
