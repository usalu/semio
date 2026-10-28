
import { receiveScriptProcessInvocation } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter, type ScriptInvocation } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { checkScriptInvocation, createIssuedResourceControlV1, requireIssuedResourceControlInvocationV1, scriptInvocationBudget, scriptInvocationEnvironment, scriptResourceProcessEnvironmentV1 } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { checkResourceLeaseControl, type ResourceLeaseControl } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { compilerAdmissionBudgetV1 } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { runOwnedCommand } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { resolveMcpBin } from "../../📚️library/📦️packages/🟦️typescript/🚨️policy/🖥️cli/🟦️.ts";
import { createHash, randomBytes } from "node:crypto";
import { homedir } from "node:os";
import { closeSync, constants, fstatSync, lstatSync, mkdirSync, openSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, posix, win32 } from "node:path";
import installationSchema from "./📥️installation/🧬️schema/🔣️.json" with { type: "json" };
import { isMainModuleV1 } from "../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";

const installationPolicy = installationSchema.$defs.Protocol.const;

/** 📍️Selects the real native executable's user installation without a shell alias or wrapper. */
export function nativeRepoInstallationPlanV1(platform:string,home:string,localAppData:string,digest:string):Readonly<{platform:string;home:string;localAppData:string;digest:string;directory:string;executable:string;root:string}> {
 if(!["darwin","linux","win32"].includes(platform)||!home||!/^([a-f0-9]{64})$/u.test(digest)||/[\r\n\0]/u.test(home+localAppData))throw Error("Supported native host and exact executable digest required");
 const path=platform==="win32"?win32:posix;if((platform==="win32"?(home+localAppData).includes(";"):home.includes(":"))||!path.isAbsolute(home)||(platform==="win32"&&!path.isAbsolute(localAppData)))throw Error("Absolute user installation roots required");
 const root=platform==="win32"?path.join(localAppData,...installationPolicy.windowsDirectory):path.join(home,".local","share","semio","repo"),directory=platform==="win32"?path.join(root,digest):path.join(home,...installationPolicy.unixDirectory),executable=path.join(directory,platform==="win32"?"repo.exe":"repo");
 if([root,directory,executable,executable+".stage-"+"0".repeat(12),path.join(root,"installed.json.stage-"+"0".repeat(12))].some(value=>value.length>installationPolicy.maximumPathLength))throw Error("Native repository installation exceeds the host path contract");
 return Object.freeze({platform,home,localAppData,digest,directory,executable,root});
}

/** 🧵️Updates only the managed shell block, retaining the surrounding user configuration. */
export function nativeRepoShellProfileV1(before:string,directory:string):string {
 const start="# >>> "+installationPolicy.marker,end="# <<< "+installationPolicy.marker,first=before.indexOf(start),last=before.indexOf(end),quote="'"+directory.replaceAll("'","'\\''")+"'",block=start+"\n_semio_repo_remaining=\"$PATH:\"\n_semio_repo_path="+quote+"\nwhile [ -n \"$_semio_repo_remaining\" ]; do\n  _semio_repo_entry=${_semio_repo_remaining%%:*}\n  _semio_repo_remaining=${_semio_repo_remaining#*:}\n  if [ \"$_semio_repo_entry\" != "+quote+" ]; then _semio_repo_path=\"$_semio_repo_path:$_semio_repo_entry\"; fi\ndone\nexport PATH=\"$_semio_repo_path\"\nunset _semio_repo_remaining _semio_repo_path _semio_repo_entry\n"+end;
 if(first<0&&last<0)return before+(before&&!before.endsWith("\n")?"\n":"")+block+"\n";
 if(first<0||last<first||before.indexOf(start,first+start.length)>=0||before.indexOf(end,last+end.length)>=0)throw Error("Ambiguous managed repository PATH block");
 return before.slice(0,first)+block+before.slice(last+end.length);
}

function publishNativeRepoFileV1(destination:string,bytes:Uint8Array,mode:number,check:()=>void):void {
 check();const temporary=destination+".stage-"+randomBytes(6).toString("hex");let published=false,created=false;
 try{const descriptor=openSync(temporary,"wx",mode);created=true;try{writeFileSync(descriptor,bytes);}finally{closeSync(descriptor);}check();renameSync(temporary,destination);published=true;}finally{if(created&&!published)rmSync(temporary,{force:true});}
}

function nativeRepoDirectoryV1(directory:string):void {
 const facts=lstatSync(directory,{throwIfNoEntry:false});if(facts){if(!facts.isDirectory()||facts.isSymbolicLink())throw Error("Physical native installation directory required: "+directory);return;}
 const parent=dirname(directory);if(parent!==directory)nativeRepoDirectoryV1(parent);mkdirSync(directory);
}

function nativeRepoBytesV1(file:string,platform:string):Buffer {
 const descriptor=openSync(file,constants.O_RDONLY|(constants.O_NOFOLLOW??0));try{const before=fstatSync(descriptor);if(!before.isFile()||before.size<4||before.size>installationPolicy.maximumBinaryBytes)throw Error("Bounded physical native executable required");const bytes=readFileSync(descriptor),after=fstatSync(descriptor);if(before.ino!==after.ino||before.dev!==after.dev||before.size!==after.size||before.mtimeMs!==after.mtimeMs||before.ctimeMs!==after.ctimeMs||bytes.length!==before.size)throw Error("Native executable changed during installation");const magic=bytes.subarray(0,4).toString("hex"),native=platform==="win32"?magic.startsWith("4d5a"):platform==="linux"?magic==="7f454c46":["cffaedfe","cefaedfe","feedfacf","feedface","cafebabe","bebafeca"].includes(magic);if(!native)throw Error("Actual platform native executable required");return bytes;}finally{closeSync(descriptor);}
}

/** 📦️Publishes the owned native executable, persists PATH and verifies the installed command under its Original. */
class InstallScript extends BundleScript {
 incomingControl?:ResourceLeaseControl;
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("Expected install without arguments");const control=this.incomingControl??createIssuedResourceControlV1(this.invocation,"repo:install",0);requireIssuedResourceControlInvocationV1(control,this.invocation);const check=()=>{checkScriptInvocation(this.invocation);checkResourceLeaseControl(control);};check();const source=resolveMcpBin(this.repoRoot),bytes=nativeRepoBytesV1(source,process.platform),digest=createHash("sha256").update(bytes).digest("hex"),plan=nativeRepoInstallationPlanV1(process.platform,homedir(),process.env.LOCALAPPDATA??"",digest),run=async(program:string,args:string[],env:NodeJS.ProcessEnv)=>{check();await runOwnedCommand(program,args,this.repoRoot,"repo:install",scriptInvocationBudget(this.invocation,compilerAdmissionBudgetV1(control)),{control,env:scriptResourceProcessEnvironmentV1(control,scriptInvocationEnvironment(this.invocation,env)),signal:control.signal});check();};
  await run(source,["--help"],process.env);if(createHash("sha256").update(nativeRepoBytesV1(source,process.platform)).digest("hex")!==digest)throw Error("Native build changed after installation probe");
  nativeRepoDirectoryV1(plan.root);nativeRepoDirectoryV1(plan.directory);const receiptPath=join(plan.root,"installed.json");let previous:any;
  const receiptFacts=lstatSync(receiptPath,{throwIfNoEntry:false});if(receiptFacts){if(!receiptFacts.isFile()||receiptFacts.isSymbolicLink()||receiptFacts.size>4096)throw Error("Physical native installation receipt required");previous=JSON.parse(readFileSync(receiptPath,"utf8"));}
  const actual=lstatSync(plan.executable,{throwIfNoEntry:false});if(actual){if(!actual.isFile()||actual.isSymbolicLink())throw Error("Native installation refuses a non-file destination");const current=createHash("sha256").update(nativeRepoBytesV1(plan.executable,process.platform)).digest("hex");if(current!==digest&&(!previous||previous.executable!==plan.executable||previous.digest!==current))throw Error("Native installation refuses an executable owned by another installer");if(current!==digest)publishNativeRepoFileV1(plan.executable,bytes,0o755,check);}else publishNativeRepoFileV1(plan.executable,bytes,0o755,check);
  if(process.platform==="win32"){
   const script='$ErrorActionPreference="Stop";$root=$env:SEMIO_REPO_INSTALL_ROOT;$selected=$env:SEMIO_REPO_INSTALL_BIN;$old=[Environment]::GetEnvironmentVariable("Path","User");$kept=@();foreach($entry in ($old -split ";")){if(!$entry){if($null -ne $old -and $old.Length -gt 0){$kept+=$entry};continue};$parent=[IO.Path]::GetDirectoryName($entry.TrimEnd("\\"));$leaf=[IO.Path]::GetFileName($entry.TrimEnd("\\"));if($parent -ieq $root -and $leaf -cmatch "^[a-f0-9]{64}$"){continue};$kept+=$entry};[Environment]::SetEnvironmentVariable("Path",((@($selected)+$kept)-join ";"),"User")';
   await run("powershell.exe",["-NoProfile","-NonInteractive","-Command",script],{...process.env,SEMIO_REPO_INSTALL_ROOT:plan.root,SEMIO_REPO_INSTALL_BIN:plan.directory});
  }else{
   const profiles=installationPolicy.shellProfiles.filter(name=>![".bash_profile",".bash_login"].includes(name)||Boolean(lstatSync(join(plan.home,name),{throwIfNoEntry:false})));
   for(const name of profiles){check();const file=join(plan.home,name),facts=lstatSync(file,{throwIfNoEntry:false});if(facts&&(!facts.isFile()||facts.isSymbolicLink()))throw Error("Physical shell profile required: "+file);const before=facts?readFileSync(file,"utf8"):"";if(Buffer.byteLength(before)>installationPolicy.maximumProfileBytes)throw Error("Bounded shell profile required");const after=nativeRepoShellProfileV1(before,plan.directory);if(after!==before)publishNativeRepoFileV1(file,Buffer.from(after),facts?facts.mode&0o777:0o600,check);}
  }
  const pathKey=Object.keys(process.env).find(key=>key.toLowerCase()==="path")??"PATH",environment={...process.env,[pathKey]:plan.directory+(process.platform==="win32"?";":":")+(process.env[pathKey]??"")};
  await run("repo",["--help"],environment);publishNativeRepoFileV1(receiptPath,Buffer.from(JSON.stringify({version:1,platform:plan.platform,digest,executable:plan.executable,directory:plan.directory,source,mode:process.env.SEMIO_BUILD_MODE??"prototype"})+"\n"),0o600,check);const pin=join(this.repoRoot,".🧬semio/🦑️repo/native-repo-path");publishNativeRepoFileV1(pin,Buffer.from(plan.executable.replaceAll("\\","/")+"\n"),0o600,check);console.log("[repo] Installed native repository command: "+plan.executable);
 }
}

class InstallationTestScript extends BundleScript {
 async run():Promise<void>{const {runRepositoryTestCommand}=await import("../../📚️library/📦️packages/🟦️typescript/🧪️testing/🎛️repository-command/🟦️.ts");await runRepositoryTestCommand(process.execPath,["test",join(this.root,"📥️installation/🧪️tests/🟦️.ts"),join(this.root,"../../📚️library/🔍️discovery/🐹️go-layout/🧪️tests/🟦️.ts"),join(this.root,"../../📚️library/🧪️testing/🐹️go-plan/🧪️tests/🔒️lease/🟦️.ts")],{cwd:this.repoRoot});}
}

/** 🔗️Invokes the native installer with the existing authenticated root invocation. */
export async function installNativeRepoV1(context:Readonly<{root:string;repoRoot:string;invocation:ScriptInvocation;control?:ResourceLeaseControl}>):Promise<void>{
 await Object.assign(new InstallScript(context.root,context.repoRoot,context.invocation),{incomingControl:context.control}).run([]);
}

class BuildScript extends BundleScript {
  async run(): Promise<void> {
    const {runBuildScript}=await import("./🟦️.ts");
    await runBuildScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const {runTestScript}=await import("./🟦️.ts");
    await runTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

if(isMainModuleV1(import.meta.url)){
 const router = new ScriptRouter(import.meta.dirname).register("build", BuildScript).register("test", TestScript).register("install",InstallScript).register("test-install",InstallationTestScript);
 await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
}
