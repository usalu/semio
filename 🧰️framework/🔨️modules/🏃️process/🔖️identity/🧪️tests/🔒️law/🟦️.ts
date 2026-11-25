import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,readFileSync,readdirSync} from "node:fs";
import {join,relative,resolve,sep} from "node:path";
import baseline from "./🔣️.json" with {type:"json"};

/** 🔒️ Law: no production file judges a process by `process.kill(<pid>, 0)` outside the identity module; the files in the baseline are the sites not converted yet and the baseline only shrinks (a file may not gain a check, a converted file must leave the list).
 * @see ../../🟦️.ts */
const root=resolve(import.meta.dir,"../../../../../..");
const pattern=/process\.kill\([^)]*,\s*0\s*\)/gu;
const identity="🧰️framework/🔨️modules/🏃️process/🔖️identity/";
const excluded=(path:string):boolean=>path.includes("🧪️tests")||path.includes("🎫️tickets")||path.startsWith(identity)||!/\.(ts|mjs|js)$/u.test(path);

function viaGit():Map<string,number>|undefined{
 const run=spawnSync("git",["grep","-cE","process\\.kill\\([^)]*, *0 *\\)","--","*.ts","*.mjs","*.js"],{cwd:root,encoding:"utf8",maxBuffer:1<<26});
 if(run.error!==undefined||(run.status!==0&&run.status!==1))return undefined;
 const found=new Map<string,number>();
 for(const line of run.stdout.split("\n")){const at=line.lastIndexOf(":");if(at<1)continue;const path=line.slice(0,at);if(!excluded(path))found.set(path,Number(line.slice(at+1)));}
 return found;
}

function viaWalk():Map<string,number>{
 const found=new Map<string,number>(),skip=new Set(["node_modules","target",".git",".cache","🎫️tickets"]);
 const walk=(directory:string):void=>{
  for(const entry of readdirSync(directory,{withFileTypes:true})){
   if(skip.has(entry.name))continue;
   const path=join(directory,entry.name);
   if(entry.isDirectory()){walk(path);continue;}
   const rel=relative(root,path).split(sep).join("/");
   if(excluded(rel))continue;
   const count=(readFileSync(path,"utf8").match(pattern)??[]).length;
   if(count>0)found.set(rel,count);
  }
 };
 walk(join(root,"🧰️framework"));
 return found;
}

const sites=():Map<string,number>=>viaGit()??viaWalk();

test("the baseline is closed: only files, each with a count and a reason to convert",()=>{
 for(const [file,row] of Object.entries(baseline.sites)){
  expect(excluded(file),file).toBe(false);
  expect(existsSync(join(root,file)),`${file} no longer exists: remove it from the baseline`).toBe(true);
  expect(Number.isInteger(row.count)&&row.count>=1,file).toBe(true);
 }
});

test("no production file gains a pid-only liveness check, and a converted file leaves the baseline",()=>{
 const found=sites(),gained:string[]=[],converted:string[]=[];
 for(const [file,count] of found){const allowed=(baseline.sites as Record<string,{count:number}>)[file]?.count??0;if(count>allowed)gained.push(`${file}: ${count} > ${allowed}`);}
 for(const [file,row] of Object.entries(baseline.sites)){const count=found.get(file)??0;if(count<row.count)converted.push(`${file}: ${count} < ${row.count}`);}
 expect(gained,"a pid-only `process.kill(pid, 0)` judges a holder; name it by {pid, startMs} and judge it with processHolderAliveV1 of the identity module").toEqual([]);
 expect(converted,"lower the count (or remove the entry) of the converted file in the baseline").toEqual([]);
});

test("the identity module holds exactly the one raw table check",()=>{
 const text=readFileSync(join(root,identity,"🟦️.ts"),"utf8");
 expect((text.match(pattern)??[]).length).toBe(1);
});

test("an independent Python scan counts the same sites in every baseline file",()=>{
 const files=Object.keys(baseline.sites);
 const run=spawnSync("python3",["-I","-c","import json,re,sys\nroot=sys.argv[1]\nout={}\nfor f in json.loads(sys.argv[2]):\n    out[f]=len(re.findall(r'process\\.kill\\([^)]*,\\s*0\\s*\\)',open(root+'/'+f,encoding='utf8').read()))\nprint(json.dumps(out))",root,JSON.stringify(files)],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const counted=JSON.parse(run.stdout) as Record<string,number>;
 for(const [file,row] of Object.entries(baseline.sites))expect(counted[file],file).toBe(row.count);
});
