import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {compileJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

const policy=schema.definitions.Policy.const;
const receiptSchema=compileJsonSchemaSubset({$ref:"#/definitions/Receipt"},schema);
export type ReplicaProgressScopeV1="git-health-metadata"|"git-heal-metadata";
export type ReplicaProgressReceiptV1=Readonly<{version:1;reason:"git-metadata-progress-stall";scope:ReplicaProgressScopeV1;pid:number;began:number;attested:number;at:number}>;
type Observation=Readonly<{now:number;began:number|null;attested:number|null;lag:number;probeMilliseconds:number;store:boolean;local:boolean}>;

/** ⏱️ Qualifies only a stale owned operation and independently stale attestation while the current loop and both probes answer. */
export function replicaProgressStalledV1(row:Observation):boolean{
 return row.began!==null&&row.attested!==null&&row.now-row.began>=policy.quietMs&&row.now-row.attested>=policy.quietMs&&
  row.lag>=0&&row.lag<=policy.maximumLagMs&&row.probeMilliseconds>=0&&row.probeMilliseconds<=policy.probeMs&&row.store&&row.local;
}

/** 🧾️ Checks only a closed bounded local intent that the current child can use for restart backoff. */
export function isReplicaProgressReceiptV1(value:unknown):value is ReplicaProgressReceiptV1{
 if(receiptSchema.validate(value).length)return false;
 const row=value as ReplicaProgressReceiptV1;
 return row.at-row.began>=policy.quietMs&&row.at-row.attested>=policy.quietMs;
}

/** 📅️ Allows one locally proven asynchronous progress restart per cooldown window. */
export function replicaProgressRestartDueV1(previous:unknown,now:number):boolean{
 return previous===undefined||isReplicaProgressReceiptV1(previous)&&now-previous.at>=policy.restartPauseMs;
}

/** 🫀️ Watches one exact Git metadata operation at a time without unwinding or yielding any of its custody. */
export function createReplicaProgressWatchV1(input:Readonly<{
 pid:number;
 now?:()=>number;
 signal:AbortSignal;
 probe:()=>Promise<Readonly<{store:boolean;local:boolean}>>;
 prepare:(row:ReplicaProgressReceiptV1)=>Promise<boolean>;
 restart:()=>void;
}>){
 const now=input.now??Date.now;
 let active:{scope:ReplicaProgressScopeV1;began:number}|undefined,attested:number|null=null;
 let preparing:Promise<boolean>|undefined;
 let expected=now()+policy.pollMs,probing=false,restarting=false,nextProbe=0,closed=false;
 const timer=setInterval(()=>{
  const at=now(),lag=Math.max(0,at-expected),ticket=active;
  expected=at+policy.pollMs;
  if(closed||input.signal.aborted||restarting||probing||ticket===undefined||at<nextProbe)return;
  const qualifies=(store:boolean,local:boolean)=>ticket===active&&!closed&&!input.signal.aborted&&replicaProgressStalledV1({now:now(),began:ticket.began,attested,lag,probeMilliseconds:now()-at,store,local});
  if(!qualifies(true,true))return;
  probing=true;nextProbe=at+policy.probeIntervalMs;
  void input.probe().then(async({store,local})=>{
   if(!qualifies(store,local))return;
   const receipt:ReplicaProgressReceiptV1={version:1,reason:"git-metadata-progress-stall",scope:ticket.scope,pid:input.pid,began:ticket.began,attested:attested!,at:now()};
   const work=input.prepare(receipt);preparing=work;
   try{if(await work&&qualifies(store,local)){restarting=true;input.restart();}}
   finally{if(preparing===work)preparing=undefined;}
  }).catch(()=>{}).finally(()=>{probing=false;});
 },policy.pollMs);
 timer.unref?.();
 return {
  async run<T>(scope:ReplicaProgressScopeV1,work:()=>Promise<T>):Promise<T>{
   const ticket={scope,began:now()};active=ticket;
   try{return await work();}finally{if(active===ticket)active=undefined;}
  },
  attested(){attested=now();},
  async close(){closed=true;clearInterval(timer);await preparing?.catch(()=>false);},
 };
}
