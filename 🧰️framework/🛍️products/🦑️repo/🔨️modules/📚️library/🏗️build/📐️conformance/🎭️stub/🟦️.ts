/** 🎭️ The task body every backend runs in the conformance workspace: reads declared files, writes declared outputs whose content is a pure function of them, and leaves start and end markers that no backend can fake. Self-contained: it imports only the runtime's built-ins. */
import {appendFileSync,mkdirSync,readdirSync,readFileSync,statSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";

/** 📋️ What the stub knows about one task. */
export type StubTaskV1={id:string;reads:string[];outputs:string[];dependencyOutputs:string[];mode:"emit"|"fail"|"sleep";milliseconds?:number;normalize?:boolean;report?:boolean};
/** 📒️ The spec file the stub reads: where to leave markers, and every task. */
export type StubSpecV1={markers:string;tasks:Record<string,StubTaskV1>};

const sha=(value:string)=>createHash("sha256").update(value).digest("hex");

const escaped=(character:string)=>character.replace(/[.+^$()|\\[\]]/gu,"\\$&");
const source=(pattern:string):string=>{
 let text="";
 for(let at=0;at<pattern.length;at++){
  const character=pattern[at]!;
  if(character==="*"){if(pattern[at+1]==="*"){at++;if(pattern[at+1]==="/"){at++;text+="(?:.*/)?";}else text+=".*";}else text+="[^/]*";}
  else if(character==="?")text+="[^/]";
  else if(character==="{"){const end=pattern.indexOf("}",at);if(end<0)throw Error("Unclosed { in "+pattern);text+="(?:"+pattern.slice(at+1,end).split(",").map(source).join("|")+")";at=end;}
  else text+=escaped(character);
 }
 return text;
};

/** 🌐️ The stub's own matcher for the glob syntax the scenarios use (`*`, `**`, `?`, `{a,b}`), so that the files a task reads are decided without any runtime's glob: the reference every backend's selection is compared with. */
export function stubGlobV1(pattern:string):RegExp{return new RegExp("^"+source(pattern)+"$","u");}

/** 📚️ The workspace files, relative and sorted, that a pattern selects: dot entries are skipped and symlinks are followed; the walk starts at the literal directory prefix of the pattern. */
export function stubSelectV1(pattern:string,root:string):string[]{
 const matcher=stubGlobV1(pattern),first=pattern.search(/[*?{]/u),prefix=first<0?"":pattern.slice(0,first),start=prefix.slice(0,prefix.lastIndexOf("/")+1),found:string[]=[];
 const walk=(directory:string):void=>{
  let names:import("node:fs").Dirent[];
  try{names=readdirSync(join(root,directory),{withFileTypes:true});}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT"||(error as NodeJS.ErrnoException).code==="ENOTDIR")return;throw error;}
  for(const entry of names){
   if(entry.name.startsWith("."))continue;
   const path=directory+entry.name,facts=entry.isSymbolicLink()?statSync(join(root,path),{throwIfNoEntry:false}):undefined;
   if(entry.isDirectory()||facts?.isDirectory())walk(path+"/");
   else if((entry.isFile()||facts?.isFile())&&matcher.test(path))found.push(path);
  }
 };
 if(first<0){try{if(statSync(join(root,pattern)).isFile())found.push(pattern);}catch{}}else walk(start);
 return found.sort();
}

/** 🧮️ The reference content of every output of a task, from the files it reads and the outputs it depends on. `files` lists the workspace files the reads select, sorted. */
export function stubOutputsV1(task:StubTaskV1,files:readonly string[],read:(path:string)=>string):Record<string,string>{
 const text=(path:string)=>{const body=read(path);return task.normalize?body.split("\n").filter(line=>!line.startsWith("#")).join("\n"):body;};
 const base=sha([task.id,...files.map(path=>path+"\0"+text(path)),...[...task.dependencyOutputs].sort().map(path=>path+"\0"+read(path))].join("\u0001"));
 return Object.fromEntries(task.outputs.map(path=>[path,sha(base+"\n"+path)+"\n"]));
}

/** 🏃️ Runs one task in the current directory. With an output root, outputs are written below it and dependency outputs are read from it, which is how a backend that keeps products outside the workspace runs it; a stamp path receives an empty file when the task has no outputs. */
export async function runStubV1(specPath:string,id:string,outputRoot?:string,stamp?:string):Promise<number>{
 const spec=JSON.parse(readFileSync(specPath,"utf8")) as StubSpecV1,task=spec.tasks[id];
 if(!task){process.stderr.write("Unknown conformance task "+id+"\n");return 2;}
 appendFileSync(spec.markers,`start\t${id}\t${Date.now()}\n`);
 if(task.mode==="sleep")await new Promise<void>(accept=>setTimeout(accept,task.milliseconds??1000));
 if(task.mode==="fail"){appendFileSync(spec.markers,`fail\t${id}\t${Date.now()}\n`);return 1;}
 const files:string[]=[];
 for(const pattern of task.reads)files.push(...stubSelectV1(pattern,process.cwd()));
 files.splice(0,files.length,...[...new Set(files)].sort());
 const dependencies=new Set(task.dependencyOutputs),read=(path:string)=>readFileSync(dependencies.has(path)&&outputRoot?join(outputRoot,path):path,"utf8");
 for(const[path,body]of Object.entries(stubOutputsV1(task,files,read))){const target=join(outputRoot??process.cwd(),path);mkdirSync(dirname(target),{recursive:true});writeFileSync(target,body);}
 if(stamp&&!task.outputs.length){mkdirSync(dirname(stamp),{recursive:true});writeFileSync(stamp,"");}
 appendFileSync(spec.markers,`end\t${id}\t${Date.now()}\n`);
 return 0;
}

if(process.argv[1]!==undefined&&process.argv[1]===fileURLToPath(import.meta.url))process.exitCode=await runStubV1(process.argv[2]!,process.argv[3]!,process.argv[4]||undefined,process.argv[5]||undefined);
