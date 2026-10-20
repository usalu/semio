import {mkdir,writeFile} from "node:fs/promises";
import {isAbsolute,join} from "node:path";
import type {BazelTaskPlanV1} from "../📋️task-plan/🟦️.ts";

const closureFile="bazel.closure.json",logFile="bazel.execlog.jsonl",eventsFile="bazel.bep.jsonl";

/** 👁️ Names the directory the parity verifier asked evidence in, or nothing when no evidence is requested. */
export function bazelEvidenceDirectoryV1(environment:Readonly<Record<string,string|undefined>>):string|undefined{
 const directory=environment.SEMIO_BUILD_EVIDENCE_DIR;
 if(directory===undefined||directory==="")return undefined;
 if(!isAbsolute(directory)||directory.length>200)throw Error("SEMIO_BUILD_EVIDENCE_DIR must be an absolute path of at most 200 characters");
 return directory;
}

/** 🚩️ The one Bazel option that records every executed spawn with its inputs and environment. */
export function bazelEvidenceFlagsV1(directory:string|undefined):string[]{return directory===undefined?[]:[`--execution_log_json_file=${join(directory,logFile)}`];}

/** 📍️ Where the build event stream of a run is kept next to the other evidence. */
export function bazelEvidenceEventsPathV1(directory:string):string{return join(directory,eventsFile);}

/** 🔗️ Records which Bazel labels belong to which task, which labels the closure query returned and the environment names each task declares. */
export async function writeBazelClosureEvidenceV1(directory:string,input:Readonly<{plan:BazelTaskPlanV1;query:(expression:string)=>Promise<string>}>):Promise<void>{
 const {plan}=input,labelOf=(id:string)=>"//:"+plan.tasks[id]!.label,labelToTask=Object.fromEntries(Object.keys(plan.tasks).map(id=>[labelOf(id),id]));
 const answer=await input.query(`kind("repo_", deps(set(${plan.selected.map(labelOf).join(" ")})))`),queried=answer.split("\n").map(line=>line.replace(/\s+\(.*\)\s*$/u,"").trim()).filter(line=>line.startsWith("//"));
 const declaredEnvironment=Object.fromEntries(Object.keys(plan.tasks).map(id=>[labelOf(id),Object.keys(plan.tasks[id]!.definition.execution.env??{}).sort()]));
 await mkdir(directory,{recursive:true});
 await writeFile(join(directory,closureFile),JSON.stringify({version:1,labelToTask,queried:[...new Set(queried)].sort(),declaredEnvironment}));
}
