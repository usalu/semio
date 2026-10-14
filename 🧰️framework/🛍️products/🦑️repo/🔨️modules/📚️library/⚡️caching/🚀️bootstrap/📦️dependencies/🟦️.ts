import {requireJavascriptDependencyStorageV1,stageJavascriptDependencyInputsV1,publishJavascriptWorkspaceExportsV1,recheckJavascriptDependencyInputsV1,bindJavascriptDependencyDirectoriesV1} from "./💾️storage/🟦️.ts";
import type {RepositoryCacheStoreV1} from "../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {parseCargoPreparationStorageV1,type CargoPreparationStorageV1} from "../../../🗂️workspaces/🦀️cargo/🛠️preparation/📦️storage/🟦️.ts";
import { spawn, spawnSync } from "node:child_process";
import { mkdirSync,lstatSync,readFileSync,writeFileSync,renameSync,rmSync } from "node:fs";
import { dirname, join,relative } from "node:path";
import { checkScriptInvocation,type ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runOwnedCommand} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {type NativeCallerPhaseV1} from "../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import { prepareCargoWorkspaceInvocation,cargoWorkspacePreparationInvocationV1 } from "../../../🗂️workspaces/🦀️cargo/🟦️.ts";
import {cargoPreparationControlV1} from "../../📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import {withPreparedCargoDependencyPairV1} from "../../../🗂️workspaces/🦀️cargo/🛠️preparation/🟦️.ts";
import {fileURLToPath} from "node:url";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";

/** 🏃️ Runs one tool with progress, bounded optional output and cancellation of its process tree. */
export async function runTool(command: string, args: string[], cwd: string, signal: AbortSignal, capture: boolean | "ignore", environment: NodeJS.ProcessEnv, storage:CargoPreparationStorageV1): Promise<string> {
    signal.throwIfAborted();parseCargoPreparationStorageV1(storage);
    if (command === "cargo") prepareCargoWorkspaceInvocation(storage,getWorkspaceRoot(environment),args,cwd,environment);
    return executeTool(command,args,cwd,signal,capture,environment);
}

async function executeTool(command:string,args:string[],cwd:string,signal:AbortSignal,capture:boolean|"ignore",environment:NodeJS.ProcessEnv):Promise<string> {
    signal.throwIfAborted();
    const child = spawn(command, args, { cwd, env: environment, detached: process.platform !== "win32", stdio: ["ignore", capture === "ignore" ? "ignore" : capture ? "pipe" : "inherit", "inherit"], windowsHide: true });
    let output = "", overflow = false;
    let force: ReturnType<typeof setTimeout> | undefined;
    const kill = (signal: NodeJS.Signals): void => {
      if (!child.pid) return;
      if (process.platform === "win32") spawnSync("taskkill", ["/pid", String(child.pid), "/t", "/f"], { stdio: "ignore", windowsHide: true });
      else try { process.kill(-child.pid, signal); } catch {}
    };
    const stop = (): void => {
      kill("SIGTERM");
      force = setTimeout(() => kill("SIGKILL"), 5000);
    };
    child.stdout?.on("data", (chunk: Buffer) => { if (output.length + chunk.length > 1024 * 1024) { overflow = true; kill("SIGKILL"); } else output += chunk.toString(); });
    const progress = setInterval(() => (capture ? console.error : console.log)(`${command} ${args[0]} is still running…`), 10000);
    signal.addEventListener("abort", stop, { once: true });
    if (signal.aborted) stop();
    try {
      const status = await new Promise<number>((accept, reject) => { child.once("error", reject); child.once("close", code => accept(code ?? 1)); });
      signal.throwIfAborted();
      if (overflow) throw new Error(`${command} output exceeds its limit`);
      if (status !== 0) throw new Error(`${command} ${args[0]} failed (${status})${output ? "\n" + output : ""}`);
      return output;
    } finally {
      if (signal.aborted) kill("SIGKILL");
      if (force) clearTimeout(force);
      clearInterval(progress);
      signal.removeEventListener("abort", stop);
    }
}

/** 🦀️ Executes only the owned prepared dependency pair without a public preparation bypass. */
export async function runPreparedCargoDependencyPairV1(root:string,manifest:string,invocation:ScriptInvocation,storage:CargoPreparationStorageV1,runner:NativeDependencyToolRunnerV1,phase:NativeCallerPhaseV1):Promise<void> {
 checkScriptInvocation(invocation);if(phase.leaseControl.signal!==invocation.control.signal||phase.authority.original!==invocation.capabilities)throw Error("Original native pair control required");await phase.advance("cargo:prepared-pair");
 const execute=runner===runNativeDependencyToolV1?runBoundedNativeDependencyToolV1:runner;
 await withPreparedCargoDependencyPairV1(storage,root,manifest,phase.leaseControl,async(args,cwd,signal)=>{await phase.advance("cargo:"+args[0]);await execute("cargo",args,cwd,signal,false,process.env,storage,phase);phase.budget();},async(cwd,signal)=>{await phase.advance("cargo:identity");const result=await execute("cargo",["--version"],cwd,signal,true,process.env,storage,phase);phase.budget();return result;},cargoPreparationControlV1(phase),[fileURLToPath(import.meta.url),fileURLToPath(new URL("./🏗️native/📜️script.ts",import.meta.url))]);
}

/** 🔌️ Receives the original native phase for each actual bounded dependency tool. */
export type NativeDependencyToolRunnerV1=(command:string,args:string[],cwd:string,signal:AbortSignal,capture:boolean|"ignore",environment:Readonly<Record<string,string|undefined>>,storage:CargoPreparationStorageV1,phase:NativeCallerPhaseV1)=>Promise<string>;
/** 🏃️ Runs native dependency children with the exact incoming signal and independent declared transport. */
export async function runNativeDependencyToolV1(command:string,args:string[],cwd:string,signal:AbortSignal,capture:boolean|"ignore",environment:Readonly<Record<string,string|undefined>>,storage:CargoPreparationStorageV1,phase:NativeCallerPhaseV1):Promise<string>{
 if(signal!==phase.leaseControl.signal)throw Error("Original native tool signal required");phase.budget();parseCargoPreparationStorageV1(storage);
 const preparation=command==="cargo"?cargoWorkspacePreparationInvocationV1(storage,phase.authority.repositoryRoot,args,cwd,environment):undefined;
 if(preparation)await runBoundedNativeDependencyToolV1(preparation.command,[...preparation.args],preparation.cwd,signal,false,preparation.environment,storage,phase);
 return runBoundedNativeDependencyToolV1(command,args,cwd,signal,capture,environment,storage,phase);
}

/** 🔗️ Executes already prepared dependency children inside their existing exclusive custody. */
async function runBoundedNativeDependencyToolV1(command:string,args:string[],cwd:string,signal:AbortSignal,capture:boolean|"ignore",environment:Readonly<Record<string,string|undefined>>,storage:CargoPreparationStorageV1,phase:NativeCallerPhaseV1):Promise<string>{
 if(signal!==phase.leaseControl.signal)throw Error("Original native tool signal required");phase.budget();parseCargoPreparationStorageV1(storage);
 await phase.advance(command+":"+args[0]);let output="";
 try{await runOwnedCommand(command,args,cwd,"native-dependencies:"+command,phase.budget(),{...phase.options(environment),stdout:capture===true?"stderr":capture==="ignore"?"ignore":"inherit",onLine:capture===true?line=>{output+=line+"\n";}:undefined});phase.budget();return output;}
 catch(error){signal.throwIfAborted();throw error;}
}

/** 📦️ Runs one original bounded Bun child without lifecycle writers escaping its enforcing installation cwd. */
export async function runBun(args:string[],cwd:string,store:RepositoryCacheStoreV1):Promise<void>{const repository=(store.invocation.capabilities as {workspaceRoot?:unknown}).workspaceRoot;if(typeof repository!=="string")throw Error("Original JavaScript repository required");await requireJavascriptDependencyStorageV1(repository,store);const cache=join(store.root,"bun-cache"),environment={...store.environment,BUN_INSTALL_CACHE_DIR:cache,BUN_INSTALL_GLOBAL_DIR:join(store.root,"bun-global"),BUN_INSTALL_BIN:join(store.root,"bun-bin")};mkdirSync(cache,{recursive:true});process.stdout.write(await captureOwnedCommandV1(process.execPath,[...args,"--ignore-scripts","--backend=hardlink","--cache-dir",cache],cwd,{control:store.control,env:environment}));await store.recheck();}

/** 🔌️ Receives actual issued dependency custody for the complete installer child. */
export type JavascriptDependencyRunner=(args:string[],cwd:string,store:RepositoryCacheStoreV1)=>Promise<void>;

/** 🧭️ Selects only the two authored Bun dependency operations. */
export function javascriptDependencyArgumentsV1(mode:"sync"|"lock"):string[]{if(mode!=="sync"&&mode!=="lock")throw Error("Authored Bun dependency operation required");return["install",mode==="sync"?"--frozen-lockfile":"--lockfile-only"];}

/** 🧾️ Installs only exact Source inputs in a finite immutable generation and then publishes Original package bindings. */
export async function prepareJavascriptDependencies(mode:"sync"|"lock",workspace:string,store:RepositoryCacheStoreV1,runner:JavascriptDependencyRunner=runBun):Promise<void>{
 const repository=(store?.invocation?.capabilities as {workspaceRoot?:unknown}|undefined)?.workspaceRoot;if(typeof repository!=="string")throw Error("Original JavaScript repository required");await requireJavascriptDependencyStorageV1(repository,store);const scope=relative(repository,workspace),plan=await stageJavascriptDependencyInputsV1(repository,store,scope||".",mode),receipt=join(plan.directory,"bindings.json");if(mode==="sync"&&lstatSync(receipt,{throwIfNoEntry:false})?.isFile()){const row=JSON.parse(readFileSync(receipt,"utf8"));if(row.version!==1||row.generation!==plan.generation||JSON.stringify(row.bindings)!==JSON.stringify(plan.bindings))throw Error("Original immutable Bun installation receipt changed");recheckJavascriptDependencyInputsV1(plan.sources);await bindJavascriptDependencyDirectoriesV1(repository,store,scope||".",plan.members,plan.generation);console.log("[DEBUG] Actual immutable Bun dependencies reused "+plan.generation);return;}
 if(readNativeCallerAuthorityV1(store.invocation.capabilities).network.offline)throw Error("Original offline Bun invocation requires an existing immutable dependency generation");await runner(javascriptDependencyArgumentsV1(mode),plan.directory,store);await store.recheck();recheckJavascriptDependencyInputsV1(plan.sources);if(mode==="lock"){const target=join(workspace,"bun.lock"),temporary=target+"."+plan.generation.slice(0,8);try{writeFileSync(temporary,plan.restoreLock(readFileSync(join(plan.directory,"bun.lock"),"utf8")),{flag:"wx"});renameSync(temporary,target);}finally{rmSync(temporary,{force:true});}return;}await publishJavascriptWorkspaceExportsV1(repository,store,scope||".",plan.members,plan.generation);console.log("[DEBUG] Actual bounded Bun dependencies published "+JSON.stringify({generation:plan.generation,root:store.root,bindings:plan.bindings.length}));
}

