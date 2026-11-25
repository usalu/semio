import {expect,test} from "bun:test";
import {mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,relative,resolve} from "node:path";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import law from "./🔣️.json" with {type:"json"};

const MODULE="(?:node:)?child_process",quoted=`["']${MODULE}["']`,python=testPythonV1(),oracle=join(import.meta.dirname,"🐍️oracle.py"),root=resolve(import.meta.dirname,"../../../../../../../..");

/** 🪟️ Closes the bracket opened at `at`, skipping strings, template substitutions and comments; -1 when it never closes. */
function close(text:string,at:number):number{
 let depth=0,quote="";
 for(let index=at;index<text.length;index++){
  const char=text[index]!;
  if(quote!==""){if(char==="\\")index++;else if(quote==="`"&&char==="$"&&text[index+1]==="{")index=close(text,index+1);else if(char===quote)quote="";continue;}
  if(char==="\""||char==="'"||char==="`")quote=char;
  else if(char==="/"&&text[index+1]==="/"){index=text.indexOf("\n",index);if(index<0)return -1;}
  else if(char==="/"&&text[index+1]==="*")index=text.indexOf("*/",index)+1;
  else if("([{".includes(char))depth++;
  else if(")]}".includes(char)&&--depth===0)return index;
 }
 return -1;
}

/** 🪟️ The top-level arguments of the call whose brackets are `start`..`end`. */
function argumentsOf(text:string,start:number,end:number):string[]{
 const out:string[]=[];let depth=0,quote="",last=start+1;
 for(let index=start+1;index<end;index++){
  const char=text[index]!;
  if(quote!==""){if(char==="\\")index++;else if(quote==="`"&&char==="$"&&text[index+1]==="{")index=close(text,index+1);else if(char===quote)quote="";continue;}
  if(char==="\""||char==="'"||char==="`")quote=char;
  else if("([{".includes(char))depth++;
  else if(")]}".includes(char))depth--;
  else if(char===","&&depth===0){out.push(text.slice(last,index));last=index+1;}
 }
 if(text.slice(last,end).trim()!=="")out.push(text.slice(last,end));
 return out;
}

/** 🪟️ The 1-based lines of every child-process call of a source text that can open a console window on Windows: a `node:child_process` call or `Bun.spawn*` without `windowsHide`, whose program is not a literal POSIX-only tool. See https://nodejs.org/api/child_process.html#child_processspawncommand-args-options */
export function hiddenSpawnViolationsV1(text:string):number[]{
 const names=new Set<string>(),spaces=new Set<string>();
 for(const match of text.matchAll(new RegExp(`import\\s*(?:type\\s*)?\\{([^}]*)\\}\\s*from\\s*${quoted}`,"gu")))for(const part of match[1]!.split(",").map(item=>item.trim()))if(part!==""&&!part.startsWith("type "))names.add(part.split(/\s+as\s+/u).at(-1)!.trim());
 for(const match of text.matchAll(new RegExp(`import\\s+\\*\\s+as\\s+(\\w+)\\s+from\\s*${quoted}`,"gu")))spaces.add(match[1]!);
 for(const match of text.matchAll(new RegExp(`import\\s+(\\w+)\\s+from\\s*${quoted}`,"gu")))spaces.add(match[1]!);
 for(const match of text.matchAll(new RegExp(`(?:const|let|var)\\s*\\{([^}]*)\\}\\s*=\\s*(?:await\\s+import|require)\\(\\s*${quoted}\\s*\\)`,"gu")))for(const part of match[1]!.split(",").map(item=>item.trim()))if(part!=="")names.add(part.split(/\s*:\s*/u).at(-1)!.trim());
 for(const match of text.matchAll(new RegExp(`(?:const|let|var)\\s+(\\w+)\\s*=\\s*(?:await\\s+import|require)\\(\\s*${quoted}\\s*\\)`,"gu")))spaces.add(match[1]!);
 const escape=(word:string):string=>word.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&"),functions=law.functions.join("|"),patterns:string[]=[];
 if(names.size>0)patterns.push(`(?<![\\w.$])(?:${[...names].map(escape).join("|")})\\s*(?:<[^()]*>)?\\s*\\(`);
 if(spaces.size>0)patterns.push(`(?<![\\w$])(?:${[...spaces].map(escape).join("|")})\\.(?:${functions})\\s*\\(`);
 patterns.push("(?<![\\w$])Bun\\.(?:spawn|spawnSync)\\s*\\(",`import\\(\\s*${quoted}\\s*\\)\\s*\\)?\\.(?:${functions})\\s*\\(`);
 const seen=new Set<number>(),lines=new Set<number>();
 for(const pattern of patterns)for(const match of text.matchAll(new RegExp(pattern,"gu"))){
  const open=match.index+match[0].length-1;
  if(seen.has(open))continue;seen.add(open);
  if(/^\s*(\/\/|\*|\/\*)/u.test(text.slice(text.lastIndexOf("\n",match.index-1)+1,match.index)))continue;
  const end=close(text,open);if(end<0)continue;
  const args=argumentsOf(text,open,end);
  if(args.length>0&&/^\s*\w+\??\s*:\s*[A-Za-z]/u.test(args[0]!))continue;
  if(/(?<![\w"'`])windowsHide\s*(?::\s*(?!false\b)|[,}])/u.test(text.slice(open,end+1)))continue;
  const literal=args.length>0?/^\s*(?:"([^"]*)"|'([^']*)')\s*$/u.exec(args[0]!):null,program=literal===null?undefined:literal[1]??literal[2];
  if(program!==undefined&&(program.startsWith("/")||law.posixOnlyPrograms.includes(program)))continue;
  lines.add(text.slice(0,match.index).split("\n").length);
 }
 return [...lines].sort((left,right)=>left-right);
}

const sources=(directory:string):string[]=>readdirSync(directory,{withFileTypes:true}).sort((left,right)=>left.name<right.name?-1:1).flatMap(entry=>entry.isDirectory()?law.skipDirectories.includes(entry.name)?[]:sources(join(directory,entry.name)):law.extensions.some(extension=>entry.name.endsWith(extension))?[join(directory,entry.name)]:[]);
const scanTypescript=():string[]=>law.roots.flatMap(directory=>sources(join(root,directory)).flatMap(file=>hiddenSpawnViolationsV1(readFileSync(file,"utf8")).map(line=>`${relative(root,file).split("\\").join("/")}:${line}`))).sort();

test("every case of the law is judged as written, by this scanner and by the Python oracle",()=>{
 const directory=mkdtempSync(join(tmpdir(),"hidden-spawn-"));
 try{
  for(const item of law.cases){
   expect(hiddenSpawnViolationsV1(item.text),item.name).toEqual(item.lines);
   if(python){
    const file=join(directory,"case.ts");writeFileSync(file,item.text);
    const run=Bun.spawnSync([python,"-I",oracle,"text",file],{stdout:"pipe",stderr:"pipe",windowsHide:true});
    expect(run.exitCode,run.stderr.toString()).toBe(0);
    expect(JSON.parse(run.stdout.toString()),`${item.name} (oracle)`).toEqual(item.lines);
   }
  }
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("no fleet, replica, supervisor, readiness, build, caching or process code starts a child process that can open a console window on Windows",()=>{
 const found=scanTypescript();
 expect(found,"add windowsHide:true to these child-process calls (a detached or windowless parent gives every console child its own visible window on Windows)").toEqual([]);
 if(python){
  const run=Bun.spawnSync([python,"-I",oracle,"scan",root],{stdout:"pipe",stderr:"pipe",windowsHide:true,maxBuffer:64*1024*1024});
  expect(run.exitCode,run.stderr.toString()).toBe(0);
  expect(JSON.parse(run.stdout.toString())).toEqual(found);
 }
});

test("the roots of the law exist and hold source files",()=>{
 for(const directory of law.roots)expect(sources(join(root,directory)).length,directory).toBeGreaterThan(0);
});
