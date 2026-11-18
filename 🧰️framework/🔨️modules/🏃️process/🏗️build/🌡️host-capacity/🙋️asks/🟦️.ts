import{mkdirSync,readFileSync,readdirSync,renameSync,rmSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 📐️ The declared numbers of the asks: directory name, heartbeat of a writer, missed beats after which an ask is stale.
 * @see ./🧬️schema/🔣️.json */
export const HOST_ASK_POLICY_V1=schema.definitions.Policy.const;
const P=HOST_ASK_POLICY_V1;

/** 🙋️ One claim of a read on the one budget of its machine: what its first waiting owner needs (`alone`: the owner is one the cost table says needs the whole machine, claimed from the first request until it ended), since when, and the heartbeat it promises.
 * @see ./🧬️schema/🔣️.json */
export type HostAskV1=Readonly<{version:1;pid:number;key:string;needMegabytes:number;alone:boolean;sinceMs:number;updatedMs:number;heartbeatMilliseconds:number}>;

/** 🗂️ The directory of the asks inside the gate directory of the machine. */
export const hostAskDirectoryV1=(gateDirectory:string):string=>join(gateDirectory,P.directory);

const processAlive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return(error as NodeJS.ErrnoException).code==="EPERM";}};

/** 🫀️ Whether an ask still stands: its process lives and it was rewritten within `missedBeats` of its own heartbeat (a dead process whose number another process took cannot keep a claim alive). */
export const hostAskLiveV1=(ask:HostAskV1,nowMs:number,alive:(pid:number)=>boolean=processAlive,policy:typeof P=P):boolean=>nowMs-ask.updatedMs<=ask.heartbeatMilliseconds*policy.missedBeats&&alive(ask.pid);

const shaped=(value:unknown):value is HostAskV1=>{const row=value as Partial<HostAskV1>|null;return row!==null&&typeof row==="object"&&row.version===1&&Number.isInteger(row.pid)&&row.pid!>0&&typeof row.key==="string"&&row.key.length>0&&Number.isInteger(row.needMegabytes)&&row.needMegabytes!>=0&&typeof row.alone==="boolean"&&Number.isInteger(row.sinceMs)&&Number.isInteger(row.updatedMs)&&Number.isInteger(row.heartbeatMilliseconds)&&row.heartbeatMilliseconds!>0;};

/** 📖️ Reads the live asks of the machine; a missing directory, an unreadable or misshapen file and a stale ask count for nothing (a broken ask must never hold the pool).
 * @see ./🧪️tests/🟦️.ts */
export function readHostAsksV1(gateDirectory:string,nowMs:number=Date.now(),alive:(pid:number)=>boolean=processAlive):readonly HostAskV1[]{
 const directory=hostAskDirectoryV1(gateDirectory),rows:HostAskV1[]=[];
 let names:string[];try{names=readdirSync(directory);}catch{return rows;}
 for(const name of names){
  if(!name.endsWith(".json"))continue;
  try{const text=readFileSync(join(directory,name),"utf8");if(text.length>P.maximumFileBytes)continue;const value=JSON.parse(text) as unknown;if(shaped(value)&&hostAskLiveV1(value,nowMs,alive))rows.push(value);}catch{}
 }
 return rows;
}

/** 🧮️ The claim of all live asks on the one budget: the megabytes they need together, `alone` when any of them needs the whole machine; `undefined` when nobody asks.
 * @see ./🧪️tests/🟦️.ts */
export const hostAskClaimV1=(asks:readonly HostAskV1[]):Readonly<{megabytes:number;alone:boolean}>|undefined=>asks.length===0?undefined:Object.freeze({megabytes:asks.reduce((sum,ask)=>sum+ask.needMegabytes,0),alone:asks.some(ask=>ask.alone)});

/** ✍️ The writer of one process: `set` publishes (or keeps alive) the claim, `clear` withdraws it; a file is written by temporary name and rename, rewritten only when the claim changed or the heartbeat is due, and every failure is swallowed (the read must never fail because of its claim).
 * @see ./🧪️tests/🟦️.ts */
export function createHostAskV1(input:Readonly<{directory:string;pid?:number;now?:()=>number;policy?:typeof P}>):Readonly<{set(row:Readonly<{key:string;needMegabytes:number;alone:boolean}>):void;clear():void}>{
 const policy=input.policy??P,pid=input.pid??process.pid,now=input.now??Date.now,directory=hostAskDirectoryV1(input.directory),path=join(directory,`${pid}.json`);
 let last:HostAskV1|undefined;
 return Object.freeze({
  set(row){
   try{
    const at=now();
    if(last!==undefined&&last.key===row.key&&last.needMegabytes===row.needMegabytes&&last.alone===row.alone&&at-last.updatedMs<policy.heartbeatMilliseconds)return;
    const ask:HostAskV1={version:1,pid,key:row.key.slice(0,512),needMegabytes:Math.max(0,Math.ceil(row.needMegabytes)),alone:row.alone,sinceMs:last!==undefined&&last.key===row.key&&last.alone===row.alone?last.sinceMs:at,updatedMs:at,heartbeatMilliseconds:policy.heartbeatMilliseconds};
    mkdirSync(directory,{recursive:true});const temporary=`${path}.${pid}.tmp`;writeFileSync(temporary,JSON.stringify(ask));renameSync(temporary,path);last=ask;
   }catch{}
  },
  clear(){last=undefined;try{rmSync(path,{force:true});}catch{}},
 });
}
