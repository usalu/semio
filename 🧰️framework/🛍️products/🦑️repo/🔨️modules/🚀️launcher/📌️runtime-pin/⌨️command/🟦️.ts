import{spawn}from"node:child_process";
import{accessSync,constants,lstatSync,readFileSync,readdirSync,statSync}from"node:fs";
import{homedir}from"node:os";
import{delimiter,dirname,join,resolve}from"node:path";
import{fileURLToPath}from"node:url";
import{guardDecisionV1,guardMessageV1,locatePinnedRuntimeV1,parseProcCmdlineV1,parseProcessArguments2V1,pinnedRuntimeV1,splitWindowsCommandLineV1,type RuntimePinHostV1,type RuntimePinRegistryV1}from"../🟦️.ts";

const registryRelative="🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🗂️registry/🔣️.json",installRelative="🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📥️install",guardDepthV1="SEMIO_RUNTIME_GUARD";

/** 🏠️ The operating system's per-user cache directory, as the Go launcher's `os.UserCacheDir` names it. */
export function userCacheDirectoryV1(platform:string,environment:Readonly<Record<string,string|undefined>>,home:string):string|undefined{
  if(platform==="win32")return environment.LocalAppData||undefined;
  if(platform==="darwin")return home===""?undefined:join(home,"Library","Caches");
  return environment.XDG_CACHE_HOME||(home===""?undefined:join(home,".cache"));
}

/** 🧾️ Whether a directory holds a verified install: a regular, executable file and a receipt that names the pinned archive (and the file size when the receipt records one). */
export function installedRuntimeV1(directory:string,executable:string,sha256:string,platform:string=process.platform):boolean{
  try{
    const file=lstatSync(join(directory,executable));
    if(!file.isFile()||platform!=="win32"&&(file.mode&0o100)===0)return false;
    const receipt=JSON.parse(readFileSync(join(directory,".toolchain.json"),"utf8")) as{archive?:string;bytes?:number};
    return receipt.archive===sha256&&(receipt.bytes===undefined||receipt.bytes===0||receipt.bytes===file.size);
  }catch{return false;}
}

/** 💻️ The host this process runs on. */
export function systemRuntimePinHostV1(root:string,environment:Readonly<Record<string,string|undefined>>=process.env):RuntimePinHostV1{
  let musl=false;
  if(process.platform==="linux"){try{musl=readdirSync("/lib").some(name=>/^ld-musl-.*\.so\.1$/u.test(name));}catch{musl=false;}}
  return Object.freeze({root,platform:process.platform,arch:process.arch,musl,environment,userCacheDirectory:userCacheDirectoryV1(process.platform,environment,environment.HOME||environment.USERPROFILE||homedir()),installed:(directory,executable,sha256)=>installedRuntimeV1(directory,executable,sha256)});
}

/** 📖️ Reads the launcher registry of a checkout. */
export function readRuntimePinRegistryV1(root:string):RuntimePinRegistryV1{
  return JSON.parse(readFileSync(join(root,registryRelative),"utf8")) as RuntimePinRegistryV1;
}

/** ⚙️ Reads the exact command line this process was started with, which the runtime hides from a preload: `/proc` on Linux, the kernel's process arguments on macOS and `GetCommandLineW` on Windows (all through `bun:ffi`, no child process); undefined when it cannot be read. */
export async function readCommandLineV1(platform:string=process.platform,pid:number=process.pid):Promise<readonly string[]|undefined>{
  try{
    if(platform==="linux")return parseProcCmdlineV1(readFileSync(`/proc/${pid}/cmdline`));
    const{runtimeNativeSystemV1}=await import("../../../../../../📦️packages/🟦️typescript/⚙️runtime/🧲️native/🟦️.ts"),ffi=runtimeNativeSystemV1();
    if(platform==="darwin"){
      const lib=ffi.dlopen("libSystem.B.dylib",{sysctl:{args:[ffi.FFIType.ptr,ffi.FFIType.u32,ffi.FFIType.ptr,ffi.FFIType.ptr,ffi.FFIType.ptr,ffi.FFIType.u64],returns:ffi.FFIType.i32}});
      const mib=new Int32Array([1,49,pid]),size=new BigUint64Array(1);
      if(lib.symbols.sysctl(ffi.ptr(mib),3,null,ffi.ptr(size),null,0n)!==0)return undefined;
      const bytes=new Uint8Array(Number(size[0]));
      if(lib.symbols.sysctl(ffi.ptr(mib),3,ffi.ptr(bytes),ffi.ptr(size),null,0n)!==0)return undefined;
      return parseProcessArguments2V1(bytes.subarray(0,Number(size[0])));
    }
    if(platform==="win32"){
      const lib=ffi.dlopen("kernel32.dll",{GetCommandLineW:{args:[],returns:ffi.FFIType.ptr}}),start=lib.symbols.GetCommandLineW();
      if(start===null)return undefined;
      let line="";
      for(let offset=0;;offset+=2){const unit=ffi.read.u16(start,offset);if(unit===0)break;line+=String.fromCharCode(unit);}
      return splitWindowsCommandLineV1(line);
    }
  }catch{return undefined;}
  return undefined;
}

/** 🔎️ Finds the `repo` launcher on the PATH with plain file system calls: this module runs in the preload of every process, so it loads neither the runtime adapter nor the runtime seam. */
export function launcherPathV1(environment:Readonly<Record<string,string|undefined>>):string|undefined{
  const extensions=process.platform==="win32"?(environment.PATHEXT??".EXE;.CMD;.BAT").split(";").filter(Boolean):[""];
  for(const directory of (environment.PATH??"").split(delimiter)){
    if(directory==="")continue;
    for(const extension of extensions){
      const candidate=join(directory,"repo"+extension);
      try{if(!lstatSync(candidate).isFile()&&!statSync(candidate).isFile())continue;if(process.platform!=="win32")accessSync(candidate,constants.X_OK);return candidate;}catch{}
    }
  }
  return undefined;
}

/** 🏁️ The result of checking the running Bun against the pin: what to do, and the facts a message or a re-run needs. */
export type RuntimeCheckV1=Readonly<{decision:ReturnType<typeof guardDecisionV1>;running:string;version:string;name:string;path:string|undefined;launcher:string|undefined;registry:RuntimePinRegistryV1}>;

/** 🔬️ Compares the running Bun with the registry's pin and finds the provisioned executable and the `repo` launcher without starting anything; the command line is read only when the running Bun is not the pinned one. */
export async function checkRuntimeV1(root:string,running:string,mode:"run"|"test",readLine:()=>Promise<readonly string[]|undefined>,environment:Readonly<Record<string,string|undefined>>=process.env,host:RuntimePinHostV1,launcher:string|undefined):Promise<RuntimeCheckV1&Readonly<{commandLine:readonly string[]|undefined}>>{
  const registry=readRuntimePinRegistryV1(root),pin=pinnedRuntimeV1(registry,host),located=locatePinnedRuntimeV1(pin,host);
  const commandLine=running===pin.version?undefined:await readLine();
  const decision=guardDecisionV1({running,pin:pin.version,mode,commandLine:commandLine!==undefined&&commandLine.length>1,located:located!==undefined,launcher:launcher!==undefined,depth:Number(environment[guardDepthV1]??0)||0});
  return Object.freeze({decision,running,version:pin.version,name:pin.name,path:located?.path,launcher,registry,commandLine});
}

/** 📣️ The refusal of a check in the resolved language. */
export function runtimeCheckMessageV1(check:RuntimeCheckV1,root:string,environment:Readonly<Record<string,string|undefined>>,files:readonly string[]):string{
  const key={"refuse-test":"guardTest","refuse-missing":"guardMissing","refuse-loop":"guardLoop"}[check.decision as"refuse-test"];
  const install=process.platform==="win32"?`powershell -NoProfile -ExecutionPolicy Bypass -File "${join(root,installRelative,"🔵️.ps1")}"`:`bash "${join(root,installRelative,"🐚️.sh")}"`;
  return guardMessageV1(check.registry,key,{runtime:check.name,version:check.version,running:check.running,files:files.join(" "),install},environment);
}

/** ▶️ Runs a command as the child of this process with its streams, forwards interrupts and resolves with its exit code (128 plus the signal number when it was killed). */
export function runInheritedV1(command:string,args:readonly string[],environment:Readonly<Record<string,string|undefined>>):Promise<number>{
  return new Promise(done=>{
    const child=spawn(command,[...args],{stdio:"inherit",env:environment as NodeJS.ProcessEnv,windowsHide:true}),forward=(signal:NodeJS.Signals)=>()=>{child.kill(signal);};
    const handlers=((process.platform==="win32"?["SIGINT","SIGTERM","SIGBREAK","SIGHUP"]:["SIGINT","SIGTERM"])as NodeJS.Signals[]).map(signal=>[signal,forward(signal)]as const);
    for(const[signal,handler]of handlers)process.on(signal,handler);
    const settle=(code:number)=>{for(const[signal,handler]of handlers)process.removeListener(signal,handler);done(code);};
    child.once("error",()=>settle(127));
    child.once("close",(code,signal)=>settle(code??(signal===null?1:128+(({SIGINT:2,SIGTERM:15}as Record<string,number>)[signal]??9))));
  });
}

/** 🛡️ Makes the running process the pinned Bun: nothing when it already is; otherwise the exact command line re-runs under the provisioned pinned Bun, whose directory then leads the PATH of everything it starts, or under the `repo` launcher, which provisions it (a script start as a script route, the test runner through `tools exec`), and this process exits with the child's code; a hop of the guard that is still not pinned (the marker is cleared as soon as the pinned Bun runs, so later starts of another Bun are guarded again), an unreadable test command line and a missing launcher are refused with the command to use. */
/** 🏷️ The version of the selected pinned runtime that is running this process, read from the `process.versions` entry named by the pin registry, so the preload names no runtime and loads nothing. */
export function runningVersionV1(root:string,environment:Readonly<Record<string,string|undefined>>):string{
  const registry=readRuntimePinRegistryV1(root);
  return (process.versions as Readonly<Record<string,string|undefined>>)[environment[registry.cache.runtimeEnvironment]||registry.selected]??"";
}

export async function guardRuntimeV1(options:Readonly<{root?:string;running?:string;mode?:"run"|"test";argv?:readonly string[];readLine?:()=>Promise<readonly string[]|undefined>;host?:RuntimePinHostV1;launcher?:string|null;environment?:Readonly<Record<string,string|undefined>>;write?:(text:string)=>void;exit?:(code:number)=>never|void}>={}):Promise<RuntimeCheckV1>{
  const root=options.root??resolve(fileURLToPath(new URL("../../../../../../../",import.meta.url))),environment=options.environment??process.env,argv=options.argv??process.argv,exit=options.exit??((code:number)=>process.exit(code));
  const mode=options.mode??(environment.NODE_ENV==="test"?"test":"run"),check=await checkRuntimeV1(root,options.running??runningVersionV1(root,environment),mode,options.readLine??readCommandLineV1,environment,options.host??systemRuntimePinHostV1(root,environment),options.launcher===undefined?launcherPathV1(environment):options.launcher??undefined);
  if(check.decision==="proceed"){if(options.environment===undefined)delete process.env[guardDepthV1];return check;}
  if(check.decision==="reexec"||check.decision==="delegate"){
    const rest=check.commandLine?.slice(1)??[...process.execArgv,...argv.slice(1)],reexec=check.decision==="reexec";
    exit(await runInheritedV1(reexec?check.path!:check.launcher!,reexec?rest:mode==="test"?["tools","exec","--light",check.name,"--",...rest]:argv.slice(1),{...environment,[guardDepthV1]:"1",...reexec?{PATH:[dirname(check.path!),environment.PATH].filter(Boolean).join(delimiter)}:{}}));
    return check;
  }
  (options.write??((text:string)=>{process.stderr.write(text+"\n");}))(runtimeCheckMessageV1(check,root,environment,argv.slice(1,2)));
  exit(2);
  return check;
}
