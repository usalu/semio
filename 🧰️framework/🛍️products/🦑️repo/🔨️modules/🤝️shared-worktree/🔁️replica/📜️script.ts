#!/usr/bin/env repo
import {spawn} from "node:child_process";
import {createHash} from "node:crypto";
import {existsSync,readFileSync,realpathSync} from "node:fs";
import {homedir} from "node:os";
import {join,resolve} from "node:path";
import {fileURLToPath,pathToFileURL} from "node:url";

const here=import.meta.dirname,args=process.argv.slice(2),platform=process.platform,home=homedir();
const entryDirectory=process.env.SEMIO_FLEET_ENTRY_DIR??(platform==="win32"?join(process.env.LOCALAPPDATA??join(home,"AppData","Local"),"semio","fleet","entry"):platform==="darwin"?join(home,"Library","Application Support","semio","fleet","entry"):join(process.env.XDG_DATA_HOME??join(home,".local","share"),"semio","fleet","entry"));
const read=(file:string):string|undefined=>{try{return readFileSync(file,"utf8");}catch{return undefined;}};
const same=(left:string,right:string):boolean=>{try{return realpathSync(left)===realpathSync(right);}catch{return resolve(left)===resolve(right);}};
const root=process.env.SEMIO_FLEET_BOOT_ROOT??resolve(here,"../../../../../..");

/** 🪃️ The installed boot program that takes the service entry over: the shim and the program of `entry.json` exist, their sha256 equals the manifest and the manifest names this worktree. */
function installed():string|undefined{
 try{
  const manifest=JSON.parse(read(join(entryDirectory,"entry.json"))??"") as {worktree?:string;files?:Record<string,{sha256?:string}>};
  if(typeof manifest.worktree!=="string"||!same(manifest.worktree,root))return undefined;
  for(const name of ["boot.mjs",platform==="win32"?"boot.ps1":"boot.sh"])if(createHash("sha256").update(readFileSync(join(entryDirectory,name))).digest("hex")!==manifest.files?.[name]?.sha256)return undefined;
  return join(entryDirectory,platform==="win32"?"boot.ps1":"boot.sh");
 }catch{return undefined;}
}

async function supervisor():Promise<number>{
 const stage=join(root,".cache","f","boot","stage.ts"),signature=JSON.parse(read(join(root,".cache","f","boot","stage.json"))??"{}") as {sha256?:string};
 const candidates=[join(here,"🛟️supervisor","🟦️.ts"),...existsSync(stage)&&signature.sha256===createHash("sha256").update(readFileSync(stage)).digest("hex")?[stage]:[]];
 let failure:unknown;
 for(const file of candidates){
  try{return await ((await import(pathToFileURL(file).href)) as {scriptMainV1:(argv:readonly string[])=>Promise<number>}).scriptMainV1(args);}
  catch(error){failure=error;}
 }
 process.stderr.write(`The replica script does not start / Das Replica-Skript startet nicht: ${failure instanceof Error?failure.message:String(failure)}\n`);
 return 1;
}

/** 🪃️ The stable service entry. It imports nothing but Node built-ins, and the guard of the daemon refuses every incoming change to it that does not parse and pass `--trampoline-self-test`. `run` hands over to the boot program installed beside the machine (outside the worktree, see `🥾️entry`) when it belongs to this worktree and stays valid; if that program ends within 10 s with an error the script falls back to `🛟️supervisor`. Every other command, and `run` without a boot program, goes to `🛟️supervisor` (or its verified copy in `.cache/f/boot`). */
async function main():Promise<number>{
 const shim=args[0]==="run"&&process.env.SEMIO_FLEET_ENTRY===undefined?installed():undefined;
 if(shim===undefined)return supervisor();
 const started=Date.now(),program=platform==="win32"?"powershell.exe":"/bin/sh",flags=platform==="win32"?["-NoProfile","-ExecutionPolicy","Bypass","-File"]:[];
 const code=await new Promise<number>(done=>{
  const proc=spawn(program,[...flags,shim,...args],{stdio:"inherit",env:{...process.env,SEMIO_FLEET_ENTRY:"trampoline",SEMIO_FLEET_BOOT_ROOT:root},windowsHide:true});
  const forward=(name:NodeJS.Signals):void=>{try{proc.kill(name);}catch{}};
  for(const name of ["SIGINT","SIGTERM"] as const)process.on(name,()=>{forward(name);});
  proc.on("error",()=>{done(127);});
  proc.on("exit",(status,name)=>{done(status??(name==="SIGTERM"?143:1));});
 });
 return code===0||code===75||Date.now()-started>10_000?code:supervisor();
}

const entry=process.argv[1],canonical=(path:string):string=>{const real=realpathSync(path);return platform==="win32"?real.toLowerCase():real;};
if(entry!==undefined&&entry!==""&&(()=>{try{return canonical(entry)===canonical(fileURLToPath(import.meta.url));}catch{return false;}})()){
 if(args[0]==="--trampoline-self-test"){process.stdout.write("trampoline-ok\n");process.exit(0);}
 process.exitCode=await main();
}
