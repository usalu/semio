import {writeFileSync,mkdirSync} from "node:fs";
import {join} from "node:path";
import {acquirePhysicalFileLocksV1} from "../🟦️.ts";
import {captureOwnedCommandV1} from "../../../🎛️owned-execution/🟦️.ts";
import type {ResourceLeaseControl} from "../../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

/** 🧪️ Uses independent Python system flock to observe the actual Bun-owned kernel lock lifetime. */
export async function provePhysicalFileLocksV1(directory:string,control:ResourceLeaseControl):Promise<void>{
 mkdirSync(directory,{recursive:true});const path=join(directory,fixture.heldFile);writeFileSync(path,"original");const program="import fcntl,sys\nf=open(sys.argv[1],'r+')\ntry:\n fcntl.flock(f,fcntl.LOCK_EX|fcntl.LOCK_NB);print('acquired')\nexcept BlockingIOError:print('blocked')\n";
 const held=await acquirePhysicalFileLocksV1([path],control,{version:1,wait:true});try{held.check();const output=(await captureOwnedCommandV1("python3",["-c",program,path],directory,{control})).trim();if(output!==fixture.expectedWhileHeld)throw Error("Independent kernel lock oracle did not observe custody");}finally{held.close();}
 const after=(await captureOwnedCommandV1("python3",["-c",program,path],directory,{control})).trim();if(after!==fixture.expectedAfterClose)throw Error("Physical compiler lock not released");
}
