import {processRecordAliveV1} from "../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟨️.mjs"
import{readdirSync,readFileSync}from"node:fs";
import{basename,join}from"node:path";
import schema from"../🔐️custody/🧬️schema/🔣️.json"with{type:"json"};
const P=Object.freeze(structuredClone(schema.definitions.Policy.const)),M=schema.definitions.Messages.const,CLAIM=/^[a-f0-9]{24}\.json$/u,SIDE=/^([a-f0-9]{24})\.(progress|ask)\.json$/u,PHASE=/^[a-z]{1,40}$/u;
const closed=(value,keys,what)=>{if(value===null||typeof value!=="object"||Array.isArray(value)||Object.keys(value).length!==keys.length||keys.some(key=>!Object.hasOwn(value,key)))throw Object.assign(Error("Closed "+what+" required"),{});return value;};
const count=(value,what)=>{if(!Number.isSafeInteger(value)||value<0)throw Error("Non-negative integer required in "+what);return value;};
const fill=(template,fields)=>template.replace(/\{([a-z]+)\}/gu,(_token,name)=>String(fields[name]??""));
const alive=(pid,startedMs)=>processRecordAliveV1(pid,startedMs);
let remembered;
/** 🪧 Remembers the directory of the claim documents this process queues in, so a yield ticket can name it and the front door can read who is ahead without loading the custody. */
export function rememberDoorV1(directory){remembered=directory;}
/** 🪧 The directory remembered by the last custody queue of this process. */
export function rememberedDoorV1(){return remembered;}
/** 🪪 Admits a closed holder: pid, command (cut to the declared length), start time and the sha256 of the code it runs or an empty text. */
export function readHolderV1(value){
 closed(value,["pid","command","startedMs","digest"],"holder");
 if(!Number.isSafeInteger(value.pid)||value.pid<1||typeof value.command!=="string"||value.command.length>P.commandCharacters||typeof value.digest!=="string"||!/^(?:[a-f0-9]{64})?$/u.test(value.digest))throw Error("Closed holder required");
 return Object.freeze({pid:value.pid,command:value.command,startedMs:count(value.startedMs,"holder"),digest:value.digest});
}
/** 📈 Admits a closed progress document. */
export function readProgressV1(value){
 closed(value,["phase","done","total","advancedMs"],"progress");
 if(typeof value.phase!=="string"||!PHASE.test(value.phase))throw Error("Closed progress phase required");
 return Object.freeze({phase:value.phase,done:count(value.done,"progress"),total:count(value.total,"progress"),advancedMs:count(value.advancedMs,"progress")});
}
/** 🙏 Admits a closed request to hand over. */
export function readAskV1(value){
 closed(value,["version","reason","atMs","by"],"ask");
 if(value.version!==1||!["stalled","stale-code"].includes(value.reason))throw Error("Closed ask required");
 closed(value.by,["pid","digest"],"ask");
 return Object.freeze({version:1,reason:value.reason,atMs:count(value.atMs,"ask"),by:Object.freeze({pid:count(value.by.pid,"ask"),digest:String(value.by.digest)})});
}
/** 🪪 The holder document of this process: its pid, its command (script by name, then arguments), when it started and the code digest it was given. */
export function currentHolderV1({digest="",pid=process.pid,argv=process.argv,uptimeSeconds=process.uptime(),now=Date.now}={}){
 const command=argv.slice(1).map((argument,at)=>at===0?basename(argument):argument).join(" ").slice(0,P.commandCharacters);
 return readHolderV1({pid,command,startedMs:Math.max(0,Math.round(now()-uptimeSeconds*1000)),digest});
}
/** 📈 The progress after one event: it ADVANCES (moves `advancedMs` to now) when the phase changed or more owners are done; counters that stand still do not. */
export function advanceProgressV1(previous,event,now){
 const phase=typeof event.phase==="string"&&PHASE.test(event.phase)?event.phase:previous?.phase??"start",done=Number.isSafeInteger(event.done)&&event.done>=0?event.done:previous?.done??0,total=Number.isSafeInteger(event.total)&&event.total>=0?event.total:previous?.total??0;
 const moved=previous===undefined||phase!==previous.phase||done>previous.done;
 return Object.freeze({phase,done,total,advancedMs:moved?now:previous.advancedMs});
}
/** ⏱ The milliseconds since the last advance of a claim; undefined when it never reported (a holder of older code cannot be judged). */
export function idleMillisecondsV1(claim,now){return claim.progress?Math.max(0,now-claim.progress.advancedMs):undefined;}
/** ⚖ Why a held claim is asked to hand over: `stale-code` when its code digest differs from the current library, `stalled` when it did not advance for the declared time; nothing when it is asked already. */
export function askReasonV1({claim,now,current}){
 if(claim.state!=="held"||claim.ask!==undefined)return undefined;
 const digest=claim.holder?.digest;
 if(current&&digest&&digest!==current)return"stale-code";
 const idle=idleMillisecondsV1(claim,now);
 return idle!==undefined&&idle>=P.stallMilliseconds?"stalled":undefined;
}
/** 🚪 Whether a claim lost the door: it was asked and did not release for the declared grace. */
export function isDeposedV1(claim,now){return claim.ask!==undefined&&now-claim.ask.atMs>=P.askGraceMilliseconds;}
/** 🧾 The text about one holder in a language: pid, command, start, code, phase with counters and the time without progress. */
export function holderTextV1(claim,now,language){
 if(!claim.holder)return"";
 const idle=idleMillisecondsV1(claim,now),progress=claim.progress;
 return fill((M[language]??M.en).holder,{pid:claim.holder.pid,command:claim.holder.command,started:new Date(claim.holder.startedMs).toISOString().slice(0,19).replace("T"," "),digest:claim.holder.digest?claim.holder.digest.slice(0,12):"?",phase:progress?.phase??"start",done:progress?.done??0,total:progress?.total??0,idle:idle===undefined?"?":Math.round(idle/1000)});
}
/** 🧾 The same as a suffix of a waiting line; empty when the holder is not known. */
export function holderSuffixV1(claim,now,language){const text=holderTextV1(claim,now,language);return text===""?"":fill((M[language]??M.en).holderSuffix,{holder:text});}
const readJson=path=>{try{return JSON.parse(readFileSync(path,"utf8"));}catch{return undefined;}};
/** 👀 The claims that hold the door of `directory`, read without the custody: only held claims whose process lives, with their progress and ask, in ticket order. */
export function readDoorV1(directory,{isAlive=alive}={}){
 let names;try{names=readdirSync(directory);}catch{return[];}
 const side=new Set(names.filter(name=>SIDE.test(name))),rows=[];
 for(const name of names){
  if(!CLAIM.test(name))continue;
  const claim=readJson(join(directory,name));
  if(claim===undefined||typeof claim.token!=="string"||claim.state!=="held"||!Number.isSafeInteger(claim.ticket))continue;
  const row={token:claim.token,ticket:claim.ticket,state:"held",structural:claim.structural===true,owners:Array.isArray(claim.owners)?claim.owners:[]};
  try{if(claim.holder!==undefined)row.holder=readHolderV1(claim.holder);}catch{}
  for(const[kind,read]of[["progress",readProgressV1],["ask",readAskV1]]){if(!side.has(claim.token+"."+kind+".json"))continue;try{row[kind]=read(readJson(join(directory,claim.token+"."+kind+".json")));}catch{}}
  if(row.holder&&!isAlive(row.holder.pid,row.holder.startedMs))continue;
  rows.push(row);
 }
 return rows.sort((a,b)=>a.ticket-b.ticket);
}
/** 🪧 What the front door says about the holders of `directory`: at most `limit` holders, one text each, in a language; empty when nobody holds it or the door is unknown. */
export function doorSuffixV1(directory,{now=Date.now(),language="en",limit=3,isAlive}={}){
 if(typeof directory!=="string"||directory==="")return"";
 const texts=readDoorV1(directory,isAlive?{isAlive}:{}).filter(row=>row.holder).slice(0,limit).map(row=>holderTextV1(row,now,language));
 return texts.length?fill((M[language]??M.en).holderSuffix,{holder:texts.join(" | ")}):"";
}
