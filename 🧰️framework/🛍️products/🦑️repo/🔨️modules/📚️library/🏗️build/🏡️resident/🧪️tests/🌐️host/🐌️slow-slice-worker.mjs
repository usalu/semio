import{parentPort,workerData}from"../../🏗️composition/🌐️host/👷️slice-pool/🧵️process/🟨️.mjs";
parentPort.postMessage({type:"memory",heapMb:12.5,extraMb:3.5,objectCount:1000,objectTypes:[{type:"string",count:600}]});
if(workerData.reportPhase)parentPort.postMessage({type:"progress",phase:workerData.reportPhase,done:7,total:9});
const until=performance.now()+Number(workerData.busyMilliseconds??0),held=[];
if(workerData.allocateMegabytes)for(let at=0;at<workerData.allocateMegabytes;at++){const block=Buffer.alloc(1048576,1);held.push(block);}
while(performance.now()<until);
if(workerData.fail)parentPort.postMessage({type:"error",message:"fixture failure"});
else parentPort.postMessage({type:"done",graph:JSON.parse(workerData.graphText),generationPin:"pin-"+workerData.root.length});
