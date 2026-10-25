import {chmodSync,copyFileSync,existsSync,lstatSync,mkdirSync,readdirSync,readFileSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {randomUUID} from "node:crypto";
import {dirname,join} from "node:path";
import {spawnSync} from "node:child_process";
import {bazelProbeArgumentsV1,bazelProbeDigestsV1,bazelProbeWorkspacePrefixV1,bazelProbeWorkspaceV1,renderBazelProbeV1,type BazelProbeOptionsV1} from "../🟦️.ts";
import type { CacheProbeSpecV1 } from "../../../../⚙️operations/📋️contract/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";

/** 🏃️ The wire of one probe run, written by `bazelCacheProbeV1` and read here. */
export type BazelProbeRunV1=Readonly<{options:BazelProbeOptionsV1;specification:CacheProbeSpecV1;task:string;fixture:string;environment:Readonly<Record<string,string|undefined>>}>;

const write=(path:string,text:string)=>{mkdirSync(dirname(path),{recursive:true});if(!existsSync(path)||readFileSync(path,"utf8")!==text)writeFileSync(path,text);};

/** 🔓️ Restores the owner write bit below a path so a tree copied out of Bazel's read-only outputs can be replaced. */
function makeWritable(path:string):void{const state=lstatSync(path);if(state.isSymbolicLink())return;chmodSync(path,state.mode|0o200);if(state.isDirectory())for(const name of readdirSync(path))makeWritable(join(path,name));}

/** 📥️ Copies a produced tree into the fixture with its file modes (executable bits included) and owner write access. */
function restoreTree(source:string,destination:string):void{
 const state=lstatSync(source);
 if(state.isDirectory()){mkdirSync(destination,{recursive:true});for(const name of readdirSync(source))restoreTree(join(source,name),join(destination,name));return;}
 copyFileSync(source,destination);chmodSync(destination,(state.mode&0o777)|0o200);
}

/** 🧰️ Renders the ASCII scratch workspace, refreshes the input digests, runs one Bazel build of the task and restores its declared outputs into the fixture; returns Bazel's exit status. */
export function runBazelProbeV1(run:BazelProbeRunV1,base:string=tmpdir()):number{
 const identity=statSync(run.fixture),workspace=bazelProbeWorkspaceV1(run.fixture,`${identity.ino}:${identity.birthtimeMs}`,base),prefix=bazelProbeWorkspacePrefixV1(run.fixture);
 for(const name of readdirSync(base))if(name.startsWith(prefix)&&join(base,name)!==workspace){makeWritable(join(base,name));rmSync(join(base,name),{recursive:true,force:true});}
 const target=run.specification.targets.find(row=>row.name===run.task);
 if(!target)throw Error("Declared probe target required: "+run.task);
 for(const[path,text]of Object.entries(renderBazelProbeV1(run.specification,workspace,run.options.module)))write(join(workspace,path),text);
 write(join(workspace,"MODULE.bazel.lock"),run.options.lock);write(join(workspace,"fixture.path"),run.fixture);
 const read=(file:string)=>{try{return readFileSync(join(run.fixture,file));}catch{return undefined;}};
 for(const[path,text]of Object.entries(bazelProbeDigestsV1(run.specification,read,run.environment,randomUUID())))write(join(workspace,path),text);
 const child=spawnSync(run.options.executable,bazelProbeArgumentsV1(run.options,run.specification,workspace,run.task),{cwd:workspace,stdio:"inherit",env:{...process.env,LC_ALL:"en_US.UTF-8",LANG:"en_US.UTF-8"}});
 if(child.status!==0)return child.status??1;
 const produced=join(workspace,"bazel-bin",run.task+"_out");
 for(const output of target.outputs){const destination=join(run.fixture,output);if(existsSync(destination))makeWritable(destination);rmSync(destination,{recursive:true,force:true});if(existsSync(join(produced,output)))restoreTree(join(produced,output),destination);}
 return 0;
}

if(isMainModuleV1(import.meta.url))process.exit(runBazelProbeV1(JSON.parse(process.env.SEMIO_BAZEL_PROBE!) as BazelProbeRunV1));
