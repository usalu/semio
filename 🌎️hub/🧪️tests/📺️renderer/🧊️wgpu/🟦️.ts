import { requireIssuedResourceControlInvocationV1, type ScriptInvocation } from "@semio-tech/process/routing";
import { scriptCompilerAdmissionControlV1 } from "@semio-tech/process/compiler-capacity";
import { type ResourceLeaseControl } from "@semio-tech/process/leases";
import { captureOwnedCommandResultV1 } from "@semio-tech/process/owned-execution";
import { runAdmittedExactCargoLawsV1 } from "@semio-tech/process/cargo-test";
import { configuredExactCargoLawPolicyV1 } from "@semio-tech/process/testing/cargo/exact";
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { readProcessOwnerContextV1 } from "@semio-tech/process/context";
import { findWorkspaceRoot } from "@semio-tech/process/routing";
import { withBuiltLocalHubHostV1, withLocalHubHostTaskV1, startLocalHub, waitForReadiness } from "../../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { runHubCollaborationCli } from "./🤝️collaboration/🟦️.ts";
import { nativeRuntimeDirectory } from "@semio-tech/framework-renderer-wgpu/native-entrypoint/modules";

const repoRoot = findWorkspaceRoot();
const crateName = "semio-framework-os-renderer-wgpu";
type LiveHubPrincipal = { readonly prefix: string; readonly email: string; readonly displayName: string };
async function withLiveHub(invocation:ScriptInvocation,phase:ResourceLeaseControl,principals:readonly LiveHubPrincipal[],body:(env:Record<string,string>,control:ResourceLeaseControl)=>Promise<void>):Promise<string>{
 requireIssuedResourceControlInvocationV1(phase,invocation);const password="correct horse battery staple",env:Record<string,string>={};
 if(process.env.SEMIO_HUB_LIVE_ORIGIN){env.SEMIO_HUB_LIVE_ORIGIN=process.env.SEMIO_HUB_LIVE_ORIGIN;for(const{prefix}of principals)for(const key of[`${prefix}_EMAIL`,`${prefix}_PASSWORD`]){const value=process.env[key];if(!value)throw new Error(`SEMIO_HUB_LIVE_ORIGIN needs ${key}`);env[key]=value;}await body(env,phase);return env.SEMIO_HUB_LIVE_ORIGIN;}
 return withBuiltLocalHubHostV1(invocation,phase,(host,context)=>withLocalHubHostTaskV1(host,context.control,async actual=>{
  for(const{prefix,email,displayName}of principals){const result=await captureOwnedCommandResultV1(actual.binaryPath,["credential","set","--email",email,"--display-name",displayName],actual.workspace,{control:actual.control,env:{...actual.environment,OS_HUB_DATA:actual.dataRoot},input:{version:1,text:password},output:{maximumBytes:65536,maximumLines:512}});if(result.status!==0||result.signal!==null)throw new Error("Credential operator child failed: "+result.stderr);env[`${prefix}_EMAIL`]=email;env[`${prefix}_PASSWORD`]=password;}
  const hub=await startLocalHub(host,[{profileId:"developer",subject:"local-developer-01",displayName:"Local Developer",allowedClientClasses:["native","mcp"]}],{capture:true,credentialSignIn:true});await waitForReadiness(hub,true);env.SEMIO_HUB_LIVE_ORIGIN=`http://127.0.0.1:${hub.port}`;await body(env,actual.control);return env.SEMIO_HUB_LIVE_ORIGIN;
 }));
}
function stageNativeRuntime(variant: string, profile: "prototype" | "release"): string {
  const publisher = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📦️modules/📜️script.ts");
  const published = spawnSync(process.execPath, [publisher, "publish", variant, profile], { cwd: repoRoot, encoding: "utf8" });
  if (published.status !== 0) throw new Error(`native ${variant} runtime publish failed: ${published.stderr}${published.stdout}`);
  return nativeRuntimeDirectory(repoRoot, variant, profile);
}
async function runLiveLaw(invocation:ScriptInvocation,control:ResourceLeaseControl,law:string,env:Record<string,string>):Promise<void>{
 await runAdmittedExactCargoLawsV1({invocation,control,policy:configuredExactCargoLawPolicyV1(),cwd:repoRoot,env:{...process.env,...env},groups:[{package:crateName,target:{kind:"lib"},laws:[law]}]});
}

/** 🏃️ Runs the `HubLiveJourneyCheckScript` command under the caller's roots and invocation. */
export async function runHubLiveJourneyCheckScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments.length) throw new Error("journey-check accepts no arguments");
  const law = "shell::hub_projection_workspace_tests::a_live_hub_signs_in_and_its_spaces_reach_the_retained_workspace";
  const origin = await withLiveHub(context.invocation,scriptCompilerAdmissionControlV1(context.invocation,"renderer:live-hub",0),[{ prefix: "SEMIO_HUB_LIVE", email: "ada@example.org", displayName: "Ada Lovelace" }], (env,control) => runLiveLaw(context.invocation,control,law,env));
  console.log(`journey-check: origin=${origin} law=1 clean`);
}

/** 🏃️ Runs the `HubLiveCreationCheckScript` command under the caller's roots and invocation. */
export async function runHubLiveCreationCheckScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments.length) throw new Error("creation-check accepts no arguments");
  const law = "shell::hub_projection_workspace_tests::a_live_hub_artifact_is_created_through_the_wgpu_creation_door";
  const origin = await withLiveHub(context.invocation,scriptCompilerAdmissionControlV1(context.invocation,"renderer:live-hub",0),[{ prefix: "SEMIO_HUB_LIVE", email: "ada@example.org", displayName: "Ada Lovelace" }], (env,control) => runLiveLaw(context.invocation,control,law,env));
  console.log(`creation-check: origin=${origin} law=1 clean`);
}

/** 🏃️ Runs the `HubLiveCollaborationCheckScript` command under the caller's roots and invocation. */
export async function runHubLiveCollaborationCheckScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments.length) throw new Error("collaboration-check accepts no arguments");
  const variant = "block2d", modules = stageNativeRuntime(variant, "release");
  const law = "shell::hub_projection_workspace_tests::two_live_wgpu_shells_collaborate_on_one_hub_document";
  const origin = await withLiveHub(context.invocation,scriptCompilerAdmissionControlV1(context.invocation,"renderer:live-hub",0),
    [
      { prefix: "SEMIO_HUB_LIVE", email: "ada@example.org", displayName: "Ada Lovelace" },
      { prefix: "SEMIO_HUB_LIVE_PEER", email: "bo@example.org", displayName: "Bo Peep" },
    ],
    (env,control) => runLiveLaw(context.invocation,control,law,{ ...env, SEMIO_PLUGIN: variant, SEMIO_PLUGIN_MODULES: modules }),
  );
  console.log(`collaboration-check: origin=${origin} law=1 clean`);
}

/** 🏃️ Runs the `HubCollaborationAcceptanceScript` command under the caller's roots and invocation. */
export async function runHubCollaborationAcceptanceScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  await runHubCollaborationCli(repoRoot, join(context.root, "🤖️generated/🤝️hub-collaboration"), segments, { nativeModules: () => stageNativeRuntime("block2d", "release") });
}

/** 🏃️ Runs the `NativeGuestJourneyCheckScript` command under the caller's roots and invocation. */
export async function runNativeGuestJourneyCheckScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments.length) throw new Error("native-guest-journey-check accepts no arguments");
  const variant = "block2d", profile = "release", modules = stageNativeRuntime(variant, profile);
  const laws = [
    "a_native_guest_mounts_and_settles_an_authored_edit_without_a_hub",
    "a_native_guest_undoes_its_authored_edit_without_a_hub",
    "a_native_guest_redoes_its_undone_edit_without_a_hub",
    "a_native_guest_copies_and_pastes_its_selection_without_a_hub",
  ];
  for (const law of laws) await runLiveLaw(context.invocation,scriptCompilerAdmissionControlV1(context.invocation,"renderer:live-law",0),`shell::hub_projection_workspace_tests::${law}`, { SEMIO_PLUGIN: variant, SEMIO_PLUGIN_MODULES: modules });
  console.log(`native-guest-journey-check: variant=${variant} profile=${profile} laws=${laws.length} clean`);
}

/** 🏃️ Runs the `TestWorkerScript` command under the caller's roots and invocation. */
export async function runTestWorkerScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments:string[]): Promise<void> {
  if(segments.length)throw new Error("Hub renderer worker accepts no arguments");
  readProcessOwnerContextV1(process.env,process.cwd());
  const {proveHubRendererCommandOwnershipV1}=await import("./🧪️tests/🟦️.ts");
  await proveHubRendererCommandOwnershipV1(context.repoRoot);
}
