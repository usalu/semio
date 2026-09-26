//#region 🧬️ComponentCodecWire
/** 🧬️ One call of `world actor`'s `codec` interface (`🔌️plugin/🧬️schema/📜️.wit`) on an activated actor's
 * component instance: the kind fingerprint, the genesis of a document identity, or a pair's text mirror.
 * The worker answers {@link ShardCodecAnswer}.
 *
 * Language-agnostic owner: `🧬️schema/🔣️.json`
 * (`https://json.schemas.assets.semio-tech.com/framework/actor/shard-client/component-codec/schema.json#/$defs/ShardCodecRequest`)
 * + `🧫️fixtures/🔣️.json`, which the shard-client suite drives through the generated worker AND the generated
 * bridge, so the two generated halves and this declaration cannot drift.
 *
 * 🧾️ A zero-import leaf on purpose: `ShardClient` and the bridge generator (`🔌️plugin/🌐️browser-bundle/
 * 🏗️materialization/🟦️.ts`, which sits in the vite config's import graph) both read it. */
export type ShardCodecRequest =
  | { readonly operation: "pack-schema-hash"; readonly artifactKind: string }
  | { readonly operation: "genesis"; readonly artifactKind: string; readonly documentId: string }
  | { readonly operation: "print-mirror"; readonly artifactKind: string; readonly pair: ShardCodecDocumentPair };

/** 📦️ `codec::document-pair`: a document's snapshot pack and its history log. */
export type ShardCodecDocumentPair = { readonly pack: Uint8Array; readonly spr: Uint8Array };

/** 🧬️ A {@link ShardCodecRequest}'s answer: the export's value, or the guest's `plugin-error` fault pack. */
export type ShardCodecAnswer = { readonly ok: unknown } | { readonly fault: Uint8Array };

/** 🧬️ The jco binding of the `codec` interface, as the bridge destructures it off the component. */
export type ActorCodecExports = {
  readonly packSchemaHash: (artifactKind: string) => Promise<Uint8Array>;
  readonly genesis: (artifactKind: string, documentId: string) => Promise<ShardCodecDocumentPair>;
  readonly printMirror: (artifactKind: string, pair: ShardCodecDocumentPair) => Promise<readonly [string, string]>;
};

/** 🚫️ The refusals of the codec lane, one code per violation (the fixture's `refusals`). */
export const ACTOR_CODEC_REFUSAL = Object.freeze({
  unexported: "actor-codec.unexported",
  operation: "actor-codec.operation",
});
//#endregion 🧬️ComponentCodecWire

//#region 🌉️BridgeArm
/**
 * @emoji 🌉️ Answers one {@link ShardCodecRequest} from the component's `codec` exports — inlined by
 * `pluginComponentBridgeSource` (via `toString`, so it stays a self-contained function) as the generated
 * bridge's `codec` arm. The guest's own `plugin-error` — jco throws it as a `ComponentError` whose
 * `payload` is `{ tag: "fault", val: <fault pack> }` — is answered as `{ fault }` for the host to decode,
 * never as a thrown `[object Object]`; a component that exports no `codec` refuses as
 * `actor-codec.unexported`. A codec call enters the same component instance a turn does, so
 * `🟨️shard-worker.js` refuses it while a turn of that actor is in flight and the host serializes both on
 * the actor's chain.
 * @see https://github.com/bytecodealliance/jco
 */
export async function actorCodecAnswer(codec: ActorCodecExports | undefined, request: ShardCodecRequest): Promise<ShardCodecAnswer> {
  if (!codec) throw new Error("actor-codec.unexported");
  try {
    if (request.operation === "pack-schema-hash") return { ok: await codec.packSchemaHash(request.artifactKind) };
    if (request.operation === "genesis") return { ok: await codec.genesis(request.artifactKind, request.documentId) };
    if (request.operation === "print-mirror") return { ok: await codec.printMirror(request.artifactKind, request.pair) };
  } catch (error) {
    const payload = error !== null && typeof error === "object" && "payload" in error ? (error as { readonly payload?: { readonly tag?: unknown; readonly val?: unknown } }).payload : undefined;
    if (payload?.tag === "fault" && payload.val instanceof Uint8Array) return { fault: payload.val };
    throw error;
  }
  throw new Error(`actor-codec.operation:${String((request as { readonly operation?: unknown }).operation)}`);
}
//#endregion 🌉️BridgeArm
