import{machinePathV1}from"../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import{mkdirSync,readFileSync,renameSync,rmSync,writeFileSync}from"node:fs";
import{dirname,join}from"node:path";
import{OWNER_POOL_POLICY_V1 as P}from"../🟦️.ts";

/** 🔒️ One pool process per machine: the pid file `<root>/.cache/f/owner-pool.pid` names the process that pulls; a starting process that finds another live one waits for it (bounded) and, when it is still alive afterwards, does not pull, so a restart that left an old process behind can never double the memory. `alive` is the liveness test (a signal 0 by default), `sleep` the wait. */
export type OwnerPoolGuardV1=Readonly<{acquired:boolean;sibling:number|undefined;release():void}>;
const defaultAlive=(pid:number)=>{try{process.kill(pid,0);return true;}catch(error){return(error as NodeJS.ErrnoException).code==="EPERM";}};
const defaultSleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const readPid=(file:string):number|undefined=>{try{const value=JSON.parse(readFileSync(file,"utf8")) as {pid?:number};return Number.isInteger(value.pid)?value.pid:undefined;}catch{return undefined;}};

/** 🔒️ Takes the pool of this machine: see {@link OwnerPoolGuardV1}. */
export async function acquireOwnerPoolGuardV1(input:Readonly<{root:string;pid?:number;alive?:(pid:number)=>boolean;sleep?:(milliseconds:number)=>Promise<void>;say?:(pid:number)=>void;waitMilliseconds?:number;pollMilliseconds?:number}>):Promise<OwnerPoolGuardV1>{
 const file=machinePathV1(input.root,".cache","f",P.guardFile),self=input.pid??process.pid,alive=input.alive??defaultAlive,sleep=input.sleep??defaultSleep,poll=input.pollMilliseconds??P.guardPollMilliseconds;
 let waited=0,other=readPid(file);
 while(other!==undefined&&other!==self&&alive(other)){
  if(waited===0)input.say?.(other);
  if(waited>=(input.waitMilliseconds??P.guardWaitMilliseconds))return{acquired:false,sibling:other,release(){}};
  await sleep(poll);waited+=poll;other=readPid(file);
 }
 try{mkdirSync(dirname(file),{recursive:true});const temporary=file+"."+self+".tmp";writeFileSync(temporary,JSON.stringify({pid:self,startedMs:Date.now()}));renameSync(temporary,file);}catch{}
 return{acquired:true,sibling:other===self?undefined:other,release(){try{if(readPid(file)===self)rmSync(file,{force:true});}catch{}}};
}
