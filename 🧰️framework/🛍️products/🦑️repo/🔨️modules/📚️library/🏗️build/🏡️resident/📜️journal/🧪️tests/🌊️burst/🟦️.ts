import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {CHANGE_JOURNAL_POLICY_V1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import {runChangeJournalServiceV1,type ChangeWatchFactoryV1} from "../../🟦️.ts";

/** 🌊️ Child process of the burst law: a fresh runtime (a test runner's own memory would hide the effect) receives `batches` x `rows` change rows at once through a fake watcher, with the memory bound given on the command line, and prints `{result,peakBytes}` where `result` is `alive` or the reason the service stopped itself. */
const [batches,rows,bound]=process.argv.slice(2).map(Number),directory=mkdtempSync(join(tmpdir(),"journal-burst-")),policy={...CHANGE_JOURNAL_POLICY_V1,heartbeatMilliseconds:50,maximumResidentBytes:bound};
let emit!:(rows:[string,number,string][])=>Promise<void>;
const factory:ChangeWatchFactoryV1=options=>({async start(){emit=batch=>options.onBatch(batch,Date.now());return{backend:"fsevents",trust:"exact"};},async stop(){}});
const service=await runChangeJournalServiceV1({root:directory,directory:join(directory,"journal"),fence:".fence",exclude:{rooted:[],names:[]},watcherPolicy:{},createSupervisor:factory,policy});
let peak=0;const meter=setInterval(()=>{peak=Math.max(peak,process.memoryUsage().rss);},10);
for(let batch=0;batch<batches;batch++){
 await emit(Array.from({length:rows},(_,index)=>["m",0,"module/directory"+(batch%50)+"/file"+(batch*rows+index)+".ts"] as [string,number,string]));
 if(batch%20===0)await new Promise(resolve=>setTimeout(resolve,5));
}
await new Promise(resolve=>setTimeout(resolve,400));
const result=await Promise.race([service.closed,new Promise<string>(resolve=>setTimeout(()=>resolve("alive"),200))]);
clearInterval(meter);await service.stop("burst-end");rmSync(directory,{recursive:true,force:true});
console.log(JSON.stringify({result,peakBytes:peak}));
process.exit(0);
