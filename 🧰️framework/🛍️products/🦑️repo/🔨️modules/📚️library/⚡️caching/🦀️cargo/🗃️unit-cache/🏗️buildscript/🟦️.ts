import {spawn} from "node:child_process";
import {chmodSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {fileURLToPath} from "node:url";
import {join} from "node:path";
import {buildKeyEnvironmentV1,parseBuildDirectivesV1,sha256V1,toolchainIdentityV1,UNIT_CACHE_POLICY_V1,type RustcUnitV1} from "../🔏️key/🟦️.ts";
import {acquireUnitClaimV1,type ClaimV1} from "../🔒️lock/🟦️.ts";
import {digestFileV1,emitUnitEventV1,lookupBuildRunV1,publishBuildRunV1,restoreBuildRunV1,type UnitEventV1} from "../💾️store/🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1;
type Environment=Readonly<Record<string,string|undefined>>;
const module=fileURLToPath(new URL("./🟦️.ts",import.meta.url));

/** 🏗️ Whether a rustc unit compiles a package's build script, the only executable whose runs are shared between lanes. */
export function isBuildScriptUnitV1(unit:RustcUnitV1):boolean{return unit.crateName.startsWith("build_script_")&&unit.crateTypes.every(type=>type==="bin")&&unit.emit.includes("link");}

/** 🎚️ Whether build script runs are shared: switched on by the launcher, and only where an executable text file can stand in for a binary. */
export function buildScriptsEnabledV1(environment:Environment,platform:NodeJS.Platform=process.platform):boolean{return environment.SEMIO_UNIT_CACHE_BUILD_SCRIPTS==="on"&&(policy.buildScript.platforms as readonly string[]).includes(platform);}

/** 🪞️ The text of the stand-in Cargo runs instead of the compiled build script: it consults the cache and otherwise runs the real executable beside it. */
export function shimSourceV1(real:string,runtime:string=process.execPath):string{
 return `#!${runtime}\n// ${policy.buildScript.shimMarker}\nimport {runBuildScriptV1} from ${JSON.stringify(module)};\nprocess.exitCode=await runBuildScriptV1(${JSON.stringify(real)},process.argv.slice(2),process.env,process.cwd());\n`;
}

/** 🔁️ Replaces the compiled build script `artifact` by its stand-in; the real executable moves to `<artifact>.real`. Idempotent. */
export function installBuildScriptShimV1(artifact:string,runtime:string=process.execPath):string|undefined{
 if(/\s/u.test(runtime))return undefined;
 const real=artifact+".real";
 if(!isShimV1(artifact)){rmSync(real,{force:true});renameSync(artifact,real);}
 rmSync(artifact,{force:true});writeFileSync(artifact,shimSourceV1(real,runtime));chmodSync(artifact,0o755);
 return real;
}

/** 🔎️ Whether a file is already a stand-in (a script carrying the marker), not a compiled executable. */
export function isShimV1(path:string):boolean{
 try{const head=readFileSync(path).subarray(0,256).toString("latin1");return head.startsWith("#!")&&head.includes(policy.buildScript.shimMarker);}catch{return false;}
}

function runReal(real:string,args:readonly string[],environment:Environment,cwd:string,capture:boolean):Promise<{status:number;stdout:string;stderr:string}>{
 return new Promise((done,fail)=>{
  const child=spawn(real,[...args],{cwd,env:environment as NodeJS.ProcessEnv,stdio:capture?["inherit","pipe","pipe"]:"inherit"}),out:Buffer[]=[],err:Buffer[]=[];
  for(const signal of["SIGINT","SIGTERM"] as const)process.once(signal,()=>child.kill(signal));
  if(capture){child.stdout!.on("data",chunk=>{out.push(chunk);process.stdout.write(chunk);});child.stderr!.on("data",chunk=>{err.push(chunk);process.stderr.write(chunk);});}
  child.on("error",fail);
  child.on("close",(status,signal)=>done({status:status??(signal?128+({SIGINT:2,SIGTERM:15,SIGKILL:9} as Record<string,number>)[signal]!:1),stdout:Buffer.concat(out).toString("utf8"),stderr:Buffer.concat(err).toString("utf8")}));
 });
}

/** ▶️ Runs a package's build script through the cache. A hit writes the recorded OUT_DIR and replays the recorded `cargo:` lines without starting the script; a miss runs it once across all lanes and records its OUT_DIR, output and the facts that make it rerun (declared paths, the package tree, declared environment variables). Any fault is a miss. */
export async function runBuildScriptV1(real:string,args:readonly string[],environment:Environment,cwd:string):Promise<number>{
 const root=environment.SEMIO_UNIT_CACHE_ROOT,outDir=environment.OUT_DIR;
 if(!root||!outDir||!buildScriptsEnabledV1(environment))return(await runReal(real,args,environment,cwd,false)).status;
 const lane=environment.SEMIO_UNIT_CACHE_LANE??"d",laneRoot=environment.SEMIO_UNIT_CACHE_LANE_ROOT,stampDirectory=laneRoot?join(laneRoot,policy.layout.stamps):undefined,crate=environment.CARGO_PKG_NAME??"build-script";
 const event=(kind:UnitEventV1["kind"],key:string|null,milliseconds:number,waitedMs=0,reason?:string)=>emitUnitEventV1(root,{lane,kind,crate,key,milliseconds,waitedMs,...(reason?{reason}:{})});
 const started=Date.now();let key:string;
 try{
  const digest=digestFileV1(real,stampDirectory);if(digest===null)throw Error("real");
  const inner=environment.SEMIO_UNIT_CACHE_INNER?JSON.parse(environment.SEMIO_UNIT_CACHE_INNER) as string[]:["rustc"];
  key=sha256V1(JSON.stringify([policy.namespace,"build-script",toolchainIdentityV1(root,inner,environment),digest,args,buildKeyEnvironmentV1(environment,laneRoot)]));
 }catch{const outcome=await runReal(real,args,environment,cwd,false);event("script-bypass",null,Date.now()-started,0,"unreadable-input");return outcome.status;}
 const context={cwd,laneRoot,environment,stampDirectory},serve=():boolean=>{
  const entry=lookupBuildRunV1(root,key,context);if(!entry)return false;
  const messages=restoreBuildRunV1(root,entry,{outDir,laneRoot});process.stdout.write(messages.stdout);process.stderr.write(messages.stderr);return true;
 };
 try{if(serve()){event("script-hit",key,Date.now()-started);return 0;}}catch{}
 let claim:ClaimV1|undefined;
 try{claim=await acquireUnitClaimV1(join(root,policy.layout.locks),"run-"+key.slice(0,36),{});}catch{claim=undefined;}
 const waited=claim?.waitedMs??0;
 try{
  if(claim){try{if(serve()){event("script-hit",key,Date.now()-started,waited,"after-wait");return 0;}}catch{}}
  const compiled=Date.now(),outcome=await runReal(real,args,environment,cwd,true);
  if(outcome.status!==0){event("failed",key,Date.now()-compiled,waited);return outcome.status;}
  event("script-miss",key,Date.now()-compiled,waited);
  try{
   const entry=publishBuildRunV1(root,{key,package:crate,cwd,laneRoot,directives:parseBuildDirectivesV1(outcome.stdout),environment,outDir,stdout:outcome.stdout,stderr:outcome.stderr,stampDirectory});
   event(entry?"script-publish":"script-bypass",key,Date.now()-compiled,0,entry?undefined:"not-cacheable");
  }catch{}
  return 0;
 }finally{claim?.release();}
}
