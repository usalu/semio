import{readFileSync,renameSync,rmSync,writeFileSync}from"node:fs";
import{setTimeout as delay}from"node:timers/promises";
import policyDocument from"./🔣️.json"with{type:"json"};
import{doorSuffixV1,readDoorV1,rememberedDoorV1}from"../🪧️door-board/🟨️.mjs";
import{churnLanguagesV1}from"../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
const policy=Object.freeze(JSON.parse(JSON.stringify(policyDocument))),names=policy.environment;
const render=(key,values,environment)=>churnLanguagesV1(environment).map(language=>policy.texts[key][language].replace(/\{([a-z]+)\}/gu,(_,name)=>String(typeof values[name]==="function"?values[name](language):values[name]??""))).join("\n");
const defaultWrite=text=>{try{process.stderr.write(text+"\n");}catch{}};
const pause=(milliseconds,signal)=>delay(milliseconds,undefined,{signal}).catch(error=>{signal?.throwIfAborted();throw error;});
/** 📜️ The closed numbers, variable names and EN/DE texts of the graph queue yield, exactly as the policy declares them. */
export function graphQueueYieldPolicyV1(){return policy;}
/** 🕰️ The time the first start of this command asked for the graph, carried through every start of it so a peer's coverage stamp keeps serving it; `undefined` for a first start. */
export function graphQueueRequestedAtV1(environment=process.env){const value=Number(environment[names.requestedAt]);return Number.isSafeInteger(value)&&value>0?value:undefined;}
/** 🔢️ How often this command has already yielded the graph queue. */
export function graphQueueYieldRoundV1(environment=process.env){const value=Number.parseInt(environment[names.round]??"0",10);return Number.isInteger(value)&&value>=0?value:0;}
/** ⚖️ Whether a command that has queued for `queuedMs` for the graph builder yields now: only a command whose front door offered a ticket, that yielded fewer than the maximum times and queued at least the declared time. */
export function graphQueueYieldsV1({queuedMs,environment=process.env,yieldAfterMilliseconds=policy.yieldAfterMilliseconds}){return typeof environment[names.ticket]==="string"&&environment[names.ticket]!==""&&graphQueueYieldRoundV1(environment)<policy.maximumYields&&queuedMs>=yieldAfterMilliseconds;}
/** 🛌️ Ends the waiting process: writes the ticket (generation that was newest, path of the pointer that advances with every publication, request time of the first start, the directory of the door it queued in) beside the path the front door named, tells the user in both languages who holds the door and exits with the declared restart code; the front door waits alone and starts the same command again. */
export function yieldGraphQueueV1({generation,pointer,requestedAt,queuedMs=0,environment=process.env,write=defaultWrite,exit=code=>process.exit(code),now=Date.now,door=rememberedDoorV1()}){
 const path=environment[names.ticket],round=graphQueueYieldRoundV1(environment);
 const body=JSON.stringify({version:1,generation,pointer,requestedAt,round,at:now(),...typeof door==="string"&&door!==""?{door}:{}}),temporary=path+"."+process.pid+".tmp";
 writeFileSync(temporary,body,{mode:0o600});renameSync(temporary,path);
 write(render("waiting",{seconds:Math.round(queuedMs/1000),generation,holder:language=>doorSuffixV1(door,{now:now(),language})},environment));
 exit(policy.exitCode);
}
function readTicket(path){
 try{const row=JSON.parse(readFileSync(path,"utf8"));rmSync(path,{force:true});return row&&row.version===1&&Number.isInteger(row.generation)&&typeof row.pointer==="string"&&Number.isSafeInteger(row.requestedAt)&&Number.isInteger(row.round)&&(row.door===undefined||typeof row.door==="string")?row:undefined;}
 catch{try{rmSync(path,{force:true});}catch{}return undefined;}
}
function readGeneration(pointer){try{const row=JSON.parse(readFileSync(pointer,"utf8"));return Number.isInteger(row.generation)?row.generation:-1;}catch{return -1;}}
/** ⏳️ Consumes a graph yield ticket, settles its wait preparation, then polls for a newer generation, for the door to lose its last live holder (read by pid and start time, so a holder that already ended is never waited for) or for the bounded wake. Carries the initial request time and next round to the replacement. */
export async function waitGraphQueueYieldV1({environment=process.env,now=Date.now,sleep=pause,write=defaultWrite,read=readGeneration,holders=readDoorV1,signal,onAccepted}={}){
 const path=environment[names.ticket];if(typeof path!=="string"||path==="")return undefined;
 const ticket=readTicket(path);if(!ticket)return undefined;
 if(onAccepted!==undefined)await onAccepted();
 const started=now(),watched=typeof ticket.door==="string"&&ticket.door!=="";let reported=started;
 for(;;){
  signal?.throwIfAborted();
  const generation=read(ticket.pointer),elapsed=now()-started;
  if(generation>ticket.generation||elapsed>=policy.wakeMilliseconds){write(render("resumed",{generation:Math.max(generation,ticket.generation),seconds:Math.round(elapsed/1000)},environment));break;}
  if(watched&&holders(ticket.door).length===0){write(render("holderGone",{seconds:Math.round(elapsed/1000)},environment));break;}
  if(now()-reported>=policy.progressMilliseconds){reported=now();const sinceRequest=now()-ticket.requestedAt;write(render("waiting",{seconds:Math.round(Math.max(elapsed,sinceRequest)/1000),generation:ticket.generation,holder:language=>doorSuffixV1(ticket.door,{now:now(),language})},environment));}
  await sleep(watched?Math.min(policy.pollMilliseconds,policy.doorPollMilliseconds):policy.pollMilliseconds,signal);
 }
 return{[names.requestedAt]:String(ticket.requestedAt),[names.round]:String(ticket.round+1)};
}
