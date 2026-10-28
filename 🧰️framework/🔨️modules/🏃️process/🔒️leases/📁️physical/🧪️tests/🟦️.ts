import {writeFileSync,mkdirSync} from "node:fs";
import {join} from "node:path";
import {acquirePhysicalFileLocksV1} from "../🟦️.ts";
import {captureOwnedCommandV1} from "../../../🎛️owned-execution/🟦️.ts";
import {testPythonV1} from "../../../🧪️testing/🧰️toolchain/🟦️.ts";
import type {ResourceLeaseControl} from "../../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

const programs={
 posix:"import fcntl,sys\nf=open(sys.argv[1],'r+')\ntry:\n fcntl.flock(f,fcntl.LOCK_EX|fcntl.LOCK_NB);print('acquired')\nexcept BlockingIOError:print('blocked')\n",
 win32:"import msvcrt,sys\nf=open(sys.argv[1],'r+b')\ntry:\n msvcrt.locking(f.fileno(),msvcrt.LK_NBLCK,1);print('acquired')\nexcept OSError:print('blocked')\n",
};

/** 🧪️ Uses an independent Python lock call (`fcntl.flock`, on Windows `msvcrt.locking`) to observe the actual Bun-owned kernel lock lifetime. */
export async function provePhysicalFileLocksV1(directory:string,control:ResourceLeaseControl):Promise<void>{
 mkdirSync(directory,{recursive:true});const path=join(directory,fixture.heldFile);writeFileSync(path,"original");const program=process.platform==="win32"?programs.win32:programs.posix,python=testPythonV1()??(process.platform==="win32"?"python":"python3"),observe=async()=>(await captureOwnedCommandV1(python,["-c",program,path],directory,{control})).trim();
 const held=await acquirePhysicalFileLocksV1([path],control,{version:1,wait:true});try{held.check();if(await observe()!==fixture.expectedWhileHeld)throw Error("Independent kernel lock oracle did not observe custody");}finally{held.close();}
 if(await observe()!==fixture.expectedAfterClose)throw Error("Physical compiler lock not released");
}
