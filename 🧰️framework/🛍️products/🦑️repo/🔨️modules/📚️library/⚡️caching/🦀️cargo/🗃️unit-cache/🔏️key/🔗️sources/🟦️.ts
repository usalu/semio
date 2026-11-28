import {existsSync,readFileSync,readdirSync} from "node:fs";
import {dirname,join,relative,resolve,sep} from "node:path";
import {parseDepInfoV1,sha256V1,UNIT_CACHE_POLICY_V1} from "../🟦️.ts";

/** 🏷️ The version of the source part of a unit key: the digest of the entry file and of every file the crate reads, so a change of any module, include or build output moves the key. A new version re-keys every unit once. */
export const SOURCE_KEY_VERSION_V1="sources-v2";

/** 🕳️ One file of a slot's input manifest, as this machine keys it: its portable name (roots as tokens) and its path on this machine. */
export type RecordedGapV1=Readonly<{name:string;path:string}>;

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

/** 🔎️ What the compiler will read for the unit whose root source is `entry`, found before compiling: the files of `mod name;`, of `#[path]` and of the literal `include!` and `include_<name>!` macros (the scan of the offload planner, `reachedFilesV1`), the files of the build script output directory (`OUT_DIR`, bounded) and the environment names of `env!` and `option_env!`. `digest` is the digest of a file (null when unreadable); `recorded` and `extraNames` are the files and environment names of the slot's input manifest (see `obtainManifestV1`), keyed like the scanned ones; null when the entry cannot be read. */
export async function unitSourcesV1(entry:string,manifest:string|undefined,outDirectory:string|undefined,digest:(path:string)=>string|null,recorded:readonly RecordedGapV1[]=[],extraNames:readonly string[]=[]):Promise<UnitSourcesV1|null>{
 const first=digest(entry);if(first===null)return null;
 let reached:readonly string[]=[];
 try{reached=(await import("../../../../../🏗️build/🌐️remote-execution/🚀️offload/🗺️paths/🟦️.ts")).reachedFilesV1(entry,undefined,undefined,manifest)??[];}catch{}
 const generated:string[]=[];if(outDirectory!==undefined&&existsSync(outDirectory))walk(outDirectory,generated);
 const rows:(readonly [string,string])[]=[],home=dirname(entry);
 for(const file of reached)rows.push([relative(home,file).split(sep).join("/"),digest(file)??"unreadable"]);
 for(const file of generated)rows.push(["⟨out⟩/"+relative(outDirectory!,file).split(sep).join("/"),digest(file)??"unreadable"]);
 for(const row of recorded)rows.push(["⟨gap⟩/"+row.name,digest(row.path)??"unreadable"]);
 rows.sort((a,b)=>a[0]<b[0]?-1:a[0]>b[0]?1:0);
 const names=new Set<string>(extraNames);
 for(const file of [entry,...reached])if(file.endsWith(".rs")){let text:string;try{text=readFileSync(file,"utf8");}catch{continue;}for(const match of text.matchAll(ENVIRONMENT_MACRO))names.add(match[1]!);}
 return Object.freeze({files:Object.freeze([entry,...reached,...generated,...recorded.map(row=>row.path)]),rows:Object.freeze(rows),names:Object.freeze([...names].sort()),sha256:sourceKeyV1(first,rows)});
}

/** 🌱️ The environment rows of the names a crate asks for with `env!` or `option_env!` that the key does not select already (a name that is not set is a row of its own, so setting it moves the key): `value` turns a set value into its key form. */
export function sourceEnvironmentV1(names:readonly string[],environment:Readonly<Record<string,string|undefined>>,selected:ReadonlySet<string>,value:(name:string,text:string)=>string):Array<[string,string]>{
 return names.filter(name=>!selected.has(name)).map(name=>[name,environment[name]===undefined?"⟨unset⟩":value(name,environment[name]!)] as [string,string]);
}

const WRITTEN=UNIT_CACHE_POLICY_V1.key.compilerWrittenFiles.map(pattern=>new RegExp(pattern,"u"));

/** 🖊️ Whether a path of the dependency file is written by the compilation itself, so it is no input of it: the observation file a proc-macro writes under `semio-compiler-resources/<ms>-<pid>-<n>/` for the build receipt (`with_compiler_resources_v1` registers it with the compiler, so it appears in the dependency file, but nothing the compiler produces is a function of it). */
export const compilerWrittenV1=(path:string):boolean=>WRITTEN.some(pattern=>pattern.test(path.replaceAll("\\","/")));

/** 🧼️ The dependency file of a compilation without what the compilation itself wrote: the observation file of a proc-macro (a target-line entry, its `path:` line and its `# checksum:` line) lives under a directory named by the clock and the process id of that run, so two compilations of one unit, on one machine or two, listed other paths and other checksums and the stored `.d` of every unit with a proc-macro differed by origin. Everything else stays byte for byte; equal compilations give equal text. */
export function canonicalDepInfoV1(text:string):string{
 const written=(path:string):boolean=>compilerWrittenV1(path.replaceAll("\\ "," "));
 return text.split("\n").flatMap(line=>{
  if(line.startsWith("# checksum:")){const fields=line.split(" ");return fields.length>3&&written(fields.slice(3).join(" "))?[]:[line];}
  if(line.startsWith("#"))return[line];
  if(line.endsWith(":")&&!line.includes(": "))return written(line.slice(0,-1))?[]:[line];
  const cut=line.indexOf(": ");if(cut<0)return[line];
  return[line.slice(0,cut+2)+line.slice(cut+2).split(/(?<!\\) +/u).filter(path=>path===""||!written(path)).join(" ")];
 }).join("\n");
}

/** 🌫️ A captured compiler stream (`stderr`, `stdout`) without the number rustc draws for the file that holds a type name too long to print (`long-type-<digits>.txt`, another number in every compilation of one unit): two compilations of one unit, on one machine or two, then give the same stream, and the stored result of a unit whose warning names such a file is no longer refused as diverged. Everything else stays byte for byte; equal compilations give equal text. */
export const canonicalStreamV1=(text:string):string=>text.replace(/long-type-[0-9]+\.txt/gu,"long-type-0.txt");

/** 🏷️ The name under which the captured streams of a unit are converted for the fleet (see {@link canonicalOutputV1}); no output file has it. */
export const STREAM_NAME_V1="⟨stream⟩";

/** 🔀️ The text of one stored output of a unit as the fleet keeps it: a dependency file without what the compilation itself wrote ({@link canonicalDepInfoV1}), a captured stream without compiler noise ({@link canonicalStreamV1}), anything else unchanged. */
export function canonicalOutputV1(name:string,text:string):string{return name.endsWith(".d")?canonicalDepInfoV1(text):name===STREAM_NAME_V1?canonicalStreamV1(text):text;}

/** 📦️ Whether a directory is the unpacked source of one version of a registry crate (`…/registry/src/<index>/<name>-<version>`): its content is fixed by the name and version that the lock file pins and that the compiler arguments already carry, so every file below it is covered by the key without being scanned. */
export const registryCrateDirectoryV1=(directory:string):boolean=>/(^|\/)registry\/src\/[^/]+\/[^/]+$/u.test(directory.replaceAll("\\","/").replace(/\/+$/u,""));

/** 🕳️ What the dependency file of a finished compilation lists that the key did not cover: a file the scan missed (a path of the dependency file that is none of `covered`, resolved against `cwd`) or an environment name the key neither selects nor scanned (`env:NAME`). What the finished compilation lists beyond the scan becomes the slot's input manifest (see `observedManifestV1`), the unit is keyed on the complete set and published under that key: no compiled unit is discarded for a gap; only a dependency file that is missing leaves the unit unshared. */
export function keyGapV1(input:Readonly<{cwd:string;covered:ReadonlySet<string>;keyedNames:ReadonlySet<string>;depInfo:string;crateDirectory?:string}>):string[]{
 const parsed=parseDepInfoV1(input.depInfo),gap:string[]=[],selected=new Set<string>(UNIT_CACHE_POLICY_V1.key.environmentNames),ignored=new Set<string>(UNIT_CACHE_POLICY_V1.key.environmentIgnored),sealed=input.crateDirectory!==undefined&&registryCrateDirectoryV1(input.crateDirectory)?input.crateDirectory.replaceAll("\\","/").replace(/\/+$/u,"")+"/":undefined;
 for(const path of parsed.dependencies)if(!input.covered.has(resolve(input.cwd,path))&&!compilerWrittenV1(path)&&!(sealed!==undefined&&resolve(input.cwd,path).replaceAll("\\","/").startsWith(sealed)))gap.push(path);
 for(const row of parsed.environment)if(!input.keyedNames.has(row.name)&&!selected.has(row.name)&&!ignored.has(row.name)&&!UNIT_CACHE_POLICY_V1.key.environmentPrefixes.some(prefix=>row.name.startsWith(prefix)))gap.push("env:"+row.name);
 return gap;
}
