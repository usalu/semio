import {appendFileSync} from "node:fs";
import {runHostForegroundV1} from "../../../🏡️resident/🏗️composition/🌐️host/🟨️.mjs";
const[,,implementation,loadLog]=process.argv;
const code=await runHostForegroundV1({implementation,loadSlice:async({arguments:invocation})=>{
 appendFileSync(loadLog,JSON.stringify({pid:process.pid,arguments:invocation})+"\n");
 const[project,target]=invocation[1].split(":");
 return{graph:{version:1,projects:{[project]:{name:project,root:"p/"+project,targets:{[target]:{execution:{kind:"noop"}}}}},dependencies:[]},generationPin:"pin-"+project};
}});
process.exit(code);
