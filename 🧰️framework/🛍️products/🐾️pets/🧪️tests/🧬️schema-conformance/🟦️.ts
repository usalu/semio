/** 🧬️ Subject adapter of the schema-conformance case: `menagerieIssues`, `speciesIssues` and `ensembleIssues` of `@semio-tech/pets` judge every committed document.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/✅️validation/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDEA\uFE0Ftest/\uD83D\uDD0C\uFE0Fadapter/\uD83D\uDFE6\uFE0F.ts";
import { ensembleIssues, menagerieIssues, speciesIssues, type Issue } from "../../🔨️modules/✅️validation/🟦️.ts";

const VECTORS = "shared://🧬️schema-conformance/🔣️.json";

type Vector = { readonly id: string; readonly definition: string; readonly document?: unknown; readonly pointer?: string };
type Vectors = { readonly accepted: readonly Vector[]; readonly structural: readonly Vector[]; readonly rules: readonly Vector[] };

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
}

/** 🔎️ The document of a vector: inline, or the value its JSON pointer reaches in the vectors. */
function documentOf(committed: Vectors, vector: Vector): unknown {
  if (vector.pointer === undefined) return vector.document;
  return vector.pointer
    .split("/")
    .slice(1)
    .reduce<unknown>((value, part) => (value as Record<string, unknown>)[part.replaceAll("~1", "/").replaceAll("~0", "~")], committed);
}

/** ⚖️ The findings of the owned validator of the definition a vector names. */
function issuesOf(definition: string, document: unknown): Issue[] {
  return definition === "Species" ? speciesIssues(document) : definition === "Ensemble" ? ensembleIssues(document) : menagerieIssues(document);
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "accepted-documents": {
      subject: (ctx) => {
        const committed = vectors(ctx);
        return { projection: Object.fromEntries(committed.accepted.map((vector) => [vector.id, issuesOf(vector.definition, documentOf(committed, vector)).length === 0])) };
      },
    },
    "structural-rejections": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).structural.map((vector) => [vector.id, issuesOf(vector.definition, vector.document).length === 0])) }) },
    "rule-violations": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).rules.map((vector) => [vector.id, issuesOf(vector.definition, vector.document)])) }) },
  },
});
