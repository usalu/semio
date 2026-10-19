#!/usr/bin/env bun
/** 🎨️ `@semio-tech/framework-renderer-react` task router. */

import { createIssuedResourceControlV1 } from "@semio-tech/framework/process/routing";
import { typecheckAdmittedProjectV1 } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🟦️typescript/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../../../../📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../../../../📦️packages/🟦️typescript/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { runVitest } from "@semio-tech/framework/process/vitest";
import { BundleScript, ScriptRouter } from "../../../../../../../../../📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../../../📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { SceneShadingPixelCheckScript } from "@semio-tech/framework/os/modules/renderer/engine/oracles/world3d-scene-shading/script";
import { directoryHomeBootstrapOracle, directoryInviteCapabilityOracle, hubAuthContractOracle, scopedPresenceOracle } from "../../../../🔮️oracles/📇️directory-and-hub-contracts/🟦️.ts";
import { interactionSelectionSetOracle, selectionMergeVocabularyOracle, world3dPointerGestureOracle } from "../../../../🔮️oracles/🖱️world3d-gestures-and-selection/🟦️.ts";
import { surfaceHostRetentionOracle } from "../../../../🔮️oracles/🪪️surface-host-retention/🟦️.ts";

/** 👥️ Verifies canonical viewport presence on actual mounted generic Canvas2d and World3d hosts. */
class ViewportPresenceCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("viewport-presence-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    await runVitest(this.root, ["../../../../🧱️elements/👕️canvas-presence/🧪️tests/✏️text-carets/🟦️.tsx", "../../../../🧱️elements/👕️canvas-presence/🧪️tests/🔬️unit/🟦️.ts", "../../../../🧱️elements/📐️Canvas2dHost/🧪️tests/👕️peer-presence/🟦️.tsx", "../../../../🧪️tests/🖱️world3d-interaction/🟦️.tsx", "--silent=false", "--reporter=verbose"], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}
/** 📬️ Executes original diagnostic command identity, replay, progress and before-ACK terminal laws. */
class OperationDiagnosticHostCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("operation-diagnostic-host-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    await runVitest(this.root, ["../../../../🧱️elements/🔌️PluginRuntime/🟦️.tsx", "--silent=false", "--reporter=verbose", "--testNamePattern=typed-operation diagnostic|routes a foreign typed-operation|bounds parked typed-operation"], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

/** 🧾️ Executes the exact-u64 completion and causal publication interleaving laws. */
class CausalInputPublicationCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("causal-input-publication-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    await runVitest(this.root, ["../../../../🧪️tests/⌨️causal-input-publication/🟦️.ts", "--silent=false", "--reporter=verbose"], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

/** 🎬️ Executes exact owner/Store/target/action admission and mounted Table/Tree recovery laws. */
class RowActionAdmissionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("row-action-admission-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    await runVitest(this.root, ["../../../../🧪️tests/🎬️row-action-admission/🟦️.tsx", "--silent=false", "--reporter=verbose"], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

/** 📍 Executes exact document-scope and newly-created session handoff laws. */
class DocumentOpeningScopeCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("document-opening-scope-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    await runVitest(this.root, ["../../../../🧪️tests/🚪️opening/🟦️.ts", "--silent=false", "--reporter=verbose"], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

/** 🔗️ Executes the registered AgentBridge codec and inference-state laws. */
class AgentBridgeCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("agent-bridge-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    process.env.SEMIO_INCLUDE_AGENT_BRIDGE = "1";
    await runVitest(this.root, ["--run", "--silent=false"], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

class TypecheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await typecheckAdmittedProjectV1({ workspace: this.repoRoot, invocation: this.invocation, control:createIssuedResourceControlV1(this.invocation,"typecheck:entry",0), cwd: this.root, project: "tsconfig.json", owner: "renderer-react:typescript", args: segments, timeoutMs: 120_000 });
  }
}

/** 🎥️ Executes the typed InteractionState tutorial capture, diff, and playback laws. */
class TutorialInteractionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("tutorial-interaction-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    await runVitest(
      this.root,
      [
        "../../../../🧪️tests/🗃️shell-store-reducer/🟦️.ts",
        "--silent=false",
        "--reporter=verbose",
        "--testNamePattern=interaction recording|decodes the bounded actor interaction capture|captures the observed typed interaction|plays full and sparse typed selections|projects tutorial selection playback|records comma-bearing selection|APPLY_TUTORIAL_UI_SNAPSHOT restores",
      ],
      "../../🧪️tests/🎚️config/🟦️.ts",
    );
  }
}

/** 🌊️ Executes the shared Flow browser runtime and mounted React session-retirement laws. */
class FlowBrowserRuntimeCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("flow-browser-runtime-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    await runVitest(
      this.root,
      [
        "../../../../🧪️tests/🧶️node-graph-hosts/🟦️.ts",
        "--silent=false",
        "--reporter=verbose",
        "--testNamePattern=retains one shared Flow browser runtime|retires a graph host unmounted before its open reply",
      ],
      "../../🧪️tests/🎚️config/🟦️.ts",
    );
  }
}

/** 🌱️ Executes the retained artifact-creation progress, cancellation and accessibility laws. */
class ArtifactCreationProgressCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("artifact-creation-progress-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    await runVitest(
      this.root,
      ["../../../../🧪️tests/🔬️artifact-creation-ready-opening/🟦️.ts", "--silent=false", "--reporter=verbose"],
      "../../🧪️tests/🎚️config/🟦️.ts",
    );
    process.env.SEMIO_INCLUDE_BACKBONE_WORKER = "1";
    await runVitest(
      this.root,
      [join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"), "--silent=false", "--reporter=verbose", "--testNamePattern=space artifact creation owner"],
      "../../🧪️tests/🎚️config/🟦️.ts",
    );
  }
}
class DirectoryHomeBootstrapCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("directory-home-bootstrap-check accepts no arguments");
    console.log(`directory-home-bootstrap-oracle: checks=${directoryHomeBootstrapOracle(this.repoRoot)} clean`);
    process.env.SEMIO_TEST_LEVEL = "long";
    await runVitest(this.root, ["../../../../🧪️tests/📇️directory-home-bootstrap/🟦️.tsx"], "../../🧪️tests/🎚️config/🟦️.ts");
    await runVitest(this.root, ["../../../../🧱️elements/🔌️PluginRuntime/🟦️.tsx", "--testNamePattern=validates fixed result page authority and preserves document and download effects|hands one completion its own effects"], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}
/** 🔐️ Proves the browser-facing hub sign-in contract and the end-user spaces surface: the fixture
 * schemas, the closed refusal tables, the local-only connection book, and both panes under a fake
 * hub transport speaking `📓️au1-hub-auth-sessions-and-rate-limit.md` §1's real wire. */
class HubSignInSpacesCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("hub-sign-in-spaces-check accepts no arguments");
    console.log(`hub-auth-contract-oracle: checks=${hubAuthContractOracle(this.repoRoot)} clean`);
    process.env.SEMIO_TEST_LEVEL = "long";
    await runVitest(
      this.root,
      ["../../../../🧱️elements/🔐️HubSignIn/🧪️tests/🧩️component/🟦️.tsx", "../../../../🧱️elements/🏘️SpaceBrowser/🧪️tests/🧩️component/🟦️.tsx", "../../../../🧱️elements/🤖️AgentDelegations/🧪️tests/🧩️component/🟦️.tsx", "--silent=false", "--reporter=verbose"],
      "../../🧪️tests/🎚️config/🟦️.ts",
    );
  }
}
class DirectoryInviteCapabilityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("directory-invite-capability-check accepts no arguments");
    console.log(`directory-invite-capability-oracle: checks=${directoryInviteCapabilityOracle(this.repoRoot)} clean`);
    process.env.SEMIO_TEST_LEVEL = "long";
    await runVitest(this.root, ["../../../../🧪️tests/🏛️space-administration/🟦️.tsx"], "../../🧪️tests/🎚️config/🟦️.ts");
    process.env.SEMIO_INCLUDE_BACKBONE_WORKER = "1";
    await runVitest(this.root, [join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"), "--testNamePattern=backbone-worker space administration"], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}
class ScopedPresenceCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("scoped-presence-check accepts no arguments");
    console.log(`scoped-presence-oracle: checks=${scopedPresenceOracle(this.repoRoot)} clean`);
    process.env.SEMIO_TEST_LEVEL = "long";
    await runVitest(this.root, ["../../../../🧪️tests/👥️scoped-presence/🟦️.tsx"], "../../🧪️tests/🎚️config/🟦️.ts");
    process.env.SEMIO_INCLUDE_BACKBONE_WORKER = "1";
    await runVitest(this.root, [join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"), "--testNamePattern=backbone-worker scope-safe presence"], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}
/** 🖱️ Executes the mounted `World3dHost` gesture laws next to their language-neutral Node twin. */
/** 🔀️ Executes the surface-role/mode switching laws over the language-neutral switching fixture. */
class SurfaceSwitchCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("surface-switch-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    await runVitest(this.root, ["../../../../🧪️tests/🔀️surface-switch/🟦️.ts", "--silent=false", "--reporter=verbose"], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

/** 🎯️ Executes the Input Causality Ledger laws over the pure `ShellHost/input-ledger` module. */
class InputLedgerCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("input-ledger-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    await runVitest(this.root, ["../../../../🧪️tests/🎯️input-ledger/🟦️.ts", "--silent=false", "--reporter=verbose"], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

/** ⌨️ Executes the window-scope laws over the language-neutral dock-seed/chord-owner fixture. */
class WindowScopeCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("window-scope-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    await runVitest(this.root, ["../../../../🧪️tests/⌨️window-scope/🟦️.ts", "--silent=false", "--reporter=verbose"], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

/** 📌️ Runs the view-state carriage laws: contributions are not a view-state field, and `panelJson`
 * is the one long field, bounded at the schema capacity. */
class ViewStateCarriageCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("view-state-carriage-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    await runVitest(this.root, ["../../../../🧪️tests/📌️view-state-carriage/🟦️.ts", "--silent=false", "--reporter=verbose"], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

class World3dInteractionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("world3d-interaction-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    await runVitest(this.root, ["../../../../🧪️tests/🖱️world3d-interaction/🟦️.tsx", "../../../../🧪️tests/🎯️world3d-pick-bounds/🟦️.ts", "--silent=false", "--reporter=verbose"], "../../🧪️tests/🎚️config/🟦️.ts");
    console.log(`world3d-pointer-gesture-oracle: checks=${world3dPointerGestureOracle(this.repoRoot)} clean`);
    console.log(`interaction-selection-set-oracle: checks=${interactionSelectionSetOracle(this.repoRoot)} clean`);
    console.log(`selection-merge-vocabulary-oracle: checks=${selectionMergeVocabularyOracle(this.repoRoot)} clean`);
  }
}

/** 📏️ Executes the mounted World3d modelling layer (annotations, heatmap legend, pick filter, section plane, highlight tokens). */
class World3dModellingCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("world3d-modelling-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    await runVitest(this.root, ["../../../../🧱️elements/🌐️World3dHost/🧪️tests/📏️modelling/🟦️.tsx", "--silent=false", "--reporter=verbose"], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}
/** 🪪️ Runs the retained-surface-host laws: the framework-free twin over the language-neutral fixture,
 * then the React half that witnesses DOM identity and the single attach. */
class SurfaceRetentionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("surface-retention-check accepts no arguments");
    console.log(`surface-host-retention-oracle: checks=${surfaceHostRetentionOracle(this.repoRoot)} clean`);
    process.env.SEMIO_TEST_LEVEL = "long";
    await runVitest(this.root, ["../../../../🧱️elements/🗣️Interpreter/🟦️.tsx", "../../../../🧪️tests/🚧️hidden-tab-surfaces/🟦️.ts", "--silent=false", "--reporter=verbose", "-t", "retained surface host|sibling reconciliation keys"], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

//#region 🔖️LintScript
const REGION_BALANCE_FILES = ["../../🟦️.tsx"] as const;

/** 🧭️Counts unmatched `//#region` / `//#endregion` markers per file — a typo'd region silently corrupts the file's canonical structure. */
function collectRegionBalanceViolations(root: string): string[] {
  const violations: string[] = [];
  for (const name of REGION_BALANCE_FILES) {
    const text = readFileSync(join(root, name), "utf8");
    const opens = (text.match(/#region\b/g) ?? []).length;
    const closes = (text.match(/#endregion\b/g) ?? []).length;
    if (opens !== closes) {
      violations.push(`${name}: ${opens} #region marker(s) vs ${closes} #endregion marker(s)`);
    }
  }
  return violations;
}

/** 🧭️Every host registered in `COMPONENT_SCENE_HOSTS` must have exactly one `export function XxxHost(...: ComponentSceneHostProps)` in `🟦️.tsx` — the contract the host registry table dispatches against. */
function collectHostSignatureViolations(root: string): string[] {
  const violations: string[] = [];
  const text = readFileSync(join(root, "../../🟦️.tsx"), "utf8");
  const registryNames = [...text.matchAll(/lazyHost\(\(\) => Promise\.resolve\(\{ ([A-Z][A-Za-z0-9]*Host) \}\), "\1"\)/g)].map((m) => m[1]!);
  const hostExportCounts = new Map<string, number>();
  for (const m of text.matchAll(/^export function ([A-Z][A-Za-z0-9]*Host)\([^)]*: ComponentSceneHostProps\)/gm)) {
    hostExportCounts.set(m[1]!, (hostExportCounts.get(m[1]!) ?? 0) + 1);
  }
  for (const name of registryNames) {
    const count = hostExportCounts.get(name) ?? 0;
    if (count === 0) {
      violations.push(`🟦️.tsx: no exported component matching ${name}(...: ComponentSceneHostProps)`);
    } else if (count > 1) {
      violations.push(`🟦️.tsx: multiple ${name} exports matching the host contract`);
    }
  }
  return violations;
}

class LintScript extends BundleScript {
  run(_segments: string[]): void {
    const violations = [...collectRegionBalanceViolations(this.root), ...collectHostSignatureViolations(this.root)];
    if (violations.length === 0) {
      console.log("framework-renderer-react: region/host-contract lint passed");
      return;
    }
    console.error(`framework-renderer-react: found ${violations.length} lint violation(s):`);
    for (const v of violations) console.error(`  ${v}`);
    process.exit(1);
  }
}
//#endregion 🔖️LintScript

const router = new ScriptRouter(fileURLToPath(new URL(".", import.meta.url)))
  .register("operation-diagnostic-host-check", OperationDiagnosticHostCheckScript)
    .register("test", TestScript)
  .register("viewport-presence-check", ViewportPresenceCheckScript)
  .register("lint", LintScript)
  .register("typecheck", TypecheckScript)
  .register("causal-input-publication-check", CausalInputPublicationCheckScript)
  .register("row-action-admission-check", RowActionAdmissionCheckScript)
  .register("tutorial-interaction-check", TutorialInteractionCheckScript)
  .register("flow-browser-runtime-check", FlowBrowserRuntimeCheckScript)
  .register("artifact-creation-progress-check", ArtifactCreationProgressCheckScript)
  .register("agent-bridge-check", AgentBridgeCheckScript)
  .register("directory-home-bootstrap-check", DirectoryHomeBootstrapCheckScript)
  .register("directory-invite-capability-check", DirectoryInviteCapabilityCheckScript)
  .register("hub-sign-in-spaces-check", HubSignInSpacesCheckScript)
  .register("scoped-presence-check", ScopedPresenceCheckScript)
  .register("world3d-interaction-check", World3dInteractionCheckScript)
  .register("world3d-modelling-check", World3dModellingCheckScript)
  .register("scene-shading-pixel-check", SceneShadingPixelCheckScript)
  .register("surface-switch-check", SurfaceSwitchCheckScript)
  .register("input-ledger-check", InputLedgerCheckScript)
  .register("window-scope-check", WindowScopeCheckScript)
  .register("surface-retention-check", SurfaceRetentionCheckScript)
  .register("document-opening-scope-check", DocumentOpeningScopeCheckScript)
  .register("view-state-carriage-check", ViewStateCarriageCheckScript);

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
