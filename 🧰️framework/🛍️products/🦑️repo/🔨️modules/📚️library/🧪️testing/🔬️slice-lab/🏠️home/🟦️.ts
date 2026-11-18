import {spawnSync} from "node:child_process";
import {chmodSync,copyFileSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,rmSync,statSync,symlinkSync,writeFileSync} from "node:fs";
import {hostname,tmpdir} from "node:os";
import {delimiter,dirname,join,sep} from "node:path";
import type {ReadyStageV1} from "../../../🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🏠️ The authored home protocol of a lab machine. @see ./🧬️schema/🔣️.json */
export const SLICE_LAB_HOME_V1=schema.definitions.Policy.const;
const H=SLICE_LAB_HOME_V1,texts=schema.definitions.Texts.const,words=(key:keyof typeof texts.en,fields:Readonly<Record<string,string|number>>={})=>(["en","de"] as const).map(language=>(texts[language][key] as string).replace(/\{(\w+)\}/gu,(_all,name:string)=>String(fields[name]??""))).join("\n");
const under=(path:string,directory:string)=>path===directory||path.startsWith(directory+sep);
const round=(value:number)=>Math.round(value*10)/10;

/** 🧰️ The tools a slice prepares: the union, in the order of the kinds, of the tools each declared toolchain kind needs; `"all"` when the kinds are not declared (the real stage then prepares everything); an undeclared kind is a refusal in both languages. */
export function sliceLabToolsForKindsV1(kinds:readonly string[]|undefined):readonly string[]|"all"{
 if(kinds===undefined)return"all";
 const table=H.kinds as Readonly<Record<string,readonly string[]>>,found:string[]=[];
 for(const kind of kinds){
  const ids=table[kind];
  if(ids===undefined)throw Error(words("unknownKind",{kind,kinds:Object.keys(table).join(", ")}));
  for(const id of ids)if(!found.includes(id))found.push(id);
 }
 return found;
}

/** 🔖️ The toolchain kinds of the owners of a slice, read from the declared probe table (`name`, `kind`); `explicit` kinds win; an owner the table does not declare makes the kinds unknown (`undefined`). */
export function sliceLabKindsOfSliceV1(slice:readonly string[],probes:readonly Readonly<{name:string;kind:string}>[],explicit?:readonly string[]):readonly string[]|undefined{
 if(explicit!==undefined)return explicit;
 const kinds:string[]=[];
 for(const name of slice){const row=probes.find(probe=>probe.name===name);if(row===undefined)return undefined;if(!kinds.includes(row.kind))kinds.push(row.kind);}
 return kinds;
}

/** 🧰️ The tools whose seeds a machine needs: the tools of the slice, and with `--stages=all` also everything the real `native` stage provisions (`setup` has no component selection but `rust` and `node`: it links Go, installs Node, rustup with the pinned nightly, .NET and uv whatever the slice is). */
export function sliceLabSeedToolsV1(tools:readonly string[]|"all",stages:"workspace"|"all"|undefined):readonly string[]|"all"{
 if(tools==="all"||stages!=="all")return tools;
 return[...tools,...H.nativeTools.filter(id=>!tools.includes(id))];
}

/** 🌱️ One read-only seed of a lab home: a tree or a file of the live account that the machine gets as a copy-on-write clone. */
export type SliceLabSeedV1=Readonly<{id:string;source:string;target:string;kind:string}>;
/** 🌱️ The seeds a machine needs for its tools: every seed without a tool is always there (launcher, repo binary, runtimes), every other only when one of its tools is prepared. */
export function sliceLabHomeSeedsV1(tools:readonly string[]|"all",live:string,home:string):readonly SliceLabSeedV1[]{
 return H.seeds.filter(seed=>seed.tools.length===0||tools==="all"||seed.tools.some(id=>tools.includes(id))).map(seed=>({id:seed.id,source:join(live,...seed.path.split("/")),target:join(home,...seed.path.split("/")),kind:seed.kind}));
}

/** 🌿️ The environment of a machine over a base environment: the lab home is the account, every tool home of the live account is dropped, no PATH entry lies below the live account, every network client is pointed at a refusing proxy (loopback stays reachable), and the download log is named. */
export function sliceLabHomeEnvironmentV1(base:Readonly<Record<string,string|undefined>>,input:Readonly<{home:string;live:string;downloads:string}>):Record<string,string>{
 const env:Record<string,string>={};
 for(const[name,value]of Object.entries(base))if(value!==undefined&&!(H.droppedEnvironment as readonly string[]).includes(name))env[name]=value;
 env.PATH=(base.PATH??"").split(delimiter).filter(entry=>entry!==""&&!under(entry,input.live)).join(delimiter);
 env.HOME=input.home;
 const proxy=base[H.proxyOptIn];
 if(proxy!==undefined&&proxy!==""){for(const name of["HTTPS_PROXY","HTTP_PROXY","ALL_PROXY","https_proxy","http_proxy","all_proxy"])env[name]=proxy;const bypass=`${H.noProxy},${hostname()}`;env.NO_PROXY=bypass;env.no_proxy=bypass;}
 Object.assign(env,H.forcedEnvironment);env[H.downloadEnvironment]=input.downloads;
 return env;
}

/** 🕳️ The names of the environment variables with a path of the live account (below `live` but outside the checkout) or of the live state of the checkout (`.cache`, `.nx`): paths a machine must not be given; `allowed` names are skipped. */
export function sliceLabLeaksV1(env:Readonly<Record<string,string|undefined>>,live:string,root:string,allowed:readonly string[]=[]):readonly string[]{
 const state=[join(root,".cache"),join(root,".nx")];
 return Object.entries(env).filter(([name,value])=>value!==undefined&&!allowed.includes(name)&&value.split(delimiter).some(part=>state.some(path=>under(part,path))||under(part,live)&&!under(part,root))).map(([name])=>name).sort();
}

/** 🚫️ Writes one refusing program per download client into the machine's `.local/bin` (first on the PATH of a stage): it logs `program arguments` to the download log and exits with the refusal status, so a stage that would download fails at that call with the URL in its tail, and nothing is fetched. */
export function sliceLabShimsV1(home:string,downloads:string):void{
 const directory=join(home,...H.shimDirectory.split("/"));mkdirSync(directory,{recursive:true});
 for(const name of H.refusedPrograms){
  const refusal=words("refused",{program:name,arguments:"$*"}).split("\n").map(line=>`printf '%s\\n' "${line}" >&2`).join("\n");
  const file=join(directory,name);
  writeFileSync(file,`#!/bin/sh\nprintf '%s\\n' "${name} $*" >> "\${${H.downloadEnvironment}:-/dev/null}"\n${refusal}\nexit ${H.refusedExit}\n`);chmodSync(file,0o755);
  void downloads;
 }
}

/** 🧬️ How a seed was copied: `clonefile` (one copy-on-write call for a whole tree on APFS), `reflink` (Linux), or `copy`. */
export type SliceLabCloneRowV1=Readonly<{method:"clonefile"|"reflink"|"copy";seconds:number}>;
let cloneFile:((source:string,target:string)=>number)|undefined;
const clonefileV1=():((source:string,target:string)=>number)=>{
 if(cloneFile!==undefined)return cloneFile;
 const{dlopen,FFIType}=require("bun:ffi") as typeof import("bun:ffi"),library=dlopen("/usr/lib/libSystem.dylib",{clonefile:{args:[FFIType.cstring,FFIType.cstring,FFIType.u32],returns:FFIType.i32}});
 cloneFile=(source,target)=>library.symbols.clonefile(Buffer.from(source+"\0"),Buffer.from(target+"\0"),0);
 return cloneFile;
};
/** 🧬️ Copies a file or tree without copying its bytes where the file system can clone: `clonefile(2)` on macOS (one call per tree), `cp --reflink=auto` on Linux, a plain recursive copy otherwise. The target must not exist. Throws when nothing worked. */
export function sliceLabCloneV1(source:string,target:string):SliceLabCloneRowV1{
 const started=performance.now(),done=(method:SliceLabCloneRowV1["method"])=>({method,seconds:round((performance.now()-started)/1000)});
 mkdirSync(dirname(target),{recursive:true});
 if(process.platform==="darwin"){try{if(clonefileV1()(source,target)===0)return done("clonefile");}catch{}}
 if(process.platform==="linux"){const run=spawnSync("cp",["-a","--reflink=auto",source,target]);if(run.status===0)return done("reflink");}
 const run=spawnSync("cp",["-R",source,target],{encoding:"utf8"});
 if(run.status!==0)throw Error(`seed copy failed / Saat nicht kopiert: ${source}: ${run.stderr}`);
 return done("copy");
}

/** 🌱️ The pinned Go toolchain the launcher installer looks for in `<go home>/v<version>`: the copy the live checkout holds in its tools partition (`.cache/b/t/m/native/go/<version>/<platform>-<arch>-<digest>`), one seed per version found (another volume, so a real copy). */
export function sliceLabPinnedGoSeedsV1(checkout:string,home:string):readonly SliceLabSeedV1[]{
 const base=join(checkout,".cache","b","t","m","native","go");
 try{return readdirSync(base).sort().flatMap(version=>readdirSync(join(base,version)).sort().slice(0,1).map(name=>({id:"go-pinned",source:join(base,version,name),target:join(home,".local","share","semio","go",`v${version}`),kind:"tree"})));}catch{return[];}
}

/** 🔗️ The user tool links of the account (`~/.local/bin/<name>`) that the live account holds as links into its own tool homes, made again as links into the lab home for the tools prepared: the real worker has them from its earlier setup, `native` probes `dotnet` and `cargo` through them. */
export function sliceLabLinkHomeV1(tools:readonly string[]|"all",home:string):readonly string[]{
 const made:string[]=[];
 for(const link of H.links)if((tools==="all"||link.tools.length===0||link.tools.some(id=>tools.includes(id)))&&existsSync(join(home,...link.to.split("/")))){
  const at=join(home,".local","bin",link.name);
  try{mkdirSync(dirname(at),{recursive:true});if(!existsSync(at)){symlinkSync(join(home,...link.to.split("/")),at);made.push(link.name);}}catch{}
 }
 return made;
}

/** 🌱️ The result of seeding one path: `ok` false (nothing copied) when the live source is absent or the copy failed. */
export type SliceLabSeedRowV1=Readonly<{id:string;ok:boolean;method:string;seconds:number}>;
/** 🌱️ Seeds a lab home from the live account: skips a source that is absent and a target that already exists, never writes below the source. */
export function sliceLabSeedHomeV1(seeds:readonly SliceLabSeedV1[]):readonly SliceLabSeedRowV1[]{
 return seeds.map(seed=>{
  if(!existsSync(seed.source))return{id:seed.id,ok:false,method:"-",seconds:0};
  if(existsSync(seed.target))return{id:seed.id,ok:true,method:"present",seconds:0};
  try{const row=sliceLabCloneV1(seed.source,seed.target);return{id:seed.id,ok:true,method:row.method,seconds:row.seconds};}catch{return{id:seed.id,ok:false,method:"-",seconds:0};}
 });
}

const walk=(path:string,depth:number,rows:Record<string,string>,ignore:readonly string[]=[]):void=>{
 let status;try{status=lstatSync(path);}catch{return;}
 rows[path]=`${status.isDirectory()?"d":status.isSymbolicLink()?"l":"f"}:${status.mtimeMs}:${status.isDirectory()?0:status.size}`;
 if(!status.isDirectory()||depth<=0)return;
 let names:string[];try{names=readdirSync(path);}catch{return;}
 for(const name of names)if(!ignore.includes(name))walk(join(path,name),depth-1,rows);
};
/** 📸️ A listing of what must not change while a lab runs: two levels below every watched path of the live account and of the temporary directory, one level of the live state of the checkout (`.cache`, `b`, `f`, `boot-recovery`; the scratch directories of peers are left out). Path to `kind:mtime:size`. */
export function sliceLabLiveSnapshotV1(account:string,checkout:string):Readonly<Record<string,string>>{
 const rows:Record<string,string>={};
 for(const name of H.liveWatch)walk(join(account,...name.split("/")),2,rows);
 for(const path of H.temporaryWatch)walk(path,2,rows);
 for(const name of H.liveStateWatch){
  const path=name===""?join(checkout,".cache"):join(checkout,".cache",...name.split("/")),listed:Record<string,string>={};
  walk(path,1,listed,["scratch","test-admission","test-sandbox"]);Object.assign(rows,listed);
 }
 return rows;
}
/** 📸️ The paths added, removed or changed between two listings, sorted. */
export function sliceLabLiveDiffV1(before:Readonly<Record<string,string>>,after:Readonly<Record<string,string>>):readonly string[]{
 const names=new Set([...Object.keys(before),...Object.keys(after)]);
 return[...names].filter(name=>before[name]!==after[name]).sort();
}

/** 🪜️ The stage table of a machine: every stage of the plan with its seconds and verdict (`ok`, `failed (code, exit n)`, the outcome of its timing, `running` for the stage a run ended in, or `not reached`). Without a timing (the run ended before the driver filed its result) the seconds are the ones the samples saw the stage for (`observed`). */
export function sliceLabStageTableV1(stages:readonly Readonly<{name:string}>[],timings:readonly Readonly<{stage:string;seconds:number;outcome:string}>[],failure:Readonly<{stage:string;code:string;exit:number|null}>|null,observed:Readonly<{seconds:Readonly<Record<string,number>>;last:string|null}>={seconds:{},last:null}):readonly (readonly [string,string,string])[]{
 return stages.map(stage=>{
  const timing=[...timings].reverse().find(row=>row.stage===stage.name),failed=failure!==null&&failure.stage===stage.name,seen=observed.seconds[stage.name];
  const seconds=timing!==undefined?String(round(timing.seconds)):seen!==undefined?String(round(seen)):"-";
  if(failed)return[stage.name,seconds,`failed (${failure.code}, exit ${failure.exit??"-"})`] as const;
  if(timing!==undefined)return[stage.name,seconds,timing.outcome] as const;
  if(seen!==undefined)return[stage.name,seconds,observed.last===stage.name?"running":"passed (seen in samples)"] as const;
  return[stage.name,"-","not reached"] as const;
 });
}

/** ⏱️ The seconds each stage was seen in the samples of one machine (a sample is `step` seconds), and the stage of its last sample. */
export function sliceLabObservedStagesV1(samples:readonly Readonly<{machine:string;stage:string|null}>[],machine:string,step:number):Readonly<{seconds:Readonly<Record<string,number>>;last:string|null}>{
 const seconds:Record<string,number>={};let last:string|null=null;
 for(const sample of samples)if(sample.machine===machine&&sample.stage!==null){seconds[sample.stage]=round((seconds[sample.stage]??0)+step);last=sample.stage;}
 return{seconds,last};
}

/** 🏠️ What a run adds to its verdict: the tools prepared, the seeds with method and seconds, the stage table of each machine, the paths outside the seam that changed, the downloads the lab refused. */
export type SliceLabHomeReportV1=Readonly<{tools:readonly string[]|"all";kinds:readonly string[]|undefined;seeds:readonly Readonly<{machine:string;id:string;ok:boolean;method:string;seconds:number}>[];stageTables:Readonly<Record<string,readonly (readonly [string,string,string])[]>>;liveChanges:readonly string[];downloads:readonly string[]}>;

const rowsOf=(rows:readonly (readonly string[])[])=>rows.map(row=>"| "+row.join(" | ")+" |");
/** 📜️ The home part of the report in one language: tools, seed table, stage tables, changes outside the seam, refused downloads. */
export function sliceLabHomeReportV1(language:"en"|"de",report:SliceLabHomeReportV1):readonly string[]{
 const t=texts[language],fill=(template:string,fields:Readonly<Record<string,string|number>>)=>template.replace(/\{(\w+)\}/gu,(_all,name:string)=>String(fields[name]??"")),sample=(rows:readonly string[])=>rows.length===0?"":": "+rows.slice(0,12).join(", ")+(rows.length>12?`, ... (+${rows.length-12})`:"");
 const lines=[`- ${report.tools==="all"?t.toolsAll:fill(t.tools,{tools:report.tools.join(" ")})}`,`- ${fill(t.liveChanged,{count:report.liveChanges.length,sample:sample(report.liveChanges)})}`,`- ${fill(t.downloads,{count:report.downloads.length,sample:sample(report.downloads)})}`,""];
 lines.push(...rowsOf([t.seedHeader.split(" | ").concat("machine"),["---","---","---","---"],...report.seeds.map(row=>[row.id,row.ok?row.method:"absent",String(row.seconds),row.machine])]),"");
 for(const[machine,rows]of Object.entries(report.stageTables))lines.push(`${machine}`,"",...rowsOf([t.stageHeader.split(" | "),["---","---","---"],...rows]),"");
 return lines;
}


/** 📵️ Writes the two files every package manager of the account reads (`.npmrc`, `.bunfig.toml`) so that the registry is a loopback address nobody listens on: a dependency the seeded cache does not hold fails at once with a refused connection instead of fetching (the dependency producer copies both files from the account into its own home). */
export function sliceLabRegistryRefusalV1(home:string):readonly string[]{
 const rule=H.registry,made:string[]=[];
 for(const[name,template]of Object.entries(rule.files)){writeFileSync(join(home,name),(template as string).replaceAll("{url}",rule.url));made.push(name);}
 return made;
}

/** 🍲️ One partition item of the warm table: what a stage finds already in its partition on a worker that has run before. */
export type SliceLabWarmItemV1=Readonly<{stage:string;partition:string;id:string;path:string}>;
/** 🍲️ The items of a stage for the tools prepared: every item without a tool, and every item one of whose tools is prepared (`"all"` takes every item). */
export function sliceLabWarmItemsV1(stage:string,tools:readonly string[]|"all"):readonly SliceLabWarmItemV1[]{
 return H.warm.filter(row=>row.stage===stage).flatMap(row=>row.items.filter(item=>item.tools.length===0||tools==="all"||item.tools.some(id=>tools.includes(id))).map(item=>({stage:row.stage,partition:row.partition,id:item.id,path:item.path})));
}
/** 🍲️ The result of warming one item: `skipped` names the reason (`no-source`, `not-mounted`, `copy-failed`) when nothing was copied. */
export type SliceLabWarmRowV1=Readonly<{stage:string;id:string;ok:boolean;method:string;seconds:number;megabytes:number}>;
const mountedPartitionV1=(state:string,partition:string):boolean=>{try{return statSync(join(state,"b",partition,"m")).dev!==statSync(join(state,"b",partition)).dev;}catch{return false;}};
const megabytesOfV1=(path:string):number=>{const run=spawnSync("du",["-sk",path],{encoding:"utf8"});return Math.round(Number((run.stdout??"").split(/\s/u)[0]??0)/102.4)/10;};
const copyEntriesV1=(source:string,target:string):boolean=>{
 mkdirSync(target,{recursive:true});
 return spawnSync("sh",["-c",'find "$1" -mindepth 1 -maxdepth 1 -print0 | xargs -0 -P "$3" -I{} cp -R {} "$2"/',"sh",source,target,String(H.warmConcurrency)],{encoding:"utf8"}).status===0;
};
/** 🍲️ Fills the mounted partitions of a lab machine with what the live checkout holds in the same places (a read of the source, a copy into the target: the partitions are images of their own, so no clone exists across them), for one stage and the tools it prepares: a target that already holds entries is left as it is. Never writes below the source; every row says how long it took and how much it moved. `mounted` is the partition test (the real one compares device numbers of `<state>/b/<partition>/m` and its parent). */
export function sliceLabWarmV1(input:Readonly<{state:string;source:string;stage:string;tools:readonly string[]|"all";mounted?:(state:string,partition:string)=>boolean}>):readonly SliceLabWarmRowV1[]{
 const mounted=input.mounted??mountedPartitionV1;
 return sliceLabWarmItemsV1(input.stage,input.tools).map(item=>{
  const source=join(input.source,item.partition,...item.path.split("/")),target=join(input.state,"b",item.partition,"m",...item.path.split("/")),started=performance.now(),row=(ok:boolean,method:string,megabytes=0)=>({stage:item.stage,id:item.id,ok,method,seconds:round((performance.now()-started)/1000),megabytes});
  if(!existsSync(source))return row(true,"skipped: no-source");
  if(!mounted(input.state,item.partition))return row(true,"skipped: not-mounted");
  try{if(readdirSync(target).length>0)return row(true,"present");}catch{}
  if(!copyEntriesV1(source,target))return row(false,"skipped: copy-failed");
  return row(true,"copy",megabytesOfV1(target));
 });
}
/** 🥘️ Builds the warm directory a lab reads (`<out>/<partition>/<path>` for every item of the table) from the live checkout's partitions, read only: the one slow copy (the dependency cache is several gigabytes in another disk image), done once and then used by every run. An item that is already in `out` is kept. */
export function sliceLabWarmBuildV1(live:string,out:string):readonly SliceLabWarmRowV1[]{
 return H.warm.flatMap(row=>row.items.map(item=>{
  const source=join(live,".cache","b",row.partition,"m",...item.path.split("/")),target=join(out,row.partition,...item.path.split("/")),started=performance.now(),made=(ok:boolean,method:string,megabytes=0)=>({stage:row.stage,id:item.id,ok,method,seconds:round((performance.now()-started)/1000),megabytes});
  if(!existsSync(source))return made(true,"skipped: no-source");
  if(existsSync(target))return made(true,"present");
  return copyEntriesV1(source,target)?made(true,"copy",megabytesOfV1(target)):made(false,"skipped: copy-failed");
 }));
}
/** 🧯️ The environment of one stage over the machine's: the declared variables set and unset for that stage only. The `dependencies` stage cannot run under the offline authority of the other stages (its producer then refuses to install anything that is not already a ready generation), so for it the offline switches are lifted and every network client is pointed at a refusing proxy instead (loopback stays reachable), next to the refused registry of the account. */
export function sliceLabStageEnvironmentV1(stage:string,base:Readonly<Record<string,string|undefined>>):Record<string,string|undefined>{
 const rule=(H.stageEnvironment as Readonly<Record<string,Readonly<{unset:readonly string[];set:Readonly<Record<string,string>>}>>>)[stage],env:Record<string,string|undefined>={...base};
 if(rule===undefined)return env;
 for(const name of rule.unset)delete env[name];
 Object.assign(env,rule.set);
 return env;
}

/** 🏗️ The launcher of a lab machine built from the sources of the tree it reads (what the `native` stage does on a worker: it builds the launcher from the checkout, so a change of the Go sources reaches the machine): the member `📦️packages/🐹️go` files are copied into a private module, built offline (`GOWORK=off GOTOOLCHAIN=local GOPROXY=off CGO_ENABLED=0`) with the Go of the lab home, signed ad hoc on macOS, and written over the launcher binary of the home by removing it first (an in-place overwrite of a running binary is killed by macOS). Never touches the live account. `ok` false with a reason when no Go or no sources are there. */
export function sliceLabLauncherBuildV1(root:string,home:string,go?:string):Readonly<{ok:boolean;seconds:number;reason:string}>{
 const started=performance.now(),done=(ok:boolean,reason:string)=>({ok,seconds:round((performance.now()-started)/1000),reason});
 const module=join(root,"🧰️framework","🛍️products","🦑️repo","🔨️modules","🚀️launcher","📦️packages","🐹️go"),compiler=go??join(home,".local","go","bin","go"),target=join(home,".local","share","semio","launcher","bin","repo");
 if(!existsSync(compiler))return done(false,"no-go");
 if(!existsSync(join(module,"🐹️.go")))return done(false,"no-sources");
 const work=mkdtempSync(join(tmpdir(),"slw-lb-"));
 try{
  mkdirSync(join(work,"cmd"));writeFileSync(join(work,"go.mod"),"module github.com/usalu/semio/repo/launcher\n\ngo 1.25\n");
  let index=0;
  for(const source of[join(module,"🐹️.go"),...readdirSync(module).sort().map(name=>join(module,name,"🐹️.go"))]){
   if(!existsSync(source))continue;
   if(source.includes("📦️main")){copyFileSync(source,join(work,"cmd","main.go"));continue;}
   index++;copyFileSync(source,join(work,`s${index}.go`));
  }
  const build=spawnSync(compiler,["build","-trimpath","-ldflags","-s -w","-o",join(work,"repo"),"./cmd"],{cwd:work,encoding:"utf8",env:{PATH:"/usr/bin:/bin",HOME:home,GOWORK:"off",GOTOOLCHAIN:"local",GOPROXY:"off",CGO_ENABLED:"0",GOCACHE:join(work,"gocache"),GOPATH:join(work,"gopath")}});
  if(build.status!==0)return done(false,`build-failed: ${String(build.stderr).trim().split("\n")[0]}`);
  if(process.platform==="darwin")spawnSync("codesign",["-s","-","--identifier","tech.semio.repo","--force",join(work,"repo")]);
  mkdirSync(dirname(target),{recursive:true});rmSync(target,{force:true});copyFileSync(join(work,"repo"),target);chmodSync(target,0o755);
  return done(true,"built");
 }finally{rmSync(work,{recursive:true,force:true});}
}

/** 🪜️ The plan of a worker, stage by stage as the readiness loop runs it (`native`, `repo-mcp`, `dependencies` exactly as declared), with the `tools` stage prepared for the tools of the slice only and the `workspace` stage replaced by the slice's read. */
export function sliceLabAllStagesV1(real:readonly ReadyStageV1[],workspace:ReadyStageV1,tools:readonly string[]|"all"):readonly ReadyStageV1[]{
 if(!real.some(stage=>stage.name==="workspace"))throw Error("The plan of a worker names a workspace stage / Der Plan eines Workers nennt eine workspace-Stufe");
 return Object.freeze(real.map(stage=>stage.name==="workspace"?workspace:stage.name==="tools"?{...stage,arguments:["tools","setup",...tools==="all"?["all"]:["toolchains",...tools]]}:stage));
}

/** ⏭️ Replaces the named stages of a plan by `repo --help` (a stage the machine cannot pass for a reason of its own, recorded as a wall, so the later stages can still be measured); an unknown name is a refusal. */
export function sliceLabSkipStagesV1(stages:readonly ReadyStageV1[],names:readonly string[]|undefined):readonly ReadyStageV1[]{
 for(const name of names??[])if(!stages.some(stage=>stage.name===name))throw Error(`unknown stage ${name}; stages: ${stages.map(stage=>stage.name).join(", ")} / unbekannte Stufe ${name}`);
 return Object.freeze(stages.map(stage=>(names??[]).includes(stage.name)?{...stage,kind:"repo" as const,arguments:["--help"],timeoutMilliseconds:120000,inputs:[],bootstrapInput:false}:stage));
}
