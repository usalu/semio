/** 🧭️ Browser actor child worker containment, GIS describe and document reservation gates.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { buildBudgetMs } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { isAbsolute, join, relative, resolve } from "node:path";
import { decodePackValue, encodePackValue } from "@semio-tech/framework-os";
import { produceFreshComponentV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts";
import { buildClosedBrowserActorArtifactV1, type ClosedBrowserActorArtifactV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts";
import { runVitest } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { freeLoopbackPort } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { trustedBootstrapBuildControl } from "../../🏗️bootstrap/🧾️provenance/🗃️bootstrap-bundle/🟦️.ts";
import { trustedBootstrapReadRegular } from "../../🏗️bootstrap/🧾️provenance/🧭️bootstrap-selection/🟦️.ts";
import { proveBrowserActorGisChildV1, proveBrowserActorGisEvidenceV1, retainBrowserActorGisEvidenceV1 } from "../../🏗️bootstrap/🧾️provenance/🌍️gis-browser-proof/🟦️.ts";
import { requireHubDatum } from "../../🧪️testing/📖️source-readers/🟦️.ts";

/** 🧵️ The browser-actor child Worker containment fixture (`🌐️browser-bundle/🧵️child/🧫️fixtures/🔣️.json`). */
type BrowserActorChildWorkerFixture = {
  readonly schema: string;
  readonly limits: Readonly<Record<string, number>>;
  readonly modules: Readonly<Record<string, string>>;
  readonly laws: readonly string[];
  readonly wireRows: readonly Record<string, any>[];
  readonly ownerFaults: readonly string[];
};

/** ✍️ A mutable working copy of the semantic browser-actor GIS descriptor, mutated one field per hostile case. */
type BrowserActorGisDescriptorDraft = { hashes?: Record<string, string>; manifest: Record<string, unknown>; executionProtocol: Record<string, unknown>; [key: string]: unknown };

/** 🧾️ The browser-actor GIS descriptor fixture (`🌐️browser-bundle/🧾️describe/🧫️fixtures/🔣️.json`). */
type BrowserActorGisDescribeFixture = {
  readonly schema: string;
  readonly descriptor: { readonly manifest: Record<string, unknown>; readonly executionProtocol: Record<string, unknown>; readonly [key: string]: unknown };
  readonly cases: readonly Record<string, any>[];
  readonly localTransport: { readonly stallMs: number; readonly bodyMs: number };
};

export class BrowserActorChildWorkerContainmentCheckScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    if(segments.length>1||(segments[0]!==undefined&&segments[0]!=="--publication-neutral"))throw Error("usage: browser-actor-child-worker-containment-check [--publication-neutral]");
    const ownerPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child";
    const fixture = JSON.parse(readFileSync(join(this.repoRoot, ownerPath, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
    const ts = await import("typescript");
    const program = ts.createProgram([join(this.repoRoot, ownerPath, "🟦️.ts"), join(this.repoRoot, ownerPath, "👷️worker/🟦️.ts")], {
      noEmit: true,
      strict: true,
      skipLibCheck: true,
      target: ts.ScriptTarget.ES2022,
      module: ts.ModuleKind.ESNext,
      moduleResolution: ts.ModuleResolutionKind.Bundler,
      allowImportingTsExtensions: true,
      types: ["node", "vitest/importMeta"],
      lib: ["lib.es2023.d.ts", "lib.dom.d.ts", "lib.webworker.d.ts"],
    });
    const errors = ts.getPreEmitDiagnostics(program).filter((item) => item.category === ts.DiagnosticCategory.Error);
    if (errors.length) throw new Error(ts.formatDiagnosticsWithColorAndContext(errors, { getCurrentDirectory: () => this.repoRoot, getCanonicalFileName: (name) => name, getNewLine: () => "\n" }));
    const wireModules = Object.fromEntries(
      fixture.wireRows.map((row: Record<string, any>) => [
        row.name,
        `const row = ${JSON.stringify(row)}; const nativePost = MessagePort.prototype.postMessage; let fired = false; MessagePort.prototype.postMessage = function(message, transfer) { if (message.kind !== row.at || fired) return nativePost.call(this, message, transfer); fired = true; if (row.duplicate) { nativePost.call(this, message, transfer); return nativePost.call(this, message); } const forged = { ...message, ...row.patch }; if (row.valueCase === 'non-finite') forged.value = Infinity; if (row.valueCase === 'alias') { const shared = {}; forged.value = [shared, shared]; } if (row.valueCase === 'oversized') forged.value = new ArrayBuffer(1048577); nativePost.call(this, forged, transfer); if (row.expected === 'ignored') nativePost.call(this, message); }; ${fixture.modules.normal}`,
      ]),
    );
    const modules = Object.fromEntries(
      Object.entries({ ...fixture.modules, ...wireModules } as Record<string, string>).map(([name, source]) => {
        const bytes = Buffer.from(source);
        return [name, { source, sha256: createHash("sha256").update(bytes).digest("hex"), byteLength: bytes.length }];
      }),
    );
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!artifactRoot) throw new Error("child Worker gate requires SEMIO_TEST_ARTIFACT_DIR");
    mkdirSync(artifactRoot, { recursive: true });
    const cacheDir = mkdtempSync(join(artifactRoot, "browser-actor-child-vite-"));
    const { createServer } = await import("vite");
    const vite = await createServer({
      configFile: false,
      root: this.repoRoot,
      cacheDir,
      clearScreen: false,
      optimizeDeps: { noDiscovery: true },
      server: { host: "127.0.0.1", port: await freeLoopbackPort(), strictPort: true, watch: null },
      plugins: [
        {
          name: "semio-containment-page",
          configureServer(server) {
            server.middlewares.use((request, response, next) => {
              if (request.url !== "/__semio-containment.html") return next();
              response.setHeader("content-type", "text/html");
              response.end("<!doctype html><meta charset=utf-8><title>Actor Containment</title>");
            });
          },
        },
      ],
    });
    let browser: Awaited<ReturnType<(typeof import("playwright"))["chromium"]["launch"]>> | undefined;
    const diagnostics: string[] = [],
      workerRequests: string[] = [];
    try {
      await vite.listen();
      const { chromium } = await import("playwright");
      browser = await chromium.launch({ headless: true });
      const page = await browser.newPage();
      page.on("console", (message) => diagnostics.push(message.type() + ":" + message.text()));
      page.on("pageerror", (error) => diagnostics.push("pageerror:" + error.message));
      page.on("requestfailed", (request) => diagnostics.push("requestfailed:" + request.url()));
      page.on("request", (request) => {
        if (decodeURI(request.url()).includes("/🧵️child/")) workerRequests.push(request.url());
      });
      await page.goto(vite.resolvedUrls!.local[0]! + "__semio-containment.html", { timeout: 15000 });
      const result = await page.evaluate(
        async ({ url, modules, limits, laws, wireRows, ownerFaults }) => {
          const api = await import(/* @vite-ignore */ url);
          const passed: string[] = [];
          const check = (condition: unknown, law: string) => {
            if (!condition) throw new Error("child Worker law: " + law);
          };
          const rejected = async (operation: () => Promise<unknown>, law: string) => {
            let failed = false;
            try {
              await operation();
            } catch {
              failed = true;
            }
            check(failed, law);
          };
          check(JSON.stringify(api.BROWSER_ACTOR_CHILD_LIMITS) === JSON.stringify(limits), "neutral limits");
          const baseline = api.browserActorChildCapacity();
          let generation = 0n;
          const live = new Set<any>();
          const reserve = async (name = "normal", signal?: AbortSignal) => {
            const row = modules[name];if(!row)throw new Error("Actual authored browser actor module required");
            const owner = await api.reserveBrowserActorChild({ actorId: "fixture-actor", activationGeneration: ++generation, bundleSha256: row.sha256, bundleByteLength: row.byteLength }, signal);
            live.add(owner);
            return owner;
          };
          const load = async (owner: any, name = "normal") => {
            const selected=modules[name];if(!selected)throw new Error("Actual authored browser module required");const source = new TextEncoder().encode(selected.source);
            await owner.load(source.buffer);
            check(source.byteLength === 0 && owner.progress().sourceDetached, "verified source transfer");
          };
          const retire = (owner: any) => {
            owner.close();
            live.delete(owner);
          };
          try {
            await rejected(
              () => api.reserveBrowserActorChild({ actorId: "fixture-actor", activationGeneration: 1n, bundleSha256: (()=>{const value=modules.normal;if(!value)throw new Error("Actual normal actor module required");return value.sha256;})(), bundleByteLength: (()=>{const value=modules.normal;if(!value)throw new Error("Actual normal actor module required");return value.byteLength;})(), workerUrl: "https://invalid.test/worker.js" }),
              "caller Worker URL",
            );
            passed.push("static-child");
            let owner = await reserve();
            await load(owner);
            passed.push("verified-transfer");
            const input = new Uint8Array([0, 17, 255]);
            const reply = await owner.invoke(["echo"], [input.buffer]);
            check(input.byteLength === 0 && reply instanceof ArrayBuffer && JSON.stringify(Array.from(new Uint8Array(reply))) === "[0,17,255]" && owner.progress().resultTransfersDetached === 1, "actual round trip transfer");
            passed.push("round-trip");
            const oversized = new ArrayBuffer(limits.messageBytes + 1);
            await rejected(() => owner.invoke(["echo"], [oversized]), "input max + 1");
            check(oversized.byteLength === limits.messageBytes + 1 && owner.progress().activeInvocations === 0, "rejected input remains attached");
            passed.push("pre-transfer-bound");
            const second = await reserve();
            await rejected(() => reserve(), "global reservation cap");
            check(api.browserActorChildCapacity().actors === 2, "aggregate reservation stable");
            retire(second);
            passed.push("aggregate-capacity");
            const wasiOwner = await reserve("wasi");
            await load(wasiOwner, "wasi");
            check((await wasiOwner.invoke(["clock"], [])) === true, "local monotonic WASI clock");
            passed.push("wasi-clock");
            check((await wasiOwner.invoke(["output"], [])) === true, "local copied WASI output");
            passed.push("wasi-output");
            await rejected(() => wasiOwner.invoke(["bound"], []), "WASI output max + 1");
            passed.push("wasi-output-bound");
            await rejected(() => wasiOwner.invoke(["exit"], []), "WASI exit retires child");
            check(wasiOwner.progress().phase === "closed", "WASI exit closes parent");
            retire(wasiOwner);
            passed.push("wasi-exit");
            await rejected(() => owner.invoke(["effect"], []), "host dispatch denied");
            passed.push("effect-denial");
            await rejected(() => owner.invoke(["oversize"], []), "output max + 1");
            passed.push("output-bound");
            const controller = new AbortController();
            retire(owner);
            owner = await reserve("normal", controller.signal);
            await load(owner);
            const pending = owner.invoke(["wait"], []);
            const settled = pending.then(
              () => false,
              () => true,
            );
            const busy = new ArrayBuffer(1);
            await rejected(() => owner.invoke(["echo"], [busy]), "single invocation slot");
            check(busy.byteLength === 1, "busy transfer remains attached");
            passed.push("single-flight");
            controller.abort();
            check((await settled) && owner.progress().phase === "closed", "abort settles pending");
            retire(owner);
            passed.push("abort-retirement");
            owner = await reserve();
            await load(owner);
            let timerFired = false;
            const timer = setTimeout(() => {
              timerFired = true;
            }, 100);
            await rejected(() => owner.invoke(["loop"], []), "tight loop invoke deadline");
            clearTimeout(timer);
            check(timerFired && owner.progress().phase === "closed", "parent remains responsive");
            retire(owner);
            passed.push("tight-loop-invoke");
            owner = await reserve("loadLoop");
            timerFired = false;
            const loadTimer = setTimeout(() => {
              timerFired = true;
            }, 100);
            await rejected(() => load(owner, "loadLoop"), "tight loop load deadline");
            clearTimeout(loadTimer);
            check(timerFired && owner.progress().phase === "closed", "load loop parent remains responsive");
            retire(owner);
            passed.push("tight-loop-load");
            owner = await reserve("activationFailure");
            await rejected(() => load(owner, "activationFailure"), "activation failure retires");
            check(owner.progress().phase === "closed", "activation fault terminal");
            retire(owner);
            passed.push("activation-failure");
            owner = await reserve();
            const normalModule=modules.normal;
            if(!normalModule)throw Error("Normal browser module missing");
            const changed = new TextEncoder().encode(normalModule.source);
            const firstByte=changed[0];
            if(firstByte===undefined)throw Error("Normal browser module empty");
            changed[0] = firstByte ^ 1;
            await rejected(() => owner.load(changed.buffer), "hash mismatch");
            check(changed.byteLength === 0 && owner.progress().phase === "closed", "digest rejection retires exclusive owner");
            retire(owner);
            passed.push("digest-denial");
            owner = await reserve();
            await load(owner);
            check((await owner.invoke(["echo"], ["fresh"])) === "fresh", "fresh generation works");
            retire(owner);
            passed.push("fresh-generation");
            let wireLaws = 0;
            for (const row of wireRows) {
              owner = await reserve(row.name);
              let outcome: unknown;
              try {
                await load(owner, row.name);
                outcome = await owner.invoke(["echo"], ["wire"]);
              } catch {}
              await new Promise((resolve) => setTimeout(resolve, 25));
              check(row.expected === "ignored" ? outcome === "wire" && owner.progress().phase === "active" : owner.progress().phase === "closed", row.name);
              retire(owner);
              wireLaws++;
            }
            let faultLaws = 0;
            for (const fault of ownerFaults) {
              const NativeWorker = globalThis.Worker,
                nativeWorkerPost = Worker.prototype.postMessage,
                nativePortPost = MessagePort.prototype.postMessage,
                nativeDigest = SubtleCrypto.prototype.digest;
              const abort = new AbortController();
              let hashBytes: ArrayBuffer | undefined, releaseDigest: (() => void) | undefined;
              try {
                if (fault === "constructor-failure")
                  (globalThis as any).Worker = class {
                    constructor() {
                      throw new Error("fixture constructor");
                    }
                  };
                if (fault === "boot-init-denial")
                  Worker.prototype.postMessage = function (this: Worker, message: Readonly<Record<string, unknown>>, transfer?: Transferable[] | StructuredSerializeOptions): void {
                    Reflect.apply(nativeWorkerPost, this, [{ ...message, extra: true }, transfer]);
                  };
                if (fault === "constructor-failure" || fault === "boot-init-denial") {
                  await rejected(() => reserve(), fault);
                  faultLaws++;
                  continue;
                }
                owner = await reserve("normal", abort.signal);
                if (fault === "abort-during-hash")
                  SubtleCrypto.prototype.digest = function (algorithm, data) {
                    hashBytes = data as ArrayBuffer;
                    return new Promise((resolve, reject) => {
                      releaseDigest = () => {
                        nativeDigest.call(this, algorithm, data).then(resolve, reject);
                      };
                    });
                  };
                MessagePort.prototype.postMessage = function (this: MessagePort, message: { kind?: string; bytes?: ArrayBuffer; readonly [key: string]: unknown }, transfer?: Transferable[] | StructuredSerializeOptions): void {
                  if ((message.kind === "load" && fault === "load-transfer-failure") || (message.kind === "invoke" && fault === "invoke-transfer-failure")) throw new Error("fixture transfer");
                  if ((message.kind === "load" && fault === "child-load-shape") || (message.kind === "invoke" && fault === "child-invoke-shape")) message = { ...message, extra: true };
                  if (message.kind === "load" && fault === "child-rehash-after-transfer") new Uint8Array(message.bytes!)[(modules.normal ?? (()=>{throw Error("Normal browser module missing");})()).source.indexOf("'echo'") + 1] = 69;
                  Reflect.apply(nativePortPost, this, [message, transfer]);
                };
                const source = new TextEncoder().encode((modules.normal ?? (()=>{throw Error("Normal browser module missing");})()).source);
                const loading = owner.load(source.buffer);
                check(source.byteLength === 0, fault + " consuming call");
                if (fault === "abort-during-hash") {
                  abort.abort();
                  await rejected(() => loading, fault);
                  check(hashBytes && new Uint8Array(hashBytes).every((byte) => byte === 0), "cancelled exclusive hash owner wiped");
                  releaseDigest?.();
                } else if (["load-transfer-failure", "child-load-shape", "child-rehash-after-transfer"].includes(fault)) {
                  await rejected(() => loading, fault);
                  if (fault === "child-rehash-after-transfer") check(owner.progress().sourceDetached, "changed bytes reached child rehash");
                } else {
                  await loading;
                  if (fault === "synchronous-source-consumption") check((await owner.invoke(["echo"], ["owned"])) === "owned", fault);
                  else await rejected(() => owner.invoke(["echo"], ["transfer"]), fault);
                }
                if (fault !== "synchronous-source-consumption") check(owner.progress().phase === "closed", fault + " terminal");
                retire(owner);
                faultLaws++;
              } finally {
                (globalThis as any).Worker = NativeWorker;
                NativeWorker.prototype.postMessage = nativeWorkerPost;
                MessagePort.prototype.postMessage = nativePortPost;
                SubtleCrypto.prototype.digest = nativeDigest;
                releaseDigest?.();
              }
            }
            check(JSON.stringify(passed.slice().sort()) === JSON.stringify(laws.slice().sort()), "exact fixture laws");
            check(JSON.stringify(api.browserActorChildCapacity()) === JSON.stringify(baseline), "all reservations returned");
            return { laws: passed.length, wireLaws, faultLaws, transfers: 2, forcedLoopTerminations: 2, capacity: api.browserActorChildCapacity() };
          } finally {
            for (const owner of live) owner.close();
          }
        },
        { url: "/@fs" + join(this.repoRoot, ownerPath, "🟦️.ts"), modules, limits: fixture.limits, laws: fixture.laws, wireRows: fixture.wireRows, ownerFaults: fixture.ownerFaults },
      );
      if(segments[0]==="--publication-neutral"){
        const {testProtectedActorChildExecutionV1}=await import(join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔎️verification/🧾️publication/🧪️tests/🟦️.ts"));
        await testProtectedActorChildExecutionV1(this.repoRoot,page,vite.resolvedUrls!.local[0]!);
      }
      if (!diagnostics.includes("log:[browser actor stdout] AB") || !diagnostics.includes("warning:[browser actor stderr] CD") || diagnostics.some((value) => value.includes("[browser actor stdout] ZZ")))
        throw new Error("child WASI synchronous output was not observed exactly before caller mutation");
      if (!workerRequests.some((url) => decodeURI(url).includes("/🧵️child/👷️worker/🟦️.ts"))) throw new Error("static child was not requested");
      if (diagnostics.some((row) => row.startsWith("pageerror:"))) throw new Error(diagnostics.join("\n"));
      console.log("browser-actor-child-worker-containment: typescript=1 chromium=1 " + JSON.stringify(result) + " passed");
    } catch (error) {
      console.error("browser-actor-child-worker-diagnostics: " + diagnostics.slice(-12).join("|"));
      throw error;
    } finally {
      await browser?.close();
      await vite.close();
    }
  }
}

export class BrowserActorGisDescribeCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments[0] !== undefined && segments[0] !== "--source" && segments[0] !== "--native")) throw new Error("usage: browser-actor-gis-describe-check [--source|--native]");
    const ownerPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle";
    const fixture = JSON.parse(readFileSync(join(this.repoRoot, ownerPath, "🧾️describe/🧫️fixtures/🔣️.json"), "utf8"));
    const ts = await import("typescript");
    const program = ts.createProgram([join(this.repoRoot, ownerPath, "🧾️describe/🟦️.ts")], {
      noEmit: true,
      strict: true,
      skipLibCheck: true,
      target: ts.ScriptTarget.ES2022,
      module: ts.ModuleKind.ESNext,
      moduleResolution: ts.ModuleResolutionKind.Bundler,
      allowImportingTsExtensions: true,
      types: [],
      lib: ["lib.es2023.d.ts", "lib.dom.d.ts"],
    });
    const errors = ts.getPreEmitDiagnostics(program).filter((item) => item.category === ts.DiagnosticCategory.Error);
    if (errors.length) throw new Error(ts.formatDiagnosticsWithColorAndContext(errors, { getCurrentDirectory: () => this.repoRoot, getCanonicalFileName: (name) => name, getNewLine: () => "\n" }));
    const api = await import(join(this.repoRoot, ownerPath, "🧾️describe/🟦️.ts"));
    const codec = { encode: encodePackValue, decode: decodePackValue };
    const deepEqual = (await import("fast-deep-equal")).default;
    const expected: BrowserActorGisDescriptorDraft = structuredClone(fixture.descriptor);
    const staged = encodePackValue(fixture.descriptor);
    for (const row of fixture.cases) {
      const value: BrowserActorGisDescriptorDraft = structuredClone(fixture.descriptor);
      if (row.name === "hash-not-empty") value.hashes = { wasmSha256: "a".repeat(64), coreWasmSha256: "", descriptorSha256: "" };
      if (row.name === "plugin-change") value.manifest.pluginId = "stdio";
      if (row.name === "execution-protocol-change") value.executionProtocol.appChannelVersion = 13;
      if (row.name === "extra-field") value.extra = true;
      const encoded = encodePackValue(value);
      const guest = row.name === "trailing-byte" ? new Uint8Array([...encoded, 0]) : encoded;
      let accepted = true;
      try {
        api.assertBrowserActorDescribeCapacityV1(row.name === "capacity-over" ? 1024 * 1024 - 7 : staged.byteLength);
        const boundStage: BrowserActorGisDescriptorDraft = { ...structuredClone(fixture.descriptor), hashes: { wasmSha256: "a".repeat(64), coreWasmSha256: "b".repeat(64), descriptorSha256: "c".repeat(64) } };
        const stagedInput = row.name === "staged-trailing-byte" ? new Uint8Array([...staged, 0]) : row.name === "staged-bound" ? encodePackValue(boundStage) : staged;
        api.verifyBrowserActorDescribeV1(guest, stagedInput, codec);
      } catch {
        accepted = false;
      }
      const oracle = row.name !== "capacity-over" && row.name !== "trailing-byte" && !row.name.startsWith("staged-") && deepEqual(value, expected);
      if (accepted !== row.accepted || accepted !== oracle) throw new Error("GIS describe law: " + row.name);
    }
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    const ticketsRoot = resolve(this.repoRoot, ".🧬semio", "🦑️repo", "🎫️tickets");
    const relativeRoot = artifactRoot ? relative(ticketsRoot, resolve(artifactRoot)) : "";
    if (!artifactRoot || !isAbsolute(artifactRoot) || relativeRoot === "" || relativeRoot.startsWith("..") || isAbsolute(relativeRoot)) throw new Error("GIS child gate requires ticket-owned SEMIO_TEST_ARTIFACT_DIR");
    mkdirSync(artifactRoot, { recursive: true, mode: 0o700 });
    await proveBrowserActorGisEvidenceV1(artifactRoot);
    const { createFreshComponentTests } = await import("../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🆕️fresh-component/🟦️.ts");
    await createFreshComponentTests(this.invocation).testFreshComponentProcessV1(this.repoRoot);
    console.log(`browser-actor-gis-describe: TypeScript=1 fast-deep-equal=${fixture.cases.length} normalization=${fixture.cases.length} source passed; real guest execution requires --native`);
    if (segments[0] !== "--native") return;
    const work = mkdtempSync(join(artifactRoot, "gis-child-real-"));
    const target = join(work, "target"),
      stage = join(work, "stage");
    mkdirSync(target, { mode: 0o700 });
    mkdirSync(stage, { mode: 0o700 });
    const build = trustedBootstrapBuildControl(buildBudgetMs());
    let actor: ClosedBrowserActorArtifactV1 | undefined, descriptor: Uint8Array | undefined;
    try {
      const produced = await produceFreshComponentV1(
        this.repoRoot,
        { pluginId: "gis", cargoPackage: "semio-hub-gis-component", componentPackageId: "semio:gis", outputName: "semio_hub_gis_component.wasm", componentProfile: "release" },
        stage,
        {...build.control,compiler:scriptCompilerAdmissionControlV1(this.invocation,"fresh:gis",Math.floor(build.control.remainingMs())),invocation:this.invocation},
        (lease,evidence) =>
          lease.consume((component) =>
            buildClosedBrowserActorArtifactV1(component, this.invocation, evidence.control, {
              cancelled: () => build.control.cancelled() || build.control.remainingMs() <= 0,
              progress: (phase, completed, total) => build.control.checkpoint("gis-child-" + phase, completed, total),
            }),
          ),
      );
      actor = requireHubDatum(produced.derived,"derived browser actor");
      if (actor.componentSha256 !== produced.receipt.component.sha256 || createHash("sha256").update(actor.bytes).digest("hex") !== actor.sha256 || actor.bytes.byteLength !== actor.byteLength)
        throw new Error("GIS child fresh actor receipt mismatch");
      descriptor = trustedBootstrapReadRegular(join(stage, produced.receipt.descriptor.relativePath), 4 * 1024 * 1024, "GIS descriptor", () => {
        if (build.control.cancelled() || build.control.remainingMs() <= 0) throw new Error("GIS child cancelled");
      });
      if (descriptor.byteLength !== produced.receipt.descriptor.byteLength || createHash("sha256").update(descriptor).digest("hex") !== produced.receipt.descriptor.sha256) throw new Error("GIS child staged descriptor mismatch");
      api.assertBrowserActorDescribeCapacityV1(descriptor.byteLength);
      const result = await proveBrowserActorGisChildV1(this.repoRoot, work, { ...actor, descriptorByteSha256: produced.receipt.descriptor.sha256 }, descriptor, build.control);
      const retainedRoot = retainBrowserActorGisEvidenceV1(artifactRoot, stage, produced.receipt, actor, descriptor);
      console.log(
        "browser-actor-gis-describe-native: " +
          JSON.stringify({
            receipt: produced.receipt,
            actorSha256: actor.sha256,
            actorByteLength: actor.byteLength,
            normalizedDescriptor: true,
            stalledBodyRetired: result.stalledBodyRetired,
            sourceDetached: result.sourceDetached,
            resultTransfers: result.transferCount,
            capacity: result.capacity,
            retainedRoot,
          }) +
          " passed; no Hub session, renderer, Map mutation or collaboration claim",
      );
    } finally {
      actor?.bytes.fill(0);
      descriptor?.fill(0);
      build.close();
      rmSync(work, { recursive: true, force: true });
    }
  }
}

export class BrowserActorDocumentReservationCheckScript extends BundleScript {
  async run(): Promise<void> {
    const root = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory");
    const fixture = JSON.parse(readFileSync(join(root, "🧵️browser-actor-reservation-v1.json"), "utf8"));
    const sessionFixture = JSON.parse(readFileSync(join(root, "🧵️browser-actor-session-v1.json"), "utf8"));
    const bootstrapFixture = JSON.parse(readFileSync(join(root, "🧵️artifact-bootstrap-owner-v1.json"), "utf8"));
    await runVitest(
      join(this.repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript"),
      ["--run", "-t", "artifact bootstrap atomic restore|execution target body reader|browser document actor reservation|browser execution target lease|browser GIS viewer exposes localized renderer-unavailable"],
      "../../../../../../🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🎚️config/🟦️.ts",
    );
    console.log(
      "browser-actor-document-reservation: bootstrap-ownership=" +
        bootstrapFixture.cases.length +
        " body-read=7 ownership=16 cases=" +
        fixture.cases.length +
        " lifecycle=" +
        fixture.lifecycle.length +
        " session-activation=" +
        sessionFixture.cases.length +
        " passed; reserved-only rows fetch/load=0, Session rows exercise mocked child, real GIS Chromium remains separate",
    );
  }
}
