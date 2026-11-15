/** 🧷️ The responsible process of a macOS process: the file access consent (TCC) of a process is judged against the app that is responsible for it, not against the process itself. A child inherits the responsibility of its spawner and keeps it only while that process lives: when it ends, the kernel makes every orphaned child its own responsible process, so a service that was granted access through the session or shell that started it is judged against the bare runtime from then on and is refused (`EPERM`) in the middle of a plan.
 * @see https://developer.apple.com/documentation/bundleresources/entitlements/com.apple.developer.responsibility.disclaim the disclaim attribute, ../🟨️.mjs the consent */

let nativeCall;

const loadNativeV1=async()=>{
 if(nativeCall!==undefined)return nativeCall;
 try{
  const{dlopen,FFIType}=await import("bun:ffi"),{symbols}=dlopen("/usr/lib/libSystem.B.dylib",{responsibility_get_pid_responsible_for_pid:{args:[FFIType.i32],returns:FFIType.i32}});
  nativeCall=pid=>symbols.responsibility_get_pid_responsible_for_pid(pid);
 }catch{nativeCall=null;}
 return nativeCall;
};

/** 🔎️ The pid of the process that macOS holds responsible for `pid` (itself when it answers for itself), `undefined` on every other system and when the system does not tell. `call` replaces the system call in tests. */
export async function responsiblePidV1(pid=process.pid,{platform=process.platform,call}={}){
 if(platform!=="darwin")return undefined;
 const ask=call??await loadNativeV1();
 if(!ask)return undefined;
 try{const found=Number(await ask(pid));return Number.isInteger(found)&&found>0?found:undefined;}catch{return undefined;}
}

/** 👁️ Watches the responsible process of the calling service: the first `tick` fixes whose access the service runs under (`responsible`, `own: true` when the service answers for itself and has nothing to lose); a later `tick` answers `{lost:true,was,now}` once the system names another responsible process (the earlier one ended and the service fell back to itself). `ask(pid)` answers the responsible pid, `pid` is the watched process. */
export function createResponsibilityWatchV1({pid=process.pid,ask=responsiblePidV1}={}){
 let first;
 return Object.freeze({
  async tick(){
   const now=await ask(pid);
   if(first===undefined){first=now===undefined?null:now;return{responsible:now,own:now===undefined||now===pid};}
   if(first===null||first===pid)return{responsible:now,own:true};
   return now===first?{responsible:now,own:false}:{responsible:now,own:false,lost:true,was:first,now};
  },
  /** 🔁️ Adopts the present responsible process as the baseline (the service proved that it can still read the checkout under it). */
  async adopt(){const now=await ask(pid);first=now===undefined?null:now;},
  was:()=>first??undefined,
 });
}
