if(workerData.mode==="big"){const{parentPort}=await import("../../🏗️composition/🌐️host/👷️slice-pool/🧵️process/🟨️.mjs");parentPort.postMessage({type:"done",graph:{projects:Object.fromEntries(Array.from({length:60000},(_,i)=>["p"+i,{root:"libs/p"+i,targets:["build","test"]}]))},generationPin:"big"});await new Promise(resolve=>setTimeout(resolve,200));process.exit(0);}
import {spawn} from "node:child_process";
import {parentPort,workerData} from "../../🏗️composition/🌐️host/👷️slice-pool/🧵️process/🟨️.mjs";
const reported=workerData.mode==="reported",grand="setInterval(()=>{},1000)",script=reported?`const {spawn}=require("node:child_process"),g=spawn(process.execPath,["-e","${grand}"],{stdio:"ignore"});require("node:fs").writeFileSync(${JSON.stringify(workerData.pidFile??"")},String(g.pid));setInterval(()=>{},1000);`:grand;
const child=spawn(process.execPath,["-e",script],{stdio:"ignore",detached:reported,...reported?{env:{PATH:process.env.PATH??""}}:{}});
if(reported)parentPort.postMessage({type:"child",pid:child.pid});
parentPort.postMessage({type:"progress",phase:"child",done:child.pid,total:process.pid});
if(workerData.mode==="done")setTimeout(()=>parentPort.postMessage({type:"done",graph:{}}),300);
else setInterval(()=>{},1000);
if(workerData.mode==="busy"){process.on("SIGINT",()=>{});setTimeout(()=>{for(;;);},400);}
