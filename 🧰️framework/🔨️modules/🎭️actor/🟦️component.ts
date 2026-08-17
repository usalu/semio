/** 🎭️ TypeScript surface for `semio-framework-actor`: the ts-rs mirror of the pure kernel vocabulary
 * (`PackageId`/`ActorId`/`Lane`/`Budget`/`Envelope`/`TurnResult`/`FailureStage`/`ShardTable`/…).
 * Regenerate via `bun nx run @semio-tech/framework-actor-rs:typegen`.
 *
 * `ShardClient` (one `MessagePort` per shard, actor-id multiplexing) is packet H2's — it lives at
 * `📦️packages/🟦️typescript/🧵️shard-client.ts`, not mounted here yet. This file re-exports only the
 * generated type mirror.
 */
export * from "./🤖️generated/🟦️actor.js";
