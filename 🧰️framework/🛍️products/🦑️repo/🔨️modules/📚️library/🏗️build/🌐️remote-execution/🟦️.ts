import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🏃️execution/🧬️schema/🌐️remote-execution.json" with {type:"json"};
import buildPolicy from "../🔣️policy.json" with {type:"json"};

/** 🔢️ A content digest of the protocol: lowercase SHA-256 and size in bytes. */
export type RemoteExecutionDigestV1=Readonly<{hash:string;size:number}>;

/** 🏷️ One name/value property of a platform. */
export type RemoteExecutionPropertyV1=Readonly<{name:string;value:string}>;

/** 🔗️ The target and linker identity required by a linked final's fleet idle probe. */
export type RemoteExecutionLinkedFinalV1=Readonly<{target:string;linker:string}>;

/** 📜️ The authored remote execution section of the build policy. */
export type RemoteExecutionPolicyV1=Readonly<typeof buildPolicy.remoteExecution>;

/** 🔒️ How a worker confines an action. */
export type RemoteExecutionSandboxV1="darwin-sandbox"|"linux-sandbox"|"isolated-root";

/** 📡️ A worker's announcement when it joins (`POST /v1/workers`). */
export type RemoteExecutionWorkerRegistrationV1=Readonly<{version:1;name:string;host:string;platform:string;properties:readonly RemoteExecutionPropertyV1[];concurrency:number;sandbox:RemoteExecutionSandboxV1;runtime:string;origin:string;startedMs?:number;capacity:Readonly<{cores:number;memoryMegabytes:number}>;resumed?:readonly Readonly<{task:string;instance:string;action:RemoteExecutionDigestV1}>[];confinement:RemoteExecutionConfinementV1;network:RemoteExecutionNetworkV1}>;

/** 🧱️ Whether a sandbox confines the file system of an action: `none` is the private input root alone. */
export type RemoteExecutionConfinementV1="full"|"none";

/** 🛜️ What a worker does about the network of an action: cut by the sandbox, asked for but not enforceable by the strategy, or allowed by policy. */
export type RemoteExecutionNetworkV1="denied"|"unenforced"|"allowed";

/** 🚫️ Why a worker refused an action: its own admission at arrival, or (`warden-cancelled`) the host warden cancelled the compiler of a running action. */
export type RemoteExecutionRefusalV1="memory"|"swap"|"swap-activity"|"pressure-critical"|"pressure-warn"|"files"|"attaches"|"load"|"queue-order"|"ramp"|"quiet-limit"|"slots"|"disk"|"not-ready"|"draining"|"warden-cancelled";

/** 🤝️ The scheduler's answer to a registration. */
export type RemoteExecutionWorkerRegisteredV1=Readonly<{version:1;id:string;heartbeatSeconds:number;pollSeconds:number;leaseSeconds:number;cache:Readonly<{url:string;relative:boolean}>;language:"en"|"de"|"none"}>;

/** 📦️ One action assigned to a worker, with the peak megabytes its requester declared (`peakMegabytes`, absent when none) so the worker admits it by its own size. */
export type RemoteExecutionTaskV1=Readonly<{id:string;instance:string;action:RemoteExecutionDigestV1;timeoutSeconds:number;attempt:number;peakMegabytes?:number}>;

/** 📥️ A poll answer: maybe a task, the tasks to cancel, and whether the scheduler still knows the worker. */
export type RemoteExecutionPollAnswerV1=Readonly<{version:1;task:RemoteExecutionTaskV1|null;cancel:readonly string[];known:boolean}>;

/** 💓️ A worker's heartbeat: the tasks it still runs and, when it measures them, the memory each holds now. */
export type RemoteExecutionHeartbeatV1=Readonly<{version:1;running:readonly string[];usage?:readonly Readonly<{task:string;megabytes:number}>[]}>;

/** 🫀️ The scheduler's answer to a heartbeat: whether it still knows the worker and which tasks to abandon. */
export type RemoteExecutionHeartbeatAnswerV1=Readonly<{version:1;known:boolean;cancel:readonly string[]}>;

/** 🚦️ A task outcome in the vocabulary of gRPC; `missing` lists inputs the worker could not find. */
export type RemoteExecutionTaskStatusV1=Readonly<{code:number;message:string;missing?:readonly RemoteExecutionDigestV1[]}>;

/** 🏁️ A worker's report of one task. */
export type RemoteExecutionTaskResultV1=Readonly<{version:1;outcome:"completed"|"failed"|"cancelled"|"refused";status:RemoteExecutionTaskStatusV1;actionResult:string|null;refusal?:RemoteExecutionRefusalV1|null}>;

const definitions=schema.definitions;

/** 🛡️ Validates one value against a named definition of the schema and names the first problem. */
export function requireRemoteExecutionShapeV1(definition:keyof typeof definitions,value:unknown,message:string):void{
 const errors=validateJsonSchemaSubset({$ref:`#/definitions/${definition}`},value,schema);
 if(errors.length)throw Error(`${message}: ${errors[0]}`);
}

/** 🏛️ Reads the authored policy and refuses any drift from its schema. */
export function readRemoteExecutionPolicyV1(policy:unknown=buildPolicy.remoteExecution):RemoteExecutionPolicyV1{
 requireRemoteExecutionShapeV1("Policy",policy,"Closed remote execution policy required");return policy as RemoteExecutionPolicyV1;
}

/** 🧮️ Parses `<64 hex>/<size>` as used in resource names. */
export function remoteExecutionDigestV1(hash:string,size:number|string):RemoteExecutionDigestV1{
 const number=typeof size==="number"?size:Number(size);
 if(!/^[0-9a-f]{64}$/u.test(hash)||!Number.isSafeInteger(number)||number<0)throw Error("Valid digest required");
 return Object.freeze({hash,size:number});
}

/** 🔑️ The canonical key of a platform: its properties sorted by name and value. */
export function remoteExecutionPlatformKeyV1(properties:readonly RemoteExecutionPropertyV1[]):string{
 return [...properties].map(row=>`${row.name}=${row.value}`).sort().join(";");
}

/** 🎚️ The properties an Execute request may carry for the scheduler itself: `semio-offload` (`if-idle`: answer RESOURCE_EXHAUSTED at once when no other machine has headroom; `queue`: wait in the queue for a machine, the action is only refused when no machine could ever take it), `semio-origin` (the origin id of the requester, which never runs its own offload and whose platform key the action must match) and `semio-peak-mb` (the megabytes the action is predicted to need at its peak: a worker whose machine reports less free memory than that, beside what the scheduler already placed on it, is not given the action). They are never matched against workers. */
export const REMOTE_EXECUTION_CONTROL_PROPERTIES_V1:readonly string[]=Object.freeze(["semio-offload","semio-origin","semio-peak-mb"]);

/** 🧹️ The properties of a request that workers are matched against: everything but the control properties. */
export function remoteExecutionMatchPropertiesV1(properties:readonly RemoteExecutionPropertyV1[]):readonly RemoteExecutionPropertyV1[]{
 return properties.filter(row=>!REMOTE_EXECUTION_CONTROL_PROPERTIES_V1.includes(row.name));
}

/** 💾️ The peak megabytes an action declares in `semio-peak-mb`: a non-negative whole number, 0 when absent or malformed (no memory sizing). */
export function remoteExecutionPeakMegabytesV1(properties:readonly RemoteExecutionPropertyV1[]):number{
 const text=properties.find(row=>row.name==="semio-peak-mb")?.value;
 return text!==undefined&&/^[0-9]{1,9}$/u.test(text)?Number(text):0;
}

/** 🎛️ The value of one control property of a request, if it carries it. */
export function remoteExecutionControlV1(properties:readonly RemoteExecutionPropertyV1[],name:"semio-offload"|"semio-origin"|"semio-peak-mb"):string|undefined{
 return properties.find(row=>row.name===name)?.value;
}

/** 🧩️ Whether a worker offering `offered` can run an action that requires `required`: every required property (the control properties excepted) must be offered with the same value (extra offered properties are fine). The toolchain build (`semio-toolchain`) and the target (`semio-target`) are ordinary properties, so an action that names them runs only on workers of the same build. */
export function remoteExecutionPlatformMatchesV1(required:readonly RemoteExecutionPropertyV1[],offered:readonly RemoteExecutionPropertyV1[]):boolean{
 const have=new Map<string,Set<string>>();
 for(const row of offered){const values=have.get(row.name)??new Set<string>();values.add(row.value);have.set(row.name,values);}
 return remoteExecutionMatchPropertiesV1(required).every(row=>have.get(row.name)?.has(row.value)===true);
}

/** 🧭️ Reads `name=value,name=value` (the client contract's `properties` variable) into properties; a malformed list is refused. */
export function remoteExecutionPropertiesV1(text:string|undefined):readonly RemoteExecutionPropertyV1[]{
 if(text===undefined||text==="")return [];
 return text.split(",").map(part=>{const cut=part.indexOf("=");if(cut<1)throw Error(`Property name=value required: ${part}`);return {name:part.slice(0,cut),value:part.slice(cut+1)};});
}

/** 🖥️ The properties a worker of this machine offers, in the contract's vocabulary (`OSFamily`, `Arch`). */
export function remoteExecutionHostPropertiesV1(platform:string=process.platform,arch:string=process.arch,toolchain?:Readonly<{target:string|null;toolchain:string|null}>):readonly RemoteExecutionPropertyV1[]{
 const family=platform==="darwin"?"macos":platform==="win32"?"windows":platform,cpu=arch==="x64"?"amd64":arch;
 return [{name:"Arch",value:cpu},{name:"OSFamily",value:family},...toolchain?.target?[{name:"semio-target",value:toolchain.target}]:[],...toolchain?.toolchain?[{name:"semio-toolchain",value:toolchain.toolchain}]:[]];
}

/** 🏷️ The type of the auxiliary metadata entry in which a worker returns the measured peak resident megabytes of an action (the value is the decimal text); other clients and servers ignore an entry they do not know. */
export const REMOTE_EXECUTION_PEAK_TYPE_URL_V1="type.semio.dev/peak-memory-megabytes";

/** 📤️ The auxiliary metadata entry that carries a measured peak. */
export function remoteExecutionPeakEntryV1(megabytes:number):Readonly<{type_url:string;value:Uint8Array}>{
 return Object.freeze({type_url:REMOTE_EXECUTION_PEAK_TYPE_URL_V1,value:new TextEncoder().encode(String(Math.max(1,Math.ceil(megabytes))))});
}

/** 📥️ The measured peak megabytes in the auxiliary metadata of an action result, or nothing when the worker returned none. */
export function remoteExecutionPeakOfMetadataV1(entries:readonly Readonly<{type_url:string;value:Uint8Array}>[]|undefined):number|undefined{
 for(const entry of entries??[])if(entry.type_url===REMOTE_EXECUTION_PEAK_TYPE_URL_V1){const megabytes=Number(new TextDecoder().decode(entry.value));if(Number.isSafeInteger(megabytes)&&megabytes>0)return megabytes;}
 return undefined;
}

/** 🏷️ The type of the auxiliary metadata entry in which a worker returns how many code generation threads it granted a linked unit (decimal text). */
export const REMOTE_EXECUTION_THREADS_TYPE_URL_V1="type.semio.dev/codegen-threads";

/** 📤️ The auxiliary metadata entry that carries the granted code generation threads. */
export function remoteExecutionThreadsEntryV1(threads:number):Readonly<{type_url:string;value:Uint8Array}>{
 return Object.freeze({type_url:REMOTE_EXECUTION_THREADS_TYPE_URL_V1,value:new TextEncoder().encode(String(Math.max(1,Math.floor(threads))))});
}

/** 📥️ The granted code generation threads in the auxiliary metadata of an action result, or nothing when the unit is not a linked one. */
export function remoteExecutionThreadsOfMetadataV1(entries:readonly Readonly<{type_url:string;value:Uint8Array}>[]|undefined):number|undefined{
 for(const entry of entries??[])if(entry.type_url===REMOTE_EXECUTION_THREADS_TYPE_URL_V1){const threads=Number(new TextDecoder().decode(entry.value));if(Number.isSafeInteger(threads)&&threads>0)return threads;}
 return undefined;
}

/** ⏱️ How a worker spent one action, from the timestamps of its `ExecutedActionMetadata` (differences of one clock, so no clock skew between machines enters): `workerMilliseconds` from taking the action to finishing, `fetchMilliseconds` to bring the input tree into the private root, `executeMilliseconds` of the command itself, `uploadMilliseconds` to hand the outputs to the store. */
export type RemoteExecutionPhasesV1=Readonly<{workerMilliseconds:number;fetchMilliseconds:number;executeMilliseconds:number;uploadMilliseconds:number}>;

/** ⏱️ The phases a worker reported in the execution metadata of an action result, or nothing when a timestamp is missing (a worker that does not report them). */
export function remoteExecutionPhasesOfMetadataV1(metadata:Readonly<Record<string,unknown>>|undefined):RemoteExecutionPhasesV1|undefined{
 if(metadata===undefined)return undefined;
 const at=(name:string):number|undefined=>{const stamp=metadata[name] as {seconds?:number|bigint|string;nanos?:number}|undefined;if(stamp===undefined||stamp===null||stamp.seconds===undefined)return undefined;return Number(stamp.seconds)*1000+Math.round((stamp.nanos??0)/1e6);};
 const start=at("worker_start_timestamp"),done=at("worker_completed_timestamp"),fetchStart=at("input_fetch_start_timestamp"),fetchDone=at("input_fetch_completed_timestamp"),run=at("execution_start_timestamp"),ran=at("execution_completed_timestamp"),uploadStart=at("output_upload_start_timestamp"),uploadDone=at("output_upload_completed_timestamp");
 if(start===undefined||done===undefined||fetchStart===undefined||fetchDone===undefined||run===undefined||ran===undefined||uploadStart===undefined||uploadDone===undefined)return undefined;
 return Object.freeze({workerMilliseconds:Math.max(0,done-start),fetchMilliseconds:Math.max(0,fetchDone-fetchStart),executeMilliseconds:Math.max(0,ran-run),uploadMilliseconds:Math.max(0,uploadDone-uploadStart)});
}
