/** 🧬️ The jco asset URL, async result and preview2 shim rewrites and the component instantiation of the ticket-owned browser host staging suite. */
import { actorLifetimeFixtureUrl } from "@semio-tech/framework-actor/testing/fixtures";
import { createTestKit, type TestDependencies, type TestSource } from "../🧰️test-kit/🟦️.ts";

export async function registerJcoRewriteTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: TestDependencies, source: TestSource): Promise<void> {
  const { MODULE_EXTENSION_ROUTE, MODULE_PLUGIN_ROUTE, PLUGIN_HOST_SHIM_FILE, basename, hostShimSource, join, pluginComponentBridgeSource, readFileSync, repoRoot, rewriteJcoAsyncResultLifting, rewriteJcoComponentAssetUrls, rewritePreview2ShimImportSource, shardWorkerSource } = dependencies;
  const { describe, expect, it } = vitest;
  const { itLong, generatedBridge, retainedGuestSource } = createTestKit(vitest, dependencies, source);

  //#region 🔖️T-P8-tests
  describe("rewriteJcoComponentAssetUrls", () => {
    itLong("keeps generated host imports and replies isolated across same-package activations", async () => {
      const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧫️fixtures/⚡️host-activation.json"), "utf8")) as { activations: Array<{ actorId: string; generation: string; value: string }> };
      const component = rewriteJcoComponentAssetUrls(retainedGuestSource({
        prelude: `import { storageRead, emit } from "./${PLUGIN_HOST_SHIM_FILE}";\nconst identity = new URL(import.meta.url).searchParams.get("actor") + ":" + new URL(import.meta.url).searchParams.get("activation");`,
        poll: `emit(identity); return { retainedReceipt: receipt(grant.retained), fuelUsed: 0n, value: await storageRead(identity), uiPatches: [], status: { tag: "idle" }, commandIngress: { kind: 0 } };`,
      }));
      const sent: any[] = [];
      const priorSelf = Object.getOwnPropertyDescriptor(globalThis, "self");
      Object.defineProperty(globalThis, "self", { value: { postMessage: (message: unknown) => sent.push(message) }, configurable: true, writable: true });
      const bridge = await generatedBridge(component);
      try {
        const apis = [];
        for (const row of fixture.activations) apis.push(await bridge.activate(BigInt(row.generation), row.actorId));
        const pending = apis.map((api) => api.poll([], undefined, undefined, { fuel: 1, wallMs: 4, maxEffects: 8, maxPatchBytes: 4096 }));
        const isRequest = (message: any): boolean => message.frame.envelope.payload.kind === "effect-request";
        for (let attempt = 0; sent.filter(isRequest).length < apis.length; attempt += 1) {
          if (attempt > 500) throw new Error("the activations never requested their storage read");
          await new Promise((resolve) => setTimeout(resolve, 10));
        }
        const effects = fixture.activations.map((row) => sent.find((message) => isRequest(message) && message.actorId === row.actorId && String(message.activationGeneration) === row.generation));
        expect(effects.every((effect) => effect !== undefined)).toBe(true);
        expect(effects.map((message) => ({ actorId: message.actorId, generation: String(message.activationGeneration) }))).toEqual(fixture.activations.map(({ actorId, generation }) => ({ actorId, generation })));
        for (let index = 0; index < effects.length; index += 1) apis[index].resolveEffect(effects[index].frame.envelope.payload.payload.requestId, fixture.activations[index].value);
        expect((await Promise.all(pending)).map((value) => value.value)).toEqual(fixture.activations.map((row) => row.value));
        const emissions = sent.filter((message) => message.frame.envelope.payload.kind === "effect-emit").map((message) => ({ actorId: message.actorId, generation: String(message.activationGeneration) }));
        expect(emissions.sort((left, right) => `${left.actorId}:${left.generation}`.localeCompare(`${right.actorId}:${right.generation}`))).toEqual(fixture.activations.map(({ actorId, generation }) => ({ actorId, generation })));
        expect(new Set(effects.map((effect) => effect.frame.envelope.payload.payload.requestId)).size).toBe(fixture.activations.length);
      } finally {
        if (priorSelf) Object.defineProperty(globalThis, "self", priorSelf); else delete (globalThis as Record<string, unknown>).self;
        bridge.dispose();
      }
    });

    /** 🧵️ The generated worker runs in a `node:vm` context whose fake api answers the retained original source it reads.
     * @see ../../../../🔌️plugin/🧪️tests/✂️worker-vm-source/🟦️.ts */
    const workerContext = async (api: (actorId: string, generation: bigint) => unknown) => {
      const vm = await import("node:vm");
      const { classicShardWorkerSource } = await import("../../../../🔌️plugin/🧪️tests/✂️worker-vm-source/🟦️.ts");
      const messages: any[] = [];
      let listener: ((event: { data: Record<string, unknown> }) => Promise<void>) | undefined;
      const context = vm.createContext({ console, URL, Uint8Array, WebAssembly: { Suspending() {}, promising: (value: unknown) => value }, setInterval: () => 1, clearInterval: () => undefined, __import: async () => ({ createActorApi: async (actorId: string, generation: bigint) => api(actorId, generation) }), self: { location: { href: "https://fixture.invalid/🔌️plugin-modules/🧵️shard-worker.js" }, postMessage: (message: unknown) => messages.push(message), addEventListener: (_kind: string, callback: typeof listener) => { listener = callback; } } });
      new vm.Script(classicShardWorkerSource(shardWorkerSource()).replace("await import(/* @vite-ignore */ moduleUrl)", "await __import(moduleUrl)")).runInContext(context);
      if (!listener) throw new Error("Missing generated worker dispatcher");
      return { messages, send: listener };
    };
    const retainedSource = async () => {
      const { vmOriginalSource } = await import("../../../../🔌️plugin/🧪️tests/✂️worker-vm-source/🟦️.ts");
      return { originalSource: { ...vmOriginalSource, operation: 1n, generation: 1n, cancel() {}, nativeRefusal: undefined }, closeOriginal: async () => ({ operation: 1n, generation: 1n, sourceClosed: true }) };
    };

    itLong("rejects a stale effect reply in the actual generated worker dispatcher", async () => {
      const replies: { generation: string; request: string; value: string }[] = [];
      const retained = await retainedSource();
      const { send } = await workerContext((_actorId, generation) => ({ ...retained, resolveEffect: (request: string, value: string) => replies.push({ generation: String(generation), request, value }) }));
      await send({ data: { kind: "activate", requestId: "a1", actorId: "same", activationGeneration: 1n, moduleUrl: "fixture" } });
      await send({ data: { kind: "dispose", actorId: "same", activationGeneration: 1n } });
      await send({ data: { kind: "activate", requestId: "a2", actorId: "same", activationGeneration: 2n, moduleUrl: "fixture" } });
      for (const [generation, from, to, value] of [[1n, "kernel", "same", "old"], [undefined, "kernel", "same", "missing"], [2n, "actor", "same", "foreign-origin"], [2n, "kernel", "other", "foreign-target"], [2n, "kernel", "same", "fresh"]] as const) {
        await send({ data: { kind: "frame", actorId: "same", activationGeneration: generation, frame: { kind: "Envelope", envelope: { to, from: { kind: from }, payload: { kind: "effect-complete", payload: { requestId: "reused", value } } } } } });
      }
      expect(replies).toEqual([{ generation: "2", request: "reused", value: "fresh" }]);
    });

    itLong("forwards lifecycle through the captured scheduled turn and rejects the removed side message", async () => {
      const fixture = JSON.parse(readFileSync(actorLifetimeFixtureUrl, "utf8")) as { vectors: Array<{ value: { kind: string }; hex: string }> };
      const request = fixture.vectors.find((row) => row.value.kind === "close")!;
      const calls: unknown[] = [];
      const retained = await retainedSource();
      const { messages, send } = await workerContext(() => { calls.push("captured"); return { ...retained, poll: async (events: unknown) => { calls.push(events); return { status: { tag: "idle" } }; } }; });
      await send({ data: { kind: "activate", requestId: "a1", actorId: "same", activationGeneration: 1n, moduleUrl: "fixture" } });
      const wire = Uint8Array.from(Buffer.from(request.hex, "hex"));
      const payload = JSON.parse(JSON.stringify(request.value), (key, value) => ["activationGeneration", "guestLifetime", "closeGeneration"].includes(key) ? BigInt(value) : value);
      const events = [{ kind: "instance-close", payload }];
      await send({ data: { kind: "turn", requestId: "close", actorId: "same", activationGeneration: 1n, events, budget: {} } });
      await send({ data: { kind: "turn", requestId: "old", actorId: "same", activationGeneration: 2n, events, budget: {} } });
      await send({ data: { kind: "closeInstance", requestId: "side", actorId: "same", activationGeneration: 1n, wire } });
      await send({ data: { kind: "activate", requestId: "reuse", actorId: "same", activationGeneration: 2n, moduleUrl: "fixture" } });
      expect(messages.filter((message) => message.kind === "result").map(({ requestId, ok, error }) => ({ requestId, ok, ...(error ? { error } : {}) }))).toEqual([
        { requestId: "a1", ok: true },
        { requestId: "close", ok: true },
        { requestId: "old", ok: false, error: "actor-lifecycle.activation-mismatch" },
        { requestId: "side", ok: false, error: "unknown shard worker message kind: closeInstance" },
        { requestId: "reuse", ok: false, error: "actor-close.activation-already-owned" },
      ]);
      expect(messages.filter((message) => message.kind === "instanceCloseReceipt")).toHaveLength(0);
      expect(calls).toEqual(["captured", [{ kind: "instance-close", payload }]]);
    });

    it("propagates a component module rebuild version to every extracted core wasm fetch", () => {
      const generated = `const module0 = fetchCompile(new URL('./plugin_component.core.wasm', import.meta.url));
const module1 = fetchCompile(new URL('./plugin_component.core2.wasm', import.meta.url));`;
      const rewritten = rewriteJcoComponentAssetUrls(generated);
      expect(rewritten).toContain("function __semioVersionedComponentAssetUrl(path)");
      expect(rewritten).toContain("const rebuildVersion = new URL(import.meta.url).searchParams.get(\"v\")");
      expect(rewritten).toContain("__semioVersionedComponentAssetUrl('./plugin_component.core.wasm')");
      expect(rewritten).toContain("__semioVersionedComponentAssetUrl('./plugin_component.core2.wasm')");
      expect(rewriteJcoComponentAssetUrls(rewritten)).toBe(rewritten);
    });
  });

  describe("PluginComponentInstantiation", () => {
    itLong("executes independent Wasm memories from one cached explicit factory module", async () => {
      const { execFileSync } = await import("node:child_process");
      const { default: Ajv } = await import("ajv");
      const root = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧫️fixtures");
      const fixture = JSON.parse(readFileSync(join(root, "🏗️component-instantiation.json"), "utf8"));
      const oracle = new Ajv({ strict: true });
      expect(oracle.validate(JSON.parse(readFileSync(join(root, "../🧬️schema/🏗️component-instantiation/🔣️.json"), "utf8")), fixture)).toBe(true);
      const output = execFileSync("node", ["--experimental-vm-modules", "--input-type=module", "--eval", `
        import { parse, transpile } from "@bytecodealliance/jco";
        import { SourceTextModule, createContext } from "node:vm";
        import { readFileSync } from "node:fs";
        import ts from "typescript";
        const input = JSON.parse(readFileSync(0, "utf8"));
        const bytes = await parse(input.component);
        const options = { name: "counter", noTypescript: true, base64Cutoff: 0, nodejsCompat: false, wasiShim: false };
        const explicit = await transpile(bytes, { ...options, instantiation: "async" });
        const automatic = await transpile(bytes, options);
        const source = new TextDecoder().decode(explicit.files["counter.js"]);
        const auto = new TextDecoder().decode(automatic.files["counter.js"]);
        const moduleInstanceRoots = source => {
          const parsed = ts.createSourceFile("counter.js", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
          return parsed.statements.some(statement => ts.isVariableStatement(statement) && statement.declarationList.declarations.some(declaration => ts.isIdentifier(declaration.name) && /^(exports|memory)[0-9]+$/.test(declaration.name.text)));
        };
        const context = createContext({ WebAssembly, Promise, console, TextEncoder, TextDecoder });
        const module = new SourceTextModule(source, { context, identifier: "https://fixture.invalid/counter.js" });
        await module.link(() => { throw new Error("Unexpected factory import"); });
        await module.evaluate();
        const captured = [];
        const moduleImportInstances = captured.length;
        const load = path => WebAssembly.compile(explicit.files[path]);
        const instantiate = (module, imports) => { const instance = new WebAssembly.Instance(module, imports); captured.push(instance); return instance; };
        const owners = [await module.namespace.instantiate(load, {}, instantiate), await module.namespace.instantiate(load, {}, instantiate)];
        const results = input.calls.map(call => owners[call.instance].advance(call.delta));
        const core = await load("counter.core.wasm");
        const direct = [new WebAssembly.Instance(core), new WebAssembly.Instance(core)];
        const independent = input.calls.map(call => direct[call.instance].exports.advance(call.delta));
        const actual = { moduleImportInstances, instantiated: captured.length, results, memory: captured.map(instance => new DataView(instance.exports.memory.buffer).getUint32(0, true)), distinctMemories: captured[0].exports.memory !== captured[1].exports.memory, factoryHasNoModuleInstanceRoots: !moduleInstanceRoots(source), automaticHasModuleInstanceRoots: moduleInstanceRoots(auto) };
        console.log(JSON.stringify({ actual, independent }));
      `], { cwd: repoRoot, input: JSON.stringify(fixture), encoding: "utf8", timeout: 20_000, maxBuffer: 2 * 1024 * 1024 });
      const { actual, independent } = JSON.parse(output);
      expect(actual).toEqual(fixture.expected);
      expect(independent).toEqual(fixture.expected.results);
      expect(oracle.validate({ const: fixture.expected }, actual)).toBe(true);
    }, 30_000);
  });

  describe("rewriteJcoAsyncResultLifting", () => {
    it("checks the resolved callback memory", () => {
      const jcoGenerated = `function taskReturn(ctx) {
  const memory = ctx.getMemoryFn();
  if (!ctx.memory) {
      _debugLog('missing memory despite indirect param usage', { ctx });
  }
}`;
      const rewritten = rewriteJcoAsyncResultLifting(jcoGenerated);
      expect(rewritten).toContain("if (!memory) {");
      expect(rewriteJcoAsyncResultLifting(rewritten)).toBe(rewritten);
    });

    itLong("preserves direct descriptor and job results and lifts large turn results indirectly", async () => {
      const { execFileSync } = await import("node:child_process");
      const fixturePath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧫️fixtures/⏳️async-results.json");
      const generated = execFileSync("node", ["--input-type=module", "--eval", `
        import { parse, transpile } from "@bytecodealliance/jco";
        import { readFileSync } from "node:fs";
        const fixture = JSON.parse(readFileSync(process.argv[1], "utf8"));
        const results = [];
        for (const test of fixture.cases) {
          const wat = fixture.componentTemplate.replace("{{type}}", test.type).replace("{{params}}", test.params.join(" "));
          const { files } = await transpile(await parse(wat), { name: test.id, noTypescript: true });
          results.push({ ...test, source: new TextDecoder().decode(files[test.id + ".js"]) });
        }
        console.log(JSON.stringify(results));
      `, fixturePath], { cwd: repoRoot, encoding: "utf8", timeout: 20_000, maxBuffer: 8 * 1024 * 1024 });
      for (const test of JSON.parse(generated)) {
        const rewritten = rewriteJcoAsyncResultLifting(test.source);
        expect(rewritten.match(/taskReturn\.bind\([\s\S]*?useDirectParams: (true|false)/)?.[1], test.id).toBe(String(test.direct));
        expect(rewriteJcoAsyncResultLifting(rewritten), test.id).toBe(rewritten);
      }
    }, 30_000);
  });

  describe("rewritePreview2ShimImportSource", () => {
    itLong("rebases exact declared vendor URLs while preserving the public route", async () => {
      const ts = await import("typescript"), fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/🪞️vendor.json"), "utf8"));
      const printer = ts.createPrinter();
      for (const row of fixture.cases) {
        const actual = rewritePreview2ShimImportSource(row.input, fixture.prefix);
        expect(actual).toBe(row.output);
        const source = ts.createSourceFile("fixture.ts", row.input, ts.ScriptTarget.Latest, true);
        const transformed = ts.transform(source, [(context) => (node) => ts.visitEachChild(node, (child) => ts.isImportDeclaration(child) && ts.isStringLiteral(child.moduleSpecifier) && child.moduleSpecifier.text === row.inputModule ? ts.factory.updateImportDeclaration(child, child.modifiers, child.importClause, ts.factory.createStringLiteral(row.outputModule), child.attributes) : child, context)]);
        const actualTree = ts.createSourceFile("fixture.ts", actual, ts.ScriptTarget.Latest, true);
        expect(printer.printFile(transformed.transformed[0])).toBe(printer.printFile(ts.factory.updateSourceFile(actualTree, actualTree.statements.map((node) => ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier) ? ts.factory.updateImportDeclaration(node, node.modifiers, node.importClause, ts.factory.createStringLiteral(node.moduleSpecifier.text), node.attributes) : node))));
        transformed.dispose();
        if (row.outputModule.startsWith("..")) expect(decodeURIComponent(new URL(row.outputModule, `https://example.invalid${MODULE_EXTENSION_ROUTE}/🧮️flow-extension-math/component.js`).pathname)).toBe(MODULE_PLUGIN_ROUTE + "/" + fixture.vendorPath + "/" + basename(row.outputModule));
      }
    });
  });
  //#endregion 🔖️T-P8-tests
}
