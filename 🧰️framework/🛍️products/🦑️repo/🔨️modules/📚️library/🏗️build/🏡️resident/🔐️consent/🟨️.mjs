/** 🔐️ File access consent of the long-lived roles: macOS gives the right to read the Documents, Desktop and Downloads folders of a user to one identity, once per machine; this module says which checkouts need it, which identity it is given to, probes it without ever blocking, and keeps the receipts `repo preflight` reads. */
import{machinePathV1}from"../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {fork} from "node:child_process";
import {createHash} from "node:crypto";
import {mkdirSync,readdirSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {lstat,readdir} from "node:fs/promises";
import {homedir,uptime} from "node:os";
import {isAbsolute,join,relative,sep} from "node:path";
import {fileURLToPath} from "node:url";

/** 📁️ The folders of a user that macOS protects with a consent. */
export const PROTECTED_FOLDERS_V1=Object.freeze(["Documents","Desktop","Downloads"]);

const ROLE_V1=/^[a-z0-9][a-z0-9-]{0,39}$/u,STATES_V1=Object.freeze(["ok","denied","waiting","unprotected"]),DEFAULT_AGE_MILLISECONDS_V1=6*3600_000;

/** 🛣️ How a service process came to run: as a launchd job under the launcher, as a direct child of the calling process, or in the foreground of a terminal. */
export const SERVICE_ROUTES_V1=Object.freeze(["launchd","direct","foreground"]);
const ROUTES_V1=SERVICE_ROUTES_V1;

/** 🧭️ The protected folder a checkout lies below on macOS (`Documents`, `Desktop` or `Downloads`), `undefined` for every checkout that needs no consent (other systems, relative or foreign paths, the home folder itself). */
export function protectedFolderOfV1(root,{platform=process.platform,home=homedir()}={}){
 if(platform!=="darwin"||typeof root!=="string"||!isAbsolute(root))return undefined;
 const path=relative(home,root);
 if(path===""||path.startsWith("..")||isAbsolute(path))return undefined;
 const first=path.split(sep)[0];
 return PROTECTED_FOLDERS_V1.includes(first)?first:undefined;
}

/** 🪪️ The one identity the consent is given to: the launcher `repo` at its install location, the process that stays responsible after it replaces itself with the runtime, so a runtime update (a new path with a new hash) never asks again; `undefined` where no consent exists. */
export function consentIdentityV1({platform=process.platform,home=homedir(),environment=process.env}={}){
 if(platform!=="darwin")return undefined;
 return{name:"repo",path:join(environment.XDG_DATA_HOME||join(home,".local","share"),"semio","launcher","bin","repo")};
}

/** 🗂️ Where the receipts of one checkout live: a folder outside every protected folder (a receipt of a denied role could not be written into the checkout), one per checkout. */
export function accessDirectoryV1(root,{platform=process.platform,home=homedir(),environment=process.env}={}){
 const tag=createHash("sha256").update(String(root)).digest("hex").slice(0,12);
 if(platform==="darwin")return join(home,"Library","Application Support","semio","access",tag);
 if(platform==="win32")return join(environment.LOCALAPPDATA||join(home,"AppData","Local"),"semio","access",tag);
 return join(environment.XDG_STATE_HOME||join(home,".local","state"),"semio","access",tag);
}

/** 🚩️ The marker the cache image opening looks up in every image directory; a name that does not exist must answer `ENOENT`, an `EPERM` for it is the system keeping this process from the volume. */
export const RESET_MARKER_V1=".reset.json";

const refused=error=>error?.code==="EPERM"||error?.code==="EACCES",deniedByConsent=error=>error?.code==="EPERM";

/** 🔬️ Reads below the root with the identity of the calling process, the way the build does (`fstatat` on the top-level entries, a listing of the first directories down to two levels, and the cache images below `.cache` down to four levels, each with a lookup of the reset marker `.reset.json`, which most often does not exist: the build asks for it with `fstatat` and a volume the process may not enter answers `EPERM` even then): a place the system refuses with `EPERM` (the answer of the file access consent; `EACCES` below the root is an ordinary permission of the tree and no consent problem) on the way is `{place,error}`, everything else (a vanished entry included) is no access problem and gives `undefined`. At most `limit` entries are touched and `directories` listed, so the sample stays within a few milliseconds on a healthy tree. */
export async function sampleAccessV1(root,names,{list=readdir,stat=lstat,limit=400,directories=24}={}){
 let budget=limit,listings=directories;
 const queue=[],seen=new Set();
 const visit=async(parent,entries,depth,cache)=>{
  for(const name of depth===0?entries:entries.slice(0,60)){
   if(budget--<=0)return undefined;
   const path=join(parent,name);
   let info;
   try{info=await stat(path);}catch(error){if(deniedByConsent(error))return{place:path,error:error.code};continue;}
   if(!info?.isDirectory?.()||name==="node_modules"||name===".git"||seen.has(path)||(!cache&&depth>1))continue;
   seen.add(path);
   const item={path,depth:depth+1,cache:cache||(name===".cache"&&depth===0)};
   if(item.cache&&name===".cache")queue.unshift(item);else queue.push(item);
  }
  return undefined;
 };
 const top=await visit(root,names,0,false);
 if(top)return top;
 while(queue.length>0&&listings>0){
  const item=queue.shift();
  if(item.depth>4)continue;
  listings--;
  let entries;
  try{entries=await list(item.path);}catch(error){if(deniedByConsent(error))return{place:item.path,error:error.code};continue;}
  if(!Array.isArray(entries))continue;
  const found=await visit(item.path,item.cache?[RESET_MARKER_V1,...entries.filter(name=>name!==RESET_MARKER_V1)]:entries,item.depth,item.cache);
  if(found)return found;
 }
 return undefined;
}

/** 🧒️ Runs the sample in a forked, detached child of this process (the shape of a slice load) and answers its `{place,error}`, or `undefined` when nothing was refused, the child did not answer within `timeoutMilliseconds` (a slow start under load is no refusal) or could not be started; a service passes the gate only when the process that does its work can read what it reads. */
export function sampleAccessInChildV1(root,{timeoutMilliseconds=8000,spawn=fork,script=fileURLToPath(new URL("./🔎️sample/🟨️.mjs",import.meta.url)),environment=process.env}={}){
 return new Promise(done=>{
  let child,timer,settled=false;
  const finish=value=>{if(settled)return;settled=true;clearTimeout(timer);try{child?.kill?.("SIGKILL");}catch{}done(value);};
  try{child=spawn(script,[root],{execPath:process.execPath,env:environment,detached:process.platform!=="win32",windowsHide:true,stdio:["ignore","ignore","ignore","ipc"]});}catch{finish(undefined);return;}
  child.once("message",message=>finish(typeof message?.place==="string"?{place:message.place,error:String(message.error??"EPERM")}:undefined));
  child.once("error",()=>finish(undefined));
  child.once("exit",()=>{setTimeout(()=>finish(undefined),100);});
  timer=setTimeout(()=>finish(undefined),timeoutMilliseconds);
 });
}

/** 🚪️ Probes whether a checkout can be read and never blocks: a refusal of the root (`EPERM`, `EACCES`) or an `EPERM` of anything the sample below it touches (`sampleAccessV1`) is `denied` and names the `place`, a call that has not returned within `timeoutMilliseconds` is `waiting` (a consent dialog waits for an answer; the call stays pending in the background and is never started twice by the caller), a checkout that needs no consent is `unprotected` and is not touched, any other outcome (a missing directory included) is `ok` because it is no access problem. */
export async function probeConsentV1(root,{platform=process.platform,home=homedir(),timeoutMilliseconds=8000,list=readdir,stat=lstat,limit}={}){
 const folder=protectedFolderOfV1(root,{platform,home});
 if(folder===undefined)return{state:"unprotected",folder:""};
 let timer;
 const listing=(async()=>{
  let names;
  try{names=await list(root);}catch(error){return refused(error)?{state:"denied",folder,error:error.code,place:root}:{state:"ok",folder};}
  try{
   const found=Array.isArray(names)?await sampleAccessV1(root,names,{list,stat,...limit===undefined?{}:{limit}}):undefined;
   return found?{state:"denied",folder,error:found.error,place:found.place}:{state:"ok",folder};
  }catch{return{state:"ok",folder};}
 })(),
  limitation=new Promise(done=>{timer=setTimeout(()=>done({state:"waiting",folder,error:"no answer"}),timeoutMilliseconds);});
 try{return await Promise.race([listing,limitation]);}finally{clearTimeout(timer);}
}

const shaped=value=>value!==null&&typeof value==="object"&&value.version===1&&typeof value.role==="string"&&ROLE_V1.test(value.role)&&Number.isInteger(value.pid)&&value.pid>0&&STATES_V1.includes(value.state)&&typeof value.root==="string"&&value.root.length>0&&typeof value.folder==="string"&&typeof value.identity==="string"&&Number.isInteger(value.at)&&value.at>=0&&(value.error===undefined||typeof value.error==="string")&&(value.launcherSha256===undefined||/^[0-9a-f]{64}$/u.test(value.launcherSha256))&&(value.place===undefined||typeof value.place==="string")&&(value.route===undefined||ROUTES_V1.includes(value.route))&&(value.ppid===undefined||Number.isInteger(value.ppid)&&value.ppid>=0)&&Object.keys(value).every(key=>["version","role","pid","state","root","folder","identity","at","error","launcherSha256","place","route","ppid"].includes(key));

/** ✍️ Writes the receipt of one role atomically (`<role>.json` below `directory`, owner only); a receipt that does not fit `🧬️schema` is refused and nothing is written. */
export function writeAccessReceiptV1(directory,receipt){
 if(!shaped(receipt))throw Error("Declared access receipt required: "+JSON.stringify(receipt)+"\nErklärter Zugriffsbeleg erforderlich");
 mkdirSync(directory,{recursive:true,mode:0o700});
 const file=join(directory,receipt.role+".json"),temporary=file+"."+process.pid+".tmp";
 try{writeFileSync(temporary,JSON.stringify(receipt),{mode:0o600});renameSync(temporary,file);}catch(error){rmSync(temporary,{force:true});throw error;}
}

/** 📖️ The valid receipts below `directory`; a missing folder, a damaged file and a file that does not fit the schema are no receipts. */
export function readAccessReceiptsV1(directory){
 let names;
 try{names=readdirSync(directory);}catch{return[];}
 return names.filter(name=>name.endsWith(".json")).flatMap(name=>{
  try{const value=JSON.parse(readFileSync(join(directory,name),"utf8"));return shaped(value)&&name===value.role+".json"?[value]:[];}catch{return[];}
 }).sort((left,right)=>left.role<right.role?-1:1);
}

/** 🩺️ The receipts of the roles that lack the consent: denied or waiting and not older than `maximumAgeMilliseconds` (a role writes `ok` over its denial the next time it starts, so a recent denial is the current fact). */
export function accessReceiptsLackingV1(receipts,{now=Date.now(),maximumAgeMilliseconds=DEFAULT_AGE_MILLISECONDS_V1,launcherSha256}={}){
 return receipts.filter(receipt=>(receipt.state==="denied"||receipt.state==="waiting")&&now-receipt.at<=maximumAgeMilliseconds&&(launcherSha256===undefined||receipt.launcherSha256===undefined||receipt.launcherSha256===launcherSha256)).sort((left,right)=>left.role<right.role?-1:1);
}

/** 🙋️ The typed refusal of a probe that found no access: code `consent` (exit status 4, the consent code of the provision script), the message key and its fields; a probe that found access is no refusal. */
export function consentRefusalV1(probe,{root,identity}){
 if(probe.state!=="denied"&&probe.state!=="waiting")throw Error("A denied or waiting probe is required\nEine verweigerte oder wartende Prüfung ist erforderlich");
 return{code:"consent",key:probe.state==="denied"?"consent-denied":"consent-waiting",fields:{folder:probe.folder,root,place:probe.place??root,identity:identity?.path??"repo"},retry:"later"};
}

/** 🧬️ The sha-256 of the launcher file (the consent is bound to its bytes), `undefined` when it cannot be read. */
export function launcherDigestV1(path){
 try{return createHash("sha256").update(readFileSync(path)).digest("hex");}catch{return undefined;}
}

/** 🏁️ Where the proof lies that the launcher was allowed to read a checkout: `launcher.json` next to the receipts of that checkout. */
export function launcherMarkerPathV1(root,options){return join(accessDirectoryV1(root,options),"launcher.json");}

/** ✅️ What the marker says about the launcher: `consented` (its sha-256 is that of the launcher file as it is now), `changed` (a marker of an earlier launcher: the launcher was replaced by other bytes and macOS asks once more), `none` (never proved here). */
export function launcherStateV1(root,{platform=process.platform,home=homedir(),environment=process.env,digest=launcherDigestV1}={}){
 const identity=consentIdentityV1({platform,home,environment});
 if(identity===undefined)return"none";
 try{
  const marker=JSON.parse(readFileSync(launcherMarkerPathV1(root,{platform,home,environment}),"utf8")),current=digest(identity.path);
  if(marker?.version!==1||typeof marker.sha256!=="string"||current===undefined||marker.root!==root)return"none";
  return marker.sha256===current?"consented":"changed";
 }catch{return"none";}
}

/** ✅️ Whether the launcher holds the consent for a checkout: its marker names the sha-256 of the launcher file as it is now (a rebuilt launcher with other bytes has lost the consent and must ask again). */
export function launcherConsentedV1(root,options={}){return launcherStateV1(root,options)==="consented";}

/** ✍️ Writes the marker after the launcher listed the checkout (run by the probe job under the launcher identity). */
export function writeLauncherMarkerV1(root,{platform=process.platform,home=homedir(),environment=process.env,digest=launcherDigestV1,now=Date.now}={}){
 const identity=consentIdentityV1({platform,home,environment}),sha256=identity&&digest(identity.path);
 if(sha256===undefined)throw Error("The launcher is not readable\nDer Launcher ist nicht lesbar");
 const file=launcherMarkerPathV1(root,{platform,home,environment}),temporary=file+"."+process.pid+".tmp";
 mkdirSync(join(file,".."),{recursive:true,mode:0o700});
 writeFileSync(temporary,JSON.stringify({version:1,sha256,root,at:now()}),{mode:0o600});renameSync(temporary,file);
}

/** 🗑️ Forgets the proof that the launcher may read a checkout (the service under the launcher was denied after all): the next service starts under the access of the caller until the probe proves the launcher again. */
export function invalidateLauncherMarkerV1(root,options){
 try{rmSync(launcherMarkerPathV1(root,options),{force:true});}catch{}
}

const REFUSAL_FILE_V1="launchd-refused.json",BOOT_TOLERANCE_V1=60_000;

/** ⏳️ How long a remembered refusal of the launchd route holds: afterwards the next service start probes the route again (a start that is refused again costs one failed start and is remembered for another window), so a consent granted later, a volume that came back or a launcher that was rebuilt is found without a reboot. */
export const LAUNCHD_REFUSAL_RETRY_MILLISECONDS_V1=10*60_000;

/** 🧠️ Remembers for this boot that the launchd route was refused for this checkout (`place`: the volume or entry it was refused at), bound to the launcher bytes: the next services start directly under the caller at once instead of paying a failed start each time. */
export function rememberLaunchdRefusalV1(root,{place="",platform=process.platform,home=homedir(),environment=process.env,digest=launcherDigestV1,now=Date.now}={}){
 const identity=consentIdentityV1({platform,home,environment}),sha256=identity&&digest(identity.path),directory=accessDirectoryV1(root,{platform,home,environment}),file=join(directory,REFUSAL_FILE_V1),temporary=file+"."+process.pid+".tmp";
 try{mkdirSync(directory,{recursive:true,mode:0o700});writeFileSync(temporary,JSON.stringify({version:1,root,place:String(place),at:now(),...sha256?{launcherSha256:sha256}:{}}),{mode:0o600});renameSync(temporary,file);}catch{rmSync(temporary,{force:true});}
}

/** 🧠️ The refusal of the launchd route remembered for this checkout, or `undefined`: only a record written after the machine booted, for the same checkout and the same launcher bytes, not older than `LAUNCHD_REFUSAL_RETRY_MILLISECONDS_V1` (then the route is probed again). */
export function launchdRefusalV1(root,{platform=process.platform,home=homedir(),environment=process.env,digest=launcherDigestV1,now=Date.now,uptimeSeconds=uptime}={}){
 if(protectedFolderOfV1(root,{platform,home})===undefined)return undefined;
 try{
  const value=JSON.parse(readFileSync(join(accessDirectoryV1(root,{platform,home,environment}),REFUSAL_FILE_V1),"utf8")),identity=consentIdentityV1({platform,home,environment}),current=identity&&digest(identity.path),at=now();
  if(value?.version!==1||value.root!==root||!Number.isInteger(value.at)||typeof value.place!=="string")return undefined;
  if(value.at<at-uptimeSeconds()*1000-BOOT_TOLERANCE_V1||at-value.at>LAUNCHD_REFUSAL_RETRY_MILLISECONDS_V1||value.at>at+BOOT_TOLERANCE_V1)return undefined;
  if(value.launcherSha256!==current)return undefined;
  return{place:value.place,at:value.at};
 }catch{return undefined;}
}

/** 🗑️ Forgets the remembered refusal (the launcher changed or the owner asks for the launchd route again). */
export function forgetLaunchdRefusalV1(root,{platform=process.platform,home=homedir(),environment=process.env}={}){
 try{rmSync(join(accessDirectoryV1(root,{platform,home,environment}),REFUSAL_FILE_V1),{force:true});}catch{}
}

/** 🔢️ The version of the access proof a service runs: 1 the root only, 2 a sample below the root and in a forked child, 3 the reset marker of every cache image directory and the proof repeated when a mount appears after the start. Logged at every service start (`access-proof version=…`) so a report settles which code a running service has. */
export const ACCESS_PROOF_VERSION_V1=3;

/** 🏔️ The device ids of the root, `.cache`, `.cache/b` and the directories below `.cache/b` (a mounted cache image has another device than the directory it covers; a place the system refuses shows its error code instead): one string that changes when an image is attached or detached or when a place becomes unreadable. */
export async function mountSignatureV1(root,{stat=lstat,list=readdir}={}){
 const places=[root,machinePathV1(root,".cache"),machinePathV1(root,".cache","b")];
 try{const children=await list(machinePathV1(root,".cache","b"));if(Array.isArray(children))for(const name of children.slice(0,16))places.push(machinePathV1(root,".cache","b",name));}catch{}
 const out=[];
 for(const place of places){try{out.push(place+":"+String((await stat(place)).dev));}catch(error){out.push(place+":"+(error?.code??"?"));}}
 return out.join("|");
}

/** 🔁️ The proof repeated when a mount appears after the start: `prime()` records the signature the start-time proof saw, `check()` compares it with the present one and, only when it changed, runs the whole proof again (the service's own probe and the forked child) and answers `{changed,found}` with `found` = `{place,error,by}` of a refusal or `undefined`. A `waiting` answer is no refusal here (a request never blocks on a dialog). */
export function createMountReproofV1({root,probe,childProbe,signature=mountSignatureV1}){
 let known;
 return{
  async prime(){known=await signature(root);},
  async check(){
   const now=await signature(root);
   if(now===known)return{changed:false,found:undefined};
   known=now;
   const first=await probe(root);
   if(first.state==="denied")return{changed:true,found:{place:first.place??root,error:first.error??"EPERM",by:"service"}};
   const child=first.state==="ok"&&childProbe?await childProbe(root):undefined;
   return{changed:true,found:child?{place:child.place,error:child.error,by:"worker"}:undefined};
  },
 };
}
