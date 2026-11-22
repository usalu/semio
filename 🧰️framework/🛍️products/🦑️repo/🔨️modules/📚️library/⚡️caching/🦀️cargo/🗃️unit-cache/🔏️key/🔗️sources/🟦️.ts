import {existsSync,readFileSync,readdirSync} from "node:fs";
import {dirname,join,relative,resolve,sep} from "node:path";
import {parseDepInfoV1,sha256V1,UNIT_CACHE_POLICY_V1} from "../🟦️.ts";

/** 🏷️ The version of the source part of a unit key: the digest of the entry file and of every file the crate reads, so a change of any module, include or build output moves the key. A new version re-keys every unit once. */
export const SOURCE_KEY_VERSION_V1="sources-v2";

/** 🔗️ Everything one crate reads that is not the command line: the files (the entry first), a row per file besides the entry (its path below the entry's directory, forward slashes, and its digest), the names that `env!` and `option_env!` in those files ask for, and the digest that stands for all of it in the unit key. */
export type UnitSourcesV1=Readonly<{files:readonly string[];rows:readonly (readonly [string,string])[];names:readonly string[];sha256:string}>;

const ENVIRONMENT_MACRO=/\b(?:option_)?env!\s*\(\s*"([A-Za-z_][A-Za-z0-9_]{0,255})"/gu,OUTPUT_FILES=256;

/** 🔑️ The source digest of a unit key from the digest of its entry file and the rows of the other files it reads. */
export const sourceKeyV1=(entry:string,rows:readonly (readonly [string,string])[]):string=>sha256V1(JSON.stringify([SOURCE_KEY_VERSION_V1,entry,rows]));

const walk=(directory:string,found:string[]):void=>{
 let names:string[];try{names=readdirSync(directory).sort();}catch{return;}
 for(const name of names){
  if(found.length>=OUTPUT_FILES)return;
  const path=join(directory,name);
  try{if(readdirSync(path).length>=0){walk(path,found);continue;}}catch{}
  found.push(path);
 }
};

/** 🔎️ What the compiler will read for the unit whose root source is `entry`, found before compiling: the files of `mod name;`, of `#[path]` and of the literal `include!`, `include_str!` and `include_bytes!` (the scan of the offload planner, `reachedFilesV1`), the files of the build script output directory (`OUT_DIR`, bounded) and the environment names of `env!` and `option_env!`. `digest` is the digest of a file (null when unreadable); null when the entry cannot be read. */
export async function unitSourcesV1(entry:string,manifest:string|undefined,outDirectory:string|undefined,digest:(path:string)=>string|null):Promise<UnitSourcesV1|null>{
 const first=digest(entry);if(first===null)return null;
 let reached:readonly string[]=[];
 try{reached=(await import("../../../../../🏗️build/🌐️remote-execution/🚀️offload/🗺️paths/🟦️.ts")).reachedFilesV1(entry,undefined,undefined,manifest)??[];}catch{}
 const generated:string[]=[];if(outDirectory!==undefined&&existsSync(outDirectory))walk(outDirectory,generated);
 const rows:(readonly [string,string])[]=[],home=dirname(entry);
 for(const file of reached)rows.push([relative(home,file).split(sep).join("/"),digest(file)??"unreadable"]);
 for(const file of generated)rows.push(["⟨out⟩/"+relative(outDirectory!,file).split(sep).join("/"),digest(file)??"unreadable"]);
 rows.sort((a,b)=>a[0]<b[0]?-1:a[0]>b[0]?1:0);
 const names=new Set<string>();
 for(const file of [entry,...reached])if(file.endsWith(".rs")){let text:string;try{text=readFileSync(file,"utf8");}catch{continue;}for(const match of text.matchAll(ENVIRONMENT_MACRO))names.add(match[1]!);}
 return Object.freeze({files:Object.freeze([entry,...reached,...generated]),rows:Object.freeze(rows),names:Object.freeze([...names].sort()),sha256:sourceKeyV1(first,rows)});
}

/** 🌱️ The environment rows of the names a crate asks for with `env!` or `option_env!` that the key does not select already (a name that is not set is a row of its own, so setting it moves the key): `value` turns a set value into its key form. */
export function sourceEnvironmentV1(names:readonly string[],environment:Readonly<Record<string,string|undefined>>,selected:ReadonlySet<string>,value:(name:string,text:string)=>string):Array<[string,string]>{
 return names.filter(name=>!selected.has(name)).map(name=>[name,environment[name]===undefined?"⟨unset⟩":value(name,environment[name]!)] as [string,string]);
}

/** 🕳️ What the dependency file of a finished compilation lists that the key did not cover: a file the scan missed (a path of the dependency file that is none of `covered`, resolved against `cwd`) or an environment name the key neither selects nor scanned (`env:NAME`). A unit whose key has a gap is never uploaded to the fleet. */
export function keyGapV1(input:Readonly<{cwd:string;covered:ReadonlySet<string>;keyedNames:ReadonlySet<string>;depInfo:string}>):string[]{
 const parsed=parseDepInfoV1(input.depInfo),gap:string[]=[],selected=new Set<string>(UNIT_CACHE_POLICY_V1.key.environmentNames);
 for(const path of parsed.dependencies)if(!input.covered.has(resolve(input.cwd,path)))gap.push(path);
 for(const row of parsed.environment)if(!input.keyedNames.has(row.name)&&!selected.has(row.name)&&!UNIT_CACHE_POLICY_V1.key.environmentPrefixes.some(prefix=>row.name.startsWith(prefix)))gap.push("env:"+row.name);
 return gap;
}
