import{requireNativeStorageRootCapabilityV1}from"../../🛂️root-capability/🟦️.ts";
import{processIdentityOfV1}from"../../../../../🔖️identity/🟦️.ts";
import{nativeImageHdiutilProgramV1}from"../🛠️hdiutil/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
const policy=schema.definitions.Policy.const,texts=schema.definitions.Texts.const;
/** 📐️ The schema-declared bounds of the busy-detach retry: attempts, back-off steps and the holder census.
 * @see ./🧬️schema/🔣️.json */
export const NATIVE_IMAGE_DETACH_POLICY_V1=policy;
/** 🔖️ One process that keeps files open on a mounted image: the shared process identity (pid and creation time, 0 when unknown) and its command name. */
export type NativeImageHolderV1=Readonly<{pid:number;startMs:number;command:string}>;
/** 🔌️ The effects of one polite detach: the detach itself (never forced), the holder census and a cancellable wait. */
export type NativeImageDetachPortV1=Readonly<{detach(mountPoint:string):Promise<void>;holders(mountPoint:string):Promise<readonly NativeImageHolderV1[]>;sleep(milliseconds:number):Promise<void>}>;
/** 🧾️ Tells whether a failed detach only reports that something still uses the image (`hdiutil` status 16 `Resource busy`, `umount` status 32 `target is busy`, a Windows VHD held by another process): the one failure a retry can cure. Pure and language-agnostic. */
export function nativeImageDetachBusyV1(error:unknown):boolean{const text=error instanceof Error?error.message:String(error);return policy.busyStatuses.some(status=>text.includes(" exited with status "+status+" signal null"))||policy.busyTexts.some(marker=>text.includes(marker));}
/** 📋️ Reads the holders from `lsof -F pc` output (`p<pid>` starts a process, `c<command>` names it), each pid once, ascending. Pure and language-agnostic. See https://man7.org/linux/man-pages/man8/lsof.8.html */
export function parseNativeImageHoldersV1(text:string):readonly Readonly<{pid:number;command:string}>[]{
 const rows=new Map<number,string>();let pid=0;
 for(const line of text.split("\n")){
  if(line.startsWith("p")){const value=Number(line.slice(1));pid=Number.isSafeInteger(value)&&value>0?value:0;if(pid>0&&!rows.has(pid))rows.set(pid,"");}
  else if(line.startsWith("c")&&pid>0&&rows.get(pid)==="")rows.set(pid,line.slice(1));
 }
 return Object.freeze([...rows].sort((left,right)=>left[0]-right[0]).map(([id,command])=>Object.freeze({pid:id,command})));
}
/** 📊️ The seconds to wait before the next attempt: the declared steps, the last one repeated. */
export function nativeImageDetachBackoffMillisecondsV1(attempt:number):number{const steps=policy.backoffMilliseconds;return steps[Math.min(Math.max(0,attempt),steps.length-1)]!;}
const fill=(template:string,values:Readonly<Record<string,string>>)=>template.replace(/<([A-Za-z]+)>/gu,(whole,name)=>values[name]??whole);
const both=(row:Readonly<{en:string;de:string}>,values:Readonly<Record<string,string>>)=>fill(row.en,values)+" / "+fill(row.de,values);
const describe=(holders:readonly NativeImageHolderV1[])=>holders.length?holders.slice(0,policy.maximumReportedHolders).map(row=>row.command+" pid "+row.pid+(row.startMs>0?" start "+new Date(row.startMs).toISOString():"")).join(", ")+(holders.length>policy.maximumReportedHolders?", +"+(holders.length-policy.maximumReportedHolders):""):both(texts.none,{});
/** 🔁️ Detaches an image politely and waits out a transient reader: a busy refusal (status 16) is retried with the declared back-off, every wait is reported in English and German with the holders named by pid and start time, cancellation (`check` throws) ends the wait at once, and a still busy image after the last attempt fails with the holders named. It never forces: a live reader keeps its files. Any other failure propagates unchanged. `ready` observes between attempts that the image is already gone (a concurrent detach) and then ends the loop. */
export async function detachNativeImageWithRetryV1(given:Readonly<{port:NativeImageDetachPortV1;mountPoint:string;say:(line:string)=>void;check:()=>void;ready?:()=>boolean}>):Promise<void>{
 requireNativeStorageRootCapabilityV1(given.mountPoint,"detach-image");for(let attempt=0;;attempt++){
  given.check();
  try{await given.port.detach(given.mountPoint);return;}
  catch(error){
   given.check();
   if(!nativeImageDetachBusyV1(error))throw error;
   if(given.ready?.()===true)return;
   const holders=await given.port.holders(given.mountPoint).catch(()=>[] as readonly NativeImageHolderV1[]);given.check();
   if(attempt+1>=policy.maximumAttempts){const refusal=new Error(both(texts.exhausted,{mountPoint:given.mountPoint,attempts:String(policy.maximumAttempts),holders:describe(holders)}));(refusal as Error&{cause?:unknown}).cause=error;throw refusal;}
   const wait=nativeImageDetachBackoffMillisecondsV1(attempt);
   given.say(both(texts.waiting,{mountPoint:given.mountPoint,attempt:String(attempt+1),attempts:String(policy.maximumAttempts),seconds:String(wait/1000),holders:describe(holders)}));
   await given.port.sleep(wait);
  }
 }
}
/** 🍎️ The macOS port: `hdiutil detach <mount>` without force, `lsof -F pc +f -- <mount>` for the holders (an empty census is a normal answer, lsof exits 1 then) enriched with the shared process identity, and a wait that a cancellation ends. */
export function createNativeImageDetachPortV1(given:Readonly<{cwd:string;run:(program:string,args:readonly string[])=>Promise<string>;sleep:(milliseconds:number)=>Promise<void>;startOf?:(pid:number)=>number}>):NativeImageDetachPortV1{
 const startOf=given.startOf??(pid=>processIdentityOfV1(pid)?.startMs??0);
 return Object.freeze({
  detach:async mountPoint=>{await given.run(nativeImageHdiutilProgramV1,["detach",mountPoint]);},
  holders:async mountPoint=>{
   let text="";try{text=await given.run(policy.holderProgram,policy.holderArguments.map(part=>fill(part,{mountPoint})));}catch{return[];}
   return Object.freeze(parseNativeImageHoldersV1(text).map(row=>Object.freeze({...row,startMs:startOf(row.pid)})));
  },
  sleep:given.sleep
 });
}
/** 🛌️ Waits `milliseconds` and rejects with the cancellation of `check` as soon as it throws (polled every 250 ms), so a cancelled retry never sleeps out its back-off. */
export function nativeImageDetachSleepV1(check:()=>void,pollMilliseconds:number=250):(milliseconds:number)=>Promise<void>{
 return async milliseconds=>{for(let left=milliseconds;left>0;left-=pollMilliseconds){check();await new Promise<void>(resolve=>{const timer=setTimeout(resolve,Math.min(pollMilliseconds,left));timer.unref?.();});}check();};
}
