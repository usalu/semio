import {describe,expect,test} from "bun:test";
import {spawn,spawnSync,type ChildProcess,type SpawnSyncOptions} from "node:child_process";
import {existsSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {testPythonV1} from "../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {terminateOwnedChildTree,terminateOwnedProcessTree} from "../../🟦️.ts";

/** 🪟️ The Windows tree-kill branch of the termination funnel against a `taskkill` fixture (documented grammar and exit codes) on a REAL process tree: the whole tree ends, an already-gone process is not an error, a refused or missing `taskkill` still ends the root, and the argument vector equals the documented `/pid n /T /F`. The independent observers are `pgrep -P` (descendants before) and `kill -0` (survivors after). */
const python=testPythonV1()??Bun.which("python3")??"python3",fixture=join(import.meta.dir,"🐍️taskkill.py");
const SPAWNER=`const { spawn } = require("node:child_process");
const { appendFileSync } = require("node:fs");
const [depth, breadth, pidFile] = process.argv.slice(2);
appendFileSync(pidFile, process.pid + "\\n");
for (let index = 0; Number(depth) > 0 && index < Number(breadth); index++) spawn(process.execPath, [process.argv[1], String(Number(depth) - 1), breadth, pidFile], { stdio: "ignore" });
setInterval(() => {}, 1 << 30);
`;
const SIZE=1+2+4;

const alive=(pid:number):boolean=>{try{process.kill(pid,0);}catch{return false;}const state=spawnSync("ps",["-o","stat=","-p",String(pid)],{encoding:"utf8"});return state.status===0&&!state.stdout.trim().startsWith("Z");};
const descendants=(root:number):number[]=>{const found:number[]=[];for(const frontier=[root];frontier.length>0;){const children=spawnSync("pgrep",["-P",String(frontier.pop()!)],{encoding:"utf8"}).stdout.split("\n").filter(Boolean).map(Number);found.push(...children);frontier.push(...children);}return found.sort((a,b)=>a-b);};
async function until(predicate:()=>boolean,milliseconds:number):Promise<boolean>{const deadline=Date.now()+milliseconds;while(!predicate()){if(Date.now()>deadline)return false;await Bun.sleep(40);}return true;}
async function tree(root:string,id:string):Promise<{child:ChildProcess;pids:number[];exited:Promise<void>}>{
 const script=join(root,"spawner.cjs"),pidFile=join(root,id+".pids");writeFileSync(script,SPAWNER);writeFileSync(pidFile,"");
 const child=spawn(process.execPath,[script,"2","2",pidFile],{stdio:"ignore"}),exited=new Promise<void>(accept=>child.once("exit",()=>accept())),read=()=>readFileSync(pidFile,"utf8").split("\n").filter(Boolean).map(Number);
 expect(await until(()=>read().length===SIZE,30000)).toBe(true);return{child,pids:read(),exited};
}
const host=(log:string,environment:Record<string,string>={})=>({platform:"win32",spawnSync:((program:string,args:readonly string[],options:SpawnSyncOptions)=>{
 if(program!=="taskkill")throw Error("only taskkill is stood in for");return spawnSync(python,["-I",fixture,...args],{...options,env:{...process.env,SEMIO_TASKKILL_LOG:log,...environment}});
}) as typeof spawnSync});

describe("Windows tree termination against the taskkill fixture",()=>{
 const run=async(label:string,environment:Record<string,string>|null,body:(context:{child:ChildProcess;pids:number[];kids:number[];log:string;signals:unknown[][]})=>Promise<void>)=>{
  const root=mkdtempSync(join(tmpdir(),"semio-taskkill-")),log=join(root,"taskkill.log");let pids:number[]=[];
  try{
   const built=await tree(root,label);pids=built.pids;const kids=pids.filter(pid=>pid!==built.child.pid).sort((a,b)=>a-b);expect(descendants(built.child.pid!)).toEqual(kids);
   await body({child:built.child,pids,kids,log,signals:[]});
  }finally{for(const pid of pids)if(alive(pid))process.kill(pid,"SIGKILL");rmSync(root,{recursive:true,force:true});}
 };

 test("taskkill /T /F ends the root and all six descendants; the vector is the documented one",async()=>run("tree",{},async({child,kids,log})=>{
  terminateOwnedChildTree(child,host(log));
  expect(await until(()=>kids.every(pid=>!alive(pid))&&!alive(child.pid!),5000)).toBe(true);
  expect(readFileSync(log,"utf8").trim().split("\n").map(row=>JSON.parse(row))).toEqual([["/pid",String(child.pid),"/T","/F"]]);
 }),40000);

 test("a process that is already gone (exit 128) is not an error and is not signalled again",async()=>{
  const gone=spawn(process.execPath,["-e",""],{stdio:"ignore"});await new Promise<void>(accept=>gone.once("exit",()=>accept()));
  const root=mkdtempSync(join(tmpdir(),"semio-taskkill-")),log=join(root,"taskkill.log"),signals:unknown[][]=[],original=process.kill;
  process.kill=((...args:unknown[])=>{signals.push(args);return true;}) as typeof process.kill;
  try{terminateOwnedProcessTree(gone.pid!,host(log));}finally{process.kill=original;rmSync(root,{recursive:true,force:true});}
  expect(signals).toEqual([]);
 });

 for(const [label,status] of [["access denied",1],["invalid syntax",2]] as const)test(`${label} (exit ${status}) still ends the root: the descendants are a documented limit of the fallback`,async()=>run("refused-"+status,{SEMIO_TASKKILL_FAIL:String(status)},async({child,kids,log})=>{
  terminateOwnedChildTree(child,host(log,{SEMIO_TASKKILL_FAIL:String(status)}));
  expect(await until(()=>!alive(child.pid!),5000)).toBe(true);expect(kids.some(alive),"orphaned descendants stay until the owner sweeps them").toBe(true);
 }),40000);

 test("a missing taskkill (spawn throws) still ends the root",async()=>run("missing",null,async({child})=>{
  terminateOwnedChildTree(child,{platform:"win32",spawnSync:(()=>{throw Object.assign(new Error("spawn taskkill ENOENT"),{code:"ENOENT"});}) as unknown as typeof spawnSync});
  expect(await until(()=>!alive(child.pid!),5000)).toBe(true);
 }),40000);

 test("the fixture follows the documented grammar: case-insensitive switches, /T needed for the tree, /F needed at all, unknown pid 128, bad syntax 1",async()=>{
  const root=mkdtempSync(join(tmpdir(),"semio-taskkill-")),log=join(root,"taskkill.log");
  try{
   const target=(...args:string[])=>spawnSync(python,["-I",fixture,...args],{encoding:"utf8",env:{...process.env,SEMIO_TASKKILL_LOG:log}}).status;
   const holder=()=>spawn("sh",["-c","sleep 300 & wait"],{stdio:"ignore"});
   const plain=holder();expect(target("/pid",String(plain.pid))).toBe(1);expect(alive(plain.pid!)).toBe(true);
   expect(target("/PID",String(plain.pid),"/f")).toBe(0);expect(await until(()=>!alive(plain.pid!),3000)).toBe(true);
   const family=holder();expect(await until(()=>descendants(family.pid!).length>=1,3000)).toBe(true);const kids=descendants(family.pid!);
   expect(target("/pid",String(family.pid),"/F")).toBe(0);expect(await until(()=>!alive(family.pid!),3000)).toBe(true);expect(kids.every(alive),"without /T the descendants stay").toBe(true);
   for(const pid of kids)process.kill(pid,"SIGKILL");
   const deep=holder();expect(await until(()=>descendants(deep.pid!).length>=1,3000)).toBe(true);const deepKids=descendants(deep.pid!);
   expect(target("/Pid",String(deep.pid),"/T","/f")).toBe(0);expect(await until(()=>!alive(deep.pid!)&&deepKids.every(pid=>!alive(pid)),3000)).toBe(true);
   expect(target("/pid","999999","/T","/F")).toBe(128);expect(target("/T","/F")).toBe(1);expect(target("/pid","abc","/F")).toBe(1);expect(target("/im","x.exe","/F")).toBe(1);
  }finally{rmSync(root,{recursive:true,force:true});}
 },30000);
});
