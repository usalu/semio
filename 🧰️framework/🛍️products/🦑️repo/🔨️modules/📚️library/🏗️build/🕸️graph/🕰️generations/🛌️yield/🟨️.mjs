import{readFileSync,renameSync,rmSync,writeFileSync}from"node:fs";
import policyDocument from"./🔣️.json"with{type:"json"};
import{churnLanguagesV1}from"../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
const policy=Object.freeze(JSON.parse(JSON.stringify(policyDocument))),names=policy.environment;
const render=(key,values,environment)=>churnLanguagesV1(environment).map(language=>policy.texts[key][language].replace(/\{([a-z]+)\}/gu,(_,name)=>String(values[name]))).join("\n");
const defaultWrite=text=>{try{process.stderr.write(text+"\n");}catch{}};
const pause=milliseconds=>new Promise(accept=>setTimeout(accept,milliseconds));
/** 📜️ The closed numbers, variable names and EN/DE texts of the graph queue yield, exactly as the policy declares them. */
export function graphQueueYieldPolicyV1(){return policy;}
/** 🕰️ The time the first start of this command asked for the graph, carried through every start of it so a peer's coverage stamp keeps serving it; `undefined` for a first start. */
export function graphQueueRequestedAtV1(environment=process.env){const value=Number(environment[names.requestedAt]);return Number.isSafeInteger(value)&&value>0?value:undefined;}
/** 🔢️ How often this command has already yielded the graph queue. */
export function graphQueueYieldRoundV1(environment=process.env){const value=Number.parseInt(environment[names.round]??"0",10);return Number.isInteger(value)&&value>=0?value:0;}
/** ⚖️ Whether a command that has queued for `queuedMs` for the graph builder yields now: only a command whose front door offered a ticket, that yielded fewer than the maximum times and queued at least the declared time. */
export function graphQueueYieldsV1({queuedMs,environment=process.env,yieldAfterMilliseconds=policy.yieldAfterMilliseconds}){return typeof environment[names.ticket]==="string"&&environment[names.ticket]!==""&&graphQueueYieldRoundV1(environment)<policy.maximumYields&&queuedMs>=yieldAfterMilliseconds;}
/** 🛌️ Ends the waiting process: writes the ticket (generation that was newest, path of the pointer that advances with every publication, request time of the first start) beside the path the front door named, tells the user in both languages and exits with the declared restart code; the front door waits alone and starts the same command again. */
export function yieldGraphQueueV1({generation,pointer,requestedAt,queuedMs=0,environment=process.env,write=defaultWrite,exit=code=>process.exit(code),now=Date.now}){
 const path=environment[names.ticket],round=graphQueueYieldRoundV1(environment);
 const body=JSON.stringify({version:1,generation,pointer,requestedAt,round,at:now()}),temporary=path+"."+process.pid+".tmp";
 writeFileSync(temporary,body,{mode:0o600});renameSync(temporary,path);
 write(render("waiting",{seconds:Math.round(queuedMs/1000),generation},environment));
 exit(policy.exitCode);
}
function readTicket(path){
 try{const row=JSON.parse(readFileSync(path,"utf8"));rmSync(path,{force:true});return row&&row.version===1&&Number.isInteger(row.generation)&&typeof row.pointer==="string"&&Number.isSafeInteger(row.requestedAt)&&Number.isInteger(row.round)?row:undefined;}
 catch{try{rmSync(path,{force:true});}catch{}return undefined;}
}
function readGeneration(pointer){try{const row=JSON.parse(readFileSync(pointer,"utf8"));return Number.isInteger(row.generation)?row.generation:-1;}catch{return -1;}}
/** ⏳️ The front door's wait after a yielded run, alone and without a child: nothing when no ticket was written; otherwise it polls the pointer until a generation newer than the ticket's was published or `wakeMilliseconds` passed (progress every `progressMilliseconds`, EN/DE), and answers the environment of the next start: the request time of the first start and the next round. */
export async function waitGraphQueueYieldV1({environment=process.env,now=Date.now,sleep=pause,write=defaultWrite,read=readGeneration,signal}={}){
 const path=environment[names.ticket];if(typeof path!=="string"||path==="")return undefined;
 const ticket=readTicket(path);if(!ticket)return undefined;
 const started=now();let reported=started;
 for(;;){
  signal?.throwIfAborted();
  const generation=read(ticket.pointer),elapsed=now()-started;
  if(generation>ticket.generation||elapsed>=policy.wakeMilliseconds){write(render("resumed",{generation:Math.max(generation,ticket.generation),seconds:Math.round(elapsed/1000)},environment));break;}
  if(now()-reported>=policy.progressMilliseconds){reported=now();write(render("waiting",{seconds:Math.round(elapsed/1000),generation:ticket.generation},environment));}
  await sleep(policy.pollMilliseconds,signal);
 }
 return{[names.requestedAt]:String(ticket.requestedAt),[names.round]:String(ticket.round+1)};
}
