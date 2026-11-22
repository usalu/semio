import {DESCRIPTOR_EMITTER_BUILD_TARGET_V1} from "../🌡️freshness/🟦️.ts";

/** 🔐️ Names the lease that lets one of several concurrent component builds republish the emitter; a `tools:` singleton, taken before any store, so it never waits under a higher rank.
 * @see ../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🔣️policy.json */
export function descriptorEmitterPrerequisiteResourceV1(intent:Readonly<{mode:string;platform:string}>):string{return `tools:descriptor-emitter:${intent.mode}:${intent.platform}`;}

export type DescriptorEmitterStateV1=Readonly<{kind:"fresh"}|{kind:"missing"}|{kind:"stale";changed:number;first:string}|{kind:"unreadable";reason:string}>;
export type DescriptorEmitterPrerequisitePortsV1=Readonly<{
 inspect():Promise<DescriptorEmitterStateV1>;
 publish():Promise<void>;
 serialize<T>(body:()=>Promise<T>):Promise<T>;
 report(line:string):void;
 signal?:AbortSignal;
}>;

/** 🗣️ Names why the emitter is rebuilt, in English and German.
 * @see ../🌡️freshness/🟦️.ts */
export function descriptorEmitterRebuildReasonV1(state:Exclude<DescriptorEmitterStateV1,{kind:"fresh"}>):string{
 switch(state.kind){
  case "missing":return "Descriptor emitter is not published; building it first. / Der Deskriptor-Emitter ist nicht veröffentlicht; er wird zuerst gebaut.";
  case "stale":return `Descriptor emitter is stale (${state.changed} changed source file(s), first: ${state.first}); rebuilding it first. / Der Deskriptor-Emitter ist veraltet (${state.changed} geänderte Quelldatei(en), erste: ${state.first}); er wird zuerst neu gebaut.`;
  case "unreadable":return `Descriptor emitter publication is unusable (${state.reason}); rebuilding it first. / Die Veröffentlichung des Deskriptor-Emitters ist unbrauchbar (${state.reason}); er wird zuerst neu gebaut.`;
 }
}

/** 🚧️ Wraps a failed prerequisite build so the dev learns the exact target to run for the full output, in English and German. */
export function descriptorEmitterBuildFailureV1(cause:unknown):Error{
 const detail=cause instanceof Error?cause.message:String(cause);
 return new Error(`Descriptor emitter prerequisite build failed: ${detail}. Run "repo run ${DESCRIPTOR_EMITTER_BUILD_TARGET_V1}" for the full output. / Der Voraussetzungs-Build des Deskriptor-Emitters ist fehlgeschlagen: ${detail}. "repo run ${DESCRIPTOR_EMITTER_BUILD_TARGET_V1}" zeigt die vollständige Ausgabe.`,{cause});
}

/** 🥾️ Makes the published descriptor emitter current before a component build: a fresh one is left alone, a missing, stale or unusable one is rebuilt and republished through the `:build` path.
 * The inspection holds no custody when it returns, so the publisher never waits on its own reader; `serialize` lets one of several concurrent callers republish while the rest re-inspect and find it fresh.
 * @see ../🌡️freshness/🟦️.ts */
export async function ensureFreshDescriptorEmitterWithV1(ports:DescriptorEmitterPrerequisitePortsV1):Promise<"fresh"|"built">{
 if((await ports.inspect()).kind==="fresh")return "fresh";
 return ports.serialize(async()=>{
  const state=await ports.inspect();
  if(state.kind==="fresh")return "fresh";
  ports.report(descriptorEmitterRebuildReasonV1(state));
  try{await ports.publish();}catch(error){
   ports.signal?.throwIfAborted();
   throw descriptorEmitterBuildFailureV1(error);
  }
  return "built";
 });
}
