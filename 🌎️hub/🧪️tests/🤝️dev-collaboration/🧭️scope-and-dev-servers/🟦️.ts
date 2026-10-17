/** 🧭️ Collaboration e2e scope and dev servers: the external hub handle, the per-run scope with its output directory, the dev script, the shell runtime activation, the serve lane and the per-user dev server scope. */
import { LocalHubHostLedgerV1, localHubHostPolicyV1 } from "../../../🚀️local-bootstrap/🎛️host/🟦️.ts";
import { scriptResourceProcessEnvironmentV1 } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readNativeCallerAuthorityV1 } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {AsyncLocalStorage} from "node:async_hooks";
import { mkdirSync, readFileSync } from "node:fs";
import {spawn,type ChildProcess} from "node:child_process";
import {openSync,fstatSync,writeSync,closeSync} from "node:fs";
import {checkResourceLeaseControl} from "@semio-tech/process/leases";
import { terminateOwnedChildTree } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { join } from "node:path";
import { daemonBudgetOpts, runCmd } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { awaitTcpReady } from "@semio-tech/framework-os-dev/activation/readiness";
import { withLocalHubHostTaskV1, type LocalHubHostV1, type LocalHubHostContextV1 } from "../../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { COLLAB_E2E_DEV_BOOT_BUDGET_MS, COLLAB_E2E_USER1_EMAIL, COLLAB_E2E_USER2_EMAIL, repoRoot } from "../⚙️constants-relay-and-recorder/🟦️.ts";

//#region 🔖️CollabE2e
/** 🌐️ An already-running hub this run joins instead of booting one (`S_COLLAB_HUB_URL`, e.g. the canonical hub): its
 * two humans are provisioned by the hub's operator, so their passwords come from `S_COLLAB_USER1_PASSWORD` /
 * `S_COLLAB_USER2_PASSWORD`. STEP 7 reads the operator's rotating admin-relay capability from the JSON file
 * `S_COLLAB_HUB_ADMIN_CAPABILITY_FILE` (`{capability}`) at the moment it runs, since those sessions are short-lived. */
type CollabExternalHub = { readonly baseUrl: string; readonly adminCapabilityFile: string; readonly passwords: Readonly<Record<string, string>> };

export function collabExternalAdminCapability(hub: CollabExternalHub): string {
  if (hub.adminCapabilityFile === "") return "";
  const parsed = JSON.parse(readFileSync(hub.adminCapabilityFile, "utf8")) as { readonly capability?: unknown };
  return typeof parsed.capability === "string" ? parsed.capability : "";
}

export function collabExternalHub(baseUrl: string | undefined): CollabExternalHub | null {
  if (!baseUrl) return null;
  const user1 = process.env.S_COLLAB_USER1_PASSWORD;
  const user2 = process.env.S_COLLAB_USER2_PASSWORD;
  if (!user1 || !user2) throw new Error("collab e2e: S_COLLAB_HUB_URL needs S_COLLAB_USER1_PASSWORD and S_COLLAB_USER2_PASSWORD");
  return { baseUrl: baseUrl.replace(/\/+$/u, ""), adminCapabilityFile: process.env.S_COLLAB_HUB_ADMIN_CAPABILITY_FILE ?? "", passwords: { [COLLAB_E2E_USER1_EMAIL]: user1, [COLLAB_E2E_USER2_EMAIL]: user2 } };
}

export type CollabScopeV1={readonly host:LocalHubHostV1;readonly context:LocalHubHostContextV1;readonly servers:Set<CollabDevServerV1>;readonly ledger:LocalHubHostLedgerV1;live:boolean;serverBirths:number};
export const collabScopesV1=new AsyncLocalStorage<CollabScopeV1>();
export function collabScopeV1(){const scope=collabScopesV1.getStore();if(!scope)throw Error("Actual retained collaboration Host required");return scope;}
/** 📁️ Resolves diagnostics only within the actual retained Host callback. */
export function collabOutDir():string{const directory=join(collabScopeV1().context.diagnosticsRoot,"collaboration");mkdirSync(directory,{recursive:true});return directory;}

/** 📜️ The `🧑‍💻dev` bundle script both shell steps below drive. */
const COLLAB_E2E_DEV_SCRIPT = "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts";

/** 🏗️ Stages the `s` react dev runtime ONCE for both users. Activation is keyed by
 * variant+renderer+profile, not by user, so the two shells share one staged runtime and one activation
 * receipt; running it per user would stage the same tree twice and, worse, let the second run swap the
 * modules out from under the first shell's open page. It is a separate step from `collabStartUserDevServer`
 * because the Nx `dev-…` target is a WATCH: it re-activates on any peer's source write, which under this
 * repo's concurrent fleet means the staged tree moves mid-scenario. Activate once, serve detached. */
export function collabActivateShellRuntime(): void {
  runCmd("bun", ["repo", "run", "@semio-tech/framework-os-dev:activate-s-react-prototype"], {
    cwd: repoRoot,
    ...daemonBudgetOpts({ SEMIO_PLUGIN: "s", SEMIO_RENDERER: "react", SEMIO_BUILD_MODE: "prototype" }),
  });
}

/** 🛤️ The staged lane both shells serve: `S_COLLAB_LANE=release` serves the release components a trusted catalog was
 * published from (the same bytes the hub's closed actors carry), anything else the dev lane. */
function collabServeLane(): "dev" | "release" {
  return process.env.S_COLLAB_LANE === "release" ? "release" : "dev";
}

/** ▶️ Spawns one user's `s` react dev server and waits for its port to accept connections. `serve`, not
 * `dev`: the bundle script has no `dev` command at all (`prepare|activate|serve|…`), so the harness's
 * previous `script.ts dev` spawn exited 1 with `unknown command "dev"` before a single byte was served,
 * which is what made every step report "blocked — shells did not boot". `S_HUB_URL` is baked into this
 * server's bundle at Vite `define`-time, so it must be in the SERVE process's env, not the activation's. */
declare const collabDevServerBrand:unique symbol;
export type CollabDevServerV1=Readonly<{readonly[collabDevServerBrand]:true}>;
type CollabDevServerScopeV1={readonly owner:CollabScopeV1;readonly child:ChildProcess;readonly terminal:Promise<void>;readonly logPath:string;readonly descriptor:number;readonly dev:number;readonly ino:number;failure?:Readonly<{error:unknown}>;readonly cleanup:unknown[];terminationRequested:boolean;closed:boolean;stop?:Promise<void>;timer?:ReturnType<typeof setInterval>;deadline?:ReturnType<typeof setTimeout>;abort:()=>void;expectedStop:boolean};
const collabDevServersV1=new WeakMap<CollabDevServerV1,CollabDevServerScopeV1>();
export async function finishCollabDevServerV1(token:CollabDevServerV1):Promise<void>{const scope=collabDevServersV1.get(token);if(!scope)throw Error("Actual collaboration server required");if(scope.stop)return scope.stop;scope.expectedStop=true;scope.stop=(async()=>{const failures:unknown[]=[];try{if(!scope.closed)terminateOwnedChildTree(scope.child);}catch(error){failures.push(error);}await scope.terminal;if(scope.failure)failures.unshift(scope.failure.error);failures.push(...scope.cleanup);scope.owner.servers.delete(token);if(failures.length===1)throw failures[0];if(failures.length)throw new AggregateError(failures,"Collaboration server terminal and cleanup failed");})();void scope.stop.catch(()=>{});return scope.stop;}
/** ▶️ Owns only the two fixed collaboration serve children within their actual Host/Diagnostics lifetime. */
export async function collabStartUserDevServer(opts:{readonly port:number;readonly hubUrl:string;readonly user:string}):Promise<CollabDevServerV1>{
 const owner=collabScopeV1(),context=owner.context,index=opts.user===COLLAB_E2E_USER1_EMAIL?1:opts.user===COLLAB_E2E_USER2_EMAIL?2:0;if(!owner.live||!index||owner.servers.size>=localHubHostPolicyV1().maximumDevelopmentServers||owner.serverBirths>=localHubHostPolicyV1().maximumDevelopmentServerBirths||!Number.isSafeInteger(opts.port)||opts.port<1||opts.port>65535)throw Error("Finite actual collaboration server request required");owner.serverBirths++;
 return owner.ledger.run(()=>withLocalHubHostTaskV1(owner.host,context.control,async actual=>{
  const declaration=readNativeCallerAuthorityV1(actual.invocation.capabilities),dataDir=join(collabOutDir(),`user${index}`),logPath=join(collabOutDir(),`user${index}-dev.log`),fd=openSync(logPath,"wx",0o600),identity=fstatSync(fd),args=[join(repoRoot,COLLAB_E2E_DEV_SCRIPT),"serve","s","react",collabServeLane()];let child:ChildProcess;try{checkResourceLeaseControl(actual.control);child=spawn(process.execPath,args,{cwd:join(repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"),env:scriptResourceProcessEnvironmentV1(actual.control,{...actual.environment,SEMIO_PLUGIN:"s",SEMIO_RENDERER:"react",SEMIO_VITE_HMR:"0",S_OS_PORT:String(opts.port),S_HUB_URL:opts.hubUrl,S_DATA_DIR:dataDir,S_LOCAL_ONLY:"1"}),stdio:["ignore","pipe","pipe"],detached:process.platform!=="win32",shell:false});}catch(error){try{closeSync(fd);}catch(close){throw new AggregateError([error,close],"Collaboration server birth and descriptor close failed");}throw error;}
  let done!:()=>void;const token=Object.freeze({})as CollabDevServerV1,terminal=new Promise<void>(accept=>done=accept),scope:CollabDevServerScopeV1={owner,child,terminal,logPath,descriptor:fd,dev:identity.dev,ino:identity.ino,closed:false,cleanup:[],terminationRequested:false,abort:()=>{},expectedStop:false},fail=(error:unknown):void=>{scope.failure??={error};if(scope.terminationRequested)return;scope.terminationRequested=true;try{terminateOwnedChildTree(child);}catch(close){scope.cleanup.push(close);}};
  child.once("error",fail);child.once("close",(status,signal)=>{scope.closed=true;clearInterval(scope.timer);clearTimeout(scope.deadline);actual.control.signal.removeEventListener("abort",scope.abort);if(!scope.expectedStop&&(status!==0||signal!==null))scope.failure??={error:Error("Collaboration server failed: "+String(status??signal))};try{const current=fstatSync(fd);if(current.dev!==identity.dev||current.ino!==identity.ino)throw Error("Exact collaboration log descriptor required");}catch(error){scope.cleanup.push(error);}try{closeSync(fd);}catch(error){scope.cleanup.push(error);}done();});
  owner.servers.add(token);collabDevServersV1.set(token,scope);scope.abort=()=>fail(actual.control.signal.reason);actual.control.signal.addEventListener("abort",scope.abort,{once:true});scope.timer=setInterval(()=>{try{checkResourceLeaseControl(actual.control);}catch(error){fail(error);}},50);scope.timer.unref();scope.deadline=setTimeout(()=>fail(Error("Authored collaboration child deadline exceeded")),declaration.child.maximumElapsedMilliseconds);scope.deadline.unref();let bytes=0,lines=0;
  const output=(chunk:Buffer):void=>{if(scope.failure)return;try{checkResourceLeaseControl(actual.control);bytes+=chunk.length;for(const byte of chunk)if(byte===10)lines++;if(bytes>declaration.transport.maximumBytes||lines>declaration.transport.maximumLines)throw Error("Collaboration server output capacity exceeded");let written=0;while(written<chunk.length){const count=writeSync(fd,chunk,written,chunk.length-written);if(count<1)throw Error("Collaboration log short write");written+=count;}}catch(error){fail(error);}};child.stdout?.on("data",output);child.stderr?.on("data",output);
  const outcome=await awaitTcpReady("127.0.0.1",opts.port,{deadlineMs:COLLAB_E2E_DEV_BOOT_BUDGET_MS,intervalMs:500,isDead:()=>{checkResourceLeaseControl(actual.control);return scope.closed;}});checkResourceLeaseControl(actual.control);if(scope.failure)throw scope.failure.error;if(outcome!=="ready")throw Error(`Collaboration server ${opts.user} did not become ready: ${outcome}`);return token;
 }));
}
//#endregion 🔖️CollabE2e
