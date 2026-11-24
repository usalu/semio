import {startRemoteCacheServerV1} from "../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {httpRemoteExecutionContentV1} from "../../../🗄️content/🟦️.ts";
import {startRemoteExecutionServerV1,type RemoteExecutionServerV1} from "../../../🖥️server/🟦️.ts";
import {readSchedulerJournalV1,writeSchedulerJournalV1} from "../../../🗓️scheduler/📓️journal/🟦️.ts";

/** 🚀️ A scheduler (with `JOURNAL=<file>` it restores its operation table from that journal and keeps it written) (and, with `WITH_CACHE=1`, the cache it uses) as a process of its own on fixed ports, for the restart tests: prints "listening" and serves until SIGTERM. The cache keeps its directory, so a restarted scheduler finds the action results of the earlier one. */
const cache=process.env.WITH_CACHE==="1"?await startRemoteCacheServerV1({directory:process.env.CACHE_DIRECTORY!,host:"127.0.0.1",port:Number(process.env.CACHE_PORT),maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{}}):undefined;
const cacheUrl=`http://127.0.0.1:${process.env.CACHE_PORT}`;
const journal=process.env.JOURNAL,restore=journal===undefined?undefined:readSchedulerJournalV1(journal,Date.now(),600_000);
let server:RemoteExecutionServerV1|undefined,pending:ReturnType<typeof setTimeout>|undefined;
const changed=()=>{if(journal===undefined||pending!==undefined)return;pending=setTimeout(()=>{pending=undefined;if(server!==undefined)writeSchedulerJournalV1(journal,server.scheduler.journal());},100);};
server=await startRemoteExecutionServerV1({host:"127.0.0.1",grpcPort:Number(process.env.GRPC_PORT),workerPort:Number(process.env.WORKER_PORT),content:httpRemoteExecutionContentV1({endpoint:cacheUrl}),cacheUrl,tokens:{},maximumMessageBytes:1<<25,maximumBatchBytes:1<<22,
 scheduler:{unmatchedSeconds:4,lingerSeconds:4,leaseSeconds:6,maximumAttempts:3,defaultActionSeconds:60,maximumActionSeconds:600,maximumQueued:1000,heartbeatSeconds:1,pollSeconds:2,loadExpireSeconds:15,refusalCooldownSeconds:2,sweepMilliseconds:250,resumeGraceSeconds:Number(process.env.RESUME_GRACE??8),...restore===undefined?{}:{restore,onChange:changed},language:"none"}});
console.log("listening "+server.grpcUrl);
process.on("SIGTERM",()=>{void server!.stop().then(()=>cache?.stop()).finally(()=>process.exit(0));});
setInterval(()=>{},1<<30);
