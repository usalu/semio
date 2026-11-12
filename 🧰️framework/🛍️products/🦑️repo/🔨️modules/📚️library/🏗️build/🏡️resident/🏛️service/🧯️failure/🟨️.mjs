import {createHash} from "node:crypto";
import {diskFullOfV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🆓️free-space/💽️disk-full/🟦️.ts";
import {DISK_RECLAIM_COMMAND_V1,formatDiskBytesV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/💽️disk/🟦️.ts";

/** 🧯 Turns an unexpected failure of a request into the refusal the client receives. A full disk (`ENOSPC`, `database or disk is full`) is the typed `disk` refusal that names the partition, its free bytes and the command that frees space; anything else is the `internal` refusal that carries a reference, and the failure itself (with its stack) is written to the service log under that same reference, so the message can say which log file holds the details. @see ../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🆓️free-space/💽️disk-full/🟦️.ts */
export function failureRefusalV1(error,{log,logPath,retry="later"}){
 const facts=diskFullOfV1(error),detail=String(error?.stack??error),reference=createHash("sha256").update(detail).digest("hex").slice(0,8);
 log(facts?"disk-full":"request-failed",reference+" "+detail);
 if(facts)return{code:"disk",key:facts.freeBytes===undefined?"disk-full-unmeasured":"disk-full",fields:{partition:facts.partition,...(facts.freeBytes===undefined?{}:{free:formatDiskBytesV1(facts.freeBytes)}),command:DISK_RECLAIM_COMMAND_V1},retry};
 return{code:"internal",key:"internal",fields:{reference,log:logPath},retry};
}
