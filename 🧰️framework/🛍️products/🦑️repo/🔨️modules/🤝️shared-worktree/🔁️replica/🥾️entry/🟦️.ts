import {spawn,spawnSync} from "node:child_process";
import {randomBytes} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {homedir,tmpdir} from "node:os";
import {extname,join,posix,relative,win32} from "node:path";
import {compileJsonSchemaSubset,type CompiledJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {bootModuleRelativeV1} from "../🛟️supervisor/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🧾️ The result of installing or refreshing the boot program: `installed`, `current` (nothing to do, `changed` when only the recorded worktree or store moved), or one of the refusals: `refused-downgrade` (the installed version is higher), `refused-same-version` (same version, other files), `refused-selftest` (the program failed its own self-test, `reason` says how), `refused-foreign` (the installed program belongs to another worktree that still holds a machine identity) and `refused-incomplete`. */
export type EntryResultV1=Readonly<{action:"installed"|"current"|"refused-downgrade"|"refused-same-version"|"refused-selftest"|"refused-foreign"|"refused-incomplete";version:number;reason?:string;installed?:number;changed?:boolean}>;

/** 🩺️ The state file `<entry>/state.json` the boot program keeps: its phase (`starting`, `running`, `waiting` for a missing worktree, `exhausted` when no code starts), the code it runs (source `worktree`, `snapshot` or `store`), the last attempts and the time of the next try. */
export type EntryStateV1=Readonly<{version:1;pid:number;startedMs:number;wallMs:number;bootVersion:number;phase:"starting"|"running"|"waiting"|"exhausted";source:"worktree"|"snapshot"|"store"|null;digest:string|null;base:string|null;reason:string;attempts:readonly Readonly<{source:string;digest:string|null;outcome:string;atMs:number}>[];nextMs:number|null}>;

const validators=new Map<string,CompiledJsonSchemaSubset>();

/** 🛡️ Validates a value against one definition of the entry schema. */
export function requireEntryShapeV1(definition:"EntryManifest"|"EntryState"|"EntryResult"|"Fixture",value:unknown,message:string):void{
 let validator=validators.get(definition);
 if(validator===undefined){validator=compileJsonSchemaSubset({$ref:`#/definitions/${definition}`},schema);validators.set(definition,validator);}
 const errors=validator.validate(value);
 if(errors.length)throw Error(`${message}: ${errors[0]}`);
}

/** 📂️ The per-account directory of the installed boot program, outside every worktree and writable without privileges: `~/Library/Application Support/semio/fleet/entry` on macOS, `$XDG_DATA_HOME/semio/fleet/entry` (default `~/.local/share`) on Linux, `%LOCALAPPDATA%\semio\fleet\entry` on Windows; `SEMIO_FLEET_ENTRY_DIR` overrides it. The same rule runs in `🟨️.mjs` and `📜️script.ts`, which cannot import this file. */
export function entryDirectoryV1(env:Readonly<Record<string,string|undefined>>=process.env,platform:string=process.platform,home:string=homedir()):string{
 const {join:combine}=platform==="win32"?win32:posix;
 return env.SEMIO_FLEET_ENTRY_DIR??(platform==="win32"?combine(env.LOCALAPPDATA??combine(home,"AppData","Local"),"semio","fleet","entry"):platform==="darwin"?combine(home,"Library","Application Support","semio","fleet","entry"):combine(env.XDG_DATA_HOME??combine(home,".local","share"),"semio","fleet","entry"));
}

/** 🩺️ Reads the state file of the boot program; `undefined` when there is none or it is unreadable. */
export function readEntryStateV1(entry:string):EntryStateV1|undefined{
 try{const value=JSON.parse(readFileSync(join(entry,"state.json"),"utf8")) as EntryStateV1;requireEntryShapeV1("EntryState",value,"state.json");return value;}catch{return undefined;}
}

const lastLine=(text:string):string=>text.trim().split("\n").filter(line=>line.startsWith("{")).pop()??"";

/** 🪃️ Installs or refreshes the boot program from the source directory `<moduleRoot>/🔁️replica/🥾️entry` by running the source's own `install` command with `runtime`: the source proves itself first (`self-test`), a lower or equal version never replaces the installed one, a damaged installation is repaired, and a refused update leaves the installed files and `entry.json` untouched. Called by a healthy daemon at start and every hour, and by provisioning. */
export async function maintainEntryV1(options:Readonly<{worktree:string;moduleRoot:string;storeRoot:string|null;entryDirectory?:string;runtime?:string;timeoutMs?:number;signal?:AbortSignal}>):Promise<EntryResultV1>{
 const source=join(options.moduleRoot,"🔁️replica","🥾️entry"),program=existsSync(source)?readdirSync(source).find(name=>extname(name)===".mjs"):undefined;
 if(program===undefined)return {action:"refused-incomplete",version:0,reason:`no boot program in ${source}`};
 const environment={...process.env,...options.entryDirectory===undefined?{}:{SEMIO_FLEET_ENTRY_DIR:options.entryDirectory}};
 return new Promise<EntryResultV1>(done=>{
  const proc=spawn(options.runtime??process.execPath,[join(source,program),"install","--worktree",options.worktree,"--module",relative(options.worktree,options.moduleRoot).split("\\").join("/"),...options.storeRoot===null?[]:["--store",options.storeRoot]],{env:environment,stdio:["ignore","pipe","pipe"],windowsHide:true});
  let out="",err="";
  const timer=setTimeout(()=>{proc.kill("SIGKILL");},options.timeoutMs??180_000),abort=():void=>{proc.kill("SIGKILL");};
  options.signal?.addEventListener("abort",abort,{once:true});
  proc.stdout.on("data",chunk=>{out+=String(chunk);});
  proc.stderr.on("data",chunk=>{err+=String(chunk);});
  proc.on("error",error=>{clearTimeout(timer);done({action:"refused-selftest",version:0,reason:error.message});});
  proc.on("close",()=>{
   clearTimeout(timer);options.signal?.removeEventListener("abort",abort);
   try{const result=JSON.parse(lastLine(out)) as EntryResultV1;requireEntryShapeV1("EntryResult",result,"install result");done(result);}
   catch{done({action:"refused-selftest",version:0,reason:(err||out||"the install command gave no result").slice(0,512)});}
  });
 });
}

/** 🚪️ The path of the trampoline below the worktree root: the one file the service managers of already joined machines still run. */
export const entryScriptPathV1=`${bootModuleRelativeV1}/🔁️replica/📜️script.ts`;

/** 🛡️ Whether an incoming change to this worktree-relative path is guarded by `admitEntryScriptV1`. */
export const entryGuardedV1=(path:string):boolean=>path===entryScriptPathV1;

const importSpecifier=/\bfrom\s*["']([^"']+)["']|\bimport\s*\(\s*["']([^"']+)["']|\bimport\s*["']([^"']+)["']|\brequire\s*\(\s*["']([^"']+)["']/gu;
const trampolineSize=16*1024;

/** 🧪️ Decides whether a new version of the trampoline may replace the one in the worktree. The file at `file` must stay under 16 KiB, import only `node:` built-ins statically and `./`-relative files lazily (dynamic imports of an expression are allowed), and in a throwaway directory next to a stub `🛟️supervisor` it must (1) print `trampoline-ok` and exit 0 for `--trampoline-self-test`, (2) hand a non-`run` command over to the supervisor (`scriptMainV1` receives the arguments) and (3) stay silent about boot programs when `SEMIO_FLEET_ENTRY_DIR` names an empty directory. Resolves to `undefined` when admitted, else to the reason. */
export async function admitEntryScriptV1(file:string,options:Readonly<{runtime?:string;timeoutMs?:number}>={}):Promise<string|undefined>{
 let source:string;
 try{source=readFileSync(file,"utf8");}catch(error){return `unreadable: ${error instanceof Error?error.message:String(error)}`;}
 if(Buffer.byteLength(source)>trampolineSize)return `larger than ${trampolineSize} bytes`;
 for(const match of source.matchAll(importSpecifier)){
  const name=match[1]??match[2]??match[3]??match[4]!,dynamic=match[2]!==undefined;
  if(!name.startsWith("node:")&&!(dynamic&&name.startsWith("./")))return `imports ${name}, which is not a node: built-in`;
 }
 const base=mkdtempSync(join(tmpdir(),"semio-entry-guard-")),runtime=options.runtime??process.execPath,timeout=options.timeoutMs??30_000;
 try{
  const script=join(base,"🔁️replica","📜️script.ts"),stub=join(base,"🔁️replica","🛟️supervisor","🟦️.ts"),empty=join(base,"entry");
  mkdirSync(join(base,"🔁️replica","🛟️supervisor"),{recursive:true});mkdirSync(empty,{recursive:true});
  writeFileSync(script,source);
  writeFileSync(stub,"export async function scriptMainV1(argv:readonly string[]):Promise<number>{process.stdout.write(`supervisor-ok ${argv.join(\" \")}\\n`);return 0;}\n");
  const run=(args:readonly string[]):Promise<{status:number|null;out:string;err:string}>=>new Promise(done=>{
   const proc=spawn(runtime,[script,...args],{env:{...process.env,SEMIO_FLEET_ENTRY_DIR:empty,SEMIO_FLEET_BOOT_ROOT:base},stdio:["ignore","pipe","pipe"],windowsHide:true});
   let out="",err="";
   const timer=setTimeout(()=>{proc.kill("SIGKILL");},timeout);
   proc.stdout.on("data",chunk=>{out+=String(chunk);});
   proc.stderr.on("data",chunk=>{err+=String(chunk);});
   proc.on("error",error=>{clearTimeout(timer);done({status:null,out,err:error.message});});
   proc.on("close",status=>{clearTimeout(timer);done({status,out,err});});
  });
  const first=await run(["--trampoline-self-test"]);
  if(first.status!==0||first.out.trim()!=="trampoline-ok")return `the self-test failed (exit ${first.status}): ${(first.err||first.out).trim().slice(0,200)}`;
  const second=await run(["ping","1"]);
  if(second.status!==0||!second.out.includes("supervisor-ok ping 1"))return `a command did not reach the supervisor (exit ${second.status}): ${(second.err||second.out).trim().slice(0,200)}`;
  const third=await run(["run"]);
  if(third.status!==0||!third.out.includes("supervisor-ok run"))return `run without a boot program did not reach the supervisor (exit ${third.status}): ${(third.err||third.out).trim().slice(0,200)}`;
  return undefined;
 }finally{rmSync(base,{recursive:true,force:true});}
}

const xml=(text:string):string=>text.replace(/&/gu,"&amp;").replace(/</gu,"&lt;");

/** 🍎️ Points the LaunchAgent of the replica in the account's own `~/Library/LaunchAgents` at the installed shim instead of the worktree script. A user can rewrite its own agent without privileges. launchd reads the file again only when the agent is bootstrapped (login, reboot), so the running daemon is untouched. Only the exact shape that provisioning renders is rewritten (the program array with `📜️script.ts`, an optional single-line `WorkingDirectory`), the result must pass `plutil -lint`, and the previous file is kept as `<entry>/agent.plist.previous`. Returns `rewritten`, `current`, `absent` or `unrecognised`. */
export function migrateAgentV1(options:Readonly<{home:string;entryDirectory:string;platform?:string;lint?:boolean}>):"rewritten"|"current"|"absent"|"unrecognised"{
 if((options.platform??process.platform)!=="darwin")return "absent";
 const file=join(options.home,"Library","LaunchAgents","tech.semio.fleet.replica.plist");
 if(!existsSync(file))return "absent";
 const original=readFileSync(file,"utf8"),shim=join(options.entryDirectory,"boot.sh");
 if(original.includes(`<string>${xml(shim)}</string>`))return "current";
 const program=/<key>ProgramArguments<\/key><array>(?:<string>[^<]*<\/string>)*<string>\.\/📜️script\.ts<\/string><string>run<\/string><\/array>/u;
 if(!program.test(original))return "unrecognised";
 const next=original.replace(program,`<key>ProgramArguments</key><array><string>/bin/sh</string><string>${xml(shim)}</string><string>run</string></array>`).replace(/<key>WorkingDirectory<\/key><string>[^<]*<\/string>\n?/u,"");
 const temporary=`${file}.${randomBytes(4).toString("hex")}.tmp`;
 try{
  writeFileSync(temporary,next);
  if(options.lint!==false){const linted=spawnSync("plutil",["-lint",temporary],{encoding:"utf8"});if(linted.status!==0){rmSync(temporary,{force:true});return "unrecognised";}}
  mkdirSync(options.entryDirectory,{recursive:true});
  writeFileSync(join(options.entryDirectory,"agent.plist.previous"),original);
  renameSync(temporary,file);
  return "rewritten";
 }catch{rmSync(temporary,{force:true});return "unrecognised";}
}
