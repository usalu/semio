import {mkdirSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {dirname} from "node:path";
import type {RemoteExecutionDigestV1} from "../../🟦️.ts";

/** 📓️ One unfinished operation of the scheduler as its journal keeps it: what is needed to know it again after a restart (identity, action, platform, requester, bounds) and, while it executes, for how long it has run and the machine that runs it (the machine registers again with the task it kept running and the operation is bound to it). */
export type SchedulerJournalRowV1=Readonly<{id:string;instance:string;action:RemoteExecutionDigestV1;platform:readonly Readonly<{name:string;value:string}>[];timeoutSeconds:number;stage:"QUEUED"|"EXECUTING";attempt:number;failures:number;offload:boolean;requester?:string;submittedBy?:string;peakMegabytes?:number;runningMilliseconds?:number;worker?:Readonly<{name:string;origin:string}>}>;

/** 📓️ The journal file: its rows and the wall time it was written. */
export type SchedulerJournalV1=Readonly<{version:1;savedAtMs:number;operations:readonly SchedulerJournalRowV1[]}>;

/** 💾️ Writes the journal atomically (temporary name, then rename), so a process that dies in the middle leaves the earlier journal whole. Never throws: a journal that cannot be written costs only the restore. */
export function writeSchedulerJournalV1(path:string,operations:readonly SchedulerJournalRowV1[],nowMs:number=Date.now()):void{
 const temporary=`${path}.${process.pid}.tmp`;
 try{mkdirSync(dirname(path),{recursive:true});writeFileSync(temporary,JSON.stringify({version:1,savedAtMs:nowMs,operations} satisfies SchedulerJournalV1));renameSync(temporary,path);}
 catch{try{rmSync(temporary,{force:true});}catch{}}
}

/** 📖️ Reads the journal of the process that was replaced: its rows when the file is whole and no older than `maximumAgeMilliseconds`, else none (a journal of an earlier life describes work nobody waits for any more). */
export function readSchedulerJournalV1(path:string,nowMs:number,maximumAgeMilliseconds:number):readonly SchedulerJournalRowV1[]{
 try{
  const row=JSON.parse(readFileSync(path,"utf8")) as Partial<SchedulerJournalV1>;
  if(row.version!==1||!Array.isArray(row.operations)||typeof row.savedAtMs!=="number"||nowMs-row.savedAtMs>maximumAgeMilliseconds)return [];
  return row.operations.filter(operation=>typeof operation?.id==="string"&&typeof operation.action?.hash==="string"&&(operation.stage==="QUEUED"||operation.stage==="EXECUTING"));
 }catch{return [];}
}
