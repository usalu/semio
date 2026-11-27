/** 🍎️ Starts the resident service as a launchd job on macOS: the process launchd starts stays the responsible binary after it replaces itself with the runtime, so a service whose checkout lies below Documents, Desktop or Downloads is started with the launcher `repo` as its program and runs with the one consent of the machine, whichever app or shell asked for it. The job is loaded without RunAtLoad and started by a demand start (`kickstart`), because launchd defers non-demand spawns (`pended nondemand spawn = speculative|inefficient`) on a busy or penalised machine; a job that launchd does not run within a short bound is removed and the caller spawns the service directly. */
import {createHash} from "node:crypto";
import {chmodSync,closeSync,mkdirSync,openSync,readdirSync,rmSync,statSync,writeSync} from "node:fs";
import {homedir} from "node:os";
import {join} from "node:path";
import {accessDirectoryV1,consentIdentityV1,launcherConsentedV1,launcherDigestV1,launcherStateV1,protectedFolderOfV1,writeAccessReceiptV1} from "../../../🔐️consent/🟨️.mjs";

/** 🏷️ The prefix of every label of a service job. */
export const SERVICE_LABEL_PREFIX_V1="tech.semio.repo.service.";

const escapeXml=text=>String(text).replace(/&/gu,"&amp;").replace(/</gu,"&lt;").replace(/>/gu,"&gt;").replace(/[\u0000-\u0008\u000b\u000c\u000e-\u001f]/gu,"");
const string=value=>"<string>"+escapeXml(value)+"</string>";
const ALLOWED_NAMES_V1=Object.freeze(["HOME","PATH","LANG","LANGUAGE","LC_ALL","LC_CTYPE","LC_MESSAGES","TMPDIR","USER","LOGNAME","SHELL","XDG_DATA_HOME","XDG_CACHE_HOME","XDG_STATE_HOME","XDG_CONFIG_HOME","XDG_RUNTIME_DIR","BUN_JSC_forceRAMSize"]);
const SECRET_NAME_V1=/TOKEN|SECRET|PASSWORD|PASSWD|CREDENTIAL|COOKIE|EMAIL|SESSION|UUID|KEY|AUTH|INVOCATION|PROCESS_OWNER/iu;

/** 🧹️ The environment a persistent job may carry: an allow-list (`HOME`, `PATH`, the locale, `TMPDIR`, the XDG folders, the memory bound of the runtime) and the `SEMIO_*` variables whose names do not look like a secret or a session; never the rest of the environment of the process that asked for the start (agent sessions, tokens, mail addresses). */
export function allowedEnvironmentV1(environment){
 const out={};
 for(const name of Object.keys(environment).sort()){
  const value=environment[name];
  if(typeof value!=="string"||value.length===0||name.includes("=")||name.includes("\0"))continue;
  if(ALLOWED_NAMES_V1.includes(name)||(name.startsWith("SEMIO_")&&!SECRET_NAME_V1.test(name)))out[name]=value;
 }
 return out;
}

/** 🏷️ The label of the job of one service: one per runtime directory and implementation tag, so starting the service of a new implementation never boots out the running service of another one. */
export function launchAgentLabelV1(runtimeDirectory,tag=""){return SERVICE_LABEL_PREFIX_V1+createHash("sha256").update(String(runtimeDirectory)+"\0"+String(tag)).digest("hex").slice(0,12);}

/** 📜️ The property list of the job: the launcher as program with the arguments of the service, the working directory of the checkout, the allow-listed environment, log files of the service, loaded without RunAtLoad (the demand start follows), never restarted by launchd (the clients start it on demand) and without killing the children of the service when it ends. */
export function launchAgentPlistV1({label,program,arguments:argv,workingDirectory,environment,log}){
 const allowed=allowedEnvironmentV1(environment);
 return['<?xml version="1.0" encoding="UTF-8"?>','<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">','<plist version="1.0">','<dict>',
  "<key>Label</key>"+string(label),
  "<key>ProgramArguments</key><array>"+[program,...argv].map(string).join("")+"</array>",
  ...workingDirectory===undefined?[]:["<key>WorkingDirectory</key>"+string(workingDirectory)],
  "<key>EnvironmentVariables</key><dict>"+Object.keys(allowed).map(name=>"<key>"+escapeXml(name)+"</key>"+string(allowed[name])).join("")+"</dict>",
  "<key>StandardOutPath</key>"+string(log),"<key>StandardErrorPath</key>"+string(log),
  "<key>KeepAlive</key><false/>","<key>AbandonProcessGroup</key><true/>",
  "</dict>","</plist>",""].join("\n");
}

/** 🗺️ Decides whether a service start goes through launchd and with what: only on macOS, only for a checkout below a protected folder, only when the command is the runtime running the service script and the launcher exists; otherwise `undefined` and the service is spawned directly. The property list lives in a stable folder of the user (not in a temporary folder) for as long as the job exists. */
export function launchdRouteV1({command,environment=process.env,cwd=process.cwd(),runtimeDirectory,tag="",platform=process.platform,home=homedir(),exists=path=>{try{return statSync(path).isFile();}catch{return false;}},executable=process.execPath,uid=process.getuid?.()}){
 if(platform!=="darwin"||uid===undefined||environment.SEMIO_SERVICE_LAUNCHD==="off"||protectedFolderOfV1(cwd,{platform,home})===undefined||command[0]!==executable)return undefined;
 const identity=consentIdentityV1({platform,home,environment});
 if(identity===undefined||!exists(identity.path))return undefined;
 const label=launchAgentLabelV1(runtimeDirectory,tag),jobsDirectory=join(home,"Library","Application Support","semio","service-jobs");
 return{label,domain:"gui/"+uid,target:"gui/"+uid+"/"+label,program:identity.path,arguments:command.slice(1),workingDirectory:cwd,jobsDirectory,plistPath:join(jobsDirectory,label+".plist")};
}

const writePrivate=(path,text)=>{mkdirSync(join(path,".."),{recursive:true,mode:0o700});const descriptor=openSync(path,"w",0o600);try{writeSync(descriptor,text);}finally{closeSync(descriptor);}chmodSync(path,0o600);};
const sleepSync=milliseconds=>{Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,milliseconds);};
const answer=value=>typeof value==="number"?{status:value,stdout:""}:value;

/** 📋️ The labels of the service jobs that launchd knows with their pid (`undefined` when the job is not running): the lines of `launchctl list`. */
export function listServiceJobsV1(run){
 const{status,stdout}=answer(run("launchctl",["list"]));
 if(status!==0)return[];
 return stdout.split("\n").flatMap(line=>{const match=/^(\d+|-)\s+(-?\d+|-)\s+(tech\.semio\.repo\.service\.[0-9a-f]+)\s*$/u.exec(line);return match?[{label:match[3],pid:match[1]==="-"?undefined:Number(match[1])}]:[];});
}

/** 🧹️ Removes the jobs of earlier implementations that are not running (their service ended) and the property lists that belong to no loaded job; a running service of another implementation is left to retire itself. */
export function cleanServiceJobsV1(route,{run,files=directory=>{try{return readdirSync(directory);}catch{return[];}},remove=path=>rmSync(path,{force:true})}){
 const jobs=listServiceJobsV1(run),removed=[];
 for(const job of jobs)if(job.label!==route.label&&job.pid===undefined){run("launchctl",["bootout",route.domain+"/"+job.label]);remove(join(route.jobsDirectory,job.label+".plist"));removed.push(job.label);}
 for(const name of files(route.jobsDirectory)){
  const label=name.endsWith(".plist")?name.slice(0,-6):"";
  if(label.startsWith(SERVICE_LABEL_PREFIX_V1)&&label!==route.label&&!jobs.some(job=>job.label===label)){remove(join(route.jobsDirectory,name));removed.push(label);}
 }
 return removed;
}

/** 🚀 Loads the job and starts it by demand: removes stale jobs and plists, boots out an old job of the same label, writes the property list owner-only (allow-listed environment only), bootstraps it into the graphical session, `kickstart`s it and waits up to `verifyMilliseconds` until launchd reports a pid or a run. `run(program, arguments)` answers the exit status or `{status, stdout}`. Answers `{started, reason}`; on failure the job and its file are removed so the caller spawns the service directly. */
export function startViaLaunchdV1(route,{environment,log,run,write=writePrivate,remove=path=>rmSync(path,{force:true}),files,wait=sleepSync,verifyMilliseconds=6000,pollMilliseconds=200}){
 const undo=()=>{try{run("launchctl",["bootout",route.target]);}catch{}try{remove(route.plistPath);}catch{}};
 try{
  cleanServiceJobsV1(route,{run,...files?{files}:{},remove});
  run("launchctl",["bootout",route.target]);
  write(route.plistPath,launchAgentPlistV1({label:route.label,program:route.program,arguments:route.arguments,workingDirectory:route.workingDirectory,environment,log}));
  if(answer(run("launchctl",["bootstrap",route.domain,route.plistPath])).status!==0){undo();return{started:false,reason:"bootstrap refused"};}
  if(answer(run("launchctl",["kickstart",route.target])).status!==0){undo();return{started:false,reason:"kickstart refused"};}
  for(let waited=0;waited<=verifyMilliseconds;waited+=pollMilliseconds){
   const{status,stdout}=answer(run("launchctl",["print",route.target]));
   if(status===0&&(/\bpid = \d+/u.test(stdout)||/\bruns = [1-9]\d*/u.test(stdout)))return{started:true,reason:"running"};
   wait(pollMilliseconds);
  }
  undo();return{started:false,reason:"not started within "+verifyMilliseconds+" ms"};
 }catch(error){undo();return{started:false,reason:String(error?.message??error)};}
}

/** 🛑️ Unloads every service job and removes every property list (`service host stop`); a job that is not loaded is no failure. Answers how many jobs were unloaded. */
export function stopServiceJobsV1({platform=process.platform,uid=process.getuid?.(),home=homedir(),run,files=directory=>{try{return readdirSync(directory);}catch{return[];}},remove=path=>rmSync(path,{force:true})}){
 if(platform!=="darwin"||uid===undefined)return 0;
 const directory=join(home,"Library","Application Support","semio","service-jobs");
 let count=0;
 for(const job of listServiceJobsV1(run))if(answer(run("launchctl",["bootout","gui/"+uid+"/"+job.label])).status===0)count++;
 for(const name of files(directory))if(name.startsWith(SERVICE_LABEL_PREFIX_V1)&&name.endsWith(".plist"))remove(join(directory,name));
 return count;
}

/** 🏷️ The prefix of every label of a consent probe job. */
export const PROBE_LABEL_PREFIX_V1="tech.semio.repo.probe.";

/** 🔎️ Makes sure that the launcher itself, the one identity, may read a checkout, and answers within `waitMilliseconds` whatever macOS does (`changed: true` tells that the launcher was replaced by other bytes after it had been allowed, so macOS asks once more; a service that runs keeps its old launcher file until it stops): `ready` (the marker names the launcher as it is now), `waiting` (a probe job runs the launcher, which is blocked on the consent dialog "repo would like to access files in your Documents folder"; the job is left running and writes the marker when the owner clicks Allow), `failed` (launchd did not run the probe; the caller keeps its former way), `unneeded` (no consent exists here). The receipt role `launcher` says which. */
export function ensureLauncherConsentV1({root,probeScript,platform=process.platform,home=homedir(),environment=process.env,uid=process.getuid?.(),run,write=writePrivate,remove=path=>rmSync(path,{force:true}),wait=sleepSync,waitMilliseconds=6000,pollMilliseconds=200,consented=launcherConsentedV1,stateOf=launcherStateV1,exists=path=>{try{return statSync(path).isFile();}catch{return false;}},now=Date.now}){
 const folder=protectedFolderOfV1(root,{platform,home}),identity=consentIdentityV1({platform,home,environment});
 if(folder===undefined||identity===undefined||uid===undefined||!exists(identity.path))return{state:"unneeded"};
 const launcherSha=path=>{const sha=launcherDigestV1(path);return sha===undefined?{}:{launcherSha256:sha};},receipt=state=>{try{writeAccessReceiptV1(accessDirectoryV1(root,{platform,home,environment}),{version:1,role:"launcher",pid:process.pid,state,root,folder,identity:identity.path,at:now(),...launcherSha(identity.path)});}catch{}};
 if(consented(root,{platform,home,environment})){receipt("ok");return{state:"ready"};}
 const changed=stateOf(root,{platform,home,environment})==="changed";
 const label=PROBE_LABEL_PREFIX_V1+createHash("sha256").update(String(root)).digest("hex").slice(0,12),target="gui/"+uid+"/"+label,plistPath=join(home,"Library","Application Support","semio","service-jobs",label+".plist");
 try{
  const running=answer(run("launchctl",["print",target]));
  if(!(running.status===0&&/\bpid = \d+/u.test(running.stdout))){
   run("launchctl",["bootout",target]);
   write(plistPath,launchAgentPlistV1({label,program:identity.path,arguments:[probeScript,root],environment:{...allowedEnvironmentV1(environment),SEMIO_WORKSPACE_ROOT:root,HOME:home},log:"/dev/null"}));
   if(answer(run("launchctl",["bootstrap","gui/"+uid,plistPath])).status!==0){remove(plistPath);return{state:"failed",reason:"bootstrap refused"};}
   if(answer(run("launchctl",["kickstart",target])).status!==0){run("launchctl",["bootout",target]);remove(plistPath);return{state:"failed",reason:"kickstart refused"};}
  }
  for(let waited=0;waited<=waitMilliseconds;waited+=pollMilliseconds){
   if(consented(root,{platform,home,environment})){run("launchctl",["bootout",target]);remove(plistPath);receipt("ok");return{state:"ready"};}
   wait(pollMilliseconds);
  }
  receipt("waiting");return{state:"waiting",...changed?{changed:true}:{}};
 }catch(error){return{state:"failed",reason:String(error?.message??error)};}
}
