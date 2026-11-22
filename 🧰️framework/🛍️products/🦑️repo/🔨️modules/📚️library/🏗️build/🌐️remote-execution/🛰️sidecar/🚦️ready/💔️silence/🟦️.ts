import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const P=schema.definitions.Policy.const;

/** 💔️ What `ready.json` says about the sidecar: `fresh` (written within the missed-beat limit), `silent` (a sidecar at work that missed `silentBeatsMaximum` beats of `statusHeartbeatMilliseconds`), `idle` (ready or in a state that writes no beat), `absent` (no readable record). */
export type ReadySilenceV1=Readonly<{state:"fresh"|"silent"|"idle"|"absent";missedBeats:number;ageMs:number}>;

/** 💔️ Judges the record of the sidecar by its age in beats. Only a sidecar that is not ready and in a state of work (running, waiting, backing off) owes a beat. A pure function.
 * @see ./🧪️tests/🟦️.ts */
export function readySidecarSilenceV1(input:Readonly<{status:unknown;nowMs:number;policy?:typeof P}>):ReadySilenceV1{
 const policy=input.policy??P,row=input.status;
 if(row===null||typeof row!=="object")return{state:"absent",missedBeats:0,ageMs:0};
 const {ready,state,updatedMs,beatMilliseconds}=row as {ready?:unknown;state?:unknown;updatedMs?:unknown;beatMilliseconds?:unknown};
 if(typeof updatedMs!=="number"||!Number.isFinite(updatedMs))return{state:"absent",missedBeats:0,ageMs:0};
 const declared=typeof beatMilliseconds==="number"&&Number.isFinite(beatMilliseconds)&&beatMilliseconds>0?Math.min(beatMilliseconds,policy.offlineMilliseconds/policy.silentBeatsMaximum):policy.statusHeartbeatMilliseconds,ageMs=Math.max(0,input.nowMs-updatedMs),missedBeats=Math.floor(ageMs/declared);
 if(ready!==false||state!=="running"&&state!=="waiting"&&state!=="backoff")return{state:"idle",missedBeats,ageMs};
 return{state:missedBeats>=policy.silentBeatsMaximum?"silent":"fresh",missedBeats,ageMs};
}

/** 🐕️ The watchdog of the sidecar: `observe` takes the record and the clock and answers `restart` once per silence, at most once per `silenceRestartCooldownMilliseconds`, and `wait` otherwise. A pure function of the records seen so far.
 * @see ./🧪️tests/🟦️.ts */
export function readySilenceWatchV1(policy:typeof P=P):Readonly<{observe(status:unknown,nowMs:number):"wait"|"restart"}>{
 let restartedAt=-Infinity;
 return Object.freeze({observe(status,nowMs){
  if(readySidecarSilenceV1({status,nowMs,policy}).state!=="silent"||nowMs-restartedAt<policy.silenceRestartCooldownMilliseconds)return"wait";
  restartedAt=nowMs;return"restart";
 }});
}

/** 💔️ The bilingual line a restart says: how long the sidecar was silent. */
export function readySilenceNoticeV1(silence:ReadySilenceV1):string{
 const seconds=Math.round(silence.ageMs/1000);
 return `the readiness sidecar wrote nothing for ${seconds} s (${silence.missedBeats} missed beats) and is restarted / der Bereitschafts-Sidecar schrieb seit ${seconds} s nichts (${silence.missedBeats} verpasste Schläge) und wird neu gestartet`;
}

/** 🩺️ What a fleet reader may say about one worker: `absent` (no readable record), `offline` (neither its record nor any heartbeat of its machine was written within `offlineMilliseconds`, so the record is stale evidence and not a running worker), `silent` (the machine lives but a worker at work missed `silentBeatsMaximum` beats), `working`, `ready` or `idle` (a state that writes no beat). */
export type ReadyLivenessV1=Readonly<{liveness:"absent"|"offline"|"silent"|"working"|"ready"|"idle";ageMs:number|null;heartbeatAgeMs:number|null;missedBeats:number}>;

/** 🩺️ Judges one worker for a reader on another machine: the record alone cannot say a worker lives, because a machine that went away leaves its last `running` record behind, so the newest of the record and the heartbeats of the machine (the files named by `heartbeatNames`, written every few seconds by every live origin) decides `offline` first. A pure function.
 * @see ./🧪️tests/🟦️.ts, ./🧪️tests/🐍️.py the independent oracle */
export function readyWorkerLivenessV1(input:Readonly<{status:unknown;heartbeatMs:number|null;nowMs:number;policy?:typeof P}>):ReadyLivenessV1{
 const policy=input.policy??P,silence=readySidecarSilenceV1({status:input.status,nowMs:input.nowMs,policy}),heartbeatAgeMs=input.heartbeatMs===null?null:Math.max(0,input.nowMs-input.heartbeatMs);
 if(silence.state==="absent")return{liveness:"absent",ageMs:null,heartbeatAgeMs,missedBeats:0};
 const newest=Math.min(silence.ageMs,heartbeatAgeMs??Infinity);
 if(newest>=policy.offlineMilliseconds)return{liveness:"offline",ageMs:silence.ageMs,heartbeatAgeMs,missedBeats:silence.missedBeats};
 const ready=(input.status as {ready?:unknown}).ready===true,liveness=silence.state==="silent"?"silent":silence.state==="fresh"?"working":ready?"ready":"idle";
 return{liveness,ageMs:silence.ageMs,heartbeatAgeMs,missedBeats:silence.missedBeats};
}
