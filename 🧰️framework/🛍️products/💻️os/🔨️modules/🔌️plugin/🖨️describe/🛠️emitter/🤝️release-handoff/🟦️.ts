import {createHash,randomBytes} from "node:crypto";
import {join} from "node:path";
import {readIssuedScriptProcessEnvelopeV1,requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type {CompilerAdmissionControl} from "@semio-tech/process/compiler-capacity";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {readNativeCallerAuthorityV1,type RepositoryArtifactRequestV1} from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️artifacts/🟦️.ts";
import {descriptorEmitterRequestV1,type DescriptorEmitterIntentV1,type DescriptorEmitterPinV1} from "../📋️contract/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {formatStampProtocolV1} from "../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🟨️.mjs";

export type DescriptorEmitterReleaseRequestV1=Readonly<{version:1;id:string;callerSha256:string;platform:DescriptorEmitterIntentV1["platform"]}>;
export type DescriptorEmitterReleaseEvidenceV1=Readonly<Omit<DescriptorEmitterPinV1,"mode">&{mode:"release"}>;
export const descriptorEmitterReleaseHandoffPolicyV1=Object.freeze({...schema.definitions.Protocol.const});

function requestV1(value:unknown):DescriptorEmitterReleaseRequestV1{
 if(validateJsonSchemaSubset(schema.definitions.Request,value,schema).length)throw Error("Closed descriptor emitter release request required");
 return Object.freeze({...value as DescriptorEmitterReleaseRequestV1});
}

function releaseCallerSha256V1(invocation:ScriptInvocation):string{
 const envelope=readIssuedScriptProcessEnvelopeV1(invocation),caller=readNativeCallerAuthorityV1(envelope.capabilities);
 if(caller.original.buildMode!=="release")throw Error("Original release descriptor emitter caller required");
 return createHash("sha256").update(JSON.stringify(envelope.capabilities)).digest("hex");
}

/** 🪪️ Issues one fresh nonce carried only by this admitted release job attempt. */
export function issueDescriptorEmitterReleaseRequestV1(invocation:ScriptInvocation):DescriptorEmitterReleaseRequestV1{
 return requestV1({version:1,id:randomBytes(16).toString("hex"),callerSha256:releaseCallerSha256V1(invocation),platform:process.platform});
}

/** 🧭️ Admits only a request addressed to this same forwarded Original and platform. */
export function requireDescriptorEmitterReleaseRequestV1(value:unknown,invocation:ScriptInvocation):DescriptorEmitterReleaseRequestV1{
 const row=requestV1(value);
 if(row.platform!==process.platform||row.callerSha256!==releaseCallerSha256V1(invocation))throw Error("Descriptor emitter request differs from this Original release caller");
 return row;
}

/** 📮️ Reads the bounded private request supplied to the producer by the live native job. */
export function receivedDescriptorEmitterReleaseRequestV1(environment:NodeJS.ProcessEnv,invocation:ScriptInvocation):DescriptorEmitterReleaseRequestV1|undefined{
 const wire=environment[descriptorEmitterReleaseHandoffPolicyV1.environment];
 if(wire===undefined)return;
 if(Buffer.byteLength(wire)>descriptorEmitterReleaseHandoffPolicyV1.maximumBytes)throw Error("Descriptor emitter release request exceeds its transport extent");
 return requireDescriptorEmitterReleaseRequestV1(JSON.parse(wire),invocation);
}

/** 📁️ Derives the generation from an attempt nonce under the admitted products owner. */
export function descriptorEmitterReleaseArtifactRequestV1(productsRoot:string,value:DescriptorEmitterReleaseRequestV1):RepositoryArtifactRequestV1{
 const request=requestV1(value),original=descriptorEmitterRequestV1(productsRoot,{version:1,mode:"release",platform:request.platform});
 return Object.freeze({...original,owner:original.owner+":"+request.id,path:join(productsRoot,descriptorEmitterReleaseHandoffPolicyV1.namespace,request.id)});
}

/** 🔏️ Reads the exact request echo and the three actual compiler artifact identities. */
export function readDescriptorEmitterReleaseReplyV1(value:unknown,expected:DescriptorEmitterReleaseRequestV1):DescriptorEmitterReleaseEvidenceV1{
 if(validateJsonSchemaSubset(schema.definitions.Reply,value,schema).length)throw Error("Closed sealed descriptor emitter release reply required");
 const {request,evidence}=value as Readonly<{request:DescriptorEmitterReleaseRequestV1;evidence:DescriptorEmitterReleaseEvidenceV1}>;
 if(request.id!==expected.id||request.callerSha256!==expected.callerSha256||request.platform!==expected.platform||evidence.platform!==expected.platform)throw Error("Descriptor emitter reply differs from the exact admitted release request");
 return Object.freeze({...evidence});
}

/** 🚦️ Accepts a settled provider only after receipt verification, including a structured restart after the seal. */
export function descriptorEmitterReleaseJobAcceptsV1(exit:number,restartExit:number,sealed:boolean):boolean{return sealed&&(exit===0||exit===restartExit);}

/** 🧵️ Carries one accepted producer through this driver's subsequent descriptor stages. */
export function createDescriptorEmitterReleaseJobV1(invocation:ScriptInvocation,id:string){
 let selected:DescriptorEmitterReleaseRequestV1|undefined;
 const issued=new Set<DescriptorEmitterReleaseRequestV1>();
 return Object.freeze({
  id,
  requests:()=>Object.freeze([...issued]),
  selected:()=>{
   if(!selected)throw Error("Sealed release descriptor emitter prerequisite required");
   return requireDescriptorEmitterReleaseRequestV1(selected,invocation);
  },
  attempt(control:CompilerAdmissionControl,verify:(request:DescriptorEmitterReleaseRequestV1)=>Promise<boolean>){
   requireIssuedResourceControlInvocationV1(control,invocation);
   const request=issueDescriptorEmitterReleaseRequestV1(invocation),restart=formatStampProtocolV1().exitCode;
   issued.add(request);
   return Object.freeze({
    environment:Object.freeze({[descriptorEmitterReleaseHandoffPolicyV1.environment]:JSON.stringify(request)}),
    async settle(exit:number):Promise<number>{
     requireIssuedResourceControlInvocationV1(control,invocation);
     if(exit!==0&&exit!==restart)return exit;
     const accepted=await verify(request);
     if(!descriptorEmitterReleaseJobAcceptsV1(exit,restart,accepted)){
      if(exit===0)throw Error("Release descriptor emitter exited without a sealed same-request reply");
      return exit;
     }
     selected??=request;
     return 0;
    },
   });
  },
 });
}
