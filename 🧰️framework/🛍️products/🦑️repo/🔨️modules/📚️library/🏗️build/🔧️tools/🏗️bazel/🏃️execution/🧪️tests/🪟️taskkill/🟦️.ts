import {describe,expect,test} from "bun:test";
import {spawn,spawnSync} from "node:child_process";
import {join} from "node:path";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {BAZEL_HERMETIC_RUNNER} from "../../🏭️runner/🟦️.ts";

/** 🪟️ The worker's cancellation kills the whole process tree of a running command: POSIX through the process group, Windows through `taskkill /PID n /T /F`. The Windows branch runs here on a REAL process tree against the shared `taskkill` fixture (documented grammar and exit codes); the root is still ended when `taskkill` fails or is missing; a command that exited by itself is never signalled. */
const line=BAZEL_HERMETIC_RUNNER.split("\n").find(row=>row.startsWith("const terminate="))!;
const terminate=new Function("Bun","process",line+"\nreturn terminate;")(Bun,process) as (child:unknown,platform?:string,run?:(command:string[])=>{exitCode:number},kill?:(pid:number,name:string)=>void)=>void;
const python=testPythonV1()??Bun.which("python3")??"python3",fixture=join(import.meta.dir,"../../../../../../../../../../🔨️modules/🏃️process/🪓️termination/🧪️tests/🪟️taskkill/🐍️taskkill.py");
const alive=(pid:number):boolean=>{try{process.kill(pid,0);}catch{return false;}const state=spawnSync("ps",["-o","stat=","-p",String(pid)],{encoding:"utf8"});return state.status===0&&!state.stdout.trim().startsWith("Z");};
const kids=(root:number):number[]=>{const found:number[]=[];for(const frontier=[root];frontier.length>0;){const rows=spawnSync("pgrep",["-P",String(frontier.pop()!)],{encoding:"utf8"}).stdout.split("\n").filter(Boolean).map(Number);found.push(...rows);frontier.push(...rows);}return found;};
async function until(predicate:()=>boolean,milliseconds=5000):Promise<boolean>{const deadline=Date.now()+milliseconds;while(!predicate()){if(Date.now()>deadline)return false;await Bun.sleep(40);}return true;}
const family=async()=>{const child=Bun.spawn(["sh","-c","sleep 300 & (sleep 300 & wait) & wait"],{stdout:"ignore",stderr:"ignore",stdin:"ignore"});expect(await until(()=>kids(child.pid).length>=3)).toBe(true);return{child,descendants:kids(child.pid)};};
const standIn=(environment:Record<string,string>={})=>{const commands:string[][]=[];return{commands,run:(command:string[])=>{commands.push(command);return Bun.spawnSync([python,"-I",fixture,...command.slice(1)],{stdout:"ignore",stderr:"ignore",env:{...process.env,...environment}}) as unknown as {exitCode:number};}};};
const sweep=(pids:number[])=>{for(const pid of pids)if(alive(pid))process.kill(pid,"SIGKILL");};

describe("worker command termination",()=>{
 test("the runner declares one terminate function and the stop path uses it",()=>{expect(line).toContain("taskkill");expect(BAZEL_HERMETIC_RUNNER).toContain("for(const child of children)terminate(child)");expect(BAZEL_HERMETIC_RUNNER.match(/"taskkill"/g)?.length).toBe(1);});

 test("Windows: taskkill /PID n /T /F ends the command and every descendant",async()=>{
  const {child,descendants}=await family(),host=standIn();
  try{terminate(child,"win32",host.run);expect(await until(()=>!alive(child.pid)&&descendants.every(pid=>!alive(pid)))).toBe(true);expect(host.commands).toEqual([["taskkill","/PID",String(child.pid),"/T","/F"]]);}
  finally{sweep([child.pid,...descendants]);}
 });

 test("Windows: a refused taskkill (exit 1) still ends the command itself, a vanished process (128) needs nothing",async()=>{
  const {child,descendants}=await family(),refused=standIn({SEMIO_TASKKILL_FAIL:"1"});
  try{terminate(child,"win32",refused.run);expect(await until(()=>!alive(child.pid))).toBe(true);expect(refused.commands.length).toBe(1);}
  finally{sweep([child.pid,...descendants]);}
  const gone=spawn(process.execPath,["-e",""],{stdio:"ignore"});await new Promise<void>(accept=>gone.once("exit",()=>accept()));
  const calls:string[]=[],host=standIn();terminate({pid:gone.pid,exitCode:0,kill:()=>calls.push("kill")},"win32",host.run);expect(calls,"the pid is gone: nothing else is killed").toEqual([]);
 });

 test("Windows: a missing taskkill (the start throws) ends the command itself",async()=>{
  const {child,descendants}=await family();
  try{terminate(child,"win32",()=>{throw Object.assign(new Error("spawn taskkill ENOENT"),{code:"ENOENT"});});expect(await until(()=>!alive(child.pid))).toBe(true);}
  finally{sweep([child.pid,...descendants]);}
 });

 test("POSIX: the command's process group is signalled, never taskkill",()=>{
  const signals:[number,string][]=[],host=standIn();terminate({pid:4242,kill:()=>{throw Error("no root kill needed");}},"linux",host.run,(pid,name)=>{signals.push([pid,name]);});
  expect(signals).toEqual([[-4242,"SIGKILL"]]);expect(host.commands).toEqual([]);
  const rooted:string[]=[];terminate({pid:4242,kill:(name:string)=>rooted.push(name)},"linux",host.run,()=>{throw Object.assign(new Error("ESRCH"),{code:"ESRCH"});});expect(rooted,"a group that cannot be signalled falls back to the command").toEqual(["SIGKILL"]);
 });
});
