import {appendFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
/** 🔢️ A compiler shim that counts real compiles: `bun count.ts <log file> <crate> <delay ms> -- <compiler…> <rustc arguments…>` appends one JSON line per compile of a crate (never for probes) and delays the named crate so two machines race on it. */
const argv=process.argv.slice(2),separator=argv.indexOf("--"),[log,crate,delay]=argv.slice(0,separator),rest=argv.slice(separator+1);
const named=rest.indexOf("--crate-name"),name=named>=0?rest[named+1]:undefined;
if(name!==undefined&&name!=="___"){const metadata=rest.find((argument,index)=>rest[index-1]==="-C"&&argument.startsWith("metadata="))??"";appendFileSync(log!,JSON.stringify({crate:name,unit:name+"/"+metadata,pid:process.pid,at:Date.now(),cwd:process.cwd()})+"\n");if(name===crate)await new Promise(done=>setTimeout(done,Number(delay)));}
const environment={...process.env};for(const key of["CARGO_MAKEFLAGS","MAKEFLAGS","MFLAGS"])delete environment[key];
const result=spawnSync(rest[0]!,rest.slice(1),{stdio:"inherit",env:environment});
process.exitCode=result.status??1;
