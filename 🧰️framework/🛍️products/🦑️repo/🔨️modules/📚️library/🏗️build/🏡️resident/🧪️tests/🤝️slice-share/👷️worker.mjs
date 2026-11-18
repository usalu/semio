/** 👷 A slice worker of the share laws: counts its start in a file, keeps a detached grandchild alive, reports `taken N of M` and ends after M steps.
 * @see ../🟦️.ts */
import {spawn} from "node:child_process";
import {appendFileSync,writeFileSync} from "node:fs";
import {parentPort,workerData} from "../../🏗️composition/🌐️host/👷️slice-pool/🧵️process/🟨️.mjs";
appendFileSync(workerData.counter,workerData.name+"\n");
const grand=spawn(process.execPath,["-e","setInterval(()=>{},1000)"],{stdio:"ignore",detached:true});
writeFileSync(workerData.pidFile,String(grand.pid));
parentPort.postMessage({type:"child",pid:grand.pid});
let taken=0;
const beat=setInterval(()=>{
 taken++;parentPort.postMessage({type:"progress",phase:"taken",done:taken,total:workerData.total});
 if(taken>=workerData.total){clearInterval(beat);parentPort.postMessage({type:"done",graph:{by:workerData.name}});}
},workerData.stepMilliseconds);
