import {isAbsolute,resolve,basename} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🌱️ Closed service-birth data binds the actual issuer; this wire value never creates authority. */
export type NxDaemonBirthV1=Readonly<{version:1;repositoryRoot:string;workspaceRoot:string;toolkitRoot:string;toolingIdentity:string;directory:string;watcherIdentity:string|null;graphIdentity:string|null;implementationIdentity:string;generation:string;parentWitness:string;parentEnvelopeDigest:string}>;
/** 🎟️ These loans follow the current root server's actual Native, SDK and finite-data readers. */
export type NxDaemonLoanV1="leases"|"tools"|"rust"|"sdk";
/** 📨️ Lifecycle observations originate only from their actual service resource owners. */
export type NxDaemonLifetimeEventV1=Readonly<{type:"acquire";resource:NxDaemonLoanV1}>|Readonly<{type:"worker-open"|"worker-close"|"request-open"|"request-close";id:string}>|Readonly<{type:"acquired"|"server-open"|"ready"|"parent-exit"|"drain"|"server-close"|"terminal"}>;
/** 🔎️ Read-only sequencing evidence remains distinct from issued image and Original service capabilities. */
export type NxDaemonLifetimeSnapshotV1=Readonly<{birth:NxDaemonBirthV1;phase:"awaiting-acquisition"|"acquired"|"ready"|"draining"|"terminal";parentHeld:boolean;server:boolean;loans:readonly NxDaemonLoanV1[];workers:readonly string[];requests:readonly string[]}>;
/** 🛡️ Admits closed physical path spellings before the actual issuer verifies their finite namespace. */
export function readNxDaemonBirthV1(value:unknown):NxDaemonBirthV1{
 if(validateJsonSchemaSubset(schema,value).length)throw Error("Closed original daemon birth descriptor required");const birth=value as NxDaemonBirthV1;
 if([birth.repositoryRoot,birth.workspaceRoot,birth.toolkitRoot,birth.directory].some(path=>!isAbsolute(path)||resolve(path)!==path||path.includes("\0"))||basename(birth.toolkitRoot)!==birth.toolingIdentity)throw Error("Exact original daemon birth paths required");
 return Object.freeze({...birth});
}
/** 🧭️ Enforces acquisition-before-readiness and complete-drain-before-release without minting service authority. */
export function createNxDaemonLifetimeV1(value:unknown):Readonly<{record(event:NxDaemonLifetimeEventV1):NxDaemonLifetimeSnapshotV1;snapshot():NxDaemonLifetimeSnapshotV1}>{
 const birth=readNxDaemonBirthV1(value),loans=new Set<NxDaemonLoanV1>(),workers=new Set<string>(),requests=new Set<string>();let phase:NxDaemonLifetimeSnapshotV1["phase"]="awaiting-acquisition",parentHeld=true,server=false;
 const snapshot=():NxDaemonLifetimeSnapshotV1=>Object.freeze({birth,phase,parentHeld,server,loans:Object.freeze([...loans].sort()),workers:Object.freeze([...workers].sort()),requests:Object.freeze([...requests].sort())});
 const refuse=():never=>{throw Error("Original daemon lifetime transition refused");};
 return Object.freeze({snapshot,record(event:NxDaemonLifetimeEventV1){
  if(validateJsonSchemaSubset(schema.$defs.Event,event).length||phase==="terminal")return refuse();
  switch(event.type){
   case "acquire":if(phase!=="awaiting-acquisition"||loans.has(event.resource))return refuse();loans.add(event.resource);break;
   case "acquired":if(phase!=="awaiting-acquisition"||loans.size!==4)return refuse();phase="acquired";break;
   case "server-open":if(phase!=="acquired"||server)return refuse();server=true;break;
   case "ready":if(phase!=="acquired"||!server)return refuse();phase="ready";break;
   case "parent-exit":if(!parentHeld)return refuse();parentHeld=false;if(phase==="awaiting-acquisition")phase="draining";break;
   case "drain":phase="draining";break;
   case "server-close":if(phase!=="draining"||!server)return refuse();server=false;break;
   case "worker-open":if(!["acquired","ready"].includes(phase)||workers.has(event.id))return refuse();workers.add(event.id);break;
   case "request-open":if(phase!=="ready"||requests.has(event.id))return refuse();requests.add(event.id);break;
   case "worker-close":case "request-close":if(!(event.type==="worker-close"?workers:requests).delete(event.id))return refuse();break;
   case "terminal":if(phase!=="draining"||server||workers.size||requests.size)return refuse();phase="terminal";parentHeld=false;loans.clear();break;
  }
  return snapshot();
 }});
}
