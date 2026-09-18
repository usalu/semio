/** 🎬️ The hub's cross-plugin open, as the shell composes it: a session holding only the host plugin
 * cannot route another plugin's artifact kind, resolves that kind's owner from the catalog's declared
 * `on-artifact-kind:` rows, installs it, and routes the SAME open against the rebuilt router. */

import { describe, expect, it } from "vitest";
import { ActivationRegistry, AppRouter, type AppRouterManifest, type OpeningPreferences, activationEventEnvelope, activationReasonForAppId, artifactKindActivationOwner, type PluginCatalog, type PluginCatalogTarget } from "@semio-tech/framework";
import { ShardClient, type ShardBudget } from "../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
import { OwnedResidentLedger } from "../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️.ts";
import { resolveArtifactOpeningRelay } from "@semio-tech/framework-os";

const EMPTY_PREFERENCES: OpeningPreferences = { defaults: [] };

const hostManifest: AppRouterManifest = {
  pluginId: "hub",
  apps: [
    { id: "home", role: "editor", dialect: { artifactKind: "s.hub.home", standard: "1", subset: "*" } },
    { id: "home#viewer", role: "viewer", dialect: { artifactKind: "s.hub.home", standard: "1", subset: "*" } },
  ],
};

const guestManifest: AppRouterManifest = {
  pluginId: "beta",
  apps: [
    { id: "s.beta.sheet@1/*#editor", role: "editor", dialect: { artifactKind: "s.beta.sheet", standard: "1", subset: "*" } },
    { id: "s.beta.sheet@1/*#viewer", role: "viewer", dialect: { artifactKind: "s.beta.sheet", standard: "1", subset: "*" } },
  ],
};

const target = (pluginId: string, activationEvents: readonly string[]): PluginCatalogTarget => ({ pluginId, wasmOut: `${pluginId}.wasm`, role: "plugin", contributes: [], consumes: [], dependsOn: [], activationEvents });

const catalog: PluginCatalog = {
  plugins: [target("hub", ["on-artifact-kind:s.hub.home"]), target("beta", ["on-artifact-kind:2d.sheet", "on-artifact-kind:s.beta.sheet"]), target("gamma", ["on-artifact-kind:s.gamma.chart"])],
  extensions: [],
  hosts: [{ pluginId: "hub", landingAppId: "home", hostAppId: "home" }],
  playgrounds: [{ variant: "hub", pluginId: "hub", aliases: [] }],
  moduleUrl: (pluginId) => `/plugins/${pluginId}/bridge.js`,
  extensionModuleUrl: (pluginId) => `/extensions/${pluginId}/bridge.js`,
};

/** 🔌️ The shell's `installPlugin` narrowed to what this law needs: a registry entry becomes a loaded
 * manifest, and nothing else may. */
function install(loaded: AppRouterManifest[], pluginId: string): "loaded" | "missing-registry" {
  const available = [hostManifest, guestManifest].find((manifest) => manifest.pluginId === pluginId);
  if (!available || !catalog.plugins.some((row) => row.pluginId === pluginId)) return "missing-registry";
  loaded.push(available);
  return "loaded";
}

/** 📂️ `ShellHost`'s `installActivationOwnerAndResolve`, over injected state instead of React refs. */
function resolveWithActivation(loaded: AppRouterManifest[], actionId: string, args: Readonly<Record<string, unknown>>): { readonly pluginId: string; readonly appId: string; readonly installed: string | null } {
  const route = () => resolveArtifactOpeningRelay(actionId, args, AppRouter.build(loaded), EMPTY_PREFERENCES);
  try {
    const opening = route();
    return { pluginId: opening.app.pluginId, appId: opening.app.appId, installed: null };
  } catch (relayError) {
    const artifactKind = String(args.artifactRef).split("@")[0]!;
    const owner = artifactKindActivationOwner(catalog, artifactKind);
    if (owner === undefined || loaded.some((manifest) => manifest.pluginId === owner)) throw relayError;
    if (install(loaded, owner) !== "loaded") throw relayError;
    const opening = route();
    return { pluginId: opening.app.pluginId, appId: opening.app.appId, installed: owner };
  }
}

describe("hub cross-plugin artifact opening", () => {
  it("installs the kind's declared owner and opens it in the same session", () => {
    const loaded = [hostManifest];
    const outcome = resolveWithActivation(loaded, "os.open-artifact", { artifactRef: "s.beta.sheet@1/*", role: "editor", artifactId: "sheet-1", schema: "beta.sheet" });
    expect(outcome).toEqual({ pluginId: "beta", appId: "s.beta.sheet@1/*#editor", installed: "beta" });
    expect(loaded.map((manifest) => manifest.pluginId)).toEqual(["hub", "beta"]);
  });

  it("resolves the owner from the crate's own kind-spec spelling too", () => {
    expect(artifactKindActivationOwner(catalog, "2d.sheet")).toBe("beta");
    expect(artifactKindActivationOwner(catalog, "s.beta.sheet")).toBe("beta");
  });

  it("installs nothing when the kind's owner is already loaded", () => {
    const loaded = [hostManifest, guestManifest];
    const outcome = resolveWithActivation(loaded, "os.open-artifact", { artifactRef: "s.beta.sheet@1/*", role: "viewer" });
    expect(outcome).toEqual({ pluginId: "beta", appId: "s.beta.sheet@1/*#viewer", installed: null });
    expect(loaded).toHaveLength(2);
  });

  it("refuses a kind no catalog row claims, and installs nothing", () => {
    const loaded = [hostManifest];
    expect(() => resolveWithActivation(loaded, "os.open-artifact", { artifactRef: "s.delta.thing@1/*" })).toThrow(/surface/);
    expect(loaded).toHaveLength(1);
  });

  it("refuses a claimed kind whose owner cannot be installed, and installs nothing", () => {
    const loaded = [hostManifest];
    expect(() => resolveWithActivation(loaded, "os.open-artifact", { artifactRef: "s.gamma.chart@1/*" })).toThrow(/surface/);
    expect(loaded).toHaveLength(1);
  });
});

/** 🎬️ A `ShardWorkerLike` that auto-replies success to every request and keeps what it was sent — the
 * guest boundary, recorded. The `turn` message's `events` are exactly the `{kind, payload}` envelopes
 * `🟨️shard-worker.js` lifts into the guest's own `{tag, val}` WIT variant. */
function recordingShardWorker(sent: unknown[]) {
  const worker: { postMessage: (message: unknown) => void; terminate: () => void; onmessage: ((event: { readonly data: unknown }) => void) | null; onerror: ((event: unknown) => void) | null } = {
    postMessage: (message) => {
      sent.push(message);
      const requestId = (message as { readonly requestId?: string }).requestId;
      if (requestId) queueMicrotask(() => worker.onmessage?.({ data: { kind: "result", requestId, ok: true, value: undefined } }));
    },
    terminate: () => {},
    onmessage: null,
    onerror: null,
  };
  return worker;
}

const ACTIVATION_BUDGET: ShardBudget = { fuel: 1000, wallMs: 4, memoryBytes: 1 << 20, uiNodes: 100, mailboxLen: 16, maxEffects: 8, maxPatchBytes: 1 << 16 };

async function flushMicrotasks(ticks = 16): Promise<void> {
  for (let index = 0; index < ticks; index += 1) await Promise.resolve();
}

describe("hub cross-plugin activation reason", () => {
  it("derives on-artifact-kind:<kind> from the app the owner opens, and manual from a landing app", () => {
    const loaded = [hostManifest];
    const outcome = resolveWithActivation(loaded, "os.open-artifact", { artifactRef: "s.beta.sheet@1/*", role: "editor", artifactId: "sheet-1", schema: "beta.sheet" });
    expect(activationReasonForAppId(outcome.appId)).toBe("on-artifact-kind:s.beta.sheet");
    expect(activationReasonForAppId("home")).toBe("manual");
  });

  it("marshals that reason onto the WIT activate event the guest receives, and nothing for manual", () => {
    expect(activationEventEnvelope("on-artifact-kind:s.beta.sheet")).toEqual({ kind: "activate", payload: { instance: 0, reason: { tag: "on-artifact-kind", val: "s.beta.sheet" } } });
    expect(activationEventEnvelope("on-extension-request:beta")).toEqual({ kind: "activate", payload: { instance: 0, reason: { tag: "on-extension-request", val: "beta" } } });
    expect(activationEventEnvelope("on-startup-finished")).toEqual({ kind: "activate", payload: { instance: 0, reason: { tag: "on-startup-finished" } } });
    expect(activationEventEnvelope("manual")).toBeUndefined();
  });

  it("sends the kind-derived activate event to the freshly activated guest, and sends none for a manual activation", async () => {
    const kindSent: unknown[] = [];
    const kindRegistry = new ActivationRegistry({
      shardClient: new ShardClient({ residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }), shardCount: 1, createWorker: () => recordingShardWorker(kindSent) }),
      defaultBudget: ACTIVATION_BUDGET,
      fetchAssets: async () => [],
    });
    kindRegistry.registerManifest({ pluginId: "beta", moduleUrl: "https://x/beta.js", caps: [] });
    await kindRegistry.activate("beta", "beta#1", activationReasonForAppId("s.beta.sheet@1/*#editor"));
    await flushMicrotasks();
    const kindTurns = kindSent.filter((message) => (message as { kind?: string }).kind === "turn") as { readonly events: readonly { readonly kind: string; readonly payload: unknown }[] }[];
    expect(kindTurns.flatMap((turn) => turn.events)).toEqual([{ kind: "activate", payload: { instance: 0, reason: { tag: "on-artifact-kind", val: "s.beta.sheet" } } }]);

    const manualSent: unknown[] = [];
    const manualRegistry = new ActivationRegistry({
      shardClient: new ShardClient({ residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }), shardCount: 1, createWorker: () => recordingShardWorker(manualSent) }),
      defaultBudget: ACTIVATION_BUDGET,
      fetchAssets: async () => [],
    });
    manualRegistry.registerManifest({ pluginId: "hub", moduleUrl: "https://x/hub.js", caps: [] });
    await manualRegistry.activate("hub", "hub#1", activationReasonForAppId("home"));
    await flushMicrotasks();
    expect(manualSent.filter((message) => (message as { kind?: string }).kind === "turn")).toEqual([]);
  });
});
