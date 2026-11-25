import {noticeLanguagesV1} from "../../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {nativeImageOperationAdmissionV1,nativeImageOperationSlotsV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📋️state/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

export type PartitionRefusalKindV1="notIssued"|"closed"|"failed"|"closing"|"waitTimeout";
type Texts=Readonly<{en:string;de:string}>;
const policy=schema.definitions.Protocol.const,messages=schema.definitions.Messages.const as Readonly<Record<PartitionRefusalKindV1|"waiting",Texts>>;
/** 🚦️ The closed bound of concurrently admitted top-level operations of one partition owner. */
export const partitionOperationSlotsV1:number=nativeImageOperationSlotsV1;
/** 🚦️ Whether one more top-level operation fits beside `busy` others; the only decision, taken by the Native operation admission shared with the borrower. */
export function partitionHasRoomV1(busy:number):boolean{return nativeImageOperationAdmissionV1({pending:busy,topOpen:true,nestedLive:false}).accepts;}
const fill=(text:string,values:Readonly<Record<string,string|number>>):string=>text.replace(/\{(\w+)\}/gu,(_,name:string)=>String(values[name]??""));

/** 🪧️ Renders one partition message in both languages. */
export function partitionTextsV1(kind:PartitionRefusalKindV1|"waiting",values:Readonly<Record<string,string|number>>):Texts{const row=messages[kind];return{en:fill(row.en,values),de:fill(row.de,values)};}
/** 🛑️ A typed bilingual refusal naming which of the distinct causes refused the operation; the first failure of a failed owner is its `cause`. */
export function partitionRefusalV1(kind:PartitionRefusalKindV1,values:Readonly<Record<string,string|number>>,cause?:unknown):Error{const texts=partitionTextsV1(kind,values);return Object.assign(new Error(texts.en,cause===undefined?undefined:{cause}),{name:"RepositoryPartitionRefused",kind,texts});}
/** 🔎️ Names the failure of an owner for the `failed` text. */
export function partitionFailureTextV1(failure:unknown):string{return failure instanceof Error?failure.message:String(failure);}

/** 🔌️ What the waiting needs from its owner: the live count, a settle subscription and the refusal checks of the owner, which run again after every wake. */
export interface PartitionSlotWaitV1{readonly id:string;busy():number;recheck():void;onSettled(wake:()=>void):()=>void}
/** ⏱️ Injectable clock, timer and notice sink; the defaults are real. */
export interface PartitionSlotPortsV1{now():number;sleep(milliseconds:number,wake:Promise<void>):Promise<void>;write(line:string):void}
const realPorts:PartitionSlotPortsV1={now:()=>Date.now(),sleep:(milliseconds,wake)=>new Promise<void>(done=>{const timer=setTimeout(done,milliseconds);void wake.then(()=>{clearTimeout(timer);done();});}),write:line=>{process.stderr.write(line);}};

/** ⏳️ Back-pressure: waits, bounded and cancellable, until fewer than the limit of operations are busy, reporting progress; a refusal of the owner or the control ends the wait at once. */
export async function awaitPartitionSlotV1(wait:PartitionSlotWaitV1,control:ResourceLeaseControl,ports:PartitionSlotPortsV1=realPorts):Promise<void>{
 const limit=partitionOperationSlotsV1,started=ports.now();let announced=started;
 while(!partitionHasRoomV1(wait.busy())){
  wait.recheck();checkResourceLeaseControl(control);
  const elapsed=ports.now()-started;
  if(elapsed>=policy.maximumWaitMilliseconds)throw partitionRefusalV1("waitTimeout",{id:wait.id,seconds:Math.round(elapsed/1000),busy:wait.busy(),limit});
  if(ports.now()-announced>=policy.noticeMilliseconds){announced=ports.now();const texts=partitionTextsV1("waiting",{id:wait.id,seconds:Math.round(elapsed/1000),busy:wait.busy(),limit});for(const language of noticeLanguagesV1(["en","de"] as const))ports.write("[repo cache] "+texts[language]+"\n");}
  let wake!:()=>void;const woken=new Promise<void>(done=>{wake=done;}),off=wait.onSettled(wake);
  try{await ports.sleep(policy.pollMilliseconds,woken);}finally{off();}
 }
 wait.recheck();checkResourceLeaseControl(control);
}
