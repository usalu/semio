/** 🪪️ Subject adapter of the identity-shapes case: the handle policy and the id shapes of `@semio-tech/quiz`.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/✅️validation/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { type Command, type Query, commandRejection, handleActorId, normalizeHandle, queryRejection } from "../../📦️packages/🟦️typescript/🟦️.ts";

const VECTORS = "shared://🪪️identity-shapes/🔣️.json";

type Vectors = {
  readonly handles: readonly { readonly id: string; readonly handle: string }[];
  readonly shapes: readonly { readonly id: string; readonly definition: "Command" | "Query"; readonly document: Command | Query }[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.inputBytes(VECTORS))) as Vectors;
}

/** 📏️ Ascending code points as `XXXX` or `XXXX-YYYY` words. */
function ranges(points: readonly number[]): string {
  const found: [number, number][] = [];
  for (const point of points) {
    const last = found[found.length - 1];
    if (last && last[1] === point - 1) last[1] = point;
    else found.push([point, point]);
  }
  const hex = (point: number): string => point.toString(16).toUpperCase().padStart(4, "0");
  return found.map(([low, high]) => (low === high ? hex(low) : `${hex(low)}-${hex(high)}`)).join(" ");
}

/** 🔡️ Every Unicode scalar value `normalizeHandle` keeps unchanged between two letters, and the lowercase of every such character that changes. */
function alphabet(): { members: string; folds: Record<string, string> } {
  const members: number[] = [];
  const folds: Record<string, string> = {};
  for (let point = 0; point < 0x110000; point++) {
    if (point >= 0xd800 && point <= 0xdfff) continue;
    const character = String.fromCodePoint(point);
    if (normalizeHandle(`a${character}a`)?.display !== `a${character}a`) continue;
    members.push(point);
    const key = normalizeHandle(`a${character}a`)!.key.slice(1, -1);
    if (key !== character) folds[point.toString(16).toUpperCase().padStart(4, "0")] = key;
  }
  return { members: ranges(members), folds };
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    handles: {
      subject: (ctx) => ({
        projection: Object.fromEntries(
          vectors(ctx).handles.map((vector) => {
            const normalized = normalizeHandle(vector.handle);
            return [vector.id, normalized ? { ...normalized, actor: handleActorId(normalized.key) } : null];
          }),
        ),
      }),
    },
    alphabet: { subject: () => ({ projection: { alphabet: alphabet() } }) },
    shapes: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).shapes.map((vector) => [vector.id, (vector.definition === "Command" ? commandRejection(vector.document as Command) : queryRejection(vector.document as Query)) ?? null])) }) },
  },
});
