// @vitest-environment node
import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import emojiRegex from "emoji-regex";
import ts from "typescript";
import { loadTaxonomy, parseCanonicalWgpuPackageCatalog, parseSemanticPackageBrowserProfile } from "../../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import browserAuthorityFixture from "../../🧫️fixtures/🧊️wgpu-browser-entry-authority/🔣️.json";
import rendererSchema from "../../../🧬️schema/🔣️.json";
import { renderFrameWorker } from "../../🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script";
import { assertPinnedBunVersion, decodeAstralEscapes, renderBrowserEntry } from "../../🎯️targets/🧊️wgpu/⚙️browser-build/🟦️.ts";
import { decodeInvocationPayloads, pluginHandleForBridge, reconcileRetainedWindowPatch, retireWgpuOwnedUiInstanceLifecycle, settleFailedInstanceOpen, WgpuOwnedUiInstanceRoute, type WgpuPluginHandle } from "../../🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts";
import { resolvePlaygroundBoot } from "@semio-tech/framework";
import { PLUGIN_CATALOG } from "../../../../🔌️plugin/📇️registry/🟦️.ts";
import bootSelectionFixture from "../../🧫️fixtures/🔬️wgpu-shell-boot-selection/🔣️.json";
import { coerceTurnResult } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts";

function fakeHandle(overrides: Partial<WgpuPluginHandle> = {}): WgpuPluginHandle {
  return {
    pluginId: "draw",
    manifest: { pluginId: "draw", label: "Draw", version: "0.1.0", apps: [], workflows: [], examples: [] },
    createApp: async () => 1,
    destroyApp: async () => {},
    readWindowConfigPacks: async () => [],
    handleAction: async () => ({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } }),
    handleCommand: async () => ({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } }),
    render: async () => ({ type: "text", value: "hello" }),
    renderDocument: async () => "document",
    contextMenu: async () => [],
    dispose: async () => {},
    ...overrides,
  };
}

describe("framework renderer wgpu plugin bridge", () => {
  it("builds a JS bridge whose manifest() is synchronous JSON, matching ProgramBridge.rs's Reflect::get(handle, \"manifest\") contract", () => {
    const bridge = pluginHandleForBridge(fakeHandle());
    expect(JSON.parse(bridge.manifest()).pluginId).toBe("draw");
  });

  it("forwards createApp/destroyApp by identity", async () => {
    const created: string[] = [];
    const bridge = pluginHandleForBridge(fakeHandle({ createApp: async (appId) => (created.push(appId), 7), destroyApp: async () => void created.push("destroyed") }));
    expect(await bridge.createApp("s")).toBe(7);
    await bridge.destroyApp(7);
    expect(created).toEqual(["s", "destroyed"]);
  });

  it("unwraps handleAction's contextJson third argument down to just its viewState before calling the typed handle — ProgramBridge.rs passes {viewState, actor} JSON, not the bare view state", async () => {
    let seenViewState: unknown;
    const bridge = pluginHandleForBridge(
      fakeHandle({
        handleAction: async (_instanceId, _actionJson, viewState) => {
          seenViewState = viewState;
          return { output: "ok", mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } };
        },
      }),
    );
    const result = await bridge.handleAction(1, "{}", JSON.stringify({ viewState: { zoom: 2 }, actor: "local" }));
    expect(seenViewState).toEqual({ zoom: 2 });
    expect(JSON.parse(result).output).toBe("ok");
  });

  it("bridges render() through JSON round-tripping", async () => {
    const bridge = pluginHandleForBridge(fakeHandle());
    const result = await bridge.render(1, "window", "body", JSON.stringify({}));
    expect(JSON.parse(result)).toEqual({ type: "text", value: "hello" });
  });

  it("retains exact turn identity and private receipt bytes for the WGPU owner route", () => {
    const lifecycleReceipt = Uint8Array.of(1, 2, 3);
    const uiPatchReceipt = Uint8Array.of(4, 5, 6);
    const raw = { lifecycleReceipt, uiPatchReceipt, uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" }, coldPairIngress: { tag: "idle" } };
    const decoded = coerceTurnResult(raw);
    expect(decoded.original).toBe(raw);
    expect(decoded.lifecycleReceipt).toBe(lifecycleReceipt);
    expect(decoded.uiPatchReceipt).toBe(uiPatchReceipt);
    expect(decoded.status).toEqual({ tag: "idle" });
    for (const invalid of [[1, 2], "AQI=", { bytes: [1, 2] }]) {
      expect(() => coerceTurnResult({ ...raw, lifecycleReceipt: invalid })).toThrow("actor-lifecycle.receipt-bytes");
      expect(() => coerceTurnResult({ ...raw, uiPatchReceipt: invalid })).toThrow("actor-ui-patch.receipt-bytes");
    }
  });

  it("composes one nonempty WGPU surface through exact patch acknowledgement and terminal owner retirement", async () => {
    const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧫️fixtures/⏱️lifecycle-scheduler.json");
    const { default: equal } = await import("fast-deep-equal");
    const { encodePackValue } = await import("@semio-tech/framework-os");
    const { OwnedResidentLedger } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️.ts");
    const { ShardClient } = await import("../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts");
    const { DEFAULT_SHARD_BUDGET } = await import("../../../../../../../🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts");
    const { encodeActorInstanceLifecycle } = await import("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts");
    const { encodeActorUiPatchReceipt } = await import("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts");
    const validate = new Ajv({ strict: true }).addSchema(rendererSchema).getSchema(`${rendererSchema.$id}#/$defs/PluginRuntimeLifecycleSchedulerV1`)!;
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const sent: string[] = [];
    const plain = { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" }, coldPairIngress: { tag: "idle" } };
    const node = { id: 0, key: "root", component: { type: "text", value: "owned", emphasize: null, dataAttributes: null }, layout: { kind: "leaf", width: "hug", height: "hug" }, style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, activity: "idle", disabled: false, transition: null, accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false }, bindings: [], menu: null, children: [] };
    let lifetime: { readonly activationGeneration: bigint; readonly instanceId: number; readonly guestLifetime: bigint } | null = null;
    const worker = {
      onmessage: null as ((event: { readonly data: unknown }) => void) | null,
      onerror: null as ((event: unknown) => void) | null,
      postMessage(raw: unknown) {
        const message = raw as { readonly kind: string; readonly requestId?: string; readonly events?: readonly { readonly kind: string; readonly payload?: unknown }[] };
        if (message.kind === "dispose") { sent.push("dispose"); return; }
        if (!message.requestId) return;
        let value: unknown = undefined;
        if (message.kind === "turn") {
          const first = message.events?.[0];
          if (first) sent.push(first.kind);
          if (first?.kind === "instance-open") {
            const payload = first.payload as { readonly instance: number; readonly activationGeneration: bigint; readonly requestSequence: number };
            lifetime = { activationGeneration: payload.activationGeneration, instanceId: payload.instance, guestLifetime: BigInt(fixture.guestLifetime) };
            value = {
              ...plain,
              lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "captured", lifetime, requestSequence: payload.requestSequence }),
              uiPatchReceipt: encodeActorUiPatchReceipt({ lifetime, patchSequence: BigInt(fixture.uiAcknowledgement.patchSequence) }),
              uiPatches: [{ surface: { instance: payload.instance, surface: fixture.runtimeUiComposition.surface }, revision: 1n, baseRevision: 0n, ops: [{ tag: "upsert", val: { node: encodePackValue(node) } }, { tag: "set-root", val: 0n }] }],
            };
          } else if (first?.kind === "instance-close") {
            const payload = first.payload as { readonly requestSequence: number };
            value = { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "accepted", lifetime: lifetime!, requestSequence: payload.requestSequence, closeGeneration: BigInt(fixture.closeGeneration) }) };
          } else if (first?.kind === "instance-lifecycle-ack") {
            const receipt = (first.payload as { readonly receipt: { readonly kind: string; readonly requestSequence: number } }).receipt;
            value = receipt.kind === "accepted" ? { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "retired", lifetime: lifetime!, requestSequence: receipt.requestSequence, closeGeneration: BigInt(fixture.closeGeneration) }) } : plain;
          } else value = plain;
        }
        queueMicrotask(() => worker.onmessage?.({ data: { kind: "result", requestId: message.requestId, ok: true, value } }));
      },
      terminate() {},
    };
    const client = new ShardClient({ residentLedger: new OwnedResidentLedger({ bytes: 1_048_576, slots: 4_096, owners: 4_096, control: { bytes: 65_536, slots: 256, owners: 256 } }), shardCount: 1, createWorker: () => worker });
    const actorId = "wgpu-owned-ui#1";
    await client.activate(actorId, "/fixture.js", [], DEFAULT_SHARD_BUDGET);
    const lifecycle = client.captureInstanceLifecycle(actorId, 1);
    const execute = <T>(work: () => Promise<T>): Promise<T> => work();
    const opened = coerceTurnResult(await lifecycle.open({ appId: "fixture", actor: "local", config: [], assets: [], capabilities: [], quotas: [] }, DEFAULT_SHARD_BUDGET));
    const route = new WgpuOwnedUiInstanceRoute(lifecycle);
    await route.accept(opened, execute);
    const captured = lifecycle.pendingReceipt;
    if (!captured || captured.kind !== "captured") throw new Error("fixture captured receipt missing");
    await lifecycle.acknowledge(captured, DEFAULT_SHARD_BUDGET);
    expect(lifecycle.progress().kind).toBe("open");
    const projected = await route.project(fixture.runtimeUiComposition.surface);
    expect(projected?.node).toMatchObject({ key: "root", component: { type: "text", value: "owned" }, children: [] });
    expect(projected?.document).toMatchObject({ surface: fixture.runtimeUiComposition.surface, revision: 1, root: 0, nodes: [{ id: 0, key: "root" }] });
    expect(sent).toEqual(fixture.runtimeUiComposition.openEvents);
    expect(equal(sent, fixture.runtimeUiComposition.openEvents)).toBe(true);
    const beforeClose = sent.length;
    lifecycle.beginClose();
    await lifecycle.close(DEFAULT_SHARD_BUDGET);
    const accepted = lifecycle.pendingReceipt;
    if (!accepted || accepted.kind !== "accepted") throw new Error("fixture accepted receipt missing");
    await lifecycle.acknowledge(accepted, DEFAULT_SHARD_BUDGET);
    const retired = lifecycle.pendingReceipt;
    if (!retired || retired.kind !== "retired") throw new Error("fixture retired receipt missing");
    const witness = await route.retire();
    await lifecycle.acknowledge(retired, DEFAULT_SHARD_BUDGET, witness);
    lifecycle.dispose();
    expect(route.terminalIsEmpty).toBe(true);
    expect(lifecycle.progress().kind).toBe("complete");
    expect(sent.slice(beforeClose)).toEqual(fixture.runtimeUiComposition.closeEvents);
    expect(equal(sent.slice(beforeClose), fixture.runtimeUiComposition.closeEvents)).toBe(true);

    const cancelledActorId = "wgpu-owned-ui-cancelled#2";
    await client.activate(cancelledActorId, "/fixture.js", [], DEFAULT_SHARD_BUDGET);
    const cancelledLifecycle = client.captureInstanceLifecycle(cancelledActorId, 2);
    const cancelledOpen = coerceTurnResult(await cancelledLifecycle.open({ appId: "fixture", actor: "local", config: [], assets: [], capabilities: [], quotas: [] }, DEFAULT_SHARD_BUDGET));
    const cancelledRoute = new WgpuOwnedUiInstanceRoute(cancelledLifecycle);
    await cancelledRoute.accept(cancelledOpen, execute);
    expect(cancelledLifecycle.pendingReceipt?.kind).toBe("captured");
    await retireWgpuOwnedUiInstanceLifecycle(cancelledLifecycle, cancelledRoute, execute);
    expect(cancelledLifecycle.progress().kind).toBe("complete");
    expect(cancelledRoute.terminalIsEmpty).toBe(true);
    client.disposeAll();
  });
});

describe("framework renderer wgpu pack integer carriers", () => {
  it("projects lossless pack integer carriers and WIT bigint revisions off the retained-window patch boundary", async () => {
    const { encodePackValue, packUInt } = await import("@semio-tech/framework-os");
    const node = { id: packUInt(7n), key: "leaf-7", component: { type: "text", value: "a" }, children: [packUInt(9n)] };
    const first = reconcileRetainedWindowPatch(null, { revision: 1n as unknown as number, baseRevision: 0n as unknown as number, ops: [{ tag: "replace", val: { path: [], node: Array.from(encodePackValue(node)) } }] });
    expect(first.desynced).toBe(false);
    expect(first.surface).toEqual({ revision: 1, node: { id: 7, key: "leaf-7", component: { type: "text", value: "a" }, children: [9] } });
    const second = reconcileRetainedWindowPatch(first.surface, { revision: 2n as unknown as number, baseRevision: 1n as unknown as number, ops: [{ tag: "replace", val: { path: [], node: Array.from(encodePackValue({ ...node, key: "leaf-7b" })) } }] });
    expect(second.desynced).toBe(false);
    expect((second.surface?.node as { key: string }).key).toBe("leaf-7b");
  });

  it("rejects a pack integer no JSON number represents exactly instead of rounding it", async () => {
    const { encodePackValue, packUInt } = await import("@semio-tech/framework-os");
    const ops = [{ tag: "replace", val: { path: [], node: Array.from(encodePackValue({ id: packUInt(2n ** 60n) })) } }];
    expect(() => reconcileRetainedWindowPatch(null, { revision: 1n as unknown as number, baseRevision: 0n as unknown as number, ops })).toThrow(/\$\.id/u);
    expect(() => reconcileRetainedWindowPatch(null, { revision: 2n ** 60n as unknown as number, baseRevision: 0n as unknown as number, ops: [] })).toThrow(/uiPatch\.revision/u);
  });

  it("projects every pack payload of an Invocation reply frame, matching an independent JSON.parse oracle", async () => {
    const { encodePackValue, packUInt } = await import("@semio-tech/framework-os");
    const output = { revision: packUInt(9007199254740991n), label: "done" };
    const diagnostics = [{ severity: "info", code: packUInt(3n) }];
    const decoded = decodeInvocationPayloads({
      output: Array.from(encodePackValue(output)),
      diagnostics: Array.from(encodePackValue(diagnostics)),
      ui_scope: Array.from(encodePackValue({ surfaces: [packUInt(1n)] })),
      history_patch: Array.from(encodePackValue({ revision: packUInt(2n) })),
      mutations: Array.from(encodePackValue([])),
      inverse_group: Array.from(encodePackValue({ invocationId: "", mutations: [], inverseMutations: [] })),
    });
    expect(decoded.output).toEqual(JSON.parse('{"revision":9007199254740991,"label":"done"}'));
    expect(decoded.diagnostics).toEqual(JSON.parse('[{"severity":"info","code":3}]'));
    expect(decoded.uiScope).toEqual(JSON.parse('{"surfaces":[1]}'));
    expect(decoded.historyPatch).toEqual(JSON.parse('{"revision":2}'));
    expect(decoded.mutations).toEqual([]);
    expect(decoded.inverseGroup).toEqual({ invocationId: "", mutations: [], inverseMutations: [] });
    expect(JSON.stringify(decoded.output)).not.toContain("kind");
  });
});

describe("framework renderer wgpu generated worker", () => {
  it("activates only the current no-follow package through both independent TypeScript compilers", () => {
    const taxonomy = loadTaxonomy(), contract = taxonomy.generatorContracts["wgpu-frame-worker"]!;
    const directory = dirname(fileURLToPath(import.meta.url));
    const source = readFileSync(join(directory, "../../../../../../🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts"), "utf8");
    const definition = source.match(/^function packageGeneratorActivated\([\s\S]*?^\}/mu)![0];
    const catalogBytes = readFileSync(join(directory, "../../🎯️targets/🧊️wgpu/🪪️package-catalog.json"), "utf8");
    const catalog = parseCanonicalWgpuPackageCatalog(catalogBytes, contract.packageGeneration!.catalogSha256, contract.packageGeneration!.browserProfile, taxonomy);
    const manifest = `${catalog.ownerPath}/${catalog.packageRelativePath}/Cargo.toml`;
    for (const compile of [
      (code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code),
      (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText,
    ]) for (const scenario of browserAuthorityFixture.activationCases) {
      const visited: string[] = [];
      const support = {
        assertLexicalInputOutsideOpaque: (_root: string, path: string) => path,
        lstatOrNull: (path: string) => {
          visited.push(path);
          const kind = path === contract.packageGeneration!.catalogPath ? scenario.catalog : path === manifest ? scenario.manifest : "absent";
          return kind === "absent" ? null : { isFile: () => kind === "file", isSymbolicLink: () => kind === "symlink" };
        },
        readFileSync: () => catalogBytes,
        parseCanonicalWgpuPackageCatalog,
        parseSemanticPackageBrowserProfile,
      };
      const activate = new Function(...Object.keys(support), `${compile(definition)}\nreturn packageGeneratorActivated;`)(...Object.values(support));
      let observed = false;
      try { observed = activate(".", [], contract, { discoverySchema: taxonomy }, []); } catch {}
      expect(observed, scenario.id).toBe(scenario.expected);
      expect(visited.every((path) => path === contract.packageGeneration!.catalogPath || path === manifest), scenario.id).toBe(true);
    }
  });

  it("validates the current package catalog with Ajv and independent WebCrypto integrity vectors", async () => {
    const taxonomy = loadTaxonomy(), generation = taxonomy.generatorContracts["wgpu-frame-worker"]!.packageGeneration!;
    const bytes = readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🎯️targets/🧊️wgpu/🪪️package-catalog.json"), "utf8");
    const catalogExport = new Ajv({ strict: true, allErrors: true }).addKeyword("x-semio-note").addSchema(rendererSchema)
      .getSchema(`${rendererSchema.$id}#/$defs/RendererPackageCatalogV1`)!;
    expect(catalogExport(JSON.parse(bytes)), JSON.stringify(catalogExport.errors)).toBe(true);
    for (const scenario of browserAuthorityFixture.catalogCases) {
      const content = bytes + scenario.suffix;
      const digest = Buffer.from(await crypto.subtle.digest("SHA-256", Buffer.from(content))).toString("hex");
      expect(digest === generation.catalogSha256, scenario.id).toBe(scenario.expected);
      if (scenario.expected) expect(parseCanonicalWgpuPackageCatalog(content, generation.catalogSha256, generation.browserProfile, taxonomy).artifacts).toHaveLength(4);
      else expect(() => parseCanonicalWgpuPackageCatalog(content, generation.catalogSha256, generation.browserProfile, taxonomy)).toThrow(/digest drift/);
    }
    const changed = JSON.parse(bytes);
    changed.artifacts[0].targetRelativePath = "⌨️native-entrypoint/🦀️.rs";
    const changedBytes = JSON.stringify(changed), digest = createHash("sha256").update(changedBytes).digest("hex");
    expect(() => parseCanonicalWgpuPackageCatalog(changedBytes, digest, generation.browserProfile, taxonomy)).toThrow(/target drift/);
  });

  it("validates explicit browser entry identities against neutral vectors and independent Ajv/emoji parsing", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).addKeyword("x-semio-note").addSchema(rendererSchema)
      .getSchema(`${rendererSchema.$id}#/$defs/RendererPackageCatalogBrowserEntries`)!;
    const template = loadTaxonomy().generatorContracts["wgpu-frame-worker"]!.packageGeneration!.browserProfile;
    for (const scenario of browserAuthorityFixture.cases) {
      const entries = structuredClone(browserAuthorityFixture.entries) as Record<string, unknown>[];
      for (const change of scenario.changes) {
        if (change.value === null) delete entries[change.entry]![change.field];
        else entries[change.entry]![change.field] = change.value;
      }
      const identities = entries.map((entry) => {
        const directory = String(entry.sourceRelativePath).split("/")[0]!;
        const matches = [...directory.matchAll(emojiRegex())];
        return { directory, matches, emoji: matches[0]?.[0].replaceAll("\uFE0F", "") };
      });
      const oracle = Boolean(validate(entries)) && entries.every((entry, index) => {
        const identity = identities[index]!;
        return identity.matches.length === 1 && identity.matches[0]!.index === 0 && !["📁", "📂", "📄"].includes(identity.emoji!) && identity.directory.replace(emojiRegex(), "").replaceAll("\uFE0F", "") === entry.id && entry.sourceRelativePath === `${identity.directory}/🟦️.ts` && entry.outputRelativePath === `${identity.directory}/🤖️generated/🟨️.js` && entry.id === (index === 0 ? "frame-worker" : "browser-boot") && entry.inclusion === (index === 0 ? "tracked" : "ignored");
      }) && new Set(identities.map((entry) => entry.emoji)).size === entries.length;
      expect(oracle, scenario.id).toBe(scenario.expected);
      const profile = { ...template, ownerPath: "🧊️fixture", entries, sourceModulePaths: [...new Set([...template.sourceModulePaths, ...entries.map((entry) => `🧊️fixture/${entry.sourceRelativePath}`)])].sort((left, right) => Buffer.compare(Buffer.from(left), Buffer.from(right))) };
      if (scenario.expected) expect(() => parseSemanticPackageBrowserProfile(profile, loadTaxonomy().pathEmojiPolicy.genericEmojiIdentities), scenario.id).not.toThrow();
      else expect(() => parseSemanticPackageBrowserProfile(profile, loadTaxonomy().pathEmojiPolicy.genericEmojiIdentities), scenario.id).toThrow();
    }
  });

  it("fails closed unless the renderer uses the repository-pinned Bun runtime", () => {
    expect(assertPinnedBunVersion()).toBe(Bun.version);
    expect(() => assertPinnedBunVersion("0.0.0")).toThrow(/requires Bun/);
  });

  it("renders identical bytes twice with matching independent SHA-256 implementations", async () => {
    const bundleRoot = join(dirname(fileURLToPath(import.meta.url)), "../../🎯️targets/🧊️wgpu/📦️packages/🦀️rust");
    const first = await renderFrameWorker(bundleRoot);
    const second = await renderFrameWorker(bundleRoot);
    const subtle = Buffer.from(await crypto.subtle.digest("SHA-256", Buffer.from(first.content))).toString("hex");
    expect(second).toEqual(first);
    expect(createHash("sha256").update(first.content).digest("hex")).toBe(subtle);
  });

  it("renders byte-identical worker bytes from unrelated working directories and renderer environments, because the Nx generate target runs in the package directory while the dev lane checks from the repository root", async () => {
    const bundleRoot = join(dirname(fileURLToPath(import.meta.url)), "../../🎯️targets/🧊️wgpu/📦️packages/🦀️rust");
    let workspaceRoot = dirname(fileURLToPath(import.meta.url));
    while (!existsSync(join(workspaceRoot, "nx.json"))) workspaceRoot = dirname(workspaceRoot);
    const contexts = [
      { cwd: workspaceRoot, env: {} },
      { cwd: bundleRoot, env: { SEMIO_RENDERER: "wgpu", S_OS_PORT: "6118", CARGO_TARGET_DIR: join(workspaceRoot, "target-cross-context-probe") } },
      { cwd: dirname(workspaceRoot), env: { SEMIO_RENDERER: "react", S_OS_PORT: "6018", NODE_ENV: "production" } },
    ];
    const callerCwd = process.cwd();
    const renders: string[] = [];
    try {
      for (const context of contexts) {
        const restored = Object.entries(context.env).map(([key, value]) => [key, process.env[key]] as const);
        Object.assign(process.env, context.env);
        process.chdir(context.cwd);
        try {
          renders.push((await renderFrameWorker(bundleRoot)).content);
          expect(process.cwd()).toBe(context.cwd);
        } finally {
          for (const [key, value] of restored) if (value === undefined) delete process.env[key];
            else process.env[key] = value;
        }
      }
    } finally {
      process.chdir(callerCwd);
    }
    for (const render of renders) expect(createHash("sha256").update(render).digest("hex")).toBe(createHash("sha256").update(renders[0]!).digest("hex"));
    expect(renders[0]).not.toMatch(/[0-9a-f]{64}/u);
  });

  it("aligns digest-verified devcontainer and native Bun provisioning with the packageManager pin", () => {
    let repoRoot = dirname(fileURLToPath(import.meta.url));
    while (!existsSync(join(repoRoot, "nx.json"))) repoRoot = dirname(repoRoot);
    const dockerfile = readFileSync(join(repoRoot, ".devcontainer/Dockerfile"), "utf8");
    const packageManager = JSON.parse(readFileSync(join(repoRoot, "package.json"), "utf8")).packageManager;
    const pinnedVersion = /^bun@(\d+\.\d+\.\d+)$/u.exec(packageManager)?.[1];
    const configuration = ts.parseConfigFileTextToJson("devcontainer.json", readFileSync(join(repoRoot, ".devcontainer/devcontainer.json"), "utf8"));
    const nativeBootstrapPath = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh");
    const nativeBootstrap = readFileSync(nativeBootstrapPath, "utf8");
    expect(pinnedVersion).toBe(Bun.version);
    expect(/^ARG BUN_VERSION=(\d+\.\d+\.\d+)$/mu.exec(dockerfile)?.[1]).toBe(pinnedVersion);
    expect(configuration.error).toBeUndefined();
    expect(configuration.config.postCreateCommand).toEqual(["bun", "nx", "run", "workspace:deps-javascript"]);
    expect(dockerfile).not.toContain("bun.sh/install");
    expect(dockerfile).toContain("sha256sum -c -");
    expect([...dockerfile.matchAll(/bun_sha="([0-9a-f]{64})"/gu)]).toHaveLength(2);
    expect(dockerfile).toContain('test "$(bun --version)" = "$BUN_VERSION"');
    expect(nativeBootstrap).toContain('"packageManager"');
    expect(nativeBootstrap).toContain('bash -s "bun-v$required_version"');
    expect(nativeBootstrap).not.toContain(Bun.version);
    if (process.platform !== "win32") {
      execFileSync("bash", ["-n", nativeBootstrapPath]);
    }
  });

  it("decodeAstralEscapes matches JSON.parse (an independent, spec-compliant \\uXXXX decoder) on every well-formed surrogate pair, and is a no-op on plain ASCII/BMP text", () => {
    const cases = ["\\uD83E\\uDDF0️framework/\\uD83D\\uDD28️modules/\\uD83D\\uDDBC️assets", "no escapes here", "café — 日本語 — é", "\\uD83C\\uDF31️metabolism/\\uD83C\\uDFA8️representation"];
    for (const input of cases) {
      const viaOracle = JSON.parse(`"${input}"`) as string;
      expect(decodeAstralEscapes(input)).toBe(viaOracle);
    }
  });

  it("settleFailedInstanceOpen runs the instance cleanup and still rejects with the ORIGINAL open fault, even when the cleanup itself throws — the masking that turned every guest first-step trap into create_app promise failed: wgpu-ui.native-owner-required", async () => {
    const opened = new Error("shard 0 worker fault [handler/first-step] interactive-job.catalog-authority");
    const ran: string[] = [];
    await expect(settleFailedInstanceOpen(opened, async () => void ran.push("clean"))).rejects.toBe(opened);
    await expect(
      settleFailedInstanceOpen(opened, async () => {
        ran.push("threw");
        throw new Error("wgpu-ui.native-owner-required");
      }),
    ).rejects.toBe(opened);
    expect(ran).toEqual(["clean", "threw"]);
  });

  it("hands the wgpu shell its generation3d boot plan in dependency order, so the requested plugin is never plugins[0] — the ordering the Rust boot selection (🧫️fixtures/🔬️wgpu-shell-boot-selection) must survive", () => {
    const boot = resolvePlaygroundBoot(PLUGIN_CATALOG, "generation3d");
    const ids = boot.plugins.map((entry) => entry.pluginId);
    expect(ids).toContain("procedural");
    expect(ids).toContain("flow");
    expect(ids.indexOf("flow")).toBeLessThan(ids.indexOf("procedural"));
    expect(ids[0]).not.toBe("procedural");
    const fixtureCase = bootSelectionFixture.cases.find((entry) => entry.id === "dependency-sorts-first");
    expect(fixtureCase?.programs.map((program) => program.pluginId)).toEqual(["flow", "procedural"]);
    expect(fixtureCase?.expected?.appId).toBe(boot.defaultAppId);
  });

  it("renders an astral-emoji-bearing browser entry (🟦️.ts, which references the \"🎞️frame-worker.js\" filename by URL) with the emoji as literal UTF-8, not Bun's astral \\uXXXX surrogate-pair escapes — otherwise the reference scanner cannot see or rewrite it", async () => {
    const bundleRoot = dirname(fileURLToPath(import.meta.url));
    const content = await renderBrowserEntry(join(bundleRoot, "../../🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts"));
    expect(content).toContain("🎞️frame-worker.js");
    expect(content).not.toMatch(/\\u[Dd][89abAB][0-9a-fA-F]{2}/);
  });
});
