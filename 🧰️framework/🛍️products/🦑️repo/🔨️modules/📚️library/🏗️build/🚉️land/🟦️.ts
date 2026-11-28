import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {appendFileSync,closeSync,existsSync,fsyncSync,mkdirSync,mkdtempSync,openSync,readFileSync,readdirSync,renameSync,rmSync,statSync,writeFileSync,writeSync} from "node:fs";
import {homedir,tmpdir} from "node:os";
import {basename,dirname,isAbsolute,join,relative,resolve,sep} from "node:path";
import policy from "./🔣️.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {importKeysV1} from "../🔬️typecheck/🟦️.ts";
import {refuseLandV1,type LandLanguageV1} from "./📖️texts/🟦️.ts";

/** 📐️ The closed policy of `repo land`: areas and slot rules, lock protocol, overlay roots, gates and limits.
 * @see ./🧬️schema/🔣️.json */
export type LandPolicyV1=Readonly<typeof policy>;
/** 📝️ The closed arguments of `repo land`. */
export type LandOptionsV1=Readonly<{words:readonly string[];json:boolean;dryRun:boolean;outage:boolean;rebase:boolean;noWait:boolean;bench:boolean;lane:string|undefined;title:string|undefined;wave:string|undefined;language:LandLanguageV1|undefined}>;
/** 🗂️ A staged changeset on disk: the base hash of every path when it was staged and the paths staged for deletion (`null` base = a new file). */
export type LandStageV1=Readonly<{version:1;id:string;lane:string;title:string;bases:Readonly<Record<string,string|null>>;deletes:readonly string[]}>;
/** 🧾️ One change: the content hash it was staged against and the one it leaves. */
export type LandEntryV1=Readonly<{path:string;op:"write"|"delete";base:string|null;after:string|null;area:string}>;
/** 📦️ A changeset: its entries and the new bytes of every write. */
export type LandChangesetV1=Readonly<{stage:LandStageV1;entries:readonly LandEntryV1[];bytes:ReadonlyMap<string,Uint8Array>}>;
/** 🚧️ Why a staged base no longer matches the live tree. */
export type LandBaseProblemV1=Readonly<{path:string;kind:"stale"|"exists"|"gone"|"deleted-changed";live:string|null}>;
/** 🎯️ The area a path belongs to and whether landing it is bound to a slot. */
export type LandAreaV1=Readonly<{area:string;slot:boolean}>;
/** 🕐️ The slot state at one moment and the next opening (local `HH:MM`). */
export type LandSlotStateV1=Readonly<{open:boolean;next:string;nextMs:number}>;
/** 📍️ A rollback point on disk: the entries as landed; the previous bytes sit beside it. */
export type LandRollbackPointV1=Readonly<{version:1;id:string;lane:string;title:string;atMs:number;entries:readonly LandEntryV1[]}>;

const rules=policy as LandPolicyV1,valued=["--lane","--title","--wave","--lang"] as const,plain=["--json","--dry-run","--outage","--rebase","--no-wait","--bench"] as const;
const posix=(path:string):string=>path.split(sep).join("/");
const keyOf=(file:string):string=>file.replace(/(?:\.d\.mts|\.mts|\.mjs|\.tsx|\.ts|\.js|\.jsx|\.cjs|\.cts)$/u,"");

/** 🛡️ Reads the policy under its closed schema. */
export function readLandPolicyV1(value:unknown=policy):LandPolicyV1{
 const errors=validateJsonSchemaSubset({$ref:"#/definitions/Policy"},value,schema as never);
 if(errors.length)throw Error("land-policy-invalid:"+JSON.stringify(errors[0]));
 return value as LandPolicyV1;
}

/** 🛂️ Validates a stage, result or rollback point against its closed schema; the problems are returned, never thrown. */
export function validateLandDocumentV1(definition:"Stage"|"Result"|"RollbackPoint"|"Policy",value:unknown):readonly string[]{
 return validateJsonSchemaSubset({$ref:`#/definitions/${definition}`},value,schema as never).map(error=>String(typeof error==="string"?error:(error as {message?:string}).message??JSON.stringify(error)));
}

/** 🚩️ The option names the parser accepts, so the operations catalogue can be checked against it. */
export function landOptionNamesV1():{valued:readonly string[];plain:readonly string[]}{return {valued,plain};}

/** 🧷️ Reads `land [options] <word>...`; unknown and unvalued options are refused in both languages, every word after `--` is positional. */
export function landArgumentsV1(args:readonly string[]):LandOptionsV1{
 const flags=new Set<string>(),values=new Map<string,string>(),words:string[]=[];
 let tail=false;
 for(let index=0;index<args.length;index++){
  const argument=args[index]!;
  if(tail||!argument.startsWith("-")||argument==="-"){words.push(argument);continue;}
  if(argument==="--"){tail=true;continue;}
  const cut=argument.indexOf("="),name=cut<0?argument:argument.slice(0,cut);
  if((valued as readonly string[]).includes(name)){const value=cut<0?args[++index]:argument.slice(cut+1);if(!value||value.startsWith("--"))throw refuseLandV1("usage","option.value",{option:name});values.set(name,value);}
  else if((plain as readonly string[]).includes(name)&&cut<0)flags.add(name);
  else throw refuseLandV1("usage","option.unknown",{option:argument});
 }
 const language=values.get("--lang");
 if(language!==undefined&&language!=="en"&&language!=="de")throw refuseLandV1("usage","language.unknown",{value:language});
 if(!words.length)throw refuseLandV1("usage","usage");
 return Object.freeze({words:Object.freeze(words),json:flags.has("--json"),dryRun:flags.has("--dry-run"),outage:flags.has("--outage"),rebase:flags.has("--rebase"),noWait:flags.has("--no-wait"),bench:flags.has("--bench"),lane:values.get("--lane"),title:values.get("--title"),wave:values.get("--wave"),language:language as LandLanguageV1|undefined});
}

/** #️⃣ The SHA-256 of bytes as lowercase hex. */
export const landHashV1=(bytes:Uint8Array):string=>createHash("sha256").update(bytes).digest("hex");

/** 🗂️ The machine cache directory of one checkout: the variable of the policy, else the user cache of the platform, one folder per checkout. */
export function landCacheDirectoryV1(root:string,environment:NodeJS.ProcessEnv=process.env,limits:LandPolicyV1=rules,platform:string=process.platform,home:string=homedir()):string{
 const chosen=environment[limits.overlay.variable],hash=createHash("sha256").update(root).digest("hex").slice(0,12);
 if(chosen)return resolve(chosen,hash);
 const parent=platform==="darwin"?join(home,"Library","Caches"):platform==="win32"?environment.LOCALAPPDATA??join(home,"AppData","Local"):environment.XDG_CACHE_HOME??join(home,".cache");
 return join(parent,"semio",limits.overlay.cacheDirectory,hash);
}

/** 🗂️ The short parent of the scratch overlays of one checkout (`~/.cache/semio/l/<6 hex>`): a Unix socket path must stay below `SUN_LEN` (104 on macOS), so an overlay may not sit deep in a platform cache directory; the variable of the policy, when set, still wins. */
export function landOverlayBaseV1(root:string,environment:NodeJS.ProcessEnv=process.env,limits:LandPolicyV1=rules,platform:string=process.platform,home:string=homedir()):string{
 const chosen=environment[limits.overlay.variable],hash=createHash("sha256").update(root).digest("hex").slice(0,6);
 if(chosen)return resolve(chosen,"o",hash);
 const parent=platform==="win32"?environment.LOCALAPPDATA??join(home,"AppData","Local"):join(home,".cache");
 return join(parent,"semio","l",hash);
}

/** 📍️ Resolves one path to a checkout-relative POSIX path of a regular-file candidate; protected folders, paths outside the checkout and overlong paths are refused. */
export function landPathV1(root:string,directory:string,input:string,limits:LandPolicyV1=rules):string{
 const base=resolve(root),candidate=isAbsolute(input)?resolve(input):existsSync(resolve(directory,input))&&relative(base,resolve(directory,input)).split(sep)[0]!==".."?resolve(directory,input):resolve(base,input),inside=relative(base,candidate);
 if(inside===""||inside===".."||inside.startsWith(".."+sep)||isAbsolute(inside))throw refuseLandV1("invalid","path.outside",{path:input});
 const name=posix(inside);
 if(name.length>limits.limits.maximumPathCharacters)throw refuseLandV1("invalid","path.long",{path:input});
 if(name.split("/").some(segment=>limits.refusedSegments.includes(segment)))throw refuseLandV1("invalid","path.segment",{path:input});
 return name;
}

const stripEmoji=(name:string):string=>name.replace(/^[^A-Za-z0-9]+/u,"");

/** 🎯️ The area of a path (the lock name) and whether it is slot-bound: the first folder below a build or caching root, the process module, else the fallback; manifests are slot-bound wherever they lie, tests and documents never. */
export function landAreaOfV1(path:string,limits:LandPolicyV1=rules):LandAreaV1{
 const rule=limits.areas.find(row=>path===row.prefix||path.startsWith(row.prefix+"/")),exempt=limits.slot.exemptSegments.some(segment=>("/"+path).includes(segment))||limits.slot.exemptExtensions.some(extension=>path.endsWith(extension)),name=basename(path);
 let area:string=limits.fallbackArea;
 if(rule){
  const rest=path.slice(rule.prefix.length+1).split("/"),first=rest.length>1?rest[0]!:basename(rule.prefix);
  area=rule.area==="derive"?("prefixLabel" in rule?rule.prefixLabel:"")+stripEmoji(first).replace(/\.[^.]*$/u,""):rule.area;
 }
 const slot=!exempt&&((rule?.slot??false)||limits.slot.boundBasenames.includes(name));
 return {area:area.toLowerCase().replace(/[^a-z0-9-]+/gu,"-").replace(/^-+|-+$/gu,"")||limits.fallbackArea,slot};
}

/** 🕐️ Whether the landing slot (minutes start..end of every hour) is open at a moment, and when it opens next. */
export function landSlotStateV1(now:Date,limits:LandPolicyV1=rules):LandSlotStateV1{
 const minute=now.getMinutes(),open=minute>=limits.slot.startMinute&&minute<=limits.slot.endMinute,next=new Date(now);
 next.setSeconds(0,0);
 if(open)return {open,next:"now",nextMs:now.getTime()};
 next.setMinutes(limits.slot.startMinute);
 if(next.getTime()<=now.getTime())next.setHours(next.getHours()+1);
 return {open,next:String(next.getHours()).padStart(2,"0")+":"+String(next.getMinutes()).padStart(2,"0"),nextMs:next.getTime()};
}

/** ⏳️ Whether a landing that began inside the slot may still finish: until the declared overrun after the slot's last minute. */
export function landSlotOverrunV1(beginMs:number,nowMs:number,limits:LandPolicyV1=rules):boolean{
 const begin=new Date(beginMs);
 if(!landSlotStateV1(begin,limits).open)return false;
 const end=new Date(begin);end.setMinutes(limits.slot.endMinute+1+limits.slot.overrunMinutes,0,0);
 return nowMs<end.getTime();
}

const readJson=(path:string):unknown=>JSON.parse(readFileSync(path,"utf8"));
const writeAtomic=(path:string,data:string|Uint8Array):void=>{
 mkdirSync(dirname(path),{recursive:true});
 const temporary=path+"."+process.pid+".tmp",handle=openSync(temporary,"w");
 try{writeFileSync(handle,data);fsyncSync(handle);}finally{closeSync(handle);}
 renameSync(temporary,path);
};
const liveHash=(root:string,path:string):string|null=>{
 try{return statSync(resolve(root,path)).isFile()?landHashV1(readFileSync(resolve(root,path))):null;}catch{return null;}
};

/** 📖️ Reads a stage directory; a missing or invalid `stage.json` is a typed refusal. */
export function readLandStageV1(directory:string):LandStageV1{
 const file=join(directory,"stage.json");
 if(!existsSync(file))throw refuseLandV1("missing","stage.missing",{directory});
 let value:unknown;
 try{value=readJson(file);}catch(error){throw refuseLandV1("invalid","stage.invalid",{directory,details:String((error as Error).message).slice(0,rules.limits.diagnosticCharacters)});}
 const problems=validateLandDocumentV1("Stage",value);
 if(problems.length)throw refuseLandV1("invalid","stage.invalid",{directory,details:problems.slice(0,3).join("; ")});
 return value as LandStageV1;
}

/** 📥️ Stages live files into `<directory>/tree/<path>` and remembers the content hash they had (the base); a path that does not exist is a new file. Files already staged keep their edits and their first base. */
export function landCheckoutV1(root:string,directory:string,paths:readonly string[],meta:Readonly<{lane:string;title:string}>,cwd:string=process.cwd(),limits:LandPolicyV1=rules):Readonly<{stage:LandStageV1;rows:readonly Readonly<{path:string;state:"existing"|"new"}>[]}>{
 const stage=existsSync(join(directory,"stage.json"))?readLandStageV1(directory):{version:1 as const,id:basename(resolve(directory)).replace(/[^A-Za-z0-9._-]+/gu,"-").replace(/^[^A-Za-z0-9]+/u,"").slice(0,64)||"stage",lane:meta.lane,title:meta.title,bases:{},deletes:[]},bases:Record<string,string|null>={...stage.bases},rows:{path:string;state:"existing"|"new"}[]=[];
 for(const input of paths){
  const path=landPathV1(root,cwd,input,limits),live=resolve(root,path);
  if(existsSync(live)&&!statSync(live).isFile())throw refuseLandV1("invalid","path.kind",{path:input});
  if(existsSync(live)){
   const bytes=readFileSync(live),sha=landHashV1(bytes);
   if(!(path in bases))bases[path]=sha;
   if(!existsSync(join(directory,"tree",path)))writeAtomic(join(directory,"tree",path),bytes);
   if(!existsSync(join(directory,"base",sha)))writeAtomic(join(directory,"base",sha),bytes);
   rows.push({path,state:"existing"});
  }else{if(!(path in bases))bases[path]=null;rows.push({path,state:"new"});}
 }
 const next:LandStageV1={...stage,lane:meta.lane||stage.lane,title:meta.title||stage.title,bases,deletes:stage.deletes.filter(path=>!(paths.some(input=>landPathV1(root,cwd,input,limits)===path)))};
 writeAtomic(join(directory,"stage.json"),JSON.stringify(next,null,1));
 return {stage:next,rows};
}

/** 🗑️ Stages the deletion of live files: the base is remembered and the staged copy dropped. */
export function landDeleteV1(root:string,directory:string,paths:readonly string[],cwd:string=process.cwd(),limits:LandPolicyV1=rules):LandStageV1{
 const stage=existsSync(join(directory,"stage.json"))?readLandStageV1(directory):{version:1 as const,id:basename(resolve(directory)).replace(/[^A-Za-z0-9._-]+/gu,"-").replace(/^[^A-Za-z0-9]+/u,"").slice(0,64)||"stage",lane:"agent",title:"",bases:{},deletes:[]},bases:Record<string,string|null>={...stage.bases},deletes=new Set(stage.deletes);
 for(const input of paths){
  const path=landPathV1(root,cwd,input,limits),sha=liveHash(root,path);
  if(sha===null)throw refuseLandV1("missing","base.gone",{path});
  if(!(path in bases))bases[path]=sha;
  const live=readFileSync(resolve(root,path));
  if(!existsSync(join(directory,"base",sha)))writeAtomic(join(directory,"base",sha),live);
  rmSync(join(directory,"tree",path),{force:true});deletes.add(path);
 }
 const next:LandStageV1={...stage,bases,deletes:[...deletes].sort()};
 writeAtomic(join(directory,"stage.json"),JSON.stringify(next,null,1));
 return next;
}

const treeFiles=(directory:string,found:string[]=[],prefix=""):string[]=>{
 if(!existsSync(directory))return found;
 for(const entry of readdirSync(directory,{withFileTypes:true})){
  const path=prefix===""?entry.name:prefix+"/"+entry.name;
  if(entry.isDirectory())treeFiles(join(directory,entry.name),found,path);else if(entry.isFile())found.push(path);
 }
 return found;
};

/** 🧱️ Builds the changeset of a stage: every staged file whose content differs from its base is a write, every staged deletion a delete; unchanged files drop out. */
export function landChangesetV1(directory:string,limits:LandPolicyV1=rules):LandChangesetV1{
 const stage=readLandStageV1(directory),tree=new Set(treeFiles(join(directory,"tree"))),deletes=new Set(stage.deletes),entries:LandEntryV1[]=[],bytes=new Map<string,Uint8Array>();
 let total=0;
 for(const path of [...new Set([...tree,...Object.keys(stage.bases),...deletes])].sort()){
  const area=landAreaOfV1(path,limits).area,base=stage.bases[path]??null;
  if(path.length>limits.limits.maximumPathCharacters||path.split("/").some(segment=>segment===".."||limits.refusedSegments.includes(segment)))throw refuseLandV1("invalid","path.segment",{path});
  if(deletes.has(path)){entries.push({path,op:"delete",base,after:null,area});continue;}
  if(!tree.has(path)){if(base!==null)throw refuseLandV1("invalid","tracked.missing",{path});continue;}
  const content=readFileSync(join(directory,"tree",path)),after=landHashV1(content);
  if(base!==null&&after===base)continue;
  total+=content.length;entries.push({path,op:"write",base,after,area});bytes.set(path,content);
 }
 if(entries.length>limits.limits.maximumEntries)throw refuseLandV1("invalid","entries.too-many",{maximum:limits.limits.maximumEntries,count:entries.length});
 if(total>limits.limits.maximumBytes)throw refuseLandV1("invalid","bytes.too-many",{maximum:limits.limits.maximumBytes});
 return Object.freeze({stage,entries:Object.freeze(entries),bytes});
}

/** 🔬️ Compares every staged base with the live tree and names each path that moved. */
export function landBaseProblemsV1(root:string,changeset:LandChangesetV1):readonly LandBaseProblemV1[]{
 const rows:LandBaseProblemV1[]=[];
 for(const entry of changeset.entries){
  const live=liveHash(root,entry.path);
  if(live===entry.base)continue;
  if(entry.op==="delete")rows.push({path:entry.path,kind:live===null?"gone":"deleted-changed",live});
  else if(entry.base===null)rows.push({path:entry.path,kind:"exists",live});
  else rows.push({path:entry.path,kind:live===null?"gone":"stale",live});
 }
 return Object.freeze(rows);
}

/** 🔀️ A three-way text merge: the merged bytes and the number of conflicts; `undefined` when no merge tool is available. */
export type LandMergeV1=(base:Uint8Array,ours:Uint8Array,theirs:Uint8Array)=>Readonly<{merged:Uint8Array;conflicts:number}>|undefined;

/** 🔌️ The default merge: `git merge-file -p` on three scratch files, which reads and writes nothing in any repository. */
export const landMergeGitV1:LandMergeV1=(base,ours,theirs)=>{
 const directory=mkdtempSync(join(tmpdir(),"semio-land-merge-"));
 try{
  for(const [name,bytes] of [["ours",ours],["base",base],["theirs",theirs]] as const)writeFileSync(join(directory,name),bytes);
  const run=spawnSync("git",["merge-file","-p","--",join(directory,"ours"),join(directory,"base"),join(directory,"theirs")],{encoding:"buffer",windowsHide:true});
  if(run.error||run.status===null||run.status<0||run.status>127)return undefined;
  return {merged:new Uint8Array(run.stdout),conflicts:run.status};
 }finally{rmSync(directory,{recursive:true,force:true});}
};

/** 🔁️ Rebases stale writes onto the live version by a three-way merge of (their base, our staged content, the live content); what merges cleanly gets the live hash as its new base, the rest stays a problem. */
export function landRebaseV1(root:string,directory:string,changeset:LandChangesetV1,problems:readonly LandBaseProblemV1[],merge:LandMergeV1=landMergeGitV1):Readonly<{changeset:LandChangesetV1;merged:readonly string[];remaining:readonly LandBaseProblemV1[];unavailable:boolean}>{
 const entries=new Map(changeset.entries.map(entry=>[entry.path,entry])),bytes=new Map(changeset.bytes),merged:string[]=[],remaining:LandBaseProblemV1[]=[];
 let unavailable=false;
 for(const problem of problems){
  const entry=entries.get(problem.path)!;
  if(problem.kind!=="stale"||entry.op!=="write"||entry.base===null||problem.live===null||!existsSync(join(directory,"base",entry.base))){remaining.push(problem);continue;}
  const result=merge(readFileSync(join(directory,"base",entry.base)),bytes.get(entry.path)!,readFileSync(resolve(root,entry.path)));
  if(result===undefined){unavailable=true;remaining.push(problem);continue;}
  if(result.conflicts>0){remaining.push(problem);continue;}
  const after=landHashV1(result.merged);
  entries.set(entry.path,{...entry,base:problem.live,after});bytes.set(entry.path,result.merged);merged.push(entry.path);
 }
 return {changeset:Object.freeze({stage:changeset.stage,entries:Object.freeze([...entries.values()]),bytes}),merged,remaining,unavailable};
}

/** 🧮️ The order the files are renamed in: new files first (nothing imports them yet), then rewritten files with every imported file before its importer, deletions last (their importers were rewritten before). */
export function landApplyOrderV1(changeset:LandChangesetV1):readonly LandEntryV1[]{
 const decoder=new TextDecoder(),writes=changeset.entries.filter(entry=>entry.op==="write"),deletes=changeset.entries.filter(entry=>entry.op==="delete");
 const sorted=(group:readonly LandEntryV1[]):LandEntryV1[]=>{
  const byKey=new Map(group.map(entry=>[keyOf(entry.path),entry])),done=new Set<string>(),out:LandEntryV1[]=[];
  const visit=(entry:LandEntryV1,trail:Set<string>):void=>{
   if(done.has(entry.path)||trail.has(entry.path))return;
   trail.add(entry.path);
   if(/\.(?:d\.mts|mts|mjs|tsx|ts|js|jsx|cjs|cts)$/u.test(entry.path))for(const key of importKeysV1(entry.path,decoder.decode(changeset.bytes.get(entry.path)!))){const dependency=byKey.get(key);if(dependency)visit(dependency,trail);}
   trail.delete(entry.path);done.add(entry.path);out.push(entry);
  };
  for(const entry of [...group].sort((left,right)=>left.path<right.path?-1:1))visit(entry,new Set());
  return out;
 };
 return Object.freeze([...sorted(writes.filter(entry=>entry.base===null)),...sorted(writes.filter(entry=>entry.base!==null)),...deletes.sort((left,right)=>left.path<right.path?-1:1)]);
}

/** 🔢️ The generation counter of an area: odd while a landing writes its files, even otherwise; a reader that needs one consistent view reads it before and after and retries unless both reads are equal and even. */
export function landGenerationV1(lockRoot:string,area:string,limits:LandPolicyV1=rules):number{
 try{return Number(readFileSync(join(lockRoot,limits.locks.generationPrefix+area),"utf8").trim())||0;}catch{return 0;}
}
const bumpGeneration=(lockRoot:string,area:string,limits:LandPolicyV1):void=>writeAtomic(join(lockRoot,limits.locks.generationPrefix+area),String(landGenerationV1(lockRoot,area,limits)+1));

/** 🔌️ The seams of the apply; tests replace them to observe the files between two renames. */
export type LandApplyPortsV1=Readonly<{lockRoot:string;before?:(step:Readonly<{phase:"temp"|"rename"|"delete";path:string;index:number}>)=>void;rename?:(from:string,to:string)=>void}>;

/** ⚛️ Applies a changeset in two phases: every new content is written and synced to a hidden temporary file beside its target first, then the files are renamed in the safe order while the area generations are odd; deletions come last. A failure restores every file already replaced. */
export function landApplyV1(root:string,changeset:LandChangesetV1,ports:LandApplyPortsV1,limits:LandPolicyV1=rules):void{
 const order=landApplyOrderV1(changeset),areas=[...new Set(order.map(entry=>entry.area))].sort(),temporaries=new Map<string,string>(),previous=new Map<string,Uint8Array|null>(),renamed:string[]=[],rename=ports.rename??renameSync;
 const target=(path:string):string=>resolve(root,path);
 try{
  order.forEach((entry,index)=>{
   if(entry.op!=="write")return;
   ports.before?.({phase:"temp",path:entry.path,index});
   const temporary=join(dirname(target(entry.path)),".land-"+process.pid+"-"+index+".tmp");
   mkdirSync(dirname(temporary),{recursive:true});
   const handle=openSync(temporary,"w");
   try{writeSync(handle,changeset.bytes.get(entry.path)!);fsyncSync(handle);}finally{closeSync(handle);}
   temporaries.set(entry.path,temporary);
  });
 }catch(error){for(const temporary of temporaries.values())rmSync(temporary,{force:true});throw refuseLandV1("failed","apply.failed",{path:"",reason:String((error as Error).message).slice(0,limits.limits.diagnosticCharacters)});}
 for(const area of areas)bumpGeneration(ports.lockRoot,area,limits);
 let failing="";
 try{
  order.forEach((entry,index)=>{
   failing=entry.path;
   previous.set(entry.path,existsSync(target(entry.path))?readFileSync(target(entry.path)):null);
   ports.before?.({phase:entry.op==="write"?"rename":"delete",path:entry.path,index});
   if(entry.op==="write")rename(temporaries.get(entry.path)!,target(entry.path));else rmSync(target(entry.path),{force:true});
   renamed.push(entry.path);
  });
 }catch(error){
  for(const path of [...renamed].reverse()){const old=previous.get(path)!;if(old===null)rmSync(target(path),{force:true});else writeAtomic(target(path),old);}
  for(const temporary of temporaries.values())rmSync(temporary,{force:true});
  for(const area of areas)bumpGeneration(ports.lockRoot,area,limits);
  throw refuseLandV1("failed","apply.failed",{path:failing,reason:String((error as Error).message).slice(0,limits.limits.diagnosticCharacters)});
 }
 for(const area of areas)bumpGeneration(ports.lockRoot,area,limits);
}

/** 📍️ Records the rollback point of a changeset before it is applied: the manifest and the previous bytes of every path that exists. */
export function landRecordRollbackV1(root:string,changeset:LandChangesetV1,cacheDirectory:string,atMs:number):string{
 const id=changeset.stage.id+"-"+new Date(atMs).toISOString().replace(/[-:]/gu,"").replace(/\..*/u,"")+"-"+createHash("sha256").update(changeset.entries.map(entry=>entry.path+entry.after).join("|")).digest("hex").slice(0,6),directory=join(cacheDirectory,"rollback",id);
 for(const entry of changeset.entries)if(existsSync(resolve(root,entry.path)))writeAtomic(join(directory,"before",entry.path),readFileSync(resolve(root,entry.path)));
 const point:LandRollbackPointV1={version:1,id,lane:changeset.stage.lane,title:changeset.stage.title,atMs,entries:changeset.entries};
 writeAtomic(join(directory,"rollback.json"),JSON.stringify(point,null,1));
 return id;
}

/** ↩️ Builds the changeset that undoes a landing; it refuses when a landed file changed afterwards. */
export function landRollbackChangesetV1(root:string,cacheDirectory:string,id:string,limits:LandPolicyV1=rules):LandChangesetV1{
 const directory=join(cacheDirectory,"rollback",id),file=join(directory,"rollback.json");
 if(!/^[A-Za-z0-9._-]+$/u.test(id)||!existsSync(file))throw refuseLandV1("missing","rollback.unknown",{id});
 const point=readJson(file) as LandRollbackPointV1,problems=validateLandDocumentV1("RollbackPoint",point);
 if(problems.length)throw refuseLandV1("invalid","stage.invalid",{directory,details:problems.slice(0,3).join("; ")});
 const entries:LandEntryV1[]=[],bytes=new Map<string,Uint8Array>();
 for(const entry of point.entries){
  if(liveHash(root,entry.path)!==entry.after)throw refuseLandV1("stale","rollback.diverged",{path:entry.path});
  const before=join(directory,"before",entry.path);
  if(existsSync(before)){const content=readFileSync(before);entries.push({path:entry.path,op:"write",base:entry.after,after:landHashV1(content),area:entry.area});bytes.set(entry.path,content);}
  else entries.push({path:entry.path,op:"delete",base:entry.after,after:null,area:entry.area});
 }
 return Object.freeze({stage:{version:1,id:"rollback-"+id.slice(0,50),lane:point.lane,title:"Rollback of "+point.title,bases:{},deletes:[]},entries:Object.freeze(entries),bytes});
}

/** 🔐️ The held landing locks; release them right after the atomic write. */
export type LandLocksV1=Readonly<{areas:readonly string[];release:()=>void;heldMilliseconds:()=>number}>;

/** 🔒️ Takes `landing.lock.<area>` for every area in alphabetical order (no deadlock) with an owner file; a held lock is waited for up to the policy time and queued in `landing.queue.<area>`, a lock older than the stale age is reported and never stolen. */
export async function landLockV1(lockRoot:string,areas:readonly string[],owner:string,ports:Readonly<{now?:()=>number;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;signal?:AbortSignal;notice?:(key:"lock.waiting",values:Record<string,string|number>)=>void}>={},limits:LandPolicyV1=rules):Promise<LandLocksV1>{
 const now=ports.now??Date.now,sleep=ports.sleep??((milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds))),taken:string[]=[],directory=(area:string)=>join(lockRoot,limits.locks.directoryPrefix+area),release=():void=>{for(const area of taken.splice(0).reverse())rmSync(directory(area),{recursive:true,force:true});};
 let took=0;
 try{
  for(const area of [...new Set(areas)].sort()){
   const begin=now();let queued=false;
   for(;;){
    if(ports.signal?.aborted)throw refuseLandV1("cancelled","cancelled");
    try{mkdirSync(directory(area));writeFileSync(join(directory(area),limits.locks.ownerFile),owner+" "+new Date(now()).toTimeString().slice(0,8)+"\n");taken.push(area);break;}
    catch(error){
     if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;
     let holder="?",age=0;
     try{holder=readFileSync(join(directory(area),limits.locks.ownerFile),"utf8").trim();age=now()-statSync(directory(area)).mtimeMs;}catch{}
     if(age>limits.locks.staleMinutes*60000)throw refuseLandV1("stale","lock.stale",{area,owner:holder,minutes:limits.locks.staleMinutes});
     if(!queued){appendFileSync(join(lockRoot,limits.locks.queuePrefix+area),owner+" "+new Date(now()).toTimeString().slice(0,8)+" waiting for "+holder+"\n");queued=true;}
     if(now()-begin>=limits.locks.waitMilliseconds)throw refuseLandV1("refused","lock.busy",{area,owner:holder});
     ports.notice?.("lock.waiting",{area,owner:holder,seconds:Math.round((now()-begin)/1000)});
     await sleep(limits.locks.pollMilliseconds,ports.signal);
    }
   }
  }
  took=now();
 }catch(error){release();throw error;}
 return {areas:Object.freeze([...taken]),release,heldMilliseconds:()=>now()-took};
}

/** 🗒️ Appends the landing entry to the landings file (append-only, never a rewrite). */
export function landAppendLandingV1(root:string,wave:string,changeset:LandChangesetV1,summary:Readonly<{rollback:string;areas:readonly string[];gates:readonly string[];heldMilliseconds:number;atMs:number}>,limits:LandPolicyV1=rules):string{
 const file=resolve(root,limits.locks.root,limits.landings.file),stamp=new Date(summary.atMs),time=String(stamp.getHours()).padStart(2,"0")+":"+String(stamp.getMinutes()).padStart(2,"0"),paths=changeset.entries.map(entry=>(entry.op==="delete"?"-":entry.base===null?"+":"~")+entry.path.split("/").slice(-3).join("/"));
 const text=`\n## ${wave} ${changeset.stage.lane}: ${changeset.stage.title||changeset.stage.id}\n- ${time} local, landed by \`repo land\` (id ${changeset.stage.id}); locks ${summary.areas.join(", ")} held ${summary.heldMilliseconds} ms for the atomic apply. Files (${paths.length}; + new, ~ changed, - deleted): ${paths.join(", ")}. Gates: ${summary.gates.join(", ")}. Rollback point \`${summary.rollback}\` (\`repo land rollback ${summary.rollback}\`).\n`;
 appendFileSync(file,text);
 return file;
}

/** 🪞️ The live hash of one path, for tests and gates (`null` when the file is missing). */
export const landLiveHashV1=liveHash;
