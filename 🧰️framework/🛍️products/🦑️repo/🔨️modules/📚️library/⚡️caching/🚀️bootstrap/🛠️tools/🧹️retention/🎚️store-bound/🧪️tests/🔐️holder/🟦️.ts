import {acquireResourceLease} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

/** 🔐️ A live reader for the laws: holds one shared lease of a toolkit generation in this process until its standard input ends. */
const [directory,resource]=process.argv.slice(2);
const lease=await acquireResourceLease({directory:directory!,resource:resource!,mode:"shared",signal:new AbortController().signal,timeoutMs:5000});
process.stdout.write("READY\n");
await new Promise<void>(done=>{process.stdin.resume();process.stdin.on("end",done);process.stdin.on("close",done);});
lease.release();
