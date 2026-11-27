import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 🫧️ The declared policy of the resident memory release: the schema file is the single source.
 * @see ./🧬️schema/🔣️.json */
export const MEMORY_RELEASE_POLICY_V1=Object.freeze(structuredClone(schema.definitions.Policy.const));

/** ⚖️ Whether a worker releases now: never when switched off, never below the resident minimum, never within the interval of the last release, and only while the resident memory is above the declared share of what the process really holds (heap plus external). */
export function memoryReleaseDecisionV1({environment=process.env,residentBytes,heapBytes,externalBytes,lastAtMilliseconds=-Infinity,nowMilliseconds},policy=MEMORY_RELEASE_POLICY_V1){
 if(environment[policy.disableEnvironment]===policy.disabledValue)return false;
 if(![residentBytes,heapBytes,externalBytes,nowMilliseconds].every(Number.isFinite))return false;
 if(nowMilliseconds-lastAtMilliseconds<policy.minimumIntervalMilliseconds)return false;
 if(residentBytes<policy.minimumResidentMegabytes*1048576)return false;
 return residentBytes*10000>(heapBytes+externalBytes)*policy.ratioBasisPoints;
}

/** 🧹️ One release: a full collection and a shrink, answering how long the worker paused. `collect` and `shrink` are the runtime's by default. */
export function releaseResidentMemoryV1({collect=()=>globalThis.Bun.gc(true),shrink=()=>globalThis.Bun.shrink?.(),clock=()=>performance.now()}={}){
 const started=clock();collect();shrink();
 return Object.freeze({pauseMilliseconds:Math.round(clock()-started)});
}

/** 🫧️ The release of this process between two owners: decides from the process's own numbers and runs at most once per interval; answers the pause and the resident memory before, or undefined when it did nothing. The state is the closure of the returned function. */
export function createMemoryReleaseV1({usage=()=>process.memoryUsage(),now=()=>performance.now(),environment=process.env,release=releaseResidentMemoryV1,report=()=>undefined,policy=MEMORY_RELEASE_POLICY_V1}={}){
 let last=-Infinity,releases=0,paused=0;
 const between=()=>{
  const at=now();
  if(at-last<policy.minimumIntervalMilliseconds)return undefined;
  const sample=usage();
  if(!memoryReleaseDecisionV1({environment,residentBytes:sample.rss,heapBytes:sample.heapUsed,externalBytes:sample.external,lastAtMilliseconds:last,nowMilliseconds:at},policy))return undefined;
  last=at;
  const done=release();releases++;paused+=done.pauseMilliseconds;
  const row=Object.freeze({residentMegabytes:Math.round(sample.rss/1048576),heapMegabytes:Math.round(sample.heapUsed/1048576),externalMegabytes:Math.round(sample.external/1048576),pauseMilliseconds:done.pauseMilliseconds});
  report(row);return row;
 };
 return Object.freeze({between,stats:()=>({releases,pausedMilliseconds:paused})});
}
