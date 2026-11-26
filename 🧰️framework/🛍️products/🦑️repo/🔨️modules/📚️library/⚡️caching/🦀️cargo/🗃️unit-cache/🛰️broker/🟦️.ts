import {createConnection} from "node:net";
import {UNIT_CACHE_POLICY_V1,type AdmitFactsV1} from "../🔏️key/🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1.broker;
type Environment=Readonly<Record<string,string|undefined>>;
/** 🧾️ What the compiler measured when the unit was first built, kept in the cache entry so a later hit can report it. */
export type UnitProvenanceV1=Readonly<{peakRssBytes:number;userMs:number;sysMs:number;wallMs:number;rmetaOffsetMs:number|null}>;
type Endpoint=Readonly<{endpoint:string;pass:string}>;

/** 🎟️ The broker appointed by the current compiler launcher. */
export function compilerBrokerOfV1(environment:Environment):Endpoint|undefined{
 const endpoint=environment[policy.environment],pass=environment[policy.pass];
 return endpoint&&pass?{endpoint,pass}:undefined;
}

/** 🎚️ The broker the build driver started for this compiler call, when replay is switched on. */
export function brokerOfV1(environment:Environment):Endpoint|undefined{
 return environment.SEMIO_UNIT_CACHE_BROKER_REPLAY==="on"?compilerBrokerOfV1(environment):undefined;
}

function exchange(endpoint:string,message:object,reply:"verdict"|"unit"|"headroom"):Promise<Record<string,unknown>|undefined>{
 return new Promise(done=>{
  const socket=createConnection(endpoint),timer=setTimeout(()=>{socket.destroy();done(undefined);},policy.timeoutMilliseconds);let text="";
  const finish=(value:Record<string,unknown>|undefined)=>{clearTimeout(timer);socket.destroy();done(value);};
  socket.on("error",()=>finish(undefined));
  socket.on("connect",()=>socket.write(JSON.stringify(message)+"\n"));
  socket.on("data",chunk=>{text+=chunk;const at=text.indexOf("\n");if(at<0)return;try{const row=JSON.parse(text.slice(0,at));finish(row.op===reply?row:undefined);}catch{finish(undefined);}});
  socket.on("close",()=>finish(undefined));
 });
}

/** 🔭️ Asks the broker for the measurements of a unit its scheduler just finished (the scheduler reports after the compiler exits, so the answer is awaited briefly). */
export async function recallProvenanceV1(environment:Environment,facts:AdmitFactsV1,localInputKey?:string):Promise<UnitProvenanceV1|undefined>{
 const broker=localInputKey?compilerBrokerOfV1(environment):brokerOfV1(environment);if(!broker)return undefined;
 for(let attempt=0;attempt<policy.recallAttempts;attempt++){
  const row=await exchange(broker.endpoint,{v:1,op:"recall",pass:broker.pass,crate:facts.crate,outDir:facts.outDir,extraFilename:facts.extraFilename,...localInputKey?{localInputKey}:{}},"unit");
  const record=row?.record as {startMs:number;endMs:number;rmetaMs:number|null;peakRssBytes:number;userMs:number;sysMs:number}|null|undefined;
  if(record)return{peakRssBytes:record.peakRssBytes,userMs:record.userMs,sysMs:record.sysMs,wallMs:Math.max(0,record.endMs-record.startMs),rmetaOffsetMs:record.rmetaMs===null?null:Math.max(0,record.rmetaMs-record.startMs)};
  if(row===undefined)return undefined;
  await new Promise(done=>setTimeout(done,policy.recallPauseMilliseconds));
 }
 return undefined;
}

/** 📣️ Tells the broker that a unit was served from the cache, with the measurements recorded when it was built, so the build's unit list, critical path and provenance are complete. */
export async function replayCachedUnitV1(environment:Environment,facts:AdmitFactsV1,cacheKey:string,provenance:UnitProvenanceV1|null|undefined):Promise<boolean>{
 const broker=brokerOfV1(environment);if(!broker)return false;
 const row=await exchange(broker.endpoint,{v:1,op:"cached",pass:broker.pass,pid:process.pid,...facts,cacheKey,provenance:provenance??null},"verdict");
 return row?.admitted===true;
}

/** ⏳️ Waits for the broker's headroom BEFORE the compile slot is taken: a wrapper that held the single slot while its scheduler waited for admission froze every other unit behind it (one stuck admission, 28 queued units). Polls the broker's `probe` (never joins its queue) every `headroomPauseMilliseconds` until it would admit the unit or `headroomMaximumWaitMilliseconds` passed; `"none"` when no broker answers, `"timeout"` when the bound passed (the caller then goes on, and the broker's own bounds end the wait).
 * @see ./../🧪️tests/⌛️headroom-first/🟦️.ts */
export async function awaitBrokerHeadroomV1(environment:Environment,facts:AdmitFactsV1,options:Readonly<{maximumWaitMilliseconds?:number;pauseMilliseconds?:number}>={}):Promise<"admit"|"timeout"|"none">{
 const broker=compilerBrokerOfV1(environment);if(!broker)return"none";
 const started=Date.now(),limit=options.maximumWaitMilliseconds??policy.headroomMaximumWaitMilliseconds,pause=options.pauseMilliseconds??policy.headroomPauseMilliseconds;
 for(;;){
  const row=await exchange(broker.endpoint,{v:1,op:"probe",pass:broker.pass,pid:process.pid,...facts},"headroom");
  if(typeof row?.wait!=="boolean"||row.fleetOnly===true)return"none";
  if(!row.wait)return"admit";
  const waited=Date.now()-started;if(waited>=limit)return"timeout";
  await new Promise(done=>setTimeout(done,Math.min(pause,limit-waited)));
 }
}

/** 🔭️ Asks the build's admission broker whether a unit with these facts would queue behind its memory budget right now, without joining the queue: `true` for wait, `false` for admit, `undefined` when no broker answers in time. */
export async function brokerWouldWaitV1(environment:Environment,facts:AdmitFactsV1,localInputKey?:string):Promise<boolean|undefined>{
 const broker=localInputKey?compilerBrokerOfV1(environment):brokerOfV1(environment);if(!broker)return undefined;
 const row=await exchange(broker.endpoint,{v:1,op:"probe",pass:broker.pass,pid:process.pid,...facts,...localInputKey?{localInputKey}:{}},"headroom");
 return typeof row?.wait==="boolean"?row.wait:undefined;
}
