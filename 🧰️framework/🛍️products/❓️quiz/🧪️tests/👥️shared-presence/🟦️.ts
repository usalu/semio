/** 👥️ Subject adapter of the shared-presence case: the scopes and the presence admission of `@semio-tech/quiz`.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/👥️presence/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { type CursorState, type Place, type PresenceState, type ThinkingState, cursorProblem, presenceProblem, roomScope, rosterScope, thinkingProblem, thinkingScope } from "../../📦️packages/🟦️typescript/🟦️.ts";

const VECTORS = "shared://👥️shared-presence/🔣️.json";

type Vectors = {
  readonly scopes: readonly { readonly id: string; readonly catalog: string; readonly place: Place }[];
  readonly presence: readonly { readonly id: string; readonly state: PresenceState }[];
  readonly cursors: readonly { readonly id: string; readonly state: CursorState }[];
  readonly thinkingScopes: readonly { readonly id: string; readonly catalog: string; readonly quiz: string }[];
  readonly thinking: readonly { readonly id: string; readonly state: ThinkingState }[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "room-scopes": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).scopes.map((vector) => [vector.id, { roster: rosterScope(vector.catalog), room: roomScope(vector.catalog, vector.place) ?? null }])) }) },
    "presence-states": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).presence.map((vector) => [vector.id, presenceProblem(vector.state) === undefined])) }) },
    "cursor-states": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).cursors.map((vector) => [vector.id, cursorProblem(vector.state) === undefined])) }) },
    "thinking-scopes": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).thinkingScopes.map((vector) => [vector.id, thinkingScope(vector.catalog, vector.quiz)])) }) },
    "thinking-states": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).thinking.map((vector) => [vector.id, thinkingProblem(vector.state) === undefined])) }) },
  },
});
