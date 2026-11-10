import {parentPort,workerData} from "node:worker_threads";
import {setTimeout as delay} from "node:timers/promises";
import {acquireResourceLease} from "../../🟦️.ts";

const{directory,resource,mode,holdMilliseconds}=workerData as{directory:string;resource:string;mode:"shared"|"exclusive";holdMilliseconds:number};
const lease=await acquireResourceLease({directory,resource,mode,signal:AbortSignal.timeout(60_000)}),acquired=Date.now();
await delay(holdMilliseconds);
const released=Date.now();lease.release();
parentPort!.postMessage({mode,acquired,released,pid:process.pid});
