import {existsSync,readFileSync} from "node:fs";
import {runRepoInvocationV1} from "../../🏃️run/🟦️.ts";
import {requestPlanV1,spawnHostServiceV1} from "../../../🏡️resident/🟨️.mjs";
import {PLAN_HANDOFF_V1,takePlannedGraphV1} from "../../🛰️plan/🟦️.ts";
const[,,root,implementation,serviceMain,loadLog,...args]=process.argv;
const environment=process.env as NodeJS.ProcessEnv;
let executed=0,projects:string[]=[];
await runRepoInvocationV1({root,args,environment,execute:async()=>{executed++;const graph=await takePlannedGraphV1(environment);projects=Object.keys(graph.projects);},ports:{requestPlanV1:options=>requestPlanV1({...options,implementation}),spawnHostServiceV1:()=>spawnHostServiceV1({command:[process.execPath,serviceMain,implementation,loadLog],environment})}});
process.stdout.write(JSON.stringify({executed,projects,handoffCleared:environment[PLAN_HANDOFF_V1]===undefined,exitCode:process.exitCode??0})+"\n");
