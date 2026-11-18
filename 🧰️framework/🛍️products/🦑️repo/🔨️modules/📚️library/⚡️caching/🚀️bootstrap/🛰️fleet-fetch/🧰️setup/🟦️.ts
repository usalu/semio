import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {createHash,randomBytes} from "node:crypto";
import {cp,lstat,mkdir,readdir,readFile,readlink,rename,rm} from "node:fs/promises";
import {existsSync} from "node:fs";
import {dirname,join,posix} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import type {FleetFetchContextV1} from "../🟦️.ts";
import {fleetInstallAcquireV1,fleetInstallHostV1,fleetInstallKeyV1,fleetInstallPublishV1,type FleetInstallEventV1,type FleetInstallRequestV1,type FleetInstallRuntimeV1,type FleetInstallShapeV1} from "../📦️install/🟦️.ts";
import {platformClassV1} from "../../../../🏗️build/🪪️graph-state/🌍️platform-class/🟦️.ts";

/** 📐️ The authored policy of setup records: space, retention, staging place. @see ./🧬️schema/🔣️.json */
export const FLEET_SETUP_POLICY_V1=schema.definitions.Policy.const;
/** 🗒️ The answer for every node of the closure of the root task `setup`: cacheable, looked up, uploaded, host-bound, class. @see ./🧬️schema/🔣️.json */
export const FLEET_SETUP_FAMILY_V1=schema.definitions.Family.const;
/** 🔩️ The executables each `cargo install --locked <crate>` leaves in `<cargo home>/bin`. @see ./🧬️schema/🔣️.json */
export const FLEET_CARGO_TOOLS_V1:Readonly<Record<string,readonly string[]>>=schema.definitions.CargoTools.const;
const policy=FLEET_SETUP_POLICY_V1,texts=schema.definitions.Texts.const,reasons=schema.definitions.Reasons.const as Readonly<Record<string,string>>;

/** 🏷️ How far the output of a task travels: `neutral` (byte-identical everywhere), `posix` (the class of graph-state/platform-class, trees without native code), `host` (OS, architecture and libc, anything with a native binary). */
export type SetupKindV1="neutral"|"posix"|"host";
/** 🖥️ The facts of a machine that decide its class. */
export type SetupHostV1=Readonly<{platform:string;arch:string;libc?:"gnu"|"musl"}>;
/** 🌍️ The class of a machine for one kind: the runtime fields of the record key. `undefined` for a platform that has no shareable class. */
export type SetupClassV1=Readonly<{tag:string;platform:FleetInstallRuntimeV1["platform"];arch:string}>;

const PLATFORMS=["darwin","linux","win32"] as const;

/** 💻️ The host of this process; the libc of Linux is told by the musl loader's presence. */
export function setupHostV1(platform:string=process.platform,arch:string=process.arch,list?:()=>readonly string[]):SetupHostV1{
 return fleetInstallHostV1(platform,arch,list);
}

/** 🧭️ The class under which a machine stores and looks up the output of a task of this kind. `neutral` is one class for everybody, `posix` follows `graph-state/platform-class` (`posix-shared` for macOS and Linux, Windows alone), `host` is the exact OS, architecture and libc. */
export function setupClassV1(kind:SetupKindV1,host:SetupHostV1=setupHostV1()):SetupClassV1|undefined{
 if(kind==="neutral")return {tag:"any",platform:"any",arch:"any"};
 const exact=`${host.platform}-${host.arch}`;
 if(kind==="posix"){
  const tag=platformClassV1(exact);
  if(tag.startsWith("posix-"))return {tag,platform:"posix",arch:tag.slice(6)};
  return (PLATFORMS as readonly string[]).includes(host.platform)?{tag:exact,platform:host.platform as FleetInstallRuntimeV1["platform"],arch:host.arch}:undefined;
 }
 if(!(PLATFORMS as readonly string[]).includes(host.platform))return undefined;
 const arch=host.libc===undefined?host.arch:`${host.arch}-${host.libc}`;
 return {tag:`${host.platform}-${arch}`,platform:host.platform as FleetInstallRuntimeV1["platform"],arch};
}

const sha=(text:string):string=>createHash("sha256").update(text).digest("hex");
const sorted=(inputs:Readonly<Record<string,string>>):readonly (readonly [string,string])[]=>Object.entries(inputs).sort(([left],[right])=>left<right?-1:left>right?1:0);

/** 🔑️ What the key of a record is made of: the digest of the real inputs (sorted names and values), the recipe (task, kind, roots) and the class. Nothing of the machine, the checkout path or the time. */
export function setupKeyPartsV1(task:string,inputs:Readonly<Record<string,string>>,kind:SetupKindV1,roots:readonly string[],klass:SetupClassV1):Readonly<{projectionSha256:string;runtime:FleetInstallRuntimeV1;key:string}>{
 const projectionSha256=sha(JSON.stringify([task,sorted(inputs)])),runtime:FleetInstallRuntimeV1={bun:task.slice(0,policy.taskLabelMaximum),platform:klass.platform,arch:klass.arch,recipeSha256:sha(JSON.stringify([policy.version,task,kind,[...roots].sort()]))};
 return {projectionSha256,runtime,key:fleetInstallKeyV1(projectionSha256,runtime)};
}

/** 🗣️ Renders one text in the language of the environment; without a resolvable language both languages are printed. */
export function setupTextV1(name:keyof typeof texts,values:Readonly<Record<string,string|number|boolean>>,environment:Readonly<Record<string,string|undefined>>=process.env):string{
 const row=texts[name] as Readonly<{en:string;de:string}>,fill=(text:string)=>text.replace(/\{(\w+)\}/gu,(_all,key:string)=>String(values[key]??`{${key}}`)),language=environment.SEMIO_LANGUAGE;
 return language==="de"?fill(row.de):language==="en"?fill(row.en):`${fill(row.en)} / ${fill(row.de)}`;
}

/** 🧾️ The shape of a staged tree: counts and a digest of its sorted entries (kind, size, execute bit, content digest, link target). */
async function shapeOf(directory:string,root:string):Promise<FleetInstallShapeV1>{
 const rows:string[]=[];let directories=0,files=0,links=0;
 const walk=async(path:string,relative:string):Promise<void>=>{
  for(const name of (await readdir(path)).sort()){
   const child=join(path,name),facts=await lstat(child),here=posix.join(relative,name);
   if(facts.isSymbolicLink()){links++;rows.push(`l ${here} ${await readlink(child)}`);}
   else if(facts.isDirectory()){directories++;rows.push(`d ${here}`);await walk(child,here);}
   else{files++;rows.push(`f ${here} ${facts.size} ${facts.mode&0o111?1:0} ${sha(await readFile(child,"latin1"))}`);}
  }
 };
 if(existsSync(join(directory,root))){rows.push(`r ${root}`);directories++;await walk(join(directory,root),root);}
 return {entries:rows.length,directories,files,links,sha256:sha(rows.join("\n"))};
}

/** 🚚️ Puts a root of the staged tree into its real place: copied beside the target and renamed over it, so a reader never sees a half written file or directory and a failure leaves the old state. A file or link is replaced by one rename (never absent); a directory is moved aside and the new one renamed in, which is the shortest gap a rename allows for a non-empty directory. */
export async function placeSetupRootV1(from:string,to:string):Promise<void>{
 const temporary=`${to}.setup-${randomBytes(4).toString("hex")}`;
 await mkdir(dirname(to),{recursive:true});
 try{
  await cp(from,temporary,{recursive:true,verbatimSymlinks:true,preserveTimestamps:true});
  const facts=await lstat(to).catch(()=>undefined);
  if(facts===undefined||!facts.isDirectory()){await rename(temporary,to);return;}
  const old=`${to}.old-${randomBytes(4).toString("hex")}`;
  await rename(to,old);
  try{await rename(temporary,to);}catch(error){await rename(old,to).catch(()=>{});throw error;}
  await rm(old,{recursive:true,force:true}).catch(()=>{});
 }finally{await rm(temporary,{recursive:true,force:true}).catch(()=>{});}
}

/** 📋️ What one output is: the task, its real inputs, its kind and where its roots live. */
export type SetupOutputV1=Readonly<{task:string;inputs:Readonly<Record<string,string>>;kind:SetupKindV1;base:string;roots:readonly string[]}>;
/** 🎛️ How a record is run: where it talks, how it is checked and what it says. */
export type SetupOptionsV1=Readonly<{say?:(line:string)=>void;check?:()=>void;ready?:()=>Promise<boolean>;verify?:()=>Promise<boolean>;host?:SetupHostV1;environment?:Readonly<Record<string,string|undefined>>;owner?:string}>;
/** 🏁️ What happened: `present` (nothing to do), `restored` (taken from the fleet, verified), `built` (built here; `published` says whether the fleet got it) or `local` (built here and not shareable, with the reason). */
export type SetupOutcomeV1=Readonly<{source:"present"|"restored"|"built"|"local";key?:string;class?:string;published:boolean;reason?:string;files:number;bytes:number}>;

/** 🌱️ Runs one setup output through the fleet: when the roots are not there, the record for these inputs and this class is restored (signature, block digests and per-file fingerprint verified, placed by rename); otherwise exactly one machine builds it (`build`) while the others wait for its upload, and the finished roots are published. A failing fleet, a missing store, an unshareable platform or a rejected restore each end in the plain local `build`; the outcome and the log line say which. */
export async function fleetSetupOutputV1(context:FleetFetchContextV1,spec:SetupOutputV1,build:()=>Promise<void>,options:SetupOptionsV1=Object.freeze({})):Promise<SetupOutcomeV1>{
 const environment=options.environment??process.env,say=options.say??((line:string)=>{try{process.stderr.write(line+"\n");}catch{}}),check=options.check??(()=>{});
 const nothing={files:0,bytes:0};
 if(options.ready!==undefined&&await options.ready()){say(setupTextV1("present",{task:spec.task},environment));return {source:"present",published:false,...nothing};}
 const local=async(reason:string):Promise<SetupOutcomeV1>=>{await build();say(setupTextV1("local",{task:spec.task,reason},environment));return {source:"local",published:false,reason,...nothing};};
 const klass=setupClassV1(spec.kind,options.host);
 if(klass===undefined)return local(reasons["no-class"]!);
 if(context.storeRoot===undefined||context.root===undefined)return local(reasons["no-store"]!);
 const parts=setupKeyPartsV1(spec.task,spec.inputs,spec.kind,spec.roots,klass),staging=machinePathV1(context.root,...policy.stagingDirectory,randomBytes(6).toString("hex")),out=policy.stageRoot;
 const events:FleetInstallEventV1[]=[];
 const request:FleetInstallRequestV1={
  context,projectionSha256:parts.projectionSha256,runtime:parts.runtime,directory:staging,lockFile:"",lock:{packages:spec.roots.length,sha256:parts.projectionSha256},space:policy.space,keep:policy.keepStates,
  storeRoot:spec.base,roots:[out],check,scan:()=>shapeOf(staging,out),remove:async path=>{await rm(path,{recursive:true,force:true});},installed:()=>parts.projectionSha256,
  ...(options.owner===undefined?{}:{owner:options.owner}),
  event:event=>{events.push(event);if(event.stage==="wait")say(setupTextV1("waiting",{task:spec.task},environment));},
 };
 const stage=async(from:string,to:string):Promise<void>=>{for(const root of spec.roots){const source=join(from,...root.split("/"));if(!existsSync(source))throw Error(`missing ${root}`);await mkdir(dirname(join(to,...root.split("/"))),{recursive:true});await cp(source,join(to,...root.split("/")),{recursive:true,verbatimSymlinks:true,preserveTimestamps:true});}};
 await mkdir(staging,{recursive:true});
 let acquisition:Awaited<ReturnType<typeof fleetInstallAcquireV1>>|undefined;
 try{
  acquisition=await fleetInstallAcquireV1(request);
  if(acquisition.restored){
   const shape=await shapeOf(staging,out);let placed=false,reason=reasons["check-failed"]!;
   try{
    for(const root of spec.roots)if(!existsSync(join(staging,out,...root.split("/"))))throw Error(`restored record lacks ${root}`);
    for(const root of spec.roots)await placeSetupRootV1(join(staging,out,...root.split("/")),join(spec.base,...root.split("/")));
    placed=options.verify===undefined||await options.verify();
   }catch(error){reason=`${reasons["check-failed"]}: ${error instanceof Error?error.message:String(error)}`;}
   if(placed){
    const bytes=await sizeOf(join(staging,out));
    say(setupTextV1("restored",{task:spec.task,files:shape.files,megabytes:(bytes/1048576).toFixed(1),class:klass.tag,key:parts.key.slice(5,17)},environment));
    return {source:"restored",key:parts.key,class:klass.tag,published:false,files:shape.files,bytes};
   }
   say(setupTextV1("rejected",{task:spec.task,reason},environment));
   for(const root of spec.roots)await rm(join(spec.base,...root.split("/")),{recursive:true,force:true}).catch(()=>{});
   await acquisition.release();acquisition=undefined;
   await rm(join(staging,out),{recursive:true,force:true});
   await build();
   return finish(true);
  }
  await build();
  return await finish(false);
 }finally{
  await acquisition?.release().catch(()=>{});
  await rm(staging,{recursive:true,force:true}).catch(()=>{});
 }
 async function finish(replace:boolean):Promise<SetupOutcomeV1>{
  let reason=reasons["miss"]!,published=false,shape:FleetInstallShapeV1={entries:0,directories:0,files:0,links:0,sha256:""};
  try{
   await rm(staging,{recursive:true,force:true});await mkdir(staging,{recursive:true});
   await stage(spec.base,join(staging,out));
   shape=await shapeOf(staging,out);
   const skip=()=>events.filter(event=>event.stage==="skip").at(-1)?.detail;
   published=replace?await fleetInstallPublishV1(request,true):await acquisition!.publish();
   if(!published)reason=skip()??reasons["no-identity"]!;
  }catch(error){
   check();reason=error instanceof Error&&error.message.startsWith("missing ")?`${reasons["no-output"]}: ${error.message}`:`${reasons["not-relocatable"]}: ${error instanceof Error?error.message:String(error)}`;
  }
  say(setupTextV1(published?"built":"local",{task:spec.task,reason:published?reasons["miss"]!:reason,class:klass!.tag,key:parts.key.slice(5,17),published},environment));
  return {source:published?"built":"local",key:parts.key,class:klass!.tag,published,...(published?{}:{reason}),files:shape.files,bytes:0};
 }
}

async function sizeOf(directory:string):Promise<number>{
 let bytes=0;
 const walk=async(path:string):Promise<void>=>{for(const name of await readdir(path)){const child=join(path,name),facts=await lstat(child);if(facts.isDirectory())await walk(child);else bytes+=facts.size;}};
 await walk(directory).catch(()=>{});return bytes;
}

/** 🧱️ The channel of the pinned Rust toolchain: the part of a binary's identity that `cargo install` takes from the compiler. */
export function rustChannelV1(toolchainFile:string):string|undefined{return /^\s*channel\s*=\s*"([^"]+)"/mu.exec(toolchainFile)?.[1];}

/** 🛠️ `cargo install --locked <crate> --version <version>` through the fleet: the executables the crate leaves in `<cargo home>/bin` are one record, keyed by crate, version and compiler channel, class `host` (they are native executables). A restored executable must pass `verify` (it prints the right version on this machine) or it is thrown away and compiled here. */
export async function fleetCargoToolV1(context:FleetFetchContextV1,input:Readonly<{crate:string;version:string;channel:string;home:string;windows?:boolean}>,build:()=>Promise<void>,options:SetupOptionsV1=Object.freeze({})):Promise<SetupOutcomeV1>{
 const names=FLEET_CARGO_TOOLS_V1[input.crate];
 if(names===undefined){await build();return {source:"local",published:false,reason:"no record layout for this crate",files:0,bytes:0};}
 const suffix=(input.windows??process.platform==="win32")?".exe":"";
 return fleetSetupOutputV1(context,{task:`cargo-tool:${input.crate}`,inputs:{crate:input.crate,version:input.version,channel:input.channel,locked:"true"},kind:"host",base:input.home,roots:names.map(name=>`bin/${name}${suffix}`)},build,options);
}
