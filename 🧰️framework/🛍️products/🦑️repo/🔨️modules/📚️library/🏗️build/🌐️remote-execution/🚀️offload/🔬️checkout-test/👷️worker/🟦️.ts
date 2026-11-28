import {homedir} from "node:os";
import {stretchEnvironmentV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import type {RemoteExecutionPropertyV1} from "../../../🟦️.ts";
import {CHECKOUT_TEST_POLICY_V1,checkoutTestBothV1,checkoutTestPathIsV1,checkoutTestProblemV1,type CheckoutTestChunkV1,type CheckoutTestRequestV1} from "../📐️policy/🟦️.ts";
import {awaitCheckoutRevisionV1,fleetCheckoutReplicaV1,type CheckoutTestReplicaV1} from "../🧭️replica/🟦️.ts";

type Environment=Readonly<Record<string,string|undefined>>;
type Streams=Readonly<{exitCode:number;stdout:Uint8Array;stderr:Uint8Array}>;

/** 🎬️ What a worker does with one checkout test request: run `repo test-file` for the file (`run`: the command line, its environment and working directory, and `finish`, which wraps the answer of that command into the result document), refuse it so the scheduler places it elsewhere (`refuse`: this machine is not Ready or its replica does not hold the requester's revision) or fail it (`fail`: the request itself is not allowed). */
export type CheckoutTestBeginV1=Readonly<{kind:"run";argv:readonly string[];env:Readonly<Record<string,string>>;cwd:string;nonce:string;origin:string;finish(streams:Streams):Promise<Streams>}>|Readonly<{kind:"refuse";reason:"not-ready";message:string}>|Readonly<{kind:"fail";code:number;message:string}>;

/** 👷️ The checkout test capability of a worker: the platform properties it offers and the decision about one request. Without this a worker refuses every checkout test action. */
export type RemoteExecutionCheckoutTestsV1=Readonly<{properties:readonly RemoteExecutionPropertyV1[];begin(request:Uint8Array,signal:AbortSignal):Promise<CheckoutTestBeginV1>}>;

/** 🏗️ Builds the capability from its seams (`argv` replaces the command line for tests): the replica view of this machine (readiness, applied cut, file digests), the `repo` binary that runs the file, the checkout and the environment the file runs in. A request is run only on a Ready machine whose replica holds the requester's cut and its copy of the file; the file runs in this machine's own checkout, under the tool and the hermetic sandbox of its suite (`repo test-file`, with the offload mode `never` so a file never goes on from here). */
export function createCheckoutTestsV1(input:Readonly<{root:string;replica:CheckoutTestReplicaV1;repo:string;environment:Environment;argv?:(repo:string,file:string)=>readonly string[];waitMilliseconds?:number;pollMilliseconds?:number;now?:()=>number;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>}>):RemoteExecutionCheckoutTestsV1{
 const policy=CHECKOUT_TEST_POLICY_V1,fail=(detail:string):CheckoutTestBeginV1=>({kind:"fail",code:3,message:checkoutTestBothV1("refusal.shape",{detail})});
 return Object.freeze({
  properties:Object.freeze([{name:policy.property.format,value:policy.property.formatValue}]),
  async begin(bytes:Uint8Array,signal:AbortSignal):Promise<CheckoutTestBeginV1>{
   if(bytes.length===0||bytes.length>policy.limits.maximumRequestBytes)return fail("size");
   let value:unknown;
   try{value=JSON.parse(new TextDecoder().decode(bytes));}catch{return fail("json");}
   const problem=checkoutTestProblemV1("Request",value);
   if(problem!==undefined)return fail(problem.slice(0,120));
   const request=value as CheckoutTestRequestV1;
   if(!checkoutTestPathIsV1(request.file))return fail("file");
   const ready=await input.replica.ready();
   if(!ready.ready)return{kind:"refuse",reason:"not-ready",message:checkoutTestBothV1("refusal.not-ready",{detail:ready.detail})};
   const held=await awaitCheckoutRevisionV1({replica:input.replica,need:request.cut,file:request.file,fileSha256:request.fileSha256,signal,...input.waitMilliseconds===undefined?{}:{waitMilliseconds:input.waitMilliseconds},...input.pollMilliseconds===undefined?{}:{pollMilliseconds:input.pollMilliseconds},...input.now?{now:input.now}:{},...input.sleep?{sleep:input.sleep}:{}});
   if(!held.held)return{kind:"refuse",reason:"not-ready",message:held.reason==="file"?checkoutTestBothV1("refusal.file",{file:held.detail}):checkoutTestBothV1("refusal.behind",{detail:held.detail})};
   const cut=(await input.replica.cut())??request.cut;
   return{
    kind:"run",argv:input.argv?.(input.repo,request.file)??[input.repo,"test-file","--json","--",request.file],cwd:input.root,nonce:request.nonce,origin:input.replica.origin,
    env:{...Object.fromEntries(Object.entries(input.environment).filter((row):row is [string,string]=>row[1]!==undefined)),[policy.environment.mode]:policy.environment.workerMode,SEMIO_LANGUAGE:request.language,...request.stretchBasisPoints===undefined?{}:stretchEnvironmentV1(request.stretchBasisPoints),...request.environment},
    async finish(streams){
     let result:unknown;
     try{result=JSON.parse(new TextDecoder().decode(streams.stdout));}catch{return streams;}
     return{exitCode:streams.exitCode,stderr:streams.stderr,stdout:new TextEncoder().encode(JSON.stringify({version:1,origin:input.replica.origin,cut,result}))};
    },
   };
  },
 });
}

/** 🌊️ Streams the standard error of a running test file as pieces of live output: bytes pushed meanwhile are cut into pieces of at most `maximumChunkBytes`, written (`write`: one action cache entry per piece, in order) every `flushMilliseconds` or as soon as a piece is full, and a last piece closes the stream. A failing write or `maximumChunks` pieces end the streaming (the requester then prints the rest of the output when the file ends, nothing is lost); `close` never waits longer than `drainMilliseconds` for the writes. Never throws.
 * @see ../🧬️schema/🔣️.json */
export function createCheckoutTestChunkWriterV1(input:Readonly<{nonce:string;origin:string;write:(index:number,chunk:CheckoutTestChunkV1)=>Promise<void>;flushMilliseconds?:number;maximumChunkBytes?:number;maximumChunks?:number;drainMilliseconds?:number}>):Readonly<{push(bytes:Uint8Array):void;close():Promise<void>}>{
 const rule=CHECKOUT_TEST_POLICY_V1.stream,flush=input.flushMilliseconds??rule.flushMilliseconds,size=input.maximumChunkBytes??rule.maximumChunkBytes,limit=input.maximumChunks??rule.maximumChunks,drain=input.drainMilliseconds??rule.drainMilliseconds;
 let pending:Buffer=Buffer.alloc(0),index=0,offset=0,stopped=false,exhausted=false,closed=false,timer:ReturnType<typeof setTimeout>|undefined,tail:Promise<void>=Promise.resolve();
 const send=(data:Buffer,last:boolean):void=>{
  if(stopped||exhausted||closed&&!last)return;
  if(index>=limit){exhausted=true;return;}
  const chunk:CheckoutTestChunkV1={version:1,nonce:input.nonce,index,offset,origin:input.origin,data:data.toString("base64"),last};
  index++;offset+=data.length;
  tail=tail.then(()=>stopped?undefined:input.write(chunk.index,chunk)).catch(()=>{stopped=true;});
 };
 const drainPending=(last:boolean):void=>{
  if(timer!==undefined){clearTimeout(timer);timer=undefined;}
  while(pending.length>size){send(pending.subarray(0,size),false);pending=pending.subarray(size);}
  if(pending.length>0||last){send(pending,last);pending=Buffer.alloc(0);}
 };
 return Object.freeze({
  push(bytes:Uint8Array):void{
   if(stopped||closed)return;
   pending=Buffer.concat([pending,bytes]);
   if(pending.length>=size)drainPending(false);
   else if(timer===undefined&&pending.length>0){timer=setTimeout(()=>{timer=undefined;drainPending(false);},flush);timer.unref?.();}
  },
  async close():Promise<void>{
   if(closed)return;
   drainPending(true);closed=true;
   let waiting:ReturnType<typeof setTimeout>|undefined;
   await Promise.race([tail,new Promise<void>(done=>{waiting=setTimeout(done,drain);waiting.unref?.();})]);
   if(waiting!==undefined)clearTimeout(waiting);
   stopped=true;
  },
 });
}

/** 🔭️ Finds out whether this machine can run checkout tests: it is a fleet member and its account has the `repo` binary. `undefined` when not, and the worker then offers and runs none. */
export async function detectWorkerCheckoutTestsV1(root:string,environment:Environment=process.env):Promise<RemoteExecutionCheckoutTestsV1|undefined>{
 const replica=fleetCheckoutReplicaV1(root);
 if(replica===undefined)return undefined;
 const ready=await import("../../../🛰️sidecar/🚦️ready/🟦️.ts"),home=homedir(),repo=ready.readyRepoBinaryV1(home,environment);
 return repo===undefined?undefined:createCheckoutTestsV1({root,replica,repo,environment:ready.readyStageEnvironmentV1(environment,root,home)});
}
