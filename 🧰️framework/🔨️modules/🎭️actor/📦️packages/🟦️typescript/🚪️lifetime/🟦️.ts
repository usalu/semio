/** 🚪️ Defining Actor ./lifetime public facet. */
export { ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES, encodeActorInstanceLifecycle, decodeActorInstanceLifecycle, actorInstanceLifetimeEquals, actorInstanceLifecycleReceiptEquals, actorInstanceCapturedReceiptMatches, actorInstanceCloseReceiptMatches } from "../../../🚪️lifetime/🟦️.ts";
export type { ActorInstanceLifetime, ActorInstanceOpenRequest, ActorInstanceCloseRequest, ActorInstanceLifecycleReceipt, ActorInstanceLifecycleAck, ActorInstanceLifecycleWire } from "../../../🚪️lifetime/🟦️.ts";
export { default as actorInstanceLifecycleSchema } from "../../../🚪️lifetime/🧬️schema/🔣️.json" with { type: "json" };
