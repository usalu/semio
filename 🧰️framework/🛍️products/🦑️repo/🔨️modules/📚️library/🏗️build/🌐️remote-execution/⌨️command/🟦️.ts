import {availableParallelism,hostname} from "node:os";
import {isAbsolute,resolve} from "node:path";
import {readRemoteCachePolicyV1} from "../../🌐️remote-cache/🟦️.ts";
import {remoteCacheLanguagesV1,type RemoteCacheLanguageV1} from "../../🌐️remote-cache/⌨️command/🟦️.ts";
import {RemoteCacheDirectoryBusyV1,startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../🌐️remote-cache/🖥️server/🟦️.ts";
import {connectGrpcV1} from "../📡️wire/🚪️grpc/🟦️.ts";
import {httpRemoteExecutionContentV1} from "../🗄️content/🟦️.ts";
import {startRemoteExecutionServerV1} from "../🖥️server/🟦️.ts";
import {runRemoteExecutionWorkerV1} from "../👷️worker/🟦️.ts";
import texts from "../📖️texts/🔣️.json" with {type:"json"};
import {readRemoteExecutionPolicyV1,remoteExecutionHostPropertiesV1,remoteExecutionPropertiesV1,requireRemoteExecutionShapeV1,type RemoteExecutionPropertyV1,type RemoteExecutionSandboxV1} from "../🟦️.ts";

/** 👶️ A worker process started by `serve`; the host decides how processes are launched (the repository CLI in production, a script in tests). */
export interface RemoteExecutionChildV1{readonly exited:Promise<number>;stop():Promise<void>}

/** 🎛️ Carries the issued phase into the command: where the workspace is, the environment, cancellation, progress, output and how to start a worker process. */
export type RemoteExecutionCommandContextV1=Readonly<{root:string;environment:Readonly<Record<string,string|undefined>>;signal:AbortSignal;progress:(stage:string)=>Promise<void>;write:(line:string)=>void;spawnWorker:(args:readonly string[],environment:Readonly<Record<string,string>>)=>RemoteExecutionChildV1}>;

/** 🏁️ How a command ended: the process exit code and an optional machine-readable document to print. */
export type RemoteExecutionCommandResultV1=Readonly<{exitCode:number;result?:unknown}>;

type Operation="serve"|"status"|"workers";

const LANGUAGES=texts.languages as readonly RemoteCacheLanguageV1[],LOOPBACK=["127.0.0.1","localhost","::1","[::1]"],
 OPTIONS:Readonly<Record<Operation,Readonly<{valued:readonly string[];plain:readonly string[]}>>>={
  serve:{valued:["--bind","--grpc-port","--worker-port","--cache","--local-workers","--concurrency","--advertise","--tls-certificate","--tls-key","--sandbox","--work-directory","--lang"],plain:[]},
  status:{valued:["--join","--lang"],plain:[]},
  workers:{valued:["--join","--name","--concurrency","--sandbox","--properties","--work-directory","--lang"],plain:[]},
 };

/** 🧾️ The options each operation accepts, valued ones by name and plain flags; the catalogue row of the operation lists exactly their union. */
export const REMOTE_EXECUTION_OPTIONS_V1=OPTIONS;

/** 💬️ Renders one catalogue message in one language. */
export function remoteExecutionTextV1(language:RemoteCacheLanguageV1,key:string,variables:Readonly<Record<string,string|number>>={}):string{
 const message=(texts.messages as Readonly<Record<string,Readonly<Record<RemoteCacheLanguageV1,string>>>>)[key];
 if(!message)throw Error(`Declared remote execution message required: ${key}`);
 return message[language].replace(/\{([a-z]+)\}/gu,(_,name:string)=>{const value=variables[name];if(value===undefined)throw Error(`Remote execution message variable missing: ${name}`);return String(value);});
}

const both=(key:string,variables:Readonly<Record<string,string|number>>={}):string=>LANGUAGES.map(language=>remoteExecutionTextV1(language,key,variables)).join("\n");

/** 📝️ The closed arguments of one `remote-execution` operation. */
export type RemoteExecutionArgumentsV1=Readonly<{operation:Operation;values:Readonly<Record<string,string>>}>;

/** 🧭️ Reads `serve|status|workers` with its options (`--name value` or `--name=value`); every unknown, misplaced or repeated option is refused in both languages. */
export function parseRemoteExecutionArgumentsV1(args:readonly string[]):RemoteExecutionArgumentsV1{
 const operation=args[0];
 if(operation!=="serve"&&operation!=="status"&&operation!=="workers")throw Error(both("error.syntax"));
 const allowed=OPTIONS[operation],seen=new Set<string>(),values:Record<string,string>={};
 for(let index=1;index<args.length;index++){
  const argument=args[index]!,cut=argument.indexOf("="),name=cut<0?argument:argument.slice(0,cut);
  if(seen.has(name))throw Error(both("error.repeated",{option:name}));
  if(!allowed.valued.includes(name))throw Error(both("error.option",{operation,option:argument}));
  const value=cut<0?args[++index]:argument.slice(cut+1);
  if(value===undefined||value===""||value.startsWith("--"))throw Error(both("error.value",{option:name}));
  values[name]=value;seen.add(name);
 }
 return Object.freeze({operation,values:Object.freeze(values)});
}

const whole=(name:string,value:string,minimum:number,maximum:number):number=>{
 const number=Number(value);
 if(!/^[0-9]+$/u.test(value)||!Number.isSafeInteger(number)||number<minimum||number>maximum)throw Error(both("error.number",{option:name,minimum,maximum}));
 return number;
};
const where=(root:string,path:string):string=>isAbsolute(path)?path:resolve(root,path);
const optional=(environment:Readonly<Record<string,string|undefined>>,name:string):string|undefined=>{const value=environment[name];return value===undefined||value===""?undefined:value;};
const sandboxOf=(value:string|undefined,fallback:string):"auto"|RemoteExecutionSandboxV1=>{
 const chosen=value??fallback;
 if(chosen!=="auto"&&chosen!=="darwin-sandbox"&&chosen!=="linux-sandbox"&&chosen!=="isolated-root")throw Error(both("error.option",{operation:"sandbox",option:chosen}));
 return chosen;
};

function reporter(context:RemoteExecutionCommandContextV1,languages:readonly RemoteCacheLanguageV1[]):(key:string,variables?:Readonly<Record<string,string|number>>)=>void{
 return (key,variables)=>{
  for(const language of languages){
   const resolved:Record<string,string|number>={};
   for(const [name,value] of Object.entries(variables??{}))resolved[name]=typeof value=="string"&&value.startsWith("@")?remoteExecutionTextV1(language,value.slice(1)):value;
   context.write(remoteExecutionTextV1(language,key,resolved));
  }
 };
}

/** 🆔️ The platform id of this machine in the vocabulary of the build tool contracts (`darwin-arm64`, `linux-x64`, `windows-x64`). */
export function remoteExecutionPlatformIdV1(platform:string=process.platform,arch:string=process.arch):string{return `${platform==="win32"?"windows":platform}-${arch}`;}

const bracket=(host:string):string=>host.includes(":")&&!host.startsWith("[")?`[${host}]`:host;

/** 🚀️ Runs the in-house remote execution service until the phase is cancelled: the shared store (started here unless one is named), the gRPC and worker listeners with their scheduler, and local worker processes. Prints the environment a client sets and the command another machine joins with. */
export async function serveRemoteExecutionV1(args:readonly string[],context:RemoteExecutionCommandContextV1):Promise<RemoteExecutionCommandResultV1>{
 const parsed=parseRemoteExecutionArgumentsV1(["serve",...args]),flags=parsed.values,languages=remoteCacheLanguagesV1(flags["--lang"],context.environment),say=reporter(context,languages);
 const policy=readRemoteExecutionPolicyV1(),cachePolicy=readRemoteCachePolicyV1(),names=policy.environment,server=policy.server;
 const read=optional(context.environment,cachePolicy.server.tokenEnvironment.read),write=optional(context.environment,cachePolicy.server.tokenEnvironment.write),worker=optional(context.environment,names.workerToken);
 const host=flags["--bind"]??server.host,loopback=LOOPBACK.includes(host);
 if(!loopback&&((read===undefined&&write===undefined)||worker===undefined))throw Error(both("error.token"));
 const certificate=flags["--tls-certificate"],key=flags["--tls-key"];
 if((certificate===undefined)!==(key===undefined))throw Error(both("error.tls"));
 const tls=certificate&&key?{certificate:where(context.root,certificate),key:where(context.root,key)}:undefined;
 await context.progress("remote-execution:open");say("serve.open",{host});
 let embedded:RemoteCacheServerV1|undefined,cacheUrl=flags["--cache"]??optional(context.environment,cachePolicy.environment.endpoint);
 if(cacheUrl===undefined){
  const store=cachePolicy.server;
  const open=(port:number)=>startRemoteCacheServerV1({directory:where(context.root,store.directory),host,port,maximumBytes:store.maximumBytes,maximumEntryBytes:Math.min(store.maximumEntryBytes,Math.floor(store.maximumBytes/2)),maximumAgeDays:store.maximumAgeDays,defaultInstance:cachePolicy.defaults.instance,tokens:{read,write},...(tls?{tls}:{}),signal:context.signal});
  try{
   embedded=await open(store.port).catch(error=>{if((error as {code?:string}).code==="EADDRINUSE")return open(0);throw error;});
  }catch(error){
   throw Error(both("error.cache",{reason:error instanceof RemoteCacheDirectoryBusyV1?error.message:(error as Error).message}));
  }
  cacheUrl=embedded.url;say("serve.cache.embedded",{url:cacheUrl,directory:where(context.root,store.directory)});
 }else say("serve.cache.external",{url:cacheUrl});
 await context.progress("remote-execution:cache");
 const token=write??read,content=httpRemoteExecutionContentV1({endpoint:cacheUrl,token});
 let service;
 try{
  service=await startRemoteExecutionServerV1({
   host,grpcPort:flags["--grpc-port"]===undefined?server.port:whole("--grpc-port",flags["--grpc-port"],0,65535),workerPort:flags["--worker-port"]===undefined?server.workerPort:whole("--worker-port",flags["--worker-port"],0,65535),
   content,cacheUrl,tokens:{read,write,worker},tls,maximumMessageBytes:server.maximumMessageBytes,maximumBatchBytes:server.maximumBatchBytes,advertise:flags["--advertise"],
   scheduler:{unmatchedSeconds:server.unmatchedSeconds,lingerSeconds:server.lingerSeconds,leaseSeconds:server.leaseSeconds,maximumAttempts:server.maximumAttempts,defaultActionSeconds:server.defaultActionSeconds,maximumQueued:server.maximumQueued,heartbeatSeconds:policy.worker.heartbeatSeconds,pollSeconds:policy.worker.pollSeconds,language:languages.length===1?languages[0]!:"none"},
   onEvent:event=>{if(event.stage==="worker-joined"||event.stage==="worker-left"||event.stage==="worker-lost"||event.stage==="unmatched")say("serve.event",{event:event.stage,detail:event.detail});},
  });
 }catch(error){await embedded?.stop();throw error;}
 const children:Awaited<ReturnType<typeof context.spawnWorker>>[]=[];
 const stop=async():Promise<void>=>{
  say("serve.stopping");await context.progress("remote-execution:stopping");
  await Promise.all(children.map(child=>child.stop().catch(()=>{})));await service.stop();await embedded?.stop();
 };
 try{
  const grpcPort=new URL(service.grpcUrl.replace(/^grpc/u,"http")).port,shown=flags["--advertise"]??(loopback?"127.0.0.1":hostname()),endpoint=`${tls?"grpcs":"grpc"}://${bracket(shown)}:${grpcPort}`,workerUrl=service.workerUrl;
  context.write(`[remote-execution] listening ${endpoint}`);
  say("serve.listening",{grpc:endpoint,workers:workerUrl,access:token===undefined?"@serve.access.open":"@serve.access.token"});
  const count=flags["--local-workers"]===undefined?server.localWorkers:whole("--local-workers",flags["--local-workers"],0,64);
  const childEnvironment:Record<string,string>={...(worker?{[names.workerToken]:worker}:{}),...(token?{[cachePolicy.defaults.tokenEnvironment]:token}:{})};
  for(let index=0;index<count;index++){
   const childArgs=["--join",workerUrl,"--name",`${hostname()}-local-${index+1}`,...(flags["--concurrency"]?["--concurrency",flags["--concurrency"]]:[]),...(flags["--sandbox"]?["--sandbox",flags["--sandbox"]]:[]),...(flags["--work-directory"]?["--work-directory",flags["--work-directory"]]:[]),...(flags["--lang"]?["--lang",flags["--lang"]]:[])];
   children.push(context.spawnWorker(childArgs,childEnvironment));
  }
  await context.progress("remote-execution:workers");
  if(count>0)say("serve.workers",{count});else say("serve.workers.none",{url:workerUrl});
  say("serve.client");
  const platform=remoteExecutionPlatformIdV1(),lines=[`${names.endpoint}=${endpoint}`,`${names.platform}=${platform}`,`${names.instance}=${server.defaultInstance}`];
  for(const line of lines)context.write(line);
  say("serve.join",{url:workerUrl});
  await context.progress("remote-execution:serving");
  await new Promise<void>(done=>{if(context.signal.aborted)done();else context.signal.addEventListener("abort",()=>{done();},{once:true});});
 }finally{await stop();say("serve.stopped");}
 return {exitCode:0};
}

/** 👷️ Runs one worker agent of this machine until the phase is cancelled: it joins the scheduler named by `--join` (or the environment), offers this machine's platform properties and confines every action with the hermetic strategies of the host. */
export async function runRemoteExecutionWorkersV1(args:readonly string[],context:RemoteExecutionCommandContextV1):Promise<RemoteExecutionCommandResultV1>{
 const parsed=parseRemoteExecutionArgumentsV1(["workers",...args]),flags=parsed.values,languages=remoteCacheLanguagesV1(flags["--lang"],context.environment),say=reporter(context,languages);
 const policy=readRemoteExecutionPolicyV1(),cachePolicy=readRemoteCachePolicyV1(),names=policy.environment,scheduler=flags["--join"]??optional(context.environment,names.scheduler);
 if(scheduler===undefined)throw Error(both("error.join",{variable:names.scheduler}));
 let properties:readonly RemoteExecutionPropertyV1[];
 try{properties=[...remoteExecutionHostPropertiesV1(),...remoteExecutionPropertiesV1(flags["--properties"])];}catch{throw Error(both("error.properties",{value:flags["--properties"]??""}));}
 const concurrency=flags["--concurrency"]===undefined?(policy.worker.concurrency===0?availableParallelism():policy.worker.concurrency):whole("--concurrency",flags["--concurrency"],1,512),name=flags["--name"]??`${hostname()}-${process.pid}`;
 await context.progress("remote-execution:worker");
 say("workers.start",{name,url:scheduler,concurrency,platform:remoteExecutionPlatformIdV1()});
 const label=(event:{stage:string;detail:string}):void=>{say(`workers.${event.stage}`,event.stage==="sandbox"?{sandbox:event.detail}:{detail:event.detail});};
 await runRemoteExecutionWorkerV1({
  scheduler,token:optional(context.environment,names.workerToken),cacheToken:optional(context.environment,cachePolicy.defaults.tokenEnvironment),name,concurrency,
  directory:where(context.root,flags["--work-directory"]??policy.worker.directory),sandbox:sandboxOf(flags["--sandbox"],policy.worker.sandbox),materialize:policy.worker.materialize as "clone"|"copy",maximumBlobBytes:policy.worker.maximumBlobBytes,inlineOutputBytes:policy.worker.inlineOutputBytes,
  properties,signal:context.signal,onEvent:event=>{label(event);},
 }).catch(error=>{if(/Sandbox strategy not available/u.test((error as Error).message))throw Error(both("error.sandbox",{sandbox:flags["--sandbox"]??policy.worker.sandbox}));throw error;});
 return {exitCode:0};
}

/** 📋️ Prints the client contract, whether the gRPC endpoint answers the protocol's capabilities call, and what the scheduler reports about itself (workers, queues, counters); secrets never appear. */
export async function remoteExecutionStatusV1(args:readonly string[],context:RemoteExecutionCommandContextV1):Promise<unknown>{
 const parsed=parseRemoteExecutionArgumentsV1(["status",...args]),policy=readRemoteExecutionPolicyV1(),cachePolicy=readRemoteCachePolicyV1(),names=policy.environment,environment=context.environment;
 const endpoint=optional(environment,names.endpoint),derived=endpoint===undefined?null:(()=>{try{const url=new URL(endpoint.replace(/^grpc(s?):/u,"http$1:"));return `${url.protocol}//${url.hostname.includes(":")?`[${url.hostname}]`:url.hostname}:${policy.server.workerPort}`;}catch{return null;}})(),scheduler=parsed.values["--join"]??optional(environment,names.scheduler)??derived;
 const token=optional(environment,cachePolicy.server.tokenEnvironment.write)??optional(environment,cachePolicy.server.tokenEnvironment.read)??optional(environment,cachePolicy.defaults.tokenEnvironment),workerToken=optional(environment,names.workerToken);
 await context.progress("remote-execution:probe");
 const contract=endpoint===undefined?null:{endpoint,platform:optional(environment,names.platform)??remoteExecutionPlatformIdV1(),instance:optional(environment,names.instance)??policy.server.defaultInstance,properties:remoteExecutionPropertiesV1(optional(environment,names.properties)),jobs:Number(optional(environment,names.jobs)??128)};
 let grpc=null;
 if(endpoint!==undefined&&/^grpcs?:\/\//u.test(endpoint)){
  const started=performance.now(),client=connectGrpcV1(endpoint);
  try{
   const caps=await client.unary("build.bazel.remote.execution.v2.Capabilities/GetCapabilities",{},{metadata:token?{authorization:`Bearer ${token}`}:{},timeoutMilliseconds:5000,signal:context.signal});
   const version=(row:{major:number;minor:number}|undefined)=>row?`${row.major}.${row.minor}`:"";
   grpc={capabilities:true,execution:caps.execution_capabilities?.exec_enabled===true,apiVersions:[version(caps.low_api_version),version(caps.high_api_version)].filter(Boolean),digestFunctions:((caps.cache_capabilities?.digest_functions??[]) as number[]).map(value=>value===1?"SHA256":String(value)),probe:{reachable:true,milliseconds:Math.round(performance.now()-started),detail:""}};
  }catch(error){
   if(context.signal.aborted)throw error;
   grpc={capabilities:false,execution:false,apiVersions:[],digestFunctions:[],probe:{reachable:false,milliseconds:Math.round(performance.now()-started),detail:(error as Error).message.slice(0,300)}};
  }finally{client.close();}
 }
 let server=null;
 if(scheduler!==null){
  try{
   const response=await fetch(scheduler.replace(/\/+$/u,"")+"/status",{signal:AbortSignal.any([context.signal,AbortSignal.timeout(5000)]),headers:workerToken?{authorization:`Bearer ${workerToken}`}:token?{authorization:`Bearer ${token}`}:{}});
   if(response.ok){server=await response.json();requireRemoteExecutionShapeV1("ServerStatus",server,"Closed remote execution server status required");}else await response.body?.cancel();
  }catch(error){if(context.signal.aborted)throw error;server=null;}
 }
 const status={version:1,configured:endpoint!==undefined,contract,scheduler,grpc,server,token:token===undefined&&workerToken===undefined?null:{environment:names.workerToken,present:workerToken!==undefined}};
 requireRemoteExecutionShapeV1("Status",status,"Closed remote execution status required");
 return status;
}

/** 🎬️ Routes `remote-execution serve|status|workers`; `args` are the words after `remote-execution`. */
export async function runRemoteExecutionCommandV1(args:readonly string[],context:RemoteExecutionCommandContextV1):Promise<RemoteExecutionCommandResultV1>{
 const parsed=parseRemoteExecutionArgumentsV1(args);
 if(parsed.operation==="serve")return serveRemoteExecutionV1(args.slice(1),context);
 if(parsed.operation==="workers")return runRemoteExecutionWorkersV1(args.slice(1),context);
 return {exitCode:0,result:await remoteExecutionStatusV1(args.slice(1),context)};
}

