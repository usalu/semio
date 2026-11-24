/** 🔕️ The quiet gate of a slice worker restart: a worker that ended because the build tool's own code changed under it (a pinned implementation drift, a changed module-load authority) is started again only after that code has been unchanged for a quiet period, so a burst of edits restarts a load once instead of once per edit, and the new worker never loads a module a peer is still writing.
 * @see ../🟨️.mjs `createSlicePoolV1`
 * @see ../../../../../👁️observation/🟨️.mjs `staleSourceFactsErrorV1`, `requirePinnedImplementationFactsV1` */
import {lstatSync} from "node:fs";
import {isAbsolute,relative,sep} from "node:path";

const DRIFT_MARKERS_V1=["restart required","restart the owned build invocation","BUILD_AUTHORITY_CHANGED"];

/** 🧭️ The absolute paths a worker's restart cause names as changed implementation: the `kind:path` rows of a stale implementation and the `path` of a pinned drift; nothing for any other cause. */
export function implementationDriftPathsV1(cause){
 const text=String(cause??""),found=new Set();
 if(!DRIFT_MARKERS_V1.some(marker=>text.includes(marker)))return[];
 for(const match of text.matchAll(/\b[a-z][A-Za-z]*:((?:\/|[A-Za-z]:\\)[^\s,()"]+)/gu))found.add(match[1].replace(/[.;:]+$/u,""));
 for(const match of text.matchAll(/"path":"((?:\/|[A-Za-z]:\\\\)[^"]+)"/gu))found.add(match[1].replaceAll("\\\\","\\"));
 return[...found].sort();
}

/** 📍️ The last change of each path in wall-clock milliseconds: its status change time, which this machine's kernel sets for every write, rename and replica copy (a modification time may be carried over by a copy); `undefined` for a path that is gone. */
export function implementationStampsV1(paths,stat=lstatSync){
 return new Map([...paths].map(path=>{try{return[path,stat(path).ctimeMs];}catch{return[path,undefined];}}));
}

/** ⚖️ Whether the code of `stamps` has been unchanged for `quietMilliseconds` at `nowMs`: the newest change decides, a change stamped after `nowMs` counts as a change now, a removed path is quiet (a module that comes back gets a new stamp). */
export function implementationQuietV1({stamps,nowMs,quietMilliseconds}){
 let newestPath,newestAt;
 for(const[path,at]of stamps)if(Number.isFinite(at)&&(newestAt===undefined||at>newestAt||at===newestAt&&path<newestPath)){newestPath=path;newestAt=at;}
 const quietForMilliseconds=newestAt===undefined?quietMilliseconds:Math.max(0,nowMs-newestAt);
 return Object.freeze({quiet:quietForMilliseconds>=quietMilliseconds,quietForMilliseconds,...newestPath===undefined?{}:{newestPath}});
}

/** 🏷️ A path as a progress line shows it: relative to the checkout when it lies inside, otherwise as it is. */
export function implementationPathShownV1(root,path){
 if(path===undefined)return"";
 const inside=typeof root==="string"&&isAbsolute(root)?relative(root,path):"..";
 return inside!==""&&inside!==".."&&!inside.startsWith(".."+sep)&&!isAbsolute(inside)?inside.split(sep).join("/"):path;
}

/** 💰️ The quiet policy of the next restart of one load, given the quiet it already waited: while the load's total budget (`totalMaximumMilliseconds`) lasts the full policy applies with its maximum cut to what is left; afterwards only the short floor (`floorMilliseconds`) is waited. Under edits that never stop, the waits of a load therefore sum to the budget instead of growing with every restart. A slice is never mixed by this: each worker pins the implementation it loaded and ends with a drift restart when it changes later, so only a worker whose pin held answers.
 * @see ./🟨️.d.mts */
export function quietBudgetV1(policy,spentMilliseconds){
 const total=policy.totalMaximumMilliseconds??Infinity,left=total-spentMilliseconds;
 if(left>0)return left>=policy.maximumMilliseconds?policy:{...policy,maximumMilliseconds:left};
 const floor=Math.min(policy.floorMilliseconds??policy.milliseconds,policy.milliseconds);
 return{milliseconds:floor,maximumMilliseconds:floor};
}

/** ⏳️ Waits until the code at `paths` has been quiet for `policy.milliseconds`, at most `policy.maximumMilliseconds`, checking every `heartbeatMilliseconds` and announcing each check through `announce(verdict,waitedMilliseconds)`; resolves the time waited, ends early when `signal` aborts. */
export async function awaitImplementationQuietV1({paths,policy,heartbeatMilliseconds,signal,announce=()=>undefined,stamps=implementationStampsV1,now=()=>Date.now()}){
 const began=now();
 for(;;){
  const at=now(),waited=at-began,verdict=implementationQuietV1({stamps:stamps(paths),nowMs:at,quietMilliseconds:policy.milliseconds});
  if(verdict.quiet||waited>=policy.maximumMilliseconds||signal?.aborted)return waited;
  announce(verdict,waited);
  const pause=Math.max(1,Math.min(heartbeatMilliseconds,policy.milliseconds-verdict.quietForMilliseconds,policy.maximumMilliseconds-waited));
  await new Promise(done=>{
   const finish=()=>{clearTimeout(timer);signal?.removeEventListener("abort",finish);done();};
   const timer=setTimeout(finish,pause);
   signal?.addEventListener("abort",finish,{once:true});
  });
 }
}
