
import { receiveScriptProcessInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { join } from "node:path";
import { runOwnedCommand } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { cmdBudgetMs } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runHubLiveJourneyCheckScript, runHubLiveCreationCheckScript, runHubLiveCollaborationCheckScript, runHubCollaborationAcceptanceScript, runNativeGuestJourneyCheckScript, runTestWorkerScript } from "./🟦️.ts";

/** 🤝️ Drives the wgpu shell's whole hub lane against a live hub through its own native
 * transport: `/hub` → add connection → credential sign-in → create space → the space in the retained
 * workspace tree (en + de) → open space → sign out, as one principal of {@link withLiveHub}'s hub. */
class HubLiveJourneyCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runHubLiveJourneyCheckScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🌱️ The wgpu shell's artifact-creation door against a live hub: sign in, create and open a space,
 * the door loads the space's selected current catalog, the first kind is created and named, and the
 * creation reaches `Ready` with a hub-minted artifact id — as one principal of {@link withLiveHub}'s hub,
 * which must carry a ready trusted catalog. */
class HubLiveCreationCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runHubLiveCreationCheckScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
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
    await runHubLiveCollaborationCheckScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🤝️ Two humans — and a delegated AI agent — collaborate on ONE hub document through the wgpu shells: one journey per
 * run (`--journey wasm32|wasm32-react|wasm32-native|native-react|native-react-cursors|cursors|agent-pixels --hub <url>
 * [--serve <url>] [--react-serve <url>] [--locale en|de]`), one acceptance record `<check>-<locale>`; an omitted serve is started
 * and stopped by the run itself, native journeys mount the staged block2d release runtime. Humans only from
 * `SEMIO_TWO_HUMAN_USER{1,2}_{EMAIL,PASSWORD}`. See {@link runHubCollaborationCli}. */
class HubCollaborationAcceptanceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runHubCollaborationAcceptanceScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
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
    await runNativeGuestJourneyCheckScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🧪️ Owns the complete declaring command ownership check. */
class TestScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    if(segments.length)throw new Error("Hub renderer test accepts no arguments");
    await runOwnedCommand(process.execPath,[join(this.root,"📜️script.ts"),"test-worker"],this.root,"tool:owner",cmdBudgetMs(),{env:process.env});
  }
}
/** 🔎️ Checks the closed command allocation and actually imports both declaring modules. */
class TestWorkerScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runTestWorkerScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
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
