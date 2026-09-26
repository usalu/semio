/** 🧩️ The browser wgpu shell's lazy plugin source: the frame Worker's own module door. The eager boot
 * mount names only the boot plan's plugins, so a hub asked to open a kind whose owner is outside that
 * plan had no module source at all in the browser — these are the laws that source must keep. */

import { describe, expect, it } from "vitest";
import { createLazyPluginInstallDoor, type LazyPluginInstallPhase } from "../../🎯️targets/🧊️wgpu/🎞️frame-worker/🧩️lazy-install/🟦️.ts";
import { WGPU_DYNAMIC_EXTENSION_INSTALL_GLOBAL, WGPU_DYNAMIC_EXTENSION_RETIRE_GLOBAL, admitWgpuDynamicExtension, installWgpuDynamicExtensionDoor } from "../../🎯️targets/🧊️wgpu/🧩️dynamic-extension/🟦️.ts";
import extensionFixture from "../../../../🔌️plugin/🏪️store/📥️installation/🧫️fixtures/🔣️.json";

type Deferred = { readonly promise: Promise<unknown>; resolve: (value: unknown) => void; reject: (error: Error) => void };

function deferred(): Deferred {
  let resolve!: (value: unknown) => void;
  let reject!: (error: Error) => void;
  const promise = new Promise<unknown>((resolvePromise, rejectPromise) => {
    resolve = resolvePromise;
    reject = rejectPromise;
  });
  return { promise, resolve, reject };
}

const CATALOG: Record<string, string> = { cad: "/plugins/cad/bridge.js", beta: "/plugins/beta/bridge.js" };

function harness() {
  const mounts: { pluginId: string; moduleUrl: string; gate: Deferred }[] = [];
  const phases: { pluginId: string; phase: LazyPluginInstallPhase }[] = [];
  const door = createLazyPluginInstallDoor({
    moduleUrl: (pluginId) => CATALOG[pluginId],
    mount: (pluginId, moduleUrl) => {
      const gate = deferred();
      mounts.push({ pluginId, moduleUrl, gate });
      return gate.promise;
    },
    progress: (pluginId, phase) => phases.push({ pluginId, phase }),
  });
  return { door, mounts, phases };
}

describe("frame-worker lazy plugin install door", () => {
  it("mounts one plugin from its catalog module url and reports every phase", async () => {
    const { door, mounts, phases } = harness();
    const install = door.install("cad");
    expect(door.pending()).toEqual(["cad"]);
    expect(mounts).toHaveLength(1);
    expect(mounts[0]!.moduleUrl).toBe("/plugins/cad/bridge.js");
    mounts[0]!.gate.resolve({ pluginId: "cad" });
    await expect(install).resolves.toEqual({ pluginId: "cad" });
    expect(phases.map((entry) => entry.phase)).toEqual(["resolving", "loading", "mounted"]);
    expect(door.pending()).toEqual([]);
  });

  it("refuses a plugin the catalog does not carry without ever mounting", async () => {
    const { door, mounts, phases } = harness();
    await expect(door.install("gamma")).rejects.toThrow(/plugin-install\.unknown-plugin: gamma/);
    expect(mounts).toHaveLength(0);
    expect(phases.map((entry) => entry.phase)).toEqual(["resolving", "failed"]);
    expect(door.pending()).toEqual([]);
  });

  it("joins a second request for the same plugin to the first fetch", async () => {
    const { door, mounts } = harness();
    const first = door.install("cad");
    const second = door.install("cad");
    expect(mounts).toHaveLength(1);
    mounts[0]!.gate.resolve({ pluginId: "cad" });
    await expect(first).resolves.toEqual({ pluginId: "cad" });
    await expect(second).resolves.toEqual({ pluginId: "cad" });
  });

  it("settles a cancelled request immediately, leaving the fetch to finish into the loader cache", async () => {
    const { door, mounts, phases } = harness();
    const install = door.install("cad");
    expect(door.cancel("cad")).toBe(true);
    await expect(install).rejects.toThrow(/plugin-install\.cancelled: cad/);
    expect(phases.map((entry) => entry.phase)).toEqual(["resolving", "loading", "cancelled"]);
    expect(door.pending()).toEqual([]);
    mounts[0]!.gate.resolve({ pluginId: "cad" });
    await expect(mounts[0]!.gate.promise).resolves.toEqual({ pluginId: "cad" });
    expect(door.cancel("cad")).toBe(false);
    const again = door.install("cad");
    expect(mounts).toHaveLength(2);
    mounts[1]!.gate.resolve({ pluginId: "cad" });
    await expect(again).resolves.toEqual({ pluginId: "cad" });
  });

  it("surfaces a mount fault as a failure, not a cancellation, and retains nothing", async () => {
    const { door, phases, mounts } = harness();
    const install = door.install("beta");
    mounts[0]!.gate.reject(new Error("module fetch 404"));
    await expect(install).rejects.toThrow(/module fetch 404/);
    expect(phases.map((entry) => entry.phase)).toEqual(["resolving", "loading", "failed"]);
    expect(door.pending()).toEqual([]);
  });

  it("cancelling an idle plugin is inert", () => {
    const { door } = harness();
    expect(door.cancel("cad")).toBe(false);
    expect(door.pending()).toEqual([]);
  });
});

describe("dynamic extension admission", () => {
  it("loads exactly the Store-authored module URL and admits its matching manifest", async () => {
    const calls: unknown[] = [];
    const handle = { id: "bridge" };
    await expect(admitWgpuDynamicExtension(JSON.stringify(extensionFixture), async (record) => {
      calls.push(record);
      return { handle, manifest: { pluginId: record.extensionId, version: record.version } };
    })).resolves.toBe(handle);
    expect(calls).toEqual([extensionFixture]);
  });

  it("refuses substituted identity or version after load", async () => {
    await expect(admitWgpuDynamicExtension(JSON.stringify(extensionFixture), async () => ({ handle: {}, manifest: { pluginId: "substituted", version: extensionFixture.version } }))).rejects.toThrow(/extension-install\.identity/);
    await expect(admitWgpuDynamicExtension(JSON.stringify(extensionFixture), async () => ({ handle: {}, manifest: { pluginId: extensionFixture.extensionId, version: "9.9.9" } }))).rejects.toThrow(/extension-install\.version/);
  });

  it("refuses caller-shaped metadata outside the exact Store record", async () => {
    await expect(admitWgpuDynamicExtension(JSON.stringify({ ...extensionFixture, enabled: true }), async () => ({ handle: {}, manifest: { pluginId: extensionFixture.extensionId, version: extensionFixture.version } }))).rejects.toThrow(/fields mismatch/);
  });

  it("keeps dynamic installation and actor retirement on separate bridge doors", async () => {
    const host: {
      [WGPU_DYNAMIC_EXTENSION_INSTALL_GLOBAL]?: (value: string) => Promise<unknown>;
      [WGPU_DYNAMIC_EXTENSION_RETIRE_GLOBAL]?: (value: string) => Promise<void>;
    } = {};
    const retired: string[] = [];
    installWgpuDynamicExtensionDoor(host, async (record) => ({ handle: { id: record.extensionId }, manifest: { pluginId: record.extensionId, version: record.version } }), async (extensionId) => { retired.push(extensionId); });
    await expect(host[WGPU_DYNAMIC_EXTENSION_INSTALL_GLOBAL]!(JSON.stringify(extensionFixture))).resolves.toEqual({ id: extensionFixture.extensionId });
    await expect(host[WGPU_DYNAMIC_EXTENSION_RETIRE_GLOBAL]!(extensionFixture.extensionId)).resolves.toBeUndefined();
    expect(retired).toEqual([extensionFixture.extensionId]);
  });

  it("retires a mounted actor when manifest admission rejects its identity", async () => {
    const host: {
      [WGPU_DYNAMIC_EXTENSION_INSTALL_GLOBAL]?: (value: string) => Promise<unknown>;
      [WGPU_DYNAMIC_EXTENSION_RETIRE_GLOBAL]?: (value: string) => Promise<void>;
    } = {};
    const retired: string[] = [];
    installWgpuDynamicExtensionDoor(host, async () => ({ handle: {}, manifest: { pluginId: "substituted", version: extensionFixture.version } }), async (extensionId) => { retired.push(extensionId); });
    await expect(host[WGPU_DYNAMIC_EXTENSION_INSTALL_GLOBAL]!(JSON.stringify(extensionFixture))).rejects.toThrow(/extension-install\.identity/);
    expect(retired).toEqual([extensionFixture.extensionId]);
  });
});
