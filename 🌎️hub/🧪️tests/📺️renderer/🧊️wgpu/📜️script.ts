#!/usr/bin/env bun
import {requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {type ResourceLeaseControl} from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {captureOwnedCommandResultV1} from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts";
import { runAdmittedExactCargoLawsV1 } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import {configuredExactCargoLawPolicyV1} from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { spawnSync } from "node:child_process";
import { existsSync, mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { runOwnedCommand } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { cmdBudgetMs } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { readProcessOwnerContextV1 } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📋️context/🟦️.ts";
import { findWorkspaceRoot } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { withBuiltLocalHubHostV1,withLocalHubHostTaskV1,startLocalHub,waitForReadiness } from "../../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { runHubCollaborationCli } from "./🤝️collaboration/🟦️.ts";
import { nativeRuntimeDirectory } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/⌨️native-entrypoint/📦️modules/🟦️.ts";

const repoRoot = findWorkspaceRoot();
const crateName = "semio-framework-os-renderer-wgpu";
/** 🔑️ One hub principal a live law signs in as, and the environment prefix that names it
 * (`<prefix>_EMAIL`, `<prefix>_PASSWORD`). */
type LiveHubPrincipal = { readonly prefix: string; readonly email: string; readonly displayName: string };

/** 🌍️ Runs `body` with a live credential-sign-in hub: the one named by `SEMIO_HUB_LIVE_ORIGIN`
 * (every principal's `<prefix>_EMAIL`/`_PASSWORD` must then be set too), or a fresh hub booted from the
 * staged `os-hub:build-dev` binary with each principal provisioned through `os-hub credential set`,
 * torn down and its data root deleted afterwards (the `os-hub:live-sign-in-check` recipe). */
async function withLiveHub(invocation:ScriptInvocation,phase:ResourceLeaseControl,principals:readonly LiveHubPrincipal[],body:(env:Record<string,string>,control:ResourceLeaseControl)=>Promise<void>):Promise<string>{
 requireIssuedResourceControlInvocationV1(phase,invocation);const password="correct horse battery staple",env:Record<string,string>={};
 if(process.env.SEMIO_HUB_LIVE_ORIGIN){env.SEMIO_HUB_LIVE_ORIGIN=process.env.SEMIO_HUB_LIVE_ORIGIN;for(const{prefix}of principals)for(const key of[`${prefix}_EMAIL`,`${prefix}_PASSWORD`]){const value=process.env[key];if(!value)throw Error(`SEMIO_HUB_LIVE_ORIGIN needs ${key}`);env[key]=value;}await body(env,phase);return env.SEMIO_HUB_LIVE_ORIGIN;}
 return withBuiltLocalHubHostV1(invocation,phase,(host,context)=>withLocalHubHostTaskV1(host,context.control,async actual=>{
  for(const{prefix,email,displayName}of principals){const result=await captureOwnedCommandResultV1(actual.binaryPath,["credential","set","--email",email,"--display-name",displayName],actual.workspace,{control:actual.control,env:{...actual.environment,OS_HUB_DATA:actual.dataRoot},input:{version:1,text:password},output:{maximumBytes:65536,maximumLines:512}});if(result.status!==0||result.signal!==null)throw Error("Credential operator child failed: "+result.stderr);env[`${prefix}_EMAIL`]=email;env[`${prefix}_PASSWORD`]=password;}
  const hub=await startLocalHub(host,[{profileId:"developer",subject:"local-developer-01",displayName:"Local Developer",allowedClientClasses:["native","mcp"]}],{capture:true,credentialSignIn:true});await waitForReadiness(hub,true);env.SEMIO_HUB_LIVE_ORIGIN=`http://127.0.0.1:${hub.port}`;await body(env,actual.control);return env.SEMIO_HUB_LIVE_ORIGIN;
 }));
}

/** 📦️ Stages a completed plugin release component as the native runtime (the `native-entrypoint` publish verb, no
 * compilation; release because the 88 MB dev block2d component exceeds the 64 MiB execution-target component bound) and answers
 * the directory the native live laws mount it from (`SEMIO_PLUGIN_MODULES`). */
function stageNativeRuntime(variant: string, profile: "prototype" | "release"): string {
  const publisher = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📦️modules/📜️script.ts");
  const published = spawnSync(process.execPath, [publisher, "publish", variant, profile], { cwd: repoRoot, encoding: "utf8" });
  if (published.status !== 0) throw new Error(`native ${variant} runtime publish failed: ${published.stderr}${published.stdout}`);
  return nativeRuntimeDirectory(repoRoot, variant, profile);
}

/** 🧪️ Runs one ignored live law of this crate, exactly, under the crate's exhaustive budget. */
async function runLiveLaw(invocation:ScriptInvocation,control:ResourceLeaseControl,law:string,env:Record<string,string>):Promise<void>{
 await runAdmittedExactCargoLawsV1({invocation,control,policy:configuredExactCargoLawPolicyV1(),cwd:repoRoot,env:{...process.env,...env},groups:[{package:crateName,target:{kind:"lib"},laws:[law]}]});
}

/** 🤝️ Drives the wgpu shell's whole hub lane against a live hub through its own native
 * transport: `/hub` → add connection → credential sign-in → create space → the space in the retained
 * workspace tree (en + de) → open space → sign out, as one principal of {@link withLiveHub}'s hub. */
class HubLiveJourneyCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("journey-check accepts no arguments");
    const law = "shell::hub_projection_workspace_tests::a_live_hub_signs_in_and_its_spaces_reach_the_retained_workspace";
    const origin = await withLiveHub(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"renderer:live-hub",0),[{ prefix: "SEMIO_HUB_LIVE", email: "ada@example.org", displayName: "Ada Lovelace" }], (env,control) => runLiveLaw(this.invocation,control,law,env));
    console.log(`journey-check: origin=${origin} law=1 clean`);
  }
}

/** 🌱️ The wgpu shell's artifact-creation door against a live hub: sign in, create and open a space,
 * the door loads the space's selected current catalog, the first kind is created and named, and the
 * creation reaches `Ready` with a hub-minted artifact id — as one principal of {@link withLiveHub}'s hub,
 * which must carry a ready trusted catalog. */
class HubLiveCreationCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("creation-check accepts no arguments");
    const law = "shell::hub_projection_workspace_tests::a_live_hub_artifact_is_created_through_the_wgpu_creation_door";
    const origin = await withLiveHub(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"renderer:live-hub",0),[{ prefix: "SEMIO_HUB_LIVE", email: "ada@example.org", displayName: "Ada Lovelace" }], (env,control) => runLiveLaw(this.invocation,control,law,env));
    console.log(`creation-check: origin=${origin} law=1 clean`);
  }
}

/** 👥️ Two native wgpu shells as two hub users on one block2d hub document: sign-in, shared
 * space, roster, the guest mounted natively through the `os.open-artifact` relay, the document socket,
 * presence, each actor's edit crossing to the other, per-actor undo and a severed-then-healed network
 * for the second actor. Stages the completed block2d release component as the native runtime first
 * (the `native-entrypoint` publish verb, no compilation; release because the 88 MB dev component
 * exceeds the 64 MiB execution-target component bound), then runs the ledgered live law against
 * {@link withLiveHub}'s hub with two principals. */
class HubLiveCollaborationCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("collaboration-check accepts no arguments");
    const variant = "block2d", modules = stageNativeRuntime(variant, "release");
    const law = "shell::hub_projection_workspace_tests::two_live_wgpu_shells_collaborate_on_one_hub_document";
    const origin = await withLiveHub(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"renderer:live-hub",0),
      [
        { prefix: "SEMIO_HUB_LIVE", email: "ada@example.org", displayName: "Ada Lovelace" },
        { prefix: "SEMIO_HUB_LIVE_PEER", email: "bo@example.org", displayName: "Bo Peep" },
      ],
      (env,control) => runLiveLaw(this.invocation,control,law,{ ...env, SEMIO_PLUGIN: variant, SEMIO_PLUGIN_MODULES: modules }),
    );
    console.log(`collaboration-check: origin=${origin} law=1 clean`);
  }
}

/** 🤝️ Two humans — and a delegated AI agent — collaborate on ONE hub document through the wgpu shells: one journey per
 * run (`--journey wasm32|wasm32-react|wasm32-native|native-react|native-react-cursors|cursors|agent-pixels --hub <url>
 * [--serve <url>] [--react-serve <url>] [--locale en|de]`), one acceptance record `<check>-<locale>`; an omitted serve is started
 * and stopped by the run itself, native journeys mount the staged block2d release runtime. Humans only from
 * `SEMIO_TWO_HUMAN_USER{1,2}_{EMAIL,PASSWORD}`. See {@link runHubCollaborationCli}. */
class HubCollaborationAcceptanceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runHubCollaborationCli(repoRoot, join(import.meta.dir, "🤖️generated/🤝️hub-collaboration"), segments, { nativeModules: () => stageNativeRuntime("block2d", "release") });
  }
}

/** ⏯️ One native wgpu shell mounts the staged block2d guest on the native kernel thread with no
 * hub: every surface it opens is admitted, an authored verb settles and lands in the guest's ledger once,
 * undo reverts it, redo applies it again, and the selection and clipboard verbs settle (four laws; undo,
 * redo and select-all drive live framework reserved tool jobs). Stages the completed block2d release
 * component as the native runtime first (the `native-entrypoint` publish verb, no compilation), then runs
 * the laws against the shared `🧫️fixtures/⏯️native-guest-journey` fixture. */
class NativeGuestJourneyCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("native-guest-journey-check accepts no arguments");
    const variant = "block2d", profile = "release", modules = stageNativeRuntime(variant, profile);
    const laws = [
      "a_native_guest_mounts_and_settles_an_authored_edit_without_a_hub",
      "a_native_guest_undoes_its_authored_edit_without_a_hub",
      "a_native_guest_redoes_its_undone_edit_without_a_hub",
      "a_native_guest_copies_and_pastes_its_selection_without_a_hub",
    ];
    for (const law of laws) await runLiveLaw(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"renderer:live-law",0),`shell::hub_projection_workspace_tests::${law}`, { SEMIO_PLUGIN: variant, SEMIO_PLUGIN_MODULES: modules });
    console.log(`native-guest-journey-check: variant=${variant} profile=${profile} laws=${laws.length} clean`);
  }
}

/** 🧪️ Owns the complete declaring command ownership check. */
class TestScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    if(segments.length)throw Error("Hub renderer test accepts no arguments");
    await runOwnedCommand(process.execPath,[join(this.root,"📜️script.ts"),"test-worker"],this.root,"tool:owner",cmdBudgetMs(),{env:process.env});
  }
}
/** 🔎️ Checks the closed command allocation and actually imports both declaring modules. */
class TestWorkerScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    if(segments.length)throw Error("Hub renderer worker accepts no arguments");
    readProcessOwnerContextV1(process.env,process.cwd());
    const {proveHubRendererCommandOwnershipV1}=await import("./🧪️tests/🟦️.ts");
    await proveHubRendererCommandOwnershipV1(this.repoRoot);
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dir)
  .register("journey-check", HubLiveJourneyCheckScript)
  .register("creation-check", HubLiveCreationCheckScript)
  .register("collaboration-check", HubLiveCollaborationCheckScript)
  .register("collaboration-acceptance", HubCollaborationAcceptanceScript)
  .register("native-guest-journey-check", NativeGuestJourneyCheckScript)
  .register("test", TestScript)
  .register("test-worker", TestWorkerScript)).run(process.argv.slice(2), original));
