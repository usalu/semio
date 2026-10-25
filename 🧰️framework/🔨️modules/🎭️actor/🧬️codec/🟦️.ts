//#region 🧬️ComponentCodecWire
/** 🧬️ One call of `world actor`'s `codec` interface (`🔌️plugin/🧬️schema/📜️.wit`) on an activated actor's
 * component instance: the kind fingerprint, the genesis of a document identity, or a pair's text mirror.
 * The worker answers {@link ActorCodecAnswer}.
 *
 * Language-agnostic owner: `🧬️schema/🔣️.json`
 * (`https://json.schemas.assets.semio-tech.com/framework/actor/codec/schema.json#/$defs/ActorCodecRequest`)
 * + `🧫️fixtures/🔣️.json`, which the defining codec suite admits against the schema; Product bridge laws evaluate the actual
 * emitted admission and codec arms while the narrow ShardClient receiver admits transport unknown.
 *
 * 🧾️ A zero-import leaf on purpose: `ShardClient` and the bridge generator (`🔌️plugin/🌐️browser-bundle/
 * 🏗️materialization/🟦️.ts`, which sits in the vite config's import graph) both read it. */
export type ActorCodecRequest =
  | { readonly operation: "pack-schema-hash"; readonly artifactKind: string }
  | { readonly operation: "genesis"; readonly artifactKind: string; readonly documentId: string }
  | { readonly operation: "print-mirror"; readonly artifactKind: string; readonly pair: ActorCodecDocumentPair };

/** 📦️ `codec::document-pair`: a document's snapshot pack and its history log. */
export type ActorCodecDocumentPair = { readonly pack: Uint8Array; readonly spr: Uint8Array };

/** 🎁️ The three successes defined by the codec answer schema. */
export type ActorCodecSuccess<Operation extends ActorCodecRequest["operation"] = ActorCodecRequest["operation"]> = Operation extends "pack-schema-hash" ? Uint8Array : Operation extends "genesis" ? ActorCodecDocumentPair : readonly [string, string];

/** 📨️ A {@link ActorCodecRequest}'s closed success or the original guest fault pack. */
export type ActorCodecAnswer<Operation extends ActorCodecRequest["operation"] = ActorCodecRequest["operation"]> = { readonly ok: ActorCodecSuccess<Operation> } | { readonly fault: Uint8Array };

/** 🌉️ The jco binding of the `codec` interface, as the bridge destructures it off the component. */
export type ActorCodecExports = {
  readonly packSchemaHash: (artifactKind: string) => Promise<Uint8Array>;
  readonly genesis: (artifactKind: string, documentId: string) => Promise<ActorCodecDocumentPair>;
  readonly printMirror: (artifactKind: string, pair: ActorCodecDocumentPair) => Promise<readonly [string, string]>;
};

/** 🚫️ The refusals of the codec lane, one code per violation (the fixture's `refusals`). */
export const ACTOR_CODEC_REFUSAL = Object.freeze({
  unexported: "actor-codec.unexported",
  operation: "actor-codec.operation",
  answer: "actor-codec.answer",
  request: "actor-codec.request",
});
//#endregion 🧬️ComponentCodecWire

//#region 🌉️BridgeArm
/** 🏭️ Keeps operation admission and guest answering in one self-contained lexical scope. */
function createActorCodecBoundaryV1() {
  const brand=Object.getOwnPropertyDescriptor(Object.getPrototypeOf(Uint8Array.prototype),Symbol.toStringTag)!.get!;
  const bytes = (value: unknown): value is Uint8Array => ArrayBuffer.isView(value) && brand.call(value) === "Uint8Array";
  const fields = (value: unknown, names: readonly string[]): value is Record<string, unknown> => value !== null && typeof value === "object" && !Array.isArray(value) && !ArrayBuffer.isView(value) && Reflect.ownKeys(value).length === names.length && names.every(name => { const descriptor = Object.getOwnPropertyDescriptor(value, name); return descriptor !== undefined && "value" in descriptor; });
  const mirror = (value: unknown): boolean => Array.isArray(value) && value.length === 2 && Reflect.ownKeys(value).length === 3 && ["0", "1"].every(name => { const descriptor = Object.getOwnPropertyDescriptor(value, name); return descriptor !== undefined && "value" in descriptor && typeof descriptor.value === "string"; });
  function admitActorCodecRequestV1<Request extends ActorCodecRequest>(value:Request):Request;
  function admitActorCodecRequestV1(value:unknown):ActorCodecRequest;
  function admitActorCodecRequestV1(value:unknown):ActorCodecRequest {
    const descriptor=value!==null&&typeof value==="object"?Object.getOwnPropertyDescriptor(value,"operation"):undefined;
    if(!descriptor||!("value"in descriptor)||typeof descriptor.value!=="string")throw new Error("actor-codec.request");
    const operation=descriptor.value;
    if(operation!=="pack-schema-hash"&&operation!=="genesis"&&operation!=="print-mirror")throw new Error("actor-codec.operation:"+operation);
    const names=operation==="pack-schema-hash"?["operation","artifactKind"]:operation==="genesis"?["operation","artifactKind","documentId"]:["operation","artifactKind","pair"];
    if(fields(value,names)&&typeof value.artifactKind==="string"&&value.artifactKind.length>0&&(operation==="pack-schema-hash"||operation==="genesis"&&typeof value.documentId==="string"&&value.documentId.length>0||operation==="print-mirror"&&fields(value.pair,["pack","spr"])&&bytes(value.pair.pack)&&bytes(value.pair.spr)))return value as ActorCodecRequest;
    throw new Error("actor-codec.request");
  }
  function admitActorCodecAnswerV1<Operation extends ActorCodecRequest["operation"]>(operation: Operation, value: unknown): ActorCodecAnswer<Operation> {
    if (operation !== "pack-schema-hash" && operation !== "genesis" && operation !== "print-mirror") throw new Error("actor-codec.operation"+(typeof operation==="string"?":"+operation:""));
    if (fields(value, ["fault"]) && bytes(value.fault)) return value as ActorCodecAnswer<Operation>;
    if (fields(value, ["ok"])) {
      const success = value.ok;
      if (operation === "pack-schema-hash" && bytes(success) || operation === "genesis" && fields(success, ["pack", "spr"]) && bytes(success.pack) && bytes(success.spr) || operation === "print-mirror" && mirror(success)) return value as ActorCodecAnswer<Operation>;
    }
    throw new Error(`actor-codec.answer:${operation}`);
  }

  async function actorCodecAnswer<Request extends ActorCodecRequest>(codec: ActorCodecExports | undefined, request: Request): Promise<ActorCodecAnswer<Request["operation"]>> {
    admitActorCodecRequestV1(request);
    const operation=request.operation,artifactKind=request.artifactKind;
    if (!codec) throw new Error("actor-codec.unexported");
    try {
      let success: unknown;
      if (operation === "pack-schema-hash") { if (typeof codec.packSchemaHash !== "function") throw new Error("actor-codec.unexported"); success = await codec.packSchemaHash(artifactKind); }
      else if (operation === "genesis") { const documentId=request.documentId; if (typeof codec.genesis !== "function") throw new Error("actor-codec.unexported"); success = await codec.genesis(artifactKind, documentId); }
      else if (operation === "print-mirror") { const pair=request.pair; if (typeof codec.printMirror !== "function") throw new Error("actor-codec.unexported"); success = await codec.printMirror(artifactKind, pair); }
      else throw new Error("actor-codec.operation");
      return admitActorCodecAnswerV1<Request["operation"]>(operation, { ok: success });
    } catch (error) {
      const payload = error !== null && typeof error === "object" && "payload" in error ? (error as { readonly payload?: { readonly tag?: unknown; readonly val?: unknown } }).payload : undefined;
      if (payload?.tag === "fault") { try { return admitActorCodecAnswerV1<Request["operation"]>(operation, { fault: payload.val }); } catch { throw error; } }
      throw error;
    }
  }
  return {admitActorCodecRequestV1,admitActorCodecAnswerV1,actorCodecAnswer};
}

/** 🛂️ Defining operation-correlated admission and guest answering preserve every original result owner. */
export const {admitActorCodecRequestV1,admitActorCodecAnswerV1,actorCodecAnswer}=createActorCodecBoundaryV1();

/** 🧵️ Emits the complete defining lexical scope so compiler identifier changes cannot split codec linkage. */
export function actorCodecRuntimeSourceV1():string {
  return "const {admitActorCodecRequestV1,admitActorCodecAnswerV1,actorCodecAnswer:codecAnswer}=("+createActorCodecBoundaryV1.toString()+")();";
}

//#endregion 🌉️BridgeArm
