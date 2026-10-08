#!/usr/bin/env bun
import { TEST_LEVEL_BUDGET_MS, resolveTestLevel } from "../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🖥️ `@semio-tech/framework-os` task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`. */
import { join } from "node:path";
import { readFileSync } from "node:fs";
import { getWorkspaceRoot, runBunx, runVitest } from "../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { runOwnedCommand } from "../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runWgpuPackageGenerator } from "../../🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️publication/🟦️.ts";

/** ⚖️ Verifies bounded installed service dispatch, transport and contribution removal. */
class InstalledServiceCheckScript extends BundleScript {
  async run(): Promise<void> {
    const { verifyInstalledServiceLawsV1 } = await import("../../🔨️modules/💡️inference/🔌️service/🧪️tests/🟦️.ts");
    console.log("installed-service-check", verifyInstalledServiceLawsV1());
  }
}

/** 🎬️ Runs the actual OS media parser against the shared language-neutral corpus. */
class MediaTransportTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("media-transport accepts no arguments");
    await runOwnedCommand(process.execPath, ["test", join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🎬️media/🧪️tests/🎬️transport-reservation/🟦️.ts")], this.repoRoot, "os-media-transport", TEST_LEVEL_BUDGET_MS.fundamental);
  }
}

/** 📡️ Validates the defining OS browser producer against neutral General scene wire vectors. */
class SceneWireSourceScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    if(segments.length)throw Error("test-scene-wire-source accepts no arguments");
    await runOwnedCommand(process.execPath,["test",join(this.root,"../../🧪️tests/📡️scene-wire/🟦️.ts")],this.repoRoot,"os-scene-wire",TEST_LEVEL_BUDGET_MS.long);
  }
}

/** 🏪️ Runs the store's language-neutral history oracles (supersede replay, tool transaction, deferred reprojection, viewer head, supersede law) under `bun:test`. */
class StoreOraclesTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-store-oracles accepts no arguments");
    const oracles = ["🧪️supersede-replay", "🧪️tool-transaction", "🧪️deferred-reprojection", "🧪️viewer-head", "🧪️supersede-law"].map((oracle) => join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests", oracle, "🟦️.ts"));
    await runOwnedCommand(process.execPath, ["test", ...oracles], this.repoRoot, "os-store-oracles", TEST_LEVEL_BUDGET_MS.fundamental);
  }
}

/** 🗃️ Runs the archive-load host twin against the channel's language-neutral corpus, the attached-document replacement law, the document-port control turn law, the folder archive persistence law and the folder read-back law under `bun:test`. */
class ChannelOraclesTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-channel-oracles accepts no arguments");
    const oracles = ["🧪️document-archive-load-host", "🧪️attached-document-replacement", "🧪️document-port-control-turn", "🧪️folder-archive-persistence", "🧪️folder-read-back"].map((oracle) => join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🧪️tests", oracle, "🟦️.ts"));
    await runOwnedCommand(process.execPath, ["test", ...oracles], this.repoRoot, "os-channel-oracles", TEST_LEVEL_BUDGET_MS.fundamental);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments, "quick");
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

/** 🏗️ Routes the package generator through the shared workspace implementation. */
class GenerateWgpuScript extends BundleScript {
  async run(): Promise<void> {
    await runWgpuPackageGenerator(getWorkspaceRoot(), "generate");
  }
}
/** 🔎️ Checks the exact package artifacts without writing outputs. */
class CheckWgpuScript extends BundleScript {
  async run(): Promise<void> {
    await runWgpuPackageGenerator(getWorkspaceRoot(), "check");
  }
}
/** 🔮️ Streams the canonical read-only package preview. */
class PreviewGeneratedScript extends BundleScript {
  async run(): Promise<void> {
    await runWgpuPackageGenerator(getWorkspaceRoot(), "preview");
  }
}

//#region 🧬️OwnedSchemaExports
const OWNED_SCHEMA_MODULES = {
  os: "🧰️framework/🛍️products/💻️os/🧬️schema/🔣️.json",
  directory: "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json",
  renderer: "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🔣️.json",
} as const;

/** 🧬️ Compiles one named `$defs` export of an owning `🧬️schema/` module against its draft-07 `$id`. */
async function ownedExport(repoRoot: string, scope: keyof typeof OWNED_SCHEMA_MODULES, exportId: string) {
  const { semioSchemaAjvV1 } = await import("../../../../🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts");
  const doc = JSON.parse(readFileSync(join(repoRoot, OWNED_SCHEMA_MODULES[scope]), "utf8")) as { $id: string };
  const compiled = semioSchemaAjvV1({ strict: true, allErrors: true }).addSchema(doc).getSchema(`${doc.$id}#/$defs/${exportId}`);
  if (!compiled) throw new Error(`${scope} schema module publishes no export ${exportId}`);
  return compiled;
}
//#endregion 🧬️OwnedSchemaExports

//#region 🪪️DocumentOpeningAttemptCheck
/** 🪪️ Proves one Shell opening attempt owns every asynchronous outer worker lifecycle frame. */
async function proveDocumentOpeningAttempt(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory");
  const fixture = JSON.parse(readFileSync(join(root, "🧵️document-opening-attempt-v1.json"), "utf8"));
  const deepEqual = (await import("fast-deep-equal")).default;
  const observed = fixture.cases.map((row: Record<string, unknown>) => {
    const current = row.current as string | null;
    const incoming = row.incoming as string | null;
    const valid = incoming === "a" || incoming === "b";
    if (!valid) return { accepted: false, replaced: false, currentAfter: current };
    if (row.kind === "open") {
      if (current === incoming) return { accepted: false, replaced: false, currentAfter: current };
      return { accepted: true, replaced: current !== null, currentAfter: incoming };
    }
    const accepted = current === incoming;
    return { accepted, replaced: false, currentAfter: accepted && row.kind === "close" ? null : current };
  });
  const expected = fixture.cases.map((row: Record<string, unknown>) => ({ accepted: row.accepted, replaced: row.replaced, currentAfter: row.currentAfter }));
  if (!deepEqual(observed, expected)) throw new Error("document opening attempt independent model differs from the corpus");
  const rebootstrapObserved = fixture.rebootstrapCases.map((row: Record<string, unknown>) => {
    const accepted = row.entryClient === row.messageClient;
    const removed = accepted && row.currentOwner !== null;
    const ownerAfter = removed ? null : row.currentOwner;
    return { messageAccepted: accepted, ownerRemoved: removed, ownerAfter, freshBaseZeroAccepted: accepted && ownerAfter === null };
  });
  const rebootstrapExpected = fixture.rebootstrapCases.map((row: Record<string, unknown>) => ({ messageAccepted: row.messageAccepted, ownerRemoved: row.ownerRemoved, ownerAfter: row.ownerAfter, freshBaseZeroAccepted: row.freshBaseZeroAccepted }));
  if (!deepEqual(rebootstrapObserved, rebootstrapExpected)) throw new Error("document rebootstrap owner retirement differs from the corpus");

  const protocol = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🟦️.ts"), "utf8");
  const worker = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"), "utf8");
  const shell = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"), "utf8");
  const storeWire = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs"), "utf8");
  const storeWorker = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🦀️.rs"), "utf8");
  const patchHandoff = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🟦️.ts"), "utf8");
  const conforms = (candidateProtocol: string, candidateWorker: string, candidateShell: string, candidateStoreWire: string, candidateStoreWorker: string): boolean =>
    candidateProtocol.includes('readonly kind: "open"; readonly clientInstanceId?: string') &&
    candidateProtocol.includes('readonly kind: "close"; readonly documentId: string; readonly spaceId?: string; readonly clientInstanceId?: string') &&
    candidateProtocol.includes('readonly kind: "send"; readonly documentId: string; readonly spaceId?: string; readonly clientInstanceId?: string') &&
    candidateProtocol.includes("workerWireClientInstanceIdV1") &&
    candidateProtocol.includes('readonly clientInstanceId: string') &&
    candidateWorker.includes('type DocumentExecutionOwnerEntry = Readonly<{ owner: DocumentExecutionOwner; documentId: string; clientInstanceId: string; spaceId?: string }>') &&
    candidateWorker.includes("previous.clientInstanceId") &&
    candidateWorker.includes("request.clientInstanceId !== current.clientInstanceId") &&
    candidateWorker.includes("openClientInstanceId: request.clientInstanceId ?? crypto.randomUUID()") &&
    candidateWorker.includes('post({ kind: "event", documentId: state.config.documentId, clientInstanceId: state.openClientInstanceId') &&
    candidateWorker.includes("post({ ...offer, clientInstanceId: this.state.openClientInstanceId })") &&
    candidateWorker.includes("retireArtifactBeforeReplacement(runtimeKey);") &&
    candidateShell.includes('if (message.kind === "artifact-rebootstrap-required") {\n          if (browserActorUiByRuntimeKeyRef.current.delete(runtimeKey)) setBrowserActorUiVersion((current) => current + 1);\n          const active = sessionRef.current;') &&
    candidateShell.includes("const openingAttempt = { clientInstanceId: crypto.randomUUID() }") &&
    candidateShell.includes('const socketActorReadyRef = useRef<Map<string, { readonly clientInstanceId: string;') &&
    candidateShell.includes("if (waiter !== undefined && waiter.clientInstanceId === clientInstanceId)") &&
    candidateShell.includes('const request: BackboneWorkerRequest = { kind: "close", documentId: entry.documentId, clientInstanceId: entry.clientInstanceId') &&
    candidateStoreWire.includes("client_instance_id: Option<String>") &&
    candidateStoreWorker.includes("entry.client_instance_id") &&
    !patchHandoff.includes("clientInstanceId") &&
    !candidateProtocol.includes("type Bad =");
  if (!conforms(protocol, worker, shell, storeWire, storeWorker)) throw new Error("document opening attempt production correlation is incomplete");
  const hostiles = [
    [protocol, worker, shell.replace("const openingAttempt = { clientInstanceId: crypto.randomUUID() }", 'const openingAttempt = { clientInstanceId: "" }'), storeWire, storeWorker],
    [protocol, worker, shell.replace('const socketActorReadyRef = useRef<Map<string, { readonly clientInstanceId: string;', 'const socketActorReadyRef = useRef<Map<string, {'), storeWire, storeWorker],
    [protocol, worker, shell.replace("if (waiter !== undefined && waiter.clientInstanceId === clientInstanceId)", "if (waiter !== undefined)"), storeWire, storeWorker],
    [protocol, worker, shell.replace('const request: BackboneWorkerRequest = { kind: "close", documentId: entry.documentId, clientInstanceId: entry.clientInstanceId', 'const request: BackboneWorkerRequest = { kind: "close", documentId: entry.documentId'), storeWire, storeWorker],
    [protocol, worker.replace("openClientInstanceId: request.clientInstanceId ?? crypto.randomUUID()", "openClientInstanceId: crypto.randomUUID()"), shell, storeWire, storeWorker],
    [protocol, worker.replace('type DocumentExecutionOwnerEntry = Readonly<{ owner: DocumentExecutionOwner; documentId: string; clientInstanceId: string; spaceId?: string }>', 'type DocumentExecutionOwnerEntry = Readonly<{ owner: DocumentExecutionOwner; documentId: string; spaceId?: string }>'), shell, storeWire, storeWorker],
    [protocol, worker.replace("request.clientInstanceId !== current.clientInstanceId", "false"), shell, storeWire, storeWorker],
    [protocol, worker.replace("previous.clientInstanceId", "request.clientInstanceId"), shell, storeWire, storeWorker],
    [protocol, worker.replace('post({ kind: "event", documentId: state.config.documentId, clientInstanceId: state.openClientInstanceId', 'post({ kind: "event", documentId: state.config.documentId'), shell, storeWire, storeWorker],
    [protocol, worker.replace("post({ ...offer, clientInstanceId: this.state.openClientInstanceId })", "post(offer)"), shell, storeWire, storeWorker],
    [`${protocol}\ntype Bad = BrowserActorUiPatchOfferV1 & { readonly clientInstanceId: string };`, worker, shell, storeWire, storeWorker],
    [protocol, worker.replace("retireArtifactBeforeReplacement(runtimeKey);", "closeArtifactRuntime(runtimeKey);"), shell, storeWire, storeWorker],
    [protocol, worker, shell.replace('if (message.kind === "artifact-rebootstrap-required") {\n          if (browserActorUiByRuntimeKeyRef.current.delete(runtimeKey)) setBrowserActorUiVersion((current) => current + 1);', 'if (message.kind === "artifact-rebootstrap-required") {'), storeWire, storeWorker],
  ];
  if (hostiles.length !== fixture.sourceHostiles.length) throw new Error("document opening attempt hostile count differs");
  hostiles.forEach(([candidateProtocol, candidateWorker, candidateShell, candidateStoreWire, candidateStoreWorker], index) => {
    if (conforms(candidateProtocol!, candidateWorker!, candidateShell!, candidateStoreWire!, candidateStoreWorker!)) throw new Error(`document opening attempt source oracle admitted ${fixture.sourceHostiles[index]}`);
  });
  console.log(`document-opening-attempt-oracle: AJV=1 opening=${observed.length} rebootstrap=${rebootstrapObserved.length} source-hostiles=${hostiles.length}`);
  return 1 + observed.length + rebootstrapObserved.length + hostiles.length;
}

class DocumentOpeningAttemptCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("document-opening-attempt-check accepts no arguments");
    const checks = await proveDocumentOpeningAttempt(this.repoRoot);
    await runVitest(this.root, ["--testNamePattern", "document opening attempt"], "../../🧪️tests/🎚️config/🟦️.ts");
    console.log(`document-opening-attempt-check: checks=${checks}`);
  }
}
//#endregion 🪪️DocumentOpeningAttemptCheck

/** 🩺️ Type-checks every `💻️os` TypeScript source against the product-scoped `tsconfig.json`. */
class MutationVerbVocabularyCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { checkMutationVerbVocabulary } = await import("../../🔨️modules/📡️spr/🎮️command/🧪️tests/🗣️verb-vocabulary/🟦️.ts");
    await checkMutationVerbVocabulary(this.repoRoot, segments);
  }
}

class TypecheckScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["tsc", "--noEmit", "-p", "../../tsconfig.json", ...segments], this.root);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("test", TestScript)
  .register("test-media-transport", MediaTransportTestScript)
  .register("test-scene-wire-source",SceneWireSourceScript)
  .register("test-store-oracles", StoreOraclesTestScript)
  .register("test-channel-oracles", ChannelOraclesTestScript)
  .register("typecheck", TypecheckScript)
  .register("mutation-verb-vocabulary-check", MutationVerbVocabularyCheckScript)
  .register("installed-service-check", InstalledServiceCheckScript)
  .register("generate-wgpu", GenerateWgpuScript)
  .register("check-wgpu", CheckWgpuScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("document-opening-attempt-check", DocumentOpeningAttemptCheckScript)
;

await runScriptMain(router, { defaultCommand: "test" });
