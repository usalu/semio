import {spawn} from "node:child_process";
import {readFileSync} from "node:fs";
import {readFile,realpath,rm,stat,statfs,writeFile} from "node:fs/promises";
import {lookup} from "node:dns/promises";
import {homedir} from "node:os";
import {connect} from "node:net";
import {join,resolve} from "node:path";
import {noticeLanguagesV1} from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import texts from "../📖️texts/🔣️.json" with {type:"json"};
import type {StatusReportV1} from "../../🔁️replica/🟦️.ts";
import {crlfScriptsV1,gitGuardsV1,resolveGitV1,type GitGuardsV1} from "../../🌿️git/🟦️.ts";
import {pathBudgetV1} from "../../🚧️boundary/🟦️.ts";
import {accessDirectoryV1,accessReceiptsLackingV1} from "../../../📚️library/🏗️build/🏡️resident/🔐️consent/🟨️.mjs";
import {serviceBootstrapScriptV1} from "../🛠️service/🟦️.ts";
import {applyFleetHostnameV1} from "../🧭️addresses/🟦️.ts";
import {applyFleetAddressEnvironmentV1,bindFleetHostsV1,factsV1,groupV1,mountPartsV1,mountPlanV1,platformV1,readTopologyV1,serverPartsV1,serverPlanV1,workerPartsV1,workerServiceV1,worktreePathV1,type FleetGroupV1,type FleetPlatformV1,type FleetResolvedHostsV1,type FleetTopologyV1} from "../🟦️.ts";

type Operation="hostname"|"mount-plan"|"server-plan"|"worker-service"|"service-bootstrap"|"facts"|"validate"|"preflight";

const OPTIONS:Readonly<Record<Operation,Readonly<{valued:readonly string[];plain:readonly string[]}>>>={
 hostname:{valued:["--root","--lang"],plain:[]},
 "mount-plan":{valued:["--group","--platform","--part","--addresses"],plain:[]},
 "server-plan":{valued:["--group","--part","--addresses"],plain:[]},
 "worker-service":{valued:["--group","--platform","--part","--role","--addresses"],plain:[]},
 "service-bootstrap":{valued:["--group","--user","--role"],plain:[]},
 facts:{valued:["--group","--platform","--shell","--home","--addresses"],plain:[]},
 validate:{valued:["--file","--timeout","--addresses"],plain:["--reachable"]},
 preflight:{valued:["--group","--worktree","--store","--timeout","--lang","--home","--addresses"],plain:[]}
};
const PLATFORMS:readonly FleetPlatformV1[]=["linux","macos","windows"],SHELLS=["sh","powershell"] as const,ROLES=["worker","server"] as const;

/** 📎️ The closed arguments of one topology operation: the valued options by name and the plain flags that were given. */
export type TopologyArgumentsV1=Readonly<{operation:Operation;values:Readonly<Record<string,string>>;flags:ReadonlySet<string>}>;

/** 📝️ Reads `env|mount-plan|server-plan|worker-service|facts|validate|preflight` with its options (`--name value` or `--name=value`) and plain flags (`validate --reachable`); every unknown, repeated or valueless option is refused. */
export function parseTopologyArgumentsV1(args:readonly string[]):TopologyArgumentsV1{
 const operation=args[0] as Operation;
 if(!Object.hasOwn(OPTIONS,operation))throw Error(`Closed fleet topology command required: ${Object.keys(OPTIONS).join("|")}`);
 const allowed=OPTIONS[operation],values:Record<string,string>={},flags=new Set<string>();
 for(let index=1;index<args.length;index++){
  const argument=args[index]!,cut=argument.indexOf("="),name=cut<0?argument:argument.slice(0,cut);
  if(allowed.plain.includes(name)){
   if(cut>=0)throw Error(`${name} takes no value`);
   if(flags.has(name))throw Error(`Repeated option: ${name}`);
   flags.add(name);
   continue;
  }
  if(!allowed.valued.includes(name))throw Error(`Unknown option for ${operation}: ${argument}`);
  if(name in values)throw Error(`Repeated option: ${name}`);
  const value=cut<0?args[++index]:argument.slice(cut+1);
  if(value===undefined||value===""||value.startsWith("--"))throw Error(`Value required for ${name}`);
  values[name]=value;
 }
 return Object.freeze({operation,values:Object.freeze(values),flags});
}

/** 🧾️ Renders facts as `FLEET_K='v'` lines (`sh`, safe to `eval`) or `$FLEET_K='v'` lines (`powershell`, safe to `Invoke-Expression`). */
export function renderFactsV1(facts:Readonly<Record<string,string>>,shell:"sh"|"powershell"):string{
 return Object.entries(facts).map(([name,value])=>shell==="sh"?`FLEET_${name}='${value.replaceAll("'","'\\''")}'`:`$FLEET_${name}='${value.replaceAll("'","''")}'`).join("\n");
}

const need=(values:Readonly<Record<string,string>>,name:string):string=>{
 const value=values[name];
 if(value===undefined)throw Error(`Required option missing: ${name}`);
 return value;
};
const choose=<T extends string>(values:Readonly<Record<string,string>>,name:string,allowed:readonly T[],fallback:T):T=>{
 const value=values[name]??fallback;
 if(!allowed.includes(value as T))throw Error(`Option ${name} must be one of ${allowed.join("|")}: ${value}`);
 return value as T;
};
const part=(parts:Readonly<Record<string,string>>,name:string):string=>{
 const body=Object.hasOwn(parts,name)?parts[name]:undefined;
 if(body===undefined)throw Error(`Declared part required: ${Object.keys(parts).join("|")}`);
 return body;
};

const ORIGIN_PATTERN=/^[a-z][a-z0-9-]{0,63}$/u;

/** 📍️ Reads `--addresses <origin>=<address>[,<origin>=<address>...]`: the resolved address of each named origin, for one rendering; anything else is refused. */
export function parseFleetAddressesV1(text:string|undefined):FleetResolvedHostsV1{
 if(text===undefined)return {};
 const found:Record<string,string>={};
 for(const item of text.split(",").filter(part=>part!=="")){
  const at=item.indexOf("="),origin=at<0?"":item.slice(0,at),address=at<0?"":item.slice(at+1);
  if(!ORIGIN_PATTERN.test(origin)||!/^[A-Za-z0-9:.%-]{2,64}$/u.test(address))throw Error(`--addresses needs <origin>=<address> pairs: ${item}`);
  found[origin]=address;
 }
 return found;
}

const topologyOf=(value:unknown,environment:Readonly<Record<string,string|undefined>>|undefined,hosts:FleetResolvedHostsV1={}):FleetTopologyV1=>{
 const applied=applyFleetAddressEnvironmentV1(readTopologyV1(value),environment??process.env);
 return Object.keys(hosts).length===0?applied:bindFleetHostsV1(applied,hosts);
};

/** 🚀️ Runs one operation and returns the text to print: `mount-plan`, `server-plan`, `worker-service`, `facts` (the values for the provisioning scripts) or `validate`; `--platform` and `--shell` default from `context.platform` (this process), `--part` picks one piece of a plan, `--role server` (worker-service) renders the group server's variant with `compact --collect-blobs`, `--file` validates another topology file. `SEMIO_FLEET_GROUP`, `SEMIO_FLEET_NFS_HOST` and `SEMIO_FLEET_NETWORK` (optional `SEMIO_FLEET_CACHE_ENDPOINT`) replace that group's address for this run; the topology file is not written. */
export function runTopologyCommandV1(args:readonly string[],context:Readonly<{platform?:string;topology?:unknown;environment?:Readonly<Record<string,string|undefined>>;home?:string;hosts?:FleetResolvedHostsV1}>={}):string{
 const {operation,values,flags}=parseTopologyArgumentsV1(args);
 if(operation==="preflight")throw Error("preflight needs the asynchronous runTopologyCommandAsyncV1");
 if(operation==="hostname")throw Error("hostname needs the asynchronous runTopologyCommandAsyncV1");
 if(operation==="validate"){
  if(flags.has("--reachable")||values["--timeout"]!==undefined)throw Error("--reachable and --timeout need the asynchronous runTopologyCommandAsyncV1");
  const topology=topologyOf(values["--file"]===undefined?context.topology:JSON.parse(readFileSync(values["--file"],"utf8")),context.environment,{...context.hosts,...parseFleetAddressesV1(values["--addresses"])});
  return `valid fleet topology: ${topology.groups.length} groups (${topology.groups.map(group=>group.id).join(", ")})`;
 }
 const topology=topologyOf(context.topology,context.environment,{...context.hosts,...parseFleetAddressesV1(values["--addresses"])}),group=need(values,"--group"),current=platformV1(context.platform),
  shell=choose(values,"--shell",SHELLS,current==="windows"?"powershell":"sh"),platform=choose(values,"--platform",PLATFORMS,current),selected=values["--part"],role=choose(values,"--role",ROLES,"worker");
 if(operation==="service-bootstrap")return serviceBootstrapScriptV1(topology,group,{user:need(values,"--user"),server:role==="server"});
 if(operation==="facts")return renderFactsV1(factsV1(topology,group,platform,values["--home"]??context.home??homedir()),shell);
 if(operation==="mount-plan")return selected===undefined?mountPlanV1(topology,group,platform):part(mountPartsV1(topology,group,platform),selected);
 if(operation==="server-plan")return selected===undefined?serverPlanV1(topology,group):part(serverPartsV1(topology,group),selected);
 return selected===undefined&&role==="worker"?workerServiceV1(topology,group,platform):part(workerPartsV1(topology,group,platform,role==="server"),selected??"definition");
}

/** 🛰️ The ports of the reachability check, each replaceable so that tests need no network: resolve a host name to addresses and open one TCP connection within a time limit. */
export type ReachabilityPortsV1=Readonly<{resolve:(host:string)=>Promise<readonly string[]>;connect:(address:string,port:number,timeoutMilliseconds:number,signal:AbortSignal)=>Promise<void>}>;

/** 🛰️ One probe of the reachability check: the group, what was probed (`store` on 2049, or 445 for an smb group, and `cache` on its endpoint port), the host and port, and what happened. */
export type ReachabilityCheckV1=Readonly<{group:string;check:"store"|"cache";host:string;port:number;ok:boolean;detail:string}>;

/** 🛰️ The outcome of the reachability check: `ok` only when every probe succeeded and nothing was cancelled, the probes in topology order and the report text. */
export type ReachabilityReportV1=Readonly<{ok:boolean;cancelled:boolean;checks:readonly ReachabilityCheckV1[];text:string}>;

const systemPorts:ReachabilityPortsV1={
 resolve:async host=>(await lookup(host,{all:true})).map(entry=>entry.address),
 connect:(address,port,timeoutMilliseconds,signal)=>new Promise<void>((done,fail)=>{
  const socket=connect({host:address,port,timeout:timeoutMilliseconds}),stop=():void=>{socket.destroy();fail(Error("cancelled"));};
  signal.addEventListener("abort",stop,{once:true});
  const finish=(error?:Error):void=>{signal.removeEventListener("abort",stop);socket.destroy();if(error)fail(error);else done();};
  socket.once("connect",()=>finish());
  socket.once("timeout",()=>finish(Error(`no answer within ${timeoutMilliseconds} ms`)));
  socket.once("error",error=>finish(error));
 })
};

const within=<T>(promise:Promise<T>,timeoutMilliseconds:number,signal:AbortSignal,what:string):Promise<T>=>new Promise<T>((done,fail)=>{
 const timer=setTimeout(()=>fail(Error(`${what} timed out after ${timeoutMilliseconds} ms`)),timeoutMilliseconds),stop=():void=>fail(Error("cancelled"));
 signal.addEventListener("abort",stop,{once:true});
 promise.then(done,fail).finally(()=>{clearTimeout(timer);signal.removeEventListener("abort",stop);});
});

/** 🛰️ Resolves the store host and the cache host of every group and opens a TCP connection to the store port (2049 for NFS, 445 for SMB) and to the cache port, each probe limited to `timeoutMilliseconds`. All probes run at once; every finished probe is reported through `progress`, and an aborted `signal` stops them and marks the report as cancelled. The report names every group with its result and lists the groups that failed; the topology itself is not changed. */
export async function checkReachabilityV1(topology:FleetTopologyV1,options:Readonly<{timeoutMilliseconds:number;signal:AbortSignal;progress:(line:string)=>void;ports?:Partial<ReachabilityPortsV1>}>):Promise<ReachabilityReportV1>{
 const {timeoutMilliseconds,signal,progress}=options,ports:ReachabilityPortsV1={...systemPorts,...options.ports},done:ReachabilityCheckV1[]=[];
 const probe=async(group:FleetGroupV1,check:"store"|"cache",resolved:string|undefined,port:number):Promise<void>=>{
  let ok=false,detail:string;
  const host=resolved??group.roles[check];
  try{
   signal.throwIfAborted();
   if(resolved===undefined)throw Error(`the address of ${group.roles[check]} is not resolved (no published address, no discovery answer)`);
   const addresses=await within(ports.resolve(host),timeoutMilliseconds,signal,`resolving ${host}`);
   if(addresses.length===0)throw Error(`${host} resolves to no address`);
   await within(ports.connect(addresses[0]!,port,timeoutMilliseconds,signal),timeoutMilliseconds,signal,`connecting to ${addresses[0]}:${port}`);
   ok=true;detail=`${addresses[0]}:${port} open`;
  }catch(error){detail=signal.aborted?"cancelled":error instanceof Error?error.message:String(error);}
  done.push({group:group.id,check,host,port,ok,detail});
  progress(`${group.id} ${check} ${host}:${port} ${ok?"ok":"FAILED"} ${detail}`);
 };
 await Promise.all(topology.groups.flatMap(group=>[probe(group,"store",group.nfs.host,group.server.protocol==="smb"?445:2049),probe(group,"cache",group.cache.endpoint===undefined?undefined:new URL(group.cache.endpoint).hostname.replace(/^\[|\]$/gu,""),Number(group.ports?.cache??6094))]));
 const checks=topology.groups.flatMap(group=>(["store","cache"] as const).map(check=>done.find(entry=>entry.group===group.id&&entry.check===check)!)),cancelled=signal.aborted,failed=topology.groups.filter(group=>checks.some(entry=>entry.group===group.id&&!entry.ok)).map(group=>group.id);
 const lines=checks.map(entry=>`${entry.group} ${entry.check} ${entry.host}:${entry.port} ${entry.ok?"ok":"FAILED"} ${entry.detail}`);
 lines.push(cancelled?"reachability cancelled":failed.length===0?`all ${topology.groups.length} groups reachable`:`unreachable groups: ${failed.join(", ")}`);
 return Object.freeze({ok:!cancelled&&failed.length===0,cancelled,checks,text:lines.join("\n")});
}

/** 🚀️ Runs one operation like `runTopologyCommandV1` and, for `validate --reachable [--timeout <ms>]` (default 3000 ms), also checks the reachability of the hosts after the offline validation; plain `validate` stays offline. `preflight [--group <id>] [--worktree <path>] [--store <path>] [--timeout <ms>] [--lang en|de]` (default 5000 ms per check) runs `preflightV1` on this machine, reports every finished check as progress and prints the report in the chosen languages. Returns the text and the exit code (1 when a host is unreachable, a check failed or the run was cancelled). */
export async function runTopologyCommandAsyncV1(args:readonly string[],context:Readonly<{platform?:string;home?:string;topology?:unknown;signal:AbortSignal;progress:(line:string)=>void;ports?:Partial<ReachabilityPortsV1>;environment?:Readonly<Record<string,string|undefined>>;probes?:Partial<PreflightProbesV1>;hosts?:FleetResolvedHostsV1;confirm?:(question:string)=>Promise<boolean>;run?:(command:string,args:readonly string[])=>Promise<number>;resolveHosts?:(requests:readonly Readonly<{origin:string;port:number}>[])=>Promise<FleetResolvedHostsV1>}>):Promise<Readonly<{text:string;exitCode:number}>>{
 const {operation,values,flags}=parseTopologyArgumentsV1(args),given={...context.hosts,...parseFleetAddressesV1(values["--addresses"])};
 const wanted=async():Promise<FleetResolvedHostsV1>=>{
  if(context.resolveHosts===undefined||Object.keys(given).length>0)return given;
  const base=applyFleetAddressEnvironmentV1(readTopologyV1(context.topology),context.environment??process.env);
  if((context.environment??process.env).SEMIO_FLEET_STORE_ADDRESS!==undefined&&(context.environment??process.env).SEMIO_FLEET_STORE_ADDRESS!=="")return given;
  const named=values["--group"],groups=named===undefined?base.groups:base.groups.filter(group=>group.id===named||Object.hasOwn(base.groups.find(row=>row.id===named)?.peers??{},group.id));
  const requests=new Map<string,number>();
  for(const group of groups){if(!requests.has(group.roles.store))requests.set(group.roles.store,group.server.protocol==="smb"?445:2049);if(!requests.has(group.roles.cache))requests.set(group.roles.cache,group.ports?.cache??6094);}
  return {...await context.resolveHosts([...requests].map(([origin,port])=>({origin,port}))),...given};
 };
 if(operation==="hostname"){
  const lines:string[]=[],language=values["--lang"]==="de"?"de":values["--lang"]==="en"?"en":(context.environment??process.env).SEMIO_LANGUAGE==="de"?"de":"en",workspace=values["--root"]??(context.environment??process.env).SEMIO_WORKSPACE_ROOT;
  if(workspace===undefined||workspace==="")throw Error(language==="de"?"--root oder SEMIO_WORKSPACE_ROOT erforderlich":"--root or SEMIO_WORKSPACE_ROOT required");
  if(context.confirm===undefined||context.run===undefined)throw Error("hostname needs the interactive confirmation of repo fleet hostname");
  await applyFleetHostnameV1({root:workspace,language,report:line=>lines.push(line),confirm:async question=>{for(const line of lines.splice(0))context.progress(line);return context.confirm!(question);},run:context.run,...context.platform===undefined?{}:{platform:context.platform as NodeJS.Platform}});
  return {text:lines.join("\n"),exitCode:0};
 }
 if(operation==="preflight"){
  const timeout=values["--timeout"]??"5000",timeoutMilliseconds=Number(timeout);
  if(!/^[0-9]+$/u.test(timeout)||timeoutMilliseconds<100||timeoutMilliseconds>60000)throw Error("--timeout must be a whole number of milliseconds from 100 to 60000");
  const report=await preflightV1(topologyOf(context.topology,context.environment,await wanted()),{platform:platformV1(context.platform),group:values["--group"],worktree:values["--worktree"],home:values["--home"]??context.home,store:values["--store"],timeoutMilliseconds,languages:preflightLanguagesV1(values["--lang"],context.environment??process.env),signal:context.signal,progress:context.progress,probes:context.probes});
  return {text:report.text,exitCode:report.ok?0:1};
 }
 if(operation!=="validate"||(!flags.has("--reachable")&&values["--timeout"]===undefined))return {text:runTopologyCommandV1(args,{platform:context.platform,topology:context.topology,environment:context.environment,hosts:operation==="validate"?given:await wanted()}),exitCode:0};
 if(!flags.has("--reachable"))throw Error("--timeout needs --reachable");
 const timeout=values["--timeout"]??"3000",timeoutMilliseconds=Number(timeout);
 if(!/^[0-9]+$/u.test(timeout)||timeoutMilliseconds<100||timeoutMilliseconds>60000)throw Error("--timeout must be a whole number of milliseconds from 100 to 60000");
 const offline=runTopologyCommandV1(["validate",...values["--file"]===undefined?[]:["--file",values["--file"]]],{platform:context.platform,topology:context.topology,environment:context.environment}),topology=topologyOf(values["--file"]===undefined?context.topology:JSON.parse(readFileSync(values["--file"],"utf8")),context.environment,await wanted());
 const report=await checkReachabilityV1(topology,{timeoutMilliseconds,signal:context.signal,progress:context.progress,ports:context.ports});
 return {text:`${offline}\n${report.text}`,exitCode:report.ok?0:1};
}

/** 🛫️ The verdict of one preflight check: `ok`, `warn` (works, but look at it), `fail` (the machine will not work) or `skip` (cannot be measured here). */
export type PreflightStatusV1="ok"|"warn"|"fail"|"skip";

/** 🛫️ One preflight check: its id, verdict, the message key of its explanation and the values of that message. */
export type PreflightCheckV1=Readonly<{id:string;status:PreflightStatusV1;key:string;variables:Readonly<Record<string,string|number>>}>;

/** 🛫️ The outcome of the preflight: `ok` when no check failed and nothing was cancelled, the checks in their fixed order and the report text in every chosen language. */
export type PreflightReportV1=Readonly<{ok:boolean;cancelled:boolean;checks:readonly PreflightCheckV1[];text:string}>;

/** 🛫️ The ports of the preflight, each replaceable so that tests need no machine: read a text file (undefined when missing), run a program (code 127 when it does not exist), one HTTP GET, free bytes of a volume, existence of a path, one create-read-delete of a probe file, the clock in milliseconds, the replica `status` report and the git guards of the worktree. */
export type PreflightProbesV1=Readonly<{
 read:(path:string)=>Promise<string|undefined>;
 run:(command:string,args:readonly string[],timeoutMilliseconds:number,signal:AbortSignal)=>Promise<Readonly<{code:number;stdout:string}>>;
 http:(url:string,timeoutMilliseconds:number,signal:AbortSignal)=>Promise<Readonly<{status:number;body:string;date:string|null}>>;
 free:(path:string)=>Promise<number>;
 exists:(path:string)=>Promise<boolean>;
 resolved:(path:string)=>Promise<string>;
 write:(directory:string,name:string)=>Promise<void>;
 now:()=>number;
 status:(worktree:string,store:string,signal:AbortSignal)=>Promise<StatusReportV1>;
 git:()=>Promise<string|null>;
 guards:(worktree:string,platform:NodeJS.Platform,longPaths:boolean|null,signal:AbortSignal)=>Promise<GitGuardsV1|null>;
 scripts:(worktree:string,signal:AbortSignal)=>Promise<readonly string[]|null>;
}>;

/** 🛫️ The limits of the preflight: the clock skew from which it warns (2 s) and fails (30 s, half of the 60 s after which a receiver holds back an event, so that two machines that are each below it still differ by less than 60 s), and the free disk space below which it warns or fails for a worktree volume and for the store volume of a group server. */
export const PREFLIGHT_LIMITS_V1=Object.freeze({skewWarnMilliseconds:2000,skewFailMilliseconds:30000,worktreeWarnBytes:30*2**30,worktreeFailBytes:10*2**30,storeWarnBytes:300*2**30,storeFailBytes:50*2**30,lagEvents:1000,visibilityWarnMilliseconds:5000,visibilitySamples:5});

type PreflightLanguageV1="en"|"de";
type Variables=Readonly<Record<string,string|number>>;
type Outcome=Readonly<{status:PreflightStatusV1;key:string;variables?:Variables}>;

const ACCESS_ROLES=["launcher","service","client"] as const,PREFLIGHT_LANGUAGES=texts.languages as readonly PreflightLanguageV1[],
 MARKS:Readonly<Record<PreflightStatusV1,string>>={ok:"ok  ",warn:"WARN",fail:"FAIL",skip:"skip"},
 CRITICAL=["hard","lookupcache","acregmin","acregmax","acdirmin","acdirmax","nonegnamecache"],
 SERVER_ROOT=resolve(import.meta.dirname,"../../../../../../..");

/** 💬️ Renders one message of the preflight in one language. */
export function preflightTextV1(language:PreflightLanguageV1,key:string,variables:Variables={}):string{
 const message=(texts.messages as Readonly<Record<string,Readonly<Record<PreflightLanguageV1,string>>>>)[key];
 if(!message)throw Error(`Declared preflight message required: ${key}`);
 return message[language].replace(/\{([a-z0-9]+)\}/gu,(_,name:string)=>{const value=variables[name];if(value===undefined)throw Error(`Preflight message variable missing: ${name}`);return String(value);});
}

/** ☁️ Picks the languages of the preflight: the explicit request, else the one the language protocol chooses, else all of them. */
export function preflightLanguagesV1(requested:string|undefined,environment:Readonly<Record<string,string|undefined>>):readonly PreflightLanguageV1[]{
 if(requested===undefined)return noticeLanguagesV1(PREFLIGHT_LANGUAGES,environment);
 if(!PREFLIGHT_LANGUAGES.includes(requested as PreflightLanguageV1))throw Error(PREFLIGHT_LANGUAGES.map(language=>preflightTextV1(language,"preflight.language",{language:requested})).join("\n"));
 return [requested as PreflightLanguageV1];
}

const spawnProbe=(command:string,args:readonly string[],timeoutMilliseconds:number,signal:AbortSignal):Promise<Readonly<{code:number;stdout:string}>>=>new Promise(done=>{
 let stdout="";
 try{
  const child=spawn(command,[...args],{signal,timeout:timeoutMilliseconds,stdio:["ignore","pipe","pipe"],windowsHide:true});
  child.stdout.on("data",(chunk:Buffer)=>{stdout+=chunk.toString();});
  child.stderr.on("data",(chunk:Buffer)=>{stdout+=chunk.toString();});
  child.once("error",()=>done({code:127,stdout}));
  child.once("close",code=>done({code:code??1,stdout}));
 }catch{done({code:127,stdout});}
});

const systemProbes:PreflightProbesV1={
 git:()=>resolveGitV1(true),
 guards:(worktree,platform,longPaths,signal)=>gitGuardsV1(worktree,{signal,platform,longPaths}),
 scripts:(worktree,signal)=>crlfScriptsV1(worktree,signal),
 read:async path=>{try{return await readFile(path,"utf8");}catch(error){if((error as {code?:string}).code==="ENOENT")return undefined;throw error;}},
 run:spawnProbe,
 http:async(url,timeoutMilliseconds,signal)=>{
  const response=await fetch(url,{signal:AbortSignal.any([signal,AbortSignal.timeout(timeoutMilliseconds)])});
  return {status:response.status,body:(await response.text()).slice(0,200),date:response.headers.get("date")};
 },
 free:async path=>{const volume=await statfs(path);return volume.bavail*volume.bsize;},
 exists:async path=>{try{await stat(path);return true;}catch{return false;}},
 resolved:async path=>{try{return await realpath(path);}catch{return path;}},
 write:async(directory,name)=>{
  const file=join(directory,name);
  await writeFile(file,"preflight\n");
  try{if((await readFile(file,"utf8"))!=="preflight\n")throw Error("the probe file reads back differently");}finally{await rm(file,{force:true});}
 },
 now:()=>Date.now(),
 status:async(worktree,store,signal)=>{
  const {runReplicaScriptV1}=await import("../../🔁️replica/🛟️supervisor/🟦️.ts"),lines:string[]=[],code=await runReplicaScriptV1(["status","--json","--worktree",worktree,"--store",store,"--lang","en"],SERVER_ROOT,process.env,signal,line=>{lines.push(line);},()=>{});
  if(code!==0)throw Error(`replica status exited with ${code}`);
  return JSON.parse(lines.join("\n")) as StatusReportV1;
 }
};

const optionMap=(options:string):Map<string,string>=>{
 const found=new Map<string,string>();
 for(const item of options.split(/[,\s]+/u).filter(Boolean)){
  const cut=item.indexOf("="),key=cut<0?item:item.slice(0,cut),value=cut<0?"":item.slice(cut+1);
  if(key==="actimeo")for(const name of["acregmin","acregmax","acdirmin","acdirmax"])found.set(name,value);else found.set(key,value);
 }
 return found;
};

type MountedV1=Readonly<{source:string;type:string;options:string}>;
const mountedLinux=(text:string,mount:string):MountedV1|undefined=>{
 let found:MountedV1|undefined;
 for(const line of text.split("\n")){
  const cut=line.indexOf(" - ");
  if(cut<0)continue;
  const left=line.slice(0,cut).split(" "),right=line.slice(cut+3).split(" ");
  if(left[4]===mount)found={type:right[0]??"",source:right[1]??"",options:right.slice(2).join(" ")};
 }
 return found;
};
const mountedMacos=(text:string,flags:string,mount:string):MountedV1|undefined=>{
 const line=text.split("\n").map(entry=>/^(.+?) on (.+?) \(([^)]*)\)\s*$/u.exec(entry)).find(match=>match?.[2]===mount);
 if(!line)return undefined;
 const block=flags.split(/\n(?=\S)/u).find(entry=>entry.startsWith(`${mount} from `));
 return {source:line[1]!,type:line[3]!.split(",")[0]!.trim(),options:`${line[3]!} ${block??""}`};
};
const mountedWindows=(text:string,drive:string):MountedV1|undefined=>{
 for(const line of text.split("\n")){
  const match=/^(?:\S+\s+)?([A-Za-z]:)\s+(\\\\\S+)(?:\s+(.*?))?\s*$/u.exec(line);
  if(match&&match[1]!.toLowerCase()===drive.toLowerCase())return {source:match[2]!,type:match[3]??"",options:""};
 }
 return undefined;
};

const parseRegistryNumber=(text:string):number|undefined=>{const match=/REG_DWORD\s+0x([0-9a-f]+)/iu.exec(text);return match?parseInt(match[1]!,16):undefined;};
const gib=(bytes:number):string=>`${(bytes/2**30).toFixed(1)} GiB`;
const seconds=(milliseconds:number):string=>`${milliseconds<0?"-":"+"}${(Math.abs(milliseconds)/1000).toFixed(1)}`;
const shorten=(value:unknown):string=>(value instanceof Error?value.message:String(value)).replace(/\s+/gu," ").slice(0,200);

/** 🛫️ Checks, without changing anything that matters, that a provisioned machine can take part in the fleet: identity (`machine.json`), the store mounted at the right source with the correctness options (Linux `lookupcache=none`, macOS name and directory caches, Windows SMB and NFS client caches), the store writable (one probe file in `v1/tmp`, removed at once), the cache endpoint healthy (`/healthz`), the clock skew against the group server (the `Date` header of its cache endpoint: warn above 2 s, fail from 30 s; the group server itself is the reference), time synchronisation (timedatectl, systemsetup, w32tm), inotify limits (Linux), long paths (Windows), free disk, the native build (`native-build`: `ok` when the completion marker `.cache/f/native-bootstrap.done` of the provisioning exists, `warn` while the machine syncs but its native bootstrap is still pending), the daemon service (and on a group server the cache and relay), the daily compaction and the replica `status`. The group, the role (a machine named `<group>-server` is the group server), the store and the worktree come from `machine.json` and the topology unless given. All checks run at once with `timeoutMilliseconds` each; every finished check is reported through `progress` in every chosen language, and an aborted `signal` stops them and marks the report as cancelled. */
export async function preflightV1(topology:FleetTopologyV1,options:Readonly<{platform:FleetPlatformV1;group?:string;worktree?:string;home?:string;uid?:number;store?:string;timeoutMilliseconds:number;languages:readonly PreflightLanguageV1[];signal:AbortSignal;progress:(line:string)=>void;probes?:Partial<PreflightProbesV1>}>):Promise<PreflightReportV1>{
 const {platform,timeoutMilliseconds,signal,languages}=options,probes:PreflightProbesV1={...systemProbes,...options.probes},limits=PREFLIGHT_LIMITS_V1;
 const sep=platform==="windows"?"\\":"/",at=(base:string,...parts:string[]):string=>[base.replace(/[\\/]+$/u,""),...parts].join(sep);
 const home=options.home??homedir(),worktree=options.worktree??worktreePathV1(topology,platform,home),machineFile=at(worktree,".cache","f","machine.json");
 const machineText=await probes.read(machineFile).catch(()=>undefined);
 let machine:{originId?:string;groupId?:string;storeRoot?:string}={};
 try{machine=machineText===undefined?{}:JSON.parse(machineText) as typeof machine;}catch{machine={};}
 const groupId=options.group??machine.groupId;
 if(groupId===undefined)throw Error(preflightTextV1(languages[0]!,"preflight.group"));
 const group=groupV1(topology,groupId),origin=machine.originId??"-",server=origin===`${groupId}-server`,storeRoot=options.store??machine.storeRoot??topology.storeMount[platform];
 const done:PreflightCheckV1[]=[],order:string[]=[],pending:Promise<void>[]=[];
 const line=(language:PreflightLanguageV1,check:PreflightCheckV1):string=>`[${String(done.length).padStart(2," ")}/${order.length}] ${MARKS[check.status]} ${preflightTextV1(language,`preflight.name.${check.id}`)}: ${preflightTextV1(language,`preflight.${check.key}`,check.variables)}`;
 const ok=(key:string,variables?:Variables):Outcome=>({status:"ok",key,variables}),warn=(key:string,variables?:Variables):Outcome=>({status:"warn",key,variables}),fail=(key:string,variables?:Variables):Outcome=>({status:"fail",key,variables}),skip=(key:string,variables?:Variables):Outcome=>({status:"skip",key,variables});
 const memo=<T>(make:()=>Promise<T>):(()=>Promise<T>)=>{let value:Promise<T>|undefined;return ()=>value??=make();};
 const add=(id:string,limit:number,run:()=>Promise<Outcome>):void=>{
  order.push(id);
  pending.push((async()=>{
   let outcome:Outcome;
   try{outcome=await within(run(),limit,signal,id);}catch(error){if(signal.aborted)return;outcome=fail("check.error",{error:shorten(error)});}
   const check:PreflightCheckV1={id,status:outcome.status,key:outcome.key,variables:outcome.variables??{}};
   done.push(check);
   for(const language of languages)options.progress(line(language,check));
  })());
 };
 let facts:Readonly<Record<string,string>>|undefined,factsError:string|undefined;
 try{facts=factsV1(topology,groupId,platform,home);}catch(error){factsError=shorten(error);}
 const mounted=memo(async():Promise<MountedV1|undefined>=>{
  if(platform==="linux"){const text=await probes.read("/proc/self/mountinfo");return text===undefined?undefined:mountedLinux(text,storeRoot)??mountedLinux(text,await probes.resolved(storeRoot));}
  if(platform==="macos"){const [list,flags]=await Promise.all([probes.run("/sbin/mount",[],timeoutMilliseconds,signal),probes.run("/usr/bin/nfsstat",["-m"],timeoutMilliseconds,signal)]);return mountedMacos(list.stdout,flags.code===0?flags.stdout:"",storeRoot);}
  const [nfs,smb]=await Promise.all([probes.run("mount.exe",[],timeoutMilliseconds,signal),probes.run("net.exe",["use"],timeoutMilliseconds,signal)]);
  return mountedWindows(nfs.stdout,storeRoot.slice(0,2))??mountedWindows(smb.stdout,storeRoot.slice(0,2));
 });
 const status=memo(()=>probes.status(worktree,storeRoot,signal));
 const health=memo(async()=>{const before=probes.now(),response=await probes.http(`${group.cache.endpoint??(()=>{throw Error(`the address of ${group.roles.cache} is not resolved`);})()}/healthz`,timeoutMilliseconds,signal);return {response,before,after:probes.now()};});
 const nonEmpty=(text:string):boolean=>text.trim().length>0;
 const unit=(part:string,wildcard:boolean):string=>{
  const name=part.split("/").at(-1)!;
  return platform==="linux"?(wildcard?name.replace("@.","@*."):name):platform==="macos"?name.replace(/\.plist$/u,""):name;
 };
 const active=async(name:string,timer:boolean=false):Promise<boolean>=>{
  if(platform==="linux"){
   const list=timer?await probes.run("systemctl",["list-timers","--all","--plain","--no-legend",name],timeoutMilliseconds,signal):await probes.run("systemctl",["list-units",name,"--state=active","--plain","--no-legend"],timeoutMilliseconds,signal);
   return list.code===0&&nonEmpty(list.stdout)&&list.stdout.includes("semio-fleet");
  }
  if(platform==="macos"){const print=await probes.run("launchctl",["print",`gui/${options.uid??process.getuid?.()??501}/${name}`],timeoutMilliseconds,signal);return print.code===0&&(timer||/state = running/u.test(print.stdout));}
  const query=await probes.run("powershell.exe",["-NoProfile","-NonInteractive","-Command",`(Get-ScheduledTask -TaskName '${name}' -ErrorAction Stop).State`],timeoutMilliseconds,signal);
  return query.code===0&&(timer||/^\s*Running\s*$/imu.test(query.stdout));
 };

 add("identity",timeoutMilliseconds,async()=>{
  if(machineText===undefined)return fail("identity.missing",{file:machineFile});
  if(machine.groupId!==groupId)return fail("identity.group",{actual:machine.groupId??"-",expected:groupId});
  return ok("identity.ok",{origin,group:groupId,role:server?"server":"worker",store:storeRoot});
 });
 add("mount",timeoutMilliseconds,async()=>{
  if(facts===undefined)return fail("mount.unsupported",{error:factsError!});
  if(server&&platform!=="linux")return await probes.exists(storeRoot)?ok("mount.local",{path:storeRoot}):fail("mount.absent",{path:storeRoot});
  const found=await mounted();
  if(found===undefined)return fail("mount.missing",{mount:storeRoot});
  if(server)return /^nfs/u.test(found.type)?fail("mount.remote",{mount:storeRoot,type:found.type}):ok("mount.local",{path:storeRoot});
  const expected=facts["NFS_SOURCE"]!,same=platform==="windows"?found.source.toLowerCase()===expected.toLowerCase():found.source===expected;
  return same?ok("mount.ok",{source:found.source,mount:storeRoot,type:found.type}):fail("mount.source",{actual:found.source,expected,mount:storeRoot});
 });
 add("options",timeoutMilliseconds,async()=>{
  if(facts===undefined||server||platform==="windows")return skip("options.skip");
  const found=await mounted();
  if(found===undefined)return skip("options.skip");
  const expected=optionMap(facts["MOUNT_OPTIONS"]!),actual=optionMap(found.options),wrong:string[]=[],unknown:string[]=[];
  for(const key of CRITICAL.filter(name=>expected.has(name))){
   const want=expected.get(key)!,have=actual.get(key);
   if(key==="hard"?actual.has("soft"):have!==undefined&&have!==want)wrong.push(`${key}${want===""?"":`=${want}`} (found ${key==="hard"?"soft":`${key}=${have}`})`);
   else if(key==="hard"?!actual.has("hard"):have===undefined)unknown.push(key);
  }
  if(wrong.length>0)return fail("options.wrong",{wrong:wrong.join(", ")});
  return unknown.length>0?warn("options.unreadable",{keys:unknown.join(", ")}):ok("options.ok",{options:CRITICAL.filter(name=>expected.has(name)).map(name=>expected.get(name)===""?name:`${name}=${expected.get(name)}`).join(",")});
 });
 add("write",timeoutMilliseconds,async()=>{
  const directory=at(storeRoot,"v1","tmp"),started=probes.now();
  try{await probes.write(directory,`preflight-${origin}-${started}`);}catch(error){return fail("write.failed",{path:directory,error:shorten(error)});}
  return ok("write.ok",{path:directory,milliseconds:probes.now()-started});
 });
 add("cache",timeoutMilliseconds,async()=>{
  try{
   const {response,after,before}=await health();
   return response.status===200&&/"status"\s*:\s*"ok"/u.test(response.body)?ok("cache.ok",{endpoint:group.cache.endpoint??"",milliseconds:after-before}):fail("cache.bad",{endpoint:group.cache.endpoint??"",status:response.status});
  }catch(error){return fail("cache.unreachable",{endpoint:group.cache.endpoint??"",error:shorten(error)});}
 });
 add("clock",timeoutMilliseconds,async()=>{
  if(server)return skip("clock.reference");
  let measured:Awaited<ReturnType<typeof health>>;
  try{measured=await health();}catch{return skip("clock.unreachable",{endpoint:group.cache.endpoint??""});}
  const sent=measured.response.date===null?Number.NaN:Date.parse(measured.response.date);
  if(Number.isNaN(sent))return skip("clock.nodate",{endpoint:group.cache.endpoint??""});
  const skew=(measured.before+measured.after)/2-(sent+500),size=Math.abs(skew),variables={seconds:seconds(skew),server:`${groupId}-server`};
  return size>=limits.skewFailMilliseconds?fail("clock.fail",variables):size>limits.skewWarnMilliseconds?warn("clock.warn",variables):ok("clock.ok",variables);
 });
 add("timesync",timeoutMilliseconds,async()=>{
  if(platform==="linux"){const result=await probes.run("timedatectl",["show","-p","NTPSynchronized","--value"],timeoutMilliseconds,signal);return result.code!==0?warn("timesync.unknown"):result.stdout.trim()==="yes"?ok("timesync.ok"):warn("timesync.no");}
  if(platform==="macos"){
   const result=await probes.run("/usr/sbin/systemsetup",["-getusingnetworktime"],timeoutMilliseconds,signal);
   if(result.code===0&&/Network Time:/u.test(result.stdout))return /Network Time:\s*On/u.test(result.stdout)?ok("timesync.ok"):warn("timesync.no");
   const stored=await probes.run("/usr/bin/defaults",["read","/Library/Preferences/com.apple.timed","TMAutoNetworkTimeEnabled"],timeoutMilliseconds,signal);
   return stored.code!==0||!/^\s*[01]\s*$/u.test(stored.stdout)?warn("timesync.unknown"):stored.stdout.trim()==="1"?ok("timesync.ok"):warn("timesync.no");
  }
  const [service,type]=await Promise.all([probes.run("sc.exe",["query","w32time"],timeoutMilliseconds,signal),probes.run("reg.exe",["query","HKLM\\SYSTEM\\CurrentControlSet\\Services\\W32Time\\Parameters","/v","Type"],timeoutMilliseconds,signal)]);
  return service.code!==0||!/:\s*4\s/u.test(service.stdout)||type.code!==0||/REG_SZ\s+NoSync/iu.test(type.stdout)?warn("timesync.no"):ok("timesync.ok");
 });
 if(platform==="linux")add("limits",timeoutMilliseconds,async()=>{
  const wanted=Object.fromEntries(mountPartsV1(topology,groupId,"linux")["sysctl"]!.split("\n").map(item=>item.split("=") as [string,string])),found:string[]=[],low:string[]=[];
  for(const key of Object.keys(wanted)){
   const text=await probes.read(`/proc/sys/${key.replaceAll(".","/")}`),value=text===undefined?Number.NaN:Number(text.trim());
   (value>=Number(wanted[key])?found:low).push(`${key.split(".").at(-1)}=${Number.isNaN(value)?"?":value}`);
  }
  return low.length>0?fail("limits.low",{low:low.join(", "),wanted:Object.entries(wanted).map(([key,value])=>`${key.split(".").at(-1)}>=${value}`).join(", ")}):ok("limits.ok",{found:found.join(", ")});
 });
 if(platform==="windows"){
  add("longpaths",timeoutMilliseconds,async()=>{
   const result=await probes.run("reg.exe",["query","HKLM\\SYSTEM\\CurrentControlSet\\Control\\FileSystem","/v","LongPathsEnabled"],timeoutMilliseconds,signal);
   return parseRegistryNumber(result.stdout)===1?ok("longpaths.ok"):fail("longpaths.off");
  });
  add("logon",timeoutMilliseconds,async()=>{
   const result=await probes.run("reg.exe",["query","HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Winlogon","/v","AutoAdminLogon"],timeoutMilliseconds,signal);
   return /AutoAdminLogon\s+REG_SZ\s+1\s*$/mu.test(result.stdout)?ok("logon.auto"):warn("logon.manual");
  });
  const peers=topology.groups.filter(other=>other.id!==groupId),needSmb=server?peers.some(peer=>peer.server.protocol==="smb"):group.server.protocol==="smb",needNfs=server?peers.some(peer=>peer.server.protocol==="nfs"):group.server.protocol==="nfs";
  if(facts!==undefined&&(needSmb||needNfs))add("clientcache",timeoutMilliseconds,async()=>{
   const wrong:string[]=[];
   if(needSmb){
    const result=await probes.run("powershell.exe",["-NoProfile","-Command","Get-SmbClientConfiguration | Select-Object DirectoryCacheLifetime,FileInfoCacheLifetime,FileNotFoundCacheLifetime | ConvertTo-Json -Compress"],timeoutMilliseconds,signal);
    let current:Record<string,number>={};
    try{current=JSON.parse(result.stdout) as Record<string,number>;}catch{current={};}
    for(const item of facts["SMB_CACHE"]!.split(",")){const [name,value]=item.split("=");if(current[name!]!==Number(value))wrong.push(`${name}=${current[name!]??"?"}`);}
   }
   if(needNfs)for(const item of facts["NFS_CACHE"]!.split(",")){
    const [name,value]=item.split("="),result=await probes.run("reg.exe",["query",facts["NFS_CACHE_KEY"]!.replace("HKLM:\\","HKLM\\"),"/v",name!],timeoutMilliseconds,signal),found=parseRegistryNumber(result.stdout);
    if(found!==Number(value))wrong.push(`${name}=${found??"?"}`);
   }
   return wrong.length>0?fail("clientcache.wrong",{wrong:wrong.join(", ")}):ok(needNfs?"clientcache.nfs":"clientcache.smb");
  });
 }
 const diskOf=(path:string,warnBytes:number,failBytes:number,id:string)=>add(id,timeoutMilliseconds,async()=>{
  const free=await probes.free(path),variables={path,free:gib(free),minimum:gib(failBytes),recommended:gib(warnBytes)};
  return free<failBytes?fail("disk.fail",variables):free<warnBytes?warn("disk.warn",variables):ok("disk.ok",variables);
 });
 diskOf(worktree,limits.worktreeWarnBytes,limits.worktreeFailBytes,"disk.worktree");
 if(server)diskOf(storeRoot,limits.storeWarnBytes,limits.storeFailBytes,"disk.store");
 add("native-build",timeoutMilliseconds,async()=>{
  const marker=at(worktree,".cache","f","native-bootstrap.done");
  return await probes.exists(marker)?ok("native.ok",{marker}):warn("native.pending",{marker,log:at(worktree,".cache","f","native-bootstrap.log")});
 });
 add("git",timeoutMilliseconds,async()=>{
  const found=await probes.git();
  return found===null?warn("git.missing"):ok("git.ok",{path:found});
 });
 const longPaths=memo(async():Promise<boolean|null>=>platform==="windows"?parseRegistryNumber((await probes.run("reg.exe",["query","HKLM\\SYSTEM\\CurrentControlSet\\Control\\FileSystem","/v","LongPathsEnabled"],timeoutMilliseconds,signal)).stdout)===1:null);
 const nodePlatform=platform==="windows"?"win32":platform==="macos"?"darwin":"linux",guards=memo(async()=>probes.guards(worktree,nodePlatform,await longPaths(),signal));
 const guarded=(id:string,decide:(found:GitGuardsV1)=>Outcome):void=>add(id,Math.max(timeoutMilliseconds,120000),async()=>{const found=await guards();return found===null?skip("guards.skip"):decide(found);});
 const sample=(found:GitGuardsV1,kind:string):string=>found.samples.filter(line=>line.startsWith(`${kind} `)).map(line=>line.slice(kind.length+1)).join(", ")||"-";
 const before=(version:string,major:number,minor:number):boolean=>{const [first=0,second=0]=version.split(".").map(Number);return first<major||first===major&&second<minor;};
 guarded("guards.git",found=>found.version===null||before(found.version,2,32)?fail("guards.git.old",{version:found.version??"?"}):ok("guards.git.ok",{version:found.version,pins:found.pins}));
 guarded("guards.exclude",found=>found.infoExclude>0?fail("guards.exclude.fail",{count:found.infoExclude,file:at(worktree,".git","info","exclude")}):ok("guards.exclude.ok"));
 guarded("guards.case",found=>found.caseMismatch>0?warn("guards.case.mismatch",{count:found.caseMismatch,paths:sample(found,"case-mismatch")}):found.caseIgnore>0?warn("guards.case.ignore",{count:found.caseIgnore,paths:sample(found,"case-ignore")}):ok("guards.case.ok",{case:found.volume.caseInsensitive?"A=a":"A≠a",normalisation:found.volume.normalisationInsensitive?"NFC=NFD":"NFC≠NFD"}));
 guarded("guards.tracked",found=>found.trackedRefused>0?fail("guards.tracked.refused",{count:found.trackedRefused,paths:sample(found,"refused")+(found.samples.some(line=>line.startsWith("unsafe-link "))?`, ${sample(found,"unsafe-link")}`:"")}):found.trackedIgnored>0?warn("guards.tracked.ignored",{count:found.trackedIgnored}):found.modeMismatch!==null&&found.modeMismatch>0?warn("guards.tracked.mode",{count:found.modeMismatch,paths:sample(found,"mode")}):ok("guards.tracked.ok",{ignored:found.trackedIgnored,links:found.links}));
 guarded("guards.paths",found=>{const budget=pathBudgetV1(nodePlatform,found.volume.longPaths===true);return found.overBudget>0?(platform==="windows"&&found.volume.longPaths!==true?fail:warn)("guards.paths.over",{count:found.overBudget,limit:budget.limit,unit:budget.unit==="utf16"?"UTF-16":"UTF-8",paths:sample(found,"over-budget")}):ok("guards.paths.ok",{limit:budget.limit,unit:budget.unit==="utf16"?"UTF-16":"UTF-8"});});
 guarded("guards.symlinks",found=>found.volume.symlinks?ok("guards.symlinks.ok"):found.links>0?fail("guards.symlinks.fail",{count:found.links}):warn("guards.symlinks.warn"));
 add("guards.scripts",Math.max(timeoutMilliseconds,120000),async()=>{const found=await probes.scripts(worktree,signal);return found===null?skip("guards.skip"):found.length>0?warn("guards.scripts.crlf",{count:found.length,paths:found.slice(0,5).join(", ")}):ok("guards.scripts.ok");});
 const worker=workerPartsV1(topology,groupId,platform),serverParts=server&&group.server.platform===platform?serverPartsV1(topology,groupId):undefined;
 add("service.replica",timeoutMilliseconds,async()=>{const name=unit(worker["target"]!,true);return await active(name)?ok("service.ok",{name}):fail("service.down",{name});});
 add("service.compact",timeoutMilliseconds,async()=>{
  const name=unit(platform==="linux"?worker["timer-target"]!:worker["compact-target"]!,true);
  return await active(name,true)?ok("compact.ok",{name}):warn("compact.missing",{name});
 });
 if(serverParts!==undefined)for(const part of["cache","relay"])add(`service.${part}`,timeoutMilliseconds,async()=>{const name=unit(serverParts[`${part}-target`]!,false);return await active(name)?ok("service.ok",{name}):fail("service.down",{name});});
 const roleReceipts=async():Promise<ReturnType<typeof accessReceiptsLackingV1>>=>{
  const directory=accessDirectoryV1(worktree,{platform:"darwin",home,environment:{}}),found:ReturnType<typeof accessReceiptsLackingV1>=[];
  for(const role of ACCESS_ROLES){
   const text=await probes.read(at(directory,`${role}.json`)).catch(()=>undefined);
   try{const value=text===undefined?undefined:JSON.parse(text) as ReturnType<typeof accessReceiptsLackingV1>[number];if(value!==undefined&&value.version===1&&value.role===role&&typeof value.at==="number"&&typeof value.state==="string")found.push(value);}catch{}
  }
  return found;
 };
 add("access",timeoutMilliseconds,async()=>{
  if(origin==="-")return skip("access.skip");
  const text=await probes.read(at(storeRoot,"v1","origins",origin,"access.json")).catch(()=>undefined);
  let state:string|undefined;
  try{state=text===undefined?undefined:(JSON.parse(text) as {state?:string}).state;}catch{state=undefined;}
  if(state==="denied"||state==="waiting-for-consent")return fail(`access.${state==="denied"?"denied":"waiting"}.${platform==="macos"?"macos":"other"}`,{worktree});
  const lacking=platform==="macos"?accessReceiptsLackingV1(await roleReceipts(),{now:probes.now()}):[];
  if(lacking.length===0)return ok("access.ok",{worktree});
  return fail(`access.role.${lacking.some(receipt=>receipt.state==="denied")?"denied":"waiting"}`,{roles:lacking.map(receipt=>receipt.role).join(", "),worktree,folder:lacking[0]!.folder,identity:lacking[0]!.identity||"repo"});
 });
 add("replica",Math.max(timeoutMilliseconds,15000),async()=>{
  const report=await status();
  if(!report.reachable)return fail("replica.unreachable",{store:report.storeRoot});
  if(!report.daemon.running)return fail("replica.stopped");
  const rejoin=report.origins.filter(entry=>entry.rejoinRequired).map(entry=>entry.origin);
  if(rejoin.length>0)return fail("replica.rejoin",{origins:rejoin.join(", ")});
  const behind=report.origins.filter(entry=>entry.behindEvents>limits.lagEvents);
  if(behind.length>0)return warn("replica.behind",{origins:behind.map(entry=>`${entry.origin} (${entry.behindEvents})`).join(", ")});
  return ok("replica.ok",{files:report.files,origins:report.origins.length,incoming:report.queue.incoming,outgoing:report.queue.outgoing??"?"});
 });
 add("watcher",Math.max(timeoutMilliseconds,15000),async()=>{
  const report=await status().catch(()=>undefined);
  if(report===undefined||!report.daemon.running||report.health===null||report.health===undefined)return skip("watcher.skip");
  return report.health.failures.some(item=>item.kind==="watch")?warn("watcher.degraded",{platform}):ok("watcher.ok");
 });
 add("visibility",Math.max(timeoutMilliseconds,15000),async()=>{
  const report=await status().catch(()=>undefined),latency=report?.latency;
  if(latency===null||latency===undefined||latency.samples.apply<limits.visibilitySamples||latency.applyMs.p95===null)return skip("visibility.skip",{minimum:limits.visibilitySamples});
  const variables={platform,p50:latency.applyMs.p50??"?",p95:latency.applyMs.p95,samples:latency.samples.apply,limit:limits.visibilityWarnMilliseconds};
  return latency.applyMs.p95>limits.visibilityWarnMilliseconds?warn("visibility.slow",variables):ok("visibility.ok",variables);
 });
 await Promise.all(pending);
 const cancelled=signal.aborted,checks=order.flatMap(id=>done.filter(check=>check.id===id)),failed=checks.filter(check=>check.status==="fail").length,warned=checks.filter(check=>check.status==="warn").length;
 const summary=cancelled?"preflight.summary.cancelled":failed>0?"preflight.summary.fail":warned>0?"preflight.summary.warn":"preflight.summary.ok";
 const text=languages.map(language=>[preflightTextV1(language,"preflight.header",{group:groupId,origin,role:preflightTextV1(language,server?"preflight.role.server":"preflight.role.worker"),platform}),...checks.map(check=>`${MARKS[check.status]} ${preflightTextV1(language,`preflight.name.${check.id}`)}: ${preflightTextV1(language,`preflight.${check.key}`,check.variables)}`),preflightTextV1(language,summary,{fail:failed,warn:warned,total:checks.length})].join("\n")).join("\n\n");
 return Object.freeze({ok:!cancelled&&failed===0,cancelled,checks,text});
}
